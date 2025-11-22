pub fn sub_82331748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331748 size=16
    let mut pc: u32 = 0x82331748;
    'dispatch: loop {
        match pc {
            0x82331748 => {
    //   block [0x82331748..0x82331758)
	// 82331748: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233174C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82331750: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82331754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331758 size=16
    let mut pc: u32 = 0x82331758;
    'dispatch: loop {
        match pc {
            0x82331758 => {
    //   block [0x82331758..0x82331768)
	// 82331758: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233175C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82331760: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82331764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331768 size=16
    let mut pc: u32 = 0x82331768;
    'dispatch: loop {
        match pc {
            0x82331768 => {
    //   block [0x82331768..0x82331778)
	// 82331768: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233176C: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82331770: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 82331774: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331778 size=16
    let mut pc: u32 = 0x82331778;
    'dispatch: loop {
        match pc {
            0x82331778 => {
    //   block [0x82331778..0x82331788)
	// 82331778: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8233177C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82331780: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 82331784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331788 size=16
    let mut pc: u32 = 0x82331788;
    'dispatch: loop {
        match pc {
            0x82331788 => {
    //   block [0x82331788..0x82331798)
	// 82331788: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233178C: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82331790: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82331794: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331798 size=16
    let mut pc: u32 = 0x82331798;
    'dispatch: loop {
        match pc {
            0x82331798 => {
    //   block [0x82331798..0x823317A8)
	// 82331798: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8233179C: 394A00FF  addi r10, r10, 0xff
	ctx.r[10].s64 = ctx.r[10].s64 + 255;
	// 823317A0: 7D4B21AE  stbx r10, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u8) };
	// 823317A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823317A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823317A8 size=668
    let mut pc: u32 = 0x823317A8;
    'dispatch: loop {
        match pc {
            0x823317A8 => {
    //   block [0x823317A8..0x82331A44)
	// 823317A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823317AC: 48E769AD  bl 0x831a8158
	ctx.lr = 0x823317B0;
	sub_831A8130(ctx, base);
	// 823317B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823317B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823317B8: 8B060000  lbz r24, 0(r6)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 823317BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823317C0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 823317C4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823317C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823317CC: 409A000C  bne cr6, 0x823317d8
	if !ctx.cr[6].eq {
	pc = 0x823317D8; continue 'dispatch;
	}
	// 823317D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823317D4: 4800000C  b 0x823317e0
	pc = 0x823317E0; continue 'dispatch;
	// 823317D8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 823317DC: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823317E0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 823317E4: 419A0258  beq cr6, 0x82331a3c
	if ctx.cr[6].eq {
	pc = 0x82331A3C; continue 'dispatch;
	}
	// 823317E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823317EC: 409A000C  bne cr6, 0x823317f8
	if !ctx.cr[6].eq {
	pc = 0x823317F8; continue 'dispatch;
	}
	// 823317F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823317F4: 4800000C  b 0x82331800
	pc = 0x82331800; continue 'dispatch;
	// 823317F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823317FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82331800: 216BFFFF  subfic r11, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[11].s64 = (-1 as i64) - ctx.r[11].s64;
	// 82331804: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82331808: 4098000C  bge cr6, 0x82331814
	if !ctx.cr[6].lt {
	pc = 0x82331814; continue 'dispatch;
	}
	// 8233180C: 48886515  bl 0x82bb7d20
	ctx.lr = 0x82331810;
	sub_82BB7D20(ctx, base);
	// 82331810: 4800022C  b 0x82331a3c
	pc = 0x82331A3C; continue 'dispatch;
	// 82331814: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82331818: 409A000C  bne cr6, 0x82331824
	if !ctx.cr[6].eq {
	pc = 0x82331824; continue 'dispatch;
	}
	// 8233181C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82331820: 4800000C  b 0x8233182c
	pc = 0x8233182C; continue 'dispatch;
	// 82331824: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331828: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8233182C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82331830: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82331834: 4098011C  bge cr6, 0x82331950
	if !ctx.cr[6].lt {
	pc = 0x82331950; continue 'dispatch;
	}
	// 82331838: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8233183C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82331840: 210BFFFF  subfic r8, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[8].s64 = (-1 as i64) - ctx.r[11].s64;
	// 82331844: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82331848: 41980008  blt cr6, 0x82331850
	if ctx.cr[6].lt {
	pc = 0x82331850; continue 'dispatch;
	}
	// 8233184C: 7F2B4A14  add r25, r11, r9
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82331850: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82331854: 409A000C  bne cr6, 0x82331860
	if !ctx.cr[6].eq {
	pc = 0x82331860; continue 'dispatch;
	}
	// 82331858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233185C: 4800000C  b 0x82331868
	pc = 0x82331868; continue 'dispatch;
	// 82331860: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331864: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82331868: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8233186C: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82331870: 40980020  bge cr6, 0x82331890
	if !ctx.cr[6].lt {
	pc = 0x82331890; continue 'dispatch;
	}
	// 82331874: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82331878: 409A000C  bne cr6, 0x82331884
	if !ctx.cr[6].eq {
	pc = 0x82331884; continue 'dispatch;
	}
	// 8233187C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82331880: 4800000C  b 0x8233188c
	pc = 0x8233188C; continue 'dispatch;
	// 82331884: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331888: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8233188C: 7F2BDA14  add r25, r11, r27
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82331890: 3F408335  lis r26, -0x7ccb
	ctx.r[26].s64 = -2093678592;
	// 82331894: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82331898: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8233189C: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 823318A0: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 823318A4: 807A110C  lwz r3, 0x110c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4364 as u32) ) } as u64;
	// 823318A8: 48AC0821  bl 0x82df20c8
	ctx.lr = 0x823318AC;
	sub_82DF20C8(ctx, base);
	// 823318AC: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823318B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823318B4: 7CC5E051  subf. r6, r5, r28
	ctx.r[6].s64 = ctx.r[28].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 823318B8: 7FE6EA14  add r31, r6, r29
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[29].u64;
	// 823318BC: 4182000C  beq 0x823318c8
	if ctx.cr[0].eq {
	pc = 0x823318C8; continue 'dispatch;
	}
	// 823318C0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 823318C4: 48E77445  bl 0x831a8d08
	ctx.lr = 0x823318C8;
	sub_831A8D08(ctx, base);
	// 823318C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 823318CC: 419A0020  beq cr6, 0x823318ec
	if ctx.cr[6].eq {
	pc = 0x823318EC; continue 'dispatch;
	}
	// 823318D0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823318D4: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823318D8: 41820014  beq 0x823318ec
	if ctx.cr[0].eq {
	pc = 0x823318EC; continue 'dispatch;
	}
	// 823318DC: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 823318E0: 9B0B0000  stb r24, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 823318E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823318E8: 4200FFF8  bdnz 0x823318e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x823318E0; continue 'dispatch;
	}
	// 823318EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823318F0: 7CDC5851  subf. r6, r28, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 823318F4: 41820014  beq 0x82331908
	if ctx.cr[0].eq {
	pc = 0x82331908; continue 'dispatch;
	}
	// 823318F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823318FC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82331900: 7C7FDA14  add r3, r31, r27
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82331904: 48E77405  bl 0x831a8d08
	ctx.lr = 0x82331908;
	sub_831A8D08(ctx, base);
	// 82331908: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233190C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82331910: 409A000C  bne cr6, 0x8233191c
	if !ctx.cr[6].eq {
	pc = 0x8233191C; continue 'dispatch;
	}
	// 82331914: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82331918: 4800000C  b 0x82331924
	pc = 0x82331924; continue 'dispatch;
	// 8233191C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331920: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82331924: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82331928: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8233192C: 419A000C  beq cr6, 0x82331938
	if ctx.cr[6].eq {
	pc = 0x82331938; continue 'dispatch;
	}
	// 82331930: 807A110C  lwz r3, 0x110c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82331934: 48AC0855  bl 0x82df2188
	ctx.lr = 0x82331938;
	sub_82DF2188(ctx, base);
	// 82331938: 7D7DCA14  add r11, r29, r25
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[25].u64;
	// 8233193C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82331940: 7D5DFA14  add r10, r29, r31
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82331944: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82331948: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233194C: 480000F0  b 0x82331a3c
	pc = 0x82331A3C; continue 'dispatch;
	// 82331950: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331954: 7CDCF850  subf r6, r28, r31
	ctx.r[6].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82331958: 7F06D840  cmplw cr6, r6, r27
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8233195C: 4098007C  bge cr6, 0x823319d8
	if !ctx.cr[6].lt {
	pc = 0x823319D8; continue 'dispatch;
	}
	// 82331960: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82331964: 41820014  beq 0x82331978
	if ctx.cr[0].eq {
	pc = 0x82331978; continue 'dispatch;
	}
	// 82331968: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8233196C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82331970: 7C7CDA14  add r3, r28, r27
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82331974: 48E77395  bl 0x831a8d08
	ctx.lr = 0x82331978;
	sub_831A8D08(ctx, base);
	// 82331978: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8233197C: 7D6AE050  subf r11, r10, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[10].s64;
	// 82331980: 7D6BDA15  add. r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82331984: 4182001C  beq 0x823319a0
	if ctx.cr[0].eq {
	pc = 0x823319A0; continue 'dispatch;
	}
	// 82331988: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233198C: 41820014  beq 0x823319a0
	if ctx.cr[0].eq {
	pc = 0x823319A0; continue 'dispatch;
	}
	// 82331990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82331994: 9B0A0000  stb r24, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 82331998: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233199C: 4200FFF8  bdnz 0x82331994
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82331994; continue 'dispatch;
	}
	// 823319A0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823319A4: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 823319A8: 7D5B5850  subf r10, r27, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 823319AC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 823319B0: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823319B4: 419A0088  beq cr6, 0x82331a3c
	if ctx.cr[6].eq {
	pc = 0x82331A3C; continue 'dispatch;
	}
	// 823319B8: 7D5C5051  subf. r10, r28, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823319BC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823319C0: 4182007C  beq 0x82331a3c
	if ctx.cr[0].eq {
	pc = 0x82331A3C; continue 'dispatch;
	}
	// 823319C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 823319C8: 9B0B0000  stb r24, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 823319CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823319D0: 4200FFF8  bdnz 0x823319c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x823319C8; continue 'dispatch;
	}
	// 823319D4: 48000068  b 0x82331a3c
	pc = 0x82331A3C; continue 'dispatch;
	// 823319D8: 7CBBF850  subf r5, r27, r31
	ctx.r[5].s64 = ctx.r[31].s64 - ctx.r[27].s64;
	// 823319DC: 7CC5F851  subf. r6, r5, r31
	ctx.r[6].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 823319E0: 7FA6FA14  add r29, r6, r31
	ctx.r[29].u64 = ctx.r[6].u64 + ctx.r[31].u64;
	// 823319E4: 41820010  beq 0x823319f4
	if ctx.cr[0].eq {
	pc = 0x823319F4; continue 'dispatch;
	}
	// 823319E8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 823319EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823319F0: 48E77319  bl 0x831a8d08
	ctx.lr = 0x823319F4;
	sub_831A8D08(ctx, base);
	// 823319F4: 7D7CF850  subf r11, r28, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 823319F8: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 823319FC: 7CDB5851  subf. r6, r27, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82331A00: 40810014  ble 0x82331a14
	if !ctx.cr[0].gt {
	pc = 0x82331A14; continue 'dispatch;
	}
	// 82331A04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82331A08: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82331A0C: 7C66F850  subf r3, r6, r31
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[6].s64;
	// 82331A10: 48E772F9  bl 0x831a8d08
	ctx.lr = 0x82331A14;
	sub_831A8D08(ctx, base);
	// 82331A14: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82331A18: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82331A1C: 419A0020  beq cr6, 0x82331a3c
	if ctx.cr[6].eq {
	pc = 0x82331A3C; continue 'dispatch;
	}
	// 82331A20: 7D5C5851  subf. r10, r28, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82331A24: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82331A28: 41820014  beq 0x82331a3c
	if ctx.cr[0].eq {
	pc = 0x82331A3C; continue 'dispatch;
	}
	// 82331A2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82331A30: 9B0B0000  stb r24, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 82331A34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82331A38: 4200FFF8  bdnz 0x82331a30
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82331A30; continue 'dispatch;
	}
	// 82331A3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82331A40: 48E76768  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331A48 size=96
    let mut pc: u32 = 0x82331A48;
    'dispatch: loop {
        match pc {
            0x82331A48 => {
    //   block [0x82331A48..0x82331AA8)
	// 82331A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331A4C: 48E76721  bl 0x831a816c
	ctx.lr = 0x82331A50;
	sub_831A8130(ctx, base);
	// 82331A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331A58: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331A5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82331A60: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331A64: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82331A68: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82331A6C: 7F032840  cmplw cr6, r3, r5
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82331A70: 419A001C  beq cr6, 0x82331a8c
	if ctx.cr[6].eq {
	pc = 0x82331A8C; continue 'dispatch;
	}
	// 82331A74: 7CC52851  subf. r6, r5, r5
	ctx.r[6].s64 = ctx.r[5].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82331A78: 7FC61A14  add r30, r6, r3
	ctx.r[30].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 82331A7C: 4081000C  ble 0x82331a88
	if !ctx.cr[0].gt {
	pc = 0x82331A88; continue 'dispatch;
	}
	// 82331A80: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82331A84: 48E77285  bl 0x831a8d08
	ctx.lr = 0x82331A88;
	sub_831A8D08(ctx, base);
	// 82331A88: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82331A8C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82331A90: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82331A94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82331A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331A9C: 4BFFFD0D  bl 0x823317a8
	ctx.lr = 0x82331AA0;
	sub_823317A8(ctx, base);
	// 82331AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82331AA4: 48E76718  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331AA8 size=72
    let mut pc: u32 = 0x82331AA8;
    'dispatch: loop {
        match pc {
            0x82331AA8 => {
    //   block [0x82331AA8..0x82331AF0)
	// 82331AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331AB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331ABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82331AC0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82331AC4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82331AC8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82331ACC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82331AD0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82331AD4: 4BFFFF75  bl 0x82331a48
	ctx.lr = 0x82331AD8;
	sub_82331A48(ctx, base);
	// 82331AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331ADC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331AF0 size=164
    let mut pc: u32 = 0x82331AF0;
    'dispatch: loop {
        match pc {
            0x82331AF0 => {
    //   block [0x82331AF0..0x82331B94)
	// 82331AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82331AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331B00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331B04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82331B0C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82331B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82331B14: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82331B18: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82331B1C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82331B20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82331B24: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82331B28: 419A000C  beq cr6, 0x82331b34
	if ctx.cr[6].eq {
	pc = 0x82331B34; continue 'dispatch;
	}
	// 82331B2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331B30: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82331B34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82331B38: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82331B3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82331B40: 4BFFFF09  bl 0x82331a48
	ctx.lr = 0x82331B44;
	sub_82331A48(ctx, base);
	// 82331B44: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82331B48: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82331B4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82331B50: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331B54: 8101006C  lwz r8, 0x6c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82331B58: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82331B5C: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82331B60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82331B64: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82331B68: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82331B6C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82331B70: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82331B74: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82331B78: 48138489  bl 0x8246a000
	ctx.lr = 0x82331B7C;
	sub_8246A000(ctx, base);
	// 82331B7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82331B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331B88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82331B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331B98 size=16
    let mut pc: u32 = 0x82331B98;
    'dispatch: loop {
        match pc {
            0x82331B98 => {
    //   block [0x82331B98..0x82331BA8)
	// 82331B98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331B9C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82331BA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82331BA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331BA8 size=8
    let mut pc: u32 = 0x82331BA8;
    'dispatch: loop {
        match pc {
            0x82331BA8 => {
    //   block [0x82331BA8..0x82331BB0)
	// 82331BA8: 80630064  lwz r3, 0x64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82331BAC: 4BFFFB9C  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331BB0 size=8
    let mut pc: u32 = 0x82331BB0;
    'dispatch: loop {
        match pc {
            0x82331BB0 => {
    //   block [0x82331BB0..0x82331BB8)
	// 82331BB0: 80630064  lwz r3, 0x64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82331BB4: 4BFFFB7C  b 0x82331730
	sub_82331730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331BB8 size=12
    let mut pc: u32 = 0x82331BB8;
    'dispatch: loop {
        match pc {
            0x82331BB8 => {
    //   block [0x82331BB8..0x82331BC4)
	// 82331BB8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82331BBC: 386B8290  addi r3, r11, -0x7d70
	ctx.r[3].s64 = ctx.r[11].s64 + -32112;
	// 82331BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82331BC8 size=88
    let mut pc: u32 = 0x82331BC8;
    'dispatch: loop {
        match pc {
            0x82331BC8 => {
    //   block [0x82331BC8..0x82331C20)
	// 82331BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331BD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331BD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331BD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331BDC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82331BE0: 4BF8ED59  bl 0x822c0938
	ctx.lr = 0x82331BE4;
	sub_822C0938(ctx, base);
	// 82331BE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82331BE8: 41820020  beq 0x82331c08
	if ctx.cr[0].eq {
	pc = 0x82331C08; continue 'dispatch;
	}
	// 82331BEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82331BF0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82331BF4: 394A0CB4  addi r10, r10, 0xcb4
	ctx.r[10].s64 = ctx.r[10].s64 + 3252;
	// 82331BF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82331BFC: 13FF58C7  vcmpequd (lvx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82331C20 size=112
    let mut pc: u32 = 0x82331C20;
    'dispatch: loop {
        match pc {
            0x82331C20 => {
    //   block [0x82331C20..0x82331C90)
	// 82331C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82331C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331C34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82331C38: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82331C3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82331C40: 4BF8ECF9  bl 0x822c0938
	ctx.lr = 0x82331C44;
	sub_822C0938(ctx, base);
	// 82331C44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82331C48: 41820024  beq 0x82331c6c
	if ctx.cr[0].eq {
	pc = 0x82331C6C; continue 'dispatch;
	}
	// 82331C4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82331C50: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82331C54: 392B0CB4  addi r9, r11, 0xcb4
	ctx.r[9].s64 = ctx.r[11].s64 + 3252;
	// 82331C58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82331C5C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82331C60: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331C90 size=100
    let mut pc: u32 = 0x82331C90;
    'dispatch: loop {
        match pc {
            0x82331C90 => {
    //   block [0x82331C90..0x82331CF4)
	// 82331C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331C9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331CA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82331CA8: 4BFFFF79  bl 0x82331c20
	ctx.lr = 0x82331CAC;
	sub_82331C20(ctx, base);
	// 82331CAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331CB0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331CB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82331CB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82331CBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82331CC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82331CC4: 419A0018  beq cr6, 0x82331cdc
	if ctx.cr[6].eq {
	pc = 0x82331CDC; continue 'dispatch;
	}
	// 82331CC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331CCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82331CD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82331CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82331CD8: 4E800421  bctrl
	ctx.lr = 0x82331CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82331CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331CE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331CF8 size=92
    let mut pc: u32 = 0x82331CF8;
    'dispatch: loop {
        match pc {
            0x82331CF8 => {
    //   block [0x82331CF8..0x82331D54)
	// 82331CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331CFC: 48E76471  bl 0x831a816c
	ctx.lr = 0x82331D00;
	sub_831A8130(ctx, base);
	// 82331D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331D04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82331D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331D0C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82331D10: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 82331D14: 41980008  blt cr6, 0x82331d1c
	if ctx.cr[6].lt {
	pc = 0x82331D1C; continue 'dispatch;
	}
	// 82331D18: 4BFEEAF1  bl 0x82320808
	ctx.lr = 0x82331D1C;
	sub_82320808(ctx, base);
	// 82331D1C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82331D20: 57CBE8FA  rlwinm r11, r30, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 82331D24: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82331D28: 57CA06FE  clrlwi r10, r30, 0x1b
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 82331D2C: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82331D30: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82331D34: 4182000C  beq 0x82331d40
	if ctx.cr[0].eq {
	pc = 0x82331D40; continue 'dispatch;
	}
	// 82331D38: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82331D3C: 48000008  b 0x82331d44
	pc = 0x82331D44; continue 'dispatch;
	// 82331D40: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82331D44: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82331D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331D50: 48E7646C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331D58 size=100
    let mut pc: u32 = 0x82331D58;
    'dispatch: loop {
        match pc {
            0x82331D58 => {
    //   block [0x82331D58..0x82331DBC)
	// 82331D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82331D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331D6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82331D70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82331D74: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 82331D78: 41980008  blt cr6, 0x82331d80
	if ctx.cr[6].lt {
	pc = 0x82331D80; continue 'dispatch;
	}
	// 82331D7C: 4BFEEA8D  bl 0x82320808
	ctx.lr = 0x82331D80;
	sub_82320808(ctx, base);
	// 82331D80: 57EBE8FA  rlwinm r11, r31, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000007u64;
	// 82331D84: 57EA06FE  clrlwi r10, r31, 0x1b
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 82331D88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82331D8C: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82331D90: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82331D94: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82331D98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82331D9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82331DA0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82331DA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331DB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82331DB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331DC0 size=8
    let mut pc: u32 = 0x82331DC0;
    'dispatch: loop {
        match pc {
            0x82331DC0 => {
    //   block [0x82331DC0..0x82331DC8)
	// 82331DC0: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82331DC4: 4BFFFF94  b 0x82331d58
	sub_82331D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82331DC8 size=12
    let mut pc: u32 = 0x82331DC8;
    'dispatch: loop {
        match pc {
            0x82331DC8 => {
    //   block [0x82331DC8..0x82331DD4)
	// 82331DC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82331DCC: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82331DD0: 4BFFFF28  b 0x82331cf8
	sub_82331CF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331DD8 size=144
    let mut pc: u32 = 0x82331DD8;
    'dispatch: loop {
        match pc {
            0x82331DD8 => {
    //   block [0x82331DD8..0x82331E68)
	// 82331DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82331DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331DF0: 48B2AB39  bl 0x82e5c928
	ctx.lr = 0x82331DF4;
	sub_82E5C928(ctx, base);
	// 82331DF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82331DF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82331DFC: 396B0D1C  addi r11, r11, 0xd1c
	ctx.r[11].s64 = ctx.r[11].s64 + 3356;
	// 82331E00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82331E04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82331E08: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82331E0C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82331E10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82331E14: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82331E18: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82331E1C: 388A0CC0  addi r4, r10, 0xcc0
	ctx.r[4].s64 = ctx.r[10].s64 + 3264;
	// 82331E20: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82331E24: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 82331E28: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82331E2C: 4BF8E5AD  bl 0x822c03d8
	ctx.lr = 0x82331E30;
	sub_822C03D8(ctx, base);
	// 82331E30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82331E34: 41820010  beq 0x82331e44
	if ctx.cr[0].eq {
	pc = 0x82331E44; continue 'dispatch;
	}
	// 82331E38: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82331E3C: 4BFFFC6D  bl 0x82331aa8
	ctx.lr = 0x82331E40;
	sub_82331AA8(ctx, base);
	// 82331E40: 48000008  b 0x82331e48
	pc = 0x82331E48; continue 'dispatch;
	// 82331E44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82331E48: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 82331E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331E50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82331E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82331E68 size=84
    let mut pc: u32 = 0x82331E68;
    'dispatch: loop {
        match pc {
            0x82331E68 => {
    //   block [0x82331E68..0x82331EBC)
	// 82331E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82331E70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82331E74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82331E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331E7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82331E80: 83FE0064  lwz r31, 0x64(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82331E84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82331E88: 419A0014  beq cr6, 0x82331e9c
	if ctx.cr[6].eq {
	pc = 0x82331E9C; continue 'dispatch;
	}
	// 82331E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331E90: 48138171  bl 0x8246a000
	ctx.lr = 0x82331E94;
	sub_8246A000(ctx, base);
	// 82331E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82331E98: 4BF8E3D1  bl 0x822c0268
	ctx.lr = 0x82331E9C;
	sub_822C0268(ctx, base);
	// 82331E9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82331EA0: 48B2A6E9  bl 0x82e5c588
	ctx.lr = 0x82331EA4;
	sub_82E5C588(ctx, base);
	// 82331EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82331EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82331EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82331EB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82331EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82331EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82331EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82331EC0 size=596
    let mut pc: u32 = 0x82331EC0;
    'dispatch: loop {
        match pc {
            0x82331EC0 => {
    //   block [0x82331EC0..0x82332114)
	// 82331EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82331EC4: 48E762A5  bl 0x831a8168
	ctx.lr = 0x82331EC8;
	sub_831A8130(ctx, base);
	// 82331EC8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82331ECC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82331ED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82331ED4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82331ED8: 48E22729  bl 0x83154600
	ctx.lr = 0x82331EDC;
	sub_83154600(ctx, base);
	// 82331EDC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82331EE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82331EE4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82331EE8: 808B8650  lwz r4, -0x79b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31152 as u32) ) } as u64;
	// 82331EEC: 48488185  bl 0x827ba070
	ctx.lr = 0x82331EF0;
	sub_827BA070(ctx, base);
	// 82331EF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82331EF4: 4182006C  beq 0x82331f60
	if ctx.cr[0].eq {
	pc = 0x82331F60; continue 'dispatch;
	}
	// 82331EF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82331EFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82331F00: 388B0D44  addi r4, r11, 0xd44
	ctx.r[4].s64 = ctx.r[11].s64 + 3396;
	// 82331F04: 48AC1B05  bl 0x82df3a08
	ctx.lr = 0x82331F08;
	sub_82DF3A08(ctx, base);
	// 82331F08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82331F0C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82331F10: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82331F14: 4BFFA92D  bl 0x8232c840
	ctx.lr = 0x82331F18;
	sub_8232C840(ctx, base);
	// 82331F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82331F1C: 4BFFFD75  bl 0x82331c90
	ctx.lr = 0x82331F20;
	sub_82331C90(ctx, base);
	// 82331F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82331F24: 48AC1505  bl 0x82df3428
	ctx.lr = 0x82331F28;
	sub_82DF3428(ctx, base);
	// 82331F28: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82331F2C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82331F30: 808BB408  lwz r4, -0x4bf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19448 as u32) ) } as u64;
	// 82331F34: 48AC1AD5  bl 0x82df3a08
	ctx.lr = 0x82331F38;
	sub_82DF3A08(ctx, base);
	// 82331F38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82331F3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82331F40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82331F44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82331F48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82331F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82331F50: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82331F54: 48B2CB3D  bl 0x82e5ea90
	ctx.lr = 0x82331F58;
	sub_82E5EA90(ctx, base);
	// 82331F58: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82331F5C: 480001A0  b 0x823320fc
	pc = 0x823320FC; continue 'dispatch;
	// 82331F60: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 82331F64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82331F68: 4BFE1D79  bl 0x82313ce0
	ctx.lr = 0x82331F6C;
	sub_82313CE0(ctx, base);
	// 82331F6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82331F70: 418200E4  beq 0x82332054
	if ctx.cr[0].eq {
	pc = 0x82332054; continue 'dispatch;
	}
	// 82331F74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82331F78: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82331F7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82331F80: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82331F84: 3B810090  addi r28, r1, 0x90
	ctx.r[28].s64 = ctx.r[1].s64 + 144;
	// 82331F88: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82331F8C: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82331F90: D3E10090  stfs f31, 0x90(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82331F94: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82331F98: D3E10098  stfs f31, 0x98(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82331F9C: D3E1009C  stfs f31, 0x9c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82331FA0: 4BFE1BC9  bl 0x82313b68
	ctx.lr = 0x82331FA4;
	sub_82313B68(ctx, base);
	// 82331FA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82331FA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82331FAC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82331FB0: 4BFE28C1  bl 0x82314870
	ctx.lr = 0x82331FB4;
	sub_82314870(ctx, base);
	// 82331FB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82331FB8: 40820038  bne 0x82331ff0
	if !ctx.cr[0].eq {
	pc = 0x82331FF0; continue 'dispatch;
	}
	// 82331FBC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82331FC0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82331FC4: 808BB434  lwz r4, -0x4bcc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19404 as u32) ) } as u64;
	// 82331FC8: 48AC1A41  bl 0x82df3a08
	ctx.lr = 0x82331FCC;
	sub_82DF3A08(ctx, base);
	// 82331FCC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82331FD0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82331FD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82331FD8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82331FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82331FE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82331FE4: 48B2CAAD  bl 0x82e5ea90
	ctx.lr = 0x82331FE8;
	sub_82E5EA90(ctx, base);
	// 82331FE8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82331FEC: 48000110  b 0x823320fc
	pc = 0x823320FC; continue 'dispatch;
	// 82331FF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82331FF4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82331FF8: 388B0D44  addi r4, r11, 0xd44
	ctx.r[4].s64 = ctx.r[11].s64 + 3396;
	// 82331FFC: 48AC1A0D  bl 0x82df3a08
	ctx.lr = 0x82332000;
	sub_82DF3A08(ctx, base);
	// 82332000: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82332004: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82332008: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8233200C: 4BFFA835  bl 0x8232c840
	ctx.lr = 0x82332010;
	sub_8232C840(ctx, base);
	// 82332010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82332014: 4BFFFC7D  bl 0x82331c90
	ctx.lr = 0x82332018;
	sub_82331C90(ctx, base);
	// 82332018: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233201C: 48AC140D  bl 0x82df3428
	ctx.lr = 0x82332020;
	sub_82DF3428(ctx, base);
	// 82332020: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332024: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82332028: 808BB408  lwz r4, -0x4bf8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19448 as u32) ) } as u64;
	// 8233202C: 48AC19DD  bl 0x82df3a08
	ctx.lr = 0x82332030;
	sub_82DF3A08(ctx, base);
	// 82332030: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82332034: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82332038: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233203C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82332040: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82332044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82332048: 48B2CA49  bl 0x82e5ea90
	ctx.lr = 0x8233204C;
	sub_82E5EA90(ctx, base);
	// 8233204C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82332050: 480000AC  b 0x823320fc
	pc = 0x823320FC; continue 'dispatch;
	// 82332054: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332058: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233205C: 388B0D44  addi r4, r11, 0xd44
	ctx.r[4].s64 = ctx.r[11].s64 + 3396;
	// 82332060: 48AC19A9  bl 0x82df3a08
	ctx.lr = 0x82332064;
	sub_82DF3A08(ctx, base);
	// 82332064: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82332068: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8233206C: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82332070: 4BFFA7D1  bl 0x8232c840
	ctx.lr = 0x82332074;
	sub_8232C840(ctx, base);
	// 82332074: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82332078: 4BFFFC19  bl 0x82331c90
	ctx.lr = 0x8233207C;
	sub_82331C90(ctx, base);
	// 8233207C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82332080: 48AC13A9  bl 0x82df3428
	ctx.lr = 0x82332084;
	sub_82DF3428(ctx, base);
	// 82332084: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82332088: 4BFE0B81  bl 0x82312c08
	ctx.lr = 0x8233208C;
	sub_82312C08(ctx, base);
	// 8233208C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332090: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82332094: 41820038  beq 0x823320cc
	if ctx.cr[0].eq {
	pc = 0x823320CC; continue 'dispatch;
	}
	// 82332098: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233209C: 808BB40C  lwz r4, -0x4bf4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19444 as u32) ) } as u64;
	// 823320A0: 48AC1969  bl 0x82df3a08
	ctx.lr = 0x823320A4;
	sub_82DF3A08(ctx, base);
	// 823320A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823320A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823320AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823320B0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823320B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823320B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823320BC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823320C0: 48B2C9D1  bl 0x82e5ea90
	ctx.lr = 0x823320C4;
	sub_82E5EA90(ctx, base);
	// 823320C4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 823320C8: 48000034  b 0x823320fc
	pc = 0x823320FC; continue 'dispatch;
	// 823320CC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823320D0: 808BB414  lwz r4, -0x4bec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19436 as u32) ) } as u64;
	// 823320D4: 48AC1935  bl 0x82df3a08
	ctx.lr = 0x823320D8;
	sub_82DF3A08(ctx, base);
	// 823320D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823320DC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823320E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823320E4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 823320E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823320EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823320F0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823320F4: 48B2C99D  bl 0x82e5ea90
	ctx.lr = 0x823320F8;
	sub_82E5EA90(ctx, base);
	// 823320F8: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823320FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332100: 419A0008  beq cr6, 0x82332108
	if ctx.cr[6].eq {
	pc = 0x82332108; continue 'dispatch;
	}
	// 82332104: 4BF8E78D  bl 0x822c0890
	ctx.lr = 0x82332108;
	sub_822C0890(ctx, base);
	// 82332108: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8233210C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82332110: 48E760A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332118 size=244
    let mut pc: u32 = 0x82332118;
    'dispatch: loop {
        match pc {
            0x82332118 => {
    //   block [0x82332118..0x8233220C)
	// 82332118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233211C: 48E76051  bl 0x831a816c
	ctx.lr = 0x82332120;
	sub_831A8130(ctx, base);
	// 82332120: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332128: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233212C: 48E224D5  bl 0x83154600
	ctx.lr = 0x82332130;
	sub_83154600(ctx, base);
	// 82332130: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 82332134: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82332138: 4BFE1BB9  bl 0x82313cf0
	ctx.lr = 0x8233213C;
	sub_82313CF0(ctx, base);
	// 8233213C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82332140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82332144: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82332148: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233214C: 4E800421  bctrl
	ctx.lr = 0x82332150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82332150: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332158: 388B0D44  addi r4, r11, 0xd44
	ctx.r[4].s64 = ctx.r[11].s64 + 3396;
	// 8233215C: 48AC18AD  bl 0x82df3a08
	ctx.lr = 0x82332160;
	sub_82DF3A08(ctx, base);
	// 82332160: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332164: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82332168: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8233216C: 4BFFA6D5  bl 0x8232c840
	ctx.lr = 0x82332170;
	sub_8232C840(ctx, base);
	// 82332170: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82332174: 4BFFFB1D  bl 0x82331c90
	ctx.lr = 0x82332178;
	sub_82331C90(ctx, base);
	// 82332178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233217C: 48AC12AD  bl 0x82df3428
	ctx.lr = 0x82332180;
	sub_82DF3428(ctx, base);
	// 82332180: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82332184: 4BFE0A85  bl 0x82312c08
	ctx.lr = 0x82332188;
	sub_82312C08(ctx, base);
	// 82332188: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233218C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82332190: 41820038  beq 0x823321c8
	if ctx.cr[0].eq {
	pc = 0x823321C8; continue 'dispatch;
	}
	// 82332194: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332198: 808BB418  lwz r4, -0x4be8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19432 as u32) ) } as u64;
	// 8233219C: 48AC186D  bl 0x82df3a08
	ctx.lr = 0x823321A0;
	sub_82DF3A08(ctx, base);
	// 823321A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823321A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823321A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823321AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823321B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823321B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823321B8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823321BC: 48B2C8D5  bl 0x82e5ea90
	ctx.lr = 0x823321C0;
	sub_82E5EA90(ctx, base);
	// 823321C0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823321C4: 48000034  b 0x823321f8
	pc = 0x823321F8; continue 'dispatch;
	// 823321C8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823321CC: 808BB41C  lwz r4, -0x4be4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19428 as u32) ) } as u64;
	// 823321D0: 48AC1839  bl 0x82df3a08
	ctx.lr = 0x823321D4;
	sub_82DF3A08(ctx, base);
	// 823321D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823321D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823321DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823321E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823321E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823321E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823321EC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823321F0: 48B2C8A1  bl 0x82e5ea90
	ctx.lr = 0x823321F4;
	sub_82E5EA90(ctx, base);
	// 823321F4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823321F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823321FC: 419A0008  beq cr6, 0x82332204
	if ctx.cr[6].eq {
	pc = 0x82332204; continue 'dispatch;
	}
	// 82332200: 4BF8E691  bl 0x822c0890
	ctx.lr = 0x82332204;
	sub_822C0890(ctx, base);
	// 82332204: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82332208: 48E75FB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332210 size=220
    let mut pc: u32 = 0x82332210;
    'dispatch: loop {
        match pc {
            0x82332210 => {
    //   block [0x82332210..0x823322EC)
	// 82332210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332214: 48E75F55  bl 0x831a8168
	ctx.lr = 0x82332218;
	sub_831A8130(ctx, base);
	// 82332218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233221C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82332220: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332224: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82332228: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8233222C: 41820038  beq 0x82332264
	if ctx.cr[0].eq {
	pc = 0x82332264; continue 'dispatch;
	}
	// 82332230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332234: 48E77755  bl 0x831a9988
	ctx.lr = 0x82332238;
	sub_831A9988(ctx, base);
	// 82332238: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233223C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82332240: 386B8038  addi r3, r11, -0x7fc8
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	// 82332244: 48E75EB5  bl 0x831a80f8
	ctx.lr = 0x82332248;
	sub_831A80F8(ctx, base);
	// 82332248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233224C: 41820018  beq 0x82332264
	if ctx.cr[0].eq {
	pc = 0x82332264; continue 'dispatch;
	}
	// 82332250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332258: 4BFFFC69  bl 0x82331ec0
	ctx.lr = 0x8233225C;
	sub_82331EC0(ctx, base);
	// 8233225C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82332260: 48000084  b 0x823322e4
	pc = 0x823322E4; continue 'dispatch;
	// 82332264: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82332268: 419A006C  beq cr6, 0x823322d4
	if ctx.cr[6].eq {
	pc = 0x823322D4; continue 'dispatch;
	}
	// 8233226C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332270: 48E77719  bl 0x831a9988
	ctx.lr = 0x82332274;
	sub_831A9988(ctx, base);
	// 82332274: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332278: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233227C: 386B806C  addi r3, r11, -0x7f94
	ctx.r[3].s64 = ctx.r[11].s64 + -32660;
	// 82332280: 48E75E79  bl 0x831a80f8
	ctx.lr = 0x82332284;
	sub_831A80F8(ctx, base);
	// 82332284: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332288: 41820014  beq 0x8233229c
	if ctx.cr[0].eq {
	pc = 0x8233229C; continue 'dispatch;
	}
	// 8233228C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332294: 4BFFFE85  bl 0x82332118
	ctx.lr = 0x82332298;
	sub_82332118(ctx, base);
	// 82332298: 4BFFFFC4  b 0x8233225c
	pc = 0x8233225C; continue 'dispatch;
	// 8233229C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823322A0: 419A0034  beq cr6, 0x823322d4
	if ctx.cr[6].eq {
	pc = 0x823322D4; continue 'dispatch;
	}
	// 823322A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823322A8: 48E776E1  bl 0x831a9988
	ctx.lr = 0x823322AC;
	sub_831A9988(ctx, base);
	// 823322AC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823322B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823322B4: 386B79B0  addi r3, r11, 0x79b0
	ctx.r[3].s64 = ctx.r[11].s64 + 31152;
	// 823322B8: 48E75E41  bl 0x831a80f8
	ctx.lr = 0x823322BC;
	sub_831A80F8(ctx, base);
	// 823322BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823322C0: 41820014  beq 0x823322d4
	if ctx.cr[0].eq {
	pc = 0x823322D4; continue 'dispatch;
	}
	// 823322C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823322C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823322CC: 4BFFF8CD  bl 0x82331b98
	ctx.lr = 0x823322D0;
	sub_82331B98(ctx, base);
	// 823322D0: 4BFFFF8C  b 0x8233225c
	pc = 0x8233225C; continue 'dispatch;
	// 823322D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823322D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823322DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823322E0: 48E4C1A9  bl 0x8317e488
	ctx.lr = 0x823322E4;
	sub_8317E488(ctx, base);
	// 823322E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823322E8: 48E75ED0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823322F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823322F0 size=132
    let mut pc: u32 = 0x823322F0;
    'dispatch: loop {
        match pc {
            0x823322F0 => {
    //   block [0x823322F0..0x82332374)
	// 823322F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823322F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823322F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823322FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332304: 48D957DD  bl 0x830c7ae0
	ctx.lr = 0x82332308;
	sub_830C7AE0(ctx, base);
	// 82332308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233230C: 486DE565  bl 0x82a10870
	ctx.lr = 0x82332310;
	sub_82A10870(ctx, base);
	// 82332310: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233231C: 808BB04C  lwz r4, -0x4fb4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20404 as u32) ) } as u64;
	// 82332320: 48AC16E9  bl 0x82df3a08
	ctx.lr = 0x82332324;
	sub_82DF3A08(ctx, base);
	// 82332324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332328: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233232C: 4BFD55AD  bl 0x823078d8
	ctx.lr = 0x82332330;
	sub_823078D8(ctx, base);
	// 82332330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332334: 48AC10F5  bl 0x82df3428
	ctx.lr = 0x82332338;
	sub_82DF3428(ctx, base);
	// 82332338: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233233C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332340: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 82332344: 48AC16C5  bl 0x82df3a08
	ctx.lr = 0x82332348;
	sub_82DF3A08(ctx, base);
	// 82332348: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233234C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332354: 4BFD31ED  bl 0x82305540
	ctx.lr = 0x82332358;
	sub_82305540(ctx, base);
	// 82332358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233235C: 48AC10CD  bl 0x82df3428
	ctx.lr = 0x82332360;
	sub_82DF3428(ctx, base);
	// 82332360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82332364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233236C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332378 size=92
    let mut pc: u32 = 0x82332378;
    'dispatch: loop {
        match pc {
            0x82332378 => {
    //   block [0x82332378..0x823323D4)
	// 82332378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233237C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332384: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233238C: 48D95755  bl 0x830c7ae0
	ctx.lr = 0x82332390;
	sub_830C7AE0(ctx, base);
	// 82332390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332394: 486DE4DD  bl 0x82a10870
	ctx.lr = 0x82332398;
	sub_82A10870(ctx, base);
	// 82332398: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233239C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823323A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823323A4: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 823323A8: 48AC1661  bl 0x82df3a08
	ctx.lr = 0x823323AC;
	sub_82DF3A08(ctx, base);
	// 823323AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823323B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823323B4: 4BFD3195  bl 0x82305548
	ctx.lr = 0x823323B8;
	sub_82305548(ctx, base);
	// 823323B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823323BC: 48AC106D  bl 0x82df3428
	ctx.lr = 0x823323C0;
	sub_82DF3428(ctx, base);
	// 823323C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823323C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823323C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823323CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823323D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823323D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823323D8 size=60
    let mut pc: u32 = 0x823323D8;
    'dispatch: loop {
        match pc {
            0x823323D8 => {
    //   block [0x823323D8..0x82332414)
	// 823323D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823323DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823323E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823323E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823323E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823323EC: 4BFFF9ED  bl 0x82331dd8
	ctx.lr = 0x823323F0;
	sub_82331DD8(ctx, base);
	// 823323F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823323F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823323F8: 396B0D5C  addi r11, r11, 0xd5c
	ctx.r[11].s64 = ctx.r[11].s64 + 3420;
	// 823323FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82332400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82332404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233240C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332418 size=152
    let mut pc: u32 = 0x82332418;
    'dispatch: loop {
        match pc {
            0x82332418 => {
    //   block [0x82332418..0x823324B0)
	// 82332418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233241C: 48E75D51  bl 0x831a816c
	ctx.lr = 0x82332420;
	sub_831A8130(ctx, base);
	// 82332420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332428: 48D956B9  bl 0x830c7ae0
	ctx.lr = 0x8233242C;
	sub_830C7AE0(ctx, base);
	// 8233242C: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82332430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332434: 419A0048  beq cr6, 0x8233247c
	if ctx.cr[6].eq {
	pc = 0x8233247C; continue 'dispatch;
	}
	// 82332438: 48B4CFF9  bl 0x82e7f430
	ctx.lr = 0x8233243C;
	sub_82E7F430(ctx, base);
	// 8233243C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82332440: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82332444: 3BCB001C  addi r30, r11, 0x1c
	ctx.r[30].s64 = ctx.r[11].s64 + 28;
	// 82332448: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233244C: 83AB001C  lwz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82332450: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82332454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82332458: 4E800421  bctrl
	ctx.lr = 0x8233245C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233245C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82332460: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82332464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233246C: 4E800421  bctrl
	ctx.lr = 0x82332470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82332470: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82332474: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82332478: 48B4F2C9  bl 0x82e81740
	ctx.lr = 0x8233247C;
	sub_82E81740(ctx, base);
	// 8233247C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332480: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332484: 808BB4F4  lwz r4, -0x4b0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19212 as u32) ) } as u64;
	// 82332488: 48AC1581  bl 0x82df3a08
	ctx.lr = 0x8233248C;
	sub_82DF3A08(ctx, base);
	// 8233248C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332490: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82332494: 486DE3DD  bl 0x82a10870
	ctx.lr = 0x82332498;
	sub_82A10870(ctx, base);
	// 82332498: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233249C: 4BFD30AD  bl 0x82305548
	ctx.lr = 0x823324A0;
	sub_82305548(ctx, base);
	// 823324A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823324A4: 48AC0F85  bl 0x82df3428
	ctx.lr = 0x823324A8;
	sub_82DF3428(ctx, base);
	// 823324A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823324AC: 48E75D10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823324B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823324B0 size=240
    let mut pc: u32 = 0x823324B0;
    'dispatch: loop {
        match pc {
            0x823324B0 => {
    //   block [0x823324B0..0x823325A0)
	// 823324B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823324B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823324B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823324BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823324C0: 89630069  lbz r11, 0x69(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(105 as u32) ) } as u64;
	// 823324C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823324C8: 408200C0  bne 0x82332588
	if !ctx.cr[0].eq {
	pc = 0x82332588; continue 'dispatch;
	}
	// 823324CC: 486DE3A5  bl 0x82a10870
	ctx.lr = 0x823324D0;
	sub_82A10870(ctx, base);
	// 823324D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823324D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823324D8: 4BFD36C9  bl 0x82305ba0
	ctx.lr = 0x823324DC;
	sub_82305BA0(ctx, base);
	// 823324DC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 823324E0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 823324E4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 823324E8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823325A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823325A0 size=380
    let mut pc: u32 = 0x823325A0;
    'dispatch: loop {
        match pc {
            0x823325A0 => {
    //   block [0x823325A0..0x8233271C)
	// 823325A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823325A4: 48E75BC5  bl 0x831a8168
	ctx.lr = 0x823325A8;
	sub_831A8130(ctx, base);
	// 823325A8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 823325AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823325B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823325B4: 486DE2BD  bl 0x82a10870
	ctx.lr = 0x823325B8;
	sub_82A10870(ctx, base);
	// 823325B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823325BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823325C0: 9BBE0068  stb r29, 0x68(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[29].u8 ) };
	// 823325C4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 823325C8: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823325CC: 4812881D  bl 0x8245ade8
	ctx.lr = 0x823325D0;
	sub_8245ADE8(ctx, base);
	// 823325D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823325D4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823325D8: 4BFE4A09  bl 0x82316fe0
	ctx.lr = 0x823325DC;
	sub_82316FE0(ctx, base);
	// 823325DC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 823325E0: 40980064  bge cr6, 0x82332644
	if !ctx.cr[6].lt {
	pc = 0x82332644; continue 'dispatch;
	}
	// 823325E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823325E8: 4BFE0AE1  bl 0x823130c8
	ctx.lr = 0x823325EC;
	sub_823130C8(ctx, base);
	// 823325EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823325F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823325F4: 4BFD38B5  bl 0x82305ea8
	ctx.lr = 0x823325F8;
	sub_82305EA8(ctx, base);
	// 823325F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823325FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82332600: 4BFE0AB1  bl 0x823130b0
	ctx.lr = 0x82332604;
	sub_823130B0(ctx, base);
	// 82332604: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332608: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233260C: 41820010  beq 0x8233261c
	if ctx.cr[0].eq {
	pc = 0x8233261C; continue 'dispatch;
	}
	// 82332610: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332614: 808BB030  lwz r4, -0x4fd0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20432 as u32) ) } as u64;
	// 82332618: 4800000C  b 0x82332624
	pc = 0x82332624; continue 'dispatch;
	// 8233261C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332620: 808BB028  lwz r4, -0x4fd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20440 as u32) ) } as u64;
	// 82332624: 48AC13E5  bl 0x82df3a08
	ctx.lr = 0x82332628;
	sub_82DF3A08(ctx, base);
	// 82332628: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233262C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332630: 4BFD52A9  bl 0x823078d8
	ctx.lr = 0x82332634;
	sub_823078D8(ctx, base);
	// 82332634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332638: 48AC0DF1  bl 0x82df3428
	ctx.lr = 0x8233263C;
	sub_82DF3428(ctx, base);
	// 8233263C: 9BBE0069  stb r29, 0x69(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(105 as u32), ctx.r[29].u8 ) };
	// 82332640: 48000030  b 0x82332670
	pc = 0x82332670; continue 'dispatch;
	// 82332644: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233264C: 808BB03C  lwz r4, -0x4fc4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20420 as u32) ) } as u64;
	// 82332650: 48AC13B9  bl 0x82df3a08
	ctx.lr = 0x82332654;
	sub_82DF3A08(ctx, base);
	// 82332654: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233265C: 4BFD527D  bl 0x823078d8
	ctx.lr = 0x82332660;
	sub_823078D8(ctx, base);
	// 82332660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332664: 48AC0DC5  bl 0x82df3428
	ctx.lr = 0x82332668;
	sub_82DF3428(ctx, base);
	// 82332668: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8233266C: 997E0069  stb r11, 0x69(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(105 as u32), ctx.r[11].u8 ) };
	// 82332670: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332678: 808BB4F4  lwz r4, -0x4b0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19212 as u32) ) } as u64;
	// 8233267C: 48AC138D  bl 0x82df3a08
	ctx.lr = 0x82332680;
	sub_82DF3A08(ctx, base);
	// 82332680: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82332684: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233268C: 4BFD2EB5  bl 0x82305540
	ctx.lr = 0x82332690;
	sub_82305540(ctx, base);
	// 82332690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332694: 48AC0D95  bl 0x82df3428
	ctx.lr = 0x82332698;
	sub_82DF3428(ctx, base);
	// 82332698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233269C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823326A0: 388B0D90  addi r4, r11, 0xd90
	ctx.r[4].s64 = ctx.r[11].s64 + 3472;
	// 823326A4: 48AC1365  bl 0x82df3a08
	ctx.lr = 0x823326A8;
	sub_82DF3A08(ctx, base);
	// 823326A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823326AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823326B0: 388B0D84  addi r4, r11, 0xd84
	ctx.r[4].s64 = ctx.r[11].s64 + 3460;
	// 823326B4: 48AC1355  bl 0x82df3a08
	ctx.lr = 0x823326B8;
	sub_82DF3A08(ctx, base);
	// 823326B8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 823326BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823326C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823326C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823326C8: 4BFD6A11  bl 0x823090d8
	ctx.lr = 0x823326CC;
	sub_823090D8(ctx, base);
	// 823326CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823326D0: 395E006C  addi r10, r30, 0x6c
	ctx.r[10].s64 = ctx.r[30].s64 + 108;
	// 823326D4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 823326D8: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 823326DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823326E0: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 823326E4: 4BF91D7D  bl 0x822c4460
	ctx.lr = 0x823326E8;
	sub_822C4460(ctx, base);
	// 823326E8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823326EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823326F0: 419A0008  beq cr6, 0x823326f8
	if ctx.cr[6].eq {
	pc = 0x823326F8; continue 'dispatch;
	}
	// 823326F4: 4BF8E19D  bl 0x822c0890
	ctx.lr = 0x823326F8;
	sub_822C0890(ctx, base);
	// 823326F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823326FC: 48AC0D2D  bl 0x82df3428
	ctx.lr = 0x82332700;
	sub_82DF3428(ctx, base);
	// 82332700: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82332704: 48AC0D25  bl 0x82df3428
	ctx.lr = 0x82332708;
	sub_82DF3428(ctx, base);
	// 82332708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233270C: 48D953D5  bl 0x830c7ae0
	ctx.lr = 0x82332710;
	sub_830C7AE0(ctx, base);
	// 82332710: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82332714: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82332718: 48E75AA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82332720 size=596
    let mut pc: u32 = 0x82332720;
    'dispatch: loop {
        match pc {
            0x82332720 => {
    //   block [0x82332720..0x82332974)
	// 82332720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332724: 48E75A49  bl 0x831a816c
	ctx.lr = 0x82332728;
	sub_831A8130(ctx, base);
	// 82332728: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233272C: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332978 size=84
    let mut pc: u32 = 0x82332978;
    'dispatch: loop {
        match pc {
            0x82332978 => {
    //   block [0x82332978..0x823329CC)
	// 82332978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332984: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233298C: 4BFFF44D  bl 0x82331dd8
	ctx.lr = 0x82332990;
	sub_82331DD8(ctx, base);
	// 82332990: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82332994: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82332998: 394A0DA8  addi r10, r10, 0xda8
	ctx.r[10].s64 = ctx.r[10].s64 + 3496;
	// 8233299C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823329A0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823329A4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 823329A8: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 823329AC: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 823329B0: 4BFFF419  bl 0x82331dc8
	ctx.lr = 0x823329B4;
	sub_82331DC8(ctx, base);
	// 823329B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823329B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823329BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823329C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823329C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823329C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823329D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823329D0 size=96
    let mut pc: u32 = 0x823329D0;
    'dispatch: loop {
        match pc {
            0x823329D0 => {
    //   block [0x823329D0..0x82332A30)
	// 823329D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823329D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823329D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823329DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823329E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823329E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823329E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823329EC: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 823329F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823329F4: 419A0008  beq cr6, 0x823329fc
	if ctx.cr[6].eq {
	pc = 0x823329FC; continue 'dispatch;
	}
	// 823329F8: 4BF8DE99  bl 0x822c0890
	ctx.lr = 0x823329FC;
	sub_822C0890(ctx, base);
	// 823329FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332A00: 4BFFF469  bl 0x82331e68
	ctx.lr = 0x82332A04;
	sub_82331E68(ctx, base);
	// 82332A04: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332A08: 4182000C  beq 0x82332a14
	if ctx.cr[0].eq {
	pc = 0x82332A14; continue 'dispatch;
	}
	// 82332A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332A10: 48ABF9C9  bl 0x82df23d8
	ctx.lr = 0x82332A14;
	sub_82DF23D8(ctx, base);
	// 82332A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82332A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332A24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82332A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332A30 size=116
    let mut pc: u32 = 0x82332A30;
    'dispatch: loop {
        match pc {
            0x82332A30 => {
    //   block [0x82332A30..0x82332AA4)
	// 82332A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332A34: 48E75739  bl 0x831a816c
	ctx.lr = 0x82332A38;
	sub_831A8130(ctx, base);
	// 82332A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332A40: 486DDE31  bl 0x82a10870
	ctx.lr = 0x82332A44;
	sub_82A10870(ctx, base);
	// 82332A44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332A48: 4BFD50E1  bl 0x82307b28
	ctx.lr = 0x82332A4C;
	sub_82307B28(ctx, base);
	// 82332A4C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332A50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332A54: 808BB4F4  lwz r4, -0x4b0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19212 as u32) ) } as u64;
	// 82332A58: 48AC0FB1  bl 0x82df3a08
	ctx.lr = 0x82332A5C;
	sub_82DF3A08(ctx, base);
	// 82332A5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332A60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82332A64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332A68: 4BFD2AD9  bl 0x82305540
	ctx.lr = 0x82332A6C;
	sub_82305540(ctx, base);
	// 82332A6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82332A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332A74: 48AC09B5  bl 0x82df3428
	ctx.lr = 0x82332A78;
	sub_82DF3428(ctx, base);
	// 82332A78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82332A80: C02B9524  lfs f1, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82332A84: 480133C5  bl 0x82345e48
	ctx.lr = 0x82332A88;
	sub_82345E48(ctx, base);
	// 82332A88: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82332A8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332A90: 4BFE1259  bl 0x82313ce8
	ctx.lr = 0x82332A94;
	sub_82313CE8(ctx, base);
	// 82332A94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332A98: 48D95049  bl 0x830c7ae0
	ctx.lr = 0x82332A9C;
	sub_830C7AE0(ctx, base);
	// 82332A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82332AA0: 48E7571C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332AA8 size=112
    let mut pc: u32 = 0x82332AA8;
    'dispatch: loop {
        match pc {
            0x82332AA8 => {
    //   block [0x82332AA8..0x82332B18)
	// 82332AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82332AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332AC0: 486DDDB1  bl 0x82a10870
	ctx.lr = 0x82332AC4;
	sub_82A10870(ctx, base);
	// 82332AC4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332AC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332ACC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332AD0: 808BB4F4  lwz r4, -0x4b0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19212 as u32) ) } as u64;
	// 82332AD4: 48AC0F35  bl 0x82df3a08
	ctx.lr = 0x82332AD8;
	sub_82DF3A08(ctx, base);
	// 82332AD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332ADC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332AE0: 4BFD2A69  bl 0x82305548
	ctx.lr = 0x82332AE4;
	sub_82305548(ctx, base);
	// 82332AE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332AE8: 48AC0941  bl 0x82df3428
	ctx.lr = 0x82332AEC;
	sub_82DF3428(ctx, base);
	// 82332AEC: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82332AF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332AF4: 4BFE11FD  bl 0x82313cf0
	ctx.lr = 0x82332AF8;
	sub_82313CF0(ctx, base);
	// 82332AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332AFC: 48D94FE5  bl 0x830c7ae0
	ctx.lr = 0x82332B00;
	sub_830C7AE0(ctx, base);
	// 82332B00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82332B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332B0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82332B10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332B18 size=260
    let mut pc: u32 = 0x82332B18;
    'dispatch: loop {
        match pc {
            0x82332B18 => {
    //   block [0x82332B18..0x82332C1C)
	// 82332B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82332B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332B30: 486DDD41  bl 0x82a10870
	ctx.lr = 0x82332B34;
	sub_82A10870(ctx, base);
	// 82332B34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82332B38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332B3C: 4BFE1E7D  bl 0x823149b8
	ctx.lr = 0x82332B40;
	sub_823149B8(ctx, base);
	// 82332B40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332B44: 41820048  beq 0x82332b8c
	if ctx.cr[0].eq {
	pc = 0x82332B8C; continue 'dispatch;
	}
	// 82332B48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332B50: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82332B54: 48AC0EB5  bl 0x82df3a08
	ctx.lr = 0x82332B58;
	sub_82DF3A08(ctx, base);
	// 82332B58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82332B5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82332B60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332B64: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82332B68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82332B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82332B70: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82332B74: 48B2BF1D  bl 0x82e5ea90
	ctx.lr = 0x82332B78;
	sub_82E5EA90(ctx, base);
	// 82332B78: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82332B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332B80: 419A0084  beq cr6, 0x82332c04
	if ctx.cr[6].eq {
	pc = 0x82332C04; continue 'dispatch;
	}
	// 82332B84: 4BF8DD0D  bl 0x822c0890
	ctx.lr = 0x82332B88;
	sub_822C0890(ctx, base);
	// 82332B88: 4800007C  b 0x82332c04
	pc = 0x82332C04; continue 'dispatch;
	// 82332B8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332B90: 4BFD2899  bl 0x82305428
	ctx.lr = 0x82332B94;
	sub_82305428(ctx, base);
	// 82332B94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332B98: 41820044  beq 0x82332bdc
	if ctx.cr[0].eq {
	pc = 0x82332BDC; continue 'dispatch;
	}
	// 82332B9C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332BA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332BA4: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82332BA8: 48AC0E61  bl 0x82df3a08
	ctx.lr = 0x82332BAC;
	sub_82DF3A08(ctx, base);
	// 82332BAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82332BB0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82332BB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332BB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82332BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82332BC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82332BC4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82332BC8: 48B2BEC9  bl 0x82e5ea90
	ctx.lr = 0x82332BCC;
	sub_82E5EA90(ctx, base);
	// 82332BCC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82332BD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332BD4: 419A0008  beq cr6, 0x82332bdc
	if ctx.cr[6].eq {
	pc = 0x82332BDC; continue 'dispatch;
	}
	// 82332BD8: 4BF8DCB9  bl 0x822c0890
	ctx.lr = 0x82332BDC;
	sub_822C0890(ctx, base);
	// 82332BDC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82332BE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332BE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332BE8: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82332BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82332BF0: 4E800421  bctrl
	ctx.lr = 0x82332BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82332BF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332BF8: 4082000C  bne 0x82332c04
	if !ctx.cr[0].eq {
	pc = 0x82332C04; continue 'dispatch;
	}
	// 82332BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332C00: 48D94EE1  bl 0x830c7ae0
	ctx.lr = 0x82332C04;
	sub_830C7AE0(ctx, base);
	// 82332C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82332C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332C10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82332C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332C20 size=72
    let mut pc: u32 = 0x82332C20;
    'dispatch: loop {
        match pc {
            0x82332C20 => {
    //   block [0x82332C20..0x82332C68)
	// 82332C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332C28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332C2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332C30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332C34: 4BFFF1A5  bl 0x82331dd8
	ctx.lr = 0x82332C38;
	sub_82331DD8(ctx, base);
	// 82332C38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332C3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332C40: 396B0DD4  addi r11, r11, 0xdd4
	ctx.r[11].s64 = ctx.r[11].s64 + 3540;
	// 82332C44: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82332C48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82332C4C: 4BFFF17D  bl 0x82331dc8
	ctx.lr = 0x82332C50;
	sub_82331DC8(ctx, base);
	// 82332C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332C54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82332C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332C60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332C68 size=212
    let mut pc: u32 = 0x82332C68;
    'dispatch: loop {
        match pc {
            0x82332C68 => {
    //   block [0x82332C68..0x82332D3C)
	// 82332C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332C6C: 48E75501  bl 0x831a816c
	ctx.lr = 0x82332C70;
	sub_831A8130(ctx, base);
	// 82332C70: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82332C74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332C7C: 486DDBF5  bl 0x82a10870
	ctx.lr = 0x82332C80;
	sub_82A10870(ctx, base);
	// 82332C80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332C84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332C88: 388B0E00  addi r4, r11, 0xe00
	ctx.r[4].s64 = ctx.r[11].s64 + 3584;
	// 82332C8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332C90: 48AC0D79  bl 0x82df3a08
	ctx.lr = 0x82332C94;
	sub_82DF3A08(ctx, base);
	// 82332C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332C98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332C9C: 4BFD4EDD  bl 0x82307b78
	ctx.lr = 0x82332CA0;
	sub_82307B78(ctx, base);
	// 82332CA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332CA4: 48AC0785  bl 0x82df3428
	ctx.lr = 0x82332CA8;
	sub_82DF3428(ctx, base);
	// 82332CA8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332CAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332CB0: 808BB4F4  lwz r4, -0x4b0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19212 as u32) ) } as u64;
	// 82332CB4: 48AC0D55  bl 0x82df3a08
	ctx.lr = 0x82332CB8;
	sub_82DF3A08(ctx, base);
	// 82332CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332CBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82332CC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332CC4: 4BFD287D  bl 0x82305540
	ctx.lr = 0x82332CC8;
	sub_82305540(ctx, base);
	// 82332CC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82332CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332CD0: 48AC0759  bl 0x82df3428
	ctx.lr = 0x82332CD4;
	sub_82DF3428(ctx, base);
	// 82332CD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332CD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82332CDC: C02B9524  lfs f1, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82332CE0: 48013169  bl 0x82345e48
	ctx.lr = 0x82332CE4;
	sub_82345E48(ctx, base);
	// 82332CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332CE8: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82332CEC: 4BFE0FFD  bl 0x82313ce8
	ctx.lr = 0x82332CF0;
	sub_82313CE8(ctx, base);
	// 82332CF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82332CF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82332CF8: 4BFD5F69  bl 0x82308c60
	ctx.lr = 0x82332CFC;
	sub_82308C60(ctx, base);
	// 82332CFC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82332D00: 4BFC6C71  bl 0x822f9970
	ctx.lr = 0x82332D04;
	sub_822F9970(ctx, base);
	// 82332D04: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82332D08: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82332D0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332D10: 419A0008  beq cr6, 0x82332d18
	if ctx.cr[6].eq {
	pc = 0x82332D18; continue 'dispatch;
	}
	// 82332D14: 4BF8DB7D  bl 0x822c0890
	ctx.lr = 0x82332D18;
	sub_822C0890(ctx, base);
	// 82332D18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332D20: C00B0DFC  lfs f0, 0xdfc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82332D24: EC00F824  fdivs f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[31].f64) as f32) as f64;
	// 82332D28: D01F0068  stfs f0, 0x68(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82332D2C: 48D94DB5  bl 0x830c7ae0
	ctx.lr = 0x82332D30;
	sub_830C7AE0(ctx, base);
	// 82332D30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82332D34: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82332D38: 48E75484  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332D40 size=356
    let mut pc: u32 = 0x82332D40;
    'dispatch: loop {
        match pc {
            0x82332D40 => {
    //   block [0x82332D40..0x82332EA4)
	// 82332D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332D48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82332D4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332D50: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82332D54: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332D58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332D5C: 486DDB15  bl 0x82a10870
	ctx.lr = 0x82332D60;
	sub_82A10870(ctx, base);
	// 82332D60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332D64: 4BFD26C5  bl 0x82305428
	ctx.lr = 0x82332D68;
	sub_82305428(ctx, base);
	// 82332D68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82332D6C: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82332D70: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82332D74: 41820040  beq 0x82332db4
	if ctx.cr[0].eq {
	pc = 0x82332DB4; continue 'dispatch;
	}
	// 82332D78: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332D7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332D80: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82332D84: 48AC0C85  bl 0x82df3a08
	ctx.lr = 0x82332D88;
	sub_82DF3A08(ctx, base);
	// 82332D88: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82332D8C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82332D90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332D94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82332D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82332D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82332DA0: 48B2BCF1  bl 0x82e5ea90
	ctx.lr = 0x82332DA4;
	sub_82E5EA90(ctx, base);
	// 82332DA4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82332DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82332DAC: 419A0008  beq cr6, 0x82332db4
	if ctx.cr[6].eq {
	pc = 0x82332DB4; continue 'dispatch;
	}
	// 82332DB0: 4BF8DAE1  bl 0x822c0890
	ctx.lr = 0x82332DB4;
	sub_822C0890(ctx, base);
	// 82332DB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82332DB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82332DBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332DC0: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82332DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82332DC8: 4E800421  bctrl
	ctx.lr = 0x82332DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82332DCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332DD0: 408200B8  bne 0x82332e88
	if !ctx.cr[0].eq {
	pc = 0x82332E88; continue 'dispatch;
	}
	// 82332DD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82332DD8: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82332DDC: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82332DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332DE4: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82332DE8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82332DEC: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82332DF0: 48B26989  bl 0x82e59778
	ctx.lr = 0x82332DF4;
	sub_82E59778(ctx, base);
	// 82332DF4: C01F0068  lfs f0, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82332DF8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82332DFC: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82332E00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82332E04: 48B49E3D  bl 0x82e7cc40
	ctx.lr = 0x82332E08;
	sub_82E7CC40(ctx, base);
	// 82332E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332E0C: 4BFDFED5  bl 0x82312ce0
	ctx.lr = 0x82332E10;
	sub_82312CE0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82332EA8 size=52
    let mut pc: u32 = 0x82332EA8;
    'dispatch: loop {
        match pc {
            0x82332EA8 => {
    //   block [0x82332EA8..0x82332EDC)
	// 82332EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332EBC: 486DD9B5  bl 0x82a10870
	ctx.lr = 0x82332EC0;
	sub_82A10870(ctx, base);
	// 82332EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332EC4: 4800033D  bl 0x82333200
	ctx.lr = 0x82332EC8;
	sub_82333200(ctx, base);
	// 82332EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82332ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332EE0 size=172
    let mut pc: u32 = 0x82332EE0;
    'dispatch: loop {
        match pc {
            0x82332EE0 => {
    //   block [0x82332EE0..0x82332F8C)
	// 82332EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82332EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332EF8: 486DD979  bl 0x82a10870
	ctx.lr = 0x82332EFC;
	sub_82A10870(ctx, base);
	// 82332EFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82332F00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332F04: 997F006C  stb r11, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 82332F08: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 82332F0C: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 82332F10: 48127ED9  bl 0x8245ade8
	ctx.lr = 0x82332F14;
	sub_8245ADE8(ctx, base);
	// 82332F14: D03F0068  stfs f1, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82332F18: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332F1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332F20: 808BB094  lwz r4, -0x4f6c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20332 as u32) ) } as u64;
	// 82332F24: 48AC0AE5  bl 0x82df3a08
	ctx.lr = 0x82332F28;
	sub_82DF3A08(ctx, base);
	// 82332F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332F2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332F30: 4BFD49A9  bl 0x823078d8
	ctx.lr = 0x82332F34;
	sub_823078D8(ctx, base);
	// 82332F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332F38: 48AC04F1  bl 0x82df3428
	ctx.lr = 0x82332F3C;
	sub_82DF3428(ctx, base);
	// 82332F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332F44: 388B0E08  addi r4, r11, 0xe08
	ctx.r[4].s64 = ctx.r[11].s64 + 3592;
	// 82332F48: 48AC0AC1  bl 0x82df3a08
	ctx.lr = 0x82332F4C;
	sub_82DF3A08(ctx, base);
	// 82332F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332F50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82332F54: 4BFD2D8D  bl 0x82305ce0
	ctx.lr = 0x82332F58;
	sub_82305CE0(ctx, base);
	// 82332F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332F5C: 48AC04CD  bl 0x82df3428
	ctx.lr = 0x82332F60;
	sub_82DF3428(ctx, base);
	// 82332F60: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82332F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332F68: 4BFE0D81  bl 0x82313ce8
	ctx.lr = 0x82332F6C;
	sub_82313CE8(ctx, base);
	// 82332F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332F70: 480002E1  bl 0x82333250
	ctx.lr = 0x82332F74;
	sub_82333250(ctx, base);
	// 82332F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82332F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82332F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82332F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82332F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82332F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82332F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82332F90 size=264
    let mut pc: u32 = 0x82332F90;
    'dispatch: loop {
        match pc {
            0x82332F90 => {
    //   block [0x82332F90..0x82333098)
	// 82332F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82332F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82332F98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82332F9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82332FA0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82332FA4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82332FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82332FAC: 486DD8C5  bl 0x82a10870
	ctx.lr = 0x82332FB0;
	sub_82A10870(ctx, base);
	// 82332FB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82332FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82332FB8: 48000211  bl 0x823331c8
	ctx.lr = 0x82332FBC;
	sub_823331C8(ctx, base);
	// 82332FBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332FC0: 4BFDFC49  bl 0x82312c08
	ctx.lr = 0x82332FC4;
	sub_82312C08(ctx, base);
	// 82332FC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82332FC8: 41820054  beq 0x8233301c
	if ctx.cr[0].eq {
	pc = 0x8233301C; continue 'dispatch;
	}
	// 82332FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82332FD0: 4BFD24E1  bl 0x823054b0
	ctx.lr = 0x82332FD4;
	sub_823054B0(ctx, base);
	// 82332FD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82332FD8: C00B0A98  lfs f0, 0xa98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82332FDC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82332FE0: 4099003C  ble cr6, 0x8233301c
	if !ctx.cr[6].gt {
	pc = 0x8233301C; continue 'dispatch;
	}
	// 82332FE4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82332FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82332FEC: 808BB3D4  lwz r4, -0x4c2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19500 as u32) ) } as u64;
	// 82332FF0: 48AC0A19  bl 0x82df3a08
	ctx.lr = 0x82332FF4;
	sub_82DF3A08(ctx, base);
	// 82332FF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82332FF8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82332FFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333000: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82333004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82333008: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233300C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333010: 48B2BA81  bl 0x82e5ea90
	ctx.lr = 0x82333014;
	sub_82E5EA90(ctx, base);
	// 82333014: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82333018: 48000058  b 0x82333070
	pc = 0x82333070; continue 'dispatch;
	// 8233301C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82333020: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 82333024: 48127DC5  bl 0x8245ade8
	ctx.lr = 0x82333028;
	sub_8245ADE8(ctx, base);
	// 82333028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233302C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82333030: 48B26731  bl 0x82e59760
	ctx.lr = 0x82333034;
	sub_82E59760(ctx, base);
	// 82333034: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82333038: 40990044  ble cr6, 0x8233307c
	if !ctx.cr[6].gt {
	pc = 0x8233307C; continue 'dispatch;
	}
	// 8233303C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333044: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 82333048: 48AC09C1  bl 0x82df3a08
	ctx.lr = 0x8233304C;
	sub_82DF3A08(ctx, base);
	// 8233304C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82333050: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82333054: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333058: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233305C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82333060: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82333064: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333068: 48B2BA29  bl 0x82e5ea90
	ctx.lr = 0x8233306C;
	sub_82E5EA90(ctx, base);
	// 8233306C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82333070: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82333074: 419A0008  beq cr6, 0x8233307c
	if ctx.cr[6].eq {
	pc = 0x8233307C; continue 'dispatch;
	}
	// 82333078: 4BF8D819  bl 0x822c0890
	ctx.lr = 0x8233307C;
	sub_822C0890(ctx, base);
	// 8233307C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82333080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333088: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8233308C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333098 size=100
    let mut pc: u32 = 0x82333098;
    'dispatch: loop {
        match pc {
            0x82333098 => {
    //   block [0x82333098..0x823330FC)
	// 82333098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233309C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823330A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823330A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823330A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823330AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823330B0: 486DD7C1  bl 0x82a10870
	ctx.lr = 0x823330B4;
	sub_82A10870(ctx, base);
	// 823330B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823330B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823330BC: 480002FD  bl 0x823333b8
	ctx.lr = 0x823330C0;
	sub_823333B8(ctx, base);
	// 823330C0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823330C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823330C8: 808BB09C  lwz r4, -0x4f64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20324 as u32) ) } as u64;
	// 823330CC: 48AC093D  bl 0x82df3a08
	ctx.lr = 0x823330D0;
	sub_82DF3A08(ctx, base);
	// 823330D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823330D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823330D8: 4BFD4801  bl 0x823078d8
	ctx.lr = 0x823330DC;
	sub_823078D8(ctx, base);
	// 823330DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823330E0: 48AC0349  bl 0x82df3428
	ctx.lr = 0x823330E4;
	sub_82DF3428(ctx, base);
	// 823330E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823330E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823330EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823330F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823330F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823330F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82333100 size=4
    let mut pc: u32 = 0x82333100;
    'dispatch: loop {
        match pc {
            0x82333100 => {
    //   block [0x82333100..0x82333104)
	// 82333100: 480003A0  b 0x823334a0
	sub_823334A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333108 size=192
    let mut pc: u32 = 0x82333108;
    'dispatch: loop {
        match pc {
            0x82333108 => {
    //   block [0x82333108..0x823331C8)
	// 82333108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82333114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333118: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233311C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333124: 486DD74D  bl 0x82a10870
	ctx.lr = 0x82333128;
	sub_82A10870(ctx, base);
	// 82333128: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233312C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333130: 480002E9  bl 0x82333418
	ctx.lr = 0x82333134;
	sub_82333418(ctx, base);
	// 82333134: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82333138: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233313C: 48127CAD  bl 0x8245ade8
	ctx.lr = 0x82333140;
	sub_8245ADE8(ctx, base);
	// 82333140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333144: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82333148: 48B26619  bl 0x82e59760
	ctx.lr = 0x8233314C;
	sub_82E59760(ctx, base);
	// 8233314C: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82333150: 4099005C  ble cr6, 0x823331ac
	if !ctx.cr[6].gt {
	pc = 0x823331AC; continue 'dispatch;
	}
	// 82333154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333158: 4BFE3E89  bl 0x82316fe0
	ctx.lr = 0x8233315C;
	sub_82316FE0(ctx, base);
	// 8233315C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333160: C00B964C  lfs f0, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82333164: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82333168: 40980044  bge cr6, 0x823331ac
	if !ctx.cr[6].lt {
	pc = 0x823331AC; continue 'dispatch;
	}
	// 8233316C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333174: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82333178: 48AC0891  bl 0x82df3a08
	ctx.lr = 0x8233317C;
	sub_82DF3A08(ctx, base);
	// 8233317C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82333180: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82333184: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333188: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233318C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82333190: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82333194: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333198: 48B2B8F9  bl 0x82e5ea90
	ctx.lr = 0x8233319C;
	sub_82E5EA90(ctx, base);
	// 8233319C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823331A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823331A4: 419A0008  beq cr6, 0x823331ac
	if ctx.cr[6].eq {
	pc = 0x823331AC; continue 'dispatch;
	}
	// 823331A8: 4BF8D6E9  bl 0x822c0890
	ctx.lr = 0x823331AC;
	sub_822C0890(ctx, base);
	// 823331AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823331B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823331B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823331B8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823331BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823331C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823331C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823331C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823331C8 size=52
    let mut pc: u32 = 0x823331C8;
    'dispatch: loop {
        match pc {
            0x823331C8 => {
    //   block [0x823331C8..0x823331FC)
	// 823331C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823331CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823331D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823331D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823331D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823331DC: 486DD695  bl 0x82a10870
	ctx.lr = 0x823331E0;
	sub_82A10870(ctx, base);
	// 823331E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823331E4: 48D948FD  bl 0x830c7ae0
	ctx.lr = 0x823331E8;
	sub_830C7AE0(ctx, base);
	// 823331E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823331EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823331F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823331F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823331F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333200 size=76
    let mut pc: u32 = 0x82333200;
    'dispatch: loop {
        match pc {
            0x82333200 => {
    //   block [0x82333200..0x8233324C)
	// 82333200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233320C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333218: 486DD659  bl 0x82a10870
	ctx.lr = 0x8233321C;
	sub_82A10870(ctx, base);
	// 8233321C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333224: 48D948BD  bl 0x830c7ae0
	ctx.lr = 0x82333228;
	sub_830C7AE0(ctx, base);
	// 82333228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233322C: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82333230: 4BFE7829  bl 0x8231aa58
	ctx.lr = 0x82333234;
	sub_8231AA58(ctx, base);
	// 82333234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333250 size=284
    let mut pc: u32 = 0x82333250;
    'dispatch: loop {
        match pc {
            0x82333250 => {
    //   block [0x82333250..0x8233336C)
	// 82333250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233325C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333260: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333268: 486DD609  bl 0x82a10870
	ctx.lr = 0x8233326C;
	sub_82A10870(ctx, base);
	// 8233326C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333274: 48D9486D  bl 0x830c7ae0
	ctx.lr = 0x82333278;
	sub_830C7AE0(ctx, base);
	// 82333278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233327C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333280: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333284: 4BFFB865  bl 0x8232eae8
	ctx.lr = 0x82333288;
	sub_8232EAE8(ctx, base);
	// 82333288: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 8233328C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82333290: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333294: 4BFD4A2D  bl 0x82307cc0
	ctx.lr = 0x82333298;
	sub_82307CC0(ctx, base);
	// 82333298: 39600068  li r11, 0x68
	ctx.r[11].s64 = 104;
	// 8233329C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 823332A0: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823332A4: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823332A8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 823332AC: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823332B0: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 823332B4: C163000C  lfs f11, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823332B8: FD806050  fneg f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 823332BC: 13FF5C07  vcmpneb. (lvlx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823332C0: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333370 size=68
    let mut pc: u32 = 0x82333370;
    'dispatch: loop {
        match pc {
            0x82333370 => {
    //   block [0x82333370..0x823333B4)
	// 82333370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233337C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333384: 4BFFEA55  bl 0x82331dd8
	ctx.lr = 0x82333388;
	sub_82331DD8(ctx, base);
	// 82333388: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233338C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82333390: 396B0E7C  addi r11, r11, 0xe7c
	ctx.r[11].s64 = ctx.r[11].s64 + 3708;
	// 82333394: 995F006C  stb r10, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u8 ) };
	// 82333398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233339C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823333A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823333A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823333A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823333AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823333B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823333B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823333B8 size=96
    let mut pc: u32 = 0x823333B8;
    'dispatch: loop {
        match pc {
            0x823333B8 => {
    //   block [0x823333B8..0x82333418)
	// 823333B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823333BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823333C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823333C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823333C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823333CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823333D0: 486DD4A1  bl 0x82a10870
	ctx.lr = 0x823333D4;
	sub_82A10870(ctx, base);
	// 823333D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823333D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823333DC: 48D94705  bl 0x830c7ae0
	ctx.lr = 0x823333E0;
	sub_830C7AE0(ctx, base);
	// 823333E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823333E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823333E8: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823333EC: 4BFFB6FD  bl 0x8232eae8
	ctx.lr = 0x823333F0;
	sub_8232EAE8(ctx, base);
	// 823333F0: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 823333F4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 823333F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823333FC: 4BFE08ED  bl 0x82313ce8
	ctx.lr = 0x82333400;
	sub_82313CE8(ctx, base);
	// 82333400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233340C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333418 size=136
    let mut pc: u32 = 0x82333418;
    'dispatch: loop {
        match pc {
            0x82333418 => {
    //   block [0x82333418..0x823334A0)
	// 82333418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82333424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233342C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333430: 486DD441  bl 0x82a10870
	ctx.lr = 0x82333434;
	sub_82A10870(ctx, base);
	// 82333434: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233343C: 48D946A5  bl 0x830c7ae0
	ctx.lr = 0x82333440;
	sub_830C7AE0(ctx, base);
	// 82333440: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82333444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333448: 4BFE66A1  bl 0x82319ae8
	ctx.lr = 0x8233344C;
	sub_82319AE8(ctx, base);
	// 8233344C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333450: 48B26329  bl 0x82e59778
	ctx.lr = 0x82333454;
	sub_82E59778(ctx, base);
	// 82333454: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333458: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8233345C: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82333460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333464: 388A6910  addi r4, r10, 0x6910
	ctx.r[4].s64 = ctx.r[10].s64 + 26896;
	// 82333468: C02B0F0C  lfs f1, 0xf0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3852 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233346C: 48164B35  bl 0x82497fa0
	ctx.lr = 0x82333470;
	sub_82497FA0(ctx, base);
	// 82333470: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333474: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82333478: 38AB0EA8  addi r5, r11, 0xea8
	ctx.r[5].s64 = ctx.r[11].s64 + 3752;
	// 8233347C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82333480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333484: 4BFE06B5  bl 0x82313b38
	ctx.lr = 0x82333488;
	sub_82313B38(ctx, base);
	// 82333488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233348C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233349C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823334A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823334A0 size=76
    let mut pc: u32 = 0x823334A0;
    'dispatch: loop {
        match pc {
            0x823334A0 => {
    //   block [0x823334A0..0x823334EC)
	// 823334A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823334A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823334A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823334AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823334B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823334B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823334B8: 486DD3B9  bl 0x82a10870
	ctx.lr = 0x823334BC;
	sub_82A10870(ctx, base);
	// 823334BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823334C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823334C4: 48D9461D  bl 0x830c7ae0
	ctx.lr = 0x823334C8;
	sub_830C7AE0(ctx, base);
	// 823334C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823334CC: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 823334D0: 4BFE7589  bl 0x8231aa58
	ctx.lr = 0x823334D4;
	sub_8231AA58(ctx, base);
	// 823334D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823334D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823334DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823334E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823334E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823334E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823334F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823334F0 size=60
    let mut pc: u32 = 0x823334F0;
    'dispatch: loop {
        match pc {
            0x823334F0 => {
    //   block [0x823334F0..0x8233352C)
	// 823334F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823334F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823334F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823334FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333504: 4BFFE8D5  bl 0x82331dd8
	ctx.lr = 0x82333508;
	sub_82331DD8(ctx, base);
	// 82333508: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233350C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333510: 396B0F14  addi r11, r11, 0xf14
	ctx.r[11].s64 = ctx.r[11].s64 + 3860;
	// 82333514: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82333518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233351C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82333530 size=4
    let mut pc: u32 = 0x82333530;
    'dispatch: loop {
        match pc {
            0x82333530 => {
    //   block [0x82333530..0x82333534)
	// 82333530: 486DD340  b 0x82a10870
	sub_82A10870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333538 size=132
    let mut pc: u32 = 0x82333538;
    'dispatch: loop {
        match pc {
            0x82333538 => {
    //   block [0x82333538..0x823335BC)
	// 82333538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333544: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333548: 486DD329  bl 0x82a10870
	ctx.lr = 0x8233354C;
	sub_82A10870(ctx, base);
	// 8233354C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333554: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333558: 808BB174  lwz r4, -0x4e8c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20108 as u32) ) } as u64;
	// 8233355C: 48AC04AD  bl 0x82df3a08
	ctx.lr = 0x82333560;
	sub_82DF3A08(ctx, base);
	// 82333560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333564: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82333568: 4BFD4371  bl 0x823078d8
	ctx.lr = 0x8233356C;
	sub_823078D8(ctx, base);
	// 8233356C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333570: 48ABFEB9  bl 0x82df3428
	ctx.lr = 0x82333574;
	sub_82DF3428(ctx, base);
	// 82333574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233357C: C02BD5B8  lfs f1, -0x2a48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10824 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333580: 4BFE1BA1  bl 0x82315120
	ctx.lr = 0x82333584;
	sub_82315120(ctx, base);
	// 82333584: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233358C: 808BB4D8  lwz r4, -0x4b28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19240 as u32) ) } as u64;
	// 82333590: 48AC0479  bl 0x82df3a08
	ctx.lr = 0x82333594;
	sub_82DF3A08(ctx, base);
	// 82333594: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82333598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233359C: 4BFD1F8D  bl 0x82305528
	ctx.lr = 0x823335A0;
	sub_82305528(ctx, base);
	// 823335A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823335A4: 48ABFE85  bl 0x82df3428
	ctx.lr = 0x823335A8;
	sub_82DF3428(ctx, base);
	// 823335A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823335AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823335B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823335B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823335B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823335C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823335C0 size=92
    let mut pc: u32 = 0x823335C0;
    'dispatch: loop {
        match pc {
            0x823335C0 => {
    //   block [0x823335C0..0x8233361C)
	// 823335C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823335C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823335C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823335CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823335D0: 486DD2A1  bl 0x82a10870
	ctx.lr = 0x823335D4;
	sub_82A10870(ctx, base);
	// 823335D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823335D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823335DC: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823335E0: 4BFE1B41  bl 0x82315120
	ctx.lr = 0x823335E4;
	sub_82315120(ctx, base);
	// 823335E4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823335E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823335EC: 808BB4D8  lwz r4, -0x4b28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19240 as u32) ) } as u64;
	// 823335F0: 48AC0419  bl 0x82df3a08
	ctx.lr = 0x823335F4;
	sub_82DF3A08(ctx, base);
	// 823335F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823335F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823335FC: 4BFD1F35  bl 0x82305530
	ctx.lr = 0x82333600;
	sub_82305530(ctx, base);
	// 82333600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333604: 48ABFE25  bl 0x82df3428
	ctx.lr = 0x82333608;
	sub_82DF3428(ctx, base);
	// 82333608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233360C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333620 size=112
    let mut pc: u32 = 0x82333620;
    'dispatch: loop {
        match pc {
            0x82333620 => {
    //   block [0x82333620..0x82333690)
	// 82333620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233362C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333630: 486DD241  bl 0x82a10870
	ctx.lr = 0x82333634;
	sub_82A10870(ctx, base);
	// 82333634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333638: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233363C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333640: 4BFE6669  bl 0x82319ca8
	ctx.lr = 0x82333644;
	sub_82319CA8(ctx, base);
	// 82333644: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82333648: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8233364C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82333650: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82333654: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82333658: C00BAD00  lfs f0, -0x5300(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233365C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333660: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82333664: 13E05407  vcmpneb. (lvlx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333690 size=60
    let mut pc: u32 = 0x82333690;
    'dispatch: loop {
        match pc {
            0x82333690 => {
    //   block [0x82333690..0x823336CC)
	// 82333690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233369C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823336A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823336A4: 4BFFE735  bl 0x82331dd8
	ctx.lr = 0x823336A8;
	sub_82331DD8(ctx, base);
	// 823336A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823336AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823336B0: 396B0F40  addi r11, r11, 0xf40
	ctx.r[11].s64 = ctx.r[11].s64 + 3904;
	// 823336B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823336B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823336BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823336C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823336C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823336C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823336D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823336D0 size=124
    let mut pc: u32 = 0x823336D0;
    'dispatch: loop {
        match pc {
            0x823336D0 => {
    //   block [0x823336D0..0x8233374C)
	// 823336D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823336D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823336D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823336DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823336E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823336E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823336E8: 48D943F9  bl 0x830c7ae0
	ctx.lr = 0x823336EC;
	sub_830C7AE0(ctx, base);
	// 823336EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823336F0: 486DD181  bl 0x82a10870
	ctx.lr = 0x823336F4;
	sub_82A10870(ctx, base);
	// 823336F4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823336F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823336FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333700: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 82333704: 48AC0305  bl 0x82df3a08
	ctx.lr = 0x82333708;
	sub_82DF3A08(ctx, base);
	// 82333708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233370C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82333710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82333714: 4BFD1E2D  bl 0x82305540
	ctx.lr = 0x82333718;
	sub_82305540(ctx, base);
	// 82333718: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233371C: 48ABFD0D  bl 0x82df3428
	ctx.lr = 0x82333720;
	sub_82DF3428(ctx, base);
	// 82333720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82333724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333728: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 8233372C: 4BFDFA2D  bl 0x82313158
	ctx.lr = 0x82333730;
	sub_82313158(ctx, base);
	// 82333730: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82333734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233373C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333740: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333750 size=56
    let mut pc: u32 = 0x82333750;
    'dispatch: loop {
        match pc {
            0x82333750 => {
    //   block [0x82333750..0x82333788)
	// 82333750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233375C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333764: 486DD10D  bl 0x82a10870
	ctx.lr = 0x82333768;
	sub_82A10870(ctx, base);
	// 82333768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233376C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82333770: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82333774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82333778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233377C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333788 size=124
    let mut pc: u32 = 0x82333788;
    'dispatch: loop {
        match pc {
            0x82333788 => {
    //   block [0x82333788..0x82333804)
	// 82333788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82333794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333798: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233379C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823337A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823337A4: 486DD0CD  bl 0x82a10870
	ctx.lr = 0x823337A8;
	sub_82A10870(ctx, base);
	// 823337A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823337AC: 4BFD1D55  bl 0x82305500
	ctx.lr = 0x823337B0;
	sub_82305500(ctx, base);
	// 823337B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823337B4: C00BCC2C  lfs f0, -0x33d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823337B8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823337BC: 40980010  bge cr6, 0x823337cc
	if !ctx.cr[6].lt {
	pc = 0x823337CC; continue 'dispatch;
	}
	// 823337C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823337C4: C00B0F68  lfs f0, 0xf68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3944 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823337C8: 4800001C  b 0x823337e4
	pc = 0x823337E4; continue 'dispatch;
	// 823337CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823337D0: 4BFD1D09  bl 0x823054d8
	ctx.lr = 0x823337D4;
	sub_823054D8(ctx, base);
	// 823337D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823337D8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823337DC: 4BFD1D25  bl 0x82305500
	ctx.lr = 0x823337E0;
	sub_82305500(ctx, base);
	// 823337E0: EC1F0824  fdivs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[31].f64 / ctx.f[1].f64) as f32) as f64;
	// 823337E4: D01E0018  stfs f0, 0x18(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 823337E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823337EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823337F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823337F4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823337F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823337FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333808 size=52
    let mut pc: u32 = 0x82333808;
    'dispatch: loop {
        match pc {
            0x82333808 => {
    //   block [0x82333808..0x8233383C)
	// 82333808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233380C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333818: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233381C: 486DD055  bl 0x82a10870
	ctx.lr = 0x82333820;
	sub_82A10870(ctx, base);
	// 82333820: C03F0018  lfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333824: 4BFD1C55  bl 0x82305478
	ctx.lr = 0x82333828;
	sub_82305478(ctx, base);
	// 82333828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233382C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333834: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333840 size=52
    let mut pc: u32 = 0x82333840;
    'dispatch: loop {
        match pc {
            0x82333840 => {
    //   block [0x82333840..0x82333874)
	// 82333840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233384C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333850: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82333854: 486DD01D  bl 0x82a10870
	ctx.lr = 0x82333858;
	sub_82A10870(ctx, base);
	// 82333858: 4BFD1BD1  bl 0x82305428
	ctx.lr = 0x8233385C;
	sub_82305428(ctx, base);
	// 8233385C: 987F0018  stb r3, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u8 ) };
	// 82333860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82333864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233386C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333878 size=52
    let mut pc: u32 = 0x82333878;
    'dispatch: loop {
        match pc {
            0x82333878 => {
    //   block [0x82333878..0x823338AC)
	// 82333878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333888: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233388C: 486DCFE5  bl 0x82a10870
	ctx.lr = 0x82333890;
	sub_82A10870(ctx, base);
	// 82333890: C03F0018  lfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333894: 4BFD1E8D  bl 0x82305720
	ctx.lr = 0x82333898;
	sub_82305720(ctx, base);
	// 82333898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823338A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823338A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823338A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823338B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823338B0 size=196
    let mut pc: u32 = 0x823338B0;
    'dispatch: loop {
        match pc {
            0x823338B0 => {
    //   block [0x823338B0..0x82333974)
	// 823338B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823338B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823338B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823338BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823338C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823338C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823338C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823338CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823338D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823338D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823338D8: 4BF8D061  bl 0x822c0938
	ctx.lr = 0x823338DC;
	sub_822C0938(ctx, base);
	// 823338DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823338E0: 41820028  beq 0x82333908
	if ctx.cr[0].eq {
	pc = 0x82333908; continue 'dispatch;
	}
	// 823338E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823338E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823338EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823338F0: 392B0F70  addi r9, r11, 0xf70
	ctx.r[9].s64 = ctx.r[11].s64 + 3952;
	// 823338F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823338F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823338FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82333900: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82333904: 48000008  b 0x8233390c
	pc = 0x8233390C; continue 'dispatch;
	// 82333908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233390C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82333910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82333914: 409A0044  bne cr6, 0x82333958
	if !ctx.cr[6].eq {
	pc = 0x82333958; continue 'dispatch;
	}
	// 82333918: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233391C: 419A001C  beq cr6, 0x82333938
	if ctx.cr[6].eq {
	pc = 0x82333938; continue 'dispatch;
	}
	// 82333920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333924: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82333928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233392C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82333934: 4E800421  bctrl
	ctx.lr = 0x82333938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82333938: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233393C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82333940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333944: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82333948: 816B8478  lwz r11, -0x7b88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31624 as u32) ) } as u64;
	// 8233394C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82333950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82333954: 4BF8C6AD  bl 0x822c0000
	ctx.lr = 0x82333958;
	sub_822C0000(ctx, base);
	// 82333958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233395C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233396C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333978 size=104
    let mut pc: u32 = 0x82333978;
    'dispatch: loop {
        match pc {
            0x82333978 => {
    //   block [0x82333978..0x823339E0)
	// 82333978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333984: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333988: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233398C: 486DCEE5  bl 0x82a10870
	ctx.lr = 0x82333990;
	sub_82A10870(ctx, base);
	// 82333990: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333994: 38BF0018  addi r5, r31, 0x18
	ctx.r[5].s64 = ctx.r[31].s64 + 24;
	// 82333998: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233399C: 4BFD4875  bl 0x82308210
	ctx.lr = 0x823339A0;
	sub_82308210(ctx, base);
	// 823339A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823339A4: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 823339A8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 823339AC: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 823339B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823339B4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 823339B8: 4BF90AA9  bl 0x822c4460
	ctx.lr = 0x823339BC;
	sub_822C4460(ctx, base);
	// 823339BC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823339C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823339C4: 419A0008  beq cr6, 0x823339cc
	if ctx.cr[6].eq {
	pc = 0x823339CC; continue 'dispatch;
	}
	// 823339C8: 4BF8CEC9  bl 0x822c0890
	ctx.lr = 0x823339CC;
	sub_822C0890(ctx, base);
	// 823339CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823339D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823339D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823339D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823339DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823339E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823339E0 size=120
    let mut pc: u32 = 0x823339E0;
    'dispatch: loop {
        match pc {
            0x823339E0 => {
    //   block [0x823339E0..0x82333A58)
	// 823339E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823339E4: 48E74789  bl 0x831a816c
	ctx.lr = 0x823339E8;
	sub_831A8130(ctx, base);
	// 823339E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823339EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823339F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823339F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823339F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823339FC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82333A00: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82333A04: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82333A08: 48ABE9E1  bl 0x82df23e8
	ctx.lr = 0x82333A0C;
	sub_82DF23E8(ctx, base);
	// 82333A0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82333A10: 41820014  beq 0x82333a24
	if ctx.cr[0].eq {
	pc = 0x82333A24; continue 'dispatch;
	}
	// 82333A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333A18: 488230D9  bl 0x82b56af0
	ctx.lr = 0x82333A1C;
	sub_82B56AF0(ctx, base);
	// 82333A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333A20: 48000008  b 0x82333a28
	pc = 0x82333A28; continue 'dispatch;
	// 82333A24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82333A28: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82333A2C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82333A30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333A34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333A38: 4BFFFE79  bl 0x823338b0
	ctx.lr = 0x82333A3C;
	sub_823338B0(ctx, base);
	// 82333A3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82333A40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333A48: 4BF8C5B9  bl 0x822c0000
	ctx.lr = 0x82333A4C;
	sub_822C0000(ctx, base);
	// 82333A4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82333A50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333A54: 48E74768  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333A58 size=388
    let mut pc: u32 = 0x82333A58;
    'dispatch: loop {
        match pc {
            0x82333A58 => {
    //   block [0x82333A58..0x82333BDC)
	// 82333A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333A5C: 48E7470D  bl 0x831a8168
	ctx.lr = 0x82333A60;
	sub_831A8130(ctx, base);
	// 82333A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333A64: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82333A68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333A6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82333A70: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82333A74: 41820034  beq 0x82333aa8
	if ctx.cr[0].eq {
	pc = 0x82333AA8; continue 'dispatch;
	}
	// 82333A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333A7C: 48E75F0D  bl 0x831a9988
	ctx.lr = 0x82333A80;
	sub_831A9988(ctx, base);
	// 82333A80: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82333A84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333A88: 386B73B0  addi r3, r11, 0x73b0
	ctx.r[3].s64 = ctx.r[11].s64 + 29616;
	// 82333A8C: 48E7466D  bl 0x831a80f8
	ctx.lr = 0x82333A90;
	sub_831A80F8(ctx, base);
	// 82333A90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333A94: 41820014  beq 0x82333aa8
	if ctx.cr[0].eq {
	pc = 0x82333AA8; continue 'dispatch;
	}
	// 82333A98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333AA0: 4BFFFCB1  bl 0x82333750
	ctx.lr = 0x82333AA4;
	sub_82333750(ctx, base);
	// 82333AA4: 48000130  b 0x82333bd4
	pc = 0x82333BD4; continue 'dispatch;
	// 82333AA8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82333AAC: 419A0118  beq cr6, 0x82333bc4
	if ctx.cr[6].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333AB4: 48E75ED5  bl 0x831a9988
	ctx.lr = 0x82333AB8;
	sub_831A9988(ctx, base);
	// 82333AB8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333ABC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333AC0: 386B85F0  addi r3, r11, -0x7a10
	ctx.r[3].s64 = ctx.r[11].s64 + -31248;
	// 82333AC4: 48E74635  bl 0x831a80f8
	ctx.lr = 0x82333AC8;
	sub_831A80F8(ctx, base);
	// 82333AC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333ACC: 41820018  beq 0x82333ae4
	if ctx.cr[0].eq {
	pc = 0x82333AE4; continue 'dispatch;
	}
	// 82333AD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333AD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333AD8: 4BFFFEA1  bl 0x82333978
	ctx.lr = 0x82333ADC;
	sub_82333978(ctx, base);
	// 82333ADC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82333AE0: 480000F4  b 0x82333bd4
	pc = 0x82333BD4; continue 'dispatch;
	// 82333AE4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82333AE8: 419A00DC  beq cr6, 0x82333bc4
	if ctx.cr[6].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333AF0: 48E75E99  bl 0x831a9988
	ctx.lr = 0x82333AF4;
	sub_831A9988(ctx, base);
	// 82333AF4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333AF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333AFC: 386B85A0  addi r3, r11, -0x7a60
	ctx.r[3].s64 = ctx.r[11].s64 + -31328;
	// 82333B00: 48E745F9  bl 0x831a80f8
	ctx.lr = 0x82333B04;
	sub_831A80F8(ctx, base);
	// 82333B04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333B08: 41820014  beq 0x82333b1c
	if ctx.cr[0].eq {
	pc = 0x82333B1C; continue 'dispatch;
	}
	// 82333B0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333B10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333B14: 4BFFFC75  bl 0x82333788
	ctx.lr = 0x82333B18;
	sub_82333788(ctx, base);
	// 82333B18: 4BFFFFC4  b 0x82333adc
	pc = 0x82333ADC; continue 'dispatch;
	// 82333B1C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82333B20: 419A00A4  beq cr6, 0x82333bc4
	if ctx.cr[6].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333B28: 48E75E61  bl 0x831a9988
	ctx.lr = 0x82333B2C;
	sub_831A9988(ctx, base);
	// 82333B2C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333B30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333B34: 386B8560  addi r3, r11, -0x7aa0
	ctx.r[3].s64 = ctx.r[11].s64 + -31392;
	// 82333B38: 48E745C1  bl 0x831a80f8
	ctx.lr = 0x82333B3C;
	sub_831A80F8(ctx, base);
	// 82333B3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333B40: 41820014  beq 0x82333b54
	if ctx.cr[0].eq {
	pc = 0x82333B54; continue 'dispatch;
	}
	// 82333B44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333B4C: 4BFFFCBD  bl 0x82333808
	ctx.lr = 0x82333B50;
	sub_82333808(ctx, base);
	// 82333B50: 4BFFFF8C  b 0x82333adc
	pc = 0x82333ADC; continue 'dispatch;
	// 82333B54: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82333B58: 419A006C  beq cr6, 0x82333bc4
	if ctx.cr[6].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333B60: 48E75E29  bl 0x831a9988
	ctx.lr = 0x82333B64;
	sub_831A9988(ctx, base);
	// 82333B64: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333B68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333B6C: 386B8524  addi r3, r11, -0x7adc
	ctx.r[3].s64 = ctx.r[11].s64 + -31452;
	// 82333B70: 48E74589  bl 0x831a80f8
	ctx.lr = 0x82333B74;
	sub_831A80F8(ctx, base);
	// 82333B74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333B78: 41820014  beq 0x82333b8c
	if ctx.cr[0].eq {
	pc = 0x82333B8C; continue 'dispatch;
	}
	// 82333B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333B80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333B84: 4BFFFCBD  bl 0x82333840
	ctx.lr = 0x82333B88;
	sub_82333840(ctx, base);
	// 82333B88: 4BFFFF54  b 0x82333adc
	pc = 0x82333ADC; continue 'dispatch;
	// 82333B8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82333B90: 419A0034  beq cr6, 0x82333bc4
	if ctx.cr[6].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333B98: 48E75DF1  bl 0x831a9988
	ctx.lr = 0x82333B9C;
	sub_831A9988(ctx, base);
	// 82333B9C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333BA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333BA4: 386B84E4  addi r3, r11, -0x7b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -31516;
	// 82333BA8: 48E74551  bl 0x831a80f8
	ctx.lr = 0x82333BAC;
	sub_831A80F8(ctx, base);
	// 82333BAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333BB0: 41820014  beq 0x82333bc4
	if ctx.cr[0].eq {
	pc = 0x82333BC4; continue 'dispatch;
	}
	// 82333BB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333BB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333BBC: 4BFFFCBD  bl 0x82333878
	ctx.lr = 0x82333BC0;
	sub_82333878(ctx, base);
	// 82333BC0: 4BFFFF1C  b 0x82333adc
	pc = 0x82333ADC; continue 'dispatch;
	// 82333BC4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82333BC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82333BCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333BD0: 4BFFE641  bl 0x82332210
	ctx.lr = 0x82333BD4;
	sub_82332210(ctx, base);
	// 82333BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82333BD8: 48E745E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333BE0 size=212
    let mut pc: u32 = 0x82333BE0;
    'dispatch: loop {
        match pc {
            0x82333BE0 => {
    //   block [0x82333BE0..0x82333CB4)
	// 82333BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333BE4: 48E74589  bl 0x831a816c
	ctx.lr = 0x82333BE8;
	sub_831A8130(ctx, base);
	// 82333BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333BEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333BF0: 486DCC81  bl 0x82a10870
	ctx.lr = 0x82333BF4;
	sub_82A10870(ctx, base);
	// 82333BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333BF8: 4BFD1831  bl 0x82305428
	ctx.lr = 0x82333BFC;
	sub_82305428(ctx, base);
	// 82333BFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333C00: 418200A4  beq 0x82333ca4
	if ctx.cr[0].eq {
	pc = 0x82333CA4; continue 'dispatch;
	}
	// 82333C04: 897E0068  lbz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82333C08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82333C0C: 40820098  bne 0x82333ca4
	if !ctx.cr[0].eq {
	pc = 0x82333CA4; continue 'dispatch;
	}
	// 82333C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333C14: 4BFD17ED  bl 0x82305400
	ctx.lr = 0x82333C18;
	sub_82305400(ctx, base);
	// 82333C18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82333C1C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82333C20: 4BFFFDC1  bl 0x823339e0
	ctx.lr = 0x82333C24;
	sub_823339E0(ctx, base);
	// 82333C24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333C28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82333C2C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82333C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82333C34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82333C38: 419A0024  beq cr6, 0x82333c5c
	if ctx.cr[6].eq {
	pc = 0x82333C5C; continue 'dispatch;
	}
	// 82333C3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82333C40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82333C44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82333C48: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82333C4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82333C50: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82333C54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82333C58: 4082FFE8  bne 0x82333c40
	if !ctx.cr[0].eq {
	pc = 0x82333C40; continue 'dispatch;
	}
	// 82333C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333C60: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82333C64: 4BFDF4F5  bl 0x82313158
	ctx.lr = 0x82333C68;
	sub_82313158(ctx, base);
	// 82333C68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82333C6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82333C70: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82333C74: 388A0F80  addi r4, r10, 0xf80
	ctx.r[4].s64 = ctx.r[10].s64 + 3968;
	// 82333C78: 38A00045  li r5, 0x45
	ctx.r[5].s64 = 69;
	// 82333C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333C80: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333C84: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82333C88: 4BFD4201  bl 0x82307e88
	ctx.lr = 0x82333C8C;
	sub_82307E88(ctx, base);
	// 82333C8C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82333C90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82333C94: 419A0008  beq cr6, 0x82333c9c
	if ctx.cr[6].eq {
	pc = 0x82333C9C; continue 'dispatch;
	}
	// 82333C98: 4BF8CBF9  bl 0x822c0890
	ctx.lr = 0x82333C9C;
	sub_822C0890(ctx, base);
	// 82333C9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82333CA0: 997E0068  stb r11, 0x68(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82333CA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333CA8: 48D93E39  bl 0x830c7ae0
	ctx.lr = 0x82333CAC;
	sub_830C7AE0(ctx, base);
	// 82333CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82333CB0: 48E7450C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333CB8 size=312
    let mut pc: u32 = 0x82333CB8;
    'dispatch: loop {
        match pc {
            0x82333CB8 => {
    //   block [0x82333CB8..0x82333DF0)
	// 82333CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333CBC: 48E744B1  bl 0x831a816c
	ctx.lr = 0x82333CC0;
	sub_831A8130(ctx, base);
	// 82333CC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333CC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333CC8: 486DCBA9  bl 0x82a10870
	ctx.lr = 0x82333CCC;
	sub_82A10870(ctx, base);
	// 82333CCC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333CD8: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 82333CDC: 48ABFD2D  bl 0x82df3a08
	ctx.lr = 0x82333CE0;
	sub_82DF3A08(ctx, base);
	// 82333CE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82333CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333CE8: 4BFD1861  bl 0x82305548
	ctx.lr = 0x82333CEC;
	sub_82305548(ctx, base);
	// 82333CEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333CF0: 48ABF739  bl 0x82df3428
	ctx.lr = 0x82333CF4;
	sub_82DF3428(ctx, base);
	// 82333CF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82333CF8: 4BFDD009  bl 0x82310d00
	ctx.lr = 0x82333CFC;
	sub_82310D00(ctx, base);
	// 82333CFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333D00: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82333D04: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82333D08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82333D0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82333D10: 419A0024  beq cr6, 0x82333d34
	if ctx.cr[6].eq {
	pc = 0x82333D34; continue 'dispatch;
	}
	// 82333D14: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82333D18: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82333D1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82333D20: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82333D24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82333D28: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82333D2C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82333D30: 4082FFE8  bne 0x82333d18
	if !ctx.cr[0].eq {
	pc = 0x82333D18; continue 'dispatch;
	}
	// 82333D34: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82333D38: 80DE006C  lwz r6, 0x6c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82333D3C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82333D40: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82333D44: 388A0F80  addi r4, r10, 0xf80
	ctx.r[4].s64 = ctx.r[10].s64 + 3968;
	// 82333D48: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82333D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333D50: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82333D54: 4BFD4135  bl 0x82307e88
	ctx.lr = 0x82333D58;
	sub_82307E88(ctx, base);
	// 82333D58: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82333D5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82333D60: 419A0008  beq cr6, 0x82333d68
	if ctx.cr[6].eq {
	pc = 0x82333D68; continue 'dispatch;
	}
	// 82333D64: 4BF8CB2D  bl 0x822c0890
	ctx.lr = 0x82333D68;
	sub_822C0890(ctx, base);
	// 82333D68: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82333D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333D70: 4BFDFF81  bl 0x82313cf0
	ctx.lr = 0x82333D74;
	sub_82313CF0(ctx, base);
	// 82333D74: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333D78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333D7C: 808BB3F4  lwz r4, -0x4c0c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19468 as u32) ) } as u64;
	// 82333D80: 48ABFC89  bl 0x82df3a08
	ctx.lr = 0x82333D84;
	sub_82DF3A08(ctx, base);
	// 82333D84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82333D88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82333D8C: 48B25C45  bl 0x82e599d0
	ctx.lr = 0x82333D90;
	sub_82E599D0(ctx, base);
	// 82333D90: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333D94: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82333D98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333D9C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82333DA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82333DA4: 4E800421  bctrl
	ctx.lr = 0x82333DA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82333DA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82333DAC: 48ABF4F5  bl 0x82df32a0
	ctx.lr = 0x82333DB0;
	sub_82DF32A0(ctx, base);
	// 82333DB0: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82333DB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82333DB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82333DBC: 419A000C  beq cr6, 0x82333dc8
	if ctx.cr[6].eq {
	pc = 0x82333DC8; continue 'dispatch;
	}
	// 82333DC0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82333DC4: 4BF8CACD  bl 0x822c0890
	ctx.lr = 0x82333DC8;
	sub_822C0890(ctx, base);
	// 82333DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82333DCC: 48ABF65D  bl 0x82df3428
	ctx.lr = 0x82333DD0;
	sub_82DF3428(ctx, base);
	// 82333DD0: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333DD4: 4182000C  beq 0x82333de0
	if ctx.cr[0].eq {
	pc = 0x82333DE0; continue 'dispatch;
	}
	// 82333DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333DDC: 4BFE77B5  bl 0x8231b590
	ctx.lr = 0x82333DE0;
	sub_8231B590(ctx, base);
	// 82333DE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333DE4: 48D93CFD  bl 0x830c7ae0
	ctx.lr = 0x82333DE8;
	sub_830C7AE0(ctx, base);
	// 82333DE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82333DEC: 48E743D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333DF0 size=60
    let mut pc: u32 = 0x82333DF0;
    'dispatch: loop {
        match pc {
            0x82333DF0 => {
    //   block [0x82333DF0..0x82333E2C)
	// 82333DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333E00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333E04: 4BFFDFD5  bl 0x82331dd8
	ctx.lr = 0x82333E08;
	sub_82331DD8(ctx, base);
	// 82333E08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333E10: 396B0FE8  addi r11, r11, 0xfe8
	ctx.r[11].s64 = ctx.r[11].s64 + 4072;
	// 82333E14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82333E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82333E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333E24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333E30 size=108
    let mut pc: u32 = 0x82333E30;
    'dispatch: loop {
        match pc {
            0x82333E30 => {
    //   block [0x82333E30..0x82333E9C)
	// 82333E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82333E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333E48: 486DCA29  bl 0x82a10870
	ctx.lr = 0x82333E4C;
	sub_82A10870(ctx, base);
	// 82333E4C: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82333E50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333E54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82333E58: 4182000C  beq 0x82333e64
	if ctx.cr[0].eq {
	pc = 0x82333E64; continue 'dispatch;
	}
	// 82333E5C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82333E60: 4BFDFEA1  bl 0x82313d00
	ctx.lr = 0x82333E64;
	sub_82313D00(ctx, base);
	// 82333E64: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82333E68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82333E6C: 419A000C  beq cr6, 0x82333e78
	if ctx.cr[6].eq {
	pc = 0x82333E78; continue 'dispatch;
	}
	// 82333E70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333E74: 4BFE6BE5  bl 0x8231aa58
	ctx.lr = 0x82333E78;
	sub_8231AA58(ctx, base);
	// 82333E78: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 82333E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333E80: 4BFDFE71  bl 0x82313cf0
	ctx.lr = 0x82333E84;
	sub_82313CF0(ctx, base);
	// 82333E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82333E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333E90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82333E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82333EA0 size=12
    let mut pc: u32 = 0x82333EA0;
    'dispatch: loop {
        match pc {
            0x82333EA0 => {
    //   block [0x82333EA0..0x82333EAC)
	// 82333EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82333EA4: 386B1020  addi r3, r11, 0x1020
	ctx.r[3].s64 = ctx.r[11].s64 + 4128;
	// 82333EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333EB0 size=124
    let mut pc: u32 = 0x82333EB0;
    'dispatch: loop {
        match pc {
            0x82333EB0 => {
    //   block [0x82333EB0..0x82333F2C)
	// 82333EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333EC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82333EC8: 419A004C  beq cr6, 0x82333f14
	if ctx.cr[6].eq {
	pc = 0x82333F14; continue 'dispatch;
	}
	// 82333ECC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82333ED4: 419A0018  beq cr6, 0x82333eec
	if ctx.cr[6].eq {
	pc = 0x82333EEC; continue 'dispatch;
	}
	// 82333ED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333EDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82333EE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82333EE4: 4E800421  bctrl
	ctx.lr = 0x82333EE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82333EE8: 4800000C  b 0x82333ef4
	pc = 0x82333EF4; continue 'dispatch;
	// 82333EEC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333EF0: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 82333EF4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333EF8: 388B86A8  addi r4, r11, -0x7958
	ctx.r[4].s64 = ctx.r[11].s64 + -31064;
	// 82333EFC: 48E741FD  bl 0x831a80f8
	ctx.lr = 0x82333F00;
	sub_831A80F8(ctx, base);
	// 82333F00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333F04: 41820010  beq 0x82333f14
	if ctx.cr[0].eq {
	pc = 0x82333F14; continue 'dispatch;
	}
	// 82333F08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333F0C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82333F10: 48000008  b 0x82333f18
	pc = 0x82333F18; continue 'dispatch;
	// 82333F14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82333F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82333F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82333F30 size=124
    let mut pc: u32 = 0x82333F30;
    'dispatch: loop {
        match pc {
            0x82333F30 => {
    //   block [0x82333F30..0x82333FAC)
	// 82333F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82333F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82333F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333F44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82333F48: 419A004C  beq cr6, 0x82333f94
	if ctx.cr[6].eq {
	pc = 0x82333F94; continue 'dispatch;
	}
	// 82333F4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333F50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82333F54: 419A0018  beq cr6, 0x82333f6c
	if ctx.cr[6].eq {
	pc = 0x82333F6C; continue 'dispatch;
	}
	// 82333F58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333F5C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82333F60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82333F64: 4E800421  bctrl
	ctx.lr = 0x82333F68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82333F68: 4800000C  b 0x82333f74
	pc = 0x82333F74; continue 'dispatch;
	// 82333F6C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333F70: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 82333F74: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82333F78: 388B7FC4  addi r4, r11, 0x7fc4
	ctx.r[4].s64 = ctx.r[11].s64 + 32708;
	// 82333F7C: 48E7417D  bl 0x831a80f8
	ctx.lr = 0x82333F80;
	sub_831A80F8(ctx, base);
	// 82333F80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82333F84: 41820010  beq 0x82333f94
	if ctx.cr[0].eq {
	pc = 0x82333F94; continue 'dispatch;
	}
	// 82333F88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82333F8C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82333F90: 48000008  b 0x82333f98
	pc = 0x82333F98; continue 'dispatch;
	// 82333F94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82333F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82333F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82333FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82333FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82333FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82333FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82333FB0 size=280
    let mut pc: u32 = 0x82333FB0;
    'dispatch: loop {
        match pc {
            0x82333FB0 => {
    //   block [0x82333FB0..0x823340C8)
	// 82333FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82333FB4: 48E741B9  bl 0x831a816c
	ctx.lr = 0x82333FB8;
	sub_831A8130(ctx, base);
	// 82333FB8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82333FBC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82333FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82333FC4: 486DC8AD  bl 0x82a10870
	ctx.lr = 0x82333FC8;
	sub_82A10870(ctx, base);
	// 82333FC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82333FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82333FD0: 48B257A9  bl 0x82e59778
	ctx.lr = 0x82333FD4;
	sub_82E59778(ctx, base);
	// 82333FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82333FD8: 4BFDED29  bl 0x82312d00
	ctx.lr = 0x82333FDC;
	sub_82312D00(ctx, base);
	// 82333FDC: 897F006D  lbz r11, 0x6d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(109 as u32) ) } as u64;
	// 82333FE0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82333FE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82333FE8: 40820050  bne 0x82334038
	if !ctx.cr[0].eq {
	pc = 0x82334038; continue 'dispatch;
	}
	// 82333FEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82333FF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82333FF4: 4BFE5AF5  bl 0x82319ae8
	ctx.lr = 0x82333FF8;
	sub_82319AE8(ctx, base);
	// 82333FF8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82333FFC: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82334000: C00B8670  lfs f0, -0x7990(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334004: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82334008: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8233400C: 4098002C  bge cr6, 0x82334038
	if !ctx.cr[6].lt {
	pc = 0x82334038; continue 'dispatch;
	}
	// 82334010: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334018: 808BB064  lwz r4, -0x4f9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20380 as u32) ) } as u64;
	// 8233401C: 48ABF9ED  bl 0x82df3a08
	ctx.lr = 0x82334020;
	sub_82DF3A08(ctx, base);
	// 82334020: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334028: 4BFD38B1  bl 0x823078d8
	ctx.lr = 0x8233402C;
	sub_823078D8(ctx, base);
	// 8233402C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334030: 48ABF3F9  bl 0x82df3428
	ctx.lr = 0x82334034;
	sub_82DF3428(ctx, base);
	// 82334034: 9BBF006D  stb r29, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[29].u8 ) };
	// 82334038: 897F006E  lbz r11, 0x6e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(110 as u32) ) } as u64;
	// 8233403C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82334040: 40820058  bne 0x82334098
	if !ctx.cr[0].eq {
	pc = 0x82334098; continue 'dispatch;
	}
	// 82334044: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 82334048: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233404C: 48126D9D  bl 0x8245ade8
	ctx.lr = 0x82334050;
	sub_8245ADE8(ctx, base);
	// 82334050: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82334054: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334058: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233405C: 4BFE5A8D  bl 0x82319ae8
	ctx.lr = 0x82334060;
	sub_82319AE8(ctx, base);
	// 82334060: FC00F850  fneg f0, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 82334064: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82334068: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8233406C: 4098002C  bge cr6, 0x82334098
	if !ctx.cr[6].lt {
	pc = 0x82334098; continue 'dispatch;
	}
	// 82334070: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334074: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334078: 388B1060  addi r4, r11, 0x1060
	ctx.r[4].s64 = ctx.r[11].s64 + 4192;
	// 8233407C: 48ABF98D  bl 0x82df3a08
	ctx.lr = 0x82334080;
	sub_82DF3A08(ctx, base);
	// 82334080: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334088: 4BFD1C59  bl 0x82305ce0
	ctx.lr = 0x8233408C;
	sub_82305CE0(ctx, base);
	// 8233408C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334090: 48ABF399  bl 0x82df3428
	ctx.lr = 0x82334094;
	sub_82DF3428(ctx, base);
	// 82334094: 9BBF006E  stb r29, 0x6e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(110 as u32), ctx.r[29].u8 ) };
	// 82334098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233409C: 4BFDEB6D  bl 0x82312c08
	ctx.lr = 0x823340A0;
	sub_82312C08(ctx, base);
	// 823340A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823340A4: 41820018  beq 0x823340bc
	if ctx.cr[0].eq {
	pc = 0x823340BC; continue 'dispatch;
	}
	// 823340A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823340AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823340B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823340B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823340B8: 4E800421  bctrl
	ctx.lr = 0x823340BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823340BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823340C0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823340C4: 48E740F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823340C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823340C8 size=96
    let mut pc: u32 = 0x823340C8;
    'dispatch: loop {
        match pc {
            0x823340C8 => {
    //   block [0x823340C8..0x82334128)
	// 823340C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823340CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823340D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823340D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823340D8: 4BFFFDD9  bl 0x82333eb0
	ctx.lr = 0x823340DC;
	sub_82333EB0(ctx, base);
	// 823340DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823340E0: 40820030  bne 0x82334110
	if !ctx.cr[0].eq {
	pc = 0x82334110; continue 'dispatch;
	}
	// 823340E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823340E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823340EC: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 823340F0: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 823340F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823340F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823340FC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82334100: 4BF8BF01  bl 0x822c0000
	ctx.lr = 0x82334104;
	sub_822C0000(ctx, base);
	// 82334104: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334108: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 8233410C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82334110: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82334114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233411C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334128 size=172
    let mut pc: u32 = 0x82334128;
    'dispatch: loop {
        match pc {
            0x82334128 => {
    //   block [0x82334128..0x823341D4)
	// 82334128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233412C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233413C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334140: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82334144: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82334148: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 8233414C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334150: 4815E1B9  bl 0x82492308
	ctx.lr = 0x82334154;
	sub_82492308(ctx, base);
	// 82334154: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82334158: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8233415C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82334160: 419A0058  beq cr6, 0x823341b8
	if ctx.cr[6].eq {
	pc = 0x823341B8; continue 'dispatch;
	}
	// 82334164: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82334168: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8233416C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334170: 419A0018  beq cr6, 0x82334188
	if ctx.cr[6].eq {
	pc = 0x82334188; continue 'dispatch;
	}
	// 82334174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82334178: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233417C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82334180: 4E800421  bctrl
	ctx.lr = 0x82334184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82334184: 4800000C  b 0x82334190
	pc = 0x82334190; continue 'dispatch;
	// 82334188: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233418C: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 82334190: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334194: 388B86A8  addi r4, r11, -0x7958
	ctx.r[4].s64 = ctx.r[11].s64 + -31064;
	// 82334198: 48E73F61  bl 0x831a80f8
	ctx.lr = 0x8233419C;
	sub_831A80F8(ctx, base);
	// 8233419C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823341A0: 41820018  beq 0x823341b8
	if ctx.cr[0].eq {
	pc = 0x823341B8; continue 'dispatch;
	}
	// 823341A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823341A8: 4BFFFF21  bl 0x823340c8
	ctx.lr = 0x823341AC;
	sub_823340C8(ctx, base);
	// 823341AC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 823341B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823341B4: 48000008  b 0x823341bc
	pc = 0x823341BC; continue 'dispatch;
	// 823341B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823341BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823341C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823341C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823341C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823341CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823341D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823341D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823341D8 size=172
    let mut pc: u32 = 0x823341D8;
    'dispatch: loop {
        match pc {
            0x823341D8 => {
    //   block [0x823341D8..0x82334284)
	// 823341D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823341DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823341E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823341E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823341E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823341EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823341F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823341F4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 823341F8: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 823341FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334200: 4815E109  bl 0x82492308
	ctx.lr = 0x82334204;
	sub_82492308(ctx, base);
	// 82334204: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82334208: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8233420C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82334210: 419A0058  beq cr6, 0x82334268
	if ctx.cr[6].eq {
	pc = 0x82334268; continue 'dispatch;
	}
	// 82334214: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82334218: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8233421C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334220: 419A0018  beq cr6, 0x82334238
	if ctx.cr[6].eq {
	pc = 0x82334238; continue 'dispatch;
	}
	// 82334224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82334228: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233422C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82334230: 4E800421  bctrl
	ctx.lr = 0x82334234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82334234: 4800000C  b 0x82334240
	pc = 0x82334240; continue 'dispatch;
	// 82334238: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233423C: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 82334240: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82334244: 388B7FC4  addi r4, r11, 0x7fc4
	ctx.r[4].s64 = ctx.r[11].s64 + 32708;
	// 82334248: 48E73EB1  bl 0x831a80f8
	ctx.lr = 0x8233424C;
	sub_831A80F8(ctx, base);
	// 8233424C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82334250: 41820018  beq 0x82334268
	if ctx.cr[0].eq {
	pc = 0x82334268; continue 'dispatch;
	}
	// 82334254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334258: 48126631  bl 0x8245a888
	ctx.lr = 0x8233425C;
	sub_8245A888(ctx, base);
	// 8233425C: 987E0000  stb r3, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82334260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82334264: 48000008  b 0x8233426c
	pc = 0x8233426C; continue 'dispatch;
	// 82334268: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8233426C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233427C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334288 size=184
    let mut pc: u32 = 0x82334288;
    'dispatch: loop {
        match pc {
            0x82334288 => {
    //   block [0x82334288..0x82334340)
	// 82334288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233428C: 48E73EE1  bl 0x831a816c
	ctx.lr = 0x82334290;
	sub_831A8130(ctx, base);
	// 82334290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334298: 486DC5D9  bl 0x82a10870
	ctx.lr = 0x8233429C;
	sub_82A10870(ctx, base);
	// 8233429C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823342A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823342A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823342A8: 808BB060  lwz r4, -0x4fa0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 823342AC: 48ABF75D  bl 0x82df3a08
	ctx.lr = 0x823342B0;
	sub_82DF3A08(ctx, base);
	// 823342B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823342B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823342B8: 4BFD3621  bl 0x823078d8
	ctx.lr = 0x823342BC;
	sub_823078D8(ctx, base);
	// 823342BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823342C0: 48ABF169  bl 0x82df3428
	ctx.lr = 0x823342C4;
	sub_82DF3428(ctx, base);
	// 823342C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823342C8: 4BFDF939  bl 0x82313c00
	ctx.lr = 0x823342CC;
	sub_82313C00(ctx, base);
	// 823342CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823342D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823342D4: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 823342D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823342DC: 388B1078  addi r4, r11, 0x1078
	ctx.r[4].s64 = ctx.r[11].s64 + 4216;
	// 823342E0: 3BBF0068  addi r29, r31, 0x68
	ctx.r[29].s64 = ctx.r[31].s64 + 104;
	// 823342E4: 48ABF725  bl 0x82df3a08
	ctx.lr = 0x823342E8;
	sub_82DF3A08(ctx, base);
	// 823342E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823342EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823342F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823342F4: 4BFFFE35  bl 0x82334128
	ctx.lr = 0x823342F8;
	sub_82334128(ctx, base);
	// 823342F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823342FC: 48ABF12D  bl 0x82df3428
	ctx.lr = 0x82334300;
	sub_82DF3428(ctx, base);
	// 82334300: 9BDF006C  stb r30, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u8 ) };
	// 82334304: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233430C: 388B1068  addi r4, r11, 0x1068
	ctx.r[4].s64 = ctx.r[11].s64 + 4200;
	// 82334310: 3BBF006C  addi r29, r31, 0x6c
	ctx.r[29].s64 = ctx.r[31].s64 + 108;
	// 82334314: 48ABF6F5  bl 0x82df3a08
	ctx.lr = 0x82334318;
	sub_82DF3A08(ctx, base);
	// 82334318: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8233431C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334324: 4BFFFEB5  bl 0x823341d8
	ctx.lr = 0x82334328;
	sub_823341D8(ctx, base);
	// 82334328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233432C: 48ABF0FD  bl 0x82df3428
	ctx.lr = 0x82334330;
	sub_82DF3428(ctx, base);
	// 82334330: 9BDF006D  stb r30, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[30].u8 ) };
	// 82334334: 9BDF006E  stb r30, 0x6e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(110 as u32), ctx.r[30].u8 ) };
	// 82334338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233433C: 48E73E80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334340 size=284
    let mut pc: u32 = 0x82334340;
    'dispatch: loop {
        match pc {
            0x82334340 => {
    //   block [0x82334340..0x8233445C)
	// 82334340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334344: 48E73E29  bl 0x831a816c
	ctx.lr = 0x82334348;
	sub_831A8130(ctx, base);
	// 82334348: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233434C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334354: 486DC51D  bl 0x82a10870
	ctx.lr = 0x82334358;
	sub_82A10870(ctx, base);
	// 82334358: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233435C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82334360: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334364: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82334368: 419A000C  beq cr6, 0x82334374
	if ctx.cr[6].eq {
	pc = 0x82334374; continue 'dispatch;
	}
	// 8233436C: 4BFE66ED  bl 0x8231aa58
	ctx.lr = 0x82334370;
	sub_8231AA58(ctx, base);
	// 82334370: 93BF0068  stw r29, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82334374: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233437C: 4BFFA385  bl 0x8232e700
	ctx.lr = 0x82334380;
	sub_8232E700(ctx, base);
	// 82334380: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82334384: 408200CC  bne 0x82334450
	if !ctx.cr[0].eq {
	pc = 0x82334450; continue 'dispatch;
	}
	// 82334388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233438C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334390: 48130A09  bl 0x82464d98
	ctx.lr = 0x82334394;
	sub_82464D98(ctx, base);
	// 82334394: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334398: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233439C: C00B8670  lfs f0, -0x7990(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823343A0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 823343A4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823343A8: 40990008  ble cr6, 0x823343b0
	if !ctx.cr[6].gt {
	pc = 0x823343B0; continue 'dispatch;
	}
	// 823343AC: 9BBF006D  stb r29, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[29].u8 ) };
	// 823343B0: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 823343B4: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823343B8: 48126A31  bl 0x8245ade8
	ctx.lr = 0x823343BC;
	sub_8245ADE8(ctx, base);
	// 823343BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823343C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823343C4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823343C8: 481309D1  bl 0x82464d98
	ctx.lr = 0x823343CC;
	sub_82464D98(ctx, base);
	// 823343CC: FC00F850  fneg f0, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 823343D0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823343D4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823343D8: 40990008  ble cr6, 0x823343e0
	if !ctx.cr[6].gt {
	pc = 0x823343E0; continue 'dispatch;
	}
	// 823343DC: 9BBF006E  stb r29, 0x6e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(110 as u32), ctx.r[29].u8 ) };
	// 823343E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823343E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823343E8: 388B108C  addi r4, r11, 0x108c
	ctx.r[4].s64 = ctx.r[11].s64 + 4236;
	// 823343EC: 48ABF61D  bl 0x82df3a08
	ctx.lr = 0x823343F0;
	sub_82DF3A08(ctx, base);
	// 823343F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823343F4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 823343F8: 3BDF006D  addi r30, r31, 0x6d
	ctx.r[30].s64 = ctx.r[31].s64 + 109;
	// 823343FC: 4BFF8445  bl 0x8232c840
	ctx.lr = 0x82334400;
	sub_8232C840(ctx, base);
	// 82334400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82334404: 4BFE9035  bl 0x8231d438
	ctx.lr = 0x82334408;
	sub_8231D438(ctx, base);
	// 82334408: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233440C: 48ABF01D  bl 0x82df3428
	ctx.lr = 0x82334410;
	sub_82DF3428(ctx, base);
	// 82334410: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334414: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82334418: 808BB3B4  lwz r4, -0x4c4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19532 as u32) ) } as u64;
	// 8233441C: 48ABF5ED  bl 0x82df3a08
	ctx.lr = 0x82334420;
	sub_82DF3A08(ctx, base);
	// 82334420: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334424: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82334428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233442C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82334430: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82334438: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233443C: 48B2A655  bl 0x82e5ea90
	ctx.lr = 0x82334440;
	sub_82E5EA90(ctx, base);
	// 82334440: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82334444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334448: 419A0008  beq cr6, 0x82334450
	if ctx.cr[6].eq {
	pc = 0x82334450; continue 'dispatch;
	}
	// 8233444C: 4BF8C445  bl 0x822c0890
	ctx.lr = 0x82334450;
	sub_822C0890(ctx, base);
	// 82334450: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82334454: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82334458: 48E73D64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334460 size=56
    let mut pc: u32 = 0x82334460;
    'dispatch: loop {
        match pc {
            0x82334460 => {
    //   block [0x82334460..0x82334498)
	// 82334460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233446C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334474: 486DC3FD  bl 0x82a10870
	ctx.lr = 0x82334478;
	sub_82A10870(ctx, base);
	// 82334478: 4BFDF789  bl 0x82313c00
	ctx.lr = 0x8233447C;
	sub_82313C00(ctx, base);
	// 8233447C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82334480: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82334484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82334488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233448C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334498 size=72
    let mut pc: u32 = 0x82334498;
    'dispatch: loop {
        match pc {
            0x82334498 => {
    //   block [0x82334498..0x823344E0)
	// 82334498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233449C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823344A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823344A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823344A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823344AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823344B0: 486DC3C1  bl 0x82a10870
	ctx.lr = 0x823344B4;
	sub_82A10870(ctx, base);
	// 823344B4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 823344B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823344BC: 4BFD341D  bl 0x823078d8
	ctx.lr = 0x823344C0;
	sub_823078D8(ctx, base);
	// 823344C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823344C4: 4BFD10ED  bl 0x823055b0
	ctx.lr = 0x823344C8;
	sub_823055B0(ctx, base);
	// 823344C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823344CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823344D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823344D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823344D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823344DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823344E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823344E0 size=108
    let mut pc: u32 = 0x823344E0;
    'dispatch: loop {
        match pc {
            0x823344E0 => {
    //   block [0x823344E0..0x8233454C)
	// 823344E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823344E4: 48E73C89  bl 0x831a816c
	ctx.lr = 0x823344E8;
	sub_831A8130(ctx, base);
	// 823344E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823344EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823344F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823344F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823344F8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823344FC: 41820038  beq 0x82334534
	if ctx.cr[0].eq {
	pc = 0x82334534; continue 'dispatch;
	}
	// 82334500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334504: 48E75485  bl 0x831a9988
	ctx.lr = 0x82334508;
	sub_831A9988(ctx, base);
	// 82334508: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8233450C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82334510: 386B73B0  addi r3, r11, 0x73b0
	ctx.r[3].s64 = ctx.r[11].s64 + 29616;
	// 82334514: 48E73BE5  bl 0x831a80f8
	ctx.lr = 0x82334518;
	sub_831A80F8(ctx, base);
	// 82334518: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233451C: 41820018  beq 0x82334534
	if ctx.cr[0].eq {
	pc = 0x82334534; continue 'dispatch;
	}
	// 82334520: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334524: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334528: 4BFFFF71  bl 0x82334498
	ctx.lr = 0x8233452C;
	sub_82334498(ctx, base);
	// 8233452C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82334530: 48000014  b 0x82334544
	pc = 0x82334544; continue 'dispatch;
	// 82334534: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82334538: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233453C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334540: 4BFFDCD1  bl 0x82332210
	ctx.lr = 0x82334544;
	sub_82332210(ctx, base);
	// 82334544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334548: 48E73C74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334550 size=244
    let mut pc: u32 = 0x82334550;
    'dispatch: loop {
        match pc {
            0x82334550 => {
    //   block [0x82334550..0x82334644)
	// 82334550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334554: 48E73C19  bl 0x831a816c
	ctx.lr = 0x82334558;
	sub_831A8130(ctx, base);
	// 82334558: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233455C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334560: 486DC311  bl 0x82a10870
	ctx.lr = 0x82334564;
	sub_82A10870(ctx, base);
	// 82334564: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334568: 4BFDE6A1  bl 0x82312c08
	ctx.lr = 0x8233456C;
	sub_82312C08(ctx, base);
	// 8233456C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82334570: 41820048  beq 0x823345b8
	if ctx.cr[0].eq {
	pc = 0x823345B8; continue 'dispatch;
	}
	// 82334574: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233457C: 808BB3B4  lwz r4, -0x4c4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19532 as u32) ) } as u64;
	// 82334580: 48ABF489  bl 0x82df3a08
	ctx.lr = 0x82334584;
	sub_82DF3A08(ctx, base);
	// 82334584: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334588: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233458C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334590: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82334594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334598: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233459C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823345A0: 48B2A4F1  bl 0x82e5ea90
	ctx.lr = 0x823345A4;
	sub_82E5EA90(ctx, base);
	// 823345A4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823345A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823345AC: 419A0090  beq cr6, 0x8233463c
	if ctx.cr[6].eq {
	pc = 0x8233463C; continue 'dispatch;
	}
	// 823345B0: 4BF8C2E1  bl 0x822c0890
	ctx.lr = 0x823345B4;
	sub_822C0890(ctx, base);
	// 823345B4: 48000088  b 0x8233463c
	pc = 0x8233463C; continue 'dispatch;
	// 823345B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823345BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823345C0: 4BFDF5A9  bl 0x82313b68
	ctx.lr = 0x823345C4;
	sub_82313B68(ctx, base);
	// 823345C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823345C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823345CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823345D0: 4BFE5519  bl 0x82319ae8
	ctx.lr = 0x823345D4;
	sub_82319AE8(ctx, base);
	// 823345D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823345D8: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823345DC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 823345E0: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 823345E4: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823345E8: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334648 size=100
    let mut pc: u32 = 0x82334648;
    'dispatch: loop {
        match pc {
            0x82334648 => {
    //   block [0x82334648..0x823346AC)
	// 82334648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334658: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233465C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334664: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82334668: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233466C: 48B25015  bl 0x82e59680
	ctx.lr = 0x82334670;
	sub_82E59680(ctx, base);
	// 82334670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334674: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82334678: 396B109C  addi r11, r11, 0x109c
	ctx.r[11].s64 = ctx.r[11].s64 + 4252;
	// 8233467C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82334680: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82334684: 48ABF57D  bl 0x82df3c00
	ctx.lr = 0x82334688;
	sub_82DF3C00(ctx, base);
	// 82334688: D3FF001C  stfs f31, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8233468C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334690: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233469C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823346A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823346A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823346A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823346B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823346B0 size=92
    let mut pc: u32 = 0x823346B0;
    'dispatch: loop {
        match pc {
            0x823346B0 => {
    //   block [0x823346B0..0x8233470C)
	// 823346B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823346B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823346B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823346BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823346C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823346C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823346C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823346CC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 823346D0: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 823346D4: 48ABF4FD  bl 0x82df3bd0
	ctx.lr = 0x823346D8;
	sub_82DF3BD0(ctx, base);
	// 823346D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823346DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823346E0: C1BE001C  lfs f13, 0x1c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823346E4: D1BF00CC  stfs f13, 0xcc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 823346E8: 995F00D4  stb r10, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u8 ) };
	// 823346EC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823346F0: D01F00D8  stfs f0, 0xd8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 823346F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823346F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823346FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334710 size=108
    let mut pc: u32 = 0x82334710;
    'dispatch: loop {
        match pc {
            0x82334710 => {
    //   block [0x82334710..0x8233477C)
	// 82334710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334714: 48E73A59  bl 0x831a816c
	ctx.lr = 0x82334718;
	sub_831A8130(ctx, base);
	// 82334718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233471C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82334720: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82334724: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82334728: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233472C: 41820038  beq 0x82334764
	if ctx.cr[0].eq {
	pc = 0x82334764; continue 'dispatch;
	}
	// 82334730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334734: 48E75255  bl 0x831a9988
	ctx.lr = 0x82334738;
	sub_831A9988(ctx, base);
	// 82334738: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233473C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82334740: 386B86C0  addi r3, r11, -0x7940
	ctx.r[3].s64 = ctx.r[11].s64 + -31040;
	// 82334744: 48E739B5  bl 0x831a80f8
	ctx.lr = 0x82334748;
	sub_831A80F8(ctx, base);
	// 82334748: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233474C: 41820018  beq 0x82334764
	if ctx.cr[0].eq {
	pc = 0x82334764; continue 'dispatch;
	}
	// 82334750: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334758: 4BFFFF59  bl 0x823346b0
	ctx.lr = 0x8233475C;
	sub_823346B0(ctx, base);
	// 8233475C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82334760: 48000014  b 0x82334774
	pc = 0x82334774; continue 'dispatch;
	// 82334764: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82334768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233476C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334770: 4BFFDAA1  bl 0x82332210
	ctx.lr = 0x82334774;
	sub_82332210(ctx, base);
	// 82334774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334778: 48E73A44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334780 size=196
    let mut pc: u32 = 0x82334780;
    'dispatch: loop {
        match pc {
            0x82334780 => {
    //   block [0x82334780..0x82334844)
	// 82334780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233478C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233479C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823347A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823347A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823347A8: 4BF8C191  bl 0x822c0938
	ctx.lr = 0x823347AC;
	sub_822C0938(ctx, base);
	// 823347AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823347B0: 41820028  beq 0x823347d8
	if ctx.cr[0].eq {
	pc = 0x823347D8; continue 'dispatch;
	}
	// 823347B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823347B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823347BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823347C0: 392B10A4  addi r9, r11, 0x10a4
	ctx.r[9].s64 = ctx.r[11].s64 + 4260;
	// 823347C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823347C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823347CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823347D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823347D4: 48000008  b 0x823347dc
	pc = 0x823347DC; continue 'dispatch;
	// 823347D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823347DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823347E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823347E4: 409A0044  bne cr6, 0x82334828
	if !ctx.cr[6].eq {
	pc = 0x82334828; continue 'dispatch;
	}
	// 823347E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823347EC: 419A001C  beq cr6, 0x82334808
	if ctx.cr[6].eq {
	pc = 0x82334808; continue 'dispatch;
	}
	// 823347F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823347F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823347F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823347FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82334800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82334804: 4E800421  bctrl
	ctx.lr = 0x82334808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82334808: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233480C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82334810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334814: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82334818: 816B86B8  lwz r11, -0x7948(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31048 as u32) ) } as u64;
	// 8233481C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82334820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82334824: 4BF8B7DD  bl 0x822c0000
	ctx.lr = 0x82334828;
	sub_822C0000(ctx, base);
	// 82334828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233482C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334838: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233483C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334848 size=196
    let mut pc: u32 = 0x82334848;
    'dispatch: loop {
        match pc {
            0x82334848 => {
    //   block [0x82334848..0x8233490C)
	// 82334848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233485C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82334864: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82334868: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233486C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82334870: 4BF8C0C9  bl 0x822c0938
	ctx.lr = 0x82334874;
	sub_822C0938(ctx, base);
	// 82334874: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82334878: 41820028  beq 0x823348a0
	if ctx.cr[0].eq {
	pc = 0x823348A0; continue 'dispatch;
	}
	// 8233487C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334880: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82334884: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82334888: 392B10B8  addi r9, r11, 0x10b8
	ctx.r[9].s64 = ctx.r[11].s64 + 4280;
	// 8233488C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82334890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82334894: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82334898: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233489C: 48000008  b 0x823348a4
	pc = 0x823348A4; continue 'dispatch;
	// 823348A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823348A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823348A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823348AC: 409A0044  bne cr6, 0x823348f0
	if !ctx.cr[6].eq {
	pc = 0x823348F0; continue 'dispatch;
	}
	// 823348B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823348B4: 419A001C  beq cr6, 0x823348d0
	if ctx.cr[6].eq {
	pc = 0x823348D0; continue 'dispatch;
	}
	// 823348B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823348BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823348C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823348C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823348C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823348CC: 4E800421  bctrl
	ctx.lr = 0x823348D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823348D0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823348D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823348D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823348DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823348E0: 816B86B8  lwz r11, -0x7948(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31048 as u32) ) } as u64;
	// 823348E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823348E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823348EC: 4BF8B715  bl 0x822c0000
	ctx.lr = 0x823348F0;
	sub_822C0000(ctx, base);
	// 823348F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823348F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823348F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823348FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334900: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334910 size=576
    let mut pc: u32 = 0x82334910;
    'dispatch: loop {
        match pc {
            0x82334910 => {
    //   block [0x82334910..0x82334B50)
	// 82334910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334914: 48E73855  bl 0x831a8168
	ctx.lr = 0x82334918;
	sub_831A8130(ctx, base);
	// 82334918: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8233491C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82334920: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334928: 486DBF49  bl 0x82a10870
	ctx.lr = 0x8233492C;
	sub_82A10870(ctx, base);
	// 8233492C: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82334930: 3B9F006C  addi r28, r31, 0x6c
	ctx.r[28].s64 = ctx.r[31].s64 + 108;
	// 82334934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334938: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233493C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82334940: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82334944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82334948: 4E800421  bctrl
	ctx.lr = 0x8233494C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233494C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334950: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334954: 388B1140  addi r4, r11, 0x1140
	ctx.r[4].s64 = ctx.r[11].s64 + 4416;
	// 82334958: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 8233495C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82334960: 48ABDA89  bl 0x82df23e8
	ctx.lr = 0x82334964;
	sub_82DF23E8(ctx, base);
	// 82334964: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82334968: 4182001C  beq 0x82334984
	if ctx.cr[0].eq {
	pc = 0x82334984; continue 'dispatch;
	}
	// 8233496C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334970: 48B27FB9  bl 0x82e5c928
	ctx.lr = 0x82334974;
	sub_82E5C928(ctx, base);
	// 82334974: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334978: 396B10CC  addi r11, r11, 0x10cc
	ctx.r[11].s64 = ctx.r[11].s64 + 4300;
	// 8233497C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82334980: 48000008  b 0x82334988
	pc = 0x82334988; continue 'dispatch;
	// 82334984: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82334988: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8233498C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82334990: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82334994: 4BFFFDED  bl 0x82334780
	ctx.lr = 0x82334998;
	sub_82334780(ctx, base);
	// 82334998: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8233499C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823349A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823349A4: 4BF8B65D  bl 0x822c0000
	ctx.lr = 0x823349A8;
	sub_822C0000(ctx, base);
	// 823349A8: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823349AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823349B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823349B4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 823349B8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 823349BC: 419A0024  beq cr6, 0x823349e0
	if ctx.cr[6].eq {
	pc = 0x823349E0; continue 'dispatch;
	}
	// 823349C0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 823349C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823349C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823349CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823349D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823349D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823349D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823349DC: 4082FFE8  bne 0x823349c4
	if !ctx.cr[0].eq {
	pc = 0x823349C4; continue 'dispatch;
	}
	// 823349E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823349E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823349E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823349EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 823349F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823349F4: C3CB08A4  lfs f30, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 823349F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823349FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82334A00: 48B29CA9  bl 0x82e5e6a8
	ctx.lr = 0x82334A04;
	sub_82E5E6A8(ctx, base);
	// 82334A04: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82334A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334A0C: 419A0008  beq cr6, 0x82334a14
	if ctx.cr[6].eq {
	pc = 0x82334A14; continue 'dispatch;
	}
	// 82334A10: 4BF8BE81  bl 0x822c0890
	ctx.lr = 0x82334A14;
	sub_822C0890(ctx, base);
	// 82334A14: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82334A18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334A1C: 419A0008  beq cr6, 0x82334a24
	if ctx.cr[6].eq {
	pc = 0x82334A24; continue 'dispatch;
	}
	// 82334A20: 4BF8BE71  bl 0x822c0890
	ctx.lr = 0x82334A24;
	sub_822C0890(ctx, base);
	// 82334A24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82334A28: 419A000C  beq cr6, 0x82334a34
	if ctx.cr[6].eq {
	pc = 0x82334A34; continue 'dispatch;
	}
	// 82334A2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82334A30: 4BF8BE61  bl 0x822c0890
	ctx.lr = 0x82334A34;
	sub_822C0890(ctx, base);
	// 82334A34: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334A38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334A3C: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 82334A40: 48ABEFC9  bl 0x82df3a08
	ctx.lr = 0x82334A44;
	sub_82DF3A08(ctx, base);
	// 82334A44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82334A48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334A4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334A50: 4BFD0AF1  bl 0x82305540
	ctx.lr = 0x82334A54;
	sub_82305540(ctx, base);
	// 82334A54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334A58: 48ABE9D1  bl 0x82df3428
	ctx.lr = 0x82334A5C;
	sub_82DF3428(ctx, base);
	// 82334A5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334A60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334A64: C3EB9534  lfs f31, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82334A68: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82334A6C: 4BFFA07D  bl 0x8232eae8
	ctx.lr = 0x82334A70;
	sub_8232EAE8(ctx, base);
	// 82334A70: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82334A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334A78: 4BFE0F59  bl 0x823159d0
	ctx.lr = 0x82334A7C;
	sub_823159D0(ctx, base);
	// 82334A7C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334A84: 808BB01C  lwz r4, -0x4fe4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20452 as u32) ) } as u64;
	// 82334A88: 48ABEF81  bl 0x82df3a08
	ctx.lr = 0x82334A8C;
	sub_82DF3A08(ctx, base);
	// 82334A8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334A90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334A94: 4BFD2E45  bl 0x823078d8
	ctx.lr = 0x82334A98;
	sub_823078D8(ctx, base);
	// 82334A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334A9C: 48ABE98D  bl 0x82df3428
	ctx.lr = 0x82334AA0;
	sub_82DF3428(ctx, base);
	// 82334AA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82334AA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334AA8: 4BFD10B9  bl 0x82305b60
	ctx.lr = 0x82334AAC;
	sub_82305B60(ctx, base);
	// 82334AAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82334AB0: D3FF00CC  stfs f31, 0xcc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82334AB4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82334AB8: 997F00D4  stb r11, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 82334ABC: 481B5FCD  bl 0x824eaa88
	ctx.lr = 0x82334AC0;
	sub_824EAA88(ctx, base);
	// 82334AC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82334AC4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82334AC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82334ACC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82334AD0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82334AD4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82334AD8: 557EDFFE  rlwinm r30, r11, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82334ADC: 48ABD1B5  bl 0x82df1c90
	ctx.lr = 0x82334AE0;
	sub_82DF1C90(ctx, base);
	// 82334AE0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82334AE4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82334AE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82334AEC: 808B271C  lwz r4, 0x271c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82334AF0: 41820028  beq 0x82334b18
	if ctx.cr[0].eq {
	pc = 0x82334B18; continue 'dispatch;
	}
	// 82334AF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334AF8: 48ABD101  bl 0x82df1bf8
	ctx.lr = 0x82334AFC;
	sub_82DF1BF8(ctx, base);
	// 82334AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334B00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82334B04: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82334B08: 388B112C  addi r4, r11, 0x112c
	ctx.r[4].s64 = ctx.r[11].s64 + 4396;
	// 82334B0C: 48B1B595  bl 0x82e500a0
	ctx.lr = 0x82334B10;
	sub_82E500A0(ctx, base);
	// 82334B10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334B14: 48000024  b 0x82334b38
	pc = 0x82334B38; continue 'dispatch;
	// 82334B18: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82334B1C: 48ABD0DD  bl 0x82df1bf8
	ctx.lr = 0x82334B20;
	sub_82DF1BF8(ctx, base);
	// 82334B20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82334B28: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82334B2C: 388B1118  addi r4, r11, 0x1118
	ctx.r[4].s64 = ctx.r[11].s64 + 4376;
	// 82334B30: 48B1B571  bl 0x82e500a0
	ctx.lr = 0x82334B34;
	sub_82E500A0(ctx, base);
	// 82334B34: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82334B38: 48ABD159  bl 0x82df1c90
	ctx.lr = 0x82334B3C;
	sub_82DF1C90(ctx, base);
	// 82334B3C: D3DF00D8  stfs f30, 0xd8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 82334B40: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82334B44: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82334B48: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82334B4C: 48E7366C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334B50 size=208
    let mut pc: u32 = 0x82334B50;
    'dispatch: loop {
        match pc {
            0x82334B50 => {
    //   block [0x82334B50..0x82334C20)
	// 82334B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334B58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334B5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334B68: 486DBD09  bl 0x82a10870
	ctx.lr = 0x82334B6C;
	sub_82A10870(ctx, base);
	// 82334B6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334B70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334B74: 48B24BF5  bl 0x82e59768
	ctx.lr = 0x82334B78;
	sub_82E59768(ctx, base);
	// 82334B78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82334B7C: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82334B80: 48B282F9  bl 0x82e5ce78
	ctx.lr = 0x82334B84;
	sub_82E5CE78(ctx, base);
	// 82334B84: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82334B88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82334B8C: 41820054  beq 0x82334be0
	if ctx.cr[0].eq {
	pc = 0x82334BE0; continue 'dispatch;
	}
	// 82334B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334B94: 48B24BE5  bl 0x82e59778
	ctx.lr = 0x82334B98;
	sub_82E59778(ctx, base);
	// 82334B98: C01F00CC  lfs f0, 0xcc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334B9C: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82334BA0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334BA4: D1BF00CC  stfs f13, 0xcc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82334BA8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334BAC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82334BB0: 40980030  bge cr6, 0x82334be0
	if !ctx.cr[6].lt {
	pc = 0x82334BE0; continue 'dispatch;
	}
	// 82334BB4: 38DF00D0  addi r6, r31, 0xd0
	ctx.r[6].s64 = ctx.r[31].s64 + 208;
	// 82334BB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82334BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82334BC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334BC4: 4BFD3B15  bl 0x823086d8
	ctx.lr = 0x82334BC8;
	sub_823086D8(ctx, base);
	// 82334BC8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82334BCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334BD0: 419A0008  beq cr6, 0x82334bd8
	if ctx.cr[6].eq {
	pc = 0x82334BD8; continue 'dispatch;
	}
	// 82334BD4: 4BF8BCBD  bl 0x822c0890
	ctx.lr = 0x82334BD8;
	sub_822C0890(ctx, base);
	// 82334BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82334BDC: 997F00D4  stb r11, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 82334BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334BE4: 48B24B7D  bl 0x82e59760
	ctx.lr = 0x82334BE8;
	sub_82E59760(ctx, base);
	// 82334BE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334BEC: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334BF0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82334BF4: 40990014  ble cr6, 0x82334c08
	if !ctx.cr[6].gt {
	pc = 0x82334C08; continue 'dispatch;
	}
	// 82334BF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C00: C02B9534  lfs f1, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82334C04: 4BFDE19D  bl 0x82312da0
	ctx.lr = 0x82334C08;
	sub_82312DA0(ctx, base);
	// 82334C08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334C14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334C18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334C20 size=176
    let mut pc: u32 = 0x82334C20;
    'dispatch: loop {
        match pc {
            0x82334C20 => {
    //   block [0x82334C20..0x82334CD0)
	// 82334C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334C38: 486DBC39  bl 0x82a10870
	ctx.lr = 0x82334C3C;
	sub_82A10870(ctx, base);
	// 82334C3C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334C40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334C44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334C48: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 82334C4C: 48ABEDBD  bl 0x82df3a08
	ctx.lr = 0x82334C50;
	sub_82DF3A08(ctx, base);
	// 82334C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334C58: 4BFD08F1  bl 0x82305548
	ctx.lr = 0x82334C5C;
	sub_82305548(ctx, base);
	// 82334C5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334C60: 48ABE7C9  bl 0x82df3428
	ctx.lr = 0x82334C64;
	sub_82DF3428(ctx, base);
	// 82334C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C68: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82334C6C: 4BFE5DED  bl 0x8231aa58
	ctx.lr = 0x82334C70;
	sub_8231AA58(ctx, base);
	// 82334C70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C74: 4BFE0D6D  bl 0x823159e0
	ctx.lr = 0x82334C78;
	sub_823159E0(ctx, base);
	// 82334C78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82334C80: 4BFD0EE1  bl 0x82305b60
	ctx.lr = 0x82334C84;
	sub_82305B60(ctx, base);
	// 82334C84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334C88: 4BFDE159  bl 0x82312de0
	ctx.lr = 0x82334C8C;
	sub_82312DE0(ctx, base);
	// 82334C8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82334C90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82334C94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82334C98: 808B271C  lwz r4, 0x271c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82334C9C: 48ABCF5D  bl 0x82df1bf8
	ctx.lr = 0x82334CA0;
	sub_82DF1BF8(ctx, base);
	// 82334CA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334CA4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82334CA8: 388B112C  addi r4, r11, 0x112c
	ctx.r[4].s64 = ctx.r[11].s64 + 4396;
	// 82334CAC: 48B1B43D  bl 0x82e500e8
	ctx.lr = 0x82334CB0;
	sub_82E500E8(ctx, base);
	// 82334CB0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82334CB4: 48ABCFDD  bl 0x82df1c90
	ctx.lr = 0x82334CB8;
	sub_82DF1C90(ctx, base);
	// 82334CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82334CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334CC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334CC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334CD0 size=368
    let mut pc: u32 = 0x82334CD0;
    'dispatch: loop {
        match pc {
            0x82334CD0 => {
    //   block [0x82334CD0..0x82334E40)
	// 82334CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334CD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334CDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334CE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334CE8: 48E1F919  bl 0x83154600
	ctx.lr = 0x82334CEC;
	sub_83154600(ctx, base);
	// 82334CEC: 486DBB85  bl 0x82a10870
	ctx.lr = 0x82334CF0;
	sub_82A10870(ctx, base);
	// 82334CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334CF4: 4BFE22ED  bl 0x82316fe0
	ctx.lr = 0x82334CF8;
	sub_82316FE0(ctx, base);
	// 82334CF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334CFC: C00B964C  lfs f0, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334D00: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82334D04: 4198001C  blt cr6, 0x82334d20
	if ctx.cr[6].lt {
	pc = 0x82334D20; continue 'dispatch;
	}
	// 82334D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334D0C: 48B24A55  bl 0x82e59760
	ctx.lr = 0x82334D10;
	sub_82E59760(ctx, base);
	// 82334D10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334D14: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334D18: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82334D1C: 4099010C  ble cr6, 0x82334e28
	if !ctx.cr[6].gt {
	pc = 0x82334E28; continue 'dispatch;
	}
	// 82334D20: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334D24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334D28: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 82334D2C: 48ABECDD  bl 0x82df3a08
	ctx.lr = 0x82334D30;
	sub_82DF3A08(ctx, base);
	// 82334D30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334D38: 4BFD2BA1  bl 0x823078d8
	ctx.lr = 0x82334D3C;
	sub_823078D8(ctx, base);
	// 82334D3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334D40: 48ABE6E9  bl 0x82df3428
	ctx.lr = 0x82334D44;
	sub_82DF3428(ctx, base);
	// 82334D44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334D4C: 388B1140  addi r4, r11, 0x1140
	ctx.r[4].s64 = ctx.r[11].s64 + 4416;
	// 82334D50: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 82334D54: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82334D58: 48ABD691  bl 0x82df23e8
	ctx.lr = 0x82334D5C;
	sub_82DF23E8(ctx, base);
	// 82334D5C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82334D60: 4182001C  beq 0x82334d7c
	if ctx.cr[0].eq {
	pc = 0x82334D7C; continue 'dispatch;
	}
	// 82334D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334D68: 48B27BC1  bl 0x82e5c928
	ctx.lr = 0x82334D6C;
	sub_82E5C928(ctx, base);
	// 82334D6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82334D70: 396B10F4  addi r11, r11, 0x10f4
	ctx.r[11].s64 = ctx.r[11].s64 + 4340;
	// 82334D74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82334D78: 48000008  b 0x82334d80
	pc = 0x82334D80; continue 'dispatch;
	// 82334D7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82334D80: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82334D84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334D88: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82334D8C: 4BFFFABD  bl 0x82334848
	ctx.lr = 0x82334D90;
	sub_82334848(ctx, base);
	// 82334D90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82334D94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334D98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82334D9C: 4BF8B265  bl 0x822c0000
	ctx.lr = 0x82334DA0;
	sub_822C0000(ctx, base);
	// 82334DA0: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82334DA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82334DA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82334DAC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82334DB0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82334DB4: 419A0024  beq cr6, 0x82334dd8
	if ctx.cr[6].eq {
	pc = 0x82334DD8; continue 'dispatch;
	}
	// 82334DB8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82334DBC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82334DC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82334DC4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82334DC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82334DCC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82334DD0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82334DD4: 4082FFE8  bne 0x82334dbc
	if !ctx.cr[0].eq {
	pc = 0x82334DBC; continue 'dispatch;
	}
	// 82334DD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82334DE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334DE4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82334DE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82334DEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334DF0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82334DF4: 48B29ACD  bl 0x82e5e8c0
	ctx.lr = 0x82334DF8;
	sub_82E5E8C0(ctx, base);
	// 82334DF8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82334DFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334E00: 419A0008  beq cr6, 0x82334e08
	if ctx.cr[6].eq {
	pc = 0x82334E08; continue 'dispatch;
	}
	// 82334E04: 4BF8BA8D  bl 0x822c0890
	ctx.lr = 0x82334E08;
	sub_822C0890(ctx, base);
	// 82334E08: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82334E0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334E10: 419A0008  beq cr6, 0x82334e18
	if ctx.cr[6].eq {
	pc = 0x82334E18; continue 'dispatch;
	}
	// 82334E14: 4BF8BA7D  bl 0x822c0890
	ctx.lr = 0x82334E18;
	sub_822C0890(ctx, base);
	// 82334E18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82334E1C: 419A000C  beq cr6, 0x82334e28
	if ctx.cr[6].eq {
	pc = 0x82334E28; continue 'dispatch;
	}
	// 82334E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334E24: 4BF8BA6D  bl 0x822c0890
	ctx.lr = 0x82334E28;
	sub_822C0890(ctx, base);
	// 82334E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82334E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334E34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334E40 size=196
    let mut pc: u32 = 0x82334E40;
    'dispatch: loop {
        match pc {
            0x82334E40 => {
    //   block [0x82334E40..0x82334F04)
	// 82334E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334E54: 48E1F7AD  bl 0x83154600
	ctx.lr = 0x82334E58;
	sub_83154600(ctx, base);
	// 82334E58: 486DBA19  bl 0x82a10870
	ctx.lr = 0x82334E5C;
	sub_82A10870(ctx, base);
	// 82334E5C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82334E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334E64: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82334E68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334E6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82334E70: 4BFD4681  bl 0x823094f0
	ctx.lr = 0x82334E74;
	sub_823094F0(ctx, base);
	// 82334E74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82334E78: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82334E7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82334E80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82334E84: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82334E88: 419A0024  beq cr6, 0x82334eac
	if ctx.cr[6].eq {
	pc = 0x82334EAC; continue 'dispatch;
	}
	// 82334E8C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82334E90: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82334E94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82334E98: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82334E9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82334EA0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82334EA4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82334EA8: 4082FFE8  bne 0x82334e90
	if !ctx.cr[0].eq {
	pc = 0x82334E90; continue 'dispatch;
	}
	// 82334EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334EB0: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82334EB4: 4BFD0EA5  bl 0x82305d58
	ctx.lr = 0x82334EB8;
	sub_82305D58(ctx, base);
	// 82334EB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334EBC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82334EC0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82334EC4: 388A1140  addi r4, r10, 0x1140
	ctx.r[4].s64 = ctx.r[10].s64 + 4416;
	// 82334EC8: 38A00091  li r5, 0x91
	ctx.r[5].s64 = 145;
	// 82334ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334ED0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82334ED4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82334ED8: 4BFD2FB1  bl 0x82307e88
	ctx.lr = 0x82334EDC;
	sub_82307E88(ctx, base);
	// 82334EDC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82334EE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82334EE4: 419A0008  beq cr6, 0x82334eec
	if ctx.cr[6].eq {
	pc = 0x82334EEC; continue 'dispatch;
	}
	// 82334EE8: 4BF8B9A9  bl 0x822c0890
	ctx.lr = 0x82334EEC;
	sub_822C0890(ctx, base);
	// 82334EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82334EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334EF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334EFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82334F08 size=112
    let mut pc: u32 = 0x82334F08;
    'dispatch: loop {
        match pc {
            0x82334F08 => {
    //   block [0x82334F08..0x82334F78)
	// 82334F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82334F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334F20: 486DB951  bl 0x82a10870
	ctx.lr = 0x82334F24;
	sub_82A10870(ctx, base);
	// 82334F24: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334F28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82334F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334F30: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 82334F34: 48ABEAD5  bl 0x82df3a08
	ctx.lr = 0x82334F38;
	sub_82DF3A08(ctx, base);
	// 82334F38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334F3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82334F40: 4BFD0609  bl 0x82305548
	ctx.lr = 0x82334F44;
	sub_82305548(ctx, base);
	// 82334F44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334F48: 48ABE4E1  bl 0x82df3428
	ctx.lr = 0x82334F4C;
	sub_82DF3428(ctx, base);
	// 82334F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334F50: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82334F54: 4BFE5B05  bl 0x8231aa58
	ctx.lr = 0x82334F58;
	sub_8231AA58(ctx, base);
	// 82334F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82334F5C: 4BFE0A85  bl 0x823159e0
	ctx.lr = 0x82334F60;
	sub_823159E0(ctx, base);
	// 82334F60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82334F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82334F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82334F6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82334F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82334F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82334F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82334F78 size=200
    let mut pc: u32 = 0x82334F78;
    'dispatch: loop {
        match pc {
            0x82334F78 => {
    //   block [0x82334F78..0x82335040)
	// 82334F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82334F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82334F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82334F84: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82334F88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82334F8C: 486DB8E5  bl 0x82a10870
	ctx.lr = 0x82334F90;
	sub_82A10870(ctx, base);
	// 82334F90: 4BFDDC79  bl 0x82312c08
	ctx.lr = 0x82334F94;
	sub_82312C08(ctx, base);
	// 82334F94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82334F98: 4182003C  beq 0x82334fd4
	if ctx.cr[0].eq {
	pc = 0x82334FD4; continue 'dispatch;
	}
	// 82334F9C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334FA4: 808BB3F0  lwz r4, -0x4c10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19472 as u32) ) } as u64;
	// 82334FA8: 48ABEA61  bl 0x82df3a08
	ctx.lr = 0x82334FAC;
	sub_82DF3A08(ctx, base);
	// 82334FAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334FB0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82334FB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82334FB8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82334FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82334FC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82334FC4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82334FC8: 48B29AC9  bl 0x82e5ea90
	ctx.lr = 0x82334FCC;
	sub_82E5EA90(ctx, base);
	// 82334FCC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82334FD0: 48000050  b 0x82335020
	pc = 0x82335020; continue 'dispatch;
	// 82334FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82334FD8: 48B24789  bl 0x82e59760
	ctx.lr = 0x82334FDC;
	sub_82E59760(ctx, base);
	// 82334FDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82334FE0: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82334FE4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82334FE8: 40990044  ble cr6, 0x8233502c
	if !ctx.cr[6].gt {
	pc = 0x8233502C; continue 'dispatch;
	}
	// 82334FEC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82334FF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82334FF4: 808BB3F0  lwz r4, -0x4c10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19472 as u32) ) } as u64;
	// 82334FF8: 48ABEA11  bl 0x82df3a08
	ctx.lr = 0x82334FFC;
	sub_82DF3A08(ctx, base);
	// 82334FFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82335000: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82335004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335008: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233500C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82335010: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82335014: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335018: 48B29A79  bl 0x82e5ea90
	ctx.lr = 0x8233501C;
	sub_82E5EA90(ctx, base);
	// 8233501C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82335020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82335024: 419A0008  beq cr6, 0x8233502c
	if ctx.cr[6].eq {
	pc = 0x8233502C; continue 'dispatch;
	}
	// 82335028: 4BF8B869  bl 0x822c0890
	ctx.lr = 0x8233502C;
	sub_822C0890(ctx, base);
	// 8233502C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82335030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335040 size=376
    let mut pc: u32 = 0x82335040;
    'dispatch: loop {
        match pc {
            0x82335040 => {
    //   block [0x82335040..0x823351B8)
	// 82335040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233504C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335054: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335058: 486DB819  bl 0x82a10870
	ctx.lr = 0x8233505C;
	sub_82A10870(ctx, base);
	// 8233505C: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82335060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335064: 4BFDEC7D  bl 0x82313ce0
	ctx.lr = 0x82335068;
	sub_82313CE0(ctx, base);
	// 82335068: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233506C: 40820018  bne 0x82335084
	if !ctx.cr[0].eq {
	pc = 0x82335084; continue 'dispatch;
	}
	// 82335070: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82335074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335078: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8233507C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82335080: 4E800421  bctrl
	ctx.lr = 0x82335084;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82335084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335088: 4BFDDB81  bl 0x82312c08
	ctx.lr = 0x8233508C;
	sub_82312C08(ctx, base);
	// 8233508C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335094: 41820044  beq 0x823350d8
	if ctx.cr[0].eq {
	pc = 0x823350D8; continue 'dispatch;
	}
	// 82335098: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233509C: 808BB3F0  lwz r4, -0x4c10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19472 as u32) ) } as u64;
	// 823350A0: 48ABE969  bl 0x82df3a08
	ctx.lr = 0x823350A4;
	sub_82DF3A08(ctx, base);
	// 823350A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823350A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823350AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823350B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823350B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823350B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823350BC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823350C0: 48B299D1  bl 0x82e5ea90
	ctx.lr = 0x823350C4;
	sub_82E5EA90(ctx, base);
	// 823350C4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823350C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823350CC: 419A00D4  beq cr6, 0x823351a0
	if ctx.cr[6].eq {
	pc = 0x823351A0; continue 'dispatch;
	}
	// 823350D0: 4BF8B7C1  bl 0x822c0890
	ctx.lr = 0x823350D4;
	sub_822C0890(ctx, base);
	// 823350D4: 480000CC  b 0x823351a0
	pc = 0x823351A0; continue 'dispatch;
	// 823350D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823350DC: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 823350E0: 48ABE929  bl 0x82df3a08
	ctx.lr = 0x823350E4;
	sub_82DF3A08(ctx, base);
	// 823350E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823350E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823350EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823350F0: 4BFD0451  bl 0x82305540
	ctx.lr = 0x823350F4;
	sub_82305540(ctx, base);
	// 823350F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823350F8: 48ABE331  bl 0x82df3428
	ctx.lr = 0x823350FC;
	sub_82DF3428(ctx, base);
	// 823350FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335104: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335108: 4BFF99E1  bl 0x8232eae8
	ctx.lr = 0x8233510C;
	sub_8232EAE8(ctx, base);
	// 8233510C: 907E0068  stw r3, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82335110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335114: 4BFE08BD  bl 0x823159d0
	ctx.lr = 0x82335118;
	sub_823159D0(ctx, base);
	// 82335118: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233511C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335120: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 82335124: 48ABE8E5  bl 0x82df3a08
	ctx.lr = 0x82335128;
	sub_82DF3A08(ctx, base);
	// 82335128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233512C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335130: 48B24861  bl 0x82e59990
	ctx.lr = 0x82335134;
	sub_82E59990(ctx, base);
	// 82335134: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82335138: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8233513C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82335140: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82335144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82335148: 4E800421  bctrl
	ctx.lr = 0x8233514C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233514C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82335150: 48ABE151  bl 0x82df32a0
	ctx.lr = 0x82335154;
	sub_82DF32A0(ctx, base);
	// 82335154: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82335158: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233515C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82335160: 419A000C  beq cr6, 0x8233516c
	if ctx.cr[6].eq {
	pc = 0x8233516C; continue 'dispatch;
	}
	// 82335164: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82335168: 4BF8B729  bl 0x822c0890
	ctx.lr = 0x8233516C;
	sub_822C0890(ctx, base);
	// 8233516C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335170: 48ABE2B9  bl 0x82df3428
	ctx.lr = 0x82335174;
	sub_82DF3428(ctx, base);
	// 82335174: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335178: 41820028  beq 0x823351a0
	if ctx.cr[0].eq {
	pc = 0x823351A0; continue 'dispatch;
	}
	// 8233517C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335184: 808BB060  lwz r4, -0x4fa0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 82335188: 48ABE881  bl 0x82df3a08
	ctx.lr = 0x8233518C;
	sub_82DF3A08(ctx, base);
	// 8233518C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82335190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335194: 4BFD2745  bl 0x823078d8
	ctx.lr = 0x82335198;
	sub_823078D8(ctx, base);
	// 82335198: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233519C: 48ABE28D  bl 0x82df3428
	ctx.lr = 0x823351A0;
	sub_82DF3428(ctx, base);
	// 823351A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823351A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823351A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823351AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823351B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823351B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823351B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823351B8 size=72
    let mut pc: u32 = 0x823351B8;
    'dispatch: loop {
        match pc {
            0x823351B8 => {
    //   block [0x823351B8..0x82335200)
	// 823351B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823351BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823351C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823351C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823351C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823351CC: 486DB6A5  bl 0x82a10870
	ctx.lr = 0x823351D0;
	sub_82A10870(ctx, base);
	// 823351D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823351D4: 48B24595  bl 0x82e59768
	ctx.lr = 0x823351D8;
	sub_82E59768(ctx, base);
	// 823351D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823351DC: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 823351E0: 48B27C99  bl 0x82e5ce78
	ctx.lr = 0x823351E4;
	sub_82E5CE78(ctx, base);
	// 823351E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823351E8: 48D928F9  bl 0x830c7ae0
	ctx.lr = 0x823351EC;
	sub_830C7AE0(ctx, base);
	// 823351EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823351F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823351F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823351F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823351FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335200 size=84
    let mut pc: u32 = 0x82335200;
    'dispatch: loop {
        match pc {
            0x82335200 => {
    //   block [0x82335200..0x82335254)
	// 82335200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233520C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335210: 48E1F3F1  bl 0x83154600
	ctx.lr = 0x82335214;
	sub_83154600(ctx, base);
	// 82335214: 486DB65D  bl 0x82a10870
	ctx.lr = 0x82335218;
	sub_82A10870(ctx, base);
	// 82335218: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233521C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335224: 808BB180  lwz r4, -0x4e80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20096 as u32) ) } as u64;
	// 82335228: 48ABE7E1  bl 0x82df3a08
	ctx.lr = 0x8233522C;
	sub_82DF3A08(ctx, base);
	// 8233522C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82335230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335234: 4BFD26A5  bl 0x823078d8
	ctx.lr = 0x82335238;
	sub_823078D8(ctx, base);
	// 82335238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233523C: 48ABE1ED  bl 0x82df3428
	ctx.lr = 0x82335240;
	sub_82DF3428(ctx, base);
	// 82335240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233524C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335258 size=156
    let mut pc: u32 = 0x82335258;
    'dispatch: loop {
        match pc {
            0x82335258 => {
    //   block [0x82335258..0x823352F4)
	// 82335258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233525C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82335264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233526C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335270: 48E1F391  bl 0x83154600
	ctx.lr = 0x82335274;
	sub_83154600(ctx, base);
	// 82335274: 486DB5FD  bl 0x82a10870
	ctx.lr = 0x82335278;
	sub_82A10870(ctx, base);
	// 82335278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233527C: 4BFD0235  bl 0x823054b0
	ctx.lr = 0x82335280;
	sub_823054B0(ctx, base);
	// 82335280: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82335284: C00B2784  lfs f0, 0x2784(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82335288: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233528C: 40990050  ble cr6, 0x823352dc
	if !ctx.cr[6].gt {
	pc = 0x823352DC; continue 'dispatch;
	}
	// 82335290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335294: 4BFE5F15  bl 0x8231b1a8
	ctx.lr = 0x82335298;
	sub_8231B1A8(ctx, base);
	// 82335298: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 8233529C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823352A0: 4BFDE899  bl 0x82313b38
	ctx.lr = 0x823352A4;
	sub_82313B38(ctx, base);
	// 823352A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823352A8: C03E0070  lfs f1, 0x70(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823352AC: 4BFF98BD  bl 0x8232eb68
	ctx.lr = 0x823352B0;
	sub_8232EB68(ctx, base);
	// 823352B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823352B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823352B8: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 823352BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823352C0: 4E800421  bctrl
	ctx.lr = 0x823352C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823352C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 823352C8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823352CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823352D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823352D4: C02BD7BC  lfs f1, -0x2844(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823352D8: 4BFDE2E1  bl 0x823135b8
	ctx.lr = 0x823352DC;
	sub_823135B8(ctx, base);
	// 823352DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823352E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823352E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823352E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823352EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823352F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823352F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823352F8 size=84
    let mut pc: u32 = 0x823352F8;
    'dispatch: loop {
        match pc {
            0x823352F8 => {
    //   block [0x823352F8..0x8233534C)
	// 823352F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823352FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335304: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335308: 48E1F2F9  bl 0x83154600
	ctx.lr = 0x8233530C;
	sub_83154600(ctx, base);
	// 8233530C: 486DB565  bl 0x82a10870
	ctx.lr = 0x82335310;
	sub_82A10870(ctx, base);
	// 82335310: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233531C: 808BB18C  lwz r4, -0x4e74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20084 as u32) ) } as u64;
	// 82335320: 48ABE6E9  bl 0x82df3a08
	ctx.lr = 0x82335324;
	sub_82DF3A08(ctx, base);
	// 82335324: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82335328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233532C: 4BFD25AD  bl 0x823078d8
	ctx.lr = 0x82335330;
	sub_823078D8(ctx, base);
	// 82335330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335334: 48ABE0F5  bl 0x82df3428
	ctx.lr = 0x82335338;
	sub_82DF3428(ctx, base);
	// 82335338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335350 size=196
    let mut pc: u32 = 0x82335350;
    'dispatch: loop {
        match pc {
            0x82335350 => {
    //   block [0x82335350..0x82335414)
	// 82335350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233535C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233536C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82335370: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82335374: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335378: 4BF8B5C1  bl 0x822c0938
	ctx.lr = 0x8233537C;
	sub_822C0938(ctx, base);
	// 8233537C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82335380: 41820028  beq 0x823353a8
	if ctx.cr[0].eq {
	pc = 0x823353A8; continue 'dispatch;
	}
	// 82335384: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335388: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233538C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82335390: 392B11AC  addi r9, r11, 0x11ac
	ctx.r[9].s64 = ctx.r[11].s64 + 4524;
	// 82335394: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82335398: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233539C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823353A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823353A4: 48000008  b 0x823353ac
	pc = 0x823353AC; continue 'dispatch;
	// 823353A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823353AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823353B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823353B4: 409A0044  bne cr6, 0x823353f8
	if !ctx.cr[6].eq {
	pc = 0x823353F8; continue 'dispatch;
	}
	// 823353B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823353BC: 419A001C  beq cr6, 0x823353d8
	if ctx.cr[6].eq {
	pc = 0x823353D8; continue 'dispatch;
	}
	// 823353C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823353C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823353C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823353CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823353D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823353D4: 4E800421  bctrl
	ctx.lr = 0x823353D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823353D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823353DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823353E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823353E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823353E8: 816B88B8  lwz r11, -0x7748(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30536 as u32) ) } as u64;
	// 823353EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823353F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823353F4: 4BF8AC0D  bl 0x822c0000
	ctx.lr = 0x823353F8;
	sub_822C0000(ctx, base);
	// 823353F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823353FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233540C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335418 size=196
    let mut pc: u32 = 0x82335418;
    'dispatch: loop {
        match pc {
            0x82335418 => {
    //   block [0x82335418..0x823354DC)
	// 82335418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82335424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233542C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82335434: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82335438: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233543C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335440: 4BF8B4F9  bl 0x822c0938
	ctx.lr = 0x82335444;
	sub_822C0938(ctx, base);
	// 82335444: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82335448: 41820028  beq 0x82335470
	if ctx.cr[0].eq {
	pc = 0x82335470; continue 'dispatch;
	}
	// 8233544C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335450: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82335454: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82335458: 392B11C0  addi r9, r11, 0x11c0
	ctx.r[9].s64 = ctx.r[11].s64 + 4544;
	// 8233545C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82335460: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82335464: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82335468: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233546C: 48000008  b 0x82335474
	pc = 0x82335474; continue 'dispatch;
	// 82335470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82335474: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335478: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233547C: 409A0044  bne cr6, 0x823354c0
	if !ctx.cr[6].eq {
	pc = 0x823354C0; continue 'dispatch;
	}
	// 82335480: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82335484: 419A001C  beq cr6, 0x823354a0
	if ctx.cr[6].eq {
	pc = 0x823354A0; continue 'dispatch;
	}
	// 82335488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233548C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82335490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335494: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82335498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233549C: 4E800421  bctrl
	ctx.lr = 0x823354A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823354A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823354A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823354A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823354AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823354B0: 816B88B8  lwz r11, -0x7748(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30536 as u32) ) } as u64;
	// 823354B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823354B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823354BC: 4BF8AB45  bl 0x822c0000
	ctx.lr = 0x823354C0;
	sub_822C0000(ctx, base);
	// 823354C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823354C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823354C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823354CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823354D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823354D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823354D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823354E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823354E0 size=196
    let mut pc: u32 = 0x823354E0;
    'dispatch: loop {
        match pc {
            0x823354E0 => {
    //   block [0x823354E0..0x823355A4)
	// 823354E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823354E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823354E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823354EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823354F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823354F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823354F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823354FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82335500: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82335504: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335508: 4BF8B431  bl 0x822c0938
	ctx.lr = 0x8233550C;
	sub_822C0938(ctx, base);
	// 8233550C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82335510: 41820028  beq 0x82335538
	if ctx.cr[0].eq {
	pc = 0x82335538; continue 'dispatch;
	}
	// 82335514: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335518: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233551C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82335520: 392B11D4  addi r9, r11, 0x11d4
	ctx.r[9].s64 = ctx.r[11].s64 + 4564;
	// 82335524: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82335528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233552C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82335530: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82335534: 48000008  b 0x8233553c
	pc = 0x8233553C; continue 'dispatch;
	// 82335538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233553C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335540: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82335544: 409A0044  bne cr6, 0x82335588
	if !ctx.cr[6].eq {
	pc = 0x82335588; continue 'dispatch;
	}
	// 82335548: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233554C: 419A001C  beq cr6, 0x82335568
	if ctx.cr[6].eq {
	pc = 0x82335568; continue 'dispatch;
	}
	// 82335550: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82335554: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82335558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233555C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82335560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82335564: 4E800421  bctrl
	ctx.lr = 0x82335568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82335568: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233556C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82335570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335574: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82335578: 816B88B8  lwz r11, -0x7748(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30536 as u32) ) } as u64;
	// 8233557C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82335580: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82335584: 4BF8AA7D  bl 0x822c0000
	ctx.lr = 0x82335588;
	sub_822C0000(ctx, base);
	// 82335588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233558C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335598: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233559C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823355A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823355A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823355A8 size=212
    let mut pc: u32 = 0x823355A8;
    'dispatch: loop {
        match pc {
            0x823355A8 => {
    //   block [0x823355A8..0x8233567C)
	// 823355A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823355AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823355B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823355B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823355B8: 48E1F049  bl 0x83154600
	ctx.lr = 0x823355BC;
	sub_83154600(ctx, base);
	// 823355BC: 486DB2B5  bl 0x82a10870
	ctx.lr = 0x823355C0;
	sub_82A10870(ctx, base);
	// 823355C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823355C4: 4BFCFEED  bl 0x823054b0
	ctx.lr = 0x823355C8;
	sub_823054B0(ctx, base);
	// 823355C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 823355CC: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823355D0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823355D4: 40990094  ble cr6, 0x82335668
	if !ctx.cr[6].gt {
	pc = 0x82335668; continue 'dispatch;
	}
	// 823355D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823355DC: 4BFE5BCD  bl 0x8231b1a8
	ctx.lr = 0x823355E0;
	sub_8231B1A8(ctx, base);
	// 823355E0: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 823355E4: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823355E8: 48125801  bl 0x8245ade8
	ctx.lr = 0x823355EC;
	sub_8245ADE8(ctx, base);
	// 823355EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823355F0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823355F4: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823355F8: 4BFDE571  bl 0x82313b68
	ctx.lr = 0x823355FC;
	sub_82313B68(ctx, base);
	// 823355FC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82335600: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82335604: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82335608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233560C: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335680 size=360
    let mut pc: u32 = 0x82335680;
    'dispatch: loop {
        match pc {
            0x82335680 => {
    //   block [0x82335680..0x823357E8)
	// 82335680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233568C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335698: 48D92449  bl 0x830c7ae0
	ctx.lr = 0x8233569C;
	sub_830C7AE0(ctx, base);
	// 8233569C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823356A0: 486DB1D1  bl 0x82a10870
	ctx.lr = 0x823356A4;
	sub_82A10870(ctx, base);
	// 823356A4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823356A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823356AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823356B0: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 823356B4: 48ABE355  bl 0x82df3a08
	ctx.lr = 0x823356B8;
	sub_82DF3A08(ctx, base);
	// 823356B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823356BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823356C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823356C4: 4BFCFE7D  bl 0x82305540
	ctx.lr = 0x823356C8;
	sub_82305540(ctx, base);
	// 823356C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823356CC: 48ABDD5D  bl 0x82df3428
	ctx.lr = 0x823356D0;
	sub_82DF3428(ctx, base);
	// 823356D0: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 823356D4: 3BDF006C  addi r30, r31, 0x6c
	ctx.r[30].s64 = ctx.r[31].s64 + 108;
	// 823356D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823356DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823356E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823356E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823356E8: 4E800421  bctrl
	ctx.lr = 0x823356EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823356EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823356F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823356F4: 388B1260  addi r4, r11, 0x1260
	ctx.r[4].s64 = ctx.r[11].s64 + 4704;
	// 823356F8: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 823356FC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82335700: 48ABCCE9  bl 0x82df23e8
	ctx.lr = 0x82335704;
	sub_82DF23E8(ctx, base);
	// 82335704: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82335708: 4182001C  beq 0x82335724
	if ctx.cr[0].eq {
	pc = 0x82335724; continue 'dispatch;
	}
	// 8233570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335710: 48B27219  bl 0x82e5c928
	ctx.lr = 0x82335714;
	sub_82E5C928(ctx, base);
	// 82335714: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335718: 396B11E8  addi r11, r11, 0x11e8
	ctx.r[11].s64 = ctx.r[11].s64 + 4584;
	// 8233571C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335720: 48000008  b 0x82335728
	pc = 0x82335728; continue 'dispatch;
	// 82335724: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82335728: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8233572C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335730: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335734: 4BFFFC1D  bl 0x82335350
	ctx.lr = 0x82335738;
	sub_82335350(ctx, base);
	// 82335738: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233573C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335740: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335744: 4BF8A8BD  bl 0x822c0000
	ctx.lr = 0x82335748;
	sub_822C0000(ctx, base);
	// 82335748: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233574C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82335750: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82335754: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82335758: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8233575C: 419A0024  beq cr6, 0x82335780
	if ctx.cr[6].eq {
	pc = 0x82335780; continue 'dispatch;
	}
	// 82335760: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82335764: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82335768: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233576C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82335770: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82335774: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82335778: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233577C: 4082FFE8  bne 0x82335764
	if !ctx.cr[0].eq {
	pc = 0x82335764; continue 'dispatch;
	}
	// 82335780: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82335784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82335788: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233578C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82335790: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82335794: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335798: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233579C: 48B28F0D  bl 0x82e5e6a8
	ctx.lr = 0x823357A0;
	sub_82E5E6A8(ctx, base);
	// 823357A0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823357A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823357A8: 419A0008  beq cr6, 0x823357b0
	if ctx.cr[6].eq {
	pc = 0x823357B0; continue 'dispatch;
	}
	// 823357AC: 4BF8B0E5  bl 0x822c0890
	ctx.lr = 0x823357B0;
	sub_822C0890(ctx, base);
	// 823357B0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823357B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823357B8: 419A0008  beq cr6, 0x823357c0
	if ctx.cr[6].eq {
	pc = 0x823357C0; continue 'dispatch;
	}
	// 823357BC: 4BF8B0D5  bl 0x822c0890
	ctx.lr = 0x823357C0;
	sub_822C0890(ctx, base);
	// 823357C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823357C4: 419A000C  beq cr6, 0x823357d0
	if ctx.cr[6].eq {
	pc = 0x823357D0; continue 'dispatch;
	}
	// 823357C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823357CC: 4BF8B0C5  bl 0x822c0890
	ctx.lr = 0x823357D0;
	sub_822C0890(ctx, base);
	// 823357D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823357D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823357D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823357DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823357E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823357E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823357E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823357E8 size=204
    let mut pc: u32 = 0x823357E8;
    'dispatch: loop {
        match pc {
            0x823357E8 => {
    //   block [0x823357E8..0x823358B4)
	// 823357E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823357EC: 48E7297D  bl 0x831a8168
	ctx.lr = 0x823357F0;
	sub_831A8130(ctx, base);
	// 823357F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823357F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823357F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823357FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82335800: 486DB071  bl 0x82a10870
	ctx.lr = 0x82335804;
	sub_82A10870(ctx, base);
	// 82335804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335808: 9B9D0068  stb r28, 0x68(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[28].u8 ) };
	// 8233580C: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 82335810: 4198002C  blt cr6, 0x8233583c
	if ctx.cr[6].lt {
	pc = 0x8233583C; continue 'dispatch;
	}
	// 82335814: 409A0098  bne cr6, 0x823358ac
	if !ctx.cr[6].eq {
	pc = 0x823358AC; continue 'dispatch;
	}
	// 82335818: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233581C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335820: 808BB178  lwz r4, -0x4e88(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20104 as u32) ) } as u64;
	// 82335824: 48ABE1E5  bl 0x82df3a08
	ctx.lr = 0x82335828;
	sub_82DF3A08(ctx, base);
	// 82335828: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233582C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335830: 4BFD20A9  bl 0x823078d8
	ctx.lr = 0x82335834;
	sub_823078D8(ctx, base);
	// 82335834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335838: 48000070  b 0x823358a8
	pc = 0x823358A8; continue 'dispatch;
	// 8233583C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335840: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335844: 808BB184  lwz r4, -0x4e7c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20092 as u32) ) } as u64;
	// 82335848: 48ABE1C1  bl 0x82df3a08
	ctx.lr = 0x8233584C;
	sub_82DF3A08(ctx, base);
	// 8233584C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82335850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335854: 4BFD2085  bl 0x823078d8
	ctx.lr = 0x82335858;
	sub_823078D8(ctx, base);
	// 82335858: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233585C: 48ABDBCD  bl 0x82df3428
	ctx.lr = 0x82335860;
	sub_82DF3428(ctx, base);
	// 82335860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335864: 4BFDDE95  bl 0x823136f8
	ctx.lr = 0x82335868;
	sub_823136F8(ctx, base);
	// 82335868: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233586C: 40820040  bne 0x823358ac
	if !ctx.cr[0].eq {
	pc = 0x823358AC; continue 'dispatch;
	}
	// 82335870: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335874: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82335878: 388B12BC  addi r4, r11, 0x12bc
	ctx.r[4].s64 = ctx.r[11].s64 + 4796;
	// 8233587C: 48ABE18D  bl 0x82df3a08
	ctx.lr = 0x82335880;
	sub_82DF3A08(ctx, base);
	// 82335880: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82335884: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82335888: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233588C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335890: 4BFD3809  bl 0x82309098
	ctx.lr = 0x82335894;
	sub_82309098(ctx, base);
	// 82335894: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82335898: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233589C: 419A0008  beq cr6, 0x823358a4
	if ctx.cr[6].eq {
	pc = 0x823358A4; continue 'dispatch;
	}
	// 823358A0: 4BF8AFF1  bl 0x822c0890
	ctx.lr = 0x823358A4;
	sub_822C0890(ctx, base);
	// 823358A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823358A8: 48ABDB81  bl 0x82df3428
	ctx.lr = 0x823358AC;
	sub_82DF3428(ctx, base);
	// 823358AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823358B0: 48E72908  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823358B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823358B8 size=284
    let mut pc: u32 = 0x823358B8;
    'dispatch: loop {
        match pc {
            0x823358B8 => {
    //   block [0x823358B8..0x823359D4)
	// 823358B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823358BC: 48E728B1  bl 0x831a816c
	ctx.lr = 0x823358C0;
	sub_831A8130(ctx, base);
	// 823358C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823358C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823358C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823358CC: 486DAFA5  bl 0x82a10870
	ctx.lr = 0x823358D0;
	sub_82A10870(ctx, base);
	// 823358D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823358D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823358D8: 388B1260  addi r4, r11, 0x1260
	ctx.r[4].s64 = ctx.r[11].s64 + 4704;
	// 823358DC: 38A0007D  li r5, 0x7d
	ctx.r[5].s64 = 125;
	// 823358E0: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 823358E4: 48ABCB05  bl 0x82df23e8
	ctx.lr = 0x823358E8;
	sub_82DF23E8(ctx, base);
	// 823358E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823358EC: 4182001C  beq 0x82335908
	if ctx.cr[0].eq {
	pc = 0x82335908; continue 'dispatch;
	}
	// 823358F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823358F4: 48B27035  bl 0x82e5c928
	ctx.lr = 0x823358F8;
	sub_82E5C928(ctx, base);
	// 823358F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823358FC: 396B1210  addi r11, r11, 0x1210
	ctx.r[11].s64 = ctx.r[11].s64 + 4624;
	// 82335900: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335904: 48000008  b 0x8233590c
	pc = 0x8233590C; continue 'dispatch;
	// 82335908: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8233590C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82335910: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335914: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335918: 4BFFFB01  bl 0x82335418
	ctx.lr = 0x8233591C;
	sub_82335418(ctx, base);
	// 8233591C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82335920: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335924: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335928: 4BF8A6D9  bl 0x822c0000
	ctx.lr = 0x8233592C;
	sub_822C0000(ctx, base);
	// 8233592C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82335930: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82335934: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 82335938: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233593C: C01E0040  lfs f0, 0x40(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82335940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82335944: 13FE50C7  vcmpequd (lvx128) v31, v30, v10
	tmp.u32 = ctx.r[30].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823359D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823359D8 size=328
    let mut pc: u32 = 0x823359D8;
    'dispatch: loop {
        match pc {
            0x823359D8 => {
    //   block [0x823359D8..0x82335B20)
	// 823359D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823359DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823359E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823359E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823359E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823359EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823359F0: 48E1EC11  bl 0x83154600
	ctx.lr = 0x823359F4;
	sub_83154600(ctx, base);
	// 823359F4: 486DAE7D  bl 0x82a10870
	ctx.lr = 0x823359F8;
	sub_82A10870(ctx, base);
	// 823359F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823359FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335A00: 48E1EC01  bl 0x83154600
	ctx.lr = 0x82335A04;
	sub_83154600(ctx, base);
	// 82335A04: 89630068  lbz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 82335A08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82335A0C: 418200FC  beq 0x82335b08
	if ctx.cr[0].eq {
	pc = 0x82335B08; continue 'dispatch;
	}
	// 82335A10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82335A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335A18: 4BFD00F9  bl 0x82305b10
	ctx.lr = 0x82335A1C;
	sub_82305B10(ctx, base);
	// 82335A1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335A20: 418200E8  beq 0x82335b08
	if ctx.cr[0].eq {
	pc = 0x82335B08; continue 'dispatch;
	}
	// 82335A24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335A28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82335A2C: 388B1260  addi r4, r11, 0x1260
	ctx.r[4].s64 = ctx.r[11].s64 + 4704;
	// 82335A30: 38A000A2  li r5, 0xa2
	ctx.r[5].s64 = 162;
	// 82335A34: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82335A38: 48ABC9B1  bl 0x82df23e8
	ctx.lr = 0x82335A3C;
	sub_82DF23E8(ctx, base);
	// 82335A3C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82335A40: 4182001C  beq 0x82335a5c
	if ctx.cr[0].eq {
	pc = 0x82335A5C; continue 'dispatch;
	}
	// 82335A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335A48: 48B26EE1  bl 0x82e5c928
	ctx.lr = 0x82335A4C;
	sub_82E5C928(ctx, base);
	// 82335A4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335A50: 396B1238  addi r11, r11, 0x1238
	ctx.r[11].s64 = ctx.r[11].s64 + 4664;
	// 82335A54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335A58: 48000008  b 0x82335a60
	pc = 0x82335A60; continue 'dispatch;
	// 82335A5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82335A60: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82335A64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335A68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335A6C: 4BFFFA75  bl 0x823354e0
	ctx.lr = 0x82335A70;
	sub_823354E0(ctx, base);
	// 82335A70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82335A74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335A78: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82335A7C: 4BF8A585  bl 0x822c0000
	ctx.lr = 0x82335A80;
	sub_822C0000(ctx, base);
	// 82335A80: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82335A84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82335A88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82335A8C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82335A90: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82335A94: 419A0024  beq cr6, 0x82335ab8
	if ctx.cr[6].eq {
	pc = 0x82335AB8; continue 'dispatch;
	}
	// 82335A98: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82335A9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82335AA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82335AA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82335AA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82335AAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82335AB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82335AB4: 4082FFE8  bne 0x82335a9c
	if !ctx.cr[0].eq {
	pc = 0x82335A9C; continue 'dispatch;
	}
	// 82335AB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82335ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82335AC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82335AC4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82335AC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82335ACC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335AD0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335AD4: 48B28DED  bl 0x82e5e8c0
	ctx.lr = 0x82335AD8;
	sub_82E5E8C0(ctx, base);
	// 82335AD8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82335ADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82335AE0: 419A0008  beq cr6, 0x82335ae8
	if ctx.cr[6].eq {
	pc = 0x82335AE8; continue 'dispatch;
	}
	// 82335AE4: 4BF8ADAD  bl 0x822c0890
	ctx.lr = 0x82335AE8;
	sub_822C0890(ctx, base);
	// 82335AE8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82335AEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82335AF0: 419A0008  beq cr6, 0x82335af8
	if ctx.cr[6].eq {
	pc = 0x82335AF8; continue 'dispatch;
	}
	// 82335AF4: 4BF8AD9D  bl 0x822c0890
	ctx.lr = 0x82335AF8;
	sub_822C0890(ctx, base);
	// 82335AF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82335AFC: 419A000C  beq cr6, 0x82335b08
	if ctx.cr[6].eq {
	pc = 0x82335B08; continue 'dispatch;
	}
	// 82335B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335B04: 4BF8AD8D  bl 0x822c0890
	ctx.lr = 0x82335B08;
	sub_822C0890(ctx, base);
	// 82335B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82335B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82335B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335B20 size=108
    let mut pc: u32 = 0x82335B20;
    'dispatch: loop {
        match pc {
            0x82335B20 => {
    //   block [0x82335B20..0x82335B8C)
	// 82335B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335B24: 48E72649  bl 0x831a816c
	ctx.lr = 0x82335B28;
	sub_831A8130(ctx, base);
	// 82335B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335B2C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82335B30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82335B34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82335B38: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335B3C: 41820038  beq 0x82335b74
	if ctx.cr[0].eq {
	pc = 0x82335B74; continue 'dispatch;
	}
	// 82335B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335B44: 48E73E45  bl 0x831a9988
	ctx.lr = 0x82335B48;
	sub_831A9988(ctx, base);
	// 82335B48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335B4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82335B50: 386B8B50  addi r3, r11, -0x74b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29872;
	// 82335B54: 48E725A5  bl 0x831a80f8
	ctx.lr = 0x82335B58;
	sub_831A80F8(ctx, base);
	// 82335B58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335B5C: 41820018  beq 0x82335b74
	if ctx.cr[0].eq {
	pc = 0x82335B74; continue 'dispatch;
	}
	// 82335B60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335B64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82335B68: 4BFFFD51  bl 0x823358b8
	ctx.lr = 0x82335B6C;
	sub_823358B8(ctx, base);
	// 82335B6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82335B70: 48000014  b 0x82335b84
	pc = 0x82335B84; continue 'dispatch;
	// 82335B74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82335B78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82335B80: 4BFFC691  bl 0x82332210
	ctx.lr = 0x82335B84;
	sub_82332210(ctx, base);
	// 82335B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335B88: 48E72634  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335B90 size=212
    let mut pc: u32 = 0x82335B90;
    'dispatch: loop {
        match pc {
            0x82335B90 => {
    //   block [0x82335B90..0x82335C64)
	// 82335B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335B94: 48E725D9  bl 0x831a816c
	ctx.lr = 0x82335B98;
	sub_831A8130(ctx, base);
	// 82335B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335B9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335BA0: 486DACD1  bl 0x82a10870
	ctx.lr = 0x82335BA4;
	sub_82A10870(ctx, base);
	// 82335BA4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335BA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335BAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335BB0: 808BB4FC  lwz r4, -0x4b04(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19204 as u32) ) } as u64;
	// 82335BB4: 48ABDE55  bl 0x82df3a08
	ctx.lr = 0x82335BB8;
	sub_82DF3A08(ctx, base);
	// 82335BB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82335BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335BC0: 4BFCF989  bl 0x82305548
	ctx.lr = 0x82335BC4;
	sub_82305548(ctx, base);
	// 82335BC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335BC8: 48ABD861  bl 0x82df3428
	ctx.lr = 0x82335BCC;
	sub_82DF3428(ctx, base);
	// 82335BCC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82335BD0: 4BFDAFB1  bl 0x82310b80
	ctx.lr = 0x82335BD4;
	sub_82310B80(ctx, base);
	// 82335BD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82335BD8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82335BDC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82335BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82335BE4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82335BE8: 419A0024  beq cr6, 0x82335c0c
	if ctx.cr[6].eq {
	pc = 0x82335C0C; continue 'dispatch;
	}
	// 82335BEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82335BF0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82335BF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82335BF8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82335BFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82335C00: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82335C04: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82335C08: 4082FFE8  bne 0x82335bf0
	if !ctx.cr[0].eq {
	pc = 0x82335BF0; continue 'dispatch;
	}
	// 82335C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335C10: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82335C14: 4BFDD545  bl 0x82313158
	ctx.lr = 0x82335C18;
	sub_82313158(ctx, base);
	// 82335C18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82335C1C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82335C20: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82335C24: 388A1260  addi r4, r10, 0x1260
	ctx.r[4].s64 = ctx.r[10].s64 + 4704;
	// 82335C28: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82335C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335C30: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335C34: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82335C38: 4BFD2251  bl 0x82307e88
	ctx.lr = 0x82335C3C;
	sub_82307E88(ctx, base);
	// 82335C3C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82335C40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82335C44: 419A0008  beq cr6, 0x82335c4c
	if ctx.cr[6].eq {
	pc = 0x82335C4C; continue 'dispatch;
	}
	// 82335C48: 4BF8AC49  bl 0x822c0890
	ctx.lr = 0x82335C4C;
	sub_822C0890(ctx, base);
	// 82335C4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335C50: 4BFE5559  bl 0x8231b1a8
	ctx.lr = 0x82335C54;
	sub_8231B1A8(ctx, base);
	// 82335C54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335C58: 48D91E89  bl 0x830c7ae0
	ctx.lr = 0x82335C5C;
	sub_830C7AE0(ctx, base);
	// 82335C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82335C60: 48E7255C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335C68 size=80
    let mut pc: u32 = 0x82335C68;
    'dispatch: loop {
        match pc {
            0x82335C68 => {
    //   block [0x82335C68..0x82335CB8)
	// 82335C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335C7C: 4BFFC15D  bl 0x82331dd8
	ctx.lr = 0x82335C80;
	sub_82331DD8(ctx, base);
	// 82335C80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335C84: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82335C88: 396B12CC  addi r11, r11, 0x12cc
	ctx.r[11].s64 = ctx.r[11].s64 + 4812;
	// 82335C8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82335C90: 48B27C41  bl 0x82e5d8d0
	ctx.lr = 0x82335C94;
	sub_82E5D8D0(ctx, base);
	// 82335C94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335C9C: 396B119C  addi r11, r11, 0x119c
	ctx.r[11].s64 = ctx.r[11].s64 + 4508;
	// 82335CA0: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82335CA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82335CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335CB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82335CB8 size=88
    let mut pc: u32 = 0x82335CB8;
    'dispatch: loop {
        match pc {
            0x82335CB8 => {
    //   block [0x82335CB8..0x82335D10)
	// 82335CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335CC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82335CC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335CC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335CD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82335CD4: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82335CD8: 48B27B71  bl 0x82e5d848
	ctx.lr = 0x82335CDC;
	sub_82E5D848(ctx, base);
	// 82335CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335CE0: 4BFFC189  bl 0x82331e68
	ctx.lr = 0x82335CE4;
	sub_82331E68(ctx, base);
	// 82335CE4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335CE8: 4182000C  beq 0x82335cf4
	if ctx.cr[0].eq {
	pc = 0x82335CF4; continue 'dispatch;
	}
	// 82335CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335CF0: 48ABC6E9  bl 0x82df23d8
	ctx.lr = 0x82335CF4;
	sub_82DF23D8(ctx, base);
	// 82335CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335CF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335D04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82335D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335D10 size=176
    let mut pc: u32 = 0x82335D10;
    'dispatch: loop {
        match pc {
            0x82335D10 => {
    //   block [0x82335D10..0x82335DC0)
	// 82335D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335D14: 48E72459  bl 0x831a816c
	ctx.lr = 0x82335D18;
	sub_831A8130(ctx, base);
	// 82335D18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335D1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335D20: 486DAB51  bl 0x82a10870
	ctx.lr = 0x82335D24;
	sub_82A10870(ctx, base);
	// 82335D24: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335D28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335D2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335D30: 808BB050  lwz r4, -0x4fb0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20400 as u32) ) } as u64;
	// 82335D34: 48ABDCD5  bl 0x82df3a08
	ctx.lr = 0x82335D38;
	sub_82DF3A08(ctx, base);
	// 82335D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335D3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82335D40: 4BFD1B99  bl 0x823078d8
	ctx.lr = 0x82335D44;
	sub_823078D8(ctx, base);
	// 82335D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82335D48: 48ABD6E1  bl 0x82df3428
	ctx.lr = 0x82335D4C;
	sub_82DF3428(ctx, base);
	// 82335D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335D50: 4BFDCF59  bl 0x82312ca8
	ctx.lr = 0x82335D54;
	sub_82312CA8(ctx, base);
	// 82335D54: D03E0068  stfs f1, 0x68(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82335D58: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82335D5C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82335D60: 48125089  bl 0x8245ade8
	ctx.lr = 0x82335D64;
	sub_8245ADE8(ctx, base);
	// 82335D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335D68: 4BFDCF49  bl 0x82312cb0
	ctx.lr = 0x82335D6C;
	sub_82312CB0(ctx, base);
	// 82335D6C: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 82335D70: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82335D74: 4812502D  bl 0x8245ada0
	ctx.lr = 0x82335D78;
	sub_8245ADA0(ctx, base);
	// 82335D78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335D7C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82335D80: 41820024  beq 0x82335da4
	if ctx.cr[0].eq {
	pc = 0x82335DA4; continue 'dispatch;
	}
	// 82335D84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335D88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335D8C: 4BFDE1D5  bl 0x82313f60
	ctx.lr = 0x82335D90;
	sub_82313F60(ctx, base);
	// 82335D90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82335D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335D98: 4BFE2D11  bl 0x82318aa8
	ctx.lr = 0x82335D9C;
	sub_82318AA8(ctx, base);
	// 82335D9C: 9BBE006D  stb r29, 0x6d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(109 as u32), ctx.r[29].u8 ) };
	// 82335DA0: 48000014  b 0x82335db4
	pc = 0x82335DB4; continue 'dispatch;
	// 82335DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335DA8: 4BFDDE59  bl 0x82313c00
	ctx.lr = 0x82335DAC;
	sub_82313C00(ctx, base);
	// 82335DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82335DB0: 997E006D  stb r11, 0x6d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(109 as u32), ctx.r[11].u8 ) };
	// 82335DB4: 9BBE006C  stb r29, 0x6c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[29].u8 ) };
	// 82335DB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82335DBC: 48E72400  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335DC0 size=144
    let mut pc: u32 = 0x82335DC0;
    'dispatch: loop {
        match pc {
            0x82335DC0 => {
    //   block [0x82335DC0..0x82335E50)
	// 82335DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82335DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82335DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82335DD0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82335DD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335DD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335DDC: 486DAA95  bl 0x82a10870
	ctx.lr = 0x82335DE0;
	sub_82A10870(ctx, base);
	// 82335DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335DE4: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82335DE8: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82335DEC: 48124FFD  bl 0x8245ade8
	ctx.lr = 0x82335DF0;
	sub_8245ADE8(ctx, base);
	// 82335DF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335DF4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82335DF8: 4BFDCEB1  bl 0x82312ca8
	ctx.lr = 0x82335DFC;
	sub_82312CA8(ctx, base);
	// 82335DFC: EDA1F828  fsubs f13, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[31].f64) as f32) as f64);
	// 82335E00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82335E04: C00B9528  lfs f0, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82335E08: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82335E0C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82335E10: 40980010  bge cr6, 0x82335e20
	if !ctx.cr[6].lt {
	pc = 0x82335E20; continue 'dispatch;
	}
	// 82335E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335E18: C03E0068  lfs f1, 0x68(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335E1C: 4BFDCE95  bl 0x82312cb0
	ctx.lr = 0x82335E20;
	sub_82312CB0(ctx, base);
	// 82335E20: 897E006D  lbz r11, 0x6d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(109 as u32) ) } as u64;
	// 82335E24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82335E28: 4182000C  beq 0x82335e34
	if ctx.cr[0].eq {
	pc = 0x82335E34; continue 'dispatch;
	}
	// 82335E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335E30: 4BFDF2B1  bl 0x823150e0
	ctx.lr = 0x82335E34;
	sub_823150E0(ctx, base);
	// 82335E34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82335E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82335E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82335E40: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82335E44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82335E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82335E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82335E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82335E50 size=428
    let mut pc: u32 = 0x82335E50;
    'dispatch: loop {
        match pc {
            0x82335E50 => {
    //   block [0x82335E50..0x82335FFC)
	// 82335E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82335E54: 48E72319  bl 0x831a816c
	ctx.lr = 0x82335E58;
	sub_831A8130(ctx, base);
	// 82335E58: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82335E5C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82335E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82335E64: 486DAA0D  bl 0x82a10870
	ctx.lr = 0x82335E68;
	sub_82A10870(ctx, base);
	// 82335E68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82335E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335E70: 48B23909  bl 0x82e59778
	ctx.lr = 0x82335E74;
	sub_82E59778(ctx, base);
	// 82335E74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335E78: 4BFDCE89  bl 0x82312d00
	ctx.lr = 0x82335E7C;
	sub_82312D00(ctx, base);
	// 82335E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335E80: 4BFDCD89  bl 0x82312c08
	ctx.lr = 0x82335E84;
	sub_82312C08(ctx, base);
	// 82335E84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335E88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335E8C: 41820058  beq 0x82335ee4
	if ctx.cr[0].eq {
	pc = 0x82335EE4; continue 'dispatch;
	}
	// 82335E90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335E94: 4BFF886D  bl 0x8232e700
	ctx.lr = 0x82335E98;
	sub_8232E700(ctx, base);
	// 82335E98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335E9C: 40820154  bne 0x82335ff0
	if !ctx.cr[0].eq {
	pc = 0x82335FF0; continue 'dispatch;
	}
	// 82335EA0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82335EA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82335EA8: 808BB3B4  lwz r4, -0x4c4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19532 as u32) ) } as u64;
	// 82335EAC: 48ABDB5D  bl 0x82df3a08
	ctx.lr = 0x82335EB0;
	sub_82DF3A08(ctx, base);
	// 82335EB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82335EB4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82335EB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82335EBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82335EC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82335EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82335EC8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82335ECC: 48B28BC5  bl 0x82e5ea90
	ctx.lr = 0x82335ED0;
	sub_82E5EA90(ctx, base);
	// 82335ED0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82335ED4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82335ED8: 419A0118  beq cr6, 0x82335ff0
	if ctx.cr[6].eq {
	pc = 0x82335FF0; continue 'dispatch;
	}
	// 82335EDC: 4BF8A9B5  bl 0x822c0890
	ctx.lr = 0x82335EE0;
	sub_822C0890(ctx, base);
	// 82335EE0: 48000110  b 0x82335ff0
	pc = 0x82335FF0; continue 'dispatch;
	// 82335EE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82335EE8: 4BFCFC59  bl 0x82305b40
	ctx.lr = 0x82335EEC;
	sub_82305B40(ctx, base);
	// 82335EEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82335EF0: 41820028  beq 0x82335f18
	if ctx.cr[0].eq {
	pc = 0x82335F18; continue 'dispatch;
	}
	// 82335EF4: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82335EF8: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 82335EFC: 48124EED  bl 0x8245ade8
	ctx.lr = 0x82335F00;
	sub_8245ADE8(ctx, base);
	// 82335F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82335F04: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82335F08: 48B23859  bl 0x82e59760
	ctx.lr = 0x82335F0C;
	sub_82E59760(ctx, base);
	// 82335F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82335F10: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82335F14: 4BFE424D  bl 0x8231a160
	ctx.lr = 0x82335F18;
	sub_8231A160(ctx, base);
	// 82335F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82335F1C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82335F20: 4BFDDC49  bl 0x82313b68
	ctx.lr = 0x82335F24;
	sub_82313B68(ctx, base);
	// 82335F24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82335F28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82335F2C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82335F30: 4BFE3BB9  bl 0x82319ae8
	ctx.lr = 0x82335F34;
	sub_82319AE8(ctx, base);
	// 82335F34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82335F38: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82335F3C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82335F40: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82335F44: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 82335F48: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336000 size=60
    let mut pc: u32 = 0x82336000;
    'dispatch: loop {
        match pc {
            0x82336000 => {
    //   block [0x82336000..0x8233603C)
	// 82336000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233600C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336014: 486DA85D  bl 0x82a10870
	ctx.lr = 0x82336018;
	sub_82A10870(ctx, base);
	// 82336018: 897F0069  lbz r11, 0x69(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(105 as u32) ) } as u64;
	// 8233601C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336020: 41820008  beq 0x82336028
	if ctx.cr[0].eq {
	pc = 0x82336028; continue 'dispatch;
	}
	// 82336024: 4BFDF0BD  bl 0x823150e0
	ctx.lr = 0x82336028;
	sub_823150E0(ctx, base);
	// 82336028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233602C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336040 size=124
    let mut pc: u32 = 0x82336040;
    'dispatch: loop {
        match pc {
            0x82336040 => {
    //   block [0x82336040..0x823360BC)
	// 82336040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233604C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336054: 486DA81D  bl 0x82a10870
	ctx.lr = 0x82336058;
	sub_82A10870(ctx, base);
	// 82336058: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233605C: 4BFF86A5  bl 0x8232e700
	ctx.lr = 0x82336060;
	sub_8232E700(ctx, base);
	// 82336060: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336064: 40820044  bne 0x823360a8
	if !ctx.cr[0].eq {
	pc = 0x823360A8; continue 'dispatch;
	}
	// 82336068: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233606C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336070: 808BB3B4  lwz r4, -0x4c4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19532 as u32) ) } as u64;
	// 82336074: 48ABD995  bl 0x82df3a08
	ctx.lr = 0x82336078;
	sub_82DF3A08(ctx, base);
	// 82336078: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233607C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82336080: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336084: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336088: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233608C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82336090: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336094: 48B289FD  bl 0x82e5ea90
	ctx.lr = 0x82336098;
	sub_82E5EA90(ctx, base);
	// 82336098: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233609C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823360A0: 419A0008  beq cr6, 0x823360a8
	if ctx.cr[6].eq {
	pc = 0x823360A8; continue 'dispatch;
	}
	// 823360A4: 4BF8A7ED  bl 0x822c0890
	ctx.lr = 0x823360A8;
	sub_822C0890(ctx, base);
	// 823360A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823360AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823360B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823360B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823360B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823360C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823360C0 size=712
    let mut pc: u32 = 0x823360C0;
    'dispatch: loop {
        match pc {
            0x823360C0 => {
    //   block [0x823360C0..0x82336388)
	// 823360C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823360C4: 48E720A9  bl 0x831a816c
	ctx.lr = 0x823360C8;
	sub_831A8130(ctx, base);
	// 823360C8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823360CC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823360D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823360D4: 486DA79D  bl 0x82a10870
	ctx.lr = 0x823360D8;
	sub_82A10870(ctx, base);
	// 823360D8: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 823360DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823360E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823360E4: 409A0098  bne cr6, 0x8233617c
	if !ctx.cr[6].eq {
	pc = 0x8233617C; continue 'dispatch;
	}
	// 823360E8: 897F0068  lbz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 823360EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823360F0: 4182001C  beq 0x8233610c
	if ctx.cr[0].eq {
	pc = 0x8233610C; continue 'dispatch;
	}
	// 823360F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823360F8: 4BFCFA31  bl 0x82305b28
	ctx.lr = 0x823360FC;
	sub_82305B28(ctx, base);
	// 823360FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336100: 4082000C  bne 0x8233610c
	if !ctx.cr[0].eq {
	pc = 0x8233610C; continue 'dispatch;
	}
	// 82336104: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82336108: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 8233610C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336110: 4BFCF3A1  bl 0x823054b0
	ctx.lr = 0x82336114;
	sub_823054B0(ctx, base);
	// 82336114: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336118: C00BA1C4  lfs f0, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233611C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82336120: 4198025C  blt cr6, 0x8233637c
	if ctx.cr[6].lt {
	pc = 0x8233637C; continue 'dispatch;
	}
	// 82336124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336128: 4BFE9809  bl 0x8231f930
	ctx.lr = 0x8233612C;
	sub_8231F930(ctx, base);
	// 8233612C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336130: 48B23631  bl 0x82e59760
	ctx.lr = 0x82336134;
	sub_82E59760(ctx, base);
	// 82336134: 897F0068  lbz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82336138: D03F0070  stfs f1, 0x70(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8233613C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336140: 40820030  bne 0x82336170
	if !ctx.cr[0].eq {
	pc = 0x82336170; continue 'dispatch;
	}
	// 82336144: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82336148: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233614C: 48124C9D  bl 0x8245ade8
	ctx.lr = 0x82336150;
	sub_8245ADE8(ctx, base);
	// 82336150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336154: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82336158: 48B23609  bl 0x82e59760
	ctx.lr = 0x8233615C;
	sub_82E59760(ctx, base);
	// 8233615C: C01F0070  lfs f0, 0x70(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82336160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336164: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82336168: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8233616C: 4BFE3FF5  bl 0x8231a160
	ctx.lr = 0x82336170;
	sub_8231A160(ctx, base);
	// 82336170: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82336174: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82336178: 48000204  b 0x8233637c
	pc = 0x8233637C; continue 'dispatch;
	// 8233617C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82336180: 409A0148  bne cr6, 0x823362c8
	if !ctx.cr[6].eq {
	pc = 0x823362C8; continue 'dispatch;
	}
	// 82336184: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336188: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233618C: 4BFE395D  bl 0x82319ae8
	ctx.lr = 0x82336190;
	sub_82319AE8(ctx, base);
	// 82336190: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82336194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336198: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8233619C: 4BFDD9CD  bl 0x82313b68
	ctx.lr = 0x823361A0;
	sub_82313B68(ctx, base);
	// 823361A0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823361A4: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823361A8: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336388 size=240
    let mut pc: u32 = 0x82336388;
    'dispatch: loop {
        match pc {
            0x82336388 => {
    //   block [0x82336388..0x82336478)
	// 82336388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233638C: 48E71DDD  bl 0x831a8168
	ctx.lr = 0x82336390;
	sub_831A8130(ctx, base);
	// 82336390: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82336398: 486DA4D9  bl 0x82a10870
	ctx.lr = 0x8233639C;
	sub_82A10870(ctx, base);
	// 8233639C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823363A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823363A4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823363A8: 388B12FC  addi r4, r11, 0x12fc
	ctx.r[4].s64 = ctx.r[11].s64 + 4860;
	// 823363AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823363B0: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 823363B4: 48ABD655  bl 0x82df3a08
	ctx.lr = 0x823363B8;
	sub_82DF3A08(ctx, base);
	// 823363B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823363BC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823363C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823363C4: 4BFFDE15  bl 0x823341d8
	ctx.lr = 0x823363C8;
	sub_823341D8(ctx, base);
	// 823363C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823363CC: 48ABD05D  bl 0x82df3428
	ctx.lr = 0x823363D0;
	sub_82DF3428(ctx, base);
	// 823363D0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823363D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823363D8: 41820028  beq 0x82336400
	if ctx.cr[0].eq {
	pc = 0x82336400; continue 'dispatch;
	}
	// 823363DC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823363E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823363E4: 808BB054  lwz r4, -0x4fac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20396 as u32) ) } as u64;
	// 823363E8: 48ABD621  bl 0x82df3a08
	ctx.lr = 0x823363EC;
	sub_82DF3A08(ctx, base);
	// 823363EC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823363F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823363F4: 4BFD14E5  bl 0x823078d8
	ctx.lr = 0x823363F8;
	sub_823078D8(ctx, base);
	// 823363F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823363FC: 48ABD02D  bl 0x82df3428
	ctx.lr = 0x82336400;
	sub_82DF3428(ctx, base);
	// 82336400: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82336404: 9BBE0068  stb r29, 0x68(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[29].u8 ) };
	// 82336408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233640C: 939E006C  stw r28, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82336410: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82336414: 4BFD18AD  bl 0x82307cc0
	ctx.lr = 0x82336418;
	sub_82307CC0(ctx, base);
	// 82336418: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233641C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336420: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82336424: 4BFDDA8D  bl 0x82313eb0
	ctx.lr = 0x82336428;
	sub_82313EB0(ctx, base);
	// 82336428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233642C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336430: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82336434: 4BFE0E75  bl 0x823172a8
	ctx.lr = 0x82336438;
	sub_823172A8(ctx, base);
	// 82336438: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 8233643C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82336440: 48124961  bl 0x8245ada0
	ctx.lr = 0x82336444;
	sub_8245ADA0(ctx, base);
	// 82336444: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336448: 41820024  beq 0x8233646c
	if ctx.cr[0].eq {
	pc = 0x8233646C; continue 'dispatch;
	}
	// 8233644C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336450: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82336454: 4BFDDB0D  bl 0x82313f60
	ctx.lr = 0x82336458;
	sub_82313F60(ctx, base);
	// 82336458: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233645C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336460: 4BFE2649  bl 0x82318aa8
	ctx.lr = 0x82336464;
	sub_82318AA8(ctx, base);
	// 82336464: 9BBE0069  stb r29, 0x69(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(105 as u32), ctx.r[29].u8 ) };
	// 82336468: 48000008  b 0x82336470
	pc = 0x82336470; continue 'dispatch;
	// 8233646C: 9B9E0069  stb r28, 0x69(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(105 as u32), ctx.r[28].u8 ) };
	// 82336470: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82336474: 48E71D44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336478 size=68
    let mut pc: u32 = 0x82336478;
    'dispatch: loop {
        match pc {
            0x82336478 => {
    //   block [0x82336478..0x823364BC)
	// 82336478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233648C: 4BFFB94D  bl 0x82331dd8
	ctx.lr = 0x82336490;
	sub_82331DD8(ctx, base);
	// 82336490: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82336498: 396B1314  addi r11, r11, 0x1314
	ctx.r[11].s64 = ctx.r[11].s64 + 4884;
	// 8233649C: 995F0069  stb r10, 0x69(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(105 as u32), ctx.r[10].u8 ) };
	// 823364A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823364A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823364A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823364AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823364B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823364B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823364B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823364C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823364C0 size=204
    let mut pc: u32 = 0x823364C0;
    'dispatch: loop {
        match pc {
            0x823364C0 => {
    //   block [0x823364C0..0x8233658C)
	// 823364C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823364C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823364C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823364CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823364D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823364D4: 486DA39D  bl 0x82a10870
	ctx.lr = 0x823364D8;
	sub_82A10870(ctx, base);
	// 823364D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823364DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823364E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823364E4: 808BB07C  lwz r4, -0x4f84(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20356 as u32) ) } as u64;
	// 823364E8: 48ABD521  bl 0x82df3a08
	ctx.lr = 0x823364EC;
	sub_82DF3A08(ctx, base);
	// 823364EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823364F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823364F4: 4BFD13E5  bl 0x823078d8
	ctx.lr = 0x823364F8;
	sub_823078D8(ctx, base);
	// 823364F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823364FC: 48ABCF2D  bl 0x82df3428
	ctx.lr = 0x82336500;
	sub_82DF3428(ctx, base);
	// 82336500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336504: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82336508: 4BFDD7E1  bl 0x82313ce8
	ctx.lr = 0x8233650C;
	sub_82313CE8(ctx, base);
	// 8233650C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336510: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82336514: 4BFE35D5  bl 0x82319ae8
	ctx.lr = 0x82336518;
	sub_82319AE8(ctx, base);
	// 82336518: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233651C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82336520: 48B46F31  bl 0x82e7d450
	ctx.lr = 0x82336524;
	sub_82E7D450(ctx, base);
	// 82336524: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82336528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233652C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82336530: 4BFE4AC1  bl 0x8231aff0
	ctx.lr = 0x82336534;
	sub_8231AFF0(ctx, base);
	// 82336534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336538: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8233653C: 4BFDD7AD  bl 0x82313ce8
	ctx.lr = 0x82336540;
	sub_82313CE8(ctx, base);
	// 82336540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336544: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82336548: 4BFDD7A1  bl 0x82313ce8
	ctx.lr = 0x8233654C;
	sub_82313CE8(ctx, base);
	// 8233654C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336550: 4BFDD6B1  bl 0x82313c00
	ctx.lr = 0x82336554;
	sub_82313C00(ctx, base);
	// 82336554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336558: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233655C: 4BFCF55D  bl 0x82305ab8
	ctx.lr = 0x82336560;
	sub_82305AB8(ctx, base);
	// 82336560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82336564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336568: 4BFCF5F1  bl 0x82305b58
	ctx.lr = 0x8233656C;
	sub_82305B58(ctx, base);
	// 8233656C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336570: 487AE5D1  bl 0x82ae4b40
	ctx.lr = 0x82336574;
	sub_82AE4B40(ctx, base);
	// 82336574: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82336578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233657C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82336584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336590 size=100
    let mut pc: u32 = 0x82336590;
    'dispatch: loop {
        match pc {
            0x82336590 => {
    //   block [0x82336590..0x823365F4)
	// 82336590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336594: 48E71BD5  bl 0x831a8168
	ctx.lr = 0x82336598;
	sub_831A8130(ctx, base);
	// 82336598: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8233659C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823365A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823365A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823365A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823365AC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 823365B0: 486DA2C1  bl 0x82a10870
	ctx.lr = 0x823365B4;
	sub_82A10870(ctx, base);
	// 823365B4: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823365B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823365BC: 4182000C  beq 0x823365c8
	if ctx.cr[0].eq {
	pc = 0x823365C8; continue 'dispatch;
	}
	// 823365C0: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 823365C4: 4BFDD735  bl 0x82313cf8
	ctx.lr = 0x823365C8;
	sub_82313CF8(ctx, base);
	// 823365C8: D3FF0068  stfs f31, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 823365CC: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823365D0: 9BDF0070  stb r30, 0x70(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u8 ) };
	// 823365D4: 41820014  beq 0x823365e8
	if ctx.cr[0].eq {
	pc = 0x823365E8; continue 'dispatch;
	}
	// 823365D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823365DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823365E0: C02B093C  lfs f1, 0x93c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2364 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823365E4: 4BFDF40D  bl 0x823159f0
	ctx.lr = 0x823365E8;
	sub_823159F0(ctx, base);
	// 823365E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823365EC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 823365F0: 48E71BC8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823365F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823365F8 size=12
    let mut pc: u32 = 0x823365F8;
    'dispatch: loop {
        match pc {
            0x823365F8 => {
    //   block [0x823365F8..0x82336604)
	// 823365F8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823365FC: 386B86A8  addi r3, r11, -0x7958
	ctx.r[3].s64 = ctx.r[11].s64 + -31064;
	// 82336600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336608 size=84
    let mut pc: u32 = 0x82336608;
    'dispatch: loop {
        match pc {
            0x82336608 => {
    //   block [0x82336608..0x8233665C)
	// 82336608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233660C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233661C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82336620: 4BF8A319  bl 0x822c0938
	ctx.lr = 0x82336624;
	sub_822C0938(ctx, base);
	// 82336624: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336628: 4182001C  beq 0x82336644
	if ctx.cr[0].eq {
	pc = 0x82336644; continue 'dispatch;
	}
	// 8233662C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336630: 396B1340  addi r11, r11, 0x1340
	ctx.r[11].s64 = ctx.r[11].s64 + 4928;
	// 82336634: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82336638: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233663C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82336640: 48000008  b 0x82336648
	pc = 0x82336648; continue 'dispatch;
	// 82336644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82336648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336660 size=108
    let mut pc: u32 = 0x82336660;
    'dispatch: loop {
        match pc {
            0x82336660 => {
    //   block [0x82336660..0x823366CC)
	// 82336660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233666C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336674: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82336678: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8233667C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82336680: 4BF8A2B9  bl 0x822c0938
	ctx.lr = 0x82336684;
	sub_822C0938(ctx, base);
	// 82336684: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336688: 41820020  beq 0x823366a8
	if ctx.cr[0].eq {
	pc = 0x823366A8; continue 'dispatch;
	}
	// 8233668C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82336690: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82336694: 394A1340  addi r10, r10, 0x1340
	ctx.r[10].s64 = ctx.r[10].s64 + 4928;
	// 82336698: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8233669C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823366A0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823366A4: 48000008  b 0x823366ac
	pc = 0x823366AC; continue 'dispatch;
	// 823366A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823366AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823366B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823366B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823366B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823366BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823366C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823366C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823366C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823366D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823366D0 size=332
    let mut pc: u32 = 0x823366D0;
    'dispatch: loop {
        match pc {
            0x823366D0 => {
    //   block [0x823366D0..0x8233681C)
	// 823366D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823366D4: 48E71A99  bl 0x831a816c
	ctx.lr = 0x823366D8;
	sub_831A8130(ctx, base);
	// 823366D8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823366DC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823366E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823366E4: 486DA18D  bl 0x82a10870
	ctx.lr = 0x823366E8;
	sub_82A10870(ctx, base);
	// 823366E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823366EC: 4BFDC51D  bl 0x82312c08
	ctx.lr = 0x823366F0;
	sub_82312C08(ctx, base);
	// 823366F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823366F4: 4182001C  beq 0x82336710
	if ctx.cr[0].eq {
	pc = 0x82336710; continue 'dispatch;
	}
	// 823366F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823366FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336700: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82336704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336708: 4E800421  bctrl
	ctx.lr = 0x8233670C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233670C: 48000104  b 0x82336810
	pc = 0x82336810; continue 'dispatch;
	// 82336710: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233671C: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82336720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336724: 4E800421  bctrl
	ctx.lr = 0x82336728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82336728: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233672C: 408200E4  bne 0x82336810
	if !ctx.cr[0].eq {
	pc = 0x82336810; continue 'dispatch;
	}
	// 82336730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336734: 48B2302D  bl 0x82e59760
	ctx.lr = 0x82336738;
	sub_82E59760(ctx, base);
	// 82336738: C01E0068  lfs f0, 0x68(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233673C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82336740: 40990028  ble cr6, 0x82336768
	if !ctx.cr[6].gt {
	pc = 0x82336768; continue 'dispatch;
	}
	// 82336744: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82336748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233674C: 4BFDD5A5  bl 0x82313cf0
	ctx.lr = 0x82336750;
	sub_82313CF0(ctx, base);
	// 82336750: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82336754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336758: 4BFDD599  bl 0x82313cf0
	ctx.lr = 0x8233675C;
	sub_82313CF0(ctx, base);
	// 8233675C: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82336760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336764: 4BFDD58D  bl 0x82313cf0
	ctx.lr = 0x82336768;
	sub_82313CF0(ctx, base);
	// 82336768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233676C: 48B22FF5  bl 0x82e59760
	ctx.lr = 0x82336770;
	sub_82E59760(ctx, base);
	// 82336770: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336774: C00B9450  lfs f0, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82336778: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233677C: 40990094  ble cr6, 0x82336810
	if !ctx.cr[6].gt {
	pc = 0x82336810; continue 'dispatch;
	}
	// 82336780: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336784: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82336788: 4BFDD3E1  bl 0x82313b68
	ctx.lr = 0x8233678C;
	sub_82313B68(ctx, base);
	// 8233678C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82336790: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336794: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82336798: 4BFE3351  bl 0x82319ae8
	ctx.lr = 0x8233679C;
	sub_82319AE8(ctx, base);
	// 8233679C: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823367A0: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823367A4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336820 size=100
    let mut pc: u32 = 0x82336820;
    'dispatch: loop {
        match pc {
            0x82336820 => {
    //   block [0x82336820..0x82336884)
	// 82336820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233682C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336838: 4BFFFE29  bl 0x82336660
	ctx.lr = 0x8233683C;
	sub_82336660(ctx, base);
	// 8233683C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336840: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336844: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82336848: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8233684C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82336850: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82336854: 419A0018  beq cr6, 0x8233686c
	if ctx.cr[6].eq {
	pc = 0x8233686C; continue 'dispatch;
	}
	// 82336858: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233685C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82336860: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336868: 4E800421  bctrl
	ctx.lr = 0x8233686C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233686C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82336874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233687C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336888 size=80
    let mut pc: u32 = 0x82336888;
    'dispatch: loop {
        match pc {
            0x82336888 => {
    //   block [0x82336888..0x823368D8)
	// 82336888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233688C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233689C: 4BFFB53D  bl 0x82331dd8
	ctx.lr = 0x823368A0;
	sub_82331DD8(ctx, base);
	// 823368A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823368A4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823368A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823368AC: 394A1350  addi r10, r10, 0x1350
	ctx.r[10].s64 = ctx.r[10].s64 + 4944;
	// 823368B0: 913F006C  stw r9, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 823368B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823368B8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823368BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823368C0: D01F0068  stfs f0, 0x68(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 823368C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823368C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823368CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823368D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823368D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823368D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823368D8 size=328
    let mut pc: u32 = 0x823368D8;
    'dispatch: loop {
        match pc {
            0x823368D8 => {
    //   block [0x823368D8..0x82336A20)
	// 823368D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823368DC: 48E7188D  bl 0x831a8168
	ctx.lr = 0x823368E0;
	sub_831A8130(ctx, base);
	// 823368E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823368E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823368E8: 486D9F89  bl 0x82a10870
	ctx.lr = 0x823368EC;
	sub_82A10870(ctx, base);
	// 823368EC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823368F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823368F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823368F8: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 823368FC: 48ABD10D  bl 0x82df3a08
	ctx.lr = 0x82336900;
	sub_82DF3A08(ctx, base);
	// 82336900: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336904: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82336908: 48B230C9  bl 0x82e599d0
	ctx.lr = 0x8233690C;
	sub_82E599D0(ctx, base);
	// 8233690C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336910: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82336914: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336918: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8233691C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336920: 4E800421  bctrl
	ctx.lr = 0x82336924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82336924: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82336928: 48ABC9E1  bl 0x82df3308
	ctx.lr = 0x8233692C;
	sub_82DF3308(ctx, base);
	// 8233692C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82336930: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82336934: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82336938: 419A000C  beq cr6, 0x82336944
	if ctx.cr[6].eq {
	pc = 0x82336944; continue 'dispatch;
	}
	// 8233693C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82336940: 4BF89F51  bl 0x822c0890
	ctx.lr = 0x82336944;
	sub_822C0890(ctx, base);
	// 82336944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336948: 48ABCAE1  bl 0x82df3428
	ctx.lr = 0x8233694C;
	sub_82DF3428(ctx, base);
	// 8233694C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336950: 4182006C  beq 0x823369bc
	if ctx.cr[0].eq {
	pc = 0x823369BC; continue 'dispatch;
	}
	// 82336954: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336958: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233695C: 388B1078  addi r4, r11, 0x1078
	ctx.r[4].s64 = ctx.r[11].s64 + 4216;
	// 82336960: 48ABD0A9  bl 0x82df3a08
	ctx.lr = 0x82336964;
	sub_82DF3A08(ctx, base);
	// 82336964: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 82336968: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8233696C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82336970: 3B9F006C  addi r28, r31, 0x6c
	ctx.r[28].s64 = ctx.r[31].s64 + 108;
	// 82336974: 4BFF5ECD  bl 0x8232c840
	ctx.lr = 0x82336978;
	sub_8232C840(ctx, base);
	// 82336978: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8233697C: 4BFFFEA5  bl 0x82336820
	ctx.lr = 0x82336980;
	sub_82336820(ctx, base);
	// 82336980: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336984: 48ABCAA5  bl 0x82df3428
	ctx.lr = 0x82336988;
	sub_82DF3428(ctx, base);
	// 82336988: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233698C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336990: 388B1068  addi r4, r11, 0x1068
	ctx.r[4].s64 = ctx.r[11].s64 + 4200;
	// 82336994: 48ABD075  bl 0x82df3a08
	ctx.lr = 0x82336998;
	sub_82DF3A08(ctx, base);
	// 82336998: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8233699C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823369A0: 3BFF0070  addi r31, r31, 0x70
	ctx.r[31].s64 = ctx.r[31].s64 + 112;
	// 823369A4: 4BFF5E9D  bl 0x8232c840
	ctx.lr = 0x823369A8;
	sub_8232C840(ctx, base);
	// 823369A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823369AC: 4BFE6A8D  bl 0x8231d438
	ctx.lr = 0x823369B0;
	sub_8231D438(ctx, base);
	// 823369B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823369B4: 48ABCA75  bl 0x82df3428
	ctx.lr = 0x823369B8;
	sub_82DF3428(ctx, base);
	// 823369B8: 48000030  b 0x823369e8
	pc = 0x823369E8; continue 'dispatch;
	// 823369BC: 897F0070  lbz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 823369C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823369C4: 41820018  beq 0x823369dc
	if ctx.cr[0].eq {
	pc = 0x823369DC; continue 'dispatch;
	}
	// 823369C8: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 823369CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823369D0: 4BFDD331  bl 0x82313d00
	ctx.lr = 0x823369D4;
	sub_82313D00(ctx, base);
	// 823369D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823369D8: 997F0070  stb r11, 0x70(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 823369DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823369E0: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 823369E4: 4BFE4075  bl 0x8231aa58
	ctx.lr = 0x823369E8;
	sub_8231AA58(ctx, base);
	// 823369E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823369EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823369F0: 4BFDD301  bl 0x82313cf0
	ctx.lr = 0x823369F4;
	sub_82313CF0(ctx, base);
	// 823369F4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 823369F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823369FC: 4BFDD2F5  bl 0x82313cf0
	ctx.lr = 0x82336A00;
	sub_82313CF0(ctx, base);
	// 82336A00: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82336A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A08: 4BFDD2E9  bl 0x82313cf0
	ctx.lr = 0x82336A0C;
	sub_82313CF0(ctx, base);
	// 82336A0C: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82336A10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A14: 4BFDD2DD  bl 0x82313cf0
	ctx.lr = 0x82336A18;
	sub_82313CF0(ctx, base);
	// 82336A18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82336A1C: 48E7179C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336A20 size=468
    let mut pc: u32 = 0x82336A20;
    'dispatch: loop {
        match pc {
            0x82336A20 => {
    //   block [0x82336A20..0x82336BF4)
	// 82336A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336A24: 48E71745  bl 0x831a8168
	ctx.lr = 0x82336A28;
	sub_831A8130(ctx, base);
	// 82336A28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336A2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336A30: 486D9E41  bl 0x82a10870
	ctx.lr = 0x82336A34;
	sub_82A10870(ctx, base);
	// 82336A34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82336A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336A3C: 48B22D25  bl 0x82e59760
	ctx.lr = 0x82336A40;
	sub_82E59760(ctx, base);
	// 82336A40: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82336A44: C00B6150  lfs f0, 0x6150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82336A48: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82336A4C: 409901A0  ble cr6, 0x82336bec
	if !ctx.cr[6].gt {
	pc = 0x82336BEC; continue 'dispatch;
	}
	// 82336A50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A54: 4BFDC38D  bl 0x82312de0
	ctx.lr = 0x82336A58;
	sub_82312DE0(ctx, base);
	// 82336A58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A5C: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82336A60: 4BFE3FF9  bl 0x8231aa58
	ctx.lr = 0x82336A64;
	sub_8231AA58(ctx, base);
	// 82336A64: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82336A68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A6C: 4BFDD275  bl 0x82313ce0
	ctx.lr = 0x82336A70;
	sub_82313CE0(ctx, base);
	// 82336A70: 897F0070  lbz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82336A74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82336A78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336A7C: 41820018  beq 0x82336a94
	if ctx.cr[0].eq {
	pc = 0x82336A94; continue 'dispatch;
	}
	// 82336A80: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82336A84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336A88: 4BFDD279  bl 0x82313d00
	ctx.lr = 0x82336A8C;
	sub_82313D00(ctx, base);
	// 82336A8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82336A90: 997F0070  stb r11, 0x70(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 82336A94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336A98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336A9C: 388B1384  addi r4, r11, 0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + 4996;
	// 82336AA0: 48ABCF69  bl 0x82df3a08
	ctx.lr = 0x82336AA4;
	sub_82DF3A08(ctx, base);
	// 82336AA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336AA8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82336AAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336AB0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82336AB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336AB8: 4E800421  bctrl
	ctx.lr = 0x82336ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82336ABC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336AC0: 48ABC969  bl 0x82df3428
	ctx.lr = 0x82336AC4;
	sub_82DF3428(ctx, base);
	// 82336AC4: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82336AC8: 41820028  beq 0x82336af0
	if ctx.cr[0].eq {
	pc = 0x82336AF0; continue 'dispatch;
	}
	// 82336ACC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336AD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336AD4: 388BD584  addi r4, r11, -0x2a7c
	ctx.r[4].s64 = ctx.r[11].s64 + -10876;
	// 82336AD8: 48ABCF31  bl 0x82df3a08
	ctx.lr = 0x82336ADC;
	sub_82DF3A08(ctx, base);
	// 82336ADC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82336AE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336AE4: 4BFCF1FD  bl 0x82305ce0
	ctx.lr = 0x82336AE8;
	sub_82305CE0(ctx, base);
	// 82336AE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336AEC: 48ABC93D  bl 0x82df3428
	ctx.lr = 0x82336AF0;
	sub_82DF3428(ctx, base);
	// 82336AF0: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82336AF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82336AF8: 419A006C  beq cr6, 0x82336b64
	if ctx.cr[6].eq {
	pc = 0x82336B64; continue 'dispatch;
	}
	// 82336AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336B00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336B04: 388B1378  addi r4, r11, 0x1378
	ctx.r[4].s64 = ctx.r[11].s64 + 4984;
	// 82336B08: 48ABCF01  bl 0x82df3a08
	ctx.lr = 0x82336B0C;
	sub_82DF3A08(ctx, base);
	// 82336B0C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82336B10: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82336B14: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82336B18: 4BFF5D29  bl 0x8232c840
	ctx.lr = 0x82336B1C;
	sub_8232C840(ctx, base);
	// 82336B1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336B20: 4BFE6919  bl 0x8231d438
	ctx.lr = 0x82336B24;
	sub_8231D438(ctx, base);
	// 82336B24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336B28: 48ABC901  bl 0x82df3428
	ctx.lr = 0x82336B2C;
	sub_82DF3428(ctx, base);
	// 82336B2C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336B30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336B34: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82336B38: 48ABCED1  bl 0x82df3a08
	ctx.lr = 0x82336B3C;
	sub_82DF3A08(ctx, base);
	// 82336B3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82336B40: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82336B44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336B48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82336B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82336B50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82336B54: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336B58: 48B27F39  bl 0x82e5ea90
	ctx.lr = 0x82336B5C;
	sub_82E5EA90(ctx, base);
	// 82336B5C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82336B60: 48000080  b 0x82336be0
	pc = 0x82336BE0; continue 'dispatch;
	// 82336B64: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82336B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336B6C: 4BFDDE4D  bl 0x823149b8
	ctx.lr = 0x82336B70;
	sub_823149B8(ctx, base);
	// 82336B70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336B74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336B78: 41820038  beq 0x82336bb0
	if ctx.cr[0].eq {
	pc = 0x82336BB0; continue 'dispatch;
	}
	// 82336B7C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336B80: 808BB3BC  lwz r4, -0x4c44(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19524 as u32) ) } as u64;
	// 82336B84: 48ABCE85  bl 0x82df3a08
	ctx.lr = 0x82336B88;
	sub_82DF3A08(ctx, base);
	// 82336B88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82336B8C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82336B90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336B94: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82336B98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82336B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82336BA0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336BA4: 48B27EED  bl 0x82e5ea90
	ctx.lr = 0x82336BA8;
	sub_82E5EA90(ctx, base);
	// 82336BA8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82336BAC: 48000034  b 0x82336be0
	pc = 0x82336BE0; continue 'dispatch;
	// 82336BB0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336BB4: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82336BB8: 48ABCE51  bl 0x82df3a08
	ctx.lr = 0x82336BBC;
	sub_82DF3A08(ctx, base);
	// 82336BBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82336BC0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82336BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336BC8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82336BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82336BD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82336BD4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336BD8: 48B27EB9  bl 0x82e5ea90
	ctx.lr = 0x82336BDC;
	sub_82E5EA90(ctx, base);
	// 82336BDC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82336BE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82336BE4: 419A0008  beq cr6, 0x82336bec
	if ctx.cr[6].eq {
	pc = 0x82336BEC; continue 'dispatch;
	}
	// 82336BE8: 4BF89CA9  bl 0x822c0890
	ctx.lr = 0x82336BEC;
	sub_822C0890(ctx, base);
	// 82336BEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82336BF0: 48E715C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336BF8 size=88
    let mut pc: u32 = 0x82336BF8;
    'dispatch: loop {
        match pc {
            0x82336BF8 => {
    //   block [0x82336BF8..0x82336C50)
	// 82336BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336C04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336C08: 486D9C69  bl 0x82a10870
	ctx.lr = 0x82336C0C;
	sub_82A10870(ctx, base);
	// 82336C0C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336C14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336C18: 808BB540  lwz r4, -0x4ac0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19136 as u32) ) } as u64;
	// 82336C1C: 48ABCDED  bl 0x82df3a08
	ctx.lr = 0x82336C20;
	sub_82DF3A08(ctx, base);
	// 82336C20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82336C24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82336C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336C2C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336C30: 4BFCE939  bl 0x82305568
	ctx.lr = 0x82336C34;
	sub_82305568(ctx, base);
	// 82336C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336C38: 48ABC7F1  bl 0x82df3428
	ctx.lr = 0x82336C3C;
	sub_82DF3428(ctx, base);
	// 82336C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82336C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82336C50 size=384
    let mut pc: u32 = 0x82336C50;
    'dispatch: loop {
        match pc {
            0x82336C50 => {
    //   block [0x82336C50..0x82336DD0)
	// 82336C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336C58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336C5C: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336DD0 size=160
    let mut pc: u32 = 0x82336DD0;
    'dispatch: loop {
        match pc {
            0x82336DD0 => {
    //   block [0x82336DD0..0x82336E70)
	// 82336DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82336DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336DE8: 486D9A89  bl 0x82a10870
	ctx.lr = 0x82336DEC;
	sub_82A10870(ctx, base);
	// 82336DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82336DF0: 4BFCE639  bl 0x82305428
	ctx.lr = 0x82336DF4;
	sub_82305428(ctx, base);
	// 82336DF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336DF8: 41820048  beq 0x82336e40
	if ctx.cr[0].eq {
	pc = 0x82336E40; continue 'dispatch;
	}
	// 82336DFC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336E04: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82336E08: 48ABCC01  bl 0x82df3a08
	ctx.lr = 0x82336E0C;
	sub_82DF3A08(ctx, base);
	// 82336E0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82336E10: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82336E14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336E18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82336E20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82336E24: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82336E28: 48B27C69  bl 0x82e5ea90
	ctx.lr = 0x82336E2C;
	sub_82E5EA90(ctx, base);
	// 82336E2C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82336E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82336E34: 419A0024  beq cr6, 0x82336e58
	if ctx.cr[6].eq {
	pc = 0x82336E58; continue 'dispatch;
	}
	// 82336E38: 4BF89A59  bl 0x822c0890
	ctx.lr = 0x82336E3C;
	sub_822C0890(ctx, base);
	// 82336E3C: 4800001C  b 0x82336e58
	pc = 0x82336E58; continue 'dispatch;
	// 82336E40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82336E4C: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82336E50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82336E54: 4E800421  bctrl
	ctx.lr = 0x82336E58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82336E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82336E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336E64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82336E68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336E70 size=60
    let mut pc: u32 = 0x82336E70;
    'dispatch: loop {
        match pc {
            0x82336E70 => {
    //   block [0x82336E70..0x82336EAC)
	// 82336E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336E80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336E84: 4BFFAF55  bl 0x82331dd8
	ctx.lr = 0x82336E88;
	sub_82331DD8(ctx, base);
	// 82336E88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336E90: 396B1394  addi r11, r11, 0x1394
	ctx.r[11].s64 = ctx.r[11].s64 + 5012;
	// 82336E94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82336E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82336E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82336EB0 size=100
    let mut pc: u32 = 0x82336EB0;
    'dispatch: loop {
        match pc {
            0x82336EB0 => {
    //   block [0x82336EB0..0x82336F14)
	// 82336EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82336EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82336EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82336EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336EC4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82336EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336ED0: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 82336ED4: 48ABCB35  bl 0x82df3a08
	ctx.lr = 0x82336ED8;
	sub_82DF3A08(ctx, base);
	// 82336ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336EDC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82336EE0: 486D9991  bl 0x82a10870
	ctx.lr = 0x82336EE4;
	sub_82A10870(ctx, base);
	// 82336EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336EE8: 4BFCE661  bl 0x82305548
	ctx.lr = 0x82336EEC;
	sub_82305548(ctx, base);
	// 82336EEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336EF0: 48ABC539  bl 0x82df3428
	ctx.lr = 0x82336EF4;
	sub_82DF3428(ctx, base);
	// 82336EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336EF8: 48D90BE9  bl 0x830c7ae0
	ctx.lr = 0x82336EFC;
	sub_830C7AE0(ctx, base);
	// 82336EFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82336F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82336F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82336F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82336F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82336F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82336F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82336F18 size=432
    let mut pc: u32 = 0x82336F18;
    'dispatch: loop {
        match pc {
            0x82336F18 => {
    //   block [0x82336F18..0x823370C8)
	// 82336F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82336F1C: 48E7124D  bl 0x831a8168
	ctx.lr = 0x82336F20;
	sub_831A8130(ctx, base);
	// 82336F20: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82336F24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82336F28: 486D9949  bl 0x82a10870
	ctx.lr = 0x82336F2C;
	sub_82A10870(ctx, base);
	// 82336F2C: 897D0068  lbz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 82336F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82336F34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82336F38: 40820074  bne 0x82336fac
	if !ctx.cr[0].eq {
	pc = 0x82336FAC; continue 'dispatch;
	}
	// 82336F3C: 4BFCE575  bl 0x823054b0
	ctx.lr = 0x82336F40;
	sub_823054B0(ctx, base);
	// 82336F40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336F44: C00BA1C4  lfs f0, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82336F48: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82336F4C: 40990060  ble cr6, 0x82336fac
	if !ctx.cr[6].gt {
	pc = 0x82336FAC; continue 'dispatch;
	}
	// 82336F50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82336F54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336F58: 3BCB0D90  addi r30, r11, 0xd90
	ctx.r[30].s64 = ctx.r[11].s64 + 3472;
	// 82336F5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336F60: 48ABCAA9  bl 0x82df3a08
	ctx.lr = 0x82336F64;
	sub_82DF3A08(ctx, base);
	// 82336F64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82336F68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336F6C: 48ABCA9D  bl 0x82df3a08
	ctx.lr = 0x82336F70;
	sub_82DF3A08(ctx, base);
	// 82336F70: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82336F74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82336F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82336F7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82336F80: 4BFD2159  bl 0x823090d8
	ctx.lr = 0x82336F84;
	sub_823090D8(ctx, base);
	// 82336F84: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82336F88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82336F8C: 419A0008  beq cr6, 0x82336f94
	if ctx.cr[6].eq {
	pc = 0x82336F94; continue 'dispatch;
	}
	// 82336F90: 4BF89901  bl 0x822c0890
	ctx.lr = 0x82336F94;
	sub_822C0890(ctx, base);
	// 82336F94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82336F98: 48ABC491  bl 0x82df3428
	ctx.lr = 0x82336F9C;
	sub_82DF3428(ctx, base);
	// 82336F9C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336FA0: 48ABC489  bl 0x82df3428
	ctx.lr = 0x82336FA4;
	sub_82DF3428(ctx, base);
	// 82336FA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82336FA8: 997D0068  stb r11, 0x68(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82336FAC: 3FC08326  lis r30, -0x7cda
	ctx.r[30].s64 = -2094661632;
	// 82336FB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336FB4: 809EB078  lwz r4, -0x4f88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-20360 as u32) ) } as u64;
	// 82336FB8: 48ABCA51  bl 0x82df3a08
	ctx.lr = 0x82336FBC;
	sub_82DF3A08(ctx, base);
	// 82336FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336FC0: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 82336FC4: 4BFCE43D  bl 0x82305400
	ctx.lr = 0x82336FC8;
	sub_82305400(ctx, base);
	// 82336FC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82336FCC: 48ABC33D  bl 0x82df3308
	ctx.lr = 0x82336FD0;
	sub_82DF3308(ctx, base);
	// 82336FD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82336FD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82336FD8: 48ABC451  bl 0x82df3428
	ctx.lr = 0x82336FDC;
	sub_82DF3428(ctx, base);
	// 82336FDC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336FE0: 4182002C  beq 0x8233700c
	if ctx.cr[0].eq {
	pc = 0x8233700C; continue 'dispatch;
	}
	// 82336FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82336FE8: 4BFCE441  bl 0x82305428
	ctx.lr = 0x82336FEC;
	sub_82305428(ctx, base);
	// 82336FEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82336FF0: 418200A8  beq 0x82337098
	if ctx.cr[0].eq {
	pc = 0x82337098; continue 'dispatch;
	}
	// 82336FF4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82336FF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82336FFC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82337000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82337004: 4E800421  bctrl
	ctx.lr = 0x82337008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82337008: 480000B8  b 0x823370c0
	pc = 0x823370C0; continue 'dispatch;
	// 8233700C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337010: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337014: 808BB070  lwz r4, -0x4f90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20368 as u32) ) } as u64;
	// 82337018: 48ABC9F1  bl 0x82df3a08
	ctx.lr = 0x8233701C;
	sub_82DF3A08(ctx, base);
	// 8233701C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337020: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 82337024: 4BFCE3DD  bl 0x82305400
	ctx.lr = 0x82337028;
	sub_82305400(ctx, base);
	// 82337028: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8233702C: 48ABC275  bl 0x82df32a0
	ctx.lr = 0x82337030;
	sub_82DF32A0(ctx, base);
	// 82337030: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82337034: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337038: 48ABC3F1  bl 0x82df3428
	ctx.lr = 0x8233703C;
	sub_82DF3428(ctx, base);
	// 8233703C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82337040: 41820058  beq 0x82337098
	if ctx.cr[0].eq {
	pc = 0x82337098; continue 'dispatch;
	}
	// 82337044: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82337048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233704C: 4BFCEB55  bl 0x82305ba0
	ctx.lr = 0x82337050;
	sub_82305BA0(ctx, base);
	// 82337050: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82337054: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82337058: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8233705C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823370C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823370C8 size=388
    let mut pc: u32 = 0x823370C8;
    'dispatch: loop {
        match pc {
            0x823370C8 => {
    //   block [0x823370C8..0x8233724C)
	// 823370C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823370CC: 48E710A1  bl 0x831a816c
	ctx.lr = 0x823370D0;
	sub_831A8130(ctx, base);
	// 823370D0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823370D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823370D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823370DC: 486D9795  bl 0x82a10870
	ctx.lr = 0x823370E0;
	sub_82A10870(ctx, base);
	// 823370E0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823370E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823370E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823370EC: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 823370F0: 48ABC919  bl 0x82df3a08
	ctx.lr = 0x823370F4;
	sub_82DF3A08(ctx, base);
	// 823370F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823370F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823370FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82337100: 4BFCE441  bl 0x82305540
	ctx.lr = 0x82337104;
	sub_82305540(ctx, base);
	// 82337104: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337108: 48ABC321  bl 0x82df3428
	ctx.lr = 0x8233710C;
	sub_82DF3428(ctx, base);
	// 8233710C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337114: 388B1384  addi r4, r11, 0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + 4996;
	// 82337118: 48ABC8F1  bl 0x82df3a08
	ctx.lr = 0x8233711C;
	sub_82DF3A08(ctx, base);
	// 8233711C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337124: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82337128: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233712C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82337130: 4E800421  bctrl
	ctx.lr = 0x82337134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82337134: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337138: 48ABC2F1  bl 0x82df3428
	ctx.lr = 0x8233713C;
	sub_82DF3428(ctx, base);
	// 8233713C: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 82337140: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82337144: 48123CA5  bl 0x8245ade8
	ctx.lr = 0x82337148;
	sub_8245ADE8(ctx, base);
	// 82337148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233714C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82337150: 4BFDFE91  bl 0x82316fe0
	ctx.lr = 0x82337154;
	sub_82316FE0(ctx, base);
	// 82337154: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82337158: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233715C: 4099006C  ble cr6, 0x823371c8
	if !ctx.cr[6].gt {
	pc = 0x823371C8; continue 'dispatch;
	}
	// 82337160: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337164: 3BAB0D90  addi r29, r11, 0xd90
	ctx.r[29].s64 = ctx.r[11].s64 + 3472;
	// 82337168: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233716C: 48ABC89D  bl 0x82df3a08
	ctx.lr = 0x82337170;
	sub_82DF3A08(ctx, base);
	// 82337170: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82337174: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337178: 48ABC891  bl 0x82df3a08
	ctx.lr = 0x8233717C;
	sub_82DF3A08(ctx, base);
	// 8233717C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82337180: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82337184: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337188: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233718C: 4BFD1F4D  bl 0x823090d8
	ctx.lr = 0x82337190;
	sub_823090D8(ctx, base);
	// 82337190: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82337194: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82337198: 419A0008  beq cr6, 0x823371a0
	if ctx.cr[6].eq {
	pc = 0x823371A0; continue 'dispatch;
	}
	// 8233719C: 4BF896F5  bl 0x822c0890
	ctx.lr = 0x823371A0;
	sub_822C0890(ctx, base);
	// 823371A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823371A4: 48ABC285  bl 0x82df3428
	ctx.lr = 0x823371A8;
	sub_82DF3428(ctx, base);
	// 823371A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823371AC: 48ABC27D  bl 0x82df3428
	ctx.lr = 0x823371B0;
	sub_82DF3428(ctx, base);
	// 823371B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823371B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823371B8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823371BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823371C0: 4E800421  bctrl
	ctx.lr = 0x823371C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823371C4: 4800007C  b 0x82337240
	pc = 0x82337240; continue 'dispatch;
	// 823371C8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823371CC: 808BB070  lwz r4, -0x4f90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20368 as u32) ) } as u64;
	// 823371D0: 48ABC839  bl 0x82df3a08
	ctx.lr = 0x823371D4;
	sub_82DF3A08(ctx, base);
	// 823371D4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823371D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823371DC: 4BFD06FD  bl 0x823078d8
	ctx.lr = 0x823371E0;
	sub_823078D8(ctx, base);
	// 823371E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823371E4: 48ABC245  bl 0x82df3428
	ctx.lr = 0x823371E8;
	sub_82DF3428(ctx, base);
	// 823371E8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823371EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823371F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823371F4: 808B1674  lwz r4, 0x1674(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5748 as u32) ) } as u64;
	// 823371F8: 48ABAA01  bl 0x82df1bf8
	ctx.lr = 0x823371FC;
	sub_82DF1BF8(ctx, base);
	// 823371FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337200: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337204: 388B13BC  addi r4, r11, 0x13bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5052;
	// 82337208: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8233720C: 48AC2325  bl 0x82df9530
	ctx.lr = 0x82337210;
	sub_82DF9530(ctx, base);
	// 82337210: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82337214: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82337218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233721C: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337220: C02B7BC8  lfs f1, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337224: 481EF22D  bl 0x82526450
	ctx.lr = 0x82337228;
	sub_82526450(ctx, base);
	// 82337228: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233722C: 48ABAA65  bl 0x82df1c90
	ctx.lr = 0x82337230;
	sub_82DF1C90(ctx, base);
	// 82337230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82337234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337238: 997E0068  stb r11, 0x68(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 8233723C: 48D908A5  bl 0x830c7ae0
	ctx.lr = 0x82337240;
	sub_830C7AE0(ctx, base);
	// 82337240: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82337244: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82337248: 48E70F74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337250 size=104
    let mut pc: u32 = 0x82337250;
    'dispatch: loop {
        match pc {
            0x82337250 => {
    //   block [0x82337250..0x823372B8)
	// 82337250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233725C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337260: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233726C: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82337270: 48ABC799  bl 0x82df3a08
	ctx.lr = 0x82337274;
	sub_82DF3A08(ctx, base);
	// 82337274: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337278: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233727C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337280: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82337284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82337288: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233728C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337290: 48B27801  bl 0x82e5ea90
	ctx.lr = 0x82337294;
	sub_82E5EA90(ctx, base);
	// 82337294: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82337298: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233729C: 419A0008  beq cr6, 0x823372a4
	if ctx.cr[6].eq {
	pc = 0x823372A4; continue 'dispatch;
	}
	// 823372A0: 4BF895F1  bl 0x822c0890
	ctx.lr = 0x823372A4;
	sub_822C0890(ctx, base);
	// 823372A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823372A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823372AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823372B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823372B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823372B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823372B8 size=72
    let mut pc: u32 = 0x823372B8;
    'dispatch: loop {
        match pc {
            0x823372B8 => {
    //   block [0x823372B8..0x82337300)
	// 823372B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823372BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823372C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823372C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823372C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823372CC: 4BFFAB0D  bl 0x82331dd8
	ctx.lr = 0x823372D0;
	sub_82331DD8(ctx, base);
	// 823372D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823372D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823372D8: 396B13C8  addi r11, r11, 0x13c8
	ctx.r[11].s64 = ctx.r[11].s64 + 5064;
	// 823372DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823372E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823372E4: 4BFFA8C5  bl 0x82331ba8
	ctx.lr = 0x823372E8;
	sub_82331BA8(ctx, base);
	// 823372E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823372EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823372F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823372F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823372F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823372FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337300 size=376
    let mut pc: u32 = 0x82337300;
    'dispatch: loop {
        match pc {
            0x82337300 => {
    //   block [0x82337300..0x82337478)
	// 82337300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337304: 48E70E69  bl 0x831a816c
	ctx.lr = 0x82337308;
	sub_831A8130(ctx, base);
	// 82337308: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233730C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337314: 486D955D  bl 0x82a10870
	ctx.lr = 0x82337318;
	sub_82A10870(ctx, base);
	// 82337318: 897F0068  lbz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233731C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337320: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82337324: 40820074  bne 0x82337398
	if !ctx.cr[0].eq {
	pc = 0x82337398; continue 'dispatch;
	}
	// 82337328: 4BFCE189  bl 0x823054b0
	ctx.lr = 0x8233732C;
	sub_823054B0(ctx, base);
	// 8233732C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337330: C00BA1C4  lfs f0, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82337334: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82337338: 40990060  ble cr6, 0x82337398
	if !ctx.cr[6].gt {
	pc = 0x82337398; continue 'dispatch;
	}
	// 8233733C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337340: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337344: 3BAB0D90  addi r29, r11, 0xd90
	ctx.r[29].s64 = ctx.r[11].s64 + 3472;
	// 82337348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233734C: 48ABC6BD  bl 0x82df3a08
	ctx.lr = 0x82337350;
	sub_82DF3A08(ctx, base);
	// 82337350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82337354: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337358: 48ABC6B1  bl 0x82df3a08
	ctx.lr = 0x8233735C;
	sub_82DF3A08(ctx, base);
	// 8233735C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82337360: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82337364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82337368: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233736C: 4BFD1D6D  bl 0x823090d8
	ctx.lr = 0x82337370;
	sub_823090D8(ctx, base);
	// 82337370: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82337374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82337378: 419A0008  beq cr6, 0x82337380
	if ctx.cr[6].eq {
	pc = 0x82337380; continue 'dispatch;
	}
	// 8233737C: 4BF89515  bl 0x822c0890
	ctx.lr = 0x82337380;
	sub_822C0890(ctx, base);
	// 82337380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337384: 48ABC0A5  bl 0x82df3428
	ctx.lr = 0x82337388;
	sub_82DF3428(ctx, base);
	// 82337388: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233738C: 48ABC09D  bl 0x82df3428
	ctx.lr = 0x82337390;
	sub_82DF3428(ctx, base);
	// 82337390: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82337394: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82337398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233739C: 4BFCE08D  bl 0x82305428
	ctx.lr = 0x823373A0;
	sub_82305428(ctx, base);
	// 823373A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823373A4: 41820048  beq 0x823373ec
	if ctx.cr[0].eq {
	pc = 0x823373EC; continue 'dispatch;
	}
	// 823373A8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823373AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823373B0: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 823373B4: 48ABC655  bl 0x82df3a08
	ctx.lr = 0x823373B8;
	sub_82DF3A08(ctx, base);
	// 823373B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823373BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823373C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823373C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823373C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823373CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823373D0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823373D4: 48B276BD  bl 0x82e5ea90
	ctx.lr = 0x823373D8;
	sub_82E5EA90(ctx, base);
	// 823373D8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823373DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823373E0: 419A008C  beq cr6, 0x8233746c
	if ctx.cr[6].eq {
	pc = 0x8233746C; continue 'dispatch;
	}
	// 823373E4: 4BF894AD  bl 0x822c0890
	ctx.lr = 0x823373E8;
	sub_822C0890(ctx, base);
	// 823373E8: 48000084  b 0x8233746c
	pc = 0x8233746C; continue 'dispatch;
	// 823373EC: 38800053  li r4, 0x53
	ctx.r[4].s64 = 83;
	// 823373F0: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823373F4: 481239F5  bl 0x8245ade8
	ctx.lr = 0x823373F8;
	sub_8245ADE8(ctx, base);
	// 823373F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823373FC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82337400: 4BFDFBE1  bl 0x82316fe0
	ctx.lr = 0x82337404;
	sub_82316FE0(ctx, base);
	// 82337404: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82337408: 4099003C  ble cr6, 0x82337444
	if !ctx.cr[6].gt {
	pc = 0x82337444; continue 'dispatch;
	}
	// 8233740C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337410: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337414: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 82337418: 48ABC5F1  bl 0x82df3a08
	ctx.lr = 0x8233741C;
	sub_82DF3A08(ctx, base);
	// 8233741C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337420: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82337424: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337428: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8233742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82337430: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82337434: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337438: 48B27659  bl 0x82e5ea90
	ctx.lr = 0x8233743C;
	sub_82E5EA90(ctx, base);
	// 8233743C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82337440: 4BFFFF9C  b 0x823373dc
	pc = 0x823373DC; continue 'dispatch;
	// 82337444: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337448: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233744C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337450: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82337454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82337458: 4E800421  bctrl
	ctx.lr = 0x8233745C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233745C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82337460: 4082000C  bne 0x8233746c
	if !ctx.cr[0].eq {
	pc = 0x8233746C; continue 'dispatch;
	}
	// 82337464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337468: 48D90679  bl 0x830c7ae0
	ctx.lr = 0x8233746C;
	sub_830C7AE0(ctx, base);
	// 8233746C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82337470: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82337474: 48E70D48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337478 size=580
    let mut pc: u32 = 0x82337478;
    'dispatch: loop {
        match pc {
            0x82337478 => {
    //   block [0x82337478..0x823376BC)
	// 82337478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233747C: 48E70CED  bl 0x831a8168
	ctx.lr = 0x82337480;
	sub_831A8130(ctx, base);
	// 82337480: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82337484: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337488: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233748C: 486D93E5  bl 0x82a10870
	ctx.lr = 0x82337490;
	sub_82A10870(ctx, base);
	// 82337490: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337498: 388B1384  addi r4, r11, 0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + 4996;
	// 8233749C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823374A0: 48ABC569  bl 0x82df3a08
	ctx.lr = 0x823374A4;
	sub_82DF3A08(ctx, base);
	// 823374A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823374A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823374AC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823374B0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 823374B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823374B8: 4E800421  bctrl
	ctx.lr = 0x823374BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823374BC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823374C0: 48ABBF69  bl 0x82df3428
	ctx.lr = 0x823374C4;
	sub_82DF3428(ctx, base);
	// 823374C4: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 823374C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823374CC: 4BFDC815  bl 0x82313ce0
	ctx.lr = 0x823374D0;
	sub_82313CE0(ctx, base);
	// 823374D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823374D4: 40820040  bne 0x82337514
	if !ctx.cr[0].eq {
	pc = 0x82337514; continue 'dispatch;
	}
	// 823374D8: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 823374DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823374E0: 4BFDD4D9  bl 0x823149b8
	ctx.lr = 0x823374E4;
	sub_823149B8(ctx, base);
	// 823374E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823374E8: 4182002C  beq 0x82337514
	if ctx.cr[0].eq {
	pc = 0x82337514; continue 'dispatch;
	}
	// 823374EC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823374F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823374F4: 808BB3BC  lwz r4, -0x4c44(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19524 as u32) ) } as u64;
	// 823374F8: 48ABC511  bl 0x82df3a08
	ctx.lr = 0x823374FC;
	sub_82DF3A08(ctx, base);
	// 823374FC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82337500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337504: 4BFCE08D  bl 0x82305590
	ctx.lr = 0x82337508;
	sub_82305590(ctx, base);
	// 82337508: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233750C: 48ABBF1D  bl 0x82df3428
	ctx.lr = 0x82337510;
	sub_82DF3428(ctx, base);
	// 82337510: 480001A0  b 0x823376b0
	pc = 0x823376B0; continue 'dispatch;
	// 82337514: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337518: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8233751C: 388B108C  addi r4, r11, 0x108c
	ctx.r[4].s64 = ctx.r[11].s64 + 4236;
	// 82337520: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337524: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82337528: 48ABC4E1  bl 0x82df3a08
	ctx.lr = 0x8233752C;
	sub_82DF3A08(ctx, base);
	// 8233752C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82337530: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82337534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82337538: 4BFFCCA1  bl 0x823341d8
	ctx.lr = 0x8233753C;
	sub_823341D8(ctx, base);
	// 8233753C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82337540: 48ABBEE9  bl 0x82df3428
	ctx.lr = 0x82337544;
	sub_82DF3428(ctx, base);
	// 82337544: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82337548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233754C: 4BFCE655  bl 0x82305ba0
	ctx.lr = 0x82337550;
	sub_82305BA0(ctx, base);
	// 82337550: 8BC10050  lbz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82337554: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337558: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233755C: 41820010  beq 0x8233756c
	if ctx.cr[0].eq {
	pc = 0x8233756C; continue 'dispatch;
	}
	// 82337560: 396B8CF0  addi r11, r11, -0x7310
	ctx.r[11].s64 = ctx.r[11].s64 + -29456;
	// 82337564: C3EB0004  lfs f31, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82337568: 48000008  b 0x82337570
	pc = 0x82337570; continue 'dispatch;
	// 8233756C: C3EB8CF0  lfs f31, -0x7310(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29456 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82337570: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82337574: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82337578: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8233757C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823376C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823376C0 size=72
    let mut pc: u32 = 0x823376C0;
    'dispatch: loop {
        match pc {
            0x823376C0 => {
    //   block [0x823376C0..0x82337708)
	// 823376C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823376C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823376C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823376CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823376D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823376D4: 4BFFA705  bl 0x82331dd8
	ctx.lr = 0x823376D8;
	sub_82331DD8(ctx, base);
	// 823376D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823376DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823376E0: 396B1460  addi r11, r11, 0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + 5216;
	// 823376E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823376E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823376EC: 4BFFA4BD  bl 0x82331ba8
	ctx.lr = 0x823376F0;
	sub_82331BA8(ctx, base);
	// 823376F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823376F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823376F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823376FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82337700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82337704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82337708 size=36
    let mut pc: u32 = 0x82337708;
    'dispatch: loop {
        match pc {
            0x82337708 => {
    //   block [0x82337708..0x8233772C)
	// 82337708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337714: 486D915D  bl 0x82a10870
	ctx.lr = 0x82337718;
	sub_82A10870(ctx, base);
	// 82337718: 4BFDBF19  bl 0x82313630
	ctx.lr = 0x8233771C;
	sub_82313630(ctx, base);
	// 8233771C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82337720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82337724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82337728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337730 size=368
    let mut pc: u32 = 0x82337730;
    'dispatch: loop {
        match pc {
            0x82337730 => {
    //   block [0x82337730..0x823378A0)
	// 82337730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233773C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82337740: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337748: 486D9129  bl 0x82a10870
	ctx.lr = 0x8233774C;
	sub_82A10870(ctx, base);
	// 8233774C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337750: 4BFCDCD9  bl 0x82305428
	ctx.lr = 0x82337754;
	sub_82305428(ctx, base);
	// 82337754: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82337758: 41820048  beq 0x823377a0
	if ctx.cr[0].eq {
	pc = 0x823377A0; continue 'dispatch;
	}
	// 8233775C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337764: 808BB38C  lwz r4, -0x4c74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19572 as u32) ) } as u64;
	// 82337768: 48ABC2A1  bl 0x82df3a08
	ctx.lr = 0x8233776C;
	sub_82DF3A08(ctx, base);
	// 8233776C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337770: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82337774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337778: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233777C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82337780: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82337784: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337788: 48B27309  bl 0x82e5ea90
	ctx.lr = 0x8233778C;
	sub_82E5EA90(ctx, base);
	// 8233778C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82337790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82337794: 419A00F4  beq cr6, 0x82337888
	if ctx.cr[6].eq {
	pc = 0x82337888; continue 'dispatch;
	}
	// 82337798: 4BF890F9  bl 0x822c0890
	ctx.lr = 0x8233779C;
	sub_822C0890(ctx, base);
	// 8233779C: 480000EC  b 0x82337888
	pc = 0x82337888; continue 'dispatch;
	// 823377A0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 823377A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823377A8: 4BFCE3F9  bl 0x82305ba0
	ctx.lr = 0x823377AC;
	sub_82305BA0(ctx, base);
	// 823377AC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 823377B0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 823377B4: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823377B8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 823377BC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823378A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823378A0 size=444
    let mut pc: u32 = 0x823378A0;
    'dispatch: loop {
        match pc {
            0x823378A0 => {
    //   block [0x823378A0..0x82337A5C)
	// 823378A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823378A4: 48E708C5  bl 0x831a8168
	ctx.lr = 0x823378A8;
	sub_831A8130(ctx, base);
	// 823378A8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 823378AC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 823378B0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823378B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823378B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823378BC: 486D8FB5  bl 0x82a10870
	ctx.lr = 0x823378C0;
	sub_82A10870(ctx, base);
	// 823378C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823378C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823378C8: 48D90219  bl 0x830c7ae0
	ctx.lr = 0x823378CC;
	sub_830C7AE0(ctx, base);
	// 823378CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823378D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823378D4: 4BFD03ED  bl 0x82307cc0
	ctx.lr = 0x823378D8;
	sub_82307CC0(ctx, base);
	// 823378D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823378DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823378E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 823378E4: 4BFE23C5  bl 0x82319ca8
	ctx.lr = 0x823378E8;
	sub_82319CA8(ctx, base);
	// 823378E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823378EC: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823378F0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 823378F4: 3D408326  lis r10, -0x7cda
	ctx.r[10].s64 = -2094661632;
	// 823378F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823378FC: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337A60 size=152
    let mut pc: u32 = 0x82337A60;
    'dispatch: loop {
        match pc {
            0x82337A60 => {
    //   block [0x82337A60..0x82337AF8)
	// 82337A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82337A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82337A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337A74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337A78: 486D8DF9  bl 0x82a10870
	ctx.lr = 0x82337A7C;
	sub_82A10870(ctx, base);
	// 82337A7C: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 82337A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337A84: 4BFE2FD5  bl 0x8231aa58
	ctx.lr = 0x82337A88;
	sub_8231AA58(ctx, base);
	// 82337A88: 897E009C  lbz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82337A8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82337A90: 41820010  beq 0x82337aa0
	if ctx.cr[0].eq {
	pc = 0x82337AA0; continue 'dispatch;
	}
	// 82337A94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337A98: 809E0098  lwz r4, 0x98(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82337A9C: 4BFE920D  bl 0x82320ca8
	ctx.lr = 0x82337AA0;
	sub_82320CA8(ctx, base);
	// 82337AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337AA4: 4BFDBB8D  bl 0x82313630
	ctx.lr = 0x82337AA8;
	sub_82313630(ctx, base);
	// 82337AA8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337AAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337AB0: 808BB4EC  lwz r4, -0x4b14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19220 as u32) ) } as u64;
	// 82337AB4: 48ABBF55  bl 0x82df3a08
	ctx.lr = 0x82337AB8;
	sub_82DF3A08(ctx, base);
	// 82337AB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82337ABC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82337AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337AC4: 4BFCDA75  bl 0x82305538
	ctx.lr = 0x82337AC8;
	sub_82305538(ctx, base);
	// 82337AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337ACC: 48ABB95D  bl 0x82df3428
	ctx.lr = 0x82337AD0;
	sub_82DF3428(ctx, base);
	// 82337AD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337AD8: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337ADC: 4BFDB1D5  bl 0x82312cb0
	ctx.lr = 0x82337AE0;
	sub_82312CB0(ctx, base);
	// 82337AE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82337AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82337AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82337AEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82337AF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82337AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82337AF8 size=16
    let mut pc: u32 = 0x82337AF8;
    'dispatch: loop {
        match pc {
            0x82337AF8 => {
    //   block [0x82337AF8..0x82337B08)
	// 82337AF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82337AFC: 9883009C  stb r4, 0x9c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[4].u8 ) };
	// 82337B00: 996300A8  stb r11, 0xa8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u8 ) };
	// 82337B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337B08 size=364
    let mut pc: u32 = 0x82337B08;
    'dispatch: loop {
        match pc {
            0x82337B08 => {
    //   block [0x82337B08..0x82337C74)
	// 82337B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337B0C: 48E70661  bl 0x831a816c
	ctx.lr = 0x82337B10;
	sub_831A8130(ctx, base);
	// 82337B10: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82337B14: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82337B18: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82337B1C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337B20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337B24: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82337B28: 486D8D49  bl 0x82a10870
	ctx.lr = 0x82337B2C;
	sub_82A10870(ctx, base);
	// 82337B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337B30: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82337B34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82337B38: 4BFD0189  bl 0x82307cc0
	ctx.lr = 0x82337B3C;
	sub_82307CC0(ctx, base);
	// 82337B3C: C0410088  lfs f2, 0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82337B40: C0210080  lfs f1, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337B44: 48E73475  bl 0x831aafb8
	ctx.lr = 0x82337B48;
	sub_831AAFB8(ctx, base);
	// 82337B48: FFE00818  frsp f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82337B4C: C05D0008  lfs f2, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82337B50: C03D0000  lfs f1, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337B54: 48E73465  bl 0x831aafb8
	ctx.lr = 0x82337B58;
	sub_831AAFB8(ctx, base);
	// 82337B58: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82337B5C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82337B60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337B64: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82337B68: 48160269  bl 0x82497dd0
	ctx.lr = 0x82337B6C;
	sub_82497DD0(ctx, base);
	// 82337B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337B70: 3BBF00A4  addi r29, r31, 0xa4
	ctx.r[29].s64 = ctx.r[31].s64 + 164;
	// 82337B74: 48B21C05  bl 0x82e59778
	ctx.lr = 0x82337B78;
	sub_82E59778(ctx, base);
	// 82337B78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337B7C: 3D408326  lis r10, -0x7cda
	ctx.r[10].s64 = -2094661632;
	// 82337B80: FCC00890  fmr f6, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[6].f64 = ctx.f[1].f64;
	// 82337B84: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82337B88: C0410050  lfs f2, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82337B8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82337B90: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82337B94: C0ABF388  lfs f5, -0xc78(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3192 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82337B98: C08A8D58  lfs f4, -0x72a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29352 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82337B9C: C0691514  lfs f3, 0x1514(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5396 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82337BA0: 481602C1  bl 0x82497e60
	ctx.lr = 0x82337BA4;
	sub_82497E60(ctx, base);
	// 82337BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337BA8: 48B21BD1  bl 0x82e59778
	ctx.lr = 0x82337BAC;
	sub_82E59778(ctx, base);
	// 82337BAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337BB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82337BB4: C01F00A4  lfs f0, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82337BB8: EFA1F83A  fmadds f29, f1, f0, f31
	ctx.f[29].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82337BBC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82337BC0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82337BC4: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82337BC8: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82337BCC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82337BD0: D3C10064  stfs f30, 0x64(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82337BD4: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82337BD8: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82337BDC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82337BE0: 48B45061  bl 0x82e7cc40
	ctx.lr = 0x82337BE4;
	sub_82E7CC40(ctx, base);
	// 82337BE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337BE8: 38E000DC  li r7, 0xdc
	ctx.r[7].s64 = 220;
	// 82337BEC: 3BEB14A8  addi r31, r11, 0x14a8
	ctx.r[31].s64 = ctx.r[11].s64 + 5288;
	// 82337BF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82337BF4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82337BF8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82337BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337C00: 4BFDF3D9  bl 0x82316fd8
	ctx.lr = 0x82337C04;
	sub_82316FD8(ctx, base);
	// 82337C04: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337C08: EDBD0828  fsubs f13, f29, f1
	ctx.f[13].f64 = (((ctx.f[29].f64 - ctx.f[1].f64) as f32) as f64);
	// 82337C0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337C10: C00BCEE4  lfs f0, -0x311c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82337C14: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82337C18: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82337C1C: 40980040  bge cr6, 0x82337c5c
	if !ctx.cr[6].lt {
	pc = 0x82337C5C; continue 'dispatch;
	}
	// 82337C20: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82337C24: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82337C28: D3C10074  stfs f30, 0x74(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82337C2C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82337C30: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82337C34: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82337C38: 48B45009  bl 0x82e7cc40
	ctx.lr = 0x82337C3C;
	sub_82E7CC40(ctx, base);
	// 82337C3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82337C40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337C44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82337C48: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82337C4C: 38E000DF  li r7, 0xdf
	ctx.r[7].s64 = 223;
	// 82337C50: 4BFDF389  bl 0x82316fd8
	ctx.lr = 0x82337C54;
	sub_82316FD8(ctx, base);
	// 82337C54: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82337C58: 48000008  b 0x82337c60
	pc = 0x82337C60; continue 'dispatch;
	// 82337C5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82337C60: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82337C64: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82337C68: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82337C6C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82337C70: 48E7054C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82337C78 size=40
    let mut pc: u32 = 0x82337C78;
    'dispatch: loop {
        match pc {
            0x82337C78 => {
    //   block [0x82337C78..0x82337CA0)
	// 82337C78: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82337C7C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82337C80: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82337C84: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82337CA0 size=28
    let mut pc: u32 = 0x82337CA0;
    'dispatch: loop {
        match pc {
            0x82337CA0 => {
    //   block [0x82337CA0..0x82337CBC)
	// 82337CA0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82337CA4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82337CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82337CC0 size=196
    let mut pc: u32 = 0x82337CC0;
    'dispatch: loop {
        match pc {
            0x82337CC0 => {
    //   block [0x82337CC0..0x82337D84)
	// 82337CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82337CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82337CD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337CD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82337CDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82337CE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82337CE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82337CE8: 4BF88C51  bl 0x822c0938
	ctx.lr = 0x82337CEC;
	sub_822C0938(ctx, base);
	// 82337CEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82337CF0: 41820028  beq 0x82337d18
	if ctx.cr[0].eq {
	pc = 0x82337D18; continue 'dispatch;
	}
	// 82337CF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337CF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82337CFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82337D00: 392B1498  addi r9, r11, 0x1498
	ctx.r[9].s64 = ctx.r[11].s64 + 5272;
	// 82337D04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82337D08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82337D0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82337D10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82337D14: 48000008  b 0x82337d1c
	pc = 0x82337D1C; continue 'dispatch;
	// 82337D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82337D1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82337D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82337D24: 409A0044  bne cr6, 0x82337d68
	if !ctx.cr[6].eq {
	pc = 0x82337D68; continue 'dispatch;
	}
	// 82337D28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82337D2C: 419A001C  beq cr6, 0x82337d48
	if ctx.cr[6].eq {
	pc = 0x82337D48; continue 'dispatch;
	}
	// 82337D30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337D34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82337D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337D3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337D40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82337D44: 4E800421  bctrl
	ctx.lr = 0x82337D48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82337D48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337D4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82337D50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337D54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82337D58: 816B8D5C  lwz r11, -0x72a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29348 as u32) ) } as u64;
	// 82337D5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82337D60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82337D64: 4BF8829D  bl 0x822c0000
	ctx.lr = 0x82337D68;
	sub_822C0000(ctx, base);
	// 82337D68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337D6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82337D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82337D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82337D78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82337D7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82337D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82337D88 size=124
    let mut pc: u32 = 0x82337D88;
    'dispatch: loop {
        match pc {
            0x82337D88 => {
    //   block [0x82337D88..0x82337E04)
	// 82337D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337D8C: 48E703E1  bl 0x831a816c
	ctx.lr = 0x82337D90;
	sub_831A8130(ctx, base);
	// 82337D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337D94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337D98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82337D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82337DA0: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82337DA4: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82337DA8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82337DAC: 48ABA63D  bl 0x82df23e8
	ctx.lr = 0x82337DB0;
	sub_82DF23E8(ctx, base);
	// 82337DB0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82337DB4: 4182001C  beq 0x82337dd0
	if ctx.cr[0].eq {
	pc = 0x82337DD0; continue 'dispatch;
	}
	// 82337DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82337DBC: 48B218C5  bl 0x82e59680
	ctx.lr = 0x82337DC0;
	sub_82E59680(ctx, base);
	// 82337DC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337DC4: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 82337DC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82337DCC: 48000008  b 0x82337dd4
	pc = 0x82337DD4; continue 'dispatch;
	// 82337DD0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82337DD4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82337DD8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82337DDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337DE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337DE4: 4BFFFEDD  bl 0x82337cc0
	ctx.lr = 0x82337DE8;
	sub_82337CC0(ctx, base);
	// 82337DE8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82337DEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82337DF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337DF4: 4BF8820D  bl 0x822c0000
	ctx.lr = 0x82337DF8;
	sub_822C0000(ctx, base);
	// 82337DF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82337DFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82337E00: 48E703BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337E08 size=452
    let mut pc: u32 = 0x82337E08;
    'dispatch: loop {
        match pc {
            0x82337E08 => {
    //   block [0x82337E08..0x82337FCC)
	// 82337E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82337E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82337E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82337E18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337E1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337E20: 486D8A51  bl 0x82a10870
	ctx.lr = 0x82337E24;
	sub_82A10870(ctx, base);
	// 82337E24: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82337E28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337E2C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82337E30: 419A0180  beq cr6, 0x82337fb0
	if ctx.cr[6].eq {
	pc = 0x82337FB0; continue 'dispatch;
	}
	// 82337E34: 4BFDADD5  bl 0x82312c08
	ctx.lr = 0x82337E38;
	sub_82312C08(ctx, base);
	// 82337E38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82337E3C: 41820174  beq 0x82337fb0
	if ctx.cr[0].eq {
	pc = 0x82337FB0; continue 'dispatch;
	}
	// 82337E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337E44: 4BFDF19D  bl 0x82316fe0
	ctx.lr = 0x82337E48;
	sub_82316FE0(ctx, base);
	// 82337E48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337E4C: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82337E50: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82337E54: 4098015C  bge cr6, 0x82337fb0
	if !ctx.cr[6].lt {
	pc = 0x82337FB0; continue 'dispatch;
	}
	// 82337E58: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82337E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337E60: 388B6910  addi r4, r11, 0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + 26896;
	// 82337E64: 4BFE1CC5  bl 0x82319b28
	ctx.lr = 0x82337E68;
	sub_82319B28(ctx, base);
	// 82337E68: 897F00A8  lbz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82337E6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82337E70: 418200C8  beq 0x82337f38
	if ctx.cr[0].eq {
	pc = 0x82337F38; continue 'dispatch;
	}
	// 82337E74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82337E78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82337E7C: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82337E80: 4BFFFF09  bl 0x82337d88
	ctx.lr = 0x82337E84;
	sub_82337D88(ctx, base);
	// 82337E84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82337E88: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82337E8C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82337E90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82337E94: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82337E98: 419A0024  beq cr6, 0x82337ebc
	if ctx.cr[6].eq {
	pc = 0x82337EBC; continue 'dispatch;
	}
	// 82337E9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82337EA0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82337EA4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82337EA8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82337EAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82337EB0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82337EB4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82337EB8: 4082FFE8  bne 0x82337ea0
	if !ctx.cr[0].eq {
	pc = 0x82337EA0; continue 'dispatch;
	}
	// 82337EBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82337EC0: 80DF00B0  lwz r6, 0xb0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82337EC4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82337EC8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82337ECC: 388A14A8  addi r4, r10, 0x14a8
	ctx.r[4].s64 = ctx.r[10].s64 + 5288;
	// 82337ED0: 38A0006A  li r5, 0x6a
	ctx.r[5].s64 = 106;
	// 82337ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337ED8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337EDC: 4BFCFFAD  bl 0x82307e88
	ctx.lr = 0x82337EE0;
	sub_82307E88(ctx, base);
	// 82337EE0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82337EE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82337EE8: 419A0008  beq cr6, 0x82337ef0
	if ctx.cr[6].eq {
	pc = 0x82337EF0; continue 'dispatch;
	}
	// 82337EEC: 4BF889A5  bl 0x822c0890
	ctx.lr = 0x82337EF0;
	sub_822C0890(ctx, base);
	// 82337EF0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82337EF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337EF8: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 82337EFC: 48ABBB0D  bl 0x82df3a08
	ctx.lr = 0x82337F00;
	sub_82DF3A08(ctx, base);
	// 82337F00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82337F04: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82337F08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82337F0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82337F10: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82337F14: C02BD5B8  lfs f1, -0x2a48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10824 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337F18: 4BFC0DD9  bl 0x822f8cf0
	ctx.lr = 0x82337F1C;
	sub_822F8CF0(ctx, base);
	// 82337F1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82337F20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82337F24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337F28: 4BFCFA11  bl 0x82307938
	ctx.lr = 0x82337F2C;
	sub_82307938(ctx, base);
	// 82337F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337F30: 48ABB4F9  bl 0x82df3428
	ctx.lr = 0x82337F34;
	sub_82DF3428(ctx, base);
	// 82337F34: 4800004C  b 0x82337f80
	pc = 0x82337F80; continue 'dispatch;
	// 82337F38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82337F3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82337F40: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82337F44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82337F48: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82337F4C: 4BFDAD65  bl 0x82312cb0
	ctx.lr = 0x82337F50;
	sub_82312CB0(ctx, base);
	// 82337F50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82337F54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82337F58: 4BFCDB19  bl 0x82305a70
	ctx.lr = 0x82337F5C;
	sub_82305A70(ctx, base);
	// 82337F5C: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82337F60: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82337F64: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82337F68: 13DF58C7  vcmpequd (lvx128) v30, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82337FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82337FD0 size=728
    let mut pc: u32 = 0x82337FD0;
    'dispatch: loop {
        match pc {
            0x82337FD0 => {
    //   block [0x82337FD0..0x823382A8)
	// 82337FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82337FD4: 48E70199  bl 0x831a816c
	ctx.lr = 0x82337FD8;
	sub_831A8130(ctx, base);
	// 82337FD8: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82337FDC: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82337FE0: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82337FE4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82337FE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82337FEC: 486D8885  bl 0x82a10870
	ctx.lr = 0x82337FF0;
	sub_82A10870(ctx, base);
	// 82337FF0: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82337FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82337FF8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82337FFC: 409A000C  bne cr6, 0x82338008
	if !ctx.cr[6].eq {
	pc = 0x82338008; continue 'dispatch;
	}
	// 82338000: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82338004: 48000290  b 0x82338294
	pc = 0x82338294; continue 'dispatch;
	// 82338008: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233800C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338010: 4BFCDA61  bl 0x82305a70
	ctx.lr = 0x82338014;
	sub_82305A70(ctx, base);
	// 82338014: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82338018: 3BBE0070  addi r29, r30, 0x70
	ctx.r[29].s64 = ctx.r[30].s64 + 112;
	// 8233801C: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82338020: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82338024: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82338028: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823382A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823382A8 size=292
    let mut pc: u32 = 0x823382A8;
    'dispatch: loop {
        match pc {
            0x823382A8 => {
    //   block [0x823382A8..0x823383CC)
	// 823382A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823382AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823382B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823382B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823382B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823382BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823382C0: 486D85B1  bl 0x82a10870
	ctx.lr = 0x823382C4;
	sub_82A10870(ctx, base);
	// 823382C4: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 823382C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823382CC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 823382D0: 419A00E0  beq cr6, 0x823383b0
	if ctx.cr[6].eq {
	pc = 0x823383B0; continue 'dispatch;
	}
	// 823382D4: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 823382D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823382DC: 4BFFF82D  bl 0x82337b08
	ctx.lr = 0x823382E0;
	sub_82337B08(ctx, base);
	// 823382E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823382E4: 418200CC  beq 0x823383b0
	if ctx.cr[0].eq {
	pc = 0x823383B0; continue 'dispatch;
	}
	// 823382E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823382EC: 4BFFFA9D  bl 0x82337d88
	ctx.lr = 0x823382F0;
	sub_82337D88(ctx, base);
	// 823382F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823382F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823382F8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823382FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82338300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82338304: 419A0024  beq cr6, 0x82338328
	if ctx.cr[6].eq {
	pc = 0x82338328; continue 'dispatch;
	}
	// 82338308: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8233830C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82338310: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82338314: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82338318: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233831C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82338320: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82338324: 4082FFE8  bne 0x8233830c
	if !ctx.cr[0].eq {
	pc = 0x8233830C; continue 'dispatch;
	}
	// 82338328: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233832C: 80DF00B0  lwz r6, 0xb0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82338330: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82338334: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82338338: 388A14A8  addi r4, r10, 0x14a8
	ctx.r[4].s64 = ctx.r[10].s64 + 5288;
	// 8233833C: 38A000C0  li r5, 0xc0
	ctx.r[5].s64 = 192;
	// 82338340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338344: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82338348: 4BFCFB41  bl 0x82307e88
	ctx.lr = 0x8233834C;
	sub_82307E88(ctx, base);
	// 8233834C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82338350: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82338354: 419A0008  beq cr6, 0x8233835c
	if ctx.cr[6].eq {
	pc = 0x8233835C; continue 'dispatch;
	}
	// 82338358: 4BF88539  bl 0x822c0890
	ctx.lr = 0x8233835C;
	sub_822C0890(ctx, base);
	// 8233835C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82338360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338364: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 82338368: 48ABB6A1  bl 0x82df3a08
	ctx.lr = 0x8233836C;
	sub_82DF3A08(ctx, base);
	// 8233836C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82338370: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82338374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82338378: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233837C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82338380: C02BD5B8  lfs f1, -0x2a48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10824 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82338384: 4BFC096D  bl 0x822f8cf0
	ctx.lr = 0x82338388;
	sub_822F8CF0(ctx, base);
	// 82338388: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233838C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82338390: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338394: 4BFCF5A5  bl 0x82307938
	ctx.lr = 0x82338398;
	sub_82307938(ctx, base);
	// 82338398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233839C: 48ABB08D  bl 0x82df3428
	ctx.lr = 0x823383A0;
	sub_82DF3428(ctx, base);
	// 823383A0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 823383A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823383A8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 823383AC: 48000008  b 0x823383b4
	pc = 0x823383B4; continue 'dispatch;
	// 823383B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823383B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823383B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823383BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823383C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823383C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823383C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823383D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823383D0 size=232
    let mut pc: u32 = 0x823383D0;
    'dispatch: loop {
        match pc {
            0x823383D0 => {
    //   block [0x823383D0..0x823384B8)
	// 823383D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823383D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823383D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823383DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823383E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823383E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823383E8: 486D8489  bl 0x82a10870
	ctx.lr = 0x823383EC;
	sub_82A10870(ctx, base);
	// 823383EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823383F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823383F4: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823383F8: 4BFF66F1  bl 0x8232eae8
	ctx.lr = 0x823383FC;
	sub_8232EAE8(ctx, base);
	// 823383FC: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82338400: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82338404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338408: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 8233840C: 48ABB5FD  bl 0x82df3a08
	ctx.lr = 0x82338410;
	sub_82DF3A08(ctx, base);
	// 82338410: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82338414: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82338418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233841C: 4BFCD11D  bl 0x82305538
	ctx.lr = 0x82338420;
	sub_82305538(ctx, base);
	// 82338420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338424: 48ABB005  bl 0x82df3428
	ctx.lr = 0x82338428;
	sub_82DF3428(ctx, base);
	// 82338428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233842C: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82338430: 897F009C  lbz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82338434: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82338438: 41820010  beq 0x82338448
	if ctx.cr[0].eq {
	pc = 0x82338448; continue 'dispatch;
	}
	// 8233843C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338440: 4BFE9581  bl 0x823219c0
	ctx.lr = 0x82338444;
	sub_823219C0(ctx, base);
	// 82338444: 907F0098  stw r3, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82338448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233844C: 4BFFF9BD  bl 0x82337e08
	ctx.lr = 0x82338450;
	sub_82337E08(ctx, base);
	// 82338450: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82338454: 4182001C  beq 0x82338470
	if ctx.cr[0].eq {
	pc = 0x82338470; continue 'dispatch;
	}
	// 82338458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233845C: 4BFFFB75  bl 0x82337fd0
	ctx.lr = 0x82338460;
	sub_82337FD0(ctx, base);
	// 82338460: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82338464: 4182000C  beq 0x82338470
	if ctx.cr[0].eq {
	pc = 0x82338470; continue 'dispatch;
	}
	// 82338468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233846C: 4BFFFE3D  bl 0x823382a8
	ctx.lr = 0x82338470;
	sub_823382A8(ctx, base);
	// 82338470: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82338474: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82338478: 419A0028  beq cr6, 0x823384a0
	if ctx.cr[6].eq {
	pc = 0x823384A0; continue 'dispatch;
	}
	// 8233847C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82338480: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338484: 808BB01C  lwz r4, -0x4fe4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20452 as u32) ) } as u64;
	// 82338488: 48ABB581  bl 0x82df3a08
	ctx.lr = 0x8233848C;
	sub_82DF3A08(ctx, base);
	// 8233848C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82338490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338494: 4BFCF445  bl 0x823078d8
	ctx.lr = 0x82338498;
	sub_823078D8(ctx, base);
	// 82338498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233849C: 48ABAF8D  bl 0x82df3428
	ctx.lr = 0x823384A0;
	sub_82DF3428(ctx, base);
	// 823384A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823384A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823384A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823384AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823384B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823384B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823384B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823384B8 size=100
    let mut pc: u32 = 0x823384B8;
    'dispatch: loop {
        match pc {
            0x823384B8 => {
    //   block [0x823384B8..0x8233851C)
	// 823384B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823384BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823384C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823384C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823384C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823384CC: 486D83A5  bl 0x82a10870
	ctx.lr = 0x823384D0;
	sub_82A10870(ctx, base);
	// 823384D0: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 823384D4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 823384D8: 41980028  blt cr6, 0x82338500
	if ctx.cr[6].lt {
	pc = 0x82338500; continue 'dispatch;
	}
	// 823384DC: 419A0018  beq cr6, 0x823384f4
	if ctx.cr[6].eq {
	pc = 0x823384F4; continue 'dispatch;
	}
	// 823384E0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823384E4: 40980024  bge cr6, 0x82338508
	if !ctx.cr[6].lt {
	pc = 0x82338508; continue 'dispatch;
	}
	// 823384E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823384EC: 4BFFFDBD  bl 0x823382a8
	ctx.lr = 0x823384F0;
	sub_823382A8(ctx, base);
	// 823384F0: 48000018  b 0x82338508
	pc = 0x82338508; continue 'dispatch;
	// 823384F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823384F8: 4BFFFAD9  bl 0x82337fd0
	ctx.lr = 0x823384FC;
	sub_82337FD0(ctx, base);
	// 823384FC: 4800000C  b 0x82338508
	pc = 0x82338508; continue 'dispatch;
	// 82338500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338504: 4BFFF905  bl 0x82337e08
	ctx.lr = 0x82338508;
	sub_82337E08(ctx, base);
	// 82338508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233850C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82338510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82338514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82338518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82338520 size=84
    let mut pc: u32 = 0x82338520;
    'dispatch: loop {
        match pc {
            0x82338520 => {
    //   block [0x82338520..0x82338574)
	// 82338520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82338524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233852C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82338530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82338534: 4BFF98A5  bl 0x82331dd8
	ctx.lr = 0x82338538;
	sub_82331DD8(ctx, base);
	// 82338538: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233853C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82338540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82338544: 3929151C  addi r9, r9, 0x151c
	ctx.r[9].s64 = ctx.r[9].s64 + 5404;
	// 82338548: 997F00A8  stb r11, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u8 ) };
	// 8233854C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338550: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82338554: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82338558: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8233855C: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82338560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82338564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82338568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233856C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82338570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82338578 size=124
    let mut pc: u32 = 0x82338578;
    'dispatch: loop {
        match pc {
            0x82338578 => {
    //   block [0x82338578..0x823385F4)
	// 82338578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233857C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82338584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82338588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233858C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82338590: 419A004C  beq cr6, 0x823385dc
	if ctx.cr[6].eq {
	pc = 0x823385DC; continue 'dispatch;
	}
	// 82338594: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82338598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233859C: 419A0018  beq cr6, 0x823385b4
	if ctx.cr[6].eq {
	pc = 0x823385B4; continue 'dispatch;
	}
	// 823385A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823385A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823385A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823385AC: 4E800421  bctrl
	ctx.lr = 0x823385B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823385B0: 4800000C  b 0x823385bc
	pc = 0x823385BC; continue 'dispatch;
	// 823385B4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823385B8: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 823385BC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823385C0: 388B8290  addi r4, r11, -0x7d70
	ctx.r[4].s64 = ctx.r[11].s64 + -32112;
	// 823385C4: 48E6FB35  bl 0x831a80f8
	ctx.lr = 0x823385C8;
	sub_831A80F8(ctx, base);
	// 823385C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823385CC: 41820010  beq 0x823385dc
	if ctx.cr[0].eq {
	pc = 0x823385DC; continue 'dispatch;
	}
	// 823385D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823385D4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 823385D8: 48000008  b 0x823385e0
	pc = 0x823385E0; continue 'dispatch;
	// 823385DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823385E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823385E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823385E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823385EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823385F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823385F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823385F8 size=352
    let mut pc: u32 = 0x823385F8;
    'dispatch: loop {
        match pc {
            0x823385F8 => {
    //   block [0x823385F8..0x82338758)
	// 823385F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823385FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82338604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82338608: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233860C: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82338758 size=120
    let mut pc: u32 = 0x82338758;
    'dispatch: loop {
        match pc {
            0x82338758 => {
    //   block [0x82338758..0x823387D0)
	// 82338758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82338764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82338768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233876C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82338770: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82338774: 4BFFFE05  bl 0x82338578
	ctx.lr = 0x82338778;
	sub_82338578(ctx, base);
	// 82338778: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8233877C: 40820030  bne 0x823387ac
	if !ctx.cr[0].eq {
	pc = 0x823387AC; continue 'dispatch;
	}
	// 82338780: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82338784: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82338788: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 8233878C: 394A1014  addi r10, r10, 0x1014
	ctx.r[10].s64 = ctx.r[10].s64 + 4116;
	// 82338790: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82338794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338798: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233879C: 4BF87865  bl 0x822c0000
	ctx.lr = 0x823387A0;
	sub_822C0000(ctx, base);
	// 823387A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823387A4: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 823387A8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823387AC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823387B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823387D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823387D0 size=204
    let mut pc: u32 = 0x823387D0;
    'dispatch: loop {
        match pc {
            0x823387D0 => {
    //   block [0x823387D0..0x8233889C)
	// 823387D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823387D4: 48E6F999  bl 0x831a816c
	ctx.lr = 0x823387D8;
	sub_831A8130(ctx, base);
	// 823387D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823387DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823387E0: 486D8091  bl 0x82a10870
	ctx.lr = 0x823387E4;
	sub_82A10870(ctx, base);
	// 823387E4: 809E006C  lwz r4, 0x6c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 823387E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823387EC: 4BFE84BD  bl 0x82320ca8
	ctx.lr = 0x823387F0;
	sub_82320CA8(ctx, base);
	// 823387F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823387F4: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 823387F8: 4BFE2261  bl 0x8231aa58
	ctx.lr = 0x823387FC;
	sub_8231AA58(ctx, base);
	// 823387FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338800: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82338804: 4BFCD35D  bl 0x82305b60
	ctx.lr = 0x82338808;
	sub_82305B60(ctx, base);
	// 82338808: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 8233880C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338810: 4BFDB4E1  bl 0x82313cf0
	ctx.lr = 0x82338814;
	sub_82313CF0(ctx, base);
	// 82338814: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82338818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233881C: 808BB410  lwz r4, -0x4bf0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19440 as u32) ) } as u64;
	// 82338820: 48ABB1E9  bl 0x82df3a08
	ctx.lr = 0x82338824;
	sub_82DF3A08(ctx, base);
	// 82338824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82338828: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233882C: 48B211A5  bl 0x82e599d0
	ctx.lr = 0x82338830;
	sub_82E599D0(ctx, base);
	// 82338830: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82338834: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82338838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233883C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82338840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82338844: 4E800421  bctrl
	ctx.lr = 0x82338848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82338848: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233884C: 48ABAA55  bl 0x82df32a0
	ctx.lr = 0x82338850;
	sub_82DF32A0(ctx, base);
	// 82338850: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82338854: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82338858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233885C: 419A000C  beq cr6, 0x82338868
	if ctx.cr[6].eq {
	pc = 0x82338868; continue 'dispatch;
	}
	// 82338860: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82338864: 4BF8802D  bl 0x822c0890
	ctx.lr = 0x82338868;
	sub_822C0890(ctx, base);
	// 82338868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233886C: 48ABABBD  bl 0x82df3428
	ctx.lr = 0x82338870;
	sub_82DF3428(ctx, base);
	// 82338870: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82338874: 41820018  beq 0x8233888c
	if ctx.cr[0].eq {
	pc = 0x8233888C; continue 'dispatch;
	}
	// 82338878: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 8233887C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82338880: 48122569  bl 0x8245ade8
	ctx.lr = 0x82338884;
	sub_8245ADE8(ctx, base);
	// 82338884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338888: 4BFDAC11  bl 0x82313498
	ctx.lr = 0x8233888C;
	sub_82313498(ctx, base);
	// 8233888C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338890: 48D8F251  bl 0x830c7ae0
	ctx.lr = 0x82338894;
	sub_830C7AE0(ctx, base);
	// 82338894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82338898: 48E6F924  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823388A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823388A0 size=184
    let mut pc: u32 = 0x823388A0;
    'dispatch: loop {
        match pc {
            0x823388A0 => {
    //   block [0x823388A0..0x82338958)
	// 823388A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823388A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823388A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823388AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823388B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823388B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823388B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823388BC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 823388C0: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 823388C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823388C8: 48159A41  bl 0x82492308
	ctx.lr = 0x823388CC;
	sub_82492308(ctx, base);
	// 823388CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823388D0: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823388D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823388D8: 419A0064  beq cr6, 0x8233893c
	if ctx.cr[6].eq {
	pc = 0x8233893C; continue 'dispatch;
	}
	// 823388DC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 823388E0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 823388E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823388E8: 419A0018  beq cr6, 0x82338900
	if ctx.cr[6].eq {
	pc = 0x82338900; continue 'dispatch;
	}
	// 823388EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823388F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823388F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823388F8: 4E800421  bctrl
	ctx.lr = 0x823388FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823388FC: 4800000C  b 0x82338908
	pc = 0x82338908; continue 'dispatch;
	// 82338900: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82338904: 386B869C  addi r3, r11, -0x7964
	ctx.r[3].s64 = ctx.r[11].s64 + -31076;
	// 82338908: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233890C: 388B8290  addi r4, r11, -0x7d70
	ctx.r[4].s64 = ctx.r[11].s64 + -32112;
	// 82338910: 48E6F7E9  bl 0x831a80f8
	ctx.lr = 0x82338914;
	sub_831A80F8(ctx, base);
	// 82338914: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82338918: 41820024  beq 0x8233893c
	if ctx.cr[0].eq {
	pc = 0x8233893C; continue 'dispatch;
	}
	// 8233891C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82338920: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82338924: 4BFFFE35  bl 0x82338758
	ctx.lr = 0x82338928;
	sub_82338758(ctx, base);
	// 82338928: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233892C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82338930: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82338958 size=1148
    let mut pc: u32 = 0x82338958;
    'dispatch: loop {
        match pc {
            0x82338958 => {
    //   block [0x82338958..0x82338DD4)
	// 82338958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233895C: 48E6F80D  bl 0x831a8168
	ctx.lr = 0x82338960;
	sub_831A8130(ctx, base);
	// 82338960: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82338964: 48E70109  bl 0x831a8a6c
	ctx.lr = 0x82338968;
	sub_831A8A40(ctx, base);
	// 82338968: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82338DD8 size=60
    let mut pc: u32 = 0x82338DD8;
    'dispatch: loop {
        match pc {
            0x82338DD8 => {
    //   block [0x82338DD8..0x82338E14)
	// 82338DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82338DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82338DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82338DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82338DEC: 4BFF8FED  bl 0x82331dd8
	ctx.lr = 0x82338DF0;
	sub_82331DD8(ctx, base);
	// 82338DF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82338DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338DF8: 396B15BC  addi r11, r11, 0x15bc
	ctx.r[11].s64 = ctx.r[11].s64 + 5564;
	// 82338DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82338E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82338E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82338E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82338E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82338E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82338E18 size=108
    let mut pc: u32 = 0x82338E18;
    'dispatch: loop {
        match pc {
            0x82338E18 => {
    //   block [0x82338E18..0x82338E84)
	// 82338E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82338E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82338E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82338E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82338E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82338E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82338E30: 486D7A41  bl 0x82a10870
	ctx.lr = 0x82338E34;
	sub_82A10870(ctx, base);
	// 82338E34: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82338E38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82338E3C: 4BFDAEB5  bl 0x82313cf0
	ctx.lr = 0x82338E40;
	sub_82313CF0(ctx, base);
	// 82338E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338E44: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82338E48: 4BFE7E61  bl 0x82320ca8
	ctx.lr = 0x82338E4C;
	sub_82320CA8(ctx, base);
	// 82338E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338E50: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82338E54: 4BFE1C05  bl 0x8231aa58
	ctx.lr = 0x82338E58;
	sub_8231AA58(ctx, base);
	// 82338E58: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82338E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338E60: 4BFDAE91  bl 0x82313cf0
	ctx.lr = 0x82338E64;
	sub_82313CF0(ctx, base);
	// 82338E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82338E68: 48D8EC79  bl 0x830c7ae0
	ctx.lr = 0x82338E6C;
	sub_830C7AE0(ctx, base);
	// 82338E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82338E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82338E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82338E78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82338E7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82338E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82338E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82338E88 size=496
    let mut pc: u32 = 0x82338E88;
    'dispatch: loop {
        match pc {
            0x82338E88 => {
    //   block [0x82338E88..0x82339078)
	// 82338E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82338E8C: 48E6F2E1  bl 0x831a816c
	ctx.lr = 0x82338E90;
	sub_831A8130(ctx, base);
	// 82338E90: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82338E94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82338E98: 486D79D9  bl 0x82a10870
	ctx.lr = 0x82338E9C;
	sub_82A10870(ctx, base);
	// 82338E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82338EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338EA4: 48D8EC3D  bl 0x830c7ae0
	ctx.lr = 0x82338EA8;
	sub_830C7AE0(ctx, base);
	// 82338EA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82338EAC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82338EB0: 4BFCEE11  bl 0x82307cc0
	ctx.lr = 0x82338EB4;
	sub_82307CC0(ctx, base);
	// 82338EB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82338EB8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82338EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338EC0: 388A0D44  addi r4, r10, 0xd44
	ctx.r[4].s64 = ctx.r[10].s64 + 3396;
	// 82338EC4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82338EC8: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82338ECC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82338ED0: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82338ED4: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82338ED8: C16B000C  lfs f11, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82338EDC: FD806050  fneg f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82338EE0: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 82338EE4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82338EE8: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82338EEC: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82338EF0: D161006C  stfs f11, 0x6c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82338EF4: 48ABAB15  bl 0x82df3a08
	ctx.lr = 0x82338EF8;
	sub_82DF3A08(ctx, base);
	// 82338EF8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82338EFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82338F00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82338F04: 4BFFF99D  bl 0x823388a0
	ctx.lr = 0x82338F08;
	sub_823388A0(ctx, base);
	// 82338F08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82338F0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82338F10: 48ABA519  bl 0x82df3428
	ctx.lr = 0x82338F14;
	sub_82DF3428(ctx, base);
	// 82338F14: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82338F18: 41820044  beq 0x82338f5c
	if ctx.cr[0].eq {
	pc = 0x82338F5C; continue 'dispatch;
	}
	// 82338F1C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82338F20: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82338F24: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82338F28: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339078 size=60
    let mut pc: u32 = 0x82339078;
    'dispatch: loop {
        match pc {
            0x82339078 => {
    //   block [0x82339078..0x823390B4)
	// 82339078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233908C: 4BFF8D4D  bl 0x82331dd8
	ctx.lr = 0x82339090;
	sub_82331DD8(ctx, base);
	// 82339090: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339098: 396B1650  addi r11, r11, 0x1650
	ctx.r[11].s64 = ctx.r[11].s64 + 5712;
	// 8233909C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823390A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823390A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823390A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823390AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823390B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823390B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823390B8 size=440
    let mut pc: u32 = 0x823390B8;
    'dispatch: loop {
        match pc {
            0x823390B8 => {
    //   block [0x823390B8..0x82339270)
	// 823390B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823390BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823390C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823390C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823390C8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823390CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823390D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823390D4: 486D779D  bl 0x82a10870
	ctx.lr = 0x823390D8;
	sub_82A10870(ctx, base);
	// 823390D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823390DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823390E0: 48D8EA01  bl 0x830c7ae0
	ctx.lr = 0x823390E4;
	sub_830C7AE0(ctx, base);
	// 823390E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823390E8: 4BFD9B21  bl 0x82312c08
	ctx.lr = 0x823390EC;
	sub_82312C08(ctx, base);
	// 823390EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823390F0: 418200D0  beq 0x823391c0
	if ctx.cr[0].eq {
	pc = 0x823391C0; continue 'dispatch;
	}
	// 823390F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823390F8: 4BFCC3B9  bl 0x823054b0
	ctx.lr = 0x823390FC;
	sub_823054B0(ctx, base);
	// 823390FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339100: C00B0A98  lfs f0, 0xa98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82339104: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82339108: 409900B8  ble cr6, 0x823391c0
	if !ctx.cr[6].gt {
	pc = 0x823391C0; continue 'dispatch;
	}
	// 8233910C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82339110: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82339114: 808BB108  lwz r4, -0x4ef8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20216 as u32) ) } as u64;
	// 82339118: 48ABA8F1  bl 0x82df3a08
	ctx.lr = 0x8233911C;
	sub_82DF3A08(ctx, base);
	// 8233911C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82339120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339124: 4BFCE7B5  bl 0x823078d8
	ctx.lr = 0x82339128;
	sub_823078D8(ctx, base);
	// 82339128: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233912C: 48ABA2FD  bl 0x82df3428
	ctx.lr = 0x82339130;
	sub_82DF3428(ctx, base);
	// 82339130: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339138: C02B1678  lfs f1, 0x1678(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5752 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233913C: 4BFCC5AD  bl 0x823056e8
	ctx.lr = 0x82339140;
	sub_823056E8(ctx, base);
	// 82339140: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339144: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82339148: 388B12FC  addi r4, r11, 0x12fc
	ctx.r[4].s64 = ctx.r[11].s64 + 4860;
	// 8233914C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82339150: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82339154: 48ABA8B5  bl 0x82df3a08
	ctx.lr = 0x82339158;
	sub_82DF3A08(ctx, base);
	// 82339158: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 8233915C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82339160: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82339164: 4BFF36DD  bl 0x8232c840
	ctx.lr = 0x82339168;
	sub_8232C840(ctx, base);
	// 82339168: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233916C: 4BFE42CD  bl 0x8231d438
	ctx.lr = 0x82339170;
	sub_8231D438(ctx, base);
	// 82339170: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82339174: 48ABA2B5  bl 0x82df3428
	ctx.lr = 0x82339178;
	sub_82DF3428(ctx, base);
	// 82339178: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8233917C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339180: 4BFDAB61  bl 0x82313ce0
	ctx.lr = 0x82339184;
	sub_82313CE0(ctx, base);
	// 82339184: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82339188: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233918C: 41820010  beq 0x8233919c
	if ctx.cr[0].eq {
	pc = 0x8233919C; continue 'dispatch;
	}
	// 82339190: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82339194: 808BB418  lwz r4, -0x4be8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19432 as u32) ) } as u64;
	// 82339198: 4800000C  b 0x823391a4
	pc = 0x823391A4; continue 'dispatch;
	// 8233919C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823391A0: 808BB40C  lwz r4, -0x4bf4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19444 as u32) ) } as u64;
	// 823391A4: 48ABA865  bl 0x82df3a08
	ctx.lr = 0x823391A8;
	sub_82DF3A08(ctx, base);
	// 823391A8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823391AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823391B0: 4BFCC3E1  bl 0x82305590
	ctx.lr = 0x823391B4;
	sub_82305590(ctx, base);
	// 823391B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823391B8: 48ABA271  bl 0x82df3428
	ctx.lr = 0x823391BC;
	sub_82DF3428(ctx, base);
	// 823391BC: 48000098  b 0x82339254
	pc = 0x82339254; continue 'dispatch;
	// 823391C0: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 823391C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823391C8: 4BFDAB19  bl 0x82313ce0
	ctx.lr = 0x823391CC;
	sub_82313CE0(ctx, base);
	// 823391CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823391D0: 4082007C  bne 0x8233924c
	if !ctx.cr[0].eq {
	pc = 0x8233924C; continue 'dispatch;
	}
	// 823391D4: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 823391D8: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823391DC: 48121C0D  bl 0x8245ade8
	ctx.lr = 0x823391E0;
	sub_8245ADE8(ctx, base);
	// 823391E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823391E4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823391E8: 48B20579  bl 0x82e59760
	ctx.lr = 0x823391EC;
	sub_82E59760(ctx, base);
	// 823391EC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 823391F0: 4099005C  ble cr6, 0x8233924c
	if !ctx.cr[6].gt {
	pc = 0x8233924C; continue 'dispatch;
	}
	// 823391F4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823391F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823391FC: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 82339200: 48ABA809  bl 0x82df3a08
	ctx.lr = 0x82339204;
	sub_82DF3A08(ctx, base);
	// 82339204: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82339208: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233920C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82339210: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82339214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82339218: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233921C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82339220: 48B25871  bl 0x82e5ea90
	ctx.lr = 0x82339224;
	sub_82E5EA90(ctx, base);
	// 82339224: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82339228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233922C: 419A0008  beq cr6, 0x82339234
	if ctx.cr[6].eq {
	pc = 0x82339234; continue 'dispatch;
	}
	// 82339230: 4BF87661  bl 0x822c0890
	ctx.lr = 0x82339234;
	sub_822C0890(ctx, base);
	// 82339234: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 82339238: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233923C: 48121BAD  bl 0x8245ade8
	ctx.lr = 0x82339240;
	sub_8245ADE8(ctx, base);
	// 82339240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339244: 4BFDA255  bl 0x82313498
	ctx.lr = 0x82339248;
	sub_82313498(ctx, base);
	// 82339248: 4800000C  b 0x82339254
	pc = 0x82339254; continue 'dispatch;
	// 8233924C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339250: 48D8E891  bl 0x830c7ae0
	ctx.lr = 0x82339254;
	sub_830C7AE0(ctx, base);
	// 82339254: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82339258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233925C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339260: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82339264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233926C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339270 size=68
    let mut pc: u32 = 0x82339270;
    'dispatch: loop {
        match pc {
            0x82339270 => {
    //   block [0x82339270..0x823392B4)
	// 82339270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233927C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339284: 486D75ED  bl 0x82a10870
	ctx.lr = 0x82339288;
	sub_82A10870(ctx, base);
	// 82339288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233928C: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82339290: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82339294: 4E800421  bctrl
	ctx.lr = 0x82339298;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82339298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233929C: 4BFFF6BD  bl 0x82338958
	ctx.lr = 0x823392A0;
	sub_82338958(ctx, base);
	// 823392A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823392A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823392A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823392AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823392B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823392B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823392B8 size=4
    let mut pc: u32 = 0x823392B8;
    'dispatch: loop {
        match pc {
            0x823392B8 => {
    //   block [0x823392B8..0x823392BC)
	// 823392B8: 4BFFF518  b 0x823387d0
	sub_823387D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823392C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823392C0 size=116
    let mut pc: u32 = 0x823392C0;
    'dispatch: loop {
        match pc {
            0x823392C0 => {
    //   block [0x823392C0..0x82339334)
	// 823392C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823392C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823392C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823392CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823392D0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823392D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823392D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823392DC: 486D7595  bl 0x82a10870
	ctx.lr = 0x823392E0;
	sub_82A10870(ctx, base);
	// 823392E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823392E4: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 823392E8: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823392EC: 48121AFD  bl 0x8245ade8
	ctx.lr = 0x823392F0;
	sub_8245ADE8(ctx, base);
	// 823392F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823392F4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823392F8: 48B20469  bl 0x82e59760
	ctx.lr = 0x823392FC;
	sub_82E59760(ctx, base);
	// 823392FC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82339300: 40990010  ble cr6, 0x82339310
	if !ctx.cr[6].gt {
	pc = 0x82339310; continue 'dispatch;
	}
	// 82339304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339308: 4BFE4CB9  bl 0x8231dfc0
	ctx.lr = 0x8233930C;
	sub_8231DFC0(ctx, base);
	// 8233930C: 4800000C  b 0x82339318
	pc = 0x82339318; continue 'dispatch;
	// 82339310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339314: 4BFFF2E5  bl 0x823385f8
	ctx.lr = 0x82339318;
	sub_823385F8(ctx, base);
	// 82339318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233931C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339324: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82339328: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233932C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339338 size=60
    let mut pc: u32 = 0x82339338;
    'dispatch: loop {
        match pc {
            0x82339338 => {
    //   block [0x82339338..0x82339374)
	// 82339338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233934C: 4BFFFA8D  bl 0x82338dd8
	ctx.lr = 0x82339350;
	sub_82338DD8(ctx, base);
	// 82339350: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339358: 396B1680  addi r11, r11, 0x1680
	ctx.r[11].s64 = ctx.r[11].s64 + 5760;
	// 8233935C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82339364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233936C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339378 size=68
    let mut pc: u32 = 0x82339378;
    'dispatch: loop {
        match pc {
            0x82339378 => {
    //   block [0x82339378..0x823393BC)
	// 82339378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233937C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233938C: 486D74E5  bl 0x82a10870
	ctx.lr = 0x82339390;
	sub_82A10870(ctx, base);
	// 82339390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339394: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82339398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233939C: 4E800421  bctrl
	ctx.lr = 0x823393A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823393A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823393A4: 4BFFFAE5  bl 0x82338e88
	ctx.lr = 0x823393A8;
	sub_82338E88(ctx, base);
	// 823393A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823393AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823393B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823393B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823393B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823393C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823393C0 size=4
    let mut pc: u32 = 0x823393C0;
    'dispatch: loop {
        match pc {
            0x823393C0 => {
    //   block [0x823393C0..0x823393C4)
	// 823393C0: 4BFFFA58  b 0x82338e18
	sub_82338E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823393C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823393C8 size=116
    let mut pc: u32 = 0x823393C8;
    'dispatch: loop {
        match pc {
            0x823393C8 => {
    //   block [0x823393C8..0x8233943C)
	// 823393C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823393CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823393D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823393D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823393D8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823393DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823393E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823393E4: 486D748D  bl 0x82a10870
	ctx.lr = 0x823393E8;
	sub_82A10870(ctx, base);
	// 823393E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823393EC: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 823393F0: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823393F4: 481219F5  bl 0x8245ade8
	ctx.lr = 0x823393F8;
	sub_8245ADE8(ctx, base);
	// 823393F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823393FC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82339400: 48B20361  bl 0x82e59760
	ctx.lr = 0x82339404;
	sub_82E59760(ctx, base);
	// 82339404: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82339408: 40990010  ble cr6, 0x82339418
	if !ctx.cr[6].gt {
	pc = 0x82339418; continue 'dispatch;
	}
	// 8233940C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339410: 4BFE4BB1  bl 0x8231dfc0
	ctx.lr = 0x82339414;
	sub_8231DFC0(ctx, base);
	// 82339414: 4800000C  b 0x82339420
	pc = 0x82339420; continue 'dispatch;
	// 82339418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233941C: 4BFFFC9D  bl 0x823390b8
	ctx.lr = 0x82339420;
	sub_823390B8(ctx, base);
	// 82339420: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233942C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82339430: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339440 size=60
    let mut pc: u32 = 0x82339440;
    'dispatch: loop {
        match pc {
            0x82339440 => {
    //   block [0x82339440..0x8233947C)
	// 82339440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233944C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339454: 4BFFFC25  bl 0x82339078
	ctx.lr = 0x82339458;
	sub_82339078(ctx, base);
	// 82339458: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233945C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339460: 396B16AC  addi r11, r11, 0x16ac
	ctx.r[11].s64 = ctx.r[11].s64 + 5804;
	// 82339464: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339480 size=112
    let mut pc: u32 = 0x82339480;
    'dispatch: loop {
        match pc {
            0x82339480 => {
    //   block [0x82339480..0x823394F0)
	// 82339480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233948C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339498: 486D73D9  bl 0x82a10870
	ctx.lr = 0x8233949C;
	sub_82A10870(ctx, base);
	// 8233949C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823394A0: 4BFCE689  bl 0x82307b28
	ctx.lr = 0x823394A4;
	sub_82307B28(ctx, base);
	// 823394A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823394A8: 4BFE8519  bl 0x823219c0
	ctx.lr = 0x823394AC;
	sub_823219C0(ctx, base);
	// 823394AC: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 823394B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823394B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823394B8: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823394BC: 4BFF562D  bl 0x8232eae8
	ctx.lr = 0x823394C0;
	sub_8232EAE8(ctx, base);
	// 823394C0: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 823394C4: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 823394C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823394CC: 4BFDA81D  bl 0x82313ce8
	ctx.lr = 0x823394D0;
	sub_82313CE8(ctx, base);
	// 823394D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823394D4: 48D8E60D  bl 0x830c7ae0
	ctx.lr = 0x823394D8;
	sub_830C7AE0(ctx, base);
	// 823394D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823394DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823394E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823394E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823394E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823394EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823394F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823394F0 size=136
    let mut pc: u32 = 0x823394F0;
    'dispatch: loop {
        match pc {
            0x823394F0 => {
    //   block [0x823394F0..0x82339578)
	// 823394F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823394F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823394F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823394FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339504: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339508: 486D7369  bl 0x82a10870
	ctx.lr = 0x8233950C;
	sub_82A10870(ctx, base);
	// 8233950C: 809E006C  lwz r4, 0x6c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82339510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339514: 4BFE7795  bl 0x82320ca8
	ctx.lr = 0x82339518;
	sub_82320CA8(ctx, base);
	// 82339518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233951C: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82339520: 4BFE1539  bl 0x8231aa58
	ctx.lr = 0x82339524;
	sub_8231AA58(ctx, base);
	// 82339524: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82339528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233952C: 4BFDA7B5  bl 0x82313ce0
	ctx.lr = 0x82339530;
	sub_82313CE0(ctx, base);
	// 82339530: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82339534: 40820018  bne 0x8233954c
	if !ctx.cr[0].eq {
	pc = 0x8233954C; continue 'dispatch;
	}
	// 82339538: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 8233953C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82339540: 481218A9  bl 0x8245ade8
	ctx.lr = 0x82339544;
	sub_8245ADE8(ctx, base);
	// 82339544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339548: 4BFD9F51  bl 0x82313498
	ctx.lr = 0x8233954C;
	sub_82313498(ctx, base);
	// 8233954C: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82339550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339554: 4BFDA79D  bl 0x82313cf0
	ctx.lr = 0x82339558;
	sub_82313CF0(ctx, base);
	// 82339558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233955C: 48D8E585  bl 0x830c7ae0
	ctx.lr = 0x82339560;
	sub_830C7AE0(ctx, base);
	// 82339560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233956C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339578 size=344
    let mut pc: u32 = 0x82339578;
    'dispatch: loop {
        match pc {
            0x82339578 => {
    //   block [0x82339578..0x823396D0)
	// 82339578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82339584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339588: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233958C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339594: 486D72DD  bl 0x82a10870
	ctx.lr = 0x82339598;
	sub_82A10870(ctx, base);
	// 82339598: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233959C: 4BFCBE8D  bl 0x82305428
	ctx.lr = 0x823395A0;
	sub_82305428(ctx, base);
	// 823395A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823395A4: 41820048  beq 0x823395ec
	if ctx.cr[0].eq {
	pc = 0x823395EC; continue 'dispatch;
	}
	// 823395A8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823395AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823395B0: 808BB38C  lwz r4, -0x4c74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19572 as u32) ) } as u64;
	// 823395B4: 48ABA455  bl 0x82df3a08
	ctx.lr = 0x823395B8;
	sub_82DF3A08(ctx, base);
	// 823395B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823395BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823395C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823395C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823395C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823395CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823395D0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823395D4: 48B254BD  bl 0x82e5ea90
	ctx.lr = 0x823395D8;
	sub_82E5EA90(ctx, base);
	// 823395D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823395DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823395E0: 419A00D4  beq cr6, 0x823396b4
	if ctx.cr[6].eq {
	pc = 0x823396B4; continue 'dispatch;
	}
	// 823395E4: 4BF872AD  bl 0x822c0890
	ctx.lr = 0x823395E8;
	sub_822C0890(ctx, base);
	// 823395E8: 480000CC  b 0x823396b4
	pc = 0x823396B4; continue 'dispatch;
	// 823395EC: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 823395F0: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823395F4: 481217F5  bl 0x8245ade8
	ctx.lr = 0x823395F8;
	sub_8245ADE8(ctx, base);
	// 823395F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823395FC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82339600: 48B20161  bl 0x82e59760
	ctx.lr = 0x82339604;
	sub_82E59760(ctx, base);
	// 82339604: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82339608: 40990084  ble cr6, 0x8233968c
	if !ctx.cr[6].gt {
	pc = 0x8233968C; continue 'dispatch;
	}
	// 8233960C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339610: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82339614: 4BFE1445  bl 0x8231aa58
	ctx.lr = 0x82339618;
	sub_8231AA58(ctx, base);
	// 82339618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233961C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82339620: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82339624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339628: 4BFE2D89  bl 0x8231c3b0
	ctx.lr = 0x8233962C;
	sub_8231C3B0(ctx, base);
	// 8233962C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82339630: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82339634: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82339638: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823396D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823396D0 size=60
    let mut pc: u32 = 0x823396D0;
    'dispatch: loop {
        match pc {
            0x823396D0 => {
    //   block [0x823396D0..0x8233970C)
	// 823396D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823396D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823396D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823396DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823396E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823396E4: 4BFF86F5  bl 0x82331dd8
	ctx.lr = 0x823396E8;
	sub_82331DD8(ctx, base);
	// 823396E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823396EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823396F0: 396B16D8  addi r11, r11, 0x16d8
	ctx.r[11].s64 = ctx.r[11].s64 + 5848;
	// 823396F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823396F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823396FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339710 size=112
    let mut pc: u32 = 0x82339710;
    'dispatch: loop {
        match pc {
            0x82339710 => {
    //   block [0x82339710..0x82339780)
	// 82339710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233971C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339728: 486D7149  bl 0x82a10870
	ctx.lr = 0x8233972C;
	sub_82A10870(ctx, base);
	// 8233972C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339730: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339734: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82339738: 4BFF53B1  bl 0x8232eae8
	ctx.lr = 0x8233973C;
	sub_8232EAE8(ctx, base);
	// 8233973C: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82339740: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82339744: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82339748: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 8233974C: 48ABA2BD  bl 0x82df3a08
	ctx.lr = 0x82339750;
	sub_82DF3A08(ctx, base);
	// 82339750: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82339754: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82339758: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233975C: 4BFCBDE5  bl 0x82305540
	ctx.lr = 0x82339760;
	sub_82305540(ctx, base);
	// 82339760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82339764: 48AB9CC5  bl 0x82df3428
	ctx.lr = 0x82339768;
	sub_82DF3428(ctx, base);
	// 82339768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233976C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339774: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233977C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339780 size=100
    let mut pc: u32 = 0x82339780;
    'dispatch: loop {
        match pc {
            0x82339780 => {
    //   block [0x82339780..0x823397E4)
	// 82339780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233978C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339798: 486D70D9  bl 0x82a10870
	ctx.lr = 0x8233979C;
	sub_82A10870(ctx, base);
	// 8233979C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 823397A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823397A4: 4BFE12B5  bl 0x8231aa58
	ctx.lr = 0x823397A8;
	sub_8231AA58(ctx, base);
	// 823397A8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823397AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823397B0: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 823397B4: 48ABA255  bl 0x82df3a08
	ctx.lr = 0x823397B8;
	sub_82DF3A08(ctx, base);
	// 823397B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823397BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823397C0: 4BFCBD89  bl 0x82305548
	ctx.lr = 0x823397C4;
	sub_82305548(ctx, base);
	// 823397C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823397C8: 48AB9C61  bl 0x82df3428
	ctx.lr = 0x823397CC;
	sub_82DF3428(ctx, base);
	// 823397CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823397D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823397D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823397D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823397DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823397E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823397E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823397E8 size=196
    let mut pc: u32 = 0x823397E8;
    'dispatch: loop {
        match pc {
            0x823397E8 => {
    //   block [0x823397E8..0x823398AC)
	// 823397E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823397EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823397F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823397F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823397F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823397FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82339804: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82339808: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233980C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339810: 4BF87129  bl 0x822c0938
	ctx.lr = 0x82339814;
	sub_822C0938(ctx, base);
	// 82339814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82339818: 41820028  beq 0x82339840
	if ctx.cr[0].eq {
	pc = 0x82339840; continue 'dispatch;
	}
	// 8233981C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339820: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82339824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82339828: 392B1704  addi r9, r11, 0x1704
	ctx.r[9].s64 = ctx.r[11].s64 + 5892;
	// 8233982C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82339830: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82339834: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82339838: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233983C: 48000008  b 0x82339844
	pc = 0x82339844; continue 'dispatch;
	// 82339840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82339844: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233984C: 409A0044  bne cr6, 0x82339890
	if !ctx.cr[6].eq {
	pc = 0x82339890; continue 'dispatch;
	}
	// 82339850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82339854: 419A001C  beq cr6, 0x82339870
	if ctx.cr[6].eq {
	pc = 0x82339870; continue 'dispatch;
	}
	// 82339858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233985C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82339860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339864: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233986C: 4E800421  bctrl
	ctx.lr = 0x82339870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82339870: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82339874: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82339878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233987C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82339880: 816B8F84  lwz r11, -0x707c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28796 as u32) ) } as u64;
	// 82339884: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82339888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233988C: 4BF86775  bl 0x822c0000
	ctx.lr = 0x82339890;
	sub_822C0000(ctx, base);
	// 82339890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233989C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823398A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823398A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823398A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823398B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823398B0 size=196
    let mut pc: u32 = 0x823398B0;
    'dispatch: loop {
        match pc {
            0x823398B0 => {
    //   block [0x823398B0..0x82339974)
	// 823398B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823398B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823398B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823398BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823398C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823398C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823398C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823398CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823398D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823398D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823398D8: 4BF87061  bl 0x822c0938
	ctx.lr = 0x823398DC;
	sub_822C0938(ctx, base);
	// 823398DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823398E0: 41820028  beq 0x82339908
	if ctx.cr[0].eq {
	pc = 0x82339908; continue 'dispatch;
	}
	// 823398E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823398E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823398EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823398F0: 392B1718  addi r9, r11, 0x1718
	ctx.r[9].s64 = ctx.r[11].s64 + 5912;
	// 823398F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823398F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823398FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82339900: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82339904: 48000008  b 0x8233990c
	pc = 0x8233990C; continue 'dispatch;
	// 82339908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233990C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82339914: 409A0044  bne cr6, 0x82339958
	if !ctx.cr[6].eq {
	pc = 0x82339958; continue 'dispatch;
	}
	// 82339918: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233991C: 419A001C  beq cr6, 0x82339938
	if ctx.cr[6].eq {
	pc = 0x82339938; continue 'dispatch;
	}
	// 82339920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339924: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82339928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233992C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82339934: 4E800421  bctrl
	ctx.lr = 0x82339938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82339938: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233993C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82339940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82339944: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82339948: 816B8F84  lwz r11, -0x707c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28796 as u32) ) } as u64;
	// 8233994C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82339950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82339954: 4BF866AD  bl 0x822c0000
	ctx.lr = 0x82339958;
	sub_822C0000(ctx, base);
	// 82339958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233995C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233996C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339978 size=132
    let mut pc: u32 = 0x82339978;
    'dispatch: loop {
        match pc {
            0x82339978 => {
    //   block [0x82339978..0x823399FC)
	// 82339978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233997C: 48E6E7F1  bl 0x831a816c
	ctx.lr = 0x82339980;
	sub_831A8130(ctx, base);
	// 82339980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339984: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339988: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233998C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82339990: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82339994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82339998: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8233999C: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 823399A0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 823399A4: 48AB8A45  bl 0x82df23e8
	ctx.lr = 0x823399A8;
	sub_82DF23E8(ctx, base);
	// 823399A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823399AC: 4182001C  beq 0x823399c8
	if ctx.cr[0].eq {
	pc = 0x823399C8; continue 'dispatch;
	}
	// 823399B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823399B4: 88BE0000  lbz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823399B8: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823399BC: 4881C135  bl 0x82b55af0
	ctx.lr = 0x823399C0;
	sub_82B55AF0(ctx, base);
	// 823399C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823399C4: 48000008  b 0x823399cc
	pc = 0x823399CC; continue 'dispatch;
	// 823399C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823399CC: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823399D0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823399D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823399D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823399DC: 4BFFFE0D  bl 0x823397e8
	ctx.lr = 0x823399E0;
	sub_823397E8(ctx, base);
	// 823399E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823399E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823399E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823399EC: 4BF86615  bl 0x822c0000
	ctx.lr = 0x823399F0;
	sub_822C0000(ctx, base);
	// 823399F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823399F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823399F8: 48E6E7C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339A00 size=404
    let mut pc: u32 = 0x82339A00;
    'dispatch: loop {
        match pc {
            0x82339A00 => {
    //   block [0x82339A00..0x82339B94)
	// 82339A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339A04: 48E6E765  bl 0x831a8168
	ctx.lr = 0x82339A08;
	sub_831A8130(ctx, base);
	// 82339A08: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82339A0C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339A10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339A14: 486D6E5D  bl 0x82a10870
	ctx.lr = 0x82339A18;
	sub_82A10870(ctx, base);
	// 82339A18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339A1C: 4BFCBA0D  bl 0x82305428
	ctx.lr = 0x82339A20;
	sub_82305428(ctx, base);
	// 82339A20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82339A24: 41820164  beq 0x82339b88
	if ctx.cr[0].eq {
	pc = 0x82339B88; continue 'dispatch;
	}
	// 82339A28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339A2C: 48B24D9D  bl 0x82e5e7c8
	ctx.lr = 0x82339A30;
	sub_82E5E7C8(ctx, base);
	// 82339A30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339A34: 38C0003E  li r6, 0x3e
	ctx.r[6].s64 = 62;
	// 82339A38: 3BAB1728  addi r29, r11, 0x1728
	ctx.r[29].s64 = ctx.r[11].s64 + 5928;
	// 82339A3C: 389E0070  addi r4, r30, 0x70
	ctx.r[4].s64 = ctx.r[30].s64 + 112;
	// 82339A40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82339A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339A48: 4BFCC021  bl 0x82305a68
	ctx.lr = 0x82339A4C;
	sub_82305A68(ctx, base);
	// 82339A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82339A50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82339A54: 4BFCE625  bl 0x82308078
	ctx.lr = 0x82339A58;
	sub_82308078(ctx, base);
	// 82339A58: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339A5C: 48AD8CB5  bl 0x82e12710
	ctx.lr = 0x82339A60;
	sub_82E12710(ctx, base);
	// 82339A60: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82339A64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82339A68: 419A0008  beq cr6, 0x82339a70
	if ctx.cr[6].eq {
	pc = 0x82339A70; continue 'dispatch;
	}
	// 82339A6C: 4BF86E25  bl 0x822c0890
	ctx.lr = 0x82339A70;
	sub_822C0890(ctx, base);
	// 82339A70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82339A74: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82339A78: 389E0080  addi r4, r30, 0x80
	ctx.r[4].s64 = ctx.r[30].s64 + 128;
	// 82339A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339A80: 4BFDA0B9  bl 0x82313b38
	ctx.lr = 0x82339A84;
	sub_82313B38(ctx, base);
	// 82339A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339A88: 486D9B11  bl 0x82a13598
	ctx.lr = 0x82339A8C;
	sub_82A13598(ctx, base);
	// 82339A8C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82339A90: 815E0090  lwz r10, 0x90(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82339A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339A98: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 82339A9C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82339AA0: 419A00E8  beq cr6, 0x82339b88
	if ctx.cr[6].eq {
	pc = 0x82339B88; continue 'dispatch;
	}
	// 82339AA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82339AA8: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82339AAC: 409A0008  bne cr6, 0x82339ab4
	if !ctx.cr[6].eq {
	pc = 0x82339AB4; continue 'dispatch;
	}
	// 82339AB0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82339AB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82339AB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82339ABC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82339AC0: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82339AC4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82339AC8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82339ACC: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82339AD0: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82339AD4: 4BFFFEA5  bl 0x82339978
	ctx.lr = 0x82339AD8;
	sub_82339978(ctx, base);
	// 82339AD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339ADC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82339AE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82339AE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82339AE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82339AEC: 419A0024  beq cr6, 0x82339b10
	if ctx.cr[6].eq {
	pc = 0x82339B10; continue 'dispatch;
	}
	// 82339AF0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82339AF4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82339AF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82339AFC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82339B00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82339B04: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82339B08: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82339B0C: 4082FFE8  bne 0x82339af4
	if !ctx.cr[0].eq {
	pc = 0x82339AF4; continue 'dispatch;
	}
	// 82339B10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82339B14: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82339B18: 481D59B1  bl 0x8250f4c8
	ctx.lr = 0x82339B1C;
	sub_8250F4C8(ctx, base);
	// 82339B1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339B20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82339B24: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82339B28: 409A0008  bne cr6, 0x82339b30
	if !ctx.cr[6].eq {
	pc = 0x82339B30; continue 'dispatch;
	}
	// 82339B2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82339B30: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82339B34: 481D0A95  bl 0x8250a5c8
	ctx.lr = 0x82339B38;
	sub_8250A5C8(ctx, base);
	// 82339B38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82339B3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82339B40: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 82339B44: 409A0008  bne cr6, 0x82339b4c
	if !ctx.cr[6].eq {
	pc = 0x82339B4C; continue 'dispatch;
	}
	// 82339B48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82339B4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82339B50: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82339B54: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82339B58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82339B5C: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82339B60: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 82339B64: 48473A6D  bl 0x827ad5d0
	ctx.lr = 0x82339B68;
	sub_827AD5D0(ctx, base);
	// 82339B68: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82339B6C: 48AB8125  bl 0x82df1c90
	ctx.lr = 0x82339B70;
	sub_82DF1C90(ctx, base);
	// 82339B70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82339B74: 48AB811D  bl 0x82df1c90
	ctx.lr = 0x82339B78;
	sub_82DF1C90(ctx, base);
	// 82339B78: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82339B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82339B80: 419A0008  beq cr6, 0x82339b88
	if ctx.cr[6].eq {
	pc = 0x82339B88; continue 'dispatch;
	}
	// 82339B84: 4BF86D0D  bl 0x822c0890
	ctx.lr = 0x82339B88;
	sub_822C0890(ctx, base);
	// 82339B88: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82339B8C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82339B90: 48E6E628  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339B98 size=348
    let mut pc: u32 = 0x82339B98;
    'dispatch: loop {
        match pc {
            0x82339B98 => {
    //   block [0x82339B98..0x82339CF4)
	// 82339B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339B9C: 48E6E5C5  bl 0x831a8160
	ctx.lr = 0x82339BA0;
	sub_831A8130(ctx, base);
	// 82339BA0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82339BA4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339BA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339BAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82339BB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82339BB4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82339BB8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82339BBC: 486D6CB5  bl 0x82a10870
	ctx.lr = 0x82339BC0;
	sub_82A10870(ctx, base);
	// 82339BC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82339BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339BC8: 4BFCDD11  bl 0x823078d8
	ctx.lr = 0x82339BCC;
	sub_823078D8(ctx, base);
	// 82339BCC: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82339BD0: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82339BD4: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82339BD8: 3D208336  lis r9, -0x7cca
	ctx.r[9].s64 = -2093613056;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339CF8 size=72
    let mut pc: u32 = 0x82339CF8;
    'dispatch: loop {
        match pc {
            0x82339CF8 => {
    //   block [0x82339CF8..0x82339D40)
	// 82339CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339D00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339D04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339D0C: 4BFF80CD  bl 0x82331dd8
	ctx.lr = 0x82339D10;
	sub_82331DD8(ctx, base);
	// 82339D10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82339D14: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82339D18: 396B1790  addi r11, r11, 0x1790
	ctx.r[11].s64 = ctx.r[11].s64 + 6032;
	// 82339D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339D20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82339D24: 816A853C  lwz r11, -0x7ac4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 82339D28: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82339D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82339D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339D38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339D40 size=116
    let mut pc: u32 = 0x82339D40;
    'dispatch: loop {
        match pc {
            0x82339D40 => {
    //   block [0x82339D40..0x82339DB4)
	// 82339D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339D48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82339D4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339D54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339D58: 486D6B19  bl 0x82a10870
	ctx.lr = 0x82339D5C;
	sub_82A10870(ctx, base);
	// 82339D5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339D64: 48D8DD7D  bl 0x830c7ae0
	ctx.lr = 0x82339D68;
	sub_830C7AE0(ctx, base);
	// 82339D68: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82339D6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339D70: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 82339D74: 4BFCBFDD  bl 0x82305d50
	ctx.lr = 0x82339D78;
	sub_82305D50(ctx, base);
	// 82339D78: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82339D7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82339D80: 808BB500  lwz r4, -0x4b00(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19200 as u32) ) } as u64;
	// 82339D84: 48AB9C85  bl 0x82df3a08
	ctx.lr = 0x82339D88;
	sub_82DF3A08(ctx, base);
	// 82339D88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82339D8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339D90: 4BFCB7B9  bl 0x82305548
	ctx.lr = 0x82339D94;
	sub_82305548(ctx, base);
	// 82339D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82339D98: 48AB9691  bl 0x82df3428
	ctx.lr = 0x82339D9C;
	sub_82DF3428(ctx, base);
	// 82339D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339DB8 size=112
    let mut pc: u32 = 0x82339DB8;
    'dispatch: loop {
        match pc {
            0x82339DB8 => {
    //   block [0x82339DB8..0x82339E28)
	// 82339DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339DC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82339DC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339DC8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82339DCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339DD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339DD4: 486D6A9D  bl 0x82a10870
	ctx.lr = 0x82339DD8;
	sub_82A10870(ctx, base);
	// 82339DD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339DE0: 48D8DD01  bl 0x830c7ae0
	ctx.lr = 0x82339DE4;
	sub_830C7AE0(ctx, base);
	// 82339DE4: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 82339DE8: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82339DEC: 48120FFD  bl 0x8245ade8
	ctx.lr = 0x82339DF0;
	sub_8245ADE8(ctx, base);
	// 82339DF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82339DF4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82339DF8: 48B1F969  bl 0x82e59760
	ctx.lr = 0x82339DFC;
	sub_82E59760(ctx, base);
	// 82339DFC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82339E00: 4099000C  ble cr6, 0x82339e0c
	if !ctx.cr[6].gt {
	pc = 0x82339E0C; continue 'dispatch;
	}
	// 82339E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339E08: 4BFE41B9  bl 0x8231dfc0
	ctx.lr = 0x82339E0C;
	sub_8231DFC0(ctx, base);
	// 82339E0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82339E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339E18: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82339E1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82339E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339E28 size=364
    let mut pc: u32 = 0x82339E28;
    'dispatch: loop {
        match pc {
            0x82339E28 => {
    //   block [0x82339E28..0x82339F94)
	// 82339E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82339E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339E38: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339E3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82339E40: 486D6A31  bl 0x82a10870
	ctx.lr = 0x82339E44;
	sub_82A10870(ctx, base);
	// 82339E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339E48: 4BFD8DC1  bl 0x82312c08
	ctx.lr = 0x82339E4C;
	sub_82312C08(ctx, base);
	// 82339E4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82339E50: 40820084  bne 0x82339ed4
	if !ctx.cr[0].eq {
	pc = 0x82339ED4; continue 'dispatch;
	}
	// 82339E54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82339E58: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82339E5C: 4BFD9D0D  bl 0x82313b68
	ctx.lr = 0x82339E60;
	sub_82313B68(ctx, base);
	// 82339E60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82339E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339E68: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82339E6C: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82339E70: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82339E74: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82339E78: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82339E7C: C16B000C  lfs f11, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82339E80: FD806050  fneg f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82339E84: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 82339E88: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82339E8C: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82339E90: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82339E94: D161006C  stfs f11, 0x6c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82339E98: 4BFD8F61  bl 0x82312df8
	ctx.lr = 0x82339E9C;
	sub_82312DF8(ctx, base);
	// 82339E9C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82339EA0: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82339EA4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82339EA8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82339EAC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82339EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82339EB4: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82339F98 size=64
    let mut pc: u32 = 0x82339F98;
    'dispatch: loop {
        match pc {
            0x82339F98 => {
    //   block [0x82339F98..0x82339FD8)
	// 82339F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82339FA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82339FAC: 48E1A655  bl 0x83154600
	ctx.lr = 0x82339FB0;
	sub_83154600(ctx, base);
	// 82339FB0: 486D68C1  bl 0x82a10870
	ctx.lr = 0x82339FB4;
	sub_82A10870(ctx, base);
	// 82339FB4: 4BFCDB75  bl 0x82307b28
	ctx.lr = 0x82339FB8;
	sub_82307B28(ctx, base);
	// 82339FB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82339FBC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82339FC0: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82339FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82339FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339FD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82339FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82339FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82339FD8 size=36
    let mut pc: u32 = 0x82339FD8;
    'dispatch: loop {
        match pc {
            0x82339FD8 => {
    //   block [0x82339FD8..0x82339FFC)
	// 82339FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82339FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82339FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82339FE4: 48E1A61D  bl 0x83154600
	ctx.lr = 0x82339FE8;
	sub_83154600(ctx, base);
	// 82339FE8: 486D6889  bl 0x82a10870
	ctx.lr = 0x82339FEC;
	sub_82A10870(ctx, base);
	// 82339FEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82339FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82339FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82339FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233A000 size=92
    let mut pc: u32 = 0x8233A000;
    'dispatch: loop {
        match pc {
            0x8233A000 => {
    //   block [0x8233A000..0x8233A05C)
	// 8233A000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A00C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A010: 48E1A5F1  bl 0x83154600
	ctx.lr = 0x8233A014;
	sub_83154600(ctx, base);
	// 8233A014: 486D685D  bl 0x82a10870
	ctx.lr = 0x8233A018;
	sub_82A10870(ctx, base);
	// 8233A018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A01C: 4BFCDB0D  bl 0x82307b28
	ctx.lr = 0x8233A020;
	sub_82307B28(ctx, base);
	// 8233A020: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A024: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A028: 808BB4F8  lwz r4, -0x4b08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19208 as u32) ) } as u64;
	// 8233A02C: 48AB99DD  bl 0x82df3a08
	ctx.lr = 0x8233A030;
	sub_82DF3A08(ctx, base);
	// 8233A030: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233A034: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233A038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A03C: 4BFCB4FD  bl 0x82305538
	ctx.lr = 0x8233A040;
	sub_82305538(ctx, base);
	// 8233A040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A044: 48AB93E5  bl 0x82df3428
	ctx.lr = 0x8233A048;
	sub_82DF3428(ctx, base);
	// 8233A048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233A060 size=88
    let mut pc: u32 = 0x8233A060;
    'dispatch: loop {
        match pc {
            0x8233A060 => {
    //   block [0x8233A060..0x8233A0B8)
	// 8233A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A06C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A070: 48E1A591  bl 0x83154600
	ctx.lr = 0x8233A074;
	sub_83154600(ctx, base);
	// 8233A074: 486D67FD  bl 0x82a10870
	ctx.lr = 0x8233A078;
	sub_82A10870(ctx, base);
	// 8233A078: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A084: 808BB4EC  lwz r4, -0x4b14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19220 as u32) ) } as u64;
	// 8233A088: 48AB9981  bl 0x82df3a08
	ctx.lr = 0x8233A08C;
	sub_82DF3A08(ctx, base);
	// 8233A08C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233A090: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233A094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A098: 4BFCB4A1  bl 0x82305538
	ctx.lr = 0x8233A09C;
	sub_82305538(ctx, base);
	// 8233A09C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A0A0: 48AB9389  bl 0x82df3428
	ctx.lr = 0x8233A0A4;
	sub_82DF3428(ctx, base);
	// 8233A0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A0B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233A0B8 size=196
    let mut pc: u32 = 0x8233A0B8;
    'dispatch: loop {
        match pc {
            0x8233A0B8 => {
    //   block [0x8233A0B8..0x8233A17C)
	// 8233A0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A0C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A0CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A0D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233A0D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233A0DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A0E0: 4BF86859  bl 0x822c0938
	ctx.lr = 0x8233A0E4;
	sub_822C0938(ctx, base);
	// 8233A0E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233A0E8: 41820028  beq 0x8233a110
	if ctx.cr[0].eq {
	pc = 0x8233A110; continue 'dispatch;
	}
	// 8233A0EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A0F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233A0F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233A0F8: 392B17CC  addi r9, r11, 0x17cc
	ctx.r[9].s64 = ctx.r[11].s64 + 6092;
	// 8233A0FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233A100: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233A104: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233A108: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233A10C: 48000008  b 0x8233a114
	pc = 0x8233A114; continue 'dispatch;
	// 8233A110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A114: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233A11C: 409A0044  bne cr6, 0x8233a160
	if !ctx.cr[6].eq {
	pc = 0x8233A160; continue 'dispatch;
	}
	// 8233A120: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A124: 419A001C  beq cr6, 0x8233a140
	if ctx.cr[6].eq {
	pc = 0x8233A140; continue 'dispatch;
	}
	// 8233A128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233A12C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233A130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A134: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233A138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233A13C: 4E800421  bctrl
	ctx.lr = 0x8233A140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233A140: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A144: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A14C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233A150: 816B9064  lwz r11, -0x6f9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28572 as u32) ) } as u64;
	// 8233A154: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233A158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233A15C: 4BF85EA5  bl 0x822c0000
	ctx.lr = 0x8233A160;
	sub_822C0000(ctx, base);
	// 8233A160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A164: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A170: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233A174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233A180 size=196
    let mut pc: u32 = 0x8233A180;
    'dispatch: loop {
        match pc {
            0x8233A180 => {
    //   block [0x8233A180..0x8233A244)
	// 8233A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A18C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A194: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A19C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233A1A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233A1A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A1A8: 4BF86791  bl 0x822c0938
	ctx.lr = 0x8233A1AC;
	sub_822C0938(ctx, base);
	// 8233A1AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233A1B0: 41820028  beq 0x8233a1d8
	if ctx.cr[0].eq {
	pc = 0x8233A1D8; continue 'dispatch;
	}
	// 8233A1B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A1B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233A1BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233A1C0: 392B17E0  addi r9, r11, 0x17e0
	ctx.r[9].s64 = ctx.r[11].s64 + 6112;
	// 8233A1C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233A1C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233A1CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233A1D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233A1D4: 48000008  b 0x8233a1dc
	pc = 0x8233A1DC; continue 'dispatch;
	// 8233A1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A1DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A1E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233A1E4: 409A0044  bne cr6, 0x8233a228
	if !ctx.cr[6].eq {
	pc = 0x8233A228; continue 'dispatch;
	}
	// 8233A1E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A1EC: 419A001C  beq cr6, 0x8233a208
	if ctx.cr[6].eq {
	pc = 0x8233A208; continue 'dispatch;
	}
	// 8233A1F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233A1F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233A1F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A1FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233A200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233A204: 4E800421  bctrl
	ctx.lr = 0x8233A208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233A208: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A20C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A214: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233A218: 816B9064  lwz r11, -0x6f9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28572 as u32) ) } as u64;
	// 8233A21C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233A220: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233A224: 4BF85DDD  bl 0x822c0000
	ctx.lr = 0x8233A228;
	sub_822C0000(ctx, base);
	// 8233A228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A22C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233A23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233A248 size=196
    let mut pc: u32 = 0x8233A248;
    'dispatch: loop {
        match pc {
            0x8233A248 => {
    //   block [0x8233A248..0x8233A30C)
	// 8233A248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A25C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A264: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233A268: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233A26C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A270: 4BF866C9  bl 0x822c0938
	ctx.lr = 0x8233A274;
	sub_822C0938(ctx, base);
	// 8233A274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233A278: 41820028  beq 0x8233a2a0
	if ctx.cr[0].eq {
	pc = 0x8233A2A0; continue 'dispatch;
	}
	// 8233A27C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A280: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233A284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233A288: 392B17F4  addi r9, r11, 0x17f4
	ctx.r[9].s64 = ctx.r[11].s64 + 6132;
	// 8233A28C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233A290: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233A294: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233A298: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233A29C: 48000008  b 0x8233a2a4
	pc = 0x8233A2A4; continue 'dispatch;
	// 8233A2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233A2A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A2A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233A2AC: 409A0044  bne cr6, 0x8233a2f0
	if !ctx.cr[6].eq {
	pc = 0x8233A2F0; continue 'dispatch;
	}
	// 8233A2B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A2B4: 419A001C  beq cr6, 0x8233a2d0
	if ctx.cr[6].eq {
	pc = 0x8233A2D0; continue 'dispatch;
	}
	// 8233A2B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233A2BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233A2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A2C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233A2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233A2CC: 4E800421  bctrl
	ctx.lr = 0x8233A2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233A2D0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A2D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A2DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233A2E0: 816B9064  lwz r11, -0x6f9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28572 as u32) ) } as u64;
	// 8233A2E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233A2E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233A2EC: 4BF85D15  bl 0x822c0000
	ctx.lr = 0x8233A2F0;
	sub_822C0000(ctx, base);
	// 8233A2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A2F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233A304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A310 size=252
    let mut pc: u32 = 0x8233A310;
    'dispatch: loop {
        match pc {
            0x8233A310 => {
    //   block [0x8233A310..0x8233A40C)
	// 8233A310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A320: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233A324: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A32C: 486D6545  bl 0x82a10870
	ctx.lr = 0x8233A330;
	sub_82A10870(ctx, base);
	// 8233A330: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A334: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A338: 4BFF4361  bl 0x8232e698
	ctx.lr = 0x8233A33C;
	sub_8232E698(ctx, base);
	// 8233A33C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233A340: 408200B0  bne 0x8233a3f0
	if !ctx.cr[0].eq {
	pc = 0x8233A3F0; continue 'dispatch;
	}
	// 8233A344: 38800081  li r4, 0x81
	ctx.r[4].s64 = 129;
	// 8233A348: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233A34C: 48120A9D  bl 0x8245ade8
	ctx.lr = 0x8233A350;
	sub_8245ADE8(ctx, base);
	// 8233A350: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 8233A354: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233A358: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233A35C: 48120A8D  bl 0x8245ade8
	ctx.lr = 0x8233A360;
	sub_8245ADE8(ctx, base);
	// 8233A360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A364: EFFF0828  fsubs f31, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 8233A368: 48B1F3F9  bl 0x82e59760
	ctx.lr = 0x8233A36C;
	sub_82E59760(ctx, base);
	// 8233A36C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A370: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A374: C00B9060  lfs f0, -0x6fa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28576 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233A378: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8233A37C: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233A380: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8233A384: 41990008  bgt cr6, 0x8233a38c
	if ctx.cr[6].gt {
	pc = 0x8233A38C; continue 'dispatch;
	}
	// 8233A388: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 8233A38C: FD60F850  fneg f11, f31
	ctx.f[11].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 8233A390: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8233A394: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A398: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233A39C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233A3A0: C18B2720  lfs f12, 0x2720(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10016 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8233A3A4: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233A3A8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8233A3AC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8233A3B0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8233A3B4: EC0C5828  fsubs f0, f12, f11
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8233A3B8: EC005B7A  fmadds f0, f0, f13, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 8233A3BC: D01F00C8  stfs f0, 0xc8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8233A3C0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8233A3C4: 48B41C15  bl 0x82e7bfd8
	ctx.lr = 0x8233A3C8;
	sub_82E7BFD8(ctx, base);
	// 8233A3C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233A3CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A3D0: 4BFCB981  bl 0x82305d50
	ctx.lr = 0x8233A3D4;
	sub_82305D50(ctx, base);
	// 8233A3D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A3D8: 48B1F391  bl 0x82e59768
	ctx.lr = 0x8233A3DC;
	sub_82E59768(ctx, base);
	// 8233A3DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233A3E0: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8233A3E4: 48B22A95  bl 0x82e5ce78
	ctx.lr = 0x8233A3E8;
	sub_82E5CE78(ctx, base);
	// 8233A3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A3EC: 48D8D6F5  bl 0x830c7ae0
	ctx.lr = 0x8233A3F0;
	sub_830C7AE0(ctx, base);
	// 8233A3F0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8233A3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A3FC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8233A400: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233A404: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A410 size=236
    let mut pc: u32 = 0x8233A410;
    'dispatch: loop {
        match pc {
            0x8233A410 => {
    //   block [0x8233A410..0x8233A4FC)
	// 8233A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A420: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233A424: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A42C: 486D6445  bl 0x82a10870
	ctx.lr = 0x8233A430;
	sub_82A10870(ctx, base);
	// 8233A430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A434: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8233A438: 48B22261  bl 0x82e5c698
	ctx.lr = 0x8233A43C;
	sub_82E5C698(ctx, base);
	// 8233A43C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8233A440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A444: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 8233A448: 4BFCB909  bl 0x82305d50
	ctx.lr = 0x8233A44C;
	sub_82305D50(ctx, base);
	// 8233A44C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233A450: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233A454: C3FF00C8  lfs f31, 0xc8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233A458: 4BFCD869  bl 0x82307cc0
	ctx.lr = 0x8233A45C;
	sub_82307CC0(ctx, base);
	// 8233A45C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8233A460: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8233A464: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8233A468: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8233A46C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8233A470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A474: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A500 size=136
    let mut pc: u32 = 0x8233A500;
    'dispatch: loop {
        match pc {
            0x8233A500 => {
    //   block [0x8233A500..0x8233A588)
	// 8233A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A50C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A514: 48E1A0ED  bl 0x83154600
	ctx.lr = 0x8233A518;
	sub_83154600(ctx, base);
	// 8233A518: 486D6359  bl 0x82a10870
	ctx.lr = 0x8233A51C;
	sub_82A10870(ctx, base);
	// 8233A51C: 4BFCAF0D  bl 0x82305428
	ctx.lr = 0x8233A520;
	sub_82305428(ctx, base);
	// 8233A520: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233A524: 41820050  beq 0x8233a574
	if ctx.cr[0].eq {
	pc = 0x8233A574; continue 'dispatch;
	}
	// 8233A528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A52C: 48E1A0D5  bl 0x83154600
	ctx.lr = 0x8233A530;
	sub_83154600(ctx, base);
	// 8233A530: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233A534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233A53C: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233A540: 48AB94C9  bl 0x82df3a08
	ctx.lr = 0x8233A544;
	sub_82DF3A08(ctx, base);
	// 8233A544: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233A548: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233A54C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A550: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233A558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233A55C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233A560: 48B24531  bl 0x82e5ea90
	ctx.lr = 0x8233A564;
	sub_82E5EA90(ctx, base);
	// 8233A564: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233A568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233A56C: 419A0008  beq cr6, 0x8233a574
	if ctx.cr[6].eq {
	pc = 0x8233A574; continue 'dispatch;
	}
	// 8233A570: 4BF86321  bl 0x822c0890
	ctx.lr = 0x8233A574;
	sub_822C0890(ctx, base);
	// 8233A574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233A578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A588 size=544
    let mut pc: u32 = 0x8233A588;
    'dispatch: loop {
        match pc {
            0x8233A588 => {
    //   block [0x8233A588..0x8233A7A8)
	// 8233A588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A58C: 48E6DBE1  bl 0x831a816c
	ctx.lr = 0x8233A590;
	sub_831A8130(ctx, base);
	// 8233A590: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8233A594: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233A598: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A59C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A5A0: 486D62D1  bl 0x82a10870
	ctx.lr = 0x8233A5A4;
	sub_82A10870(ctx, base);
	// 8233A5A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233A5A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A5AC: 48D8D535  bl 0x830c7ae0
	ctx.lr = 0x8233A5B0;
	sub_830C7AE0(ctx, base);
	// 8233A5B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233A5B4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8233A5B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233A5BC: 388A1880  addi r4, r10, 0x1880
	ctx.r[4].s64 = ctx.r[10].s64 + 6272;
	// 8233A5C0: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 8233A5C4: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233A5C8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8233A5CC: D3FE00C8  stfs f31, 0xc8(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8233A5D0: 48AB7E19  bl 0x82df23e8
	ctx.lr = 0x8233A5D4;
	sub_82DF23E8(ctx, base);
	// 8233A5D4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8233A5D8: 4182001C  beq 0x8233a5f4
	if ctx.cr[0].eq {
	pc = 0x8233A5F4; continue 'dispatch;
	}
	// 8233A5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A5E0: 48B22349  bl 0x82e5c928
	ctx.lr = 0x8233A5E4;
	sub_82E5C928(ctx, base);
	// 8233A5E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A5E8: 396B1808  addi r11, r11, 0x1808
	ctx.r[11].s64 = ctx.r[11].s64 + 6152;
	// 8233A5EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A5F0: 48000008  b 0x8233a5f8
	pc = 0x8233A5F8; continue 'dispatch;
	// 8233A5F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8233A5F8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8233A5FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A600: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8233A604: 4BFFFAB5  bl 0x8233a0b8
	ctx.lr = 0x8233A608;
	sub_8233A0B8(ctx, base);
	// 8233A608: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233A60C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A610: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8233A614: 4BF859ED  bl 0x822c0000
	ctx.lr = 0x8233A618;
	sub_822C0000(ctx, base);
	// 8233A618: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233A61C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8233A620: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A624: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8233A628: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8233A62C: 419A0024  beq cr6, 0x8233a650
	if ctx.cr[6].eq {
	pc = 0x8233A650; continue 'dispatch;
	}
	// 8233A630: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8233A634: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8233A638: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233A63C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8233A640: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233A644: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8233A648: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233A64C: 4082FFE8  bne 0x8233a634
	if !ctx.cr[0].eq {
	pc = 0x8233A634; continue 'dispatch;
	}
	// 8233A650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233A654: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8233A658: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233A65C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8233A660: 389E0068  addi r4, r30, 0x68
	ctx.r[4].s64 = ctx.r[30].s64 + 104;
	// 8233A664: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8233A668: 48B24041  bl 0x82e5e6a8
	ctx.lr = 0x8233A66C;
	sub_82E5E6A8(ctx, base);
	// 8233A66C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8233A670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233A674: 419A0008  beq cr6, 0x8233a67c
	if ctx.cr[6].eq {
	pc = 0x8233A67C; continue 'dispatch;
	}
	// 8233A678: 4BF86219  bl 0x822c0890
	ctx.lr = 0x8233A67C;
	sub_822C0890(ctx, base);
	// 8233A67C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8233A680: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233A684: 419A0008  beq cr6, 0x8233a68c
	if ctx.cr[6].eq {
	pc = 0x8233A68C; continue 'dispatch;
	}
	// 8233A688: 4BF86209  bl 0x822c0890
	ctx.lr = 0x8233A68C;
	sub_822C0890(ctx, base);
	// 8233A68C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A690: 419A000C  beq cr6, 0x8233a69c
	if ctx.cr[6].eq {
	pc = 0x8233A69C; continue 'dispatch;
	}
	// 8233A694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A698: 4BF861F9  bl 0x822c0890
	ctx.lr = 0x8233A69C;
	sub_822C0890(ctx, base);
	// 8233A69C: 38800081  li r4, 0x81
	ctx.r[4].s64 = 129;
	// 8233A6A0: 807D01FC  lwz r3, 0x1fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233A6A4: 48120745  bl 0x8245ade8
	ctx.lr = 0x8233A6A8;
	sub_8245ADE8(ctx, base);
	// 8233A6A8: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 8233A6AC: 807D01FC  lwz r3, 0x1fc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233A6B0: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8233A6B4: 48120735  bl 0x8245ade8
	ctx.lr = 0x8233A6B8;
	sub_8245ADE8(ctx, base);
	// 8233A6B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233A6BC: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8233A6C0: EFDE0828  fsubs f30, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 8233A6C4: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8233A6C8: 4BFCD5F9  bl 0x82307cc0
	ctx.lr = 0x8233A6CC;
	sub_82307CC0(ctx, base);
	// 8233A6CC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8233A6D0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8233A6D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233A6D8: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 8233A6DC: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 8233A6E0: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8233A6E4: 13C05C07  vcmpneb. (lvlx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8233A6E8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A7A8 size=296
    let mut pc: u32 = 0x8233A7A8;
    'dispatch: loop {
        match pc {
            0x8233A7A8 => {
    //   block [0x8233A7A8..0x8233A8D0)
	// 8233A7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A7B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A7B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A7BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A7C0: 48E19E41  bl 0x83154600
	ctx.lr = 0x8233A7C4;
	sub_83154600(ctx, base);
	// 8233A7C4: 486D60AD  bl 0x82a10870
	ctx.lr = 0x8233A7C8;
	sub_82A10870(ctx, base);
	// 8233A7C8: 4BFCAC61  bl 0x82305428
	ctx.lr = 0x8233A7CC;
	sub_82305428(ctx, base);
	// 8233A7CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233A7D0: 418200E8  beq 0x8233a8b8
	if ctx.cr[0].eq {
	pc = 0x8233A8B8; continue 'dispatch;
	}
	// 8233A7D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A7D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233A7DC: 388B1880  addi r4, r11, 0x1880
	ctx.r[4].s64 = ctx.r[11].s64 + 6272;
	// 8233A7E0: 38A000AA  li r5, 0xaa
	ctx.r[5].s64 = 170;
	// 8233A7E4: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8233A7E8: 48AB7C01  bl 0x82df23e8
	ctx.lr = 0x8233A7EC;
	sub_82DF23E8(ctx, base);
	// 8233A7EC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8233A7F0: 4182001C  beq 0x8233a80c
	if ctx.cr[0].eq {
	pc = 0x8233A80C; continue 'dispatch;
	}
	// 8233A7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A7F8: 48B22131  bl 0x82e5c928
	ctx.lr = 0x8233A7FC;
	sub_82E5C928(ctx, base);
	// 8233A7FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233A800: 396B1830  addi r11, r11, 0x1830
	ctx.r[11].s64 = ctx.r[11].s64 + 6192;
	// 8233A804: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233A808: 48000008  b 0x8233a810
	pc = 0x8233A810; continue 'dispatch;
	// 8233A80C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8233A810: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8233A814: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A818: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233A81C: 4BFFF965  bl 0x8233a180
	ctx.lr = 0x8233A820;
	sub_8233A180(ctx, base);
	// 8233A820: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233A824: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233A828: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233A82C: 4BF857D5  bl 0x822c0000
	ctx.lr = 0x8233A830;
	sub_822C0000(ctx, base);
	// 8233A830: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233A834: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8233A838: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A83C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8233A840: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8233A844: 419A0024  beq cr6, 0x8233a868
	if ctx.cr[6].eq {
	pc = 0x8233A868; continue 'dispatch;
	}
	// 8233A848: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8233A84C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8233A850: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233A854: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8233A858: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233A85C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8233A860: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233A864: 4082FFE8  bne 0x8233a84c
	if !ctx.cr[0].eq {
	pc = 0x8233A84C; continue 'dispatch;
	}
	// 8233A868: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233A86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233A870: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233A874: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8233A878: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233A87C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233A880: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233A884: 48B2403D  bl 0x82e5e8c0
	ctx.lr = 0x8233A888;
	sub_82E5E8C0(ctx, base);
	// 8233A888: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8233A88C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233A890: 419A0008  beq cr6, 0x8233a898
	if ctx.cr[6].eq {
	pc = 0x8233A898; continue 'dispatch;
	}
	// 8233A894: 4BF85FFD  bl 0x822c0890
	ctx.lr = 0x8233A898;
	sub_822C0890(ctx, base);
	// 8233A898: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233A89C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233A8A0: 419A0008  beq cr6, 0x8233a8a8
	if ctx.cr[6].eq {
	pc = 0x8233A8A8; continue 'dispatch;
	}
	// 8233A8A4: 4BF85FED  bl 0x822c0890
	ctx.lr = 0x8233A8A8;
	sub_822C0890(ctx, base);
	// 8233A8A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233A8AC: 419A000C  beq cr6, 0x8233a8b8
	if ctx.cr[6].eq {
	pc = 0x8233A8B8; continue 'dispatch;
	}
	// 8233A8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A8B4: 4BF85FDD  bl 0x822c0890
	ctx.lr = 0x8233A8B8;
	sub_822C0890(ctx, base);
	// 8233A8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233A8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233A8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233A8C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233A8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233A8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233A8D0 size=456
    let mut pc: u32 = 0x8233A8D0;
    'dispatch: loop {
        match pc {
            0x8233A8D0 => {
    //   block [0x8233A8D0..0x8233AA98)
	// 8233A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233A8D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233A8DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233A8E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233A8E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233A8E8: 48E19D19  bl 0x83154600
	ctx.lr = 0x8233A8EC;
	sub_83154600(ctx, base);
	// 8233A8EC: 486D5F85  bl 0x82a10870
	ctx.lr = 0x8233A8F0;
	sub_82A10870(ctx, base);
	// 8233A8F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233A8F4: 4BFD8AAD  bl 0x823133a0
	ctx.lr = 0x8233A8F8;
	sub_823133A0(ctx, base);
	// 8233A8F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233A8FC: 40820178  bne 0x8233aa74
	if !ctx.cr[0].eq {
	pc = 0x8233AA74; continue 'dispatch;
	}
	// 8233A900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233A904: 48B1EE75  bl 0x82e59778
	ctx.lr = 0x8233A908;
	sub_82E59778(ctx, base);
	// 8233A908: C01E0060  lfs f0, 0x60(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233A90C: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8233A910: D01E0060  stfs f0, 0x60(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8233A914: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8233A918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233A91C: 4BFCB285  bl 0x82305ba0
	ctx.lr = 0x8233A920;
	sub_82305BA0(ctx, base);
	// 8233A920: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233AA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8233AA98 size=408
    let mut pc: u32 = 0x8233AA98;
    'dispatch: loop {
        match pc {
            0x8233AA98 => {
    //   block [0x8233AA98..0x8233AC30)
	// 8233AA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233AA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233AAA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233AAA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233AAA8: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233AC30 size=128
    let mut pc: u32 = 0x8233AC30;
    'dispatch: loop {
        match pc {
            0x8233AC30 => {
    //   block [0x8233AC30..0x8233ACB0)
	// 8233AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233AC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233AC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233AC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233AC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233AC48: 4BFF7191  bl 0x82331dd8
	ctx.lr = 0x8233AC4C;
	sub_82331DD8(ctx, base);
	// 8233AC4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233AC50: 3BDF0068  addi r30, r31, 0x68
	ctx.r[30].s64 = ctx.r[31].s64 + 104;
	// 8233AC54: 396B18F0  addi r11, r11, 0x18f0
	ctx.r[11].s64 = ctx.r[11].s64 + 6384;
	// 8233AC58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233AC5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233AC60: 48B22C71  bl 0x82e5d8d0
	ctx.lr = 0x8233AC64;
	sub_82E5D8D0(ctx, base);
	// 8233AC64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233AC68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233AC6C: 396B17BC  addi r11, r11, 0x17bc
	ctx.r[11].s64 = ctx.r[11].s64 + 6076;
	// 8233AC70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233AC74: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8233AC78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233AC7C: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233AC80: D01F00C8  stfs f0, 0xc8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8233AC84: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233AC88: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8233AC8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233AC90: 4E800421  bctrl
	ctx.lr = 0x8233AC94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233AC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AC98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233AC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233ACA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233ACA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233ACA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233ACB0 size=88
    let mut pc: u32 = 0x8233ACB0;
    'dispatch: loop {
        match pc {
            0x8233ACB0 => {
    //   block [0x8233ACB0..0x8233AD08)
	// 8233ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233ACB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233ACBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233ACC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233ACC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233ACC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233ACCC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8233ACD0: 48B22B79  bl 0x82e5d848
	ctx.lr = 0x8233ACD4;
	sub_82E5D848(ctx, base);
	// 8233ACD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233ACD8: 4BFF7191  bl 0x82331e68
	ctx.lr = 0x8233ACDC;
	sub_82331E68(ctx, base);
	// 8233ACDC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233ACE0: 4182000C  beq 0x8233acec
	if ctx.cr[0].eq {
	pc = 0x8233ACEC; continue 'dispatch;
	}
	// 8233ACE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233ACE8: 48AB76F1  bl 0x82df23d8
	ctx.lr = 0x8233ACEC;
	sub_82DF23D8(ctx, base);
	// 8233ACEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233ACF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233ACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233ACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233ACFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233AD00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233AD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233AD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233AD08 size=156
    let mut pc: u32 = 0x8233AD08;
    'dispatch: loop {
        match pc {
            0x8233AD08 => {
    //   block [0x8233AD08..0x8233ADA4)
	// 8233AD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233AD0C: 48E6D45D  bl 0x831a8168
	ctx.lr = 0x8233AD10;
	sub_831A8130(ctx, base);
	// 8233AD10: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8233AD14: 48E6DD59  bl 0x831a8a6c
	ctx.lr = 0x8233AD18;
	sub_831A8A40(ctx, base);
	// 8233AD18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233AD1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233AD20: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233AD24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233AD28: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8233AD2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8233AD30: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 8233AD34: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8233AD38: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 8233AD3C: FF602890  fmr f27, f5
	ctx.f[27].f64 = ctx.f[5].f64;
	// 8233AD40: FF403090  fmr f26, f6
	ctx.f[26].f64 = ctx.f[6].f64;
	// 8233AD44: FF203890  fmr f25, f7
	ctx.f[25].f64 = ctx.f[7].f64;
	// 8233AD48: 486D5B29  bl 0x82a10870
	ctx.lr = 0x8233AD4C;
	sub_82A10870(ctx, base);
	// 8233AD4C: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 8233AD50: 81210104  lwz r9, 0x104(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8233AD54: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 8233AD58: 8101011C  lwz r8, 0x11c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 8233AD5C: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233ADA8 size=124
    let mut pc: u32 = 0x8233ADA8;
    'dispatch: loop {
        match pc {
            0x8233ADA8 => {
    //   block [0x8233ADA8..0x8233AE24)
	// 8233ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233ADAC: 48E6D3BD  bl 0x831a8168
	ctx.lr = 0x8233ADB0;
	sub_831A8130(ctx, base);
	// 8233ADB0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8233ADB4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8233ADB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233ADBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233ADC0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233ADC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233ADC8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8233ADCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8233ADD0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8233ADD4: 486D5A9D  bl 0x82a10870
	ctx.lr = 0x8233ADD8;
	sub_82A10870(ctx, base);
	// 8233ADD8: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 8233ADDC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8233ADE0: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 8233ADE4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8233ADE8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233AE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233AE28 size=212
    let mut pc: u32 = 0x8233AE28;
    'dispatch: loop {
        match pc {
            0x8233AE28 => {
    //   block [0x8233AE28..0x8233AEFC)
	// 8233AE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233AE2C: 48E6D33D  bl 0x831a8168
	ctx.lr = 0x8233AE30;
	sub_831A8130(ctx, base);
	// 8233AE30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233AE34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233AE38: 486D5A39  bl 0x82a10870
	ctx.lr = 0x8233AE3C;
	sub_82A10870(ctx, base);
	// 8233AE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233AE40: 4BFD92A9  bl 0x823140e8
	ctx.lr = 0x8233AE44;
	sub_823140E8(ctx, base);
	// 8233AE44: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233AE48: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8233AE4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233AE50: 808BB3C8  lwz r4, -0x4c38(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19512 as u32) ) } as u64;
	// 8233AE54: 48AB8BB5  bl 0x82df3a08
	ctx.lr = 0x8233AE58;
	sub_82DF3A08(ctx, base);
	// 8233AE58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233AE5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233AE60: 48B1EB71  bl 0x82e599d0
	ctx.lr = 0x8233AE64;
	sub_82E599D0(ctx, base);
	// 8233AE64: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233AE68: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8233AE6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233AE70: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8233AE74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233AE78: 4E800421  bctrl
	ctx.lr = 0x8233AE7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233AE7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233AE80: 48AB8421  bl 0x82df32a0
	ctx.lr = 0x8233AE84;
	sub_82DF32A0(ctx, base);
	// 8233AE84: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233AE88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233AE8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233AE90: 419A000C  beq cr6, 0x8233ae9c
	if ctx.cr[6].eq {
	pc = 0x8233AE9C; continue 'dispatch;
	}
	// 8233AE94: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8233AE98: 4BF859F9  bl 0x822c0890
	ctx.lr = 0x8233AE9C;
	sub_822C0890(ctx, base);
	// 8233AE9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233AEA0: 48AB8589  bl 0x82df3428
	ctx.lr = 0x8233AEA4;
	sub_82DF3428(ctx, base);
	// 8233AEA4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233AEA8: 4182000C  beq 0x8233aeb4
	if ctx.cr[0].eq {
	pc = 0x8233AEB4; continue 'dispatch;
	}
	// 8233AEAC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8233AEB0: 4BFD2A61  bl 0x8230d910
	ctx.lr = 0x8233AEB4;
	sub_8230D910(ctx, base);
	// 8233AEB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233AEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AEBC: 4BFD8E35  bl 0x82313cf0
	ctx.lr = 0x8233AEC0;
	sub_82313CF0(ctx, base);
	// 8233AEC0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8233AEC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AEC8: 4BFD8E29  bl 0x82313cf0
	ctx.lr = 0x8233AECC;
	sub_82313CF0(ctx, base);
	// 8233AECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AED0: 809E00DC  lwz r4, 0xdc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(220 as u32) ) } as u64;
	// 8233AED4: 4BFDFB85  bl 0x8231aa58
	ctx.lr = 0x8233AED8;
	sub_8231AA58(ctx, base);
	// 8233AED8: 389E00C0  addi r4, r30, 0xc0
	ctx.r[4].s64 = ctx.r[30].s64 + 192;
	// 8233AEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AEE0: 4BFD91B1  bl 0x82314090
	ctx.lr = 0x8233AEE4;
	sub_82314090(ctx, base);
	// 8233AEE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AEE8: 4BFD84E1  bl 0x823133c8
	ctx.lr = 0x8233AEEC;
	sub_823133C8(ctx, base);
	// 8233AEEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AEF0: 4BFCA889  bl 0x82305778
	ctx.lr = 0x8233AEF4;
	sub_82305778(ctx, base);
	// 8233AEF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8233AEF8: 48E6D2C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233AF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233AF00 size=384
    let mut pc: u32 = 0x8233AF00;
    'dispatch: loop {
        match pc {
            0x8233AF00 => {
    //   block [0x8233AF00..0x8233B080)
	// 8233AF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233AF04: 48E6D261  bl 0x831a8164
	ctx.lr = 0x8233AF08;
	sub_831A8130(ctx, base);
	// 8233AF08: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 8233AF0C: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8233AF10: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233AF14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233AF18: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8233AF1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233AF20: 486D5951  bl 0x82a10870
	ctx.lr = 0x8233AF24;
	sub_82A10870(ctx, base);
	// 8233AF24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8233AF28: 4BFD8039  bl 0x82312f60
	ctx.lr = 0x8233AF2C;
	sub_82312F60(ctx, base);
	// 8233AF2C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8233AF30: 807C01FC  lwz r3, 0x1fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233AF34: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233AF38: 4811FEB1  bl 0x8245ade8
	ctx.lr = 0x8233AF3C;
	sub_8245ADE8(ctx, base);
	// 8233AF3C: 38800039  li r4, 0x39
	ctx.r[4].s64 = 57;
	// 8233AF40: 807C01FC  lwz r3, 0x1fc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233AF44: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8233AF48: 4811FEA1  bl 0x8245ade8
	ctx.lr = 0x8233AF4C;
	sub_8245ADE8(ctx, base);
	// 8233AF4C: EC1FF028  fsubs f0, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 8233AF50: EDA1F028  fsubs f13, f1, f30
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[30].f64) as f32) as f64);
	// 8233AF54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233AF58: C19F00A4  lfs f12, 0xa4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8233AF5C: C17F00A8  lfs f11, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8233AF60: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233AF64: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8233AF68: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8233AF6C: 4099000C  ble cr6, 0x8233af78
	if !ctx.cr[6].gt {
	pc = 0x8233AF78; continue 'dispatch;
	}
	// 8233AF70: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 8233AF74: 48000018  b 0x8233af8c
	pc = 0x8233AF8C; continue 'dispatch;
	// 8233AF78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233AF7C: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233AF80: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8233AF84: 40980008  bge cr6, 0x8233af8c
	if !ctx.cr[6].lt {
	pc = 0x8233AF8C; continue 'dispatch;
	}
	// 8233AF88: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233B080 size=740
    let mut pc: u32 = 0x8233B080;
    'dispatch: loop {
        match pc {
            0x8233B080 => {
    //   block [0x8233B080..0x8233B364)
	// 8233B080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B084: 48E6D0E9  bl 0x831a816c
	ctx.lr = 0x8233B088;
	sub_831A8130(ctx, base);
	// 8233B088: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233B08C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233B090: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233B094: 486D57DD  bl 0x82a10870
	ctx.lr = 0x8233B098;
	sub_82A10870(ctx, base);
	// 8233B098: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B09C: 38E0002B  li r7, 0x2b
	ctx.r[7].s64 = 43;
	// 8233B0A0: 3BAB1930  addi r29, r11, 0x1930
	ctx.r[29].s64 = ctx.r[11].s64 + 6448;
	// 8233B0A4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8233B0A8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8233B0AC: 388BBA80  addi r4, r11, -0x4580
	ctx.r[4].s64 = ctx.r[11].s64 + -17792;
	// 8233B0B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233B0B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233B0B8: 4BFDF0A1  bl 0x8231a158
	ctx.lr = 0x8233B0BC;
	sub_8231A158(ctx, base);
	// 8233B0BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233B0C0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8233B0C4: 4BFDC98D  bl 0x82317a50
	ctx.lr = 0x8233B0C8;
	sub_82317A50(ctx, base);
	// 8233B0C8: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 8233B0CC: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8233B0D0: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8233B368 size=716
    let mut pc: u32 = 0x8233B368;
    'dispatch: loop {
        match pc {
            0x8233B368 => {
    //   block [0x8233B368..0x8233B634)
	// 8233B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B36C: 48E6CDF9  bl 0x831a8164
	ctx.lr = 0x8233B370;
	sub_831A8130(ctx, base);
	// 8233B370: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 8233B374: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8233B378: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233B638 size=60
    let mut pc: u32 = 0x8233B638;
    'dispatch: loop {
        match pc {
            0x8233B638 => {
    //   block [0x8233B638..0x8233B674)
	// 8233B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233B640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233B644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233B648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233B64C: 4BFF678D  bl 0x82331dd8
	ctx.lr = 0x8233B650;
	sub_82331DD8(ctx, base);
	// 8233B650: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233B658: 396B19AC  addi r11, r11, 0x19ac
	ctx.r[11].s64 = ctx.r[11].s64 + 6572;
	// 8233B65C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233B66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233B670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233B678 size=184
    let mut pc: u32 = 0x8233B678;
    'dispatch: loop {
        match pc {
            0x8233B678 => {
    //   block [0x8233B678..0x8233B730)
	// 8233B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233B680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233B684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233B688: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8233B68C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233B690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233B694: 486D51DD  bl 0x82a10870
	ctx.lr = 0x8233B698;
	sub_82A10870(ctx, base);
	// 8233B698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B69C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233B6A0: C3EB9534  lfs f31, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233B6A4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8233B6A8: 4BFD76F9  bl 0x82312da0
	ctx.lr = 0x8233B6AC;
	sub_82312DA0(ctx, base);
	// 8233B6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B6B0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8233B6B4: 4BFF3435  bl 0x8232eae8
	ctx.lr = 0x8233B6B8;
	sub_8232EAE8(ctx, base);
	// 8233B6B8: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 8233B6BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B6C0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8233B6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B6C8: 38AB19D8  addi r5, r11, 0x19d8
	ctx.r[5].s64 = ctx.r[11].s64 + 6616;
	// 8233B6CC: 388A6910  addi r4, r10, 0x6910
	ctx.r[4].s64 = ctx.r[10].s64 + 26896;
	// 8233B6D0: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 8233B6D4: 4BFD8465  bl 0x82313b38
	ctx.lr = 0x8233B6D8;
	sub_82313B38(ctx, base);
	// 8233B6D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B6DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233B6E0: 388B1384  addi r4, r11, 0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + 4996;
	// 8233B6E4: 48AB8325  bl 0x82df3a08
	ctx.lr = 0x8233B6E8;
	sub_82DF3A08(ctx, base);
	// 8233B6E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233B6EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B6F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233B6F4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233B6F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233B6FC: 4E800421  bctrl
	ctx.lr = 0x8233B700;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233B700: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233B704: 48AB7D25  bl 0x82df3428
	ctx.lr = 0x8233B708;
	sub_82DF3428(ctx, base);
	// 8233B708: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8233B70C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B710: 4BFD85D9  bl 0x82313ce8
	ctx.lr = 0x8233B714;
	sub_82313CE8(ctx, base);
	// 8233B714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233B718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233B71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233B720: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8233B724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233B728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233B72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233B730 size=96
    let mut pc: u32 = 0x8233B730;
    'dispatch: loop {
        match pc {
            0x8233B730 => {
    //   block [0x8233B730..0x8233B790)
	// 8233B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B734: 48E6CA39  bl 0x831a816c
	ctx.lr = 0x8233B738;
	sub_831A8130(ctx, base);
	// 8233B738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233B73C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233B740: 486D5131  bl 0x82a10870
	ctx.lr = 0x8233B744;
	sub_82A10870(ctx, base);
	// 8233B744: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233B748: 4BFD89A1  bl 0x823140e8
	ctx.lr = 0x8233B74C;
	sub_823140E8(ctx, base);
	// 8233B74C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233B750: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8233B754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B758: 4BFDF301  bl 0x8231aa58
	ctx.lr = 0x8233B75C;
	sub_8231AA58(ctx, base);
	// 8233B75C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B760: 4BFD7681  bl 0x82312de0
	ctx.lr = 0x8233B764;
	sub_82312DE0(ctx, base);
	// 8233B764: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B768: 4BFD21A9  bl 0x8230d910
	ctx.lr = 0x8233B76C;
	sub_8230D910(ctx, base);
	// 8233B76C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8233B770: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B774: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 8233B778: 4BFCA5D9  bl 0x82305d50
	ctx.lr = 0x8233B77C;
	sub_82305D50(ctx, base);
	// 8233B77C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8233B780: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B784: 4BFD856D  bl 0x82313cf0
	ctx.lr = 0x8233B788;
	sub_82313CF0(ctx, base);
	// 8233B788: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233B78C: 48E6CA30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233B790 size=416
    let mut pc: u32 = 0x8233B790;
    'dispatch: loop {
        match pc {
            0x8233B790 => {
    //   block [0x8233B790..0x8233B930)
	// 8233B790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B794: 48E6C9D1  bl 0x831a8164
	ctx.lr = 0x8233B798;
	sub_831A8130(ctx, base);
	// 8233B798: DBA1FFB8  stfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[29].u64 ) };
	// 8233B79C: DBC1FFC0  stfd f30, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 8233B7A0: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8233B7A4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233B7A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233B7AC: 486D50C5  bl 0x82a10870
	ctx.lr = 0x8233B7B0;
	sub_82A10870(ctx, base);
	// 8233B7B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233B7B4: 4BFD8935  bl 0x823140e8
	ctx.lr = 0x8233B7B8;
	sub_823140E8(ctx, base);
	// 8233B7B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233B7BC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8233B7C0: 3B6B19D8  addi r27, r11, 0x19d8
	ctx.r[27].s64 = ctx.r[11].s64 + 6616;
	// 8233B7C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233B7C8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8233B7CC: 388A6910  addi r4, r10, 0x6910
	ctx.r[4].s64 = ctx.r[10].s64 + 26896;
	// 8233B7D0: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 8233B7D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B7D8: 4BFD8361  bl 0x82313b38
	ctx.lr = 0x8233B7DC;
	sub_82313B38(ctx, base);
	// 8233B7DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B7E0: 4BFD1941  bl 0x8230d120
	ctx.lr = 0x8233B7E4;
	sub_8230D120(ctx, base);
	// 8233B7E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233B7E8: 41820048  beq 0x8233b830
	if ctx.cr[0].eq {
	pc = 0x8233B830; continue 'dispatch;
	}
	// 8233B7EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B7F0: 4BFD1A09  bl 0x8230d1f8
	ctx.lr = 0x8233B7F4;
	sub_8230D1F8(ctx, base);
	// 8233B7F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233B7F8: 40820038  bne 0x8233b830
	if !ctx.cr[0].eq {
	pc = 0x8233B830; continue 'dispatch;
	}
	// 8233B7FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B800: 4BFD1A01  bl 0x8230d200
	ctx.lr = 0x8233B804;
	sub_8230D200(ctx, base);
	// 8233B804: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B808: C3FF0080  lfs f31, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233B80C: 4BFD1ADD  bl 0x8230d2e8
	ctx.lr = 0x8233B810;
	sub_8230D2E8(ctx, base);
	// 8233B810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B814: 839F007C  lwz r28, 0x7c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8233B818: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8233B81C: 4BFCAE95  bl 0x823066b0
	ctx.lr = 0x8233B820;
	sub_823066B0(ctx, base);
	// 8233B820: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8233B824: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8233B828: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8233B82C: 48120A1D  bl 0x8245c248
	ctx.lr = 0x8233B830;
	sub_8245C248(ctx, base);
	// 8233B830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233B834: 48B1DF45  bl 0x82e59778
	ctx.lr = 0x8233B838;
	sub_82E59778(ctx, base);
	// 8233B838: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B83C: 4BFD2E75  bl 0x8230e6b0
	ctx.lr = 0x8233B840;
	sub_8230E6B0(ctx, base);
	// 8233B840: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233B844: 418200D8  beq 0x8233b91c
	if ctx.cr[0].eq {
	pc = 0x8233B91C; continue 'dispatch;
	}
	// 8233B848: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B84C: 4BFD18D5  bl 0x8230d120
	ctx.lr = 0x8233B850;
	sub_8230D120(ctx, base);
	// 8233B850: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233B854: 418200B8  beq 0x8233b90c
	if ctx.cr[0].eq {
	pc = 0x8233B90C; continue 'dispatch;
	}
	// 8233B858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233B85C: 4BFD53A5  bl 0x82310c00
	ctx.lr = 0x8233B860;
	sub_82310C00(ctx, base);
	// 8233B860: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233B864: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8233B868: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8233B86C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8233B870: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8233B874: 419A0024  beq cr6, 0x8233b898
	if ctx.cr[6].eq {
	pc = 0x8233B898; continue 'dispatch;
	}
	// 8233B878: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8233B87C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8233B880: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233B884: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8233B888: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233B88C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8233B890: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233B894: 4082FFE8  bne 0x8233b87c
	if !ctx.cr[0].eq {
	pc = 0x8233B87C; continue 'dispatch;
	}
	// 8233B898: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8233B89C: 80DF0068  lwz r6, 0x68(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233B8A0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8233B8A4: 38A00051  li r5, 0x51
	ctx.r[5].s64 = 81;
	// 8233B8A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B8AC: 4BFCC58D  bl 0x82307e38
	ctx.lr = 0x8233B8B0;
	sub_82307E38(ctx, base);
	// 8233B8B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233B8B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233B8B8: C3FC001C  lfs f31, 0x1c(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233B8BC: 837F0084  lwz r27, 0x84(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8233B8C0: C3DF0074  lfs f30, 0x74(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8233B8C4: 839C0018  lwz r28, 0x18(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8233B8C8: C3BF0070  lfs f29, 0x70(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8233B8CC: 83FF0068  lwz r31, 0x68(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233B8D0: 4BFCA1A1  bl 0x82305a70
	ctx.lr = 0x8233B8D4;
	sub_82305A70(ctx, base);
	// 8233B8D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233B8D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B8DC: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8233B8E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233B8E4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8233B8E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8233B8EC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8233B8F0: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8233B8F4: 4BFF3E8D  bl 0x8232f780
	ctx.lr = 0x8233B8F8;
	sub_8232F780(ctx, base);
	// 8233B8F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8233B8FC: 419A0020  beq cr6, 0x8233b91c
	if ctx.cr[6].eq {
	pc = 0x8233B91C; continue 'dispatch;
	}
	// 8233B900: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233B904: 4BF84F8D  bl 0x822c0890
	ctx.lr = 0x8233B908;
	sub_822C0890(ctx, base);
	// 8233B908: 48000014  b 0x8233b91c
	pc = 0x8233B91C; continue 'dispatch;
	// 8233B90C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233B910: C03F0078  lfs f1, 0x78(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233B914: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233B918: 4BFF3691  bl 0x8232efa8
	ctx.lr = 0x8233B91C;
	sub_8232EFA8(ctx, base);
	// 8233B91C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8233B920: CBA1FFB8  lfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8233B924: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8233B928: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8233B92C: 48E6C888  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8233B930 size=1620
    let mut pc: u32 = 0x8233B930;
    'dispatch: loop {
        match pc {
            0x8233B930 => {
    //   block [0x8233B930..0x8233BF84)
	// 8233B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233B934: 48E6C81D  bl 0x831a8150
	ctx.lr = 0x8233B938;
	sub_831A8130(ctx, base);
	// 8233B938: 3981FFA8  addi r12, r1, -0x58
	ctx.r[12].s64 = ctx.r[1].s64 + -88;
	// 8233B93C: 48E6D13D  bl 0x831a8a78
	ctx.lr = 0x8233B940;
	sub_831A8A40(ctx, base);
	// 8233B940: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233BF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233BF88 size=60
    let mut pc: u32 = 0x8233BF88;
    'dispatch: loop {
        match pc {
            0x8233BF88 => {
    //   block [0x8233BF88..0x8233BFC4)
	// 8233BF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233BF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233BF90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233BF94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233BF98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233BF9C: 4BFF5E3D  bl 0x82331dd8
	ctx.lr = 0x8233BFA0;
	sub_82331DD8(ctx, base);
	// 8233BFA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233BFA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233BFA8: 396B1A40  addi r11, r11, 0x1a40
	ctx.r[11].s64 = ctx.r[11].s64 + 6720;
	// 8233BFAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233BFB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233BFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233BFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233BFBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233BFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233BFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233BFC8 size=160
    let mut pc: u32 = 0x8233BFC8;
    'dispatch: loop {
        match pc {
            0x8233BFC8 => {
    //   block [0x8233BFC8..0x8233C068)
	// 8233BFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233BFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233BFD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233BFD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233BFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233BFDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233BFE0: 486D4891  bl 0x82a10870
	ctx.lr = 0x8233BFE4;
	sub_82A10870(ctx, base);
	// 8233BFE4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233BFE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233BFEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233BFF0: 808BB0B4  lwz r4, -0x4f4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20300 as u32) ) } as u64;
	// 8233BFF4: 48AB7A15  bl 0x82df3a08
	ctx.lr = 0x8233BFF8;
	sub_82DF3A08(ctx, base);
	// 8233BFF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233BFFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233C000: 4BFCB8D9  bl 0x823078d8
	ctx.lr = 0x8233C004;
	sub_823078D8(ctx, base);
	// 8233C004: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C008: 48AB7421  bl 0x82df3428
	ctx.lr = 0x8233C00C;
	sub_82DF3428(ctx, base);
	// 8233C00C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C010: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C014: 808BB504  lwz r4, -0x4afc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19196 as u32) ) } as u64;
	// 8233C018: 48AB79F1  bl 0x82df3a08
	ctx.lr = 0x8233C01C;
	sub_82DF3A08(ctx, base);
	// 8233C01C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C020: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233C024: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233C028: 4BFC9511  bl 0x82305538
	ctx.lr = 0x8233C02C;
	sub_82305538(ctx, base);
	// 8233C02C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C030: 48AB73F9  bl 0x82df3428
	ctx.lr = 0x8233C034;
	sub_82DF3428(ctx, base);
	// 8233C034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C038: 4BFD7641  bl 0x82313678
	ctx.lr = 0x8233C03C;
	sub_82313678(ctx, base);
	// 8233C03C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8233C040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C044: 4BFD7CA5  bl 0x82313ce8
	ctx.lr = 0x8233C048;
	sub_82313CE8(ctx, base);
	// 8233C048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C04C: 48D8BA95  bl 0x830c7ae0
	ctx.lr = 0x8233C050;
	sub_830C7AE0(ctx, base);
	// 8233C050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233C054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C05C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233C068 size=132
    let mut pc: u32 = 0x8233C068;
    'dispatch: loop {
        match pc {
            0x8233C068 => {
    //   block [0x8233C068..0x8233C0EC)
	// 8233C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C074: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C07C: 486D47F5  bl 0x82a10870
	ctx.lr = 0x8233C080;
	sub_82A10870(ctx, base);
	// 8233C080: 4BFD6F89  bl 0x82313008
	ctx.lr = 0x8233C084;
	sub_82313008(ctx, base);
	// 8233C084: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C088: 40820048  bne 0x8233c0d0
	if !ctx.cr[0].eq {
	pc = 0x8233C0D0; continue 'dispatch;
	}
	// 8233C08C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C094: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233C098: 48AB7971  bl 0x82df3a08
	ctx.lr = 0x8233C09C;
	sub_82DF3A08(ctx, base);
	// 8233C09C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C0A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233C0A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C0A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233C0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C0B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233C0B4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C0B8: 48B229D9  bl 0x82e5ea90
	ctx.lr = 0x8233C0BC;
	sub_82E5EA90(ctx, base);
	// 8233C0BC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233C0C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233C0C4: 419A0014  beq cr6, 0x8233c0d8
	if ctx.cr[6].eq {
	pc = 0x8233C0D8; continue 'dispatch;
	}
	// 8233C0C8: 4BF847C9  bl 0x822c0890
	ctx.lr = 0x8233C0CC;
	sub_822C0890(ctx, base);
	// 8233C0CC: 4800000C  b 0x8233c0d8
	pc = 0x8233C0D8; continue 'dispatch;
	// 8233C0D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C0D4: 48D8BA0D  bl 0x830c7ae0
	ctx.lr = 0x8233C0D8;
	sub_830C7AE0(ctx, base);
	// 8233C0D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233C0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C0E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C0F0 size=200
    let mut pc: u32 = 0x8233C0F0;
    'dispatch: loop {
        match pc {
            0x8233C0F0 => {
    //   block [0x8233C0F0..0x8233C1B8)
	// 8233C0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C0F4: 48E6C079  bl 0x831a816c
	ctx.lr = 0x8233C0F8;
	sub_831A8130(ctx, base);
	// 8233C0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C0FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C100: 486D4771  bl 0x82a10870
	ctx.lr = 0x8233C104;
	sub_82A10870(ctx, base);
	// 8233C104: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C110: 808BB504  lwz r4, -0x4afc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19196 as u32) ) } as u64;
	// 8233C114: 48AB78F5  bl 0x82df3a08
	ctx.lr = 0x8233C118;
	sub_82DF3A08(ctx, base);
	// 8233C118: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C11C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233C120: 4BFCC001  bl 0x82308120
	ctx.lr = 0x8233C124;
	sub_82308120(ctx, base);
	// 8233C124: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C128: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8233C12C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C130: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8233C134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C138: 4E800421  bctrl
	ctx.lr = 0x8233C13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C13C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233C140: 48AB71C9  bl 0x82df3308
	ctx.lr = 0x8233C144;
	sub_82DF3308(ctx, base);
	// 8233C144: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233C148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233C14C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233C150: 419A000C  beq cr6, 0x8233c15c
	if ctx.cr[6].eq {
	pc = 0x8233C15C; continue 'dispatch;
	}
	// 8233C154: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8233C158: 4BF84739  bl 0x822c0890
	ctx.lr = 0x8233C15C;
	sub_822C0890(ctx, base);
	// 8233C15C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C160: 48AB72C9  bl 0x82df3428
	ctx.lr = 0x8233C164;
	sub_82DF3428(ctx, base);
	// 8233C164: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C168: 4182002C  beq 0x8233c194
	if ctx.cr[0].eq {
	pc = 0x8233C194; continue 'dispatch;
	}
	// 8233C16C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C174: 808BB4EC  lwz r4, -0x4b14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19220 as u32) ) } as u64;
	// 8233C178: 48AB7891  bl 0x82df3a08
	ctx.lr = 0x8233C17C;
	sub_82DF3A08(ctx, base);
	// 8233C17C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8233C180: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233C184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C188: 4BFC93B1  bl 0x82305538
	ctx.lr = 0x8233C18C;
	sub_82305538(ctx, base);
	// 8233C18C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C190: 48AB7299  bl 0x82df3428
	ctx.lr = 0x8233C194;
	sub_82DF3428(ctx, base);
	// 8233C194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C198: 4BFD7531  bl 0x823136c8
	ctx.lr = 0x8233C19C;
	sub_823136C8(ctx, base);
	// 8233C19C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8233C1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C1A4: 4BFD7B4D  bl 0x82313cf0
	ctx.lr = 0x8233C1A8;
	sub_82313CF0(ctx, base);
	// 8233C1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C1AC: 48D8B935  bl 0x830c7ae0
	ctx.lr = 0x8233C1B0;
	sub_830C7AE0(ctx, base);
	// 8233C1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233C1B4: 48E6C008  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C1B8 size=84
    let mut pc: u32 = 0x8233C1B8;
    'dispatch: loop {
        match pc {
            0x8233C1B8 => {
    //   block [0x8233C1B8..0x8233C20C)
	// 8233C1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C1C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C1C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C1CC: 4BFF5C0D  bl 0x82331dd8
	ctx.lr = 0x8233C1D0;
	sub_82331DD8(ctx, base);
	// 8233C1D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C1D8: 396B1A6C  addi r11, r11, 0x1a6c
	ctx.r[11].s64 = ctx.r[11].s64 + 6764;
	// 8233C1DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8233C1E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C1E4: 4BFF59C5  bl 0x82331ba8
	ctx.lr = 0x8233C1E8;
	sub_82331BA8(ctx, base);
	// 8233C1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C1EC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8233C1F0: 4BFF5BD9  bl 0x82331dc8
	ctx.lr = 0x8233C1F4;
	sub_82331DC8(ctx, base);
	// 8233C1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233C1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C210 size=36
    let mut pc: u32 = 0x8233C210;
    'dispatch: loop {
        match pc {
            0x8233C210 => {
    //   block [0x8233C210..0x8233C234)
	// 8233C210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C21C: 486D4655  bl 0x82a10870
	ctx.lr = 0x8233C220;
	sub_82A10870(ctx, base);
	// 8233C220: 4BFD79E1  bl 0x82313c00
	ctx.lr = 0x8233C224;
	sub_82313C00(ctx, base);
	// 8233C224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233C228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C238 size=64
    let mut pc: u32 = 0x8233C238;
    'dispatch: loop {
        match pc {
            0x8233C238 => {
    //   block [0x8233C238..0x8233C278)
	// 8233C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C248: 486D4629  bl 0x82a10870
	ctx.lr = 0x8233C24C;
	sub_82A10870(ctx, base);
	// 8233C24C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8233C250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C254: 4BFD7A9D  bl 0x82313cf0
	ctx.lr = 0x8233C258;
	sub_82313CF0(ctx, base);
	// 8233C258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233C25C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C260: 4BFD7A91  bl 0x82313cf0
	ctx.lr = 0x8233C264;
	sub_82313CF0(ctx, base);
	// 8233C264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233C268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8233C278 size=224
    let mut pc: u32 = 0x8233C278;
    'dispatch: loop {
        match pc {
            0x8233C278 => {
    //   block [0x8233C278..0x8233C358)
	// 8233C278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C27C: 48E6BEF1  bl 0x831a816c
	ctx.lr = 0x8233C280;
	sub_831A8130(ctx, base);
	// 8233C280: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8233C284: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8233C288: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C358 size=80
    let mut pc: u32 = 0x8233C358;
    'dispatch: loop {
        match pc {
            0x8233C358 => {
    //   block [0x8233C358..0x8233C3A8)
	// 8233C358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C35C: 48E6BE0D  bl 0x831a8168
	ctx.lr = 0x8233C360;
	sub_831A8130(ctx, base);
	// 8233C360: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8233C364: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C36C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8233C370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233C374: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8233C378: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8233C37C: 486D44F5  bl 0x82a10870
	ctx.lr = 0x8233C380;
	sub_82A10870(ctx, base);
	// 8233C380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233C384: 4BFCB555  bl 0x823078d8
	ctx.lr = 0x8233C388;
	sub_823078D8(ctx, base);
	// 8233C388: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8233C38C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8233C390: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8233C394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C398: 4BFFFEE1  bl 0x8233c278
	ctx.lr = 0x8233C39C;
	sub_8233C278(ctx, base);
	// 8233C39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233C3A0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8233C3A4: 48E6BE14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233C3A8 size=304
    let mut pc: u32 = 0x8233C3A8;
    'dispatch: loop {
        match pc {
            0x8233C3A8 => {
    //   block [0x8233C3A8..0x8233C4D8)
	// 8233C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233C3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C3B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C3BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C3C0: 486D44B1  bl 0x82a10870
	ctx.lr = 0x8233C3C4;
	sub_82A10870(ctx, base);
	// 8233C3C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C3CC: 48B1D395  bl 0x82e59760
	ctx.lr = 0x8233C3D0;
	sub_82E59760(ctx, base);
	// 8233C3D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8233C3D4: C00B6150  lfs f0, 0x6150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233C3D8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233C3DC: 40990058  ble cr6, 0x8233c434
	if !ctx.cr[6].gt {
	pc = 0x8233C434; continue 'dispatch;
	}
	// 8233C3E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C3E4: 4BFD6825  bl 0x82312c08
	ctx.lr = 0x8233C3E8;
	sub_82312C08(ctx, base);
	// 8233C3E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C3EC: 41820048  beq 0x8233c434
	if ctx.cr[0].eq {
	pc = 0x8233C434; continue 'dispatch;
	}
	// 8233C3F0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C3F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C3F8: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233C3FC: 48AB760D  bl 0x82df3a08
	ctx.lr = 0x8233C400;
	sub_82DF3A08(ctx, base);
	// 8233C400: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C404: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233C408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C40C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233C410: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233C418: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C41C: 48B22675  bl 0x82e5ea90
	ctx.lr = 0x8233C420;
	sub_82E5EA90(ctx, base);
	// 8233C420: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233C424: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233C428: 419A0098  beq cr6, 0x8233c4c0
	if ctx.cr[6].eq {
	pc = 0x8233C4C0; continue 'dispatch;
	}
	// 8233C42C: 4BF84465  bl 0x822c0890
	ctx.lr = 0x8233C430;
	sub_822C0890(ctx, base);
	// 8233C430: 48000090  b 0x8233c4c0
	pc = 0x8233C4C0; continue 'dispatch;
	// 8233C434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C438: 4BFD6A29  bl 0x82312e60
	ctx.lr = 0x8233C43C;
	sub_82312E60(ctx, base);
	// 8233C43C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C440: 4182005C  beq 0x8233c49c
	if ctx.cr[0].eq {
	pc = 0x8233C49C; continue 'dispatch;
	}
	// 8233C444: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233C448: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233C44C: 4BFDD69D  bl 0x82319ae8
	ctx.lr = 0x8233C450;
	sub_82319AE8(ctx, base);
	// 8233C450: C01F0068  lfs f0, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233C454: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8233C458: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233C45C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8233C460: 4098003C  bge cr6, 0x8233c49c
	if !ctx.cr[6].lt {
	pc = 0x8233C49C; continue 'dispatch;
	}
	// 8233C464: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C46C: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 8233C470: 48AB7599  bl 0x82df3a08
	ctx.lr = 0x8233C474;
	sub_82DF3A08(ctx, base);
	// 8233C474: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C478: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233C47C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C480: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233C484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233C48C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C490: 48B22601  bl 0x82e5ea90
	ctx.lr = 0x8233C494;
	sub_82E5EA90(ctx, base);
	// 8233C494: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8233C498: 4BFFFF8C  b 0x8233c424
	pc = 0x8233C424; continue 'dispatch;
	// 8233C49C: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8233C4A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233C4A4: 4182001C  beq 0x8233c4c0
	if ctx.cr[0].eq {
	pc = 0x8233C4C0; continue 'dispatch;
	}
	// 8233C4A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C4AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C4B4: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 8233C4B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C4BC: 4E800421  bctrl
	ctx.lr = 0x8233C4C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C4C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8233C4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C4CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C4D8 size=68
    let mut pc: u32 = 0x8233C4D8;
    'dispatch: loop {
        match pc {
            0x8233C4D8 => {
    //   block [0x8233C4D8..0x8233C51C)
	// 8233C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C4E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C4E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C4E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C4EC: 4BFF58ED  bl 0x82331dd8
	ctx.lr = 0x8233C4F0;
	sub_82331DD8(ctx, base);
	// 8233C4F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C4F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8233C4F8: 396B1A98  addi r11, r11, 0x1a98
	ctx.r[11].s64 = ctx.r[11].s64 + 6808;
	// 8233C4FC: 995F006C  stb r10, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u8 ) };
	// 8233C500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C504: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233C50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233C520 size=132
    let mut pc: u32 = 0x8233C520;
    'dispatch: loop {
        match pc {
            0x8233C520 => {
    //   block [0x8233C520..0x8233C5A4)
	// 8233C520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233C52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C530: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C538: 486D4339  bl 0x82a10870
	ctx.lr = 0x8233C53C;
	sub_82A10870(ctx, base);
	// 8233C53C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C540: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C548: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 8233C54C: 48AB74BD  bl 0x82df3a08
	ctx.lr = 0x8233C550;
	sub_82DF3A08(ctx, base);
	// 8233C550: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C554: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8233C558: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8233C55C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233C560: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233C564: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C568: 4BFBC789  bl 0x822f8cf0
	ctx.lr = 0x8233C56C;
	sub_822F8CF0(ctx, base);
	// 8233C56C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233C570: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233C574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C578: 4BFCB3C1  bl 0x82307938
	ctx.lr = 0x8233C57C;
	sub_82307938(ctx, base);
	// 8233C57C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C580: 48AB6EA9  bl 0x82df3428
	ctx.lr = 0x8233C584;
	sub_82DF3428(ctx, base);
	// 8233C584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C588: 48D8B559  bl 0x830c7ae0
	ctx.lr = 0x8233C58C;
	sub_830C7AE0(ctx, base);
	// 8233C58C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8233C590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C598: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C5A8 size=96
    let mut pc: u32 = 0x8233C5A8;
    'dispatch: loop {
        match pc {
            0x8233C5A8 => {
    //   block [0x8233C5A8..0x8233C608)
	// 8233C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C5B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C5B8: 48E18049  bl 0x83154600
	ctx.lr = 0x8233C5BC;
	sub_83154600(ctx, base);
	// 8233C5BC: 486D42B5  bl 0x82a10870
	ctx.lr = 0x8233C5C0;
	sub_82A10870(ctx, base);
	// 8233C5C0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C5C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C5C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C5CC: 808BB164  lwz r4, -0x4e9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20124 as u32) ) } as u64;
	// 8233C5D0: 48AB7439  bl 0x82df3a08
	ctx.lr = 0x8233C5D4;
	sub_82DF3A08(ctx, base);
	// 8233C5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C5D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233C5DC: 4BFCB2FD  bl 0x823078d8
	ctx.lr = 0x8233C5E0;
	sub_823078D8(ctx, base);
	// 8233C5E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C5E4: 48AB6E45  bl 0x82df3428
	ctx.lr = 0x8233C5E8;
	sub_82DF3428(ctx, base);
	// 8233C5E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233C5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C5F0: 4BFE1E39  bl 0x8231e428
	ctx.lr = 0x8233C5F4;
	sub_8231E428(ctx, base);
	// 8233C5F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233C5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C608 size=196
    let mut pc: u32 = 0x8233C608;
    'dispatch: loop {
        match pc {
            0x8233C608 => {
    //   block [0x8233C608..0x8233C6CC)
	// 8233C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233C614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C61C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233C624: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233C628: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233C62C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C630: 4BF84309  bl 0x822c0938
	ctx.lr = 0x8233C634;
	sub_822C0938(ctx, base);
	// 8233C634: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233C638: 41820028  beq 0x8233c660
	if ctx.cr[0].eq {
	pc = 0x8233C660; continue 'dispatch;
	}
	// 8233C63C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C640: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233C644: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233C648: 392B1AC4  addi r9, r11, 0x1ac4
	ctx.r[9].s64 = ctx.r[11].s64 + 6852;
	// 8233C64C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233C650: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233C654: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233C658: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233C65C: 48000008  b 0x8233c664
	pc = 0x8233C664; continue 'dispatch;
	// 8233C660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233C664: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233C66C: 409A0044  bne cr6, 0x8233c6b0
	if !ctx.cr[6].eq {
	pc = 0x8233C6B0; continue 'dispatch;
	}
	// 8233C670: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233C674: 419A001C  beq cr6, 0x8233c690
	if ctx.cr[6].eq {
	pc = 0x8233C690; continue 'dispatch;
	}
	// 8233C678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C67C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233C680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C684: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233C688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C68C: 4E800421  bctrl
	ctx.lr = 0x8233C690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C690: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C694: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233C698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C69C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233C6A0: 816B9448  lwz r11, -0x6bb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27576 as u32) ) } as u64;
	// 8233C6A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233C6A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233C6AC: 4BF83955  bl 0x822c0000
	ctx.lr = 0x8233C6B0;
	sub_822C0000(ctx, base);
	// 8233C6B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C6B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233C6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C6C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C6C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233C6D0 size=196
    let mut pc: u32 = 0x8233C6D0;
    'dispatch: loop {
        match pc {
            0x8233C6D0 => {
    //   block [0x8233C6D0..0x8233C794)
	// 8233C6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C6D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233C6DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C6E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C6E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233C6EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233C6F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233C6F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C6F8: 4BF84241  bl 0x822c0938
	ctx.lr = 0x8233C6FC;
	sub_822C0938(ctx, base);
	// 8233C6FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233C700: 41820028  beq 0x8233c728
	if ctx.cr[0].eq {
	pc = 0x8233C728; continue 'dispatch;
	}
	// 8233C704: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C708: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233C70C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233C710: 392B1AD8  addi r9, r11, 0x1ad8
	ctx.r[9].s64 = ctx.r[11].s64 + 6872;
	// 8233C714: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233C718: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233C71C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233C720: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233C724: 48000008  b 0x8233c72c
	pc = 0x8233C72C; continue 'dispatch;
	// 8233C728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233C72C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233C734: 409A0044  bne cr6, 0x8233c778
	if !ctx.cr[6].eq {
	pc = 0x8233C778; continue 'dispatch;
	}
	// 8233C738: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233C73C: 419A001C  beq cr6, 0x8233c758
	if ctx.cr[6].eq {
	pc = 0x8233C758; continue 'dispatch;
	}
	// 8233C740: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C744: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233C748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C74C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233C750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C754: 4E800421  bctrl
	ctx.lr = 0x8233C758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C758: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C75C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233C760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C764: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233C768: 816B9448  lwz r11, -0x6bb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27576 as u32) ) } as u64;
	// 8233C76C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233C770: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233C774: 4BF8388D  bl 0x822c0000
	ctx.lr = 0x8233C778;
	sub_822C0000(ctx, base);
	// 8233C778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C77C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233C780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C788: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C78C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233C798 size=224
    let mut pc: u32 = 0x8233C798;
    'dispatch: loop {
        match pc {
            0x8233C798 => {
    //   block [0x8233C798..0x8233C878)
	// 8233C798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233C7A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233C7A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233C7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C7B0: 486D40C1  bl 0x82a10870
	ctx.lr = 0x8233C7B4;
	sub_82A10870(ctx, base);
	// 8233C7B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C7B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C7BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C7C0: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8233C7C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C7C8: 4E800421  bctrl
	ctx.lr = 0x8233C7CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C7CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C7D0: 40820090  bne 0x8233c860
	if !ctx.cr[0].eq {
	pc = 0x8233C860; continue 'dispatch;
	}
	// 8233C7D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C7DC: 4BFD863D  bl 0x82314e18
	ctx.lr = 0x8233C7E0;
	sub_82314E18(ctx, base);
	// 8233C7E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C7E4: 4082007C  bne 0x8233c860
	if !ctx.cr[0].eq {
	pc = 0x8233C860; continue 'dispatch;
	}
	// 8233C7E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C7EC: 4BFDA7F5  bl 0x82316fe0
	ctx.lr = 0x8233C7F0;
	sub_82316FE0(ctx, base);
	// 8233C7F0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C7F4: C00B9444  lfs f0, -0x6bbc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233C7F8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233C7FC: 40990048  ble cr6, 0x8233c844
	if !ctx.cr[6].gt {
	pc = 0x8233C844; continue 'dispatch;
	}
	// 8233C800: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C804: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C808: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233C80C: 48AB71FD  bl 0x82df3a08
	ctx.lr = 0x8233C810;
	sub_82DF3A08(ctx, base);
	// 8233C810: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C814: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233C818: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C81C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233C820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233C828: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C82C: 48B22265  bl 0x82e5ea90
	ctx.lr = 0x8233C830;
	sub_82E5EA90(ctx, base);
	// 8233C830: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233C834: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233C838: 419A0028  beq cr6, 0x8233c860
	if ctx.cr[6].eq {
	pc = 0x8233C860; continue 'dispatch;
	}
	// 8233C83C: 4BF84055  bl 0x822c0890
	ctx.lr = 0x8233C840;
	sub_822C0890(ctx, base);
	// 8233C840: 48000020  b 0x8233c860
	pc = 0x8233C860; continue 'dispatch;
	// 8233C844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C848: 48B1CF21  bl 0x82e59768
	ctx.lr = 0x8233C84C;
	sub_82E59768(ctx, base);
	// 8233C84C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233C850: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8233C854: 48B20625  bl 0x82e5ce78
	ctx.lr = 0x8233C858;
	sub_82E5CE78(ctx, base);
	// 8233C858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C85C: 48D8B285  bl 0x830c7ae0
	ctx.lr = 0x8233C860;
	sub_830C7AE0(ctx, base);
	// 8233C860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233C864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233C868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233C86C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233C870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233C874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233C878 size=628
    let mut pc: u32 = 0x8233C878;
    'dispatch: loop {
        match pc {
            0x8233C878 => {
    //   block [0x8233C878..0x8233CAEC)
	// 8233C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233C87C: 48E6B8F1  bl 0x831a816c
	ctx.lr = 0x8233C880;
	sub_831A8130(ctx, base);
	// 8233C880: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233C884: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233C888: 48E17D79  bl 0x83154600
	ctx.lr = 0x8233C88C;
	sub_83154600(ctx, base);
	// 8233C88C: 486D3FE5  bl 0x82a10870
	ctx.lr = 0x8233C890;
	sub_82A10870(ctx, base);
	// 8233C890: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233C894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233C898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C89C: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 8233C8A0: 48AB7169  bl 0x82df3a08
	ctx.lr = 0x8233C8A4;
	sub_82DF3A08(ctx, base);
	// 8233C8A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C8A8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8233C8AC: 4BFC8B55  bl 0x82305400
	ctx.lr = 0x8233C8B0;
	sub_82305400(ctx, base);
	// 8233C8B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8233C8B4: 48AB6A55  bl 0x82df3308
	ctx.lr = 0x8233C8B8;
	sub_82DF3308(ctx, base);
	// 8233C8B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233C8BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233C8C0: 48AB6B69  bl 0x82df3428
	ctx.lr = 0x8233C8C4;
	sub_82DF3428(ctx, base);
	// 8233C8C4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C8C8: 41820018  beq 0x8233c8e0
	if ctx.cr[0].eq {
	pc = 0x8233C8E0; continue 'dispatch;
	}
	// 8233C8CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C8D0: 48B1CEA9  bl 0x82e59778
	ctx.lr = 0x8233C8D4;
	sub_82E59778(ctx, base);
	// 8233C8D4: C01E0060  lfs f0, 0x60(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233C8D8: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8233C8DC: D01E0060  stfs f0, 0x60(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8233C8E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233C8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C8E8: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8233C8EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233C8F0: 4E800421  bctrl
	ctx.lr = 0x8233C8F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233C8F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C8F8: 408201EC  bne 0x8233cae4
	if !ctx.cr[0].eq {
	pc = 0x8233CAE4; continue 'dispatch;
	}
	// 8233C8FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C900: 4BFDBA51  bl 0x82318350
	ctx.lr = 0x8233C904;
	sub_82318350(ctx, base);
	// 8233C904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233C908: 408201DC  bne 0x8233cae4
	if !ctx.cr[0].eq {
	pc = 0x8233CAE4; continue 'dispatch;
	}
	// 8233C90C: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 8233C910: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233C914: 4811E4D5  bl 0x8245ade8
	ctx.lr = 0x8233C918;
	sub_8245ADE8(ctx, base);
	// 8233C918: C01E0060  lfs f0, 0x60(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233C91C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8233C920: 409901C4  ble cr6, 0x8233cae4
	if !ctx.cr[6].gt {
	pc = 0x8233CAE4; continue 'dispatch;
	}
	// 8233C924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233C928: 48E17CD9  bl 0x83154600
	ctx.lr = 0x8233C92C;
	sub_83154600(ctx, base);
	// 8233C92C: 817F01FC  lwz r11, 0x1fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8233C930: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233C934: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 8233C938: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8233C93C: 4811E4FD  bl 0x8245ae38
	ctx.lr = 0x8233C940;
	sub_8245AE38(ctx, base);
	// 8233C940: 817D00C8  lwz r11, 0xc8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 8233C944: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8233C948: 419800EC  blt cr6, 0x8233ca34
	if ctx.cr[6].lt {
	pc = 0x8233CA34; continue 'dispatch;
	}
	// 8233C94C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C950: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C954: 388B1B38  addi r4, r11, 0x1b38
	ctx.r[4].s64 = ctx.r[11].s64 + 6968;
	// 8233C958: 38A000A9  li r5, 0xa9
	ctx.r[5].s64 = 169;
	// 8233C95C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 8233C960: 48AB5A89  bl 0x82df23e8
	ctx.lr = 0x8233C964;
	sub_82DF23E8(ctx, base);
	// 8233C964: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8233C968: 4182001C  beq 0x8233c984
	if ctx.cr[0].eq {
	pc = 0x8233C984; continue 'dispatch;
	}
	// 8233C96C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233C970: 48B1FFB9  bl 0x82e5c928
	ctx.lr = 0x8233C974;
	sub_82E5C928(ctx, base);
	// 8233C974: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233C978: 396B1B14  addi r11, r11, 0x1b14
	ctx.r[11].s64 = ctx.r[11].s64 + 6932;
	// 8233C97C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233C980: 48000008  b 0x8233c988
	pc = 0x8233C988; continue 'dispatch;
	// 8233C984: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8233C988: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8233C98C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C990: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233C994: 4BFFFD3D  bl 0x8233c6d0
	ctx.lr = 0x8233C998;
	sub_8233C6D0(ctx, base);
	// 8233C998: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233C99C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233C9A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233C9A4: 4BF8365D  bl 0x822c0000
	ctx.lr = 0x8233C9A8;
	sub_822C0000(ctx, base);
	// 8233C9A8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233C9AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8233C9B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233C9B4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8233C9B8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8233C9BC: 419A0024  beq cr6, 0x8233c9e0
	if ctx.cr[6].eq {
	pc = 0x8233C9E0; continue 'dispatch;
	}
	// 8233C9C0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8233C9C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8233C9C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233C9CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8233C9D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233C9D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8233C9D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233C9DC: 4082FFE8  bne 0x8233c9c4
	if !ctx.cr[0].eq {
	pc = 0x8233C9C4; continue 'dispatch;
	}
	// 8233C9E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233C9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233C9E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233C9EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8233C9F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233C9F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8233C9F8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233C9FC: 48B21EC5  bl 0x82e5e8c0
	ctx.lr = 0x8233CA00;
	sub_82E5E8C0(ctx, base);
	// 8233CA00: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8233CA04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233CA08: 419A0008  beq cr6, 0x8233ca10
	if ctx.cr[6].eq {
	pc = 0x8233CA10; continue 'dispatch;
	}
	// 8233CA0C: 4BF83E85  bl 0x822c0890
	ctx.lr = 0x8233CA10;
	sub_822C0890(ctx, base);
	// 8233CA10: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233CA14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233CA18: 419A0008  beq cr6, 0x8233ca20
	if ctx.cr[6].eq {
	pc = 0x8233CA20; continue 'dispatch;
	}
	// 8233CA1C: 4BF83E75  bl 0x822c0890
	ctx.lr = 0x8233CA20;
	sub_822C0890(ctx, base);
	// 8233CA20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233CA24: 419A00C0  beq cr6, 0x8233cae4
	if ctx.cr[6].eq {
	pc = 0x8233CAE4; continue 'dispatch;
	}
	// 8233CA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CA2C: 4BF83E65  bl 0x822c0890
	ctx.lr = 0x8233CA30;
	sub_822C0890(ctx, base);
	// 8233CA30: 480000B4  b 0x8233cae4
	pc = 0x8233CAE4; continue 'dispatch;
	// 8233CA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CA38: 4BFC8CA9  bl 0x823056e0
	ctx.lr = 0x8233CA3C;
	sub_823056E0(ctx, base);
	// 8233CA3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8233CA40: C00BE830  lfs f0, -0x17d0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CA44: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8233CA48: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8233CA4C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8233CA50: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8233CA54: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8233CA58: 40980010  bge cr6, 0x8233ca68
	if !ctx.cr[6].lt {
	pc = 0x8233CA68; continue 'dispatch;
	}
	// 8233CA5C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CA60: 808BB154  lwz r4, -0x4eac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20140 as u32) ) } as u64;
	// 8233CA64: 48000034  b 0x8233ca98
	pc = 0x8233CA98; continue 'dispatch;
	// 8233CA68: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8233CA6C: 40980010  bge cr6, 0x8233ca7c
	if !ctx.cr[6].lt {
	pc = 0x8233CA7C; continue 'dispatch;
	}
	// 8233CA70: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CA74: 808BB158  lwz r4, -0x4ea8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20136 as u32) ) } as u64;
	// 8233CA78: 48000020  b 0x8233ca98
	pc = 0x8233CA98; continue 'dispatch;
	// 8233CA7C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8233CA80: 40980010  bge cr6, 0x8233ca90
	if !ctx.cr[6].lt {
	pc = 0x8233CA90; continue 'dispatch;
	}
	// 8233CA84: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CA88: 808BB15C  lwz r4, -0x4ea4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20132 as u32) ) } as u64;
	// 8233CA8C: 4800000C  b 0x8233ca98
	pc = 0x8233CA98; continue 'dispatch;
	// 8233CA90: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CA94: 808BB160  lwz r4, -0x4ea0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20128 as u32) ) } as u64;
	// 8233CA98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CA9C: 48AB6F6D  bl 0x82df3a08
	ctx.lr = 0x8233CAA0;
	sub_82DF3A08(ctx, base);
	// 8233CAA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233CAA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CAA8: 4BFCAE31  bl 0x823078d8
	ctx.lr = 0x8233CAAC;
	sub_823078D8(ctx, base);
	// 8233CAAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CAB0: 48AB6979  bl 0x82df3428
	ctx.lr = 0x8233CAB4;
	sub_82DF3428(ctx, base);
	// 8233CAB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CAB8: 48E17B49  bl 0x83154600
	ctx.lr = 0x8233CABC;
	sub_83154600(ctx, base);
	// 8233CABC: 394300C8  addi r10, r3, 0xc8
	ctx.r[10].s64 = ctx.r[3].s64 + 200;
	// 8233CAC0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8233CAC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8233CAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CACC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233CAD0: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CAD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8233CAD8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233CADC: D01E0060  stfs f0, 0x60(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8233CAE0: 4BFE1949  bl 0x8231e428
	ctx.lr = 0x8233CAE4;
	sub_8231E428(ctx, base);
	// 8233CAE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8233CAE8: 48E6B6D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233CAF0 size=852
    let mut pc: u32 = 0x8233CAF0;
    'dispatch: loop {
        match pc {
            0x8233CAF0 => {
    //   block [0x8233CAF0..0x8233CE44)
	// 8233CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CAF4: 48E6B671  bl 0x831a8164
	ctx.lr = 0x8233CAF8;
	sub_831A8130(ctx, base);
	// 8233CAF8: DBA1FFB8  stfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[29].u64 ) };
	// 8233CAFC: DBC1FFC0  stfd f30, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 8233CB00: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8233CB04: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CB08: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8233CB0C: 486D3D65  bl 0x82a10870
	ctx.lr = 0x8233CB10;
	sub_82A10870(ctx, base);
	// 8233CB10: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8233CB14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CB18: 4BFE00C1  bl 0x8231cbd8
	ctx.lr = 0x8233CB1C;
	sub_8231CBD8(ctx, base);
	// 8233CB1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233CB20: 40820310  bne 0x8233ce30
	if !ctx.cr[0].eq {
	pc = 0x8233CE30; continue 'dispatch;
	}
	// 8233CB24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8233CB28: 48D8AFB9  bl 0x830c7ae0
	ctx.lr = 0x8233CB2C;
	sub_830C7AE0(ctx, base);
	// 8233CB2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233CB30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CB34: 917B00C8  stw r11, 0xc8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8233CB38: 4BFDA4A9  bl 0x82316fe0
	ctx.lr = 0x8233CB3C;
	sub_82316FE0(ctx, base);
	// 8233CB3C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CB40: C00B9444  lfs f0, -0x6bbc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CB44: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233CB48: 40990044  ble cr6, 0x8233cb8c
	if !ctx.cr[6].gt {
	pc = 0x8233CB8C; continue 'dispatch;
	}
	// 8233CB4C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CB50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CB54: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233CB58: 48AB6EB1  bl 0x82df3a08
	ctx.lr = 0x8233CB5C;
	sub_82DF3A08(ctx, base);
	// 8233CB5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233CB60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8233CB64: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8233CB68: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8233CB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233CB70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233CB74: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233CB78: 48B21F19  bl 0x82e5ea90
	ctx.lr = 0x8233CB7C;
	sub_82E5EA90(ctx, base);
	// 8233CB7C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8233CB80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233CB84: 419A02AC  beq cr6, 0x8233ce30
	if ctx.cr[6].eq {
	pc = 0x8233CE30; continue 'dispatch;
	}
	// 8233CB88: 480002A4  b 0x8233ce2c
	pc = 0x8233CE2C; continue 'dispatch;
	// 8233CB8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233CB90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233CB94: 388B12FC  addi r4, r11, 0x12fc
	ctx.r[4].s64 = ctx.r[11].s64 + 4860;
	// 8233CB98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CB9C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8233CBA0: 48AB6E69  bl 0x82df3a08
	ctx.lr = 0x8233CBA4;
	sub_82DF3A08(ctx, base);
	// 8233CBA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8233CBA8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8233CBAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8233CBB0: 4BFF7629  bl 0x823341d8
	ctx.lr = 0x8233CBB4;
	sub_823341D8(ctx, base);
	// 8233CBB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CBB8: 48AB6871  bl 0x82df3428
	ctx.lr = 0x8233CBBC;
	sub_82DF3428(ctx, base);
	// 8233CBBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233CBC0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8233CBC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233CBC8: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8233CBCC: 41820168  beq 0x8233cd34
	if ctx.cr[0].eq {
	pc = 0x8233CD34; continue 'dispatch;
	}
	// 8233CBD0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CBD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CBD8: 808BB01C  lwz r4, -0x4fe4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20452 as u32) ) } as u64;
	// 8233CBDC: 48AB6E2D  bl 0x82df3a08
	ctx.lr = 0x8233CBE0;
	sub_82DF3A08(ctx, base);
	// 8233CBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CBE4: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 8233CBE8: 4BFC8819  bl 0x82305400
	ctx.lr = 0x8233CBEC;
	sub_82305400(ctx, base);
	// 8233CBEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233CBF0: 48AB6719  bl 0x82df3308
	ctx.lr = 0x8233CBF4;
	sub_82DF3308(ctx, base);
	// 8233CBF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233CBF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CBFC: 48AB682D  bl 0x82df3428
	ctx.lr = 0x8233CC00;
	sub_82DF3428(ctx, base);
	// 8233CC00: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233CC04: 4182010C  beq 0x8233cd10
	if ctx.cr[0].eq {
	pc = 0x8233CD10; continue 'dispatch;
	}
	// 8233CC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CC0C: 4BFC88A5  bl 0x823054b0
	ctx.lr = 0x8233CC10;
	sub_823054B0(ctx, base);
	// 8233CC10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233CC14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CC18: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8233CC1C: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233CC20: 4BFC8859  bl 0x82305478
	ctx.lr = 0x8233CC24;
	sub_82305478(ctx, base);
	// 8233CC24: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8233CC28: D3E10080  stfs f31, 0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8233CC2C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8233CC30: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8233CC34: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8233CC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233CC3C: C1AA9C28  lfs f13, -0x63d8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25560 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233CC40: 3BC10088  addi r30, r1, 0x88
	ctx.r[30].s64 = ctx.r[1].s64 + 136;
	// 8233CC44: C1899534  lfs f12, -0x6acc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8233CC48: 3B800003  li r28, 3
	ctx.r[28].s64 = 3;
	// 8233CC4C: C008D72C  lfs f0, -0x28d4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10452 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CC50: 3FA08326  lis r29, -0x7cda
	ctx.r[29].s64 = -2094661632;
	// 8233CC54: C167F904  lfs f11, -0x6fc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-1788 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8233CC58: D1A10084  stfs f13, 0x84(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8233CC5C: D1810088  stfs f12, 0x88(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 8233CC60: D1A1008C  stfs f13, 0x8c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8233CC64: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8233CC68: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8233CC6C: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8233CC70: D161009C  stfs f11, 0x9c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8233CC74: D18100A0  stfs f12, 0xa0(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8233CC78: C01EFFF8  lfs f0, -8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CC7C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8233CC80: 41990060  bgt cr6, 0x8233cce0
	if ctx.cr[6].gt {
	pc = 0x8233CCE0; continue 'dispatch;
	}
	// 8233CC84: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CC88: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8233CC8C: 40980054  bge cr6, 0x8233cce0
	if !ctx.cr[6].lt {
	pc = 0x8233CCE0; continue 'dispatch;
	}
	// 8233CC90: C3BE0000  lfs f29, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8233CC94: 809DB018  lwz r4, -0x4fe8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 8233CC98: FF1DF800  fcmpu cr6, f29, f31
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 8233CC9C: 40990024  ble cr6, 0x8233ccc0
	if !ctx.cr[6].gt {
	pc = 0x8233CCC0; continue 'dispatch;
	}
	// 8233CCA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CCA4: 48AB6D65  bl 0x82df3a08
	ctx.lr = 0x8233CCA8;
	sub_82DF3A08(ctx, base);
	// 8233CCA8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8233CCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CCB0: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8233CCB4: 4BFCADA5  bl 0x82307a58
	ctx.lr = 0x8233CCB8;
	sub_82307A58(ctx, base);
	// 8233CCB8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8233CCBC: 4800001C  b 0x8233ccd8
	pc = 0x8233CCD8; continue 'dispatch;
	// 8233CCC0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CCC4: 48AB6D45  bl 0x82df3a08
	ctx.lr = 0x8233CCC8;
	sub_82DF3A08(ctx, base);
	// 8233CCC8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8233CCCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CCD0: 4BFCAC09  bl 0x823078d8
	ctx.lr = 0x8233CCD4;
	sub_823078D8(ctx, base);
	// 8233CCD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CCD8: 48AB6751  bl 0x82df3428
	ctx.lr = 0x8233CCDC;
	sub_82DF3428(ctx, base);
	// 8233CCDC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8233CCE0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8233CCE4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 8233CCE8: 4082FF90  bne 0x8233cc78
	if !ctx.cr[0].eq {
	pc = 0x8233CC78; continue 'dispatch;
	}
	// 8233CCEC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233CCF0: 40820044  bne 0x8233cd34
	if !ctx.cr[0].eq {
	pc = 0x8233CD34; continue 'dispatch;
	}
	// 8233CCF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CCF8: 809DB018  lwz r4, -0x4fe8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 8233CCFC: 48AB6D0D  bl 0x82df3a08
	ctx.lr = 0x8233CD00;
	sub_82DF3A08(ctx, base);
	// 8233CD00: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8233CD04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CD08: 4BFCACF1  bl 0x823079f8
	ctx.lr = 0x8233CD0C;
	sub_823079F8(ctx, base);
	// 8233CD0C: 48000020  b 0x8233cd2c
	pc = 0x8233CD2C; continue 'dispatch;
	// 8233CD10: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CD14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CD18: 808BB018  lwz r4, -0x4fe8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20456 as u32) ) } as u64;
	// 8233CD1C: 48AB6CED  bl 0x82df3a08
	ctx.lr = 0x8233CD20;
	sub_82DF3A08(ctx, base);
	// 8233CD20: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8233CD24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CD28: 4BFCABB1  bl 0x823078d8
	ctx.lr = 0x8233CD2C;
	sub_823078D8(ctx, base);
	// 8233CD2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8233CD30: 48AB66F9  bl 0x82df3428
	ctx.lr = 0x8233CD34;
	sub_82DF3428(ctx, base);
	// 8233CD34: 817B0068  lwz r11, 0x68(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(104 as u32) ) } as u64;
	// 8233CD38: 3BDB0068  addi r30, r27, 0x68
	ctx.r[30].s64 = ctx.r[27].s64 + 104;
	// 8233CD3C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8233CD40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CD44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8233CD48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233CD4C: 4E800421  bctrl
	ctx.lr = 0x8233CD50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233CD50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233CD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233CD58: 388B1B38  addi r4, r11, 0x1b38
	ctx.r[4].s64 = ctx.r[11].s64 + 6968;
	// 8233CD5C: 38A00052  li r5, 0x52
	ctx.r[5].s64 = 82;
	// 8233CD60: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8233CD64: 48AB5685  bl 0x82df23e8
	ctx.lr = 0x8233CD68;
	sub_82DF23E8(ctx, base);
	// 8233CD68: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8233CD6C: 4182001C  beq 0x8233cd88
	if ctx.cr[0].eq {
	pc = 0x8233CD88; continue 'dispatch;
	}
	// 8233CD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CD74: 48B1FBB5  bl 0x82e5c928
	ctx.lr = 0x8233CD78;
	sub_82E5C928(ctx, base);
	// 8233CD78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233CD7C: 396B1AEC  addi r11, r11, 0x1aec
	ctx.r[11].s64 = ctx.r[11].s64 + 6892;
	// 8233CD80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233CD84: 48000008  b 0x8233cd8c
	pc = 0x8233CD8C; continue 'dispatch;
	// 8233CD88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8233CD8C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8233CD90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233CD94: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8233CD98: 4BFFF871  bl 0x8233c608
	ctx.lr = 0x8233CD9C;
	sub_8233C608(ctx, base);
	// 8233CD9C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8233CDA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233CDA4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8233CDA8: 4BF83259  bl 0x822c0000
	ctx.lr = 0x8233CDAC;
	sub_822C0000(ctx, base);
	// 8233CDAC: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8233CDB0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8233CDB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233CDB8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8233CDBC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8233CDC0: 419A0024  beq cr6, 0x8233cde4
	if ctx.cr[6].eq {
	pc = 0x8233CDE4; continue 'dispatch;
	}
	// 8233CDC4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8233CDC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8233CDCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233CDD0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8233CDD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8233CDD8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8233CDDC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8233CDE0: 4082FFE8  bne 0x8233cdc8
	if !ctx.cr[0].eq {
	pc = 0x8233CDC8; continue 'dispatch;
	}
	// 8233CDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8233CDE8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8233CDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8233CDF0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8233CDF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8233CDF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8233CDFC: 48B218AD  bl 0x82e5e6a8
	ctx.lr = 0x8233CE00;
	sub_82E5E6A8(ctx, base);
	// 8233CE00: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8233CE04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233CE08: 419A0008  beq cr6, 0x8233ce10
	if ctx.cr[6].eq {
	pc = 0x8233CE10; continue 'dispatch;
	}
	// 8233CE0C: 4BF83A85  bl 0x822c0890
	ctx.lr = 0x8233CE10;
	sub_822C0890(ctx, base);
	// 8233CE10: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8233CE14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8233CE18: 419A0008  beq cr6, 0x8233ce20
	if ctx.cr[6].eq {
	pc = 0x8233CE20; continue 'dispatch;
	}
	// 8233CE1C: 4BF83A75  bl 0x822c0890
	ctx.lr = 0x8233CE20;
	sub_822C0890(ctx, base);
	// 8233CE20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233CE24: 419A000C  beq cr6, 0x8233ce30
	if ctx.cr[6].eq {
	pc = 0x8233CE30; continue 'dispatch;
	}
	// 8233CE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CE2C: 4BF83A65  bl 0x822c0890
	ctx.lr = 0x8233CE30;
	sub_822C0890(ctx, base);
	// 8233CE30: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8233CE34: CBA1FFB8  lfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8233CE38: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8233CE3C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8233CE40: 48E6B374  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233CE48 size=168
    let mut pc: u32 = 0x8233CE48;
    'dispatch: loop {
        match pc {
            0x8233CE48 => {
    //   block [0x8233CE48..0x8233CEF0)
	// 8233CE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233CE50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233CE54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233CE58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CE5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CE60: 486D3A11  bl 0x82a10870
	ctx.lr = 0x8233CE64;
	sub_82A10870(ctx, base);
	// 8233CE64: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233CE68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233CE6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CE70: 808BB090  lwz r4, -0x4f70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20336 as u32) ) } as u64;
	// 8233CE74: 48AB6B95  bl 0x82df3a08
	ctx.lr = 0x8233CE78;
	sub_82DF3A08(ctx, base);
	// 8233CE78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CE7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233CE80: 4BFCAA59  bl 0x823078d8
	ctx.lr = 0x8233CE84;
	sub_823078D8(ctx, base);
	// 8233CE84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CE88: 48AB65A1  bl 0x82df3428
	ctx.lr = 0x8233CE8C;
	sub_82DF3428(ctx, base);
	// 8233CE8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CE90: 4BFD6D71  bl 0x82313c00
	ctx.lr = 0x8233CE94;
	sub_82313C00(ctx, base);
	// 8233CE94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CE98: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 8233CE9C: 4BFD6E4D  bl 0x82313ce8
	ctx.lr = 0x8233CEA0;
	sub_82313CE8(ctx, base);
	// 8233CEA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233CEA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CEA8: 388B1918  addi r4, r11, 0x1918
	ctx.r[4].s64 = ctx.r[11].s64 + 6424;
	// 8233CEAC: 48AB6B5D  bl 0x82df3a08
	ctx.lr = 0x8233CEB0;
	sub_82DF3A08(ctx, base);
	// 8233CEB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CEB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233CEB8: 4BFC8E29  bl 0x82305ce0
	ctx.lr = 0x8233CEBC;
	sub_82305CE0(ctx, base);
	// 8233CEBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233CEC0: 48AB6569  bl 0x82df3428
	ctx.lr = 0x8233CEC4;
	sub_82DF3428(ctx, base);
	// 8233CEC4: 39602710  li r11, 0x2710
	ctx.r[11].s64 = 10000;
	// 8233CEC8: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8233CECC: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8233CED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CED4: 4BFD6E15  bl 0x82313ce8
	ctx.lr = 0x8233CED8;
	sub_82313CE8(ctx, base);
	// 8233CED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233CEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233CEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233CEE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233CEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233CEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233CEF0 size=56
    let mut pc: u32 = 0x8233CEF0;
    'dispatch: loop {
        match pc {
            0x8233CEF0 => {
    //   block [0x8233CEF0..0x8233CF28)
	// 8233CEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233CEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233CEFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CF00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CF04: 48B1C865  bl 0x82e59768
	ctx.lr = 0x8233CF08;
	sub_82E59768(ctx, base);
	// 8233CF08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8233CF0C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8233CF10: 48B1FF69  bl 0x82e5ce78
	ctx.lr = 0x8233CF14;
	sub_82E5CE78(ctx, base);
	// 8233CF14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233CF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233CF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233CF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233CF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233CF28 size=68
    let mut pc: u32 = 0x8233CF28;
    'dispatch: loop {
        match pc {
            0x8233CF28 => {
    //   block [0x8233CF28..0x8233CF6C)
	// 8233CF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233CF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233CF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CF38: 486D3939  bl 0x82a10870
	ctx.lr = 0x8233CF3C;
	sub_82A10870(ctx, base);
	// 8233CF3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233CF40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CF44: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8233CF48: 4BFE0F99  bl 0x8231dee0
	ctx.lr = 0x8233CF4C;
	sub_8231DEE0(ctx, base);
	// 8233CF4C: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8233CF50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CF54: 4BFD6D9D  bl 0x82313cf0
	ctx.lr = 0x8233CF58;
	sub_82313CF0(ctx, base);
	// 8233CF58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233CF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233CF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233CF64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233CF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233CF70 size=72
    let mut pc: u32 = 0x8233CF70;
    'dispatch: loop {
        match pc {
            0x8233CF70 => {
    //   block [0x8233CF70..0x8233CFB8)
	// 8233CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233CF78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233CF7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CF80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CF84: 48B1F9A5  bl 0x82e5c928
	ctx.lr = 0x8233CF88;
	sub_82E5C928(ctx, base);
	// 8233CF88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233CF8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8233CF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CF94: 394A1C04  addi r10, r10, 0x1c04
	ctx.r[10].s64 = ctx.r[10].s64 + 7172;
	// 8233CF98: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233CF9C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8233CFA0: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8233CFA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8233CFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233CFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233CFB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233CFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233CFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233CFB8 size=88
    let mut pc: u32 = 0x8233CFB8;
    'dispatch: loop {
        match pc {
            0x8233CFB8 => {
    //   block [0x8233CFB8..0x8233D010)
	// 8233CFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233CFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233CFC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233CFC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233CFC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233CFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233CFD0: 48E17631  bl 0x83154600
	ctx.lr = 0x8233CFD4;
	sub_83154600(ctx, base);
	// 8233CFD4: 486D389D  bl 0x82a10870
	ctx.lr = 0x8233CFD8;
	sub_82A10870(ctx, base);
	// 8233CFD8: 4BFD71D9  bl 0x823141b0
	ctx.lr = 0x8233CFDC;
	sub_823141B0(ctx, base);
	// 8233CFDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233CFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233CFE4: 48E1761D  bl 0x83154600
	ctx.lr = 0x8233CFE8;
	sub_83154600(ctx, base);
	// 8233CFE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233CFEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233CFF0: 808B00B4  lwz r4, 0xb4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8233CFF4: 4BFF3B6D  bl 0x82330b60
	ctx.lr = 0x8233CFF8;
	sub_82330B60(ctx, base);
	// 8233CFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233CFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233D000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233D004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233D008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233D00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233D010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233D010 size=76
    let mut pc: u32 = 0x8233D010;
    'dispatch: loop {
        match pc {
            0x8233D010 => {
    //   block [0x8233D010..0x8233D05C)
	// 8233D010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233D014: 48E6B159  bl 0x831a816c
	ctx.lr = 0x8233D018;
	sub_831A8130(ctx, base);
	// 8233D018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233D01C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233D020: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8233D024: 48E175DD  bl 0x83154600
	ctx.lr = 0x8233D028;
	sub_83154600(ctx, base);
	// 8233D028: 486D3849  bl 0x82a10870
	ctx.lr = 0x8233D02C;
	sub_82A10870(ctx, base);
	// 8233D02C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233D030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D034: 48E175CD  bl 0x83154600
	ctx.lr = 0x8233D038;
	sub_83154600(ctx, base);
	// 8233D038: 8143006C  lwz r10, 0x6c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8233D03C: 1D7E001C  mulli r11, r30, 0x1c
	ctx.r[11].s64 = ctx.r[30].s64 * 28;
	// 8233D040: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233D044: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8233D048: 4BFD7169  bl 0x823141b0
	ctx.lr = 0x8233D04C;
	sub_823141B0(ctx, base);
	// 8233D04C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8233D050: 4BFF4699  bl 0x823316e8
	ctx.lr = 0x8233D054;
	sub_823316E8(ctx, base);
	// 8233D054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233D058: 48E6B164  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8233D060 size=236
    let mut pc: u32 = 0x8233D060;
    'dispatch: loop {
        match pc {
            0x8233D060 => {
    //   block [0x8233D060..0x8233D14C)
	// 8233D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233D064: 48E6B105  bl 0x831a8168
	ctx.lr = 0x8233D068;
	sub_831A8130(ctx, base);
	// 8233D068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233D06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8233D070: 48E17591  bl 0x83154600
	ctx.lr = 0x8233D074;
	sub_83154600(ctx, base);
	// 8233D074: 486D37FD  bl 0x82a10870
	ctx.lr = 0x8233D078;
	sub_82A10870(ctx, base);
	// 8233D078: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8233D07C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D080: 48E17581  bl 0x83154600
	ctx.lr = 0x8233D084;
	sub_83154600(ctx, base);
	// 8233D084: 486D37ED  bl 0x82a10870
	ctx.lr = 0x8233D088;
	sub_82A10870(ctx, base);
	// 8233D088: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8233D08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D090: 48B1C6D1  bl 0x82e59760
	ctx.lr = 0x8233D094;
	sub_82E59760(ctx, base);
	// 8233D094: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233D098: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8233D09C: 3BCB1B9C  addi r30, r11, 0x1b9c
	ctx.r[30].s64 = ctx.r[11].s64 + 7068;
	// 8233D0A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233D0A4: C1AA89AC  lfs f13, -0x7654(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233D0A8: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233D0AC: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8233D0B0: C18B08A8  lfs f12, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8233D0B4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8233D0B8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8233D0BC: 4099000C  ble cr6, 0x8233d0c8
	if !ctx.cr[6].gt {
	pc = 0x8233D0C8; continue 'dispatch;
	}
	// 8233D0C0: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 8233D0C4: 48000018  b 0x8233d0dc
	pc = 0x8233D0DC; continue 'dispatch;
	// 8233D0C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8233D0CC: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233D0D0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8233D0D4: 40980008  bge cr6, 0x8233d0dc
	if !ctx.cr[6].lt {
	pc = 0x8233D0DC; continue 'dispatch;
	}
	// 8233D0D8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8233D0DC: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8233D0E0: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8233D0E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233D0E8: EC20637A  fmadds f1, f0, f13, f12
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 8233D0EC: 4BFE0DF5  bl 0x8231dee0
	ctx.lr = 0x8233D0F0;
	sub_8231DEE0(ctx, base);
	// 8233D0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D0F4: 48B1C66D  bl 0x82e59760
	ctx.lr = 0x8233D0F8;
	sub_82E59760(ctx, base);
	// 8233D0F8: C01EFFFC  lfs f0, -4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8233D0FC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8233D100: 40990010  ble cr6, 0x8233d110
	if !ctx.cr[6].gt {
	pc = 0x8233D110; continue 'dispatch;
	}
	// 8233D104: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233D108: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 8233D10C: 4800001C  b 0x8233d128
	pc = 0x8233D128; continue 'dispatch;
	// 8233D110: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8233D114: 4BFD5AF5  bl 0x82312c08
	ctx.lr = 0x8233D118;
	sub_82312C08(ctx, base);
	// 8233D118: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8233D11C: 41820028  beq 0x8233d144
	if ctx.cr[0].eq {
	pc = 0x8233D144; continue 'dispatch;
	}
	// 8233D120: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233D124: 808BB390  lwz r4, -0x4c70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19568 as u32) ) } as u64;
	// 8233D128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233D12C: 48AB68DD  bl 0x82df3a08
	ctx.lr = 0x8233D130;
	sub_82DF3A08(ctx, base);
	// 8233D130: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8233D134: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8233D138: 4BFC8459  bl 0x82305590
	ctx.lr = 0x8233D13C;
	sub_82305590(ctx, base);
	// 8233D13C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233D140: 48AB62E9  bl 0x82df3428
	ctx.lr = 0x8233D144;
	sub_82DF3428(ctx, base);
	// 8233D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8233D148: 48E6B070  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233D150 size=196
    let mut pc: u32 = 0x8233D150;
    'dispatch: loop {
        match pc {
            0x8233D150 => {
    //   block [0x8233D150..0x8233D214)
	// 8233D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233D154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233D158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233D15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233D160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233D164: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233D168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233D16C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233D170: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233D174: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233D178: 4BF837C1  bl 0x822c0938
	ctx.lr = 0x8233D17C;
	sub_822C0938(ctx, base);
	// 8233D17C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233D180: 41820028  beq 0x8233d1a8
	if ctx.cr[0].eq {
	pc = 0x8233D1A8; continue 'dispatch;
	}
	// 8233D184: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233D188: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233D18C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233D190: 392B1BB4  addi r9, r11, 0x1bb4
	ctx.r[9].s64 = ctx.r[11].s64 + 7092;
	// 8233D194: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233D198: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233D19C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233D1A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233D1A4: 48000008  b 0x8233d1ac
	pc = 0x8233D1AC; continue 'dispatch;
	// 8233D1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233D1AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233D1B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233D1B4: 409A0044  bne cr6, 0x8233d1f8
	if !ctx.cr[6].eq {
	pc = 0x8233D1F8; continue 'dispatch;
	}
	// 8233D1B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233D1BC: 419A001C  beq cr6, 0x8233d1d8
	if ctx.cr[6].eq {
	pc = 0x8233D1D8; continue 'dispatch;
	}
	// 8233D1C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233D1C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233D1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D1CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233D1D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233D1D4: 4E800421  bctrl
	ctx.lr = 0x8233D1D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233D1D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233D1DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233D1E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233D1E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233D1E8: 816B95DC  lwz r11, -0x6a24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27172 as u32) ) } as u64;
	// 8233D1EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233D1F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233D1F4: 4BF82E0D  bl 0x822c0000
	ctx.lr = 0x8233D1F8;
	sub_822C0000(ctx, base);
	// 8233D1F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233D1FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233D200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233D204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233D208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233D20C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233D210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8233D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8233D218 size=196
    let mut pc: u32 = 0x8233D218;
    'dispatch: loop {
        match pc {
            0x8233D218 => {
    //   block [0x8233D218..0x8233D2DC)
	// 8233D218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8233D21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8233D220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8233D224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8233D228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8233D22C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8233D230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233D234: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8233D238: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8233D23C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233D240: 4BF836F9  bl 0x822c0938
	ctx.lr = 0x8233D244;
	sub_822C0938(ctx, base);
	// 8233D244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8233D248: 41820028  beq 0x8233d270
	if ctx.cr[0].eq {
	pc = 0x8233D270; continue 'dispatch;
	}
	// 8233D24C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8233D250: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8233D254: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8233D258: 392B1BC8  addi r9, r11, 0x1bc8
	ctx.r[9].s64 = ctx.r[11].s64 + 7112;
	// 8233D25C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8233D260: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8233D264: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8233D268: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8233D26C: 48000008  b 0x8233d274
	pc = 0x8233D274; continue 'dispatch;
	// 8233D270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8233D274: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8233D278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8233D27C: 409A0044  bne cr6, 0x8233d2c0
	if !ctx.cr[6].eq {
	pc = 0x8233D2C0; continue 'dispatch;
	}
	// 8233D280: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8233D284: 419A001C  beq cr6, 0x8233d2a0
	if ctx.cr[6].eq {
	pc = 0x8233D2A0; continue 'dispatch;
	}
	// 8233D288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8233D28C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8233D290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8233D294: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8233D298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8233D29C: 4E800421  bctrl
	ctx.lr = 0x8233D2A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8233D2A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8233D2A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8233D2A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8233D2AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8233D2B0: 816B95DC  lwz r11, -0x6a24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27172 as u32) ) } as u64;
	// 8233D2B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8233D2B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8233D2BC: 4BF82D45  bl 0x822c0000
	ctx.lr = 0x8233D2C0;
	sub_822C0000(ctx, base);
	// 8233D2C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8233D2C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8233D2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8233D2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8233D2D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8233D2D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8233D2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


