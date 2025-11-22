pub fn sub_82EF2708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF2708 size=904
    let mut pc: u32 = 0x82EF2708;
    'dispatch: loop {
        match pc {
            0x82EF2708 => {
    //   block [0x82EF2708..0x82EF2A90)
	// 82EF2708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF2714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2718: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF271C: 90610134  stw r3, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[3].u32 ) };
	// 82EF2720: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2724: 896B0488  lbz r11, 0x488(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1160 as u32) ) } as u64;
	// 82EF2728: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF272C: 41820030  beq 0x82ef275c
	if ctx.cr[0].eq {
	pc = 0x82EF275C; continue 'dispatch;
	}
	// 82EF2730: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2734: 4BFFFE95  bl 0x82ef25c8
	ctx.lr = 0x82EF2738;
	sub_82EF25C8(ctx, base);
	// 82EF2738: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF273C: 388BCB84  addi r4, r11, -0x347c
	ctx.r[4].s64 = ctx.r[11].s64 + -13436;
	// 82EF2740: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2744: 4BFFF15D  bl 0x82ef18a0
	ctx.lr = 0x82EF2748;
	sub_82EF18A0(ctx, base);
	// 82EF2748: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF274C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF2750: 994B0488  stb r10, 0x488(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1160 as u32), ctx.r[10].u8 ) };
	// 82EF2754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2758: 48000320  b 0x82ef2a78
	pc = 0x82EF2A78; continue 'dispatch;
	// 82EF275C: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2760: 896B0489  lbz r11, 0x489(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1161 as u32) ) } as u64;
	// 82EF2764: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF2768: 41820030  beq 0x82ef2798
	if ctx.cr[0].eq {
	pc = 0x82EF2798; continue 'dispatch;
	}
	// 82EF276C: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2770: 4BFFFE59  bl 0x82ef25c8
	ctx.lr = 0x82EF2774;
	sub_82EF25C8(ctx, base);
	// 82EF2774: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2778: 388B0058  addi r4, r11, 0x58
	ctx.r[4].s64 = ctx.r[11].s64 + 88;
	// 82EF277C: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2780: 4BFFF121  bl 0x82ef18a0
	ctx.lr = 0x82EF2784;
	sub_82EF18A0(ctx, base);
	// 82EF2784: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF278C: 994B0489  stb r10, 0x489(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1161 as u32), ctx.r[10].u8 ) };
	// 82EF2790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2794: 480002E4  b 0x82ef2a78
	pc = 0x82EF2A78; continue 'dispatch;
	// 82EF2798: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF279C: 896B0030  lbz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF27A0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF27A4: 40820014  bne 0x82ef27b8
	if !ctx.cr[0].eq {
	pc = 0x82EF27B8; continue 'dispatch;
	}
	// 82EF27A8: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF27AC: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF27B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF27B4: 409A000C  bne cr6, 0x82ef27c0
	if !ctx.cr[6].eq {
	pc = 0x82EF27C0; continue 'dispatch;
	}
	// 82EF27B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF27BC: 480002BC  b 0x82ef2a78
	pc = 0x82EF2A78; continue 'dispatch;
	// 82EF27C0: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF27C4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF27C8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF27CC: 4B3738E5  bl 0x822660b0
	ctx.lr = 0x82EF27D0;
	sub_822660B0(ctx, base);
	// 82EF27D0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82EF27D4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF27D8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF27DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF27E0: 4098001C  bge cr6, 0x82ef27fc
	if !ctx.cr[6].lt {
	pc = 0x82EF27FC; continue 'dispatch;
	}
	// 82EF27E4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF27E8: 216BFFFF  subfic r11, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[11].s64 = (-1 as i64) - ctx.r[11].s64;
	// 82EF27EC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF27F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF27F4: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82EF27F8: 48000014  b 0x82ef280c
	pc = 0x82EF280C; continue 'dispatch;
	// 82EF27FC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF2800: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF2804: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF2808: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82EF280C: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82EF2810: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF2814: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF2818: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82EF281C: 4098000C  bge cr6, 0x82ef2828
	if !ctx.cr[6].lt {
	pc = 0x82EF2828; continue 'dispatch;
	}
	// 82EF2820: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF2824: 48000254  b 0x82ef2a78
	pc = 0x82EF2A78; continue 'dispatch;
	// 82EF2828: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF282C: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF2830: F96100D8  std r11, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[11].u64 ) };
	// 82EF2834: C80100D8  lfd f0, 0xd8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) };
	// 82EF2838: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EF283C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EF2840: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82EF2844: C1AB9404  lfs f13, -0x6bfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF2848: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82EF284C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82EF2850: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2854: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82EF2858: 2B0B00FE  cmplwi cr6, r11, 0xfe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 254 as u32, &mut ctx.xer);
	// 82EF285C: 419A0028  beq cr6, 0x82ef2884
	if ctx.cr[6].eq {
	pc = 0x82EF2884; continue 'dispatch;
	}
	// 82EF2860: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2864: 808B0494  lwz r4, 0x494(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82EF2868: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF286C: 386B0498  addi r3, r11, 0x498
	ctx.r[3].s64 = ctx.r[11].s64 + 1176;
	// 82EF2870: 481FFA49  bl 0x830f22b8
	ctx.lr = 0x82EF2874;
	sub_830F22B8(ctx, base);
	// 82EF2874: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2878: 388B0498  addi r4, r11, 0x498
	ctx.r[4].s64 = ctx.r[11].s64 + 1176;
	// 82EF287C: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2880: 4BFFEDD1  bl 0x82ef1650
	ctx.lr = 0x82EF2884;
	sub_82EF1650(ctx, base);
	// 82EF2884: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2888: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF288C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2890: 419A01B8  beq cr6, 0x82ef2a48
	if ctx.cr[6].eq {
	pc = 0x82EF2A48; continue 'dispatch;
	}
	// 82EF2894: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2898: 808B0494  lwz r4, 0x494(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82EF289C: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF28A0: 806B04D8  lwz r3, 0x4d8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF28A4: 4BFFD8AD  bl 0x82ef0150
	ctx.lr = 0x82EF28A8;
	sub_82EF0150(ctx, base);
	// 82EF28A8: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF28AC: 806B04D8  lwz r3, 0x4d8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF28B0: 4BFFDA61  bl 0x82ef0310
	ctx.lr = 0x82EF28B4;
	sub_82EF0310(ctx, base);
	// 82EF28B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF28B8: 41820190  beq 0x82ef2a48
	if ctx.cr[0].eq {
	pc = 0x82EF2A48; continue 'dispatch;
	}
	// 82EF28BC: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF28C0: 896B04DC  lbz r11, 0x4dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1244 as u32) ) } as u64;
	// 82EF28C4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF28C8: 40820180  bne 0x82ef2a48
	if !ctx.cr[0].eq {
	pc = 0x82EF2A48; continue 'dispatch;
	}
	// 82EF28CC: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF28D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF28D4: 994B04DC  stb r10, 0x4dc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1244 as u32), ctx.r[10].u8 ) };
	// 82EF28D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF28DC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EF28E0: 48000010  b 0x82ef28f0
	pc = 0x82EF28F0; continue 'dispatch;
	// 82EF28E4: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF28E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF28EC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EF28F0: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF28F4: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF28F8: 386B016C  addi r3, r11, 0x16c
	ctx.r[3].s64 = ctx.r[11].s64 + 364;
	// 82EF28FC: 4800134D  bl 0x82ef3c48
	ctx.lr = 0x82EF2900;
	sub_82EF3C48(ctx, base);
	// 82EF2900: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF2904: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF2908: 409800F4  bge cr6, 0x82ef29fc
	if !ctx.cr[6].lt {
	pc = 0x82EF29FC; continue 'dispatch;
	}
	// 82EF290C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF2910: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2914: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF2918: 386B016C  addi r3, r11, 0x16c
	ctx.r[3].s64 = ctx.r[11].s64 + 364;
	// 82EF291C: 48001375  bl 0x82ef3c90
	ctx.lr = 0x82EF2920;
	sub_82EF3C90(ctx, base);
	// 82EF2920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2924: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82EF2928: 3D6082EF  lis r11, -0x7d11
	ctx.r[11].s64 = -2098266112;
	// 82EF292C: 38CB5780  addi r6, r11, 0x5780
	ctx.r[6].s64 = ctx.r[11].s64 + 22400;
	// 82EF2930: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82EF2934: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF2938: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82EF293C: 4B32D1CD  bl 0x8221fb08
	ctx.lr = 0x82EF2940;
	sub_8221FB08(ctx, base);
	// 82EF2940: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF2944: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF2948: F96100E0  std r11, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[11].u64 ) };
	// 82EF294C: C80100E0  lfd f0, 0xe0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) };
	// 82EF2950: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 82EF2954: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82EF2958: 48002E91  bl 0x82ef57e8
	ctx.lr = 0x82EF295C;
	sub_82EF57E8(ctx, base);
	// 82EF295C: 806100D0  lwz r3, 0xd0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EF2960: 4BFF5BD9  bl 0x82ee8538
	ctx.lr = 0x82EF2964;
	sub_82EE8538(ctx, base);
	// 82EF2964: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF2968: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82EF296C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EF2970: 48002EA1  bl 0x82ef5810
	ctx.lr = 0x82EF2974;
	sub_82EF5810(ctx, base);
	// 82EF2974: 806100D0  lwz r3, 0xd0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EF2978: 4BFF5CB9  bl 0x82ee8630
	ctx.lr = 0x82EF297C;
	sub_82EE8630(ctx, base);
	// 82EF297C: F86100E8  std r3, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[3].u64 ) };
	// 82EF2980: C80100E8  lfd f0, 0xe8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) };
	// 82EF2984: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 82EF2988: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82EF298C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82EF2990: 48002E59  bl 0x82ef57e8
	ctx.lr = 0x82EF2994;
	sub_82EF57E8(ctx, base);
	// 82EF2994: 806100D0  lwz r3, 0xd0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EF2998: 4BFF5E69  bl 0x82ee8800
	ctx.lr = 0x82EF299C;
	sub_82EE8800(ctx, base);
	// 82EF299C: F86100F0  std r3, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[3].u64 ) };
	// 82EF29A0: C80100F0  lfd f0, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) };
	// 82EF29A4: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 82EF29A8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82EF29AC: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82EF29B0: 48002E39  bl 0x82ef57e8
	ctx.lr = 0x82EF29B4;
	sub_82EF57E8(ctx, base);
	// 82EF29B4: 806100D0  lwz r3, 0xd0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EF29B8: 4BFFF361  bl 0x82ef1d18
	ctx.lr = 0x82EF29BC;
	sub_82EF1D18(ctx, base);
	// 82EF29BC: F86100F8  std r3, 0xf8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[3].u64 ) };
	// 82EF29C0: C80100F8  lfd f0, 0xf8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) };
	// 82EF29C4: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 82EF29C8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82EF29CC: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82EF29D0: 48002E19  bl 0x82ef57e8
	ctx.lr = 0x82EF29D4;
	sub_82EF57E8(ctx, base);
	// 82EF29D4: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 82EF29D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF29DC: 3BCBCB64  addi r30, r11, -0x349c
	ctx.r[30].s64 = ctx.r[11].s64 + -13468;
	// 82EF29E0: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF29E4: 48002F55  bl 0x82ef5938
	ctx.lr = 0x82EF29E8;
	sub_82EF5938(ctx, base);
	// 82EF29E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF29EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF29F0: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82EF29F4: 48002E45  bl 0x82ef5838
	ctx.lr = 0x82EF29F8;
	sub_82EF5838(ctx, base);
	// 82EF29F8: 4BFFFEEC  b 0x82ef28e4
	pc = 0x82EF28E4; continue 'dispatch;
	// 82EF29FC: 3D6082EF  lis r11, -0x7d11
	ctx.r[11].s64 = -2098266112;
	// 82EF2A00: 38CB5780  addi r6, r11, 0x5780
	ctx.r[6].s64 = ctx.r[11].s64 + 22400;
	// 82EF2A04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF2A08: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF2A0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EF2A10: 4B32D0F9  bl 0x8221fb08
	ctx.lr = 0x82EF2A14;
	sub_8221FB08(ctx, base);
	// 82EF2A14: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2A18: 48002F21  bl 0x82ef5938
	ctx.lr = 0x82EF2A1C;
	sub_82EF5938(ctx, base);
	// 82EF2A1C: 90610100  stw r3, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[3].u32 ) };
	// 82EF2A20: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF2A24: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EF2A28: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2A2C: 388BCB48  addi r4, r11, -0x34b8
	ctx.r[4].s64 = ctx.r[11].s64 + -13496;
	// 82EF2A30: 80610100  lwz r3, 0x100(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 82EF2A34: 81610100  lwz r11, 0x100(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 82EF2A38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2A3C: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF2A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF2A44: 4E800421  bctrl
	ctx.lr = 0x82EF2A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF2A48: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2A4C: 48002EED  bl 0x82ef5938
	ctx.lr = 0x82EF2A50;
	sub_82EF5938(ctx, base);
	// 82EF2A50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF2A54: 41820014  beq 0x82ef2a68
	if ctx.cr[0].eq {
	pc = 0x82EF2A68; continue 'dispatch;
	}
	// 82EF2A58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2A5C: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF2A60: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2A64: 480FB98D  bl 0x82fee3f0
	ctx.lr = 0x82EF2A68;
	sub_82FEE3F0(ctx, base);
	// 82EF2A68: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2A6C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF2A70: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82EF2A74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2A78: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82EF2A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2A84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF2A88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF2A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF2A90 size=44
    let mut pc: u32 = 0x82EF2A90;
    'dispatch: loop {
        match pc {
            0x82EF2A90 => {
    //   block [0x82EF2A90..0x82EF2ABC)
	// 82EF2A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2A9C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF2AA0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF2AA4: 480FBA05  bl 0x82fee4a8
	ctx.lr = 0x82EF2AA8;
	sub_82FEE4A8(ctx, base);
	// 82EF2AA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF2AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF2AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF2AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF2AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF2AC0 size=3148
    let mut pc: u32 = 0x82EF2AC0;
    'dispatch: loop {
        match pc {
            0x82EF2AC0 => {
    //   block [0x82EF2AC0..0x82EF370C)
	// 82EF2AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF2AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF2AC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF2ACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF2AD0: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF2AD4: 906101B4  stw r3, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[3].u32 ) };
	// 82EF2AD8: 908101BC  stw r4, 0x1bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), ctx.r[4].u32 ) };
	// 82EF2ADC: 90A101C4  stw r5, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[5].u32 ) };
	// 82EF2AE0: 90C101CC  stw r6, 0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[6].u32 ) };
	// 82EF2AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2AE8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2AEC: 388BCCC8  addi r4, r11, -0x3338
	ctx.r[4].s64 = ctx.r[11].s64 + -13112;
	// 82EF2AF0: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2AF4: 48001E6D  bl 0x82ef4960
	ctx.lr = 0x82EF2AF8;
	sub_82EF4960(ctx, base);
	// 82EF2AF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2AFC: 41820014  beq 0x82ef2b10
	if ctx.cr[0].eq {
	pc = 0x82EF2B10; continue 'dispatch;
	}
	// 82EF2B00: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2B04: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2B08: 48000C09  bl 0x82ef3710
	ctx.lr = 0x82EF2B0C;
	sub_82EF3710(ctx, base);
	// 82EF2B0C: 48000BE8  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2B10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2B14: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82EF2B18: 388B29C8  addi r4, r11, 0x29c8
	ctx.r[4].s64 = ctx.r[11].s64 + 10696;
	// 82EF2B1C: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2B20: 48001E41  bl 0x82ef4960
	ctx.lr = 0x82EF2B24;
	sub_82EF4960(ctx, base);
	// 82EF2B24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2B28: 41820014  beq 0x82ef2b3c
	if ctx.cr[0].eq {
	pc = 0x82EF2B3C; continue 'dispatch;
	}
	// 82EF2B2C: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2B30: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2B34: 48000CDD  bl 0x82ef3810
	ctx.lr = 0x82EF2B38;
	sub_82EF3810(ctx, base);
	// 82EF2B38: 48000BBC  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2B40: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2B44: 388BCCC0  addi r4, r11, -0x3340
	ctx.r[4].s64 = ctx.r[11].s64 + -13120;
	// 82EF2B48: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2B4C: 48001E15  bl 0x82ef4960
	ctx.lr = 0x82EF2B50;
	sub_82EF4960(ctx, base);
	// 82EF2B50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2B54: 41820010  beq 0x82ef2b64
	if ctx.cr[0].eq {
	pc = 0x82EF2B64; continue 'dispatch;
	}
	// 82EF2B58: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2B5C: 48000D9D  bl 0x82ef38f8
	ctx.lr = 0x82EF2B60;
	sub_82EF38F8(ctx, base);
	// 82EF2B60: 48000B94  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2B68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2B6C: 388BCCB4  addi r4, r11, -0x334c
	ctx.r[4].s64 = ctx.r[11].s64 + -13132;
	// 82EF2B70: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2B74: 48001DED  bl 0x82ef4960
	ctx.lr = 0x82EF2B78;
	sub_82EF4960(ctx, base);
	// 82EF2B78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2B7C: 41820010  beq 0x82ef2b8c
	if ctx.cr[0].eq {
	pc = 0x82EF2B8C; continue 'dispatch;
	}
	// 82EF2B80: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2B84: 48000EE5  bl 0x82ef3a68
	ctx.lr = 0x82EF2B88;
	sub_82EF3A68(ctx, base);
	// 82EF2B88: 48000B6C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2B90: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2B94: 388BCCAC  addi r4, r11, -0x3354
	ctx.r[4].s64 = ctx.r[11].s64 + -13140;
	// 82EF2B98: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2B9C: 48001DC5  bl 0x82ef4960
	ctx.lr = 0x82EF2BA0;
	sub_82EF4960(ctx, base);
	// 82EF2BA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2BA4: 41820008  beq 0x82ef2bac
	if ctx.cr[0].eq {
	pc = 0x82EF2BAC; continue 'dispatch;
	}
	// 82EF2BA8: 48000B4C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2BB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2BB4: 388BCC94  addi r4, r11, -0x336c
	ctx.r[4].s64 = ctx.r[11].s64 + -13164;
	// 82EF2BB8: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2BBC: 48001DA5  bl 0x82ef4960
	ctx.lr = 0x82EF2BC0;
	sub_82EF4960(ctx, base);
	// 82EF2BC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2BC4: 418200FC  beq 0x82ef2cc0
	if ctx.cr[0].eq {
	pc = 0x82EF2CC0; continue 'dispatch;
	}
	// 82EF2BC8: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82EF2BCC: 91610130  stw r11, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82EF2BD0: 81610130  lwz r11, 0x130(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 82EF2BD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF2BD8: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF2BDC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2BE0: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82EF2BE4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF2BE8: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2BEC: 4BDB9FBD  bl 0x82cacba8
	ctx.lr = 0x82EF2BF0;
	sub_82CACBA8(ctx, base);
	// 82EF2BF0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF2BF4: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82EF2BF8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF2BFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2C00: 4BDB9FA9  bl 0x82cacba8
	ctx.lr = 0x82EF2C04;
	sub_82CACBA8(ctx, base);
	// 82EF2C04: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF2C08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EF2C0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF2C10: 48002679  bl 0x82ef5288
	ctx.lr = 0x82EF2C14;
	sub_82EF5288(ctx, base);
	// 82EF2C14: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82EF2C18: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF2C1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2C20: 4BDB9F89  bl 0x82cacba8
	ctx.lr = 0x82EF2C24;
	sub_82CACBA8(ctx, base);
	// 82EF2C24: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF2C28: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82EF2C2C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF2C30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2C34: 4BDB9F75  bl 0x82cacba8
	ctx.lr = 0x82EF2C38;
	sub_82CACBA8(ctx, base);
	// 82EF2C38: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF2C3C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF2C40: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF2C44: 48002645  bl 0x82ef5288
	ctx.lr = 0x82EF2C48;
	sub_82EF5288(ctx, base);
	// 82EF2C48: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF2C4C: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82EF2C50: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2C54: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EF2C58: 4BFF59E9  bl 0x82ee8640
	ctx.lr = 0x82EF2C5C;
	sub_82EE8640(ctx, base);
	// 82EF2C5C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF2C60: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82EF2C64: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2C68: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EF2C6C: 4BFF5BA5  bl 0x82ee8810
	ctx.lr = 0x82EF2C70;
	sub_82EE8810(ctx, base);
	// 82EF2C70: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2C74: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2C78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2C7C: 419A0034  beq cr6, 0x82ef2cb0
	if ctx.cr[6].eq {
	pc = 0x82EF2CB0; continue 'dispatch;
	}
	// 82EF2C80: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF2C84: 7D6507B4  extsw r5, r11
	ctx.r[5].s64 = ctx.r[11].s32 as i64;
	// 82EF2C88: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF2C8C: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82EF2C90: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2C94: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2C98: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2C9C: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2CA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2CA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF2CAC: 4E800421  bctrl
	ctx.lr = 0x82EF2CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF2CB0: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2CB4: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2CB8: 48000A59  bl 0x82ef3710
	ctx.lr = 0x82EF2CBC;
	sub_82EF3710(ctx, base);
	// 82EF2CBC: 48000A38  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2CC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2CC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2CC8: 388BCC80  addi r4, r11, -0x3380
	ctx.r[4].s64 = ctx.r[11].s64 + -13184;
	// 82EF2CCC: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2CD0: 48001C91  bl 0x82ef4960
	ctx.lr = 0x82EF2CD4;
	sub_82EF4960(ctx, base);
	// 82EF2CD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2CD8: 41820054  beq 0x82ef2d2c
	if ctx.cr[0].eq {
	pc = 0x82EF2D2C; continue 'dispatch;
	}
	// 82EF2CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF2CE4: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82EF2CE8: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2CEC: 4800259D  bl 0x82ef5288
	ctx.lr = 0x82EF2CF0;
	sub_82EF5288(ctx, base);
	// 82EF2CF0: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2CF4: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2CFC: 419A002C  beq cr6, 0x82ef2d28
	if ctx.cr[6].eq {
	pc = 0x82EF2D28; continue 'dispatch;
	}
	// 82EF2D00: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EF2D04: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82EF2D08: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2D0C: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2D10: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2D14: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2D18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2D1C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EF2D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF2D24: 4E800421  bctrl
	ctx.lr = 0x82EF2D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF2D28: 480009CC  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2D30: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2D34: 388BCC6C  addi r4, r11, -0x3394
	ctx.r[4].s64 = ctx.r[11].s64 + -13204;
	// 82EF2D38: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2D3C: 48001C25  bl 0x82ef4960
	ctx.lr = 0x82EF2D40;
	sub_82EF4960(ctx, base);
	// 82EF2D40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2D44: 418201A0  beq 0x82ef2ee4
	if ctx.cr[0].eq {
	pc = 0x82EF2EE4; continue 'dispatch;
	}
	// 82EF2D48: 3961007C  addi r11, r1, 0x7c
	ctx.r[11].s64 = ctx.r[1].s64 + 124;
	// 82EF2D4C: 91610134  stw r11, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82EF2D50: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82EF2D54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF2D58: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF2D5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2D64: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82EF2D68: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2D6C: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2D70: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2D74: 4BDB9E35  bl 0x82cacba8
	ctx.lr = 0x82EF2D78;
	sub_82CACBA8(ctx, base);
	// 82EF2D78: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2D7C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2D80: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2D84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2D88: 4BDB9E21  bl 0x82cacba8
	ctx.lr = 0x82EF2D8C;
	sub_82CACBA8(ctx, base);
	// 82EF2D8C: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2D90: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EF2D94: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF2D98: 480024F1  bl 0x82ef5288
	ctx.lr = 0x82EF2D9C;
	sub_82EF5288(ctx, base);
	// 82EF2D9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82EF2DA4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2DA8: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2DAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2DB0: 4BDB9DF9  bl 0x82cacba8
	ctx.lr = 0x82EF2DB4;
	sub_82CACBA8(ctx, base);
	// 82EF2DB4: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2DB8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2DBC: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2DC4: 4BDB9DE5  bl 0x82cacba8
	ctx.lr = 0x82EF2DC8;
	sub_82CACBA8(ctx, base);
	// 82EF2DC8: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2DCC: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 82EF2DD0: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF2DD4: 480024B5  bl 0x82ef5288
	ctx.lr = 0x82EF2DD8;
	sub_82EF5288(ctx, base);
	// 82EF2DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2DDC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82EF2DE0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2DE4: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2DE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2DEC: 4BDB9DBD  bl 0x82cacba8
	ctx.lr = 0x82EF2DF0;
	sub_82CACBA8(ctx, base);
	// 82EF2DF0: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2DF4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2DF8: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2DFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2E00: 4BDB9DA9  bl 0x82cacba8
	ctx.lr = 0x82EF2E04;
	sub_82CACBA8(ctx, base);
	// 82EF2E04: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2E08: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 82EF2E0C: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF2E10: 48002479  bl 0x82ef5288
	ctx.lr = 0x82EF2E14;
	sub_82EF5288(ctx, base);
	// 82EF2E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2E18: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82EF2E1C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2E20: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2E24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2E28: 4BDB9D81  bl 0x82cacba8
	ctx.lr = 0x82EF2E2C;
	sub_82CACBA8(ctx, base);
	// 82EF2E2C: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2E30: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2E34: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2E3C: 4BDB9D6D  bl 0x82cacba8
	ctx.lr = 0x82EF2E40;
	sub_82CACBA8(ctx, base);
	// 82EF2E40: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2E44: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82EF2E48: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF2E4C: 4800243D  bl 0x82ef5288
	ctx.lr = 0x82EF2E50;
	sub_82EF5288(ctx, base);
	// 82EF2E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF2E54: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82EF2E58: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2E5C: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2E60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2E64: 4BDB9D45  bl 0x82cacba8
	ctx.lr = 0x82EF2E68;
	sub_82CACBA8(ctx, base);
	// 82EF2E68: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2E6C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EF2E70: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF2E74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2E78: 4BDB9D31  bl 0x82cacba8
	ctx.lr = 0x82EF2E7C;
	sub_82CACBA8(ctx, base);
	// 82EF2E7C: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF2E80: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 82EF2E84: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF2E88: 48002401  bl 0x82ef5288
	ctx.lr = 0x82EF2E8C;
	sub_82EF5288(ctx, base);
	// 82EF2E8C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2E90: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2E98: 419A003C  beq cr6, 0x82ef2ed4
	if ctx.cr[6].eq {
	pc = 0x82EF2ED4; continue 'dispatch;
	}
	// 82EF2E9C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF2EA0: 7D6707B4  extsw r7, r11
	ctx.r[7].s64 = ctx.r[11].s32 as i64;
	// 82EF2EA4: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82EF2EA8: 7D6607B4  extsw r6, r11
	ctx.r[6].s64 = ctx.r[11].s32 as i64;
	// 82EF2EAC: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EF2EB0: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82EF2EB4: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2EB8: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2EBC: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2EC0: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2EC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2EC8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EF2ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF2ED0: 4E800421  bctrl
	ctx.lr = 0x82EF2ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF2ED4: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2ED8: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2EDC: 48000835  bl 0x82ef3710
	ctx.lr = 0x82EF2EE0;
	sub_82EF3710(ctx, base);
	// 82EF2EE0: 48000814  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2EE8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2EEC: 388BCC5C  addi r4, r11, -0x33a4
	ctx.r[4].s64 = ctx.r[11].s64 + -13220;
	// 82EF2EF0: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2EF4: 48001A6D  bl 0x82ef4960
	ctx.lr = 0x82EF2EF8;
	sub_82EF4960(ctx, base);
	// 82EF2EF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2EFC: 418200D0  beq 0x82ef2fcc
	if ctx.cr[0].eq {
	pc = 0x82EF2FCC; continue 'dispatch;
	}
	// 82EF2F00: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82EF2F04: 91610138  stw r11, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82EF2F08: 81610138  lwz r11, 0x138(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(312 as u32) ) } as u64;
	// 82EF2F0C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF2F10: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF2F14: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF2F18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF2F1C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF2F20: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82EF2F24: 38A10094  addi r5, r1, 0x94
	ctx.r[5].s64 = ctx.r[1].s64 + 148;
	// 82EF2F28: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82EF2F2C: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2F30: 4BDB9C79  bl 0x82cacba8
	ctx.lr = 0x82EF2F34;
	sub_82CACBA8(ctx, base);
	// 82EF2F34: 90610088  stw r3, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[3].u32 ) };
	// 82EF2F38: 38A10094  addi r5, r1, 0x94
	ctx.r[5].s64 = ctx.r[1].s64 + 148;
	// 82EF2F3C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82EF2F40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF2F44: 4BDB9C65  bl 0x82cacba8
	ctx.lr = 0x82EF2F48;
	sub_82CACBA8(ctx, base);
	// 82EF2F48: 90610088  stw r3, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[3].u32 ) };
	// 82EF2F4C: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF2F50: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82EF2F54: 48002545  bl 0x82ef5498
	ctx.lr = 0x82EF2F58;
	sub_82EF5498(ctx, base);
	// 82EF2F58: C001008C  lfs f0, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF2F5C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82EF2F60: D8010140  stfd f0, 0x140(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.f[0].u64 ) };
	// 82EF2F64: E8A10140  ld r5, 0x140(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) };
	// 82EF2F68: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2F6C: 808B0050  lwz r4, 0x50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF2F70: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2F74: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EF2F78: 4BFF5831  bl 0x82ee87a8
	ctx.lr = 0x82EF2F7C;
	sub_82EE87A8(ctx, base);
	// 82EF2F7C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2F80: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2F84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF2F88: 419A0034  beq cr6, 0x82ef2fbc
	if ctx.cr[6].eq {
	pc = 0x82EF2FBC; continue 'dispatch;
	}
	// 82EF2F8C: C001008C  lfs f0, 0x8c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF2F90: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82EF2F94: D8010148  stfd f0, 0x148(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.f[0].u64 ) };
	// 82EF2F98: E8810148  ld r4, 0x148(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) };
	// 82EF2F9C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2FA0: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2FA4: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2FA8: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF2FAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF2FB0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF2FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF2FB8: 4E800421  bctrl
	ctx.lr = 0x82EF2FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF2FBC: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF2FC0: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF2FC4: 4800074D  bl 0x82ef3710
	ctx.lr = 0x82EF2FC8;
	sub_82EF3710(ctx, base);
	// 82EF2FC8: 4800072C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF2FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF2FD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF2FD4: 388BCC54  addi r4, r11, -0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13228;
	// 82EF2FD8: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF2FDC: 48001985  bl 0x82ef4960
	ctx.lr = 0x82EF2FE0;
	sub_82EF4960(ctx, base);
	// 82EF2FE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF2FE4: 418200CC  beq 0x82ef30b0
	if ctx.cr[0].eq {
	pc = 0x82EF30B0; continue 'dispatch;
	}
	// 82EF2FE8: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82EF2FEC: 91610150  stw r11, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82EF2FF0: 81610150  lwz r11, 0x150(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(336 as u32) ) } as u64;
	// 82EF2FF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF2FF8: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF2FFC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3004: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3008: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82EF300C: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82EF3010: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82EF3014: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3018: 4BDB9B91  bl 0x82cacba8
	ctx.lr = 0x82EF301C;
	sub_82CACBA8(ctx, base);
	// 82EF301C: 90610098  stw r3, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82EF3020: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82EF3024: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82EF3028: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF302C: 4BDB9B7D  bl 0x82cacba8
	ctx.lr = 0x82EF3030;
	sub_82CACBA8(ctx, base);
	// 82EF3030: 90610098  stw r3, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82EF3034: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 82EF3038: 80610098  lwz r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82EF303C: 4800245D  bl 0x82ef5498
	ctx.lr = 0x82EF3040;
	sub_82EF5498(ctx, base);
	// 82EF3040: C001009C  lfs f0, 0x9c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3044: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF3048: D8010158  stfd f0, 0x158(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), ctx.f[0].u64 ) };
	// 82EF304C: 8161015C  lwz r11, 0x15c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(348 as u32) ) } as u64;
	// 82EF3050: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82EF3054: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3058: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EF305C: 4BFF5655  bl 0x82ee86b0
	ctx.lr = 0x82EF3060;
	sub_82EE86B0(ctx, base);
	// 82EF3060: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3064: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF306C: 419A0034  beq cr6, 0x82ef30a0
	if ctx.cr[6].eq {
	pc = 0x82EF30A0; continue 'dispatch;
	}
	// 82EF3070: C001009C  lfs f0, 0x9c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3074: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82EF3078: D8010160  stfd f0, 0x160(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.f[0].u64 ) };
	// 82EF307C: E8810160  ld r4, 0x160(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(352 as u32) ) };
	// 82EF3080: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3084: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3088: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF308C: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3090: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3094: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EF3098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF309C: 4E800421  bctrl
	ctx.lr = 0x82EF30A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF30A0: 808101CC  lwz r4, 0x1cc(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF30A4: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF30A8: 48000669  bl 0x82ef3710
	ctx.lr = 0x82EF30AC;
	sub_82EF3710(ctx, base);
	// 82EF30AC: 48000648  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF30B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF30B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF30B8: 388BCC4C  addi r4, r11, -0x33b4
	ctx.r[4].s64 = ctx.r[11].s64 + -13236;
	// 82EF30BC: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF30C0: 480018A1  bl 0x82ef4960
	ctx.lr = 0x82EF30C4;
	sub_82EF4960(ctx, base);
	// 82EF30C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF30C8: 41820014  beq 0x82ef30dc
	if ctx.cr[0].eq {
	pc = 0x82EF30DC; continue 'dispatch;
	}
	// 82EF30CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF30D0: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF30D4: 4BFFF15D  bl 0x82ef2230
	ctx.lr = 0x82EF30D8;
	sub_82EF2230(ctx, base);
	// 82EF30D8: 4800061C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF30DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF30E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82EF30E4: 388B3B9C  addi r4, r11, 0x3b9c
	ctx.r[4].s64 = ctx.r[11].s64 + 15260;
	// 82EF30E8: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF30EC: 48001875  bl 0x82ef4960
	ctx.lr = 0x82EF30F0;
	sub_82EF4960(ctx, base);
	// 82EF30F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF30F4: 41820140  beq 0x82ef3234
	if ctx.cr[0].eq {
	pc = 0x82EF3234; continue 'dispatch;
	}
	// 82EF30F8: 396100BC  addi r11, r1, 0xbc
	ctx.r[11].s64 = ctx.r[1].s64 + 188;
	// 82EF30FC: 91610168  stw r11, 0x168(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 82EF3100: 81610168  lwz r11, 0x168(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(360 as u32) ) } as u64;
	// 82EF3104: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF3108: 394ACC44  addi r10, r10, -0x33bc
	ctx.r[10].s64 = ctx.r[10].s64 + -13244;
	// 82EF310C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3110: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF3114: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3118: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82EF311C: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF3120: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF3124: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3128: 4BDB9A81  bl 0x82cacba8
	ctx.lr = 0x82EF312C;
	sub_82CACBA8(ctx, base);
	// 82EF312C: 906100B4  stw r3, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 82EF3130: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3134: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3138: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82EF313C: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF3140: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF3144: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF3148: 4BDB9A61  bl 0x82cacba8
	ctx.lr = 0x82EF314C;
	sub_82CACBA8(ctx, base);
	// 82EF314C: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF3150: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF3154: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF3158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF315C: 4BDB9A4D  bl 0x82cacba8
	ctx.lr = 0x82EF3160;
	sub_82CACBA8(ctx, base);
	// 82EF3160: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF3164: 388100AC  addi r4, r1, 0xac
	ctx.r[4].s64 = ctx.r[1].s64 + 172;
	// 82EF3168: 806100B0  lwz r3, 0xb0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF316C: 4800232D  bl 0x82ef5498
	ctx.lr = 0x82EF3170;
	sub_82EF5498(ctx, base);
	// 82EF3170: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3174: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3178: D00100A8  stfs f0, 0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82EF317C: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF3180: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF3184: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF3188: 4BDB9A21  bl 0x82cacba8
	ctx.lr = 0x82EF318C;
	sub_82CACBA8(ctx, base);
	// 82EF318C: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF3190: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF3194: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF3198: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF319C: 4BDB9A0D  bl 0x82cacba8
	ctx.lr = 0x82EF31A0;
	sub_82CACBA8(ctx, base);
	// 82EF31A0: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF31A4: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 82EF31A8: 806100B0  lwz r3, 0xb0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF31AC: 480022ED  bl 0x82ef5498
	ctx.lr = 0x82EF31B0;
	sub_82EF5498(ctx, base);
	// 82EF31B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF31B4: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF31B8: D00100B8  stfs f0, 0xb8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 82EF31BC: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF31C0: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF31C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF31C8: 4BDB99E1  bl 0x82cacba8
	ctx.lr = 0x82EF31CC;
	sub_82CACBA8(ctx, base);
	// 82EF31CC: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF31D0: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82EF31D4: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82EF31D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF31DC: 4BDB99CD  bl 0x82cacba8
	ctx.lr = 0x82EF31E0;
	sub_82CACBA8(ctx, base);
	// 82EF31E0: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 82EF31E4: 388100B8  addi r4, r1, 0xb8
	ctx.r[4].s64 = ctx.r[1].s64 + 184;
	// 82EF31E8: 806100B0  lwz r3, 0xb0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82EF31EC: 480022AD  bl 0x82ef5498
	ctx.lr = 0x82EF31F0;
	sub_82EF5498(ctx, base);
	// 82EF31F0: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF31F4: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF31F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF31FC: 419A0034  beq cr6, 0x82ef3230
	if ctx.cr[6].eq {
	pc = 0x82EF3230; continue 'dispatch;
	}
	// 82EF3200: C06100B8  lfs f3, 0xb8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82EF3204: C04100A8  lfs f2, 0xa8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82EF3208: C02100AC  lfs f1, 0xac(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF320C: 808100B4  lwz r4, 0xb4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EF3210: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3214: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3218: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF321C: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3220: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3224: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF3228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF322C: 4E800421  bctrl
	ctx.lr = 0x82EF3230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3230: 480004C4  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF3234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3238: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF323C: 388BCC30  addi r4, r11, -0x33d0
	ctx.r[4].s64 = ctx.r[11].s64 + -13264;
	// 82EF3240: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF3244: 4800171D  bl 0x82ef4960
	ctx.lr = 0x82EF3248;
	sub_82EF4960(ctx, base);
	// 82EF3248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF324C: 418200A0  beq 0x82ef32ec
	if ctx.cr[0].eq {
	pc = 0x82EF32EC; continue 'dispatch;
	}
	// 82EF3250: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82EF3254: 9161016C  stw r11, 0x16c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 82EF3258: 8161016C  lwz r11, 0x16c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(364 as u32) ) } as u64;
	// 82EF325C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF3260: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF3264: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3268: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF326C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3270: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82EF3274: 38A100D4  addi r5, r1, 0xd4
	ctx.r[5].s64 = ctx.r[1].s64 + 212;
	// 82EF3278: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82EF327C: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3280: 4BDB9929  bl 0x82cacba8
	ctx.lr = 0x82EF3284;
	sub_82CACBA8(ctx, base);
	// 82EF3284: 906100C8  stw r3, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[3].u32 ) };
	// 82EF3288: 38A100D4  addi r5, r1, 0xd4
	ctx.r[5].s64 = ctx.r[1].s64 + 212;
	// 82EF328C: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82EF3290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF3294: 4BDB9915  bl 0x82cacba8
	ctx.lr = 0x82EF3298;
	sub_82CACBA8(ctx, base);
	// 82EF3298: 906100C8  stw r3, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[3].u32 ) };
	// 82EF329C: 388100CC  addi r4, r1, 0xcc
	ctx.r[4].s64 = ctx.r[1].s64 + 204;
	// 82EF32A0: 806100C8  lwz r3, 0xc8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 82EF32A4: 480021F5  bl 0x82ef5498
	ctx.lr = 0x82EF32A8;
	sub_82EF5498(ctx, base);
	// 82EF32A8: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF32AC: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF32B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF32B4: 419A0034  beq cr6, 0x82ef32e8
	if ctx.cr[6].eq {
	pc = 0x82EF32E8; continue 'dispatch;
	}
	// 82EF32B8: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF32BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF32C0: C1AB0A54  lfs f13, 0xa54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF32C4: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82EF32C8: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF32CC: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF32D0: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF32D4: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF32D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF32DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF32E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF32E4: 4E800421  bctrl
	ctx.lr = 0x82EF32E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF32E8: 4800040C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF32EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF32F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF32F4: 388BCC1C  addi r4, r11, -0x33e4
	ctx.r[4].s64 = ctx.r[11].s64 + -13284;
	// 82EF32F8: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF32FC: 48001665  bl 0x82ef4960
	ctx.lr = 0x82EF3300;
	sub_82EF4960(ctx, base);
	// 82EF3300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3304: 418200A0  beq 0x82ef33a4
	if ctx.cr[0].eq {
	pc = 0x82EF33A4; continue 'dispatch;
	}
	// 82EF3308: 396100E0  addi r11, r1, 0xe0
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	// 82EF330C: 91610170  stw r11, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[11].u32 ) };
	// 82EF3310: 81610170  lwz r11, 0x170(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(368 as u32) ) } as u64;
	// 82EF3314: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF3318: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF331C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3324: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3328: D00100DC  stfs f0, 0xdc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82EF332C: 38A100E4  addi r5, r1, 0xe4
	ctx.r[5].s64 = ctx.r[1].s64 + 228;
	// 82EF3330: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82EF3334: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3338: 4BDB9871  bl 0x82cacba8
	ctx.lr = 0x82EF333C;
	sub_82CACBA8(ctx, base);
	// 82EF333C: 906100D8  stw r3, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[3].u32 ) };
	// 82EF3340: 38A100E4  addi r5, r1, 0xe4
	ctx.r[5].s64 = ctx.r[1].s64 + 228;
	// 82EF3344: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82EF3348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF334C: 4BDB985D  bl 0x82cacba8
	ctx.lr = 0x82EF3350;
	sub_82CACBA8(ctx, base);
	// 82EF3350: 906100D8  stw r3, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[3].u32 ) };
	// 82EF3354: 388100DC  addi r4, r1, 0xdc
	ctx.r[4].s64 = ctx.r[1].s64 + 220;
	// 82EF3358: 806100D8  lwz r3, 0xd8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82EF335C: 4800213D  bl 0x82ef5498
	ctx.lr = 0x82EF3360;
	sub_82EF5498(ctx, base);
	// 82EF3360: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3364: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF336C: 419A0034  beq cr6, 0x82ef33a0
	if ctx.cr[6].eq {
	pc = 0x82EF33A0; continue 'dispatch;
	}
	// 82EF3370: C00100DC  lfs f0, 0xdc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF3374: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3378: C1AB0A54  lfs f13, 0xa54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF337C: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82EF3380: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3384: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3388: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF338C: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3390: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3394: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF3398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF339C: 4E800421  bctrl
	ctx.lr = 0x82EF33A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF33A0: 48000354  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF33A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF33A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF33AC: 388BCC4C  addi r4, r11, -0x33b4
	ctx.r[4].s64 = ctx.r[11].s64 + -13236;
	// 82EF33B0: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF33B4: 480015AD  bl 0x82ef4960
	ctx.lr = 0x82EF33B8;
	sub_82EF4960(ctx, base);
	// 82EF33B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF33BC: 4182003C  beq 0x82ef33f8
	if ctx.cr[0].eq {
	pc = 0x82EF33F8; continue 'dispatch;
	}
	// 82EF33C0: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF33C4: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF33C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF33CC: 419A0028  beq cr6, 0x82ef33f4
	if ctx.cr[6].eq {
	pc = 0x82EF33F4; continue 'dispatch;
	}
	// 82EF33D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF33D4: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF33D8: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF33DC: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF33E0: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF33E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF33E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF33EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF33F0: 4E800421  bctrl
	ctx.lr = 0x82EF33F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF33F4: 48000300  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF33F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF33FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3400: 388BCC08  addi r4, r11, -0x33f8
	ctx.r[4].s64 = ctx.r[11].s64 + -13304;
	// 82EF3404: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF3408: 48001559  bl 0x82ef4960
	ctx.lr = 0x82EF340C;
	sub_82EF4960(ctx, base);
	// 82EF340C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3410: 4182007C  beq 0x82ef348c
	if ctx.cr[0].eq {
	pc = 0x82EF348C; continue 'dispatch;
	}
	// 82EF3414: 396100EC  addi r11, r1, 0xec
	ctx.r[11].s64 = ctx.r[1].s64 + 236;
	// 82EF3418: 91610174  stw r11, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 82EF341C: 81610174  lwz r11, 0x174(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82EF3420: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF3424: 814ACC90  lwz r10, -0x3370(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13168 as u32) ) } as u64;
	// 82EF3428: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF342C: 38A100F0  addi r5, r1, 0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + 240;
	// 82EF3430: 388100EC  addi r4, r1, 0xec
	ctx.r[4].s64 = ctx.r[1].s64 + 236;
	// 82EF3434: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3438: 4BDB9771  bl 0x82cacba8
	ctx.lr = 0x82EF343C;
	sub_82CACBA8(ctx, base);
	// 82EF343C: 906100E8  stw r3, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[3].u32 ) };
	// 82EF3440: 38A100F0  addi r5, r1, 0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + 240;
	// 82EF3444: 388100EC  addi r4, r1, 0xec
	ctx.r[4].s64 = ctx.r[1].s64 + 236;
	// 82EF3448: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF344C: 4BDB975D  bl 0x82cacba8
	ctx.lr = 0x82EF3450;
	sub_82CACBA8(ctx, base);
	// 82EF3450: 906100E8  stw r3, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[3].u32 ) };
	// 82EF3454: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3458: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF345C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3460: 419A0028  beq cr6, 0x82ef3488
	if ctx.cr[6].eq {
	pc = 0x82EF3488; continue 'dispatch;
	}
	// 82EF3464: 808100E8  lwz r4, 0xe8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 82EF3468: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF346C: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3470: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3474: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3478: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF347C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF3480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF3484: 4E800421  bctrl
	ctx.lr = 0x82EF3488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3488: 4800026C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF348C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3490: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3494: 388BCBFC  addi r4, r11, -0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + -13316;
	// 82EF3498: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF349C: 480014C5  bl 0x82ef4960
	ctx.lr = 0x82EF34A0;
	sub_82EF4960(ctx, base);
	// 82EF34A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF34A4: 41820040  beq 0x82ef34e4
	if ctx.cr[0].eq {
	pc = 0x82EF34E4; continue 'dispatch;
	}
	// 82EF34A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF34AC: 996100F4  stb r11, 0xf4(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[11].u8 ) };
	// 82EF34B0: 388100F4  addi r4, r1, 0xf4
	ctx.r[4].s64 = ctx.r[1].s64 + 244;
	// 82EF34B4: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF34B8: 48002089  bl 0x82ef5540
	ctx.lr = 0x82EF34BC;
	sub_82EF5540(ctx, base);
	// 82EF34BC: 888100F4  lbz r4, 0xf4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 82EF34C0: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF34C4: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF34C8: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF34CC: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF34D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF34D4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF34D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF34DC: 4E800421  bctrl
	ctx.lr = 0x82EF34E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF34E0: 48000214  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF34E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF34E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF34EC: 388BCBE4  addi r4, r11, -0x341c
	ctx.r[4].s64 = ctx.r[11].s64 + -13340;
	// 82EF34F0: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF34F4: 4800146D  bl 0x82ef4960
	ctx.lr = 0x82EF34F8;
	sub_82EF4960(ctx, base);
	// 82EF34F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF34FC: 41820014  beq 0x82ef3510
	if ctx.cr[0].eq {
	pc = 0x82EF3510; continue 'dispatch;
	}
	// 82EF3500: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF3508: 994B0488  stb r10, 0x488(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1160 as u32), ctx.r[10].u8 ) };
	// 82EF350C: 480001E8  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF3510: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3514: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3518: 388BCBD0  addi r4, r11, -0x3430
	ctx.r[4].s64 = ctx.r[11].s64 + -13360;
	// 82EF351C: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF3520: 48001441  bl 0x82ef4960
	ctx.lr = 0x82EF3524;
	sub_82EF4960(ctx, base);
	// 82EF3524: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3528: 41820014  beq 0x82ef353c
	if ctx.cr[0].eq {
	pc = 0x82EF353C; continue 'dispatch;
	}
	// 82EF352C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3530: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF3534: 994B0489  stb r10, 0x489(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1161 as u32), ctx.r[10].u8 ) };
	// 82EF3538: 480001BC  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF353C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3540: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3544: 388BCBBC  addi r4, r11, -0x3444
	ctx.r[4].s64 = ctx.r[11].s64 + -13380;
	// 82EF3548: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF354C: 48001415  bl 0x82ef4960
	ctx.lr = 0x82EF3550;
	sub_82EF4960(ctx, base);
	// 82EF3550: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3554: 41820140  beq 0x82ef3694
	if ctx.cr[0].eq {
	pc = 0x82EF3694; continue 'dispatch;
	}
	// 82EF3558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF355C: 916100F8  stw r11, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 82EF3560: 388100F8  addi r4, r1, 0xf8
	ctx.r[4].s64 = ctx.r[1].s64 + 248;
	// 82EF3564: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF3568: 48001DF1  bl 0x82ef5358
	ctx.lr = 0x82EF356C;
	sub_82EF5358(ctx, base);
	// 82EF356C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3570: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF3574: 386B016C  addi r3, r11, 0x16c
	ctx.r[3].s64 = ctx.r[11].s64 + 364;
	// 82EF3578: 480006D1  bl 0x82ef3c48
	ctx.lr = 0x82EF357C;
	sub_82EF3C48(ctx, base);
	// 82EF357C: 816100F8  lwz r11, 0xf8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82EF3580: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF3584: 4098010C  bge cr6, 0x82ef3690
	if !ctx.cr[6].lt {
	pc = 0x82EF3690; continue 'dispatch;
	}
	// 82EF3588: 808100F8  lwz r4, 0xf8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82EF358C: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3590: 806B04D8  lwz r3, 0x4d8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF3594: 4BFFAA15  bl 0x82eedfa8
	ctx.lr = 0x82EF3598;
	sub_82EEDFA8(ctx, base);
	// 82EF3598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF359C: 808100F8  lwz r4, 0xf8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82EF35A0: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF35A4: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF35A8: 386B016C  addi r3, r11, 0x16c
	ctx.r[3].s64 = ctx.r[11].s64 + 364;
	// 82EF35AC: 480006E5  bl 0x82ef3c90
	ctx.lr = 0x82EF35B0;
	sub_82EF3C90(ctx, base);
	// 82EF35B0: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF35B4: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF35B8: 808B0494  lwz r4, 0x494(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82EF35BC: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF35C0: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EF35C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EF35C8: 4BFF6481  bl 0x82ee9a48
	ctx.lr = 0x82EF35CC;
	sub_82EE9A48(ctx, base);
	// 82EF35CC: 906100FC  stw r3, 0xfc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), ctx.r[3].u32 ) };
	// 82EF35D0: 816100FC  lwz r11, 0xfc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 82EF35D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF35D8: 41980058  blt cr6, 0x82ef3630
	if ctx.cr[6].lt {
	pc = 0x82EF3630; continue 'dispatch;
	}
	// 82EF35DC: 83E100F8  lwz r31, 0xf8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82EF35E0: 3BC10100  addi r30, r1, 0x100
	ctx.r[30].s64 = ctx.r[1].s64 + 256;
	// 82EF35E4: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 82EF35E8: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF35EC: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF35F0: 388B016C  addi r4, r11, 0x16c
	ctx.r[4].s64 = ctx.r[11].s64 + 364;
	// 82EF35F4: 48000615  bl 0x82ef3c08
	ctx.lr = 0x82EF35F8;
	sub_82EF3C08(ctx, base);
	// 82EF35F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF35FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF3600: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF3604: 480009BD  bl 0x82ef3fc0
	ctx.lr = 0x82EF3608;
	sub_82EF3FC0(ctx, base);
	// 82EF3608: E8A10100  ld r5, 0x100(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	// 82EF360C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 82EF3610: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3614: 816B04D8  lwz r11, 0x4d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF3618: 388B016C  addi r4, r11, 0x16c
	ctx.r[4].s64 = ctx.r[11].s64 + 364;
	// 82EF361C: 4800075D  bl 0x82ef3d78
	ctx.lr = 0x82EF3620;
	sub_82EF3D78(ctx, base);
	// 82EF3620: 808100F8  lwz r4, 0xf8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82EF3624: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3628: 806B04D8  lwz r3, 0x4d8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1240 as u32) ) } as u64;
	// 82EF362C: 4BFFAB95  bl 0x82eee1c0
	ctx.lr = 0x82EF3630;
	sub_82EEE1C0(ctx, base);
	// 82EF3630: 816100FC  lwz r11, 0xfc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 82EF3634: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF3638: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF363C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF3640: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82EF3644: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82EF3648: F9610178  std r11, 0x178(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[11].u64 ) };
	// 82EF364C: C8010178  lfd f0, 0x178(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(376 as u32) ) };
	// 82EF3650: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 82EF3654: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 82EF3658: 48002141  bl 0x82ef5798
	ctx.lr = 0x82EF365C;
	sub_82EF5798(ctx, base);
	// 82EF365C: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF3660: 480022D9  bl 0x82ef5938
	ctx.lr = 0x82EF3664;
	sub_82EF5938(ctx, base);
	// 82EF3664: 90610180  stw r3, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[3].u32 ) };
	// 82EF3668: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EF366C: 38A10120  addi r5, r1, 0x120
	ctx.r[5].s64 = ctx.r[1].s64 + 288;
	// 82EF3670: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3674: 388BCBA4  addi r4, r11, -0x345c
	ctx.r[4].s64 = ctx.r[11].s64 + -13404;
	// 82EF3678: 80610180  lwz r3, 0x180(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 82EF367C: 81610180  lwz r11, 0x180(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 82EF3680: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3684: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF3688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF368C: 4E800421  bctrl
	ctx.lr = 0x82EF3690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3690: 48000064  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF3694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3698: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF369C: 388BCB98  addi r4, r11, -0x3468
	ctx.r[4].s64 = ctx.r[11].s64 + -13416;
	// 82EF36A0: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF36A4: 480012BD  bl 0x82ef4960
	ctx.lr = 0x82EF36A8;
	sub_82EF4960(ctx, base);
	// 82EF36A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF36AC: 41820010  beq 0x82ef36bc
	if ctx.cr[0].eq {
	pc = 0x82EF36BC; continue 'dispatch;
	}
	// 82EF36B0: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF36B4: 480004AD  bl 0x82ef3b60
	ctx.lr = 0x82EF36B8;
	sub_82EF3B60(ctx, base);
	// 82EF36B8: 4800003C  b 0x82ef36f4
	pc = 0x82EF36F4; continue 'dispatch;
	// 82EF36BC: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF36C0: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF36C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF36C8: 419A002C  beq cr6, 0x82ef36f4
	if ctx.cr[6].eq {
	pc = 0x82EF36F4; continue 'dispatch;
	}
	// 82EF36CC: 80A101CC  lwz r5, 0x1cc(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 82EF36D0: 808101C4  lwz r4, 0x1c4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 82EF36D4: 816101B4  lwz r11, 0x1b4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF36D8: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF36DC: 814101B4  lwz r10, 0x1b4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82EF36E0: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF36E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF36E8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EF36EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF36F0: 4E800421  bctrl
	ctx.lr = 0x82EF36F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF36F4: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82EF36F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF36FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF3704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3710 size=256
    let mut pc: u32 = 0x82EF3710;
    'dispatch: loop {
        match pc {
            0x82EF3710 => {
    //   block [0x82EF3710..0x82EF3810)
	// 82EF3710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF371C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF3720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3724: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF3728: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82EF372C: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF3730: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3734: 409A0058  bne cr6, 0x82ef378c
	if !ctx.cr[6].eq {
	pc = 0x82EF378C; continue 'dispatch;
	}
	// 82EF3738: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF373C: 896B8F80  lbz r11, -0x7080(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28800 as u32) ) } as u64;
	// 82EF3740: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF3744: 40820048  bne 0x82ef378c
	if !ctx.cr[0].eq {
	pc = 0x82EF378C; continue 'dispatch;
	}
	// 82EF3748: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF374C: 3BEBCCDC  addi r31, r11, -0x3324
	ctx.r[31].s64 = ctx.r[11].s64 + -13092;
	// 82EF3750: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF3754: 3BCB8F80  addi r30, r11, -0x7080
	ctx.r[30].s64 = ctx.r[11].s64 + -28800;
	// 82EF3758: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF375C: 38CBCCD0  addi r6, r11, -0x3330
	ctx.r[6].s64 = ctx.r[11].s64 + -13104;
	// 82EF3760: 38A004A8  li r5, 0x4a8
	ctx.r[5].s64 = 1192;
	// 82EF3764: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3768: 388BC6C8  addi r4, r11, -0x3938
	ctx.r[4].s64 = ctx.r[11].s64 + -14648;
	// 82EF376C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF3770: 4BFF79F9  bl 0x82eeb168
	ctx.lr = 0x82EF3774;
	sub_82EEB168(ctx, base);
	// 82EF3774: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF3778: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF377C: 481FE715  bl 0x830f1e90
	ctx.lr = 0x82EF3780;
	sub_830F1E90(ctx, base);
	// 82EF3780: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3784: 40820008  bne 0x82ef378c
	if !ctx.cr[0].eq {
	pc = 0x82EF378C; continue 'dispatch;
	}
	// 82EF3788: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF378C: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF3790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3794: 409A0008  bne cr6, 0x82ef379c
	if !ctx.cr[6].eq {
	pc = 0x82EF379C; continue 'dispatch;
	}
	// 82EF3798: 48000060  b 0x82ef37f8
	pc = 0x82EF37F8; continue 'dispatch;
	// 82EF379C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF37A0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82EF37A4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF37A8: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF37AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF37B0: 419A002C  beq cr6, 0x82ef37dc
	if ctx.cr[6].eq {
	pc = 0x82EF37DC; continue 'dispatch;
	}
	// 82EF37B4: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF37B8: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF37BC: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF37C0: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF37C4: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF37C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF37CC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EF37D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF37D4: 4E800421  bctrl
	ctx.lr = 0x82EF37D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF37D8: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 82EF37DC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF37E0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF37E4: 40820014  bne 0x82ef37f8
	if !ctx.cr[0].eq {
	pc = 0x82EF37F8; continue 'dispatch;
	}
	// 82EF37E8: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF37EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82EF37F0: 386B231C  addi r3, r11, 0x231c
	ctx.r[3].s64 = ctx.r[11].s64 + 8988;
	// 82EF37F4: 480019CD  bl 0x82ef51c0
	ctx.lr = 0x82EF37F8;
	sub_82EF51C0(ctx, base);
	// 82EF37F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF37FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3804: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF3808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3810 size=232
    let mut pc: u32 = 0x82EF3810;
    'dispatch: loop {
        match pc {
            0x82EF3810 => {
    //   block [0x82EF3810..0x82EF38F8)
	// 82EF3810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3814: 4BDB5BF9  bl 0x82ca940c
	ctx.lr = 0x82EF3818;
	sub_82CA93D0(ctx, base);
	// 82EF3818: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF381C: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF3820: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 82EF3824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF3828: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82EF382C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF3830: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3838: 419A002C  beq cr6, 0x82ef3864
	if ctx.cr[6].eq {
	pc = 0x82EF3864; continue 'dispatch;
	}
	// 82EF383C: 808100AC  lwz r4, 0xac(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF3840: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF3844: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3848: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF384C: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3850: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3854: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF3858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF385C: 4E800421  bctrl
	ctx.lr = 0x82EF3860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3860: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 82EF3864: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF3868: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF386C: 40820084  bne 0x82ef38f0
	if !ctx.cr[0].eq {
	pc = 0x82EF38F0; continue 'dispatch;
	}
	// 82EF3870: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF3874: 896B8F81  lbz r11, -0x707f(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28799 as u32) ) } as u64;
	// 82EF3878: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF387C: 40820074  bne 0x82ef38f0
	if !ctx.cr[0].eq {
	pc = 0x82EF38F0; continue 'dispatch;
	}
	// 82EF3880: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF3884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3888: 409A0014  bne cr6, 0x82ef389c
	if !ctx.cr[6].eq {
	pc = 0x82EF389C; continue 'dispatch;
	}
	// 82EF388C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82EF3890: 396B8034  addi r11, r11, -0x7fcc
	ctx.r[11].s64 = ctx.r[11].s64 + -32716;
	// 82EF3894: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF3898: 4800000C  b 0x82ef38a4
	pc = 0x82EF38A4; continue 'dispatch;
	// 82EF389C: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF38A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF38A4: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EF38A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF38AC: 3BCBCD28  addi r30, r11, -0x32d8
	ctx.r[30].s64 = ctx.r[11].s64 + -13016;
	// 82EF38B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF38B4: 3BAB8F81  addi r29, r11, -0x707f
	ctx.r[29].s64 = ctx.r[11].s64 + -28799;
	// 82EF38B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF38BC: 38CBCD1C  addi r6, r11, -0x32e4
	ctx.r[6].s64 = ctx.r[11].s64 + -13028;
	// 82EF38C0: 38A004BB  li r5, 0x4bb
	ctx.r[5].s64 = 1211;
	// 82EF38C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF38C8: 388BC6C8  addi r4, r11, -0x3938
	ctx.r[4].s64 = ctx.r[11].s64 + -14648;
	// 82EF38CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF38D0: 4BFF7899  bl 0x82eeb168
	ctx.lr = 0x82EF38D4;
	sub_82EEB168(ctx, base);
	// 82EF38D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF38D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF38DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EF38E0: 481FE5B1  bl 0x830f1e90
	ctx.lr = 0x82EF38E4;
	sub_830F1E90(ctx, base);
	// 82EF38E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF38E8: 40820008  bne 0x82ef38f0
	if !ctx.cr[0].eq {
	pc = 0x82EF38F0; continue 'dispatch;
	}
	// 82EF38EC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF38F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF38F4: 4BDB5B68  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF38F8 size=108
    let mut pc: u32 = 0x82EF38F8;
    'dispatch: loop {
        match pc {
            0x82EF38F8 => {
    //   block [0x82EF38F8..0x82EF3964)
	// 82EF38F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF38FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3904: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF3908: 3D6082EF  lis r11, -0x7d11
	ctx.r[11].s64 = -2098266112;
	// 82EF390C: 38CB5780  addi r6, r11, 0x5780
	ctx.r[6].s64 = ctx.r[11].s64 + 22400;
	// 82EF3910: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3914: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF3918: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF391C: 4B32C1ED  bl 0x8221fb08
	ctx.lr = 0x82EF3920;
	sub_8221FB08(ctx, base);
	// 82EF3920: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3924: 48002015  bl 0x82ef5938
	ctx.lr = 0x82EF3928;
	sub_82EF5938(ctx, base);
	// 82EF3928: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82EF392C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF3930: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF3934: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3938: 388BC8D4  addi r4, r11, -0x372c
	ctx.r[4].s64 = ctx.r[11].s64 + -14124;
	// 82EF393C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF3940: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF3944: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3948: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF394C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF3950: 4E800421  bctrl
	ctx.lr = 0x82EF3954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF3958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF395C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3968 size=124
    let mut pc: u32 = 0x82EF3968;
    'dispatch: loop {
        match pc {
            0x82EF3968 => {
    //   block [0x82EF3968..0x82EF39E4)
	// 82EF3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3974: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF3978: 3D6082EF  lis r11, -0x7d11
	ctx.r[11].s64 = -2098266112;
	// 82EF397C: 38CB5780  addi r6, r11, 0x5780
	ctx.r[6].s64 = ctx.r[11].s64 + 22400;
	// 82EF3980: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3984: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF3988: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF398C: 4B32C17D  bl 0x8221fb08
	ctx.lr = 0x82EF3990;
	sub_8221FB08(ctx, base);
	// 82EF3990: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3994: C82B0D38  lfd f1, 0xd38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3384 as u32) ) };
	// 82EF3998: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF399C: 48001E4D  bl 0x82ef57e8
	ctx.lr = 0x82EF39A0;
	sub_82EF57E8(ctx, base);
	// 82EF39A0: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF39A4: 48001F95  bl 0x82ef5938
	ctx.lr = 0x82EF39A8;
	sub_82EF5938(ctx, base);
	// 82EF39A8: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82EF39AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF39B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF39B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF39B8: 388BCD4C  addi r4, r11, -0x32b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12980;
	// 82EF39BC: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF39C0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF39C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF39C8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF39CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF39D0: 4E800421  bctrl
	ctx.lr = 0x82EF39D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF39D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF39D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF39DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF39E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF39E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF39E8 size=124
    let mut pc: u32 = 0x82EF39E8;
    'dispatch: loop {
        match pc {
            0x82EF39E8 => {
    //   block [0x82EF39E8..0x82EF3A64)
	// 82EF39E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF39EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF39F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF39F4: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF39F8: 3D6082EF  lis r11, -0x7d11
	ctx.r[11].s64 = -2098266112;
	// 82EF39FC: 38CB5780  addi r6, r11, 0x5780
	ctx.r[6].s64 = ctx.r[11].s64 + 22400;
	// 82EF3A00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3A04: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF3A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3A0C: 4B32C0FD  bl 0x8221fb08
	ctx.lr = 0x82EF3A10;
	sub_8221FB08(ctx, base);
	// 82EF3A10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3A14: C82B0D38  lfd f1, 0xd38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3384 as u32) ) };
	// 82EF3A18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3A1C: 48001DCD  bl 0x82ef57e8
	ctx.lr = 0x82EF3A20;
	sub_82EF57E8(ctx, base);
	// 82EF3A20: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3A24: 48001F15  bl 0x82ef5938
	ctx.lr = 0x82EF3A28;
	sub_82EF5938(ctx, base);
	// 82EF3A28: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82EF3A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF3A30: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF3A34: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3A38: 388BCD60  addi r4, r11, -0x32a0
	ctx.r[4].s64 = ctx.r[11].s64 + -12960;
	// 82EF3A3C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF3A40: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF3A44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3A48: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF3A4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF3A50: 4E800421  bctrl
	ctx.lr = 0x82EF3A54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3A54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF3A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF3A68 size=244
    let mut pc: u32 = 0x82EF3A68;
    'dispatch: loop {
        match pc {
            0x82EF3A68 => {
    //   block [0x82EF3A68..0x82EF3B5C)
	// 82EF3A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3A70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF3A74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF3A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3A7C: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF3A80: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3A84: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF3A88: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EF3A8C: 419A0058  beq cr6, 0x82ef3ae4
	if ctx.cr[6].eq {
	pc = 0x82EF3AE4; continue 'dispatch;
	}
	// 82EF3A90: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF3A94: 896B8F82  lbz r11, -0x707e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28798 as u32) ) } as u64;
	// 82EF3A98: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF3A9C: 40820048  bne 0x82ef3ae4
	if !ctx.cr[0].eq {
	pc = 0x82EF3AE4; continue 'dispatch;
	}
	// 82EF3AA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3AA4: 3BEBCD98  addi r31, r11, -0x3268
	ctx.r[31].s64 = ctx.r[11].s64 + -12904;
	// 82EF3AA8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF3AAC: 3BCB8F82  addi r30, r11, -0x707e
	ctx.r[30].s64 = ctx.r[11].s64 + -28798;
	// 82EF3AB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3AB4: 38CBCD70  addi r6, r11, -0x3290
	ctx.r[6].s64 = ctx.r[11].s64 + -12944;
	// 82EF3AB8: 38A004DB  li r5, 0x4db
	ctx.r[5].s64 = 1243;
	// 82EF3ABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF3AC0: 388BC6C8  addi r4, r11, -0x3938
	ctx.r[4].s64 = ctx.r[11].s64 + -14648;
	// 82EF3AC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3AC8: 4BFF76A1  bl 0x82eeb168
	ctx.lr = 0x82EF3ACC;
	sub_82EEB168(ctx, base);
	// 82EF3ACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF3AD0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF3AD4: 481FE3BD  bl 0x830f1e90
	ctx.lr = 0x82EF3AD8;
	sub_830F1E90(ctx, base);
	// 82EF3AD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF3ADC: 40820008  bne 0x82ef3ae4
	if !ctx.cr[0].eq {
	pc = 0x82EF3AE4; continue 'dispatch;
	}
	// 82EF3AE0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF3AE4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3AE8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF3AEC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EF3AF0: 409A0054  bne cr6, 0x82ef3b44
	if !ctx.cr[6].eq {
	pc = 0x82EF3B44; continue 'dispatch;
	}
	// 82EF3AF4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3AF8: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3AFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3B00: 419A0024  beq cr6, 0x82ef3b24
	if ctx.cr[6].eq {
	pc = 0x82EF3B24; continue 'dispatch;
	}
	// 82EF3B04: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3B08: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3B0C: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3B10: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3B14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3B18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF3B20: 4E800421  bctrl
	ctx.lr = 0x82EF3B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF3B28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF3B2C: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF3B30: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3B34: 480FA8BD  bl 0x82fee3f0
	ctx.lr = 0x82EF3B38;
	sub_82FEE3F0(ctx, base);
	// 82EF3B38: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3B3C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF3B40: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82EF3B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF3B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3B50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF3B54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3B60 size=128
    let mut pc: u32 = 0x82EF3B60;
    'dispatch: loop {
        match pc {
            0x82EF3B60 => {
    //   block [0x82EF3B60..0x82EF3BE0)
	// 82EF3B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3B6C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3B70: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3B74: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF3B78: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EF3B7C: 409A0008  bne cr6, 0x82ef3b84
	if !ctx.cr[6].eq {
	pc = 0x82EF3B84; continue 'dispatch;
	}
	// 82EF3B80: 48000050  b 0x82ef3bd0
	pc = 0x82EF3BD0; continue 'dispatch;
	// 82EF3B84: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3B88: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82EF3B8C: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82EF3B90: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3B94: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3B9C: 419A0034  beq cr6, 0x82ef3bd0
	if ctx.cr[6].eq {
	pc = 0x82EF3BD0; continue 'dispatch;
	}
	// 82EF3BA0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3BA4: 896B04DD  lbz r11, 0x4dd(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1245 as u32) ) } as u64;
	// 82EF3BA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF3BAC: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF3BB0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3BB4: 816B046C  lwz r11, 0x46c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3BB8: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3BBC: 806A046C  lwz r3, 0x46c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1132 as u32) ) } as u64;
	// 82EF3BC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3BC4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF3BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF3BCC: 4E800421  bctrl
	ctx.lr = 0x82EF3BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF3BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3BE0 size=40
    let mut pc: u32 = 0x82EF3BE0;
    'dispatch: loop {
        match pc {
            0x82EF3BE0 => {
    //   block [0x82EF3BE0..0x82EF3C08)
	// 82EF3BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3BEC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3BF0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3BF4: 480005E5  bl 0x82ef41d8
	ctx.lr = 0x82EF3BF8;
	sub_82EF41D8(ctx, base);
	// 82EF3BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3C08 size=60
    let mut pc: u32 = 0x82EF3C08;
    'dispatch: loop {
        match pc {
            0x82EF3C08 => {
    //   block [0x82EF3C08..0x82EF3C44)
	// 82EF3C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3C14: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3C18: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3C1C: 80A1007C  lwz r5, 0x7c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3C20: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3C24: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3C28: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3C2C: 4800072D  bl 0x82ef4358
	ctx.lr = 0x82EF3C30;
	sub_82EF4358(ctx, base);
	// 82EF3C30: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3C48 size=32
    let mut pc: u32 = 0x82EF3C48;
    'dispatch: loop {
        match pc {
            0x82EF3C48 => {
    //   block [0x82EF3C48..0x82EF3C68)
	// 82EF3C48: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF3C4C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF3C50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3C58: 409A0010  bne cr6, 0x82ef3c68
	if !ctx.cr[6].eq {
		sub_82EF3C68(ctx, base);
		return;
	}
	// 82EF3C5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF3C60: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF3C64: 48000020  b 0x82ef3c84
	sub_82EF3C68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF3C68 size=36
    let mut pc: u32 = 0x82EF3C68;
    'dispatch: loop {
        match pc {
            0x82EF3C68 => {
    //   block [0x82EF3C68..0x82EF3C8C)
	// 82EF3C68: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF3C6C: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF3C70: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3C74: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3C78: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF3C7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF3C80: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF3C84: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF3C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3C90 size=80
    let mut pc: u32 = 0x82EF3C90;
    'dispatch: loop {
        match pc {
            0x82EF3C90 => {
    //   block [0x82EF3C90..0x82EF3CE0)
	// 82EF3C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3C9C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3CA0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3CA4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3CA8: 4BFFFFA1  bl 0x82ef3c48
	ctx.lr = 0x82EF3CAC;
	sub_82EF3C48(ctx, base);
	// 82EF3CAC: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3CB0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF3CB4: 41980008  blt cr6, 0x82ef3cbc
	if ctx.cr[6].lt {
	pc = 0x82EF3CBC; continue 'dispatch;
	}
	// 82EF3CB8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF3CBC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3CC0: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3CC4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF3CC8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3CCC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF3CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3CE0 size=148
    let mut pc: u32 = 0x82EF3CE0;
    'dispatch: loop {
        match pc {
            0x82EF3CE0 => {
    //   block [0x82EF3CE0..0x82EF3D74)
	// 82EF3CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF3CEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3CF0: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF3CF4: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82EF3CF8: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3CFC: 4BFFFF4D  bl 0x82ef3c48
	ctx.lr = 0x82EF3D00;
	sub_82EF3C48(ctx, base);
	// 82EF3D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF3D04: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D08: 48000309  bl 0x82ef4010
	ctx.lr = 0x82EF3D0C;
	sub_82EF4010(ctx, base);
	// 82EF3D0C: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF3D10: 40980028  bge cr6, 0x82ef3d38
	if !ctx.cr[6].lt {
	pc = 0x82EF3D38; continue 'dispatch;
	}
	// 82EF3D14: 80C1009C  lwz r6, 0x9c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF3D18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF3D1C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D20: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3D24: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D28: 48000549  bl 0x82ef4270
	ctx.lr = 0x82EF3D2C;
	sub_82EF4270(ctx, base);
	// 82EF3D2C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D30: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EF3D34: 4800002C  b 0x82ef3d60
	pc = 0x82EF3D60; continue 'dispatch;
	// 82EF3D38: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF3D3C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF3D40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3D44: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D48: 4BFFAD41  bl 0x82eeea88
	ctx.lr = 0x82EF3D4C;
	sub_82EEEA88(ctx, base);
	// 82EF3D4C: E8A30000  ld r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82EF3D50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF3D54: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3D58: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF3D5C: 480002FD  bl 0x82ef4058
	ctx.lr = 0x82EF3D60;
	sub_82EF4058(ctx, base);
	// 82EF3D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF3D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3D6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF3D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3D78 size=136
    let mut pc: u32 = 0x82EF3D78;
    'dispatch: loop {
        match pc {
            0x82EF3D78 => {
    //   block [0x82EF3D78..0x82EF3E00)
	// 82EF3D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3D84: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3D88: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3D8C: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 82EF3D90: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3D94: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3D98: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3D9C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3DA0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82EF3DA4: 48000C05  bl 0x82ef49a8
	ctx.lr = 0x82EF3DA8;
	sub_82EF49A8(ctx, base);
	// 82EF3DA8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3DAC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3DB0: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3DB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3DB8: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82EF3DBC: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3DC0: 480003E1  bl 0x82ef41a0
	ctx.lr = 0x82EF3DC4;
	sub_82EF41A0(ctx, base);
	// 82EF3DC4: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3DC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF3DCC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82EF3DD0: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3DD4: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF3DD8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3DDC: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EF3DE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF3DE4: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3DE8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF3DEC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3E00 size=92
    let mut pc: u32 = 0x82EF3E00;
    'dispatch: loop {
        match pc {
            0x82EF3E00 => {
    //   block [0x82EF3E00..0x82EF3E5C)
	// 82EF3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3E0C: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF3E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3E14: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3E18: 4BFFAC71  bl 0x82eeea88
	ctx.lr = 0x82EF3E1C;
	sub_82EEEA88(ctx, base);
	// 82EF3E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3E20: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EF3E24: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF3E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82EF3E2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF3E30: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3E34: 4BFFFDD5  bl 0x82ef3c08
	ctx.lr = 0x82EF3E38;
	sub_82EF3C08(ctx, base);
	// 82EF3E38: E8A30000  ld r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82EF3E3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EF3E40: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF3E44: E8C10068  ld r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82EF3E48: 480002C9  bl 0x82ef4110
	ctx.lr = 0x82EF3E4C;
	sub_82EF4110(ctx, base);
	// 82EF3E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF3E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3E60 size=60
    let mut pc: u32 = 0x82EF3E60;
    'dispatch: loop {
        match pc {
            0x82EF3E60 => {
    //   block [0x82EF3E60..0x82EF3E9C)
	// 82EF3E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3E6C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3E70: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3E74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3E7C: 419A0010  beq cr6, 0x82ef3e8c
	if ctx.cr[6].eq {
	pc = 0x82EF3E8C; continue 'dispatch;
	}
	// 82EF3E80: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3E84: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3E88: 48004411  bl 0x82ef8298
	ctx.lr = 0x82EF3E8C;
	sub_82EF8298(ctx, base);
	// 82EF3E8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3EA0 size=108
    let mut pc: u32 = 0x82EF3EA0;
    'dispatch: loop {
        match pc {
            0x82EF3EA0 => {
    //   block [0x82EF3EA0..0x82EF3F0C)
	// 82EF3EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3EAC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3EB0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3EB4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3EB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3EBC: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3EC0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF3EC4: 419A0034  beq cr6, 0x82ef3ef8
	if ctx.cr[6].eq {
	pc = 0x82EF3EF8; continue 'dispatch;
	}
	// 82EF3EC8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3ECC: 480003F5  bl 0x82ef42c0
	ctx.lr = 0x82EF3ED0;
	sub_82EF42C0(ctx, base);
	// 82EF3ED0: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3ED4: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3ED8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF3EDC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3EE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3EE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3EE8: 419A0010  beq cr6, 0x82ef3ef8
	if ctx.cr[6].eq {
	pc = 0x82EF3EF8; continue 'dispatch;
	}
	// 82EF3EEC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3EF0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3EF4: 48003EC5  bl 0x82ef7db8
	ctx.lr = 0x82EF3EF8;
	sub_82EF7DB8(ctx, base);
	// 82EF3EF8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3F10 size=64
    let mut pc: u32 = 0x82EF3F10;
    'dispatch: loop {
        match pc {
            0x82EF3F10 => {
    //   block [0x82EF3F10..0x82EF3F50)
	// 82EF3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3F1C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3F20: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3F28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3F2C: 419A0014  beq cr6, 0x82ef3f40
	if ctx.cr[6].eq {
	pc = 0x82EF3F40; continue 'dispatch;
	}
	// 82EF3F30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF3F34: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F38: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3F3C: 4BFF9BF5  bl 0x82eedb30
	ctx.lr = 0x82EF3F40;
	sub_82EEDB30(ctx, base);
	// 82EF3F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3F50 size=108
    let mut pc: u32 = 0x82EF3F50;
    'dispatch: loop {
        match pc {
            0x82EF3F50 => {
    //   block [0x82EF3F50..0x82EF3FBC)
	// 82EF3F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3F5C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3F60: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3F64: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3F6C: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3F70: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF3F74: 419A0034  beq cr6, 0x82ef3fa8
	if ctx.cr[6].eq {
	pc = 0x82EF3FA8; continue 'dispatch;
	}
	// 82EF3F78: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F7C: 4800038D  bl 0x82ef4308
	ctx.lr = 0x82EF3F80;
	sub_82EF4308(ctx, base);
	// 82EF3F80: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3F84: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F88: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF3F8C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3F90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF3F98: 419A0010  beq cr6, 0x82ef3fa8
	if ctx.cr[6].eq {
	pc = 0x82EF3FA8; continue 'dispatch;
	}
	// 82EF3F9C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3FA0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF3FA4: 480016ED  bl 0x82ef5690
	ctx.lr = 0x82EF3FA8;
	sub_82EF5690(ctx, base);
	// 82EF3FA8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3FAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF3FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF3FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF3FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF3FC0 size=80
    let mut pc: u32 = 0x82EF3FC0;
    'dispatch: loop {
        match pc {
            0x82EF3FC0 => {
    //   block [0x82EF3FC0..0x82EF4010)
	// 82EF3FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF3FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF3FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF3FCC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF3FD0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF3FD4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF3FD8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF3FDC: E96B0000  ld r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82EF3FE0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EF3FE4: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF3FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF3FEC: 480003AD  bl 0x82ef4398
	ctx.lr = 0x82EF3FF0;
	sub_82EF4398(ctx, base);
	// 82EF3FF0: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82EF3FF4: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF3FF8: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82EF3FFC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4010 size=32
    let mut pc: u32 = 0x82EF4010;
    'dispatch: loop {
        match pc {
            0x82EF4010 => {
    //   block [0x82EF4010..0x82EF4030)
	// 82EF4010: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4014: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4018: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF401C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF4020: 409A0010  bne cr6, 0x82ef4030
	if !ctx.cr[6].eq {
		sub_82EF4030(ctx, base);
		return;
	}
	// 82EF4024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF4028: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF402C: 48000020  b 0x82ef404c
	sub_82EF4030(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4030 size=36
    let mut pc: u32 = 0x82EF4030;
    'dispatch: loop {
        match pc {
            0x82EF4030 => {
    //   block [0x82EF4030..0x82EF4054)
	// 82EF4030: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4034: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4038: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF403C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4040: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4044: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4048: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF404C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF4050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4058 size=184
    let mut pc: u32 = 0x82EF4058;
    'dispatch: loop {
        match pc {
            0x82EF4058 => {
    //   block [0x82EF4058..0x82EF4110)
	// 82EF4058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF4064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4068: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF406C: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF4070: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 82EF4074: F8A100B0  std r5, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u64 ) };
	// 82EF4078: 90C100BC  stw r6, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[6].u32 ) };
	// 82EF407C: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF4080: 4BFFFBC9  bl 0x82ef3c48
	ctx.lr = 0x82EF4084;
	sub_82EF3C48(ctx, base);
	// 82EF4084: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF4088: 40820010  bne 0x82ef4098
	if !ctx.cr[0].eq {
	pc = 0x82EF4098; continue 'dispatch;
	}
	// 82EF408C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF4090: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EF4094: 48000020  b 0x82ef40b4
	pc = 0x82EF40B4; continue 'dispatch;
	// 82EF4098: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF409C: 808100AC  lwz r4, 0xac(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF40A0: 4BFFFB69  bl 0x82ef3c08
	ctx.lr = 0x82EF40A4;
	sub_82EF3C08(ctx, base);
	// 82EF40A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF40A8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82EF40AC: 480006CD  bl 0x82ef4778
	ctx.lr = 0x82EF40B0;
	sub_82EF4778(ctx, base);
	// 82EF40B0: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82EF40B4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EF40B8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF40BC: 80C100BC  lwz r6, 0xbc(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 82EF40C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF40C4: E88100B0  ld r4, 0xb0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) };
	// 82EF40C8: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF40CC: 48000305  bl 0x82ef43d0
	ctx.lr = 0x82EF40D0;
	sub_82EF43D0(ctx, base);
	// 82EF40D0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF40D4: 83C100A4  lwz r30, 0xa4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF40D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EF40DC: 808100AC  lwz r4, 0xac(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF40E0: 4BFFFB29  bl 0x82ef3c08
	ctx.lr = 0x82EF40E4;
	sub_82EF3C08(ctx, base);
	// 82EF40E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF40E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF40EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF40F0: 4BFFFED1  bl 0x82ef3fc0
	ctx.lr = 0x82EF40F4;
	sub_82EF3FC0(ctx, base);
	// 82EF40F4: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF40F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF40FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF4108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4110 size=144
    let mut pc: u32 = 0x82EF4110;
    'dispatch: loop {
        match pc {
            0x82EF4110 => {
    //   block [0x82EF4110..0x82EF41A0)
	// 82EF4110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF411C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4120: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4124: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 82EF4128: F8C10088  std r6, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u64 ) };
	// 82EF412C: 38810088  addi r4, r1, 0x88
	ctx.r[4].s64 = ctx.r[1].s64 + 136;
	// 82EF4130: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82EF4134: 48110305  bl 0x83004438
	ctx.lr = 0x82EF4138;
	sub_83004438(ctx, base);
	// 82EF4138: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF413C: 4182003C  beq 0x82ef4178
	if ctx.cr[0].eq {
	pc = 0x82EF4178; continue 'dispatch;
	}
	// 82EF4140: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4144: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4148: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF414C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4150: 48000859  bl 0x82ef49a8
	ctx.lr = 0x82EF4154;
	sub_82EF49A8(ctx, base);
	// 82EF4154: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF4158: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF415C: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4160: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4164: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4168: 48000039  bl 0x82ef41a0
	ctx.lr = 0x82EF416C;
	sub_82EF41A0(ctx, base);
	// 82EF416C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4170: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4174: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF4178: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF417C: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82EF4180: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF4184: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4188: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF418C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF419C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF41A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF41A0 size=56
    let mut pc: u32 = 0x82EF41A0;
    'dispatch: loop {
        match pc {
            0x82EF41A0 => {
    //   block [0x82EF41A0..0x82EF41D8)
	// 82EF41A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF41A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF41A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF41AC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF41B0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF41B4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF41B8: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF41BC: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF41C0: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF41C4: 4800087D  bl 0x82ef4a40
	ctx.lr = 0x82EF41C8;
	sub_82EF4A40(ctx, base);
	// 82EF41C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF41CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF41D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF41D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF41D8 size=148
    let mut pc: u32 = 0x82EF41D8;
    'dispatch: loop {
        match pc {
            0x82EF41D8 => {
    //   block [0x82EF41D8..0x82EF426C)
	// 82EF41D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF41DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF41E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF41E4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF41E8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF41EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF41F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF41F4: 419A0044  beq cr6, 0x82ef4238
	if ctx.cr[6].eq {
	pc = 0x82EF4238; continue 'dispatch;
	}
	// 82EF41F8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF41FC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4200: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4204: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4208: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF420C: 4BFFFF95  bl 0x82ef41a0
	ctx.lr = 0x82EF4210;
	sub_82EF41A0(ctx, base);
	// 82EF4210: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4214: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4218: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF421C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4220: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4224: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4228: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF422C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4230: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4234: 48110B3D  bl 0x83004d70
	ctx.lr = 0x82EF4238;
	sub_83004D70(ctx, base);
	// 82EF4238: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF423C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4240: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF4244: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF424C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF4250: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4258: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF425C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4270 size=80
    let mut pc: u32 = 0x82EF4270;
    'dispatch: loop {
        match pc {
            0x82EF4270 => {
    //   block [0x82EF4270..0x82EF42C0)
	// 82EF4270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF427C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4280: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4284: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4288: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF428C: 80C10074  lwz r6, 0x74(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4290: 80A1008C  lwz r5, 0x8c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4294: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4298: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF429C: 480007F5  bl 0x82ef4a90
	ctx.lr = 0x82EF42A0;
	sub_82EF4A90(ctx, base);
	// 82EF42A0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF42A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF42A8: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF42AC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF42B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF42B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF42B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF42BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF42C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF42C0 size=72
    let mut pc: u32 = 0x82EF42C0;
    'dispatch: loop {
        match pc {
            0x82EF42C0 => {
    //   block [0x82EF42C0..0x82EF4308)
	// 82EF42C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF42C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF42C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF42CC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF42D0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF42D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF42D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF42DC: 419A0010  beq cr6, 0x82ef42ec
	if ctx.cr[6].eq {
	pc = 0x82EF42EC; continue 'dispatch;
	}
	// 82EF42E0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF42E4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF42E8: 48003FB1  bl 0x82ef8298
	ctx.lr = 0x82EF42EC;
	sub_82EF8298(ctx, base);
	// 82EF42EC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF42F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF42F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF42F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF42FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4308 size=76
    let mut pc: u32 = 0x82EF4308;
    'dispatch: loop {
        match pc {
            0x82EF4308 => {
    //   block [0x82EF4308..0x82EF4354)
	// 82EF4308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF430C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4314: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4318: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF431C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF4324: 419A0014  beq cr6, 0x82ef4338
	if ctx.cr[6].eq {
	pc = 0x82EF4338; continue 'dispatch;
	}
	// 82EF4328: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF432C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4330: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4334: 4BFF97FD  bl 0x82eedb30
	ctx.lr = 0x82EF4338;
	sub_82EEDB30(ctx, base);
	// 82EF4338: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF433C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4340: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF4344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF434C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4358 size=60
    let mut pc: u32 = 0x82EF4358;
    'dispatch: loop {
        match pc {
            0x82EF4358 => {
    //   block [0x82EF4358..0x82EF4394)
	// 82EF4358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4364: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4368: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF436C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4370: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4374: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4378: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF437C: 4800042D  bl 0x82ef47a8
	ctx.lr = 0x82EF4380;
	sub_82EF47A8(ctx, base);
	// 82EF4380: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF438C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4398 size=52
    let mut pc: u32 = 0x82EF4398;
    'dispatch: loop {
        match pc {
            0x82EF4398 => {
    //   block [0x82EF4398..0x82EF43CC)
	// 82EF4398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF43A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF43A4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF43A8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF43AC: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF43B0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF43B4: 4800047D  bl 0x82ef4830
	ctx.lr = 0x82EF43B8;
	sub_82EF4830(ctx, base);
	// 82EF43B8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF43BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF43C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF43C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF43C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF43D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF43D0 size=932
    let mut pc: u32 = 0x82EF43D0;
    'dispatch: loop {
        match pc {
            0x82EF43D0 => {
    //   block [0x82EF43D0..0x82EF4774)
	// 82EF43D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF43D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF43D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF43DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF43E0: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF43E4: F8810098  std r4, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[4].u64 ) };
	// 82EF43E8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82EF43EC: 90C100AC  stw r6, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[6].u32 ) };
	// 82EF43F0: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82EF43F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF43F8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF43FC: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4400: 4BFFFC11  bl 0x82ef4010
	ctx.lr = 0x82EF4404;
	sub_82EF4010(ctx, base);
	// 82EF4404: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EF4408: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF440C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF4410: 409A0008  bne cr6, 0x82ef4418
	if !ctx.cr[6].eq {
	pc = 0x82EF4418; continue 'dispatch;
	}
	// 82EF4414: 4800034C  b 0x82ef4760
	pc = 0x82EF4760; continue 'dispatch;
	// 82EF4418: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF441C: 4BFF92FD  bl 0x82eed718
	ctx.lr = 0x82EF4420;
	sub_82EED718(ctx, base);
	// 82EF4420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF4424: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4428: 4BFFF821  bl 0x82ef3c48
	ctx.lr = 0x82EF442C;
	sub_82EF3C48(ctx, base);
	// 82EF442C: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82EF4430: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4434: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4438: 4098000C  bge cr6, 0x82ef4444
	if !ctx.cr[6].lt {
	pc = 0x82EF4444; continue 'dispatch;
	}
	// 82EF443C: 4BFF9305  bl 0x82eed740
	ctx.lr = 0x82EF4440;
	sub_82EED740(ctx, base);
	// 82EF4440: 48000320  b 0x82ef4760
	pc = 0x82EF4760; continue 'dispatch;
	// 82EF4444: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4448: 4BFFF801  bl 0x82ef3c48
	ctx.lr = 0x82EF444C;
	sub_82EF3C48(ctx, base);
	// 82EF444C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4450: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82EF4454: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4458: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF445C: 409801B0  bge cr6, 0x82ef460c
	if !ctx.cr[6].lt {
	pc = 0x82EF460C; continue 'dispatch;
	}
	// 82EF4460: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4464: 4BFF92B5  bl 0x82eed718
	ctx.lr = 0x82EF4468;
	sub_82EED718(ctx, base);
	// 82EF4468: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF446C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF4470: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82EF4474: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82EF4478: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF447C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4480: 40980010  bge cr6, 0x82ef4490
	if !ctx.cr[6].lt {
	pc = 0x82EF4490; continue 'dispatch;
	}
	// 82EF4484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF4488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF448C: 4800001C  b 0x82ef44a8
	pc = 0x82EF44A8; continue 'dispatch;
	// 82EF4490: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4494: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF4498: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82EF449C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF44A0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF44A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EF44A8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EF44AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF44B0: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF44B4: 4BFFF795  bl 0x82ef3c48
	ctx.lr = 0x82EF44B8;
	sub_82EF3C48(ctx, base);
	// 82EF44B8: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF44BC: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82EF44C0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF44C4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF44C8: 40980018  bge cr6, 0x82ef44e0
	if !ctx.cr[6].lt {
	pc = 0x82EF44E0; continue 'dispatch;
	}
	// 82EF44CC: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF44D0: 4BFFF779  bl 0x82ef3c48
	ctx.lr = 0x82EF44D4;
	sub_82EF3C48(ctx, base);
	// 82EF44D4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF44D8: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82EF44DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF44E0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF44E4: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF44E8: 4BFF92B9  bl 0x82eed7a0
	ctx.lr = 0x82EF44EC;
	sub_82EED7A0(ctx, base);
	// 82EF44EC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82EF44F0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF44F4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF44F8: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF44FC: 80A1009C  lwz r5, 0x9c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4500: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4504: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4508: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF450C: 480005F5  bl 0x82ef4b00
	ctx.lr = 0x82EF4510;
	sub_82EF4B00(ctx, base);
	// 82EF4510: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82EF4514: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EF4518: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF451C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4520: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4524: 4BFFFD4D  bl 0x82ef4270
	ctx.lr = 0x82EF4528;
	sub_82EF4270(ctx, base);
	// 82EF4528: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82EF452C: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4530: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4534: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4538: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF453C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4540: 480005C1  bl 0x82ef4b00
	ctx.lr = 0x82EF4544;
	sub_82EF4B00(ctx, base);
	// 82EF4544: 48000024  b 0x82ef4568
	pc = 0x82EF4568; continue 'dispatch;
	// 82EF4548: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF454C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF4550: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4554: 4BFFFC4D  bl 0x82ef41a0
	ctx.lr = 0x82EF4558;
	sub_82EF41A0(ctx, base);
	// 82EF4558: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF455C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF4560: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4564: 4811080D  bl 0x83004d70
	ctx.lr = 0x82EF4568;
	sub_83004D70(ctx, base);
	// 82EF4568: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF456C: 4BFFF6DD  bl 0x82ef3c48
	ctx.lr = 0x82EF4570;
	sub_82EF3C48(ctx, base);
	// 82EF4570: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4574: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82EF4578: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82EF457C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4580: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF4588: 419A0044  beq cr6, 0x82ef45cc
	if ctx.cr[6].eq {
	pc = 0x82EF45CC; continue 'dispatch;
	}
	// 82EF458C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4590: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4594: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4598: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF459C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45A0: 4BFFFC01  bl 0x82ef41a0
	ctx.lr = 0x82EF45A4;
	sub_82EF41A0(ctx, base);
	// 82EF45A4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45A8: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF45B0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF45B4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF45B8: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF45BC: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45C0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF45C4: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45C8: 481107A9  bl 0x83004d70
	ctx.lr = 0x82EF45CC;
	sub_83004D70(ctx, base);
	// 82EF45CC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF45D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF45D4: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF45D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF45DC: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45E0: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF45E4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF45E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF45EC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF45F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF45F4: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF45F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF45FC: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4600: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF4604: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF4608: 48000158  b 0x82ef4760
	pc = 0x82EF4760; continue 'dispatch;
	// 82EF460C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4610: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4614: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4618: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF461C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4620: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4624: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4628: 409800C8  bge cr6, 0x82ef46f0
	if !ctx.cr[6].lt {
	pc = 0x82EF46F0; continue 'dispatch;
	}
	// 82EF462C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4630: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4634: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4638: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF463C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4640: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4644: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4648: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF464C: 480004B5  bl 0x82ef4b00
	ctx.lr = 0x82EF4650;
	sub_82EF4B00(ctx, base);
	// 82EF4650: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EF4654: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4658: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF465C: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4660: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4664: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4668: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF466C: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EF4670: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4674: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4678: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF467C: 4BFFFBF5  bl 0x82ef4270
	ctx.lr = 0x82EF4680;
	sub_82EF4270(ctx, base);
	// 82EF4680: 48000030  b 0x82ef46b0
	pc = 0x82EF46B0; continue 'dispatch;
	// 82EF4684: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4688: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF468C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4690: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4694: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF4698: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF469C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF46A0: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF46A4: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF46A8: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF46AC: 4BFFFAF5  bl 0x82ef41a0
	ctx.lr = 0x82EF46B0;
	sub_82EF41A0(ctx, base);
	// 82EF46B0: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF46B4: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF46B8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF46BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF46C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF46C4: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF46C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF46CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF46D0: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF46D4: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF46D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF46DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF46E0: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF46E4: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF46E8: 48000459  bl 0x82ef4b40
	ctx.lr = 0x82EF46EC;
	sub_82EF4B40(ctx, base);
	// 82EF46EC: 48000074  b 0x82ef4760
	pc = 0x82EF4760; continue 'dispatch;
	// 82EF46F0: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF46F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF46F8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF46FC: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4700: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF4704: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF4708: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF470C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4710: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF4714: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EF4718: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF471C: 480003E5  bl 0x82ef4b00
	ctx.lr = 0x82EF4720;
	sub_82EF4B00(ctx, base);
	// 82EF4720: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4724: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82EF4728: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF472C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4730: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4734: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF4738: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EF473C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4740: 48000461  bl 0x82ef4ba0
	ctx.lr = 0x82EF4744;
	sub_82EF4BA0(ctx, base);
	// 82EF4744: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF4748: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF474C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4750: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF4754: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF4758: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF475C: 480003E5  bl 0x82ef4b40
	ctx.lr = 0x82EF4760;
	sub_82EF4B40(ctx, base);
	// 82EF4760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF4764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF476C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF4770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4778 size=48
    let mut pc: u32 = 0x82EF4778;
    'dispatch: loop {
        match pc {
            0x82EF4778 => {
    //   block [0x82EF4778..0x82EF47A8)
	// 82EF4778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF477C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4784: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4788: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF478C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4790: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4794: 48000135  bl 0x82ef48c8
	ctx.lr = 0x82EF4798;
	sub_82EF48C8(ctx, base);
	// 82EF4798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF47A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF47A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF47A8 size=132
    let mut pc: u32 = 0x82EF47A8;
    'dispatch: loop {
        match pc {
            0x82EF47A8 => {
    //   block [0x82EF47A8..0x82EF482C)
	// 82EF47A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF47AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF47B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF47B4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF47B8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF47BC: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF47C0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF47C4: 4811054D  bl 0x83004d10
	ctx.lr = 0x82EF47C8;
	sub_83004D10(ctx, base);
	// 82EF47C8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF47CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF47D0: 419A002C  beq cr6, 0x82ef47fc
	if ctx.cr[6].eq {
	pc = 0x82EF47FC; continue 'dispatch;
	}
	// 82EF47D4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF47D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF47DC: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF47E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF47E4: 41990018  bgt cr6, 0x82ef47fc
	if ctx.cr[6].gt {
	pc = 0x82EF47FC; continue 'dispatch;
	}
	// 82EF47E8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF47EC: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF47F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF47F4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF47F8: 40990008  ble cr6, 0x82ef4800
	if !ctx.cr[6].gt {
	pc = 0x82EF4800; continue 'dispatch;
	}
	// 82EF47FC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF4800: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4804: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4808: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF480C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4810: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4814: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF4818: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF481C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4830 size=148
    let mut pc: u32 = 0x82EF4830;
    'dispatch: loop {
        match pc {
            0x82EF4830 => {
    //   block [0x82EF4830..0x82EF48C4)
	// 82EF4830: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4834: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF4838: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF483C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF4844: 409A0008  bne cr6, 0x82ef484c
	if !ctx.cr[6].eq {
	pc = 0x82EF484C; continue 'dispatch;
	}
	// 82EF4848: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF484C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4850: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF4854: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4858: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF485C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF4860: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4864: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4868: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF486C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4870: 4199002C  bgt cr6, 0x82ef489c
	if ctx.cr[6].gt {
	pc = 0x82EF489C; continue 'dispatch;
	}
	// 82EF4874: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4878: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF487C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF4880: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4884: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF4888: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF488C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4890: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4894: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4898: 40980008  bge cr6, 0x82ef48a0
	if !ctx.cr[6].lt {
	pc = 0x82EF48A0; continue 'dispatch;
	}
	// 82EF489C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF48A0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF48A4: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF48A8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF48AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF48B0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF48B4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF48B8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF48BC: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF48C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF48C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF48C8 size=80
    let mut pc: u32 = 0x82EF48C8;
    'dispatch: loop {
        match pc {
            0x82EF48C8 => {
    //   block [0x82EF48C8..0x82EF4918)
	// 82EF48C8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF48CC: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF48D0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF48D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF48D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF48DC: 419A001C  beq cr6, 0x82ef48f8
	if ctx.cr[6].eq {
	pc = 0x82EF48F8; continue 'dispatch;
	}
	// 82EF48E0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF48E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF48E8: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF48EC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF48F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF48F4: 419A0008  beq cr6, 0x82ef48fc
	if ctx.cr[6].eq {
	pc = 0x82EF48FC; continue 'dispatch;
	}
	// 82EF48F8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF48FC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4900: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF4904: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF4908: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF490C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4910: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4918 size=72
    let mut pc: u32 = 0x82EF4918;
    'dispatch: loop {
        match pc {
            0x82EF4918 => {
    //   block [0x82EF4918..0x82EF4960)
	// 82EF4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4924: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4928: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF492C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4930: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF4934: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4938: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF493C: 480008E5  bl 0x82ef5220
	ctx.lr = 0x82EF4940;
	sub_82EF5220(ctx, base);
	// 82EF4940: 39630000  addi r11, r3, 0
	ctx.r[11].s64 = ctx.r[3].s64 + 0;
	// 82EF4944: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF4948: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF494C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF4950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF495C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4960 size=72
    let mut pc: u32 = 0x82EF4960;
    'dispatch: loop {
        match pc {
            0x82EF4960 => {
    //   block [0x82EF4960..0x82EF49A8)
	// 82EF4960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF496C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4970: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4974: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4978: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF497C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4980: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4984: 4BFF903D  bl 0x82eed9c0
	ctx.lr = 0x82EF4988;
	sub_82EED9C0(ctx, base);
	// 82EF4988: 39630000  addi r11, r3, 0
	ctx.r[11].s64 = ctx.r[3].s64 + 0;
	// 82EF498C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF4990: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF4994: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF4998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF49A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF49A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF49A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF49A8 size=152
    let mut pc: u32 = 0x82EF49A8;
    'dispatch: loop {
        match pc {
            0x82EF49A8 => {
    //   block [0x82EF49A8..0x82EF4A40)
	// 82EF49A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF49AC: 4BDB4A61  bl 0x82ca940c
	ctx.lr = 0x82EF49B0;
	sub_82CA93D0(ctx, base);
	// 82EF49B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF49B4: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF49B8: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82EF49BC: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82EF49C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF49C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF49C8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF49CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF49D0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF49D4: 388100A4  addi r4, r1, 0xa4
	ctx.r[4].s64 = ctx.r[1].s64 + 164;
	// 82EF49D8: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 82EF49DC: 4BFFADB5  bl 0x82eef790
	ctx.lr = 0x82EF49E0;
	sub_82EEF790(ctx, base);
	// 82EF49E0: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF49E4: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF49E8: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82EF49EC: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF49F0: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 82EF49F4: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82EF49F8: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 82EF49FC: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82EF4A00: 48000239  bl 0x82ef4c38
	ctx.lr = 0x82EF4A04;
	sub_82EF4C38(ctx, base);
	// 82EF4A04: 8BE30000  lbz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4A08: 83C100A4  lwz r30, 0xa4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4A0C: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 82EF4A10: 4BFFAD51  bl 0x82eef760
	ctx.lr = 0x82EF4A14;
	sub_82EEF760(ctx, base);
	// 82EF4A14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF4A18: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 82EF4A1C: 4BFFAD45  bl 0x82eef760
	ctx.lr = 0x82EF4A20;
	sub_82EEF760(ctx, base);
	// 82EF4A20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF4A24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF4A28: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EF4A2C: 88E10059  lbz r7, 0x59(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82EF4A30: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4A34: 4800021D  bl 0x82ef4c50
	ctx.lr = 0x82EF4A38;
	sub_82EF4C50(ctx, base);
	// 82EF4A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF4A3C: 4BDB4A20  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4A40 size=76
    let mut pc: u32 = 0x82EF4A40;
    'dispatch: loop {
        match pc {
            0x82EF4A40 => {
    //   block [0x82EF4A40..0x82EF4A8C)
	// 82EF4A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4A4C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4A50: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4A54: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4A58: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82EF4A5C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EF4A60: 4BFFAD31  bl 0x82eef790
	ctx.lr = 0x82EF4A64;
	sub_82EEF790(ctx, base);
	// 82EF4A64: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 82EF4A68: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4A6C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4A70: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4A74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4A78: 48000261  bl 0x82ef4cd8
	ctx.lr = 0x82EF4A7C;
	sub_82EF4CD8(ctx, base);
	// 82EF4A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4A90 size=108
    let mut pc: u32 = 0x82EF4A90;
    'dispatch: loop {
        match pc {
            0x82EF4A90 => {
    //   block [0x82EF4A90..0x82EF4AFC)
	// 82EF4A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4A9C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4AA0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4AA4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4AA8: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF4AAC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF4AB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4AB4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4ABC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF4AC0: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 82EF4AC4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EF4AC8: 4BFFACC9  bl 0x82eef790
	ctx.lr = 0x82EF4ACC;
	sub_82EEF790(ctx, base);
	// 82EF4ACC: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF4AD0: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4AD4: 88E10051  lbz r7, 0x51(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF4AD8: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4ADC: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4AE0: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4AE4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4AE8: 48000209  bl 0x82ef4cf0
	ctx.lr = 0x82EF4AEC;
	sub_82EF4CF0(ctx, base);
	// 82EF4AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4B00 size=64
    let mut pc: u32 = 0x82EF4B00;
    'dispatch: loop {
        match pc {
            0x82EF4B00 => {
    //   block [0x82EF4B00..0x82EF4B40)
	// 82EF4B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4B0C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4B10: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4B14: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4B18: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF4B1C: 80C10074  lwz r6, 0x74(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4B20: 80A1008C  lwz r5, 0x8c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4B24: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4B28: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4B2C: 4800020D  bl 0x82ef4d38
	ctx.lr = 0x82EF4B30;
	sub_82EF4D38(ctx, base);
	// 82EF4B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4B40 size=92
    let mut pc: u32 = 0x82EF4B40;
    'dispatch: loop {
        match pc {
            0x82EF4B40 => {
    //   block [0x82EF4B40..0x82EF4B9C)
	// 82EF4B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF4B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF4B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4B54: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF4B58: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF4B5C: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82EF4B60: 83E10094  lwz r31, 0x94(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4B64: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82EF4B68: 4BFFABF9  bl 0x82eef760
	ctx.lr = 0x82EF4B6C;
	sub_82EEF760(ctx, base);
	// 82EF4B6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF4B70: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82EF4B74: 4BFFABED  bl 0x82eef760
	ctx.lr = 0x82EF4B78;
	sub_82EEF760(ctx, base);
	// 82EF4B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF4B7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF4B80: 48000241  bl 0x82ef4dc0
	ctx.lr = 0x82EF4B84;
	sub_82EF4DC0(ctx, base);
	// 82EF4B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF4B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF4B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF4B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4BA0 size=148
    let mut pc: u32 = 0x82EF4BA0;
    'dispatch: loop {
        match pc {
            0x82EF4BA0 => {
    //   block [0x82EF4BA0..0x82EF4C34)
	// 82EF4BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4BA4: 4BDB4869  bl 0x82ca940c
	ctx.lr = 0x82EF4BA8;
	sub_82CA93D0(ctx, base);
	// 82EF4BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4BAC: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF4BB0: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82EF4BB4: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82EF4BB8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF4BBC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4BC0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4BC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4BC8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF4BCC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EF4BD0: 48000239  bl 0x82ef4e08
	ctx.lr = 0x82EF4BD4;
	sub_82EF4E08(ctx, base);
	// 82EF4BD4: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF4BD8: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4BDC: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82EF4BE0: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF4BE4: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 82EF4BE8: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82EF4BEC: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 82EF4BF0: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82EF4BF4: 48000045  bl 0x82ef4c38
	ctx.lr = 0x82EF4BF8;
	sub_82EF4C38(ctx, base);
	// 82EF4BF8: 8BE30000  lbz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4BFC: 83C100A4  lwz r30, 0xa4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF4C00: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 82EF4C04: 4BFFAB5D  bl 0x82eef760
	ctx.lr = 0x82EF4C08;
	sub_82EEF760(ctx, base);
	// 82EF4C08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF4C0C: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 82EF4C10: 4BFFAB51  bl 0x82eef760
	ctx.lr = 0x82EF4C14;
	sub_82EEF760(ctx, base);
	// 82EF4C14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF4C18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF4C1C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EF4C20: 88E10059  lbz r7, 0x59(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82EF4C24: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4C28: 480001F1  bl 0x82ef4e18
	ctx.lr = 0x82EF4C2C;
	sub_82EF4E18(ctx, base);
	// 82EF4C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF4C30: 4BDB482C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4C38 size=20
    let mut pc: u32 = 0x82EF4C38;
    'dispatch: loop {
        match pc {
            0x82EF4C38 => {
    //   block [0x82EF4C38..0x82EF4C4C)
	// 82EF4C38: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4C3C: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF4C40: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82EF4C44: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4C50 size=136
    let mut pc: u32 = 0x82EF4C50;
    'dispatch: loop {
        match pc {
            0x82EF4C50 => {
    //   block [0x82EF4C50..0x82EF4CD8)
	// 82EF4C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4C5C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4C60: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4C64: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4C68: 98C1008F  stb r6, 0x8f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(143 as u32), ctx.r[6].u8 ) };
	// 82EF4C6C: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF4C70: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF4C74: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4C78: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4C7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4C80: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4C84: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4C88: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4C8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4C90: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4C94: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF4C98: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF4C9C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4CA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF4CA4: 40990020  ble cr6, 0x82ef4cc4
	if !ctx.cr[6].gt {
	pc = 0x82EF4CC4; continue 'dispatch;
	}
	// 82EF4CA8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4CAC: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF4CB0: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4CB4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4CB8: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF4CBC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4CC0: 4BDB5369  bl 0x82caa028
	ctx.lr = 0x82EF4CC4;
	sub_82CAA028(ctx, base);
	// 82EF4CC4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4CD8 size=20
    let mut pc: u32 = 0x82EF4CD8;
    'dispatch: loop {
        match pc {
            0x82EF4CD8 => {
    //   block [0x82EF4CD8..0x82EF4CEC)
	// 82EF4CD8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4CDC: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF4CE0: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82EF4CE4: 98C1002F  stb r6, 0x2f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(47 as u32), ctx.r[6].u8 ) };
	// 82EF4CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4CF0 size=68
    let mut pc: u32 = 0x82EF4CF0;
    'dispatch: loop {
        match pc {
            0x82EF4CF0 => {
    //   block [0x82EF4CF0..0x82EF4D34)
	// 82EF4CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4CFC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4D00: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4D04: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4D08: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF4D0C: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF4D10: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF4D14: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4D18: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4D1C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4D20: 48000189  bl 0x82ef4ea8
	ctx.lr = 0x82EF4D24;
	sub_82EF4EA8(ctx, base);
	// 82EF4D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4D38 size=132
    let mut pc: u32 = 0x82EF4D38;
    'dispatch: loop {
        match pc {
            0x82EF4D38 => {
    //   block [0x82EF4D38..0x82EF4DBC)
	// 82EF4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4D3C: 4BDB46CD  bl 0x82ca9408
	ctx.lr = 0x82EF4D40;
	sub_82CA93D0(ctx, base);
	// 82EF4D40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4D44: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF4D48: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 82EF4D4C: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82EF4D50: 90C100BC  stw r6, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[6].u32 ) };
	// 82EF4D54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF4D58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4D5C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4D64: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF4D68: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 82EF4D6C: 4800009D  bl 0x82ef4e08
	ctx.lr = 0x82EF4D70;
	sub_82EF4E08(ctx, base);
	// 82EF4D70: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF4D74: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4D78: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82EF4D7C: 8BE10051  lbz r31, 0x51(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF4D80: 83C100BC  lwz r30, 0xbc(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 82EF4D84: 83A100B4  lwz r29, 0xb4(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EF4D88: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 82EF4D8C: 4BFFA9D5  bl 0x82eef760
	ctx.lr = 0x82EF4D90;
	sub_82EEF760(ctx, base);
	// 82EF4D90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF4D94: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EF4D98: 4BFFA9C9  bl 0x82eef760
	ctx.lr = 0x82EF4D9C;
	sub_82EEF760(ctx, base);
	// 82EF4D9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF4DA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF4DA4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF4DA8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82EF4DAC: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4DB0: 48000161  bl 0x82ef4f10
	ctx.lr = 0x82EF4DB4;
	sub_82EF4F10(ctx, base);
	// 82EF4DB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF4DB8: 4BDB46A0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DC0 size=16
    let mut pc: u32 = 0x82EF4DC0;
    'dispatch: loop {
        match pc {
            0x82EF4DC0 => {
    //   block [0x82EF4DC0..0x82EF4DD0)
	// 82EF4DC0: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4DC4: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF4DC8: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82EF4DCC: 48000010  b 0x82ef4ddc
	sub_82EF4DD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4DD0 size=48
    let mut pc: u32 = 0x82EF4DD0;
    'dispatch: loop {
        match pc {
            0x82EF4DD0 => {
    //   block [0x82EF4DD0..0x82EF4E00)
	// 82EF4DD0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4DD4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF4DD8: 91610014  stw r11, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF4DDC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4DE0: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF4DE4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF4DE8: 419A0018  beq cr6, 0x82ef4e00
	if ctx.cr[6].eq {
		sub_82EF4E00(ctx, base);
		return;
	}
	// 82EF4DEC: 81610024  lwz r11, 0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF4DF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4DF4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4DF8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF4DFC: 4BFFFFD4  b 0x82ef4dd0
	pc = 0x82EF4DD0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E00 size=4
    let mut pc: u32 = 0x82EF4E00;
    'dispatch: loop {
        match pc {
            0x82EF4E00 => {
    //   block [0x82EF4E00..0x82EF4E04)
	// 82EF4E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E08 size=12
    let mut pc: u32 = 0x82EF4E08;
    'dispatch: loop {
        match pc {
            0x82EF4E08 => {
    //   block [0x82EF4E08..0x82EF4E14)
	// 82EF4E08: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4E0C: 8861FFF0  lbz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF4E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4E18 size=116
    let mut pc: u32 = 0x82EF4E18;
    'dispatch: loop {
        match pc {
            0x82EF4E18 => {
    //   block [0x82EF4E18..0x82EF4E8C)
	// 82EF4E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4E24: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4E28: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4E2C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4E30: 98C1008F  stb r6, 0x8f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(143 as u32), ctx.r[6].u8 ) };
	// 82EF4E34: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF4E38: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF4E3C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF4E40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4E44: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4E4C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF4E50: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 82EF4E54: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EF4E58: 4BFFA939  bl 0x82eef790
	ctx.lr = 0x82EF4E5C;
	sub_82EEF790(ctx, base);
	// 82EF4E5C: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF4E60: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4E64: 88E10051  lbz r7, 0x51(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF4E68: 88C1008F  lbz r6, 0x8f(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(143 as u32) ) } as u64;
	// 82EF4E6C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4E70: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4E74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4E78: 480000E1  bl 0x82ef4f58
	ctx.lr = 0x82EF4E7C;
	sub_82EF4F58(ctx, base);
	// 82EF4E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF4E90 size=20
    let mut pc: u32 = 0x82EF4E90;
    'dispatch: loop {
        match pc {
            0x82EF4E90 => {
    //   block [0x82EF4E90..0x82EF4EA4)
	// 82EF4E90: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF4E94: 9881001F  stb r4, 0x1f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(31 as u32), ctx.r[4].u8 ) };
	// 82EF4E98: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF4E9C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4EA8 size=104
    let mut pc: u32 = 0x82EF4EA8;
    'dispatch: loop {
        match pc {
            0x82EF4EA8 => {
    //   block [0x82EF4EA8..0x82EF4F10)
	// 82EF4EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4EB4: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF4EB8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF4EBC: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82EF4EC0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF4EC4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4EC8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF4ED0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF4ED4: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4ED8: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82EF4EDC: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 82EF4EE0: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82EF4EE4: 4BFF88AD  bl 0x82eed790
	ctx.lr = 0x82EF4EE8;
	sub_82EED790(ctx, base);
	// 82EF4EE8: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF4EEC: 80A10094  lwz r5, 0x94(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF4EF0: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4EF4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4EF8: 88E10058  lbz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF4EFC: 480000E5  bl 0x82ef4fe0
	ctx.lr = 0x82EF4F00;
	sub_82EF4FE0(ctx, base);
	// 82EF4F00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF4F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4F10 size=72
    let mut pc: u32 = 0x82EF4F10;
    'dispatch: loop {
        match pc {
            0x82EF4F10 => {
    //   block [0x82EF4F10..0x82EF4F58)
	// 82EF4F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4F1C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4F20: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4F24: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4F28: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF4F2C: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF4F30: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF4F34: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF4F38: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4F3C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4F40: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4F44: 48000115  bl 0x82ef5058
	ctx.lr = 0x82EF4F48;
	sub_82EF5058(ctx, base);
	// 82EF4F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4F58 size=136
    let mut pc: u32 = 0x82EF4F58;
    'dispatch: loop {
        match pc {
            0x82EF4F58 => {
    //   block [0x82EF4F58..0x82EF4FE0)
	// 82EF4F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF4F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4F64: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF4F68: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF4F6C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF4F70: 98C1008F  stb r6, 0x8f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(143 as u32), ctx.r[6].u8 ) };
	// 82EF4F74: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF4F78: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF4F7C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF4F80: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4F84: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF4F88: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF4F8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF4F90: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4F94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF4F98: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF4F9C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82EF4FA0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF4FA4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4FA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF4FAC: 40990020  ble cr6, 0x82ef4fcc
	if !ctx.cr[6].gt {
	pc = 0x82EF4FCC; continue 'dispatch;
	}
	// 82EF4FB0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4FB4: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF4FB8: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF4FBC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF4FC0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF4FC4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4FC8: 4BDB5061  bl 0x82caa028
	ctx.lr = 0x82EF4FCC;
	sub_82CAA028(ctx, base);
	// 82EF4FCC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF4FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF4FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF4FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF4FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF4FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF4FE0 size=116
    let mut pc: u32 = 0x82EF4FE0;
    'dispatch: loop {
        match pc {
            0x82EF4FE0 => {
    //   block [0x82EF4FE0..0x82EF5054)
	// 82EF4FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF4FE4: 4BDB4429  bl 0x82ca940c
	ctx.lr = 0x82EF4FE8;
	sub_82CA93D0(ctx, base);
	// 82EF4FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF4FEC: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82EF4FF0: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82EF4FF4: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82EF4FF8: 98C100AF  stb r6, 0xaf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(175 as u32), ctx.r[6].u8 ) };
	// 82EF4FFC: 98E100B7  stb r7, 0xb7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(183 as u32), ctx.r[7].u8 ) };
	// 82EF5000: 8161009C  lwz r11, 0x9c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF5004: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5008: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF500C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF5010: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5014: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82EF5018: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF501C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF5020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5024: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF5028: 8BE10054  lbz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF502C: 83C100A4  lwz r30, 0xa4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5030: 83A1009C  lwz r29, 0x9c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF5034: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 82EF5038: 4BFFA729  bl 0x82eef760
	ctx.lr = 0x82EF503C;
	sub_82EEF760(ctx, base);
	// 82EF503C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF5040: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF5044: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EF5048: 48000099  bl 0x82ef50e0
	ctx.lr = 0x82EF504C;
	sub_82EF50E0(ctx, base);
	// 82EF504C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF5050: 4BDB440C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5058 size=136
    let mut pc: u32 = 0x82EF5058;
    'dispatch: loop {
        match pc {
            0x82EF5058 => {
    //   block [0x82EF5058..0x82EF50E0)
	// 82EF5058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF505C: 4BDB43AD  bl 0x82ca9408
	ctx.lr = 0x82EF5060;
	sub_82CA93D0(ctx, base);
	// 82EF5060: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5064: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF5068: 908100AC  stw r4, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 82EF506C: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82EF5070: 90C100BC  stw r6, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[6].u32 ) };
	// 82EF5074: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF5078: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF507C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5084: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF5088: 388100B4  addi r4, r1, 0xb4
	ctx.r[4].s64 = ctx.r[1].s64 + 180;
	// 82EF508C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EF5090: 4BFFA701  bl 0x82eef790
	ctx.lr = 0x82EF5094;
	sub_82EEF790(ctx, base);
	// 82EF5094: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 82EF5098: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF509C: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82EF50A0: 8BE10051  lbz r31, 0x51(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82EF50A4: 83C100BC  lwz r30, 0xbc(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 82EF50A8: 83A100B4  lwz r29, 0xb4(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82EF50AC: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 82EF50B0: 4BFFA6B1  bl 0x82eef760
	ctx.lr = 0x82EF50B4;
	sub_82EEF760(ctx, base);
	// 82EF50B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF50B8: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EF50BC: 4BFFA6A5  bl 0x82eef760
	ctx.lr = 0x82EF50C0;
	sub_82EEF760(ctx, base);
	// 82EF50C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF50C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF50C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EF50CC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82EF50D0: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF50D4: 4800005D  bl 0x82ef5130
	ctx.lr = 0x82EF50D8;
	sub_82EF5130(ctx, base);
	// 82EF50D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF50DC: 4BDB437C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF50E0 size=20
    let mut pc: u32 = 0x82EF50E0;
    'dispatch: loop {
        match pc {
            0x82EF50E0 => {
    //   block [0x82EF50E0..0x82EF50F4)
	// 82EF50E0: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF50E4: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF50E8: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82EF50EC: 98C1002F  stb r6, 0x2f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(47 as u32), ctx.r[6].u8 ) };
	// 82EF50F0: 4800001C  b 0x82ef510c
	sub_82EF50F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF50F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF50F4 size=56
    let mut pc: u32 = 0x82EF50F4;
    'dispatch: loop {
        match pc {
            0x82EF50F4 => {
    //   block [0x82EF50F4..0x82EF512C)
	// 82EF50F4: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF50F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF50FC: 9161001C  stw r11, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF5100: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5104: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF5108: 91610014  stw r11, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF510C: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF5110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5114: 40990018  ble cr6, 0x82ef512c
	if !ctx.cr[6].gt {
		sub_82EF512C(ctx, base);
		return;
	}
	// 82EF5118: 81610024  lwz r11, 0x24(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF511C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5120: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5124: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5128: 4BFFFFCC  b 0x82ef50f4
	pc = 0x82EF50F4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF512C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF512C size=4
    let mut pc: u32 = 0x82EF512C;
    'dispatch: loop {
        match pc {
            0x82EF512C => {
    //   block [0x82EF512C..0x82EF5130)
	// 82EF512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5130 size=136
    let mut pc: u32 = 0x82EF5130;
    'dispatch: loop {
        match pc {
            0x82EF5130 => {
    //   block [0x82EF5130..0x82EF51B8)
	// 82EF5130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF513C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5140: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5144: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF5148: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF514C: 98E10097  stb r7, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[7].u8 ) };
	// 82EF5150: 9901009F  stb r8, 0x9f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(159 as u32), ctx.r[8].u8 ) };
	// 82EF5154: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5158: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF515C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF5160: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82EF5164: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5168: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF516C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF5170: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5174: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF5178: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF517C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5184: 40990020  ble cr6, 0x82ef51a4
	if !ctx.cr[6].gt {
	pc = 0x82EF51A4; continue 'dispatch;
	}
	// 82EF5188: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF518C: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF5190: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5194: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5198: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF519C: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF51A0: 4BDB4E89  bl 0x82caa028
	ctx.lr = 0x82EF51A4;
	sub_82CAA028(ctx, base);
	// 82EF51A4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF51A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF51AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF51B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF51B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF51B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF51B8 size=8
    let mut pc: u32 = 0x82EF51B8;
    'dispatch: loop {
        match pc {
            0x82EF51B8 => {
    //   block [0x82EF51B8..0x82EF51C0)
	// 82EF51B8: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF51BC: 4BFFB2B4  b 0x82ef0470
	sub_82EF0470(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF51C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF51C0 size=96
    let mut pc: u32 = 0x82EF51C0;
    'dispatch: loop {
        match pc {
            0x82EF51C0 => {
    //   block [0x82EF51C0..0x82EF5220)
	// 82EF51C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF51C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF51C8: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 82EF51CC: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EF51D0: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EF51D4: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EF51D8: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EF51DC: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EF51E0: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EF51E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF51E8: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF51EC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF51F0: 39410078  addi r10, r1, 0x78
	ctx.r[10].s64 = ctx.r[1].s64 + 120;
	// 82EF51F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF51F8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF51FC: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF5204: 481FD225  bl 0x830f2428
	ctx.lr = 0x82EF5208;
	sub_830F2428(ctx, base);
	// 82EF5208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF520C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF521C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5220 size=100
    let mut pc: u32 = 0x82EF5220;
    'dispatch: loop {
        match pc {
            0x82EF5220 => {
    //   block [0x82EF5220..0x82EF5284)
	// 82EF5220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF522C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5230: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5234: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF5238: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF523C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5240: 409A0014  bne cr6, 0x82ef5254
	if !ctx.cr[6].eq {
	pc = 0x82EF5254; continue 'dispatch;
	}
	// 82EF5244: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5248: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82EF524C: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82EF5250: 48000024  b 0x82ef5274
	pc = 0x82EF5274; continue 'dispatch;
	// 82EF5254: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF525C: 409A000C  bne cr6, 0x82ef5268
	if !ctx.cr[6].eq {
	pc = 0x82EF5268; continue 'dispatch;
	}
	// 82EF5260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF5264: 48000010  b 0x82ef5274
	pc = 0x82EF5274; continue 'dispatch;
	// 82EF5268: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF526C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5270: 48113F31  bl 0x830091a0
	ctx.lr = 0x82EF5274;
	sub_830091A0(ctx, base);
	// 82EF5274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF527C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5288 size=204
    let mut pc: u32 = 0x82EF5288;
    'dispatch: loop {
        match pc {
            0x82EF5288 => {
    //   block [0x82EF5288..0x82EF5354)
	// 82EF5288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF528C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5294: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF5298: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF529C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF52A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF52A4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF52A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF52AC: 409A0010  bne cr6, 0x82ef52bc
	if !ctx.cr[6].eq {
	pc = 0x82EF52BC; continue 'dispatch;
	}
	// 82EF52B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF52B4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF52B8: 48000018  b 0x82ef52d0
	pc = 0x82EF52D0; continue 'dispatch;
	// 82EF52BC: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82EF52C0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82EF52C4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF52C8: 4BDB7841  bl 0x82cacb08
	ctx.lr = 0x82EF52CC;
	sub_82CACB08(ctx, base);
	// 82EF52CC: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82EF52D0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF52D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF52D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF52DC: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF52E0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF52E4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF52E8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF52EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF52F0: 419A0044  beq cr6, 0x82ef5334
	if ctx.cr[6].eq {
	pc = 0x82EF5334; continue 'dispatch;
	}
	// 82EF52F4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF52F8: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF52FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5300: 409A0034  bne cr6, 0x82ef5334
	if !ctx.cr[6].eq {
	pc = 0x82EF5334; continue 'dispatch;
	}
	// 82EF5304: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5308: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EF530C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF5310: 41980024  blt cr6, 0x82ef5334
	if ctx.cr[6].lt {
	pc = 0x82EF5334; continue 'dispatch;
	}
	// 82EF5314: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5318: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82EF531C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82EF5320: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF5324: 41990010  bgt cr6, 0x82ef5334
	if ctx.cr[6].gt {
	pc = 0x82EF5334; continue 'dispatch;
	}
	// 82EF5328: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF532C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EF5330: 4800000C  b 0x82ef533c
	pc = 0x82EF533C; continue 'dispatch;
	// 82EF5334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5338: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EF533C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF5340: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF5344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF5348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF534C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5358 size=160
    let mut pc: u32 = 0x82EF5358;
    'dispatch: loop {
        match pc {
            0x82EF5358 => {
    //   block [0x82EF5358..0x82EF53F8)
	// 82EF5358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF535C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5364: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF5368: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF536C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5370: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5374: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF537C: 409A0010  bne cr6, 0x82ef538c
	if !ctx.cr[6].eq {
	pc = 0x82EF538C; continue 'dispatch;
	}
	// 82EF5380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5384: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF5388: 48000018  b 0x82ef53a0
	pc = 0x82EF53A0; continue 'dispatch;
	// 82EF538C: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82EF5390: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF5394: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5398: 4BDB7791  bl 0x82cacb28
	ctx.lr = 0x82EF539C;
	sub_82CACB28(ctx, base);
	// 82EF539C: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EF53A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF53A4: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF53A8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF53AC: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF53B0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF53B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF53B8: 419A0020  beq cr6, 0x82ef53d8
	if ctx.cr[6].eq {
	pc = 0x82EF53D8; continue 'dispatch;
	}
	// 82EF53BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF53C0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF53C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF53C8: 409A0010  bne cr6, 0x82ef53d8
	if !ctx.cr[6].eq {
	pc = 0x82EF53D8; continue 'dispatch;
	}
	// 82EF53CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF53D0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF53D4: 4800000C  b 0x82ef53e0
	pc = 0x82EF53E0; continue 'dispatch;
	// 82EF53D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF53DC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF53E0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF53E4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF53E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF53EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF53F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF53F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF53F8 size=160
    let mut pc: u32 = 0x82EF53F8;
    'dispatch: loop {
        match pc {
            0x82EF53F8 => {
    //   block [0x82EF53F8..0x82EF5498)
	// 82EF53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5404: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF5408: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF540C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5410: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5414: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF541C: 409A0014  bne cr6, 0x82ef5430
	if !ctx.cr[6].eq {
	pc = 0x82EF5430; continue 'dispatch;
	}
	// 82EF5420: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF5424: C80B0D38  lfd f0, 0xd38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3384 as u32) ) };
	// 82EF5428: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82EF542C: 48000014  b 0x82ef5440
	pc = 0x82EF5440; continue 'dispatch;
	// 82EF5430: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF5434: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5438: 48113F91  bl 0x830093c8
	ctx.lr = 0x82EF543C;
	sub_830093C8(ctx, base);
	// 82EF543C: D8210058  stfd f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[1].u64 ) };
	// 82EF5440: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82EF5444: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF5448: D80B0000  stfd f0, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82EF544C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5450: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5454: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF5458: 419A0020  beq cr6, 0x82ef5478
	if ctx.cr[6].eq {
	pc = 0x82EF5478; continue 'dispatch;
	}
	// 82EF545C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5460: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5464: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5468: 40820010  bne 0x82ef5478
	if !ctx.cr[0].eq {
	pc = 0x82EF5478; continue 'dispatch;
	}
	// 82EF546C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF5470: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF5474: 4800000C  b 0x82ef5480
	pc = 0x82EF5480; continue 'dispatch;
	// 82EF5478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF547C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF5480: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF5484: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF5488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF548C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5498 size=164
    let mut pc: u32 = 0x82EF5498;
    'dispatch: loop {
        match pc {
            0x82EF5498 => {
    //   block [0x82EF5498..0x82EF553C)
	// 82EF5498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF549C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF54A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF54A4: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF54A8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF54AC: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF54B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF54B4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF54B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF54BC: 409A0014  bne cr6, 0x82ef54d0
	if !ctx.cr[6].eq {
	pc = 0x82EF54D0; continue 'dispatch;
	}
	// 82EF54C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EF54C4: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF54C8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82EF54CC: 48000018  b 0x82ef54e4
	pc = 0x82EF54E4; continue 'dispatch;
	// 82EF54D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF54D4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF54D8: 4BDBACB9  bl 0x82cb0190
	ctx.lr = 0x82EF54DC;
	sub_82CB0190(ctx, base);
	// 82EF54DC: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EF54E0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82EF54E4: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF54E8: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF54EC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EF54F0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF54F4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF54F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF54FC: 419A0020  beq cr6, 0x82ef551c
	if ctx.cr[6].eq {
	pc = 0x82EF551C; continue 'dispatch;
	}
	// 82EF5500: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5504: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF550C: 409A0010  bne cr6, 0x82ef551c
	if !ctx.cr[6].eq {
	pc = 0x82EF551C; continue 'dispatch;
	}
	// 82EF5510: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF5514: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF5518: 4800000C  b 0x82ef5524
	pc = 0x82EF5524; continue 'dispatch;
	// 82EF551C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5520: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF5524: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF5528: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF552C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF5530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5540 size=160
    let mut pc: u32 = 0x82EF5540;
    'dispatch: loop {
        match pc {
            0x82EF5540 => {
    //   block [0x82EF5540..0x82EF55E0)
	// 82EF5540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF554C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5550: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5554: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF555C: 419A0070  beq cr6, 0x82ef55cc
	if ctx.cr[6].eq {
	pc = 0x82EF55CC; continue 'dispatch;
	}
	// 82EF5560: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5564: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5568: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82EF556C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5570: 386BCE14  addi r3, r11, -0x31ec
	ctx.r[3].s64 = ctx.r[11].s64 + -12780;
	// 82EF5574: 4BDB4C9D  bl 0x82caa210
	ctx.lr = 0x82EF5578;
	sub_82CAA210(ctx, base);
	// 82EF5578: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF557C: 4182001C  beq 0x82ef5598
	if ctx.cr[0].eq {
	pc = 0x82EF5598; continue 'dispatch;
	}
	// 82EF5580: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5584: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5588: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF558C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF5590: 48000040  b 0x82ef55d0
	pc = 0x82EF55D0; continue 'dispatch;
	// 82EF5594: 48000038  b 0x82ef55cc
	pc = 0x82EF55CC; continue 'dispatch;
	// 82EF5598: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF559C: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF55A0: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82EF55A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF55A8: 386BCE0C  addi r3, r11, -0x31f4
	ctx.r[3].s64 = ctx.r[11].s64 + -12788;
	// 82EF55AC: 4BDB4C65  bl 0x82caa210
	ctx.lr = 0x82EF55B0;
	sub_82CAA210(ctx, base);
	// 82EF55B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF55B4: 41820018  beq 0x82ef55cc
	if ctx.cr[0].eq {
	pc = 0x82EF55CC; continue 'dispatch;
	}
	// 82EF55B8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF55BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF55C0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF55C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF55C8: 48000008  b 0x82ef55d0
	pc = 0x82EF55D0; continue 'dispatch;
	// 82EF55CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF55D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF55D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF55D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF55DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF55E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF55E0 size=72
    let mut pc: u32 = 0x82EF55E0;
    'dispatch: loop {
        match pc {
            0x82EF55E0 => {
    //   block [0x82EF55E0..0x82EF5628)
	// 82EF55E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF55E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF55E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF55EC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF55F0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF55F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF55F8: 409A0010  bne cr6, 0x82ef5608
	if !ctx.cr[6].eq {
	pc = 0x82EF5608; continue 'dispatch;
	}
	// 82EF55FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5600: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5604: 48000010  b 0x82ef5614
	pc = 0x82EF5614; continue 'dispatch;
	// 82EF5608: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF560C: 4B285A75  bl 0x8217b080
	ctx.lr = 0x82EF5610;
	sub_8217B080(ctx, base);
	// 82EF5610: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF5614: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF561C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5628 size=104
    let mut pc: u32 = 0x82EF5628;
    'dispatch: loop {
        match pc {
            0x82EF5628 => {
    //   block [0x82EF5628..0x82EF5690)
	// 82EF5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5630: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EF5634: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EF5638: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EF563C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EF5640: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EF5644: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EF5648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF564C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5650: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5654: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF5658: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82EF565C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5660: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5664: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5668: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF566C: 481162C5  bl 0x8300b930
	ctx.lr = 0x82EF5670;
	sub_8300B930(ctx, base);
	// 82EF5670: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82EF5674: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5678: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF567C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF568C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5690 size=56
    let mut pc: u32 = 0x82EF5690;
    'dispatch: loop {
        match pc {
            0x82EF5690 => {
    //   block [0x82EF5690..0x82EF56C8)
	// 82EF5690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF569C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF56A0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF56A4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF56A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF56AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF56B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF56B4: 4E800421  bctrl
	ctx.lr = 0x82EF56B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF56B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF56BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF56C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF56C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF56C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF56C8 size=40
    let mut pc: u32 = 0x82EF56C8;
    'dispatch: loop {
        match pc {
            0x82EF56C8 => {
    //   block [0x82EF56C8..0x82EF56F0)
	// 82EF56C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF56CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF56D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF56D4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF56D8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF56DC: 480081C5  bl 0x82efd8a0
	ctx.lr = 0x82EF56E0;
	sub_82EFD8A0(ctx, base);
	// 82EF56E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF56E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF56E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF56EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF56F0 size=68
    let mut pc: u32 = 0x82EF56F0;
    'dispatch: loop {
        match pc {
            0x82EF56F0 => {
    //   block [0x82EF56F0..0x82EF5734)
	// 82EF56F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF56F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF56F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF56FC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5700: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5704: 80A1007C  lwz r5, 0x7c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5708: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82EF570C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5710: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5714: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5718: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF571C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5720: 4E800421  bctrl
	ctx.lr = 0x82EF5724;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF572C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5738 size=68
    let mut pc: u32 = 0x82EF5738;
    'dispatch: loop {
        match pc {
            0x82EF5738 => {
    //   block [0x82EF5738..0x82EF577C)
	// 82EF5738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5744: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5748: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF574C: 80A1007C  lwz r5, 0x7c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5750: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EF5754: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5758: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF575C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5760: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF5764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5768: 4E800421  bctrl
	ctx.lr = 0x82EF576C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF576C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5780 size=24
    let mut pc: u32 = 0x82EF5780;
    'dispatch: loop {
        match pc {
            0x82EF5780 => {
    //   block [0x82EF5780..0x82EF5798)
	// 82EF5780: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5784: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF578C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5790: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5798 size=40
    let mut pc: u32 = 0x82EF5798;
    'dispatch: loop {
        match pc {
            0x82EF5798 => {
    //   block [0x82EF5798..0x82EF57C0)
	// 82EF5798: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF579C: D8210018  stfd f1, 0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82EF57A0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57A4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82EF57A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF57AC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57B0: C8010018  lfd f0, 0x18(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82EF57B4: D80B0008  stfd f0, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 82EF57B8: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF57C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF57C0 size=36
    let mut pc: u32 = 0x82EF57C0;
    'dispatch: loop {
        match pc {
            0x82EF57C0 => {
    //   block [0x82EF57C0..0x82EF57E4)
	// 82EF57C0: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF57C4: 9881001F  stb r4, 0x1f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(31 as u32), ctx.r[4].u8 ) };
	// 82EF57C8: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57CC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF57D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF57D4: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57D8: 8941001F  lbz r10, 0x1f(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(31 as u32) ) } as u64;
	// 82EF57DC: 994B0008  stb r10, 8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82EF57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF57E8 size=36
    let mut pc: u32 = 0x82EF57E8;
    'dispatch: loop {
        match pc {
            0x82EF57E8 => {
    //   block [0x82EF57E8..0x82EF580C)
	// 82EF57E8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF57EC: D8210018  stfd f1, 0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82EF57F0: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF57F4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82EF57F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF57FC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5800: C8010018  lfd f0, 0x18(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82EF5804: D80B0008  stfd f0, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 82EF5808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5810 size=36
    let mut pc: u32 = 0x82EF5810;
    'dispatch: loop {
        match pc {
            0x82EF5810 => {
    //   block [0x82EF5810..0x82EF5834)
	// 82EF5810: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5814: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF5818: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF581C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82EF5820: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF5824: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5828: 8141001C  lwz r10, 0x1c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF582C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF5830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5838 size=132
    let mut pc: u32 = 0x82EF5838;
    'dispatch: loop {
        match pc {
            0x82EF5838 => {
    //   block [0x82EF5838..0x82EF58BC)
	// 82EF5838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5844: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF5848: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF584C: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82EF5850: 90C1009C  stw r6, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[6].u32 ) };
	// 82EF5854: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82EF5858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF585C: 48000065  bl 0x82ef58c0
	ctx.lr = 0x82EF5860;
	sub_82EF58C0(ctx, base);
	// 82EF5860: 80E1009C  lwz r7, 0x9c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82EF5864: 80C10094  lwz r6, 0x94(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF5868: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EF586C: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF5870: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5874: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5878: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF587C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EF5880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5884: 4E800421  bctrl
	ctx.lr = 0x82EF5888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5888: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF588C: 41820014  beq 0x82ef58a0
	if ctx.cr[0].eq {
	pc = 0x82EF58A0; continue 'dispatch;
	}
	// 82EF5890: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF5894: 48000055  bl 0x82ef58e8
	ctx.lr = 0x82EF5898;
	sub_82EF58E8(ctx, base);
	// 82EF5898: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82EF589C: 4800000C  b 0x82ef58a8
	pc = 0x82EF58A8; continue 'dispatch;
	// 82EF58A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF58A4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EF58A8: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EF58AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF58B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF58B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF58B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF58C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF58C0 size=40
    let mut pc: u32 = 0x82EF58C0;
    'dispatch: loop {
        match pc {
            0x82EF58C0 => {
    //   block [0x82EF58C0..0x82EF58E8)
	// 82EF58C0: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF58C4: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF58C8: 8161001C  lwz r11, 0x1c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF58CC: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF58D0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF58D4: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF58D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF58DC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF58E0: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF58E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF58E8 size=16
    let mut pc: u32 = 0x82EF58E8;
    'dispatch: loop {
        match pc {
            0x82EF58E8 => {
    //   block [0x82EF58E8..0x82EF58F8)
	// 82EF58E8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF58EC: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF58F0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF58F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF58F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF58F8 size=44
    let mut pc: u32 = 0x82EF58F8;
    'dispatch: loop {
        match pc {
            0x82EF58F8 => {
    //   block [0x82EF58F8..0x82EF5924)
	// 82EF58F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF58FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5904: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5908: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF590C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82EF5910: 4BFF78B1  bl 0x82eed1c0
	ctx.lr = 0x82EF5914;
	sub_82EED1C0(ctx, base);
	// 82EF5914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5928 size=16
    let mut pc: u32 = 0x82EF5928;
    'dispatch: loop {
        match pc {
            0x82EF5928 => {
    //   block [0x82EF5928..0x82EF5938)
	// 82EF5928: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF592C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5930: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82EF5934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5938 size=44
    let mut pc: u32 = 0x82EF5938;
    'dispatch: loop {
        match pc {
            0x82EF5938 => {
    //   block [0x82EF5938..0x82EF5964)
	// 82EF5938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5944: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5948: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF594C: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82EF5950: 4BFF7871  bl 0x82eed1c0
	ctx.lr = 0x82EF5954;
	sub_82EED1C0(ctx, base);
	// 82EF5954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5968 size=44
    let mut pc: u32 = 0x82EF5968;
    'dispatch: loop {
        match pc {
            0x82EF5968 => {
    //   block [0x82EF5968..0x82EF5994)
	// 82EF5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5974: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5978: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF597C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82EF5980: 4BFF7841  bl 0x82eed1c0
	ctx.lr = 0x82EF5984;
	sub_82EED1C0(ctx, base);
	// 82EF5984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5998 size=164
    let mut pc: u32 = 0x82EF5998;
    'dispatch: loop {
        match pc {
            0x82EF5998 => {
    //   block [0x82EF5998..0x82EF5A3C)
	// 82EF5998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF599C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF59A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF59A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF59A8: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82EF59AC: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EF59B0: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 82EF59B4: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF59B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF59BC: 409A004C  bne cr6, 0x82ef5a08
	if !ctx.cr[6].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF59C0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF59C4: 896B8F83  lbz r11, -0x707d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28797 as u32) ) } as u64;
	// 82EF59C8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF59CC: 4082003C  bne 0x82ef5a08
	if !ctx.cr[0].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF59D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF59D4: 3BEB8F83  addi r31, r11, -0x707d
	ctx.r[31].s64 = ctx.r[11].s64 + -28797;
	// 82EF59D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF59DC: 38CBCE7C  addi r6, r11, -0x3184
	ctx.r[6].s64 = ctx.r[11].s64 + -12676;
	// 82EF59E0: 38A0003A  li r5, 0x3a
	ctx.r[5].s64 = 58;
	// 82EF59E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF59E8: 388BCE20  addi r4, r11, -0x31e0
	ctx.r[4].s64 = ctx.r[11].s64 + -12768;
	// 82EF59EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF59F0: 4BFF5779  bl 0x82eeb168
	ctx.lr = 0x82EF59F4;
	sub_82EEB168(ctx, base);
	// 82EF59F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF59F8: 481FC3C1  bl 0x830f1db8
	ctx.lr = 0x82EF59FC;
	sub_830F1DB8(ctx, base);
	// 82EF59FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5A00: 40820008  bne 0x82ef5a08
	if !ctx.cr[0].eq {
	pc = 0x82EF5A08; continue 'dispatch;
	}
	// 82EF5A04: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF5A08: 80810094  lwz r4, 0x94(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82EF5A0C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5A10: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5A14: 386AFFEC  addi r3, r10, -0x14
	ctx.r[3].s64 = ctx.r[10].s64 + -20;
	// 82EF5A18: 816BFFEC  lwz r11, -0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82EF5A1C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82EF5A20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5A24: 4E800421  bctrl
	ctx.lr = 0x82EF5A28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF5A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5A40 size=12
    let mut pc: u32 = 0x82EF5A40;
    'dispatch: loop {
        match pc {
            0x82EF5A40 => {
    //   block [0x82EF5A40..0x82EF5A4C)
	// 82EF5A40: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5A44: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82EF5A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5A50 size=168
    let mut pc: u32 = 0x82EF5A50;
    'dispatch: loop {
        match pc {
            0x82EF5A50 => {
    //   block [0x82EF5A50..0x82EF5AF8)
	// 82EF5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5A5C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5A60: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5A64: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A68: 4BFFA8E1  bl 0x82ef0348
	ctx.lr = 0x82EF5A6C;
	sub_82EF0348(ctx, base);
	// 82EF5A6C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A70: 386B04F8  addi r3, r11, 0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + 1272;
	// 82EF5A74: 480000CD  bl 0x82ef5b40
	ctx.lr = 0x82EF5A78;
	sub_82EF5B40(ctx, base);
	// 82EF5A78: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5A7C: 396BCEC8  addi r11, r11, -0x3138
	ctx.r[11].s64 = ctx.r[11].s64 + -12600;
	// 82EF5A80: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A84: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5A88: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5A90: 394ACEBC  addi r10, r10, -0x3144
	ctx.r[10].s64 = ctx.r[10].s64 + -12612;
	// 82EF5A94: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF5A98: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5A9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5AA0: 394ACE8C  addi r10, r10, -0x3174
	ctx.r[10].s64 = ctx.r[10].s64 + -12660;
	// 82EF5AA4: 914B04F8  stw r10, 0x4f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1272 as u32), ctx.r[10].u32 ) };
	// 82EF5AA8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AAC: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AB0: 914B04FC  stw r10, 0x4fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1276 as u32), ctx.r[10].u32 ) };
	// 82EF5AB4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5ABC: 914B0500  stw r10, 0x500(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1280 as u32), ctx.r[10].u32 ) };
	// 82EF5AC0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5AC8: 994B0504  stb r10, 0x504(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1284 as u32), ctx.r[10].u8 ) };
	// 82EF5ACC: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5AD4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5AD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5AE0: 4E800421  bctrl
	ctx.lr = 0x82EF5AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5AE4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5AF8 size=68
    let mut pc: u32 = 0x82EF5AF8;
    'dispatch: loop {
        match pc {
            0x82EF5AF8 => {
    //   block [0x82EF5AF8..0x82EF5B3C)
	// 82EF5AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5B04: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5B08: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5B0C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B10: 480000B1  bl 0x82ef5bc0
	ctx.lr = 0x82EF5B14;
	sub_82EF5BC0(ctx, base);
	// 82EF5B14: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5B18: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5B1C: 4182000C  beq 0x82ef5b28
	if ctx.cr[0].eq {
	pc = 0x82EF5B28; continue 'dispatch;
	}
	// 82EF5B20: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B24: 4BFFFBA5  bl 0x82ef56c8
	ctx.lr = 0x82EF5B28;
	sub_82EF56C8(ctx, base);
	// 82EF5B28: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5B40 size=28
    let mut pc: u32 = 0x82EF5B40;
    'dispatch: loop {
        match pc {
            0x82EF5B40 => {
    //   block [0x82EF5B40..0x82EF5B5C)
	// 82EF5B40: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5B48: 396BCF08  addi r11, r11, -0x30f8
	ctx.r[11].s64 = ctx.r[11].s64 + -12536;
	// 82EF5B4C: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5B50: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5B54: 80610014  lwz r3, 0x14(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5B60 size=68
    let mut pc: u32 = 0x82EF5B60;
    'dispatch: loop {
        match pc {
            0x82EF5B60 => {
    //   block [0x82EF5B60..0x82EF5BA4)
	// 82EF5B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5B6C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5B70: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5B74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B78: 48000031  bl 0x82ef5ba8
	ctx.lr = 0x82EF5B7C;
	sub_82EF5BA8(ctx, base);
	// 82EF5B7C: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5B80: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5B84: 4182000C  beq 0x82ef5b90
	if ctx.cr[0].eq {
	pc = 0x82EF5B90; continue 'dispatch;
	}
	// 82EF5B88: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B8C: 4B94FC25  bl 0x828457b0
	ctx.lr = 0x82EF5B90;
	sub_828457B0(ctx, base);
	// 82EF5B90: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5BA8 size=24
    let mut pc: u32 = 0x82EF5BA8;
    'dispatch: loop {
        match pc {
            0x82EF5BA8 => {
    //   block [0x82EF5BA8..0x82EF5BC0)
	// 82EF5BA8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5BAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5BB0: 396BCF08  addi r11, r11, -0x30f8
	ctx.r[11].s64 = ctx.r[11].s64 + -12536;
	// 82EF5BB4: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5BB8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5BC0 size=132
    let mut pc: u32 = 0x82EF5BC0;
    'dispatch: loop {
        match pc {
            0x82EF5BC0 => {
    //   block [0x82EF5BC0..0x82EF5C44)
	// 82EF5BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5BCC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5BD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5BD4: 396BCEC8  addi r11, r11, -0x3138
	ctx.r[11].s64 = ctx.r[11].s64 + -12600;
	// 82EF5BD8: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BDC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF5BE0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5BE8: 394ACEBC  addi r10, r10, -0x3144
	ctx.r[10].s64 = ctx.r[10].s64 + -12612;
	// 82EF5BEC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF5BF0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5BF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF5BF8: 394ACE8C  addi r10, r10, -0x3174
	ctx.r[10].s64 = ctx.r[10].s64 + -12660;
	// 82EF5BFC: 914B04F8  stw r10, 0x4f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1272 as u32), ctx.r[10].u32 ) };
	// 82EF5C00: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF5C08: 419A0014  beq cr6, 0x82ef5c1c
	if ctx.cr[6].eq {
	pc = 0x82EF5C1C; continue 'dispatch;
	}
	// 82EF5C0C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C10: 396B04F8  addi r11, r11, 0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + 1272;
	// 82EF5C14: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5C18: 4800000C  b 0x82ef5c24
	pc = 0x82EF5C24; continue 'dispatch;
	// 82EF5C1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF5C20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5C24: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5C28: 4BFFFF81  bl 0x82ef5ba8
	ctx.lr = 0x82EF5C2C;
	sub_82EF5BA8(ctx, base);
	// 82EF5C2C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5C30: 4BFFA889  bl 0x82ef04b8
	ctx.lr = 0x82EF5C34;
	sub_82EF04B8(ctx, base);
	// 82EF5C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF5C48 size=60
    let mut pc: u32 = 0x82EF5C48;
    'dispatch: loop {
        match pc {
            0x82EF5C48 => {
    //   block [0x82EF5C48..0x82EF5C84)
	// 82EF5C48: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF5C4C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF5C50: 396BFB08  addi r11, r11, -0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1272;
	// 82EF5C54: 396B0500  addi r11, r11, 0x500
	ctx.r[11].s64 = ctx.r[11].s64 + 1280;
	// 82EF5C58: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82EF5C5C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5C60: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF5C64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF5C68: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF5C6C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5C70: 4082FFE8  bne 0x82ef5c58
	if !ctx.cr[0].eq {
	pc = 0x82EF5C58; continue 'dispatch;
	}
	// 82EF5C74: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF5C78: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82EF5C7C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF5C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5C88 size=344
    let mut pc: u32 = 0x82EF5C88;
    'dispatch: loop {
        match pc {
            0x82EF5C88 => {
    //   block [0x82EF5C88..0x82EF5DE0)
	// 82EF5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5C90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF5C94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5C98: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EF5C9C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5CA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF5CA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5CA8: 4199004C  bgt cr6, 0x82ef5cf4
	if ctx.cr[6].gt {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CAC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5CB0: 896B8F85  lbz r11, -0x707b(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28795 as u32) ) } as u64;
	// 82EF5CB4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5CB8: 4082003C  bne 0x82ef5cf4
	if !ctx.cr[0].eq {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5CC0: 3BEB8F85  addi r31, r11, -0x707b
	ctx.r[31].s64 = ctx.r[11].s64 + -28795;
	// 82EF5CC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5CC8: 38CBC34C  addi r6, r11, -0x3cb4
	ctx.r[6].s64 = ctx.r[11].s64 + -15540;
	// 82EF5CCC: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82EF5CD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5CD4: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 82EF5CD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF5CDC: 4BFF548D  bl 0x82eeb168
	ctx.lr = 0x82EF5CE0;
	sub_82EEB168(ctx, base);
	// 82EF5CE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5CE4: 481FC0D5  bl 0x830f1db8
	ctx.lr = 0x82EF5CE8;
	sub_830F1DB8(ctx, base);
	// 82EF5CE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5CEC: 40820008  bne 0x82ef5cf4
	if !ctx.cr[0].eq {
	pc = 0x82EF5CF4; continue 'dispatch;
	}
	// 82EF5CF0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF5CF4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5CF8: 396BFB08  addi r11, r11, -0x4f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1272;
	// 82EF5CFC: 396B0500  addi r11, r11, 0x500
	ctx.r[11].s64 = ctx.r[11].s64 + 1280;
	// 82EF5D00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82EF5D04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5D08: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF5D0C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EF5D10: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF5D14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF5D18: 4082FFE8  bne 0x82ef5d00
	if !ctx.cr[0].eq {
	pc = 0x82EF5D00; continue 'dispatch;
	}
	// 82EF5D1C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF5D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82EF5D24: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5D28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF5D2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5D30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5D34: 409A0094  bne cr6, 0x82ef5dc8
	if !ctx.cr[6].eq {
	pc = 0x82EF5DC8; continue 'dispatch;
	}
	// 82EF5D38: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5D3C: 896B000C  lbz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF5D40: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5D44: 4182004C  beq 0x82ef5d90
	if ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D48: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5D4C: 896B8F84  lbz r11, -0x707c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-28796 as u32) ) } as u64;
	// 82EF5D50: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF5D54: 4082003C  bne 0x82ef5d90
	if !ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D58: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF5D5C: 3BEB8F84  addi r31, r11, -0x707c
	ctx.r[31].s64 = ctx.r[11].s64 + -28796;
	// 82EF5D60: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5D64: 38CBCF38  addi r6, r11, -0x30c8
	ctx.r[6].s64 = ctx.r[11].s64 + -12488;
	// 82EF5D68: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 82EF5D6C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF5D70: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 82EF5D74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82EF5D78: 4BFF53F1  bl 0x82eeb168
	ctx.lr = 0x82EF5D7C;
	sub_82EEB168(ctx, base);
	// 82EF5D7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF5D80: 481FC039  bl 0x830f1db8
	ctx.lr = 0x82EF5D84;
	sub_830F1DB8(ctx, base);
	// 82EF5D84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF5D88: 40820008  bne 0x82ef5d90
	if !ctx.cr[0].eq {
	pc = 0x82EF5D90; continue 'dispatch;
	}
	// 82EF5D8C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82EF5D90: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5D94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF5D9C: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5DA0: 388BFB08  addi r4, r11, -0x4f8
	ctx.r[4].s64 = ctx.r[11].s64 + -1272;
	// 82EF5DA4: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EF5DA8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5DAC: 4BFF72D5  bl 0x82eed080
	ctx.lr = 0x82EF5DB0;
	sub_82EED080(ctx, base);
	// 82EF5DB0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5DB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5DB8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF5DBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5DC4: 4E800421  bctrl
	ctx.lr = 0x82EF5DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5DC8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5DCC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF5DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF5DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF5DE0 size=84
    let mut pc: u32 = 0x82EF5DE0;
    'dispatch: loop {
        match pc {
            0x82EF5DE0 => {
    //   block [0x82EF5DE0..0x82EF5E34)
	// 82EF5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5DEC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5DF0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82EF5DF4: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82EF5DF8: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 82EF5DFC: D0210094  stfs f1, 0x94(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82EF5E00: D041009C  stfs f2, 0x9c(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82EF5E04: C041009C  lfs f2, 0x9c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82EF5E08: C0210094  lfs f1, 0x94(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82EF5E0C: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82EF5E10: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82EF5E14: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82EF5E18: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E1C: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5E20: 4BFFC261  bl 0x82ef2080
	ctx.lr = 0x82EF5E24;
	sub_82EF2080(ctx, base);
	// 82EF5E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E38 size=44
    let mut pc: u32 = 0x82EF5E38;
    'dispatch: loop {
        match pc {
            0x82EF5E38 => {
    //   block [0x82EF5E38..0x82EF5E64)
	// 82EF5E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5E44: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5E48: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E4C: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5E50: 48000019  bl 0x82ef5e68
	ctx.lr = 0x82EF5E54;
	sub_82EF5E68(ctx, base);
	// 82EF5E54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E68 size=44
    let mut pc: u32 = 0x82EF5E68;
    'dispatch: loop {
        match pc {
            0x82EF5E68 => {
    //   block [0x82EF5E68..0x82EF5E94)
	// 82EF5E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5E74: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5E78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF5E7C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5E80: 4BFFC3B1  bl 0x82ef2230
	ctx.lr = 0x82EF5E84;
	sub_82EF2230(ctx, base);
	// 82EF5E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5E98 size=44
    let mut pc: u32 = 0x82EF5E98;
    'dispatch: loop {
        match pc {
            0x82EF5E98 => {
    //   block [0x82EF5E98..0x82EF5EC4)
	// 82EF5E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5EA4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5EA8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5EAC: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5EB0: 4BFFC451  bl 0x82ef2300
	ctx.lr = 0x82EF5EB4;
	sub_82EF2300(ctx, base);
	// 82EF5EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5EC8 size=52
    let mut pc: u32 = 0x82EF5EC8;
    'dispatch: loop {
        match pc {
            0x82EF5EC8 => {
    //   block [0x82EF5EC8..0x82EF5EFC)
	// 82EF5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5ED4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5ED8: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82EF5EDC: 8881007F  lbz r4, 0x7f(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(127 as u32) ) } as u64;
	// 82EF5EE0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5EE4: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5EE8: 4BFFC529  bl 0x82ef2410
	ctx.lr = 0x82EF5EEC;
	sub_82EF2410(ctx, base);
	// 82EF5EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5F00 size=140
    let mut pc: u32 = 0x82EF5F00;
    'dispatch: loop {
        match pc {
            0x82EF5F00 => {
    //   block [0x82EF5F00..0x82EF5F8C)
	// 82EF5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5F0C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5F10: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F14: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F18: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5F1C: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5F20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5F2C: 4E800421  bctrl
	ctx.lr = 0x82EF5F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5F30: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5F38: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5F3C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F40: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5F44: 4BFFC7C5  bl 0x82ef2708
	ctx.lr = 0x82EF5F48;
	sub_82EF2708(ctx, base);
	// 82EF5F48: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF5F4C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5F54: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5F58: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F5C: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5F60: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5F64: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5F68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5F6C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF5F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5F74: 4E800421  bctrl
	ctx.lr = 0x82EF5F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5F78: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF5F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF5F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF5F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF5F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF5F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF5F90 size=140
    let mut pc: u32 = 0x82EF5F90;
    'dispatch: loop {
        match pc {
            0x82EF5F90 => {
    //   block [0x82EF5F90..0x82EF601C)
	// 82EF5F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF5F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF5F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF5F9C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF5FA0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FA4: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FA8: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5FAC: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5FB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF5FBC: 4E800421  bctrl
	ctx.lr = 0x82EF5FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF5FC0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF5FC8: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5FCC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FD0: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF5FD4: 4BFFCABD  bl 0x82ef2a90
	ctx.lr = 0x82EF5FD8;
	sub_82EF2A90(ctx, base);
	// 82EF5FD8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF5FDC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF5FE4: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF5FE8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FEC: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF5FF0: 394AFB08  addi r10, r10, -0x4f8
	ctx.r[10].s64 = ctx.r[10].s64 + -1272;
	// 82EF5FF4: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 82EF5FF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF5FFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6004: 4E800421  bctrl
	ctx.lr = 0x82EF6008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6008: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF600C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6020 size=44
    let mut pc: u32 = 0x82EF6020;
    'dispatch: loop {
        match pc {
            0x82EF6020 => {
    //   block [0x82EF6020..0x82EF604C)
	// 82EF6020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF602C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF6030: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF6034: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF6038: 48000019  bl 0x82ef6050
	ctx.lr = 0x82EF603C;
	sub_82EF6050(ctx, base);
	// 82EF603C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6050 size=16
    let mut pc: u32 = 0x82EF6050;
    'dispatch: loop {
        match pc {
            0x82EF6050 => {
    //   block [0x82EF6050..0x82EF6060)
	// 82EF6050: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82EF6054: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6058: 806B0044  lwz r3, 0x44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EF605C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6060 size=44
    let mut pc: u32 = 0x82EF6060;
    'dispatch: loop {
        match pc {
            0x82EF6060 => {
    //   block [0x82EF6060..0x82EF608C)
	// 82EF6060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF606C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF6070: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF6074: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF6078: 4BFFD8F1  bl 0x82ef3968
	ctx.lr = 0x82EF607C;
	sub_82EF3968(ctx, base);
	// 82EF607C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6090 size=44
    let mut pc: u32 = 0x82EF6090;
    'dispatch: loop {
        match pc {
            0x82EF6090 => {
    //   block [0x82EF6090..0x82EF60BC)
	// 82EF6090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF609C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EF60A0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82EF60A4: 386BFB08  addi r3, r11, -0x4f8
	ctx.r[3].s64 = ctx.r[11].s64 + -1272;
	// 82EF60A8: 4BFFD941  bl 0x82ef39e8
	ctx.lr = 0x82EF60AC;
	sub_82EF39E8(ctx, base);
	// 82EF60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF60B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF60C0 size=8
    let mut pc: u32 = 0x82EF60C0;
    'dispatch: loop {
        match pc {
            0x82EF60C0 => {
    //   block [0x82EF60C0..0x82EF60C8)
	// 82EF60C0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF60C4: 4BFFFA34  b 0x82ef5af8
	sub_82EF5AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF60C8 size=8
    let mut pc: u32 = 0x82EF60C8;
    'dispatch: loop {
        match pc {
            0x82EF60C8 => {
    //   block [0x82EF60C8..0x82EF60D0)
	// 82EF60C8: 3863FB08  addi r3, r3, -0x4f8
	ctx.r[3].s64 = ctx.r[3].s64 + -1272;
	// 82EF60CC: 4BFFFA2C  b 0x82ef5af8
	sub_82EF5AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF60D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF60D0 size=116
    let mut pc: u32 = 0x82EF60D0;
    'dispatch: loop {
        match pc {
            0x82EF60D0 => {
    //   block [0x82EF60D0..0x82EF6144)
	// 82EF60D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF60D4: 4BDB3339  bl 0x82ca940c
	ctx.lr = 0x82EF60D8;
	sub_82CA93D0(ctx, base);
	// 82EF60D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF60DC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF60E0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF60E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF60E8: 409A0008  bne cr6, 0x82ef60f0
	if !ctx.cr[6].eq {
	pc = 0x82EF60F0; continue 'dispatch;
	}
	// 82EF60EC: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82EF60F0: 3BDD0002  addi r30, r29, 2
	ctx.r[30].s64 = ctx.r[29].s64 + 2;
	// 82EF60F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF60F8: 7D7E2214  add r11, r30, r4
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[4].u64;
	// 82EF60FC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6100: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6104: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82EF6108: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EF610C: 4E800421  bctrl
	ctx.lr = 0x82EF6110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF6114: 41820028  beq 0x82ef613c
	if ctx.cr[0].eq {
	pc = 0x82EF613C; continue 'dispatch;
	}
	// 82EF6118: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82EF611C: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 82EF6120: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6124: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF6128: 7D6B5078  andc r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[10].u64;
	// 82EF612C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EF6130: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82EF6134: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF6138: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 82EF613C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6140: 4BDB331C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6148 size=8
    let mut pc: u32 = 0x82EF6148;
    'dispatch: loop {
        match pc {
            0x82EF6148 => {
    //   block [0x82EF6148..0x82EF6150)
	// 82EF6148: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF614C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6150 size=24
    let mut pc: u32 = 0x82EF6150;
    'dispatch: loop {
        match pc {
            0x82EF6150 => {
    //   block [0x82EF6150..0x82EF6168)
	// 82EF6150: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6154: A144FFFE  lhz r10, -2(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6158: 7C8A2050  subf r4, r10, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82EF615C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6164: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6168 size=4
    let mut pc: u32 = 0x82EF6168;
    'dispatch: loop {
        match pc {
            0x82EF6168 => {
    //   block [0x82EF6168..0x82EF616C)
	// 82EF6168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6170 size=8
    let mut pc: u32 = 0x82EF6170;
    'dispatch: loop {
        match pc {
            0x82EF6170 => {
    //   block [0x82EF6170..0x82EF6178)
	// 82EF6170: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EF6174: 4BDB5FCC  b 0x82cac140
	sub_82CAC140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6178 size=12
    let mut pc: u32 = 0x82EF6178;
    'dispatch: loop {
        match pc {
            0x82EF6178 => {
    //   block [0x82EF6178..0x82EF6184)
	// 82EF6178: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EF617C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF6180: 4BDB8E48  b 0x82caefc8
	sub_82CAEFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6188 size=116
    let mut pc: u32 = 0x82EF6188;
    'dispatch: loop {
        match pc {
            0x82EF6188 => {
    //   block [0x82EF6188..0x82EF61FC)
	// 82EF6188: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82EF618C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82EF6190: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 82EF6194: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6198: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EF619C: 916B000C  stw r11, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF61A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61A4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF61A8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EF61AC: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82EF61B0: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82EF61B4: 4082FFDC  bne 0x82ef6190
	if !ctx.cr[0].eq {
	pc = 0x82EF6190; continue 'dispatch;
	}
	// 82EF61B8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82EF61BC: 39430014  addi r10, r3, 0x14
	ctx.r[10].s64 = ctx.r[3].s64 + 20;
	// 82EF61C0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 82EF61C4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61C8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF61CC: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF61D0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF61D4: 4082FFF0  bne 0x82ef61c4
	if !ctx.cr[0].eq {
	pc = 0x82EF61C4; continue 'dispatch;
	}
	// 82EF61D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EF61DC: 39430294  addi r10, r3, 0x294
	ctx.r[10].s64 = ctx.r[3].s64 + 660;
	// 82EF61E0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 82EF61E4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF61E8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF61EC: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF61F0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF61F4: 4082FFF0  bne 0x82ef61e4
	if !ctx.cr[0].eq {
	pc = 0x82EF61E4; continue 'dispatch;
	}
	// 82EF61F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6200 size=212
    let mut pc: u32 = 0x82EF6200;
    'dispatch: loop {
        match pc {
            0x82EF6200 => {
    //   block [0x82EF6200..0x82EF62D4)
	// 82EF6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF620C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF621C: 817F0504  lwz r11, 0x504(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6224: 409A000C  bne cr6, 0x82ef6230
	if !ctx.cr[6].eq {
	pc = 0x82EF6230; continue 'dispatch;
	}
	// 82EF6228: 48007629  bl 0x82efd850
	ctx.lr = 0x82EF622C;
	sub_82EFD850(ctx, base);
	// 82EF622C: 907F0504  stw r3, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[3].u32 ) };
	// 82EF6230: 807F0504  lwz r3, 0x504(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6234: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 82EF6238: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF623C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6240: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82EF6244: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6248: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EF624C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6250: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF6254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6258: 4E800421  bctrl
	ctx.lr = 0x82EF625C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF625C: 7C6A1B79  or. r10, r3, r3
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6260: 41820058  beq 0x82ef62b8
	if ctx.cr[0].eq {
	pc = 0x82EF62B8; continue 'dispatch;
	}
	// 82EF6264: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF6268: 392A0004  addi r9, r10, 4
	ctx.r[9].s64 = ctx.r[10].s64 + 4;
	// 82EF626C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6270: 391F0004  addi r8, r31, 4
	ctx.r[8].s64 = ctx.r[31].s64 + 4;
	// 82EF6274: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF6278: 57C6063E  clrlwi r6, r30, 0x18
	ctx.r[6].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EF627C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF6280: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6284: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6288: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF628C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6290: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6294: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6298: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF629C: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF62A0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF62A4: 98CA0002  stb r6, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82EF62A8: 98AA0003  stb r5, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82EF62AC: 98CBFFFE  stb r6, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[6].u8 ) };
	// 82EF62B0: 988BFFFF  stb r4, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[4].u8 ) };
	// 82EF62B4: 48000008  b 0x82ef62bc
	pc = 0x82EF62BC; continue 'dispatch;
	// 82EF62B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF62BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF62C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF62C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF62C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF62CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF62D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF62D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF62D8 size=156
    let mut pc: u32 = 0x82EF62D8;
    'dispatch: loop {
        match pc {
            0x82EF62D8 => {
    //   block [0x82EF62D8..0x82EF6374)
	// 82EF62D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF62DC: 4BDB312D  bl 0x82ca9408
	ctx.lr = 0x82EF62E0;
	sub_82CA93D0(ctx, base);
	// 82EF62E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF62E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF62E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF62EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF62F0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EF62F4: 817E0508  lwz r11, 0x508(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF62F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF62FC: 409A000C  bne cr6, 0x82ef6308
	if !ctx.cr[6].eq {
	pc = 0x82EF6308; continue 'dispatch;
	}
	// 82EF6300: 48007551  bl 0x82efd850
	ctx.lr = 0x82EF6304;
	sub_82EFD850(ctx, base);
	// 82EF6304: 907E0508  stw r3, 0x508(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1288 as u32), ctx.r[3].u32 ) };
	// 82EF6308: 807E0508  lwz r3, 0x508(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF630C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF6310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6314: 419A0020  beq cr6, 0x82ef6334
	if ctx.cr[6].eq {
	pc = 0x82EF6334; continue 'dispatch;
	}
	// 82EF6318: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF631C: 38DD0008  addi r6, r29, 8
	ctx.r[6].s64 = ctx.r[29].s64 + 8;
	// 82EF6320: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6324: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF6328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF632C: 4E800421  bctrl
	ctx.lr = 0x82EF6330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6330: 48000018  b 0x82ef6348
	pc = 0x82EF6348; continue 'dispatch;
	// 82EF6334: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82EF6338: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF633C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EF6340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6344: 4E800421  bctrl
	ctx.lr = 0x82EF6348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6348: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF634C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6350: 419A001C  beq cr6, 0x82ef636c
	if ctx.cr[6].eq {
	pc = 0x82EF636C; continue 'dispatch;
	}
	// 82EF6354: 394000B1  li r10, 0xb1
	ctx.r[10].s64 = 177;
	// 82EF6358: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EF635C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF6360: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EF6364: 994B0006  stb r10, 6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 82EF6368: 992B0007  stb r9, 7(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(7 as u32), ctx.r[9].u8 ) };
	// 82EF636C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6370: 4BDB30E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6378 size=684
    let mut pc: u32 = 0x82EF6378;
    'dispatch: loop {
        match pc {
            0x82EF6378 => {
    //   block [0x82EF6378..0x82EF6624)
	// 82EF6378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF637C: 4BDB307D  bl 0x82ca93f8
	ctx.lr = 0x82EF6380;
	sub_82CA93D0(ctx, base);
	// 82EF6380: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6384: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EF6388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF638C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF6390: 3B660004  addi r27, r6, 4
	ctx.r[27].s64 = ctx.r[6].s64 + 4;
	// 82EF6394: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF6398: 419A0068  beq cr6, 0x82ef6400
	if ctx.cr[6].eq {
	pc = 0x82EF6400; continue 'dispatch;
	}
	// 82EF639C: 2B1A1000  cmplwi cr6, r26, 0x1000
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4096 as u32, &mut ctx.xer);
	// 82EF63A0: 40990054  ble cr6, 0x82ef63f4
	if !ctx.cr[6].gt {
	pc = 0x82EF63F4; continue 'dispatch;
	}
	// 82EF63A4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF63A8: 4BFFFF31  bl 0x82ef62d8
	ctx.lr = 0x82EF63AC;
	sub_82EF62D8(ctx, base);
	// 82EF63AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF63B0: 41820150  beq 0x82ef6500
	if ctx.cr[0].eq {
	pc = 0x82EF6500; continue 'dispatch;
	}
	// 82EF63B4: 3BDF0550  addi r30, r31, 0x550
	ctx.r[30].s64 = ctx.r[31].s64 + 1360;
	// 82EF63B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF63BC: 483C35A9  bl 0x832b9964
	ctx.lr = 0x82EF63C0;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF63C0: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82EF63C4: E8FF0530  ld r7, 0x530(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF63C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF63CC: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 82EF63D0: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF63D4: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF63D8: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82EF63DC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EF63E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF63E4: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF63E8: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF63EC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF63F0: 48000224  b 0x82ef6614
	pc = 0x82EF6614; continue 'dispatch;
	// 82EF63F4: 2B1A0004  cmplwi cr6, r26, 4
	ctx.cr[6].compare_u32(ctx.r[26].u32, 4 as u32, &mut ctx.xer);
	// 82EF63F8: 40980008  bge cr6, 0x82ef6400
	if !ctx.cr[6].lt {
	pc = 0x82EF6400; continue 'dispatch;
	}
	// 82EF63FC: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 82EF6400: 7D7CD214  add r11, r28, r26
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[26].u64;
	// 82EF6404: 3B1F0550  addi r24, r31, 0x550
	ctx.r[24].s64 = ctx.r[31].s64 + 1360;
	// 82EF6408: 7F2BDA14  add r25, r11, r27
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EF640C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6410: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 82EF6414: 616B000F  ori r11, r11, 0xf
	ctx.r[11].u64 = ctx.r[11].u64 | 15;
	// 82EF6418: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF641C: 23AB001C  subfic r29, r11, 0x1c
	ctx.xer.ca = ctx.r[11].u32 <= 28 as u32;
	ctx.r[29].s64 = (28 as i64) - ctx.r[11].s64;
	// 82EF6420: 483C3545  bl 0x832b9964
	ctx.lr = 0x82EF6424;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6424: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 82EF6428: 817F0510  lwz r11, 0x510(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1296 as u32) ) } as u64;
	// 82EF642C: 1D4A0014  mulli r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 * 20;
	// 82EF6430: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF6434: 917F0510  stw r11, 0x510(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1296 as u32), ctx.r[11].u32 ) };
	// 82EF6438: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF643C: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82EF6440: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82EF6444: 4198000C  blt cr6, 0x82ef6450
	if ctx.cr[6].lt {
	pc = 0x82EF6450; continue 'dispatch;
	}
	// 82EF6448: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82EF644C: 4800000C  b 0x82ef6458
	pc = 0x82EF6458; continue 'dispatch;
	// 82EF6450: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82EF6454: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 82EF6458: 1D7D0014  mulli r11, r29, 0x14
	ctx.r[11].s64 = ctx.r[29].s64 * 20;
	// 82EF645C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF6460: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82EF6464: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6468: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF646C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6470: 4800001C  b 0x82ef648c
	pc = 0x82EF648C; continue 'dispatch;
	// 82EF6474: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF6478: 41990024  bgt cr6, 0x82ef649c
	if ctx.cr[6].gt {
	pc = 0x82EF649C; continue 'dispatch;
	}
	// 82EF647C: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82EF6480: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF6484: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6488: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF648C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EF6490: 419AFFE4  beq cr6, 0x82ef6474
	if ctx.cr[6].eq {
	pc = 0x82EF6474; continue 'dispatch;
	}
	// 82EF6494: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF6498: 4099008C  ble cr6, 0x82ef6524
	if !ctx.cr[6].gt {
	pc = 0x82EF6524; continue 'dispatch;
	}
	// 82EF649C: 7F1D4800  cmpw cr6, r29, r9
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF64A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF64A4: 40990064  ble cr6, 0x82ef6508
	if !ctx.cr[6].gt {
	pc = 0x82EF6508; continue 'dispatch;
	}
	// 82EF64A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF64AC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EF64B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF64B4: 4BFFFE25  bl 0x82ef62d8
	ctx.lr = 0x82EF64B8;
	sub_82EF62D8(ctx, base);
	// 82EF64B8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF64BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF64C0: 4182003C  beq 0x82ef64fc
	if ctx.cr[0].eq {
	pc = 0x82EF64FC; continue 'dispatch;
	}
	// 82EF64C4: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82EF64C8: E8FF0530  ld r7, 0x530(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF64CC: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF64D0: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 82EF64D4: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF64D8: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82EF64DC: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82EF64E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF64E4: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF64E8: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF64EC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF64F0: 483C3465  bl 0x832b9954
	ctx.lr = 0x82EF64F4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF64F4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82EF64F8: 48000124  b 0x82ef661c
	pc = 0x82EF661C; continue 'dispatch;
	// 82EF64FC: 483C3459  bl 0x832b9954
	ctx.lr = 0x82EF6500;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6500: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF6504: 48000118  b 0x82ef661c
	pc = 0x82EF661C; continue 'dispatch;
	// 82EF6508: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82EF650C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF6510: 4BFFFCF1  bl 0x82ef6200
	ctx.lr = 0x82EF6514;
	sub_82EF6200(ctx, base);
	// 82EF6514: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF6518: 4082000C  bne 0x82ef6524
	if !ctx.cr[0].eq {
	pc = 0x82EF6524; continue 'dispatch;
	}
	// 82EF651C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6520: 4BFFFFDC  b 0x82ef64fc
	pc = 0x82EF64FC; continue 'dispatch;
	// 82EF6524: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 82EF6528: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF652C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6530: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6534: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6538: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF653C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6540: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6544: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF6548: 48000044  b 0x82ef658c
	pc = 0x82EF658C; continue 'dispatch;
	// 82EF654C: 892BFFFF  lbz r9, -1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF6550: 394AFFEC  addi r10, r10, -0x14
	ctx.r[10].s64 = ctx.r[10].s64 + -20;
	// 82EF6554: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82EF6558: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EF655C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EF6560: 992BFFFF  stb r9, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 82EF6564: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF6568: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF656C: 9BC9FFFE  stb r30, -2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-2 as u32), ctx.r[30].u8 ) };
	// 82EF6570: 98E9FFFF  stb r7, -1(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-1 as u32), ctx.r[7].u8 ) };
	// 82EF6574: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6578: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF657C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6580: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6584: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6588: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF658C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EF6590: 4199FFBC  bgt cr6, 0x82ef654c
	if ctx.cr[6].gt {
	pc = 0x82EF654C; continue 'dispatch;
	}
	// 82EF6594: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82EF6598: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF659C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82EF65A0: 512A3830  rlwimi r10, r9, 7, 0, 0x18
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(7) as u64) & 0x00000000FFFFFF80) | (ctx.r[10].u64 & 0xFFFFFFFF0000007F);
	// 82EF65A4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF65A8: 994BFFFE  stb r10, -2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u8 ) };
	// 82EF65AC: 419A0034  beq cr6, 0x82ef65e0
	if ctx.cr[6].eq {
	pc = 0x82EF65E0; continue 'dispatch;
	}
	// 82EF65B0: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82EF65B4: 393AFFFF  addi r9, r26, -1
	ctx.r[9].s64 = ctx.r[26].s64 + -1;
	// 82EF65B8: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82EF65BC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EF65C0: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82EF65C4: 7D5B5050  subf r10, r27, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[27].s64;
	// 82EF65C8: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF65CC: 41820014  beq 0x82ef65e0
	if ctx.cr[0].eq {
	pc = 0x82EF65E0; continue 'dispatch;
	}
	// 82EF65D0: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82EF65D4: B16AFFFC  sth r11, -4(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u16 ) };
	// 82EF65D8: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82EF65DC: 992AFFFF  stb r9, -1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-1 as u32), ctx.r[9].u8 ) };
	// 82EF65E0: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82EF65E4: E91F0530  ld r8, 0x530(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) };
	// 82EF65E8: 3979FFFC  addi r11, r25, -4
	ctx.r[11].s64 = ctx.r[25].s64 + -4;
	// 82EF65EC: 1D4A0014  mulli r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 * 20;
	// 82EF65F0: 7D2AFAAA  lwax r9, r10, r31
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as i32) as i64;
	// 82EF65F4: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82EF65F8: E93F0520  ld r9, 0x520(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF65FC: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6600: F91F0530  std r8, 0x530(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[8].u64 ) };
	// 82EF6604: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82EF6608: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF660C: F95F0520  std r10, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[10].u64 ) };
	// 82EF6610: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6614: 483C3341  bl 0x832b9954
	ctx.lr = 0x82EF6618;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6618: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82EF661C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6620: 4BDB2E28  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6628 size=400
    let mut pc: u32 = 0x82EF6628;
    'dispatch: loop {
        match pc {
            0x82EF6628 => {
    //   block [0x82EF6628..0x82EF67B8)
	// 82EF6628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF662C: 4BDB2DD5  bl 0x82ca9400
	ctx.lr = 0x82EF6630;
	sub_82CA93D0(ctx, base);
	// 82EF6630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6638: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF663C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6640: 419A0170  beq cr6, 0x82ef67b0
	if ctx.cr[6].eq {
	pc = 0x82EF67B0; continue 'dispatch;
	}
	// 82EF6644: 8944FFFB  lbz r10, -5(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(-5 as u32) ) } as u64;
	// 82EF6648: 3964FFFC  addi r11, r4, -4
	ctx.r[11].s64 = ctx.r[4].s64 + -4;
	// 82EF664C: 8384FFFC  lwz r28, -4(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF6650: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6654: 4182000C  beq 0x82ef6660
	if ctx.cr[0].eq {
	pc = 0x82EF6660; continue 'dispatch;
	}
	// 82EF6658: A14BFFFC  lhz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF665C: 48000008  b 0x82ef6664
	pc = 0x82EF6664; continue 'dispatch;
	// 82EF6660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF6664: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82EF6668: 3B5F0550  addi r26, r31, 0x550
	ctx.r[26].s64 = ctx.r[31].s64 + 1360;
	// 82EF666C: 7FAA5850  subf r29, r10, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF6670: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF6674: 897DFFFE  lbz r11, -2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6678: 557E067E  clrlwi r30, r11, 0x19
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82EF667C: 483C32E9  bl 0x832b9964
	ctx.lr = 0x82EF6680;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6680: 815F0514  lwz r10, 0x514(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1300 as u32) ) } as u64;
	// 82EF6684: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6688: E93F0528  ld r9, 0x528(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1320 as u32) ) };
	// 82EF668C: 2B1E0030  cmplwi cr6, r30, 0x30
	ctx.cr[6].compare_u32(ctx.r[30].u32, 48 as u32, &mut ctx.xer);
	// 82EF6690: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF6694: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF6698: 915F0514  stw r10, 0x514(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1300 as u32), ctx.r[10].u32 ) };
	// 82EF669C: F93F0528  std r9, 0x528(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[9].u64 ) };
	// 82EF66A0: E95F0538  ld r10, 0x538(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1336 as u32) ) };
	// 82EF66A4: 40990028  ble cr6, 0x82ef66cc
	if !ctx.cr[6].gt {
	pc = 0x82EF66CC; continue 'dispatch;
	}
	// 82EF66A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF66AC: F97F0538  std r11, 0x538(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[11].u64 ) };
	// 82EF66B0: 807F0508  lwz r3, 0x508(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82EF66B4: 5769063F  clrlwi. r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF66B8: 389DFFF8  addi r4, r29, -8
	ctx.r[4].s64 = ctx.r[29].s64 + -8;
	// 82EF66BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF66C0: 408200B0  bne 0x82ef6770
	if !ctx.cr[0].eq {
	pc = 0x82EF6770; continue 'dispatch;
	}
	// 82EF66C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF66C8: 480000AC  b 0x82ef6774
	pc = 0x82EF6774; continue 'dispatch;
	// 82EF66CC: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82EF66D0: 57C906B4  rlwinm r9, r30, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF66D4: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 82EF66D8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF66DC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82EF66E0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82EF66E4: E96B0002  lwa r11, 0(r11)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 82EF66E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF66EC: F97F0538  std r11, 0x538(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[11].u64 ) };
	// 82EF66F0: 895DFFFF  lbz r10, -1(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF66F4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF66F8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF66FC: 4082000C  bne 0x82ef6708
	if !ctx.cr[0].eq {
	pc = 0x82EF6708; continue 'dispatch;
	}
	// 82EF6700: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82EF6704: 48000008  b 0x82ef670c
	pc = 0x82EF670C; continue 'dispatch;
	// 82EF6708: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EF670C: 7F1E4840  cmplw cr6, r30, r9
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF6710: 41990054  bgt cr6, 0x82ef6764
	if ctx.cr[6].gt {
	pc = 0x82EF6764; continue 'dispatch;
	}
	// 82EF6714: 894BFFFE  lbz r10, -2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82EF6718: 57C7063E  clrlwi r7, r30, 0x18
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82EF671C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF6720: 409A0060  bne cr6, 0x82ef6780
	if !ctx.cr[6].eq {
	pc = 0x82EF6780; continue 'dispatch;
	}
	// 82EF6724: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6728: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EF672C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6730: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF6734: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6738: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF673C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF6740: 40980008  bge cr6, 0x82ef6748
	if !ctx.cr[6].lt {
	pc = 0x82EF6748; continue 'dispatch;
	}
	// 82EF6744: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82EF6748: 897DFFFF  lbz r11, -1(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82EF674C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF6750: 39080014  addi r8, r8, 0x14
	ctx.r[8].s64 = ctx.r[8].s64 + 20;
	// 82EF6754: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 82EF6758: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF675C: 997DFFFF  stb r11, -1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-1 as u32), ctx.r[11].u8 ) };
	// 82EF6760: 4BFFFF90  b 0x82ef66f0
	pc = 0x82EF66F0; continue 'dispatch;
	// 82EF6764: 807F0504  lwz r3, 0x504(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 82EF6768: 389DFFFC  addi r4, r29, -4
	ctx.r[4].s64 = ctx.r[29].s64 + -4;
	// 82EF676C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6770: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6778: 4E800421  bctrl
	ctx.lr = 0x82EF677C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF677C: 4800002C  b 0x82ef67a8
	pc = 0x82EF67A8; continue 'dispatch;
	// 82EF6780: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 82EF6784: 9BDDFFFE  stb r30, -2(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(-2 as u32), ctx.r[30].u8 ) };
	// 82EF6788: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF678C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF6790: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6794: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF6798: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF679C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF67A0: 93AA0004  stw r29, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF67A4: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF67A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF67AC: 483C31A9  bl 0x832b9954
	ctx.lr = 0x82EF67B0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF67B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF67B4: 4BDB2C9C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF67B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF67B8 size=420
    let mut pc: u32 = 0x82EF67B8;
    'dispatch: loop {
        match pc {
            0x82EF67B8 => {
    //   block [0x82EF67B8..0x82EF695C)
	// 82EF67B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF67BC: 4BDB2C45  bl 0x82ca9400
	ctx.lr = 0x82EF67C0;
	sub_82CA93D0(ctx, base);
	// 82EF67C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF67C4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EF67C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF67CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF67D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF67D4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF67D8: 419A0008  beq cr6, 0x82ef67e0
	if ctx.cr[6].eq {
	pc = 0x82EF67E0; continue 'dispatch;
	}
	// 82EF67DC: 83BAFFFC  lwz r29, -4(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF67E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF67E4: 409A0024  bne cr6, 0x82ef6808
	if !ctx.cr[6].eq {
	pc = 0x82EF6808; continue 'dispatch;
	}
	// 82EF67E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF67EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF67F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF67F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF67F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF67FC: 4E800421  bctrl
	ctx.lr = 0x82EF6800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF6804: 48000150  b 0x82ef6954
	pc = 0x82EF6954; continue 'dispatch;
	// 82EF6808: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EF680C: 419A0034  beq cr6, 0x82ef6840
	if ctx.cr[6].eq {
	pc = 0x82EF6840; continue 'dispatch;
	}
	// 82EF6810: 897AFFFA  lbz r11, -6(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(-6 as u32) ) } as u64;
	// 82EF6814: 556B067E  clrlwi r11, r11, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82EF6818: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 82EF681C: 4099000C  ble cr6, 0x82ef6828
	if !ctx.cr[6].gt {
	pc = 0x82EF6828; continue 'dispatch;
	}
	// 82EF6820: 817AFFF4  lwz r11, -0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EF6824: 48000014  b 0x82ef6838
	pc = 0x82EF6838; continue 'dispatch;
	// 82EF6828: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF682C: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 82EF6830: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EF6834: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82EF6838: 3B8BFFFC  addi r28, r11, -4
	ctx.r[28].s64 = ctx.r[11].s64 + -4;
	// 82EF683C: 48000008  b 0x82ef6844
	pc = 0x82EF6844; continue 'dispatch;
	// 82EF6840: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF6844: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF6848: 41990084  bgt cr6, 0x82ef68cc
	if ctx.cr[6].gt {
	pc = 0x82EF68CC; continue 'dispatch;
	}
	// 82EF684C: 578BF87E  srwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF6850: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6854: 40990070  ble cr6, 0x82ef68c4
	if !ctx.cr[6].gt {
	pc = 0x82EF68C4; continue 'dispatch;
	}
	// 82EF6858: 3B9F0550  addi r28, r31, 0x550
	ctx.r[28].s64 = ctx.r[31].s64 + 1360;
	// 82EF685C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF6860: 483C3105  bl 0x832b9964
	ctx.lr = 0x82EF6864;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6864: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82EF6868: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82EF686C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF6870: 917F0518  stw r11, 0x518(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1304 as u32), ctx.r[11].u32 ) };
	// 82EF6874: 4099001C  ble cr6, 0x82ef6890
	if !ctx.cr[6].gt {
	pc = 0x82EF6890; continue 'dispatch;
	}
	// 82EF6878: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82EF687C: E95F0520  ld r10, 0x520(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) };
	// 82EF6880: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF6884: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF6888: F97F0520  std r11, 0x520(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u64 ) };
	// 82EF688C: 48000018  b 0x82ef68a4
	pc = 0x82EF68A4; continue 'dispatch;
	// 82EF6890: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82EF6894: E95F0528  ld r10, 0x528(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(1320 as u32) ) };
	// 82EF6898: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF689C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF68A0: F97F0528  std r11, 0x528(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[11].u64 ) };
	// 82EF68A4: 817AFFFC  lwz r11, -4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EF68A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF68AC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EF68B0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF68B4: 917AFFFC  stw r11, -4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82EF68B8: 483C309D  bl 0x832b9954
	ctx.lr = 0x82EF68BC;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF68BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF68C0: 48000094  b 0x82ef6954
	pc = 0x82EF6954; continue 'dispatch;
	// 82EF68C4: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EF68C8: 48000008  b 0x82ef68d0
	pc = 0x82EF68D0; continue 'dispatch;
	// 82EF68CC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82EF68D0: 3BBF0550  addi r29, r31, 0x550
	ctx.r[29].s64 = ctx.r[31].s64 + 1360;
	// 82EF68D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF68D8: 483C308D  bl 0x832b9964
	ctx.lr = 0x82EF68DC;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF68DC: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82EF68E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF68E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF68E8: 917F0518  stw r11, 0x518(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1304 as u32), ctx.r[11].u32 ) };
	// 82EF68EC: 483C3069  bl 0x832b9954
	ctx.lr = 0x82EF68F0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF68F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF68F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF68F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF68FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6904: 4E800421  bctrl
	ctx.lr = 0x82EF6908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6908: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF690C: 40820014  bne 0x82ef6920
	if !ctx.cr[0].eq {
	pc = 0x82EF6920; continue 'dispatch;
	}
	// 82EF6910: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF6914: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EF6918: 4099003C  ble cr6, 0x82ef6954
	if !ctx.cr[6].gt {
	pc = 0x82EF6954; continue 'dispatch;
	}
	// 82EF691C: 4BFFFEE4  b 0x82ef6800
	pc = 0x82EF6800; continue 'dispatch;
	// 82EF6920: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EF6924: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF6928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF692C: 4BDB2B55  bl 0x82ca9480
	ctx.lr = 0x82EF6930;
	sub_82CA9480(ctx, base);
	// 82EF6930: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF6934: 419A001C  beq cr6, 0x82ef6950
	if ctx.cr[6].eq {
	pc = 0x82EF6950; continue 'dispatch;
	}
	// 82EF6938: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF693C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF6940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6944: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF694C: 4E800421  bctrl
	ctx.lr = 0x82EF6950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6950: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF6954: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF6958: 4BDB2AF8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6960 size=12
    let mut pc: u32 = 0x82EF6960;
    'dispatch: loop {
        match pc {
            0x82EF6960 => {
    //   block [0x82EF6960..0x82EF696C)
	// 82EF6960: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF6964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6968: 4BFFFA10  b 0x82ef6378
	sub_82EF6378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6970 size=8
    let mut pc: u32 = 0x82EF6970;
    'dispatch: loop {
        match pc {
            0x82EF6970 => {
    //   block [0x82EF6970..0x82EF6978)
	// 82EF6970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF6974: 4BFFFCB4  b 0x82ef6628
	sub_82EF6628(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6978 size=16
    let mut pc: u32 = 0x82EF6978;
    'dispatch: loop {
        match pc {
            0x82EF6978 => {
    //   block [0x82EF6978..0x82EF6988)
	// 82EF6978: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EF697C: 409A0008  bne cr6, 0x82ef6984
	if !ctx.cr[6].eq {
	pc = 0x82EF6984; continue 'dispatch;
	}
	// 82EF6980: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EF6984: 4BFFF9F4  b 0x82ef6378
	sub_82EF6378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6988 size=8
    let mut pc: u32 = 0x82EF6988;
    'dispatch: loop {
        match pc {
            0x82EF6988 => {
    //   block [0x82EF6988..0x82EF6990)
	// 82EF6988: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EF698C: 4BFFFC9C  b 0x82ef6628
	sub_82EF6628(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6990 size=100
    let mut pc: u32 = 0x82EF6990;
    'dispatch: loop {
        match pc {
            0x82EF6990 => {
    //   block [0x82EF6990..0x82EF69F4)
	// 82EF6990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF699C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF69A0: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF69A4: 817F8F88  lwz r11, -0x7078(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF69A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF69AC: 409A0034  bne cr6, 0x82ef69e0
	if !ctx.cr[6].eq {
	pc = 0x82EF69E0; continue 'dispatch;
	}
	// 82EF69B0: 48006EA1  bl 0x82efd850
	ctx.lr = 0x82EF69B4;
	sub_82EFD850(ctx, base);
	// 82EF69B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF69B8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EF69BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF69C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF69C4: 4E800421  bctrl
	ctx.lr = 0x82EF69C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF69CC: 907F8F88  stw r3, -0x7078(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28792 as u32), ctx.r[3].u32 ) };
	// 82EF69D0: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF69D4: 90630004  stw r3, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF69D8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF69DC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF69E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF69E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF69E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF69EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF69F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF69F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF69F8 size=144
    let mut pc: u32 = 0x82EF69F8;
    'dispatch: loop {
        match pc {
            0x82EF69F8 => {
    //   block [0x82EF69F8..0x82EF6A88)
	// 82EF69F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF69FC: 4BDB2A11  bl 0x82ca940c
	ctx.lr = 0x82EF6A00;
	sub_82CA93D0(ctx, base);
	// 82EF6A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6A04: 3FC08336  lis r30, -0x7cca
	ctx.r[30].s64 = -2093613056;
	// 82EF6A08: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6A0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A10: 419A0070  beq cr6, 0x82ef6a80
	if ctx.cr[6].eq {
	pc = 0x82EF6A80; continue 'dispatch;
	}
	// 82EF6A14: 48006E3D  bl 0x82efd850
	ctx.lr = 0x82EF6A18;
	sub_82EFD850(ctx, base);
	// 82EF6A18: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6A20: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6A24: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A28: 419A002C  beq cr6, 0x82ef6a54
	if ctx.cr[6].eq {
	pc = 0x82EF6A54; continue 'dispatch;
	}
	// 82EF6A2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6A34: 83A42000  lwz r29, 0x2000(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8192 as u32) ) } as u64;
	// 82EF6A38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6A40: 4E800421  bctrl
	ctx.lr = 0x82EF6A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6A44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6A48: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A4C: 409AFFE0  bne cr6, 0x82ef6a2c
	if !ctx.cr[6].eq {
	pc = 0x82EF6A2C; continue 'dispatch;
	}
	// 82EF6A50: 817E8F88  lwz r11, -0x7078(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6A54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF6A58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EF6A5C: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EF6A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6A64: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EF6A68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6A70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6A74: 4E800421  bctrl
	ctx.lr = 0x82EF6A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6A78: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EF6A7C: 917E8F88  stw r11, -0x7078(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-28792 as u32), ctx.r[11].u32 ) };
	// 82EF6A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6A84: 4BDB29D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6A88 size=12
    let mut pc: u32 = 0x82EF6A88;
    'dispatch: loop {
        match pc {
            0x82EF6A88 => {
    //   block [0x82EF6A88..0x82EF6A94)
	// 82EF6A88: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6A8C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EF6A90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6A94 size=8
    let mut pc: u32 = 0x82EF6A94;
    'dispatch: loop {
        match pc {
            0x82EF6A94 => {
    //   block [0x82EF6A94..0x82EF6A9C)
	// 82EF6A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6A98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6A9C size=60
    let mut pc: u32 = 0x82EF6A9C;
    'dispatch: loop {
        match pc {
            0x82EF6A9C => {
    //   block [0x82EF6A9C..0x82EF6AD8)
	// 82EF6A9C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF6AA4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6AA8: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6AAC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6AB0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6AB4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF6AB8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6ABC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF6AC0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6AC4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF6AC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6ACC: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EF6AD0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF6AD4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6AD8 size=8
    let mut pc: u32 = 0x82EF6AD8;
    'dispatch: loop {
        match pc {
            0x82EF6AD8 => {
    //   block [0x82EF6AD8..0x82EF6AE0)
	// 82EF6AD8: 4BFFFF20  b 0x82ef69f8
	sub_82EF69F8(ctx, base);
	return;
	// 82EF6ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6AE0 size=76
    let mut pc: u32 = 0x82EF6AE0;
    'dispatch: loop {
        match pc {
            0x82EF6AE0 => {
    //   block [0x82EF6AE0..0x82EF6B2C)
	// 82EF6AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6AEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6AF0: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF6AF4: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF6AF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6AFC: 409A000C  bne cr6, 0x82ef6b08
	if !ctx.cr[6].eq {
	pc = 0x82EF6B08; continue 'dispatch;
	}
	// 82EF6B00: 48006D41  bl 0x82efd840
	ctx.lr = 0x82EF6B04;
	sub_82EFD840(ctx, base);
	// 82EF6B04: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	// 82EF6B08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6B0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF6B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6B14: 4E800421  bctrl
	ctx.lr = 0x82EF6B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6B30 size=72
    let mut pc: u32 = 0x82EF6B30;
    'dispatch: loop {
        match pc {
            0x82EF6B30 => {
    //   block [0x82EF6B30..0x82EF6B78)
	// 82EF6B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6B48: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF6B4C: 396BCF64  addi r11, r11, -0x309c
	ctx.r[11].s64 = ctx.r[11].s64 + -12444;
	// 82EF6B50: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82EF6B54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6B58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6B5C: 4BDB2E55  bl 0x82ca99b0
	ctx.lr = 0x82EF6B60;
	sub_82CA99B0(ctx, base);
	// 82EF6B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6B78 size=68
    let mut pc: u32 = 0x82EF6B78;
    'dispatch: loop {
        match pc {
            0x82EF6B78 => {
    //   block [0x82EF6B78..0x82EF6BBC)
	// 82EF6B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6B8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6B90: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6B94: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 82EF6B98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6B9C: 41820008  beq 0x82ef6ba4
	if ctx.cr[0].eq {
	pc = 0x82EF6BA4; continue 'dispatch;
	}
	// 82EF6BA0: 4B94EC11  bl 0x828457b0
	ctx.lr = 0x82EF6BA4;
	sub_828457B0(ctx, base);
	// 82EF6BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6BC0 size=100
    let mut pc: u32 = 0x82EF6BC0;
    'dispatch: loop {
        match pc {
            0x82EF6BC0 => {
    //   block [0x82EF6BC0..0x82EF6C24)
	// 82EF6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6BC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6BD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6BD8: 387F0550  addi r3, r31, 0x550
	ctx.r[3].s64 = ctx.r[31].s64 + 1360;
	// 82EF6BDC: 396BCF80  addi r11, r11, -0x3080
	ctx.r[11].s64 = ctx.r[11].s64 + -12416;
	// 82EF6BE0: 38800190  li r4, 0x190
	ctx.r[4].s64 = 400;
	// 82EF6BE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6BE8: 48002159  bl 0x82ef8d40
	ctx.lr = 0x82EF6BEC;
	sub_82EF8D40(ctx, base);
	// 82EF6BEC: 387F0510  addi r3, r31, 0x510
	ctx.r[3].s64 = ctx.r[31].s64 + 1296;
	// 82EF6BF0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82EF6BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF6BF8: 4BDB2DB9  bl 0x82ca99b0
	ctx.lr = 0x82EF6BFC;
	sub_82CA99B0(ctx, base);
	// 82EF6BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C00: 4BFFF589  bl 0x82ef6188
	ctx.lr = 0x82EF6C04;
	sub_82EF6188(ctx, base);
	// 82EF6C04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6C08: 917F0504  stw r11, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[11].u32 ) };
	// 82EF6C0C: 917F0508  stw r11, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[11].u32 ) };
	// 82EF6C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6C28 size=8
    let mut pc: u32 = 0x82EF6C28;
    'dispatch: loop {
        match pc {
            0x82EF6C28 => {
    //   block [0x82EF6C28..0x82EF6C30)
	// 82EF6C28: 38630510  addi r3, r3, 0x510
	ctx.r[3].s64 = ctx.r[3].s64 + 1296;
	// 82EF6C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6C30 size=92
    let mut pc: u32 = 0x82EF6C30;
    'dispatch: loop {
        match pc {
            0x82EF6C30 => {
    //   block [0x82EF6C30..0x82EF6C8C)
	// 82EF6C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF6C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6C48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF6C4C: 387F0550  addi r3, r31, 0x550
	ctx.r[3].s64 = ctx.r[31].s64 + 1360;
	// 82EF6C50: 480043E9  bl 0x82efb038
	ctx.lr = 0x82EF6C54;
	sub_82EFB038(ctx, base);
	// 82EF6C54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF6C58: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF6C5C: 396BC2C0  addi r11, r11, -0x3d40
	ctx.r[11].s64 = ctx.r[11].s64 + -15680;
	// 82EF6C60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6C64: 4182000C  beq 0x82ef6c70
	if ctx.cr[0].eq {
	pc = 0x82EF6C70; continue 'dispatch;
	}
	// 82EF6C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C6C: 4B94EB45  bl 0x828457b0
	ctx.lr = 0x82EF6C70;
	sub_828457B0(ctx, base);
	// 82EF6C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF6C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF6C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6C90 size=108
    let mut pc: u32 = 0x82EF6C90;
    'dispatch: loop {
        match pc {
            0x82EF6C90 => {
    //   block [0x82EF6C90..0x82EF6CFC)
	// 82EF6C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6CA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6CA4: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF6CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6CAC: 483C2CB9  bl 0x832b9964
	ctx.lr = 0x82EF6CB0;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6CB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6CB4: 816B8F88  lwz r11, -0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6CB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6CBC: 419A0024  beq cr6, 0x82ef6ce0
	if ctx.cr[6].eq {
	pc = 0x82EF6CE0; continue 'dispatch;
	}
	// 82EF6CC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6CC4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6CC8: 419A0018  beq cr6, 0x82ef6ce0
	if ctx.cr[6].eq {
	pc = 0x82EF6CE0; continue 'dispatch;
	}
	// 82EF6CCC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6CD0: 48000008  b 0x82ef6cd8
	pc = 0x82EF6CD8; continue 'dispatch;
	// 82EF6CD4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6CD8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF6CDC: 409AFFF8  bne cr6, 0x82ef6cd4
	if !ctx.cr[6].eq {
	pc = 0x82EF6CD4; continue 'dispatch;
	}
	// 82EF6CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6CE4: 483C2C71  bl 0x832b9954
	ctx.lr = 0x82EF6CE8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6D00 size=152
    let mut pc: u32 = 0x82EF6D00;
    'dispatch: loop {
        match pc {
            0x82EF6D00 => {
    //   block [0x82EF6D00..0x82EF6D98)
	// 82EF6D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF6D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF6D0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6D10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF6D14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF6D1C: 409A005C  bne cr6, 0x82ef6d78
	if !ctx.cr[6].eq {
	pc = 0x82EF6D78; continue 'dispatch;
	}
	// 82EF6D20: 48006B31  bl 0x82efd850
	ctx.lr = 0x82EF6D24;
	sub_82EFD850(ctx, base);
	// 82EF6D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6D28: 38802004  li r4, 0x2004
	ctx.r[4].s64 = 8196;
	// 82EF6D2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6D30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6D34: 4E800421  bctrl
	ctx.lr = 0x82EF6D38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6D38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF6D3C: 41820048  beq 0x82ef6d84
	if ctx.cr[0].eq {
	pc = 0x82EF6D84; continue 'dispatch;
	}
	// 82EF6D40: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6D48: 91432000  stw r10, 0x2000(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8192 as u32), ctx.r[10].u32 ) };
	// 82EF6D4C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82EF6D50: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF6D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF6D58: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF6D5C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82EF6D60: 2B0B2000  cmplwi cr6, r11, 0x2000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8192 as u32, &mut ctx.xer);
	// 82EF6D64: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF6D68: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D6C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF6D70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF6D74: 4198FFDC  blt cr6, 0x82ef6d50
	if ctx.cr[6].lt {
	pc = 0x82EF6D50; continue 'dispatch;
	}
	// 82EF6D78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6D7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6D80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF6D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF6D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF6D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF6D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF6D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6D98 size=192
    let mut pc: u32 = 0x82EF6D98;
    'dispatch: loop {
        match pc {
            0x82EF6D98 => {
    //   block [0x82EF6D98..0x82EF6E58)
	// 82EF6D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6D9C: 4BDB2661  bl 0x82ca93fc
	ctx.lr = 0x82EF6DA0;
	sub_82CA93D0(ctx, base);
	// 82EF6DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6DA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF6DA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF6DAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF6DB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF6DB4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF6DB8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF6DBC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82EF6DC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF6DC4: 419A008C  beq cr6, 0x82ef6e50
	if ctx.cr[6].eq {
	pc = 0x82EF6E50; continue 'dispatch;
	}
	// 82EF6DC8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EF6DCC: 409A0010  bne cr6, 0x82ef6ddc
	if !ctx.cr[6].eq {
	pc = 0x82EF6DDC; continue 'dispatch;
	}
	// 82EF6DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6DD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF6DD8: 48000078  b 0x82ef6e50
	pc = 0x82EF6E50; continue 'dispatch;
	// 82EF6DDC: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82EF6DE0: 396000CC  li r11, 0xcc
	ctx.r[11].s64 = 204;
	// 82EF6DE4: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82EF6DE8: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82EF6DEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6DF0: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82EF6DF4: 997F000E  stb r11, 0xe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82EF6DF8: 997F000F  stb r11, 0xf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 82EF6DFC: 996A0004  stb r11, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82EF6E00: 996A0005  stb r11, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82EF6E04: 996A0006  stb r11, 6(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 82EF6E08: 996A0007  stb r11, 7(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(7 as u32), ctx.r[11].u8 ) };
	// 82EF6E0C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 82EF6E10: 4BFFFEF1  bl 0x82ef6d00
	ctx.lr = 0x82EF6E14;
	sub_82EF6D00(ctx, base);
	// 82EF6E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF6E18: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82EF6E1C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6E20: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EF6E24: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82EF6E28: 93430014  stw r26, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82EF6E2C: 93630018  stw r27, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 82EF6E30: 9323001C  stw r25, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[25].u32 ) };
	// 82EF6E34: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF6E38: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6E3C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF6E40: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EF6E44: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6E48: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF6E4C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF6E50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF6E54: 4BDB25F8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6E58 size=132
    let mut pc: u32 = 0x82EF6E58;
    'dispatch: loop {
        match pc {
            0x82EF6E58 => {
    //   block [0x82EF6E58..0x82EF6EDC)
	// 82EF6E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6E5C: 4BDB25AD  bl 0x82ca9408
	ctx.lr = 0x82EF6E60;
	sub_82CA93D0(ctx, base);
	// 82EF6E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6E64: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF6E68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6E6C: 419A0068  beq cr6, 0x82ef6ed4
	if ctx.cr[6].eq {
	pc = 0x82EF6ED4; continue 'dispatch;
	}
	// 82EF6E70: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6E74: 3BA4FFF0  addi r29, r4, -0x10
	ctx.r[29].s64 = ctx.r[4].s64 + -16;
	// 82EF6E78: 3BCB8F90  addi r30, r11, -0x7070
	ctx.r[30].s64 = ctx.r[11].s64 + -28784;
	// 82EF6E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6E80: 483C2AE5  bl 0x832b9964
	ctx.lr = 0x82EF6E84;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6E84: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6E88: 3BEB8F88  addi r31, r11, -0x7078
	ctx.r[31].s64 = ctx.r[11].s64 + -28792;
	// 82EF6E8C: 806B8F88  lwz r3, -0x7078(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6E94: 419A0010  beq cr6, 0x82ef6ea4
	if ctx.cr[6].eq {
	pc = 0x82EF6EA4; continue 'dispatch;
	}
	// 82EF6E98: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6E9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6EA0: 4BFFFBE9  bl 0x82ef6a88
	ctx.lr = 0x82EF6EA4;
	sub_82EF6A88(ctx, base);
	// 82EF6EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6EA8: 483C2AAD  bl 0x832b9954
	ctx.lr = 0x82EF6EAC;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6EAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6EB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6EB4: 409A000C  bne cr6, 0x82ef6ec0
	if !ctx.cr[6].eq {
	pc = 0x82EF6EC0; continue 'dispatch;
	}
	// 82EF6EB8: 48006989  bl 0x82efd840
	ctx.lr = 0x82EF6EBC;
	sub_82EFD840(ctx, base);
	// 82EF6EBC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF6EC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6EC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6EC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF6ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6ED0: 4E800421  bctrl
	ctx.lr = 0x82EF6ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6ED8: 4BDB2580  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6EE0 size=132
    let mut pc: u32 = 0x82EF6EE0;
    'dispatch: loop {
        match pc {
            0x82EF6EE0 => {
    //   block [0x82EF6EE0..0x82EF6F64)
	// 82EF6EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6EE4: 4BDB2525  bl 0x82ca9408
	ctx.lr = 0x82EF6EE8;
	sub_82CA93D0(ctx, base);
	// 82EF6EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6EEC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF6EF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF6EF4: 419A0068  beq cr6, 0x82ef6f5c
	if ctx.cr[6].eq {
	pc = 0x82EF6F5C; continue 'dispatch;
	}
	// 82EF6EF8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6EFC: 3BA4FFF0  addi r29, r4, -0x10
	ctx.r[29].s64 = ctx.r[4].s64 + -16;
	// 82EF6F00: 3BCB8F90  addi r30, r11, -0x7070
	ctx.r[30].s64 = ctx.r[11].s64 + -28784;
	// 82EF6F04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6F08: 483C2A5D  bl 0x832b9964
	ctx.lr = 0x82EF6F0C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6F0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6F10: 3BEB8F88  addi r31, r11, -0x7078
	ctx.r[31].s64 = ctx.r[11].s64 + -28792;
	// 82EF6F14: 806B8F88  lwz r3, -0x7078(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6F1C: 419A0010  beq cr6, 0x82ef6f2c
	if ctx.cr[6].eq {
	pc = 0x82EF6F2C; continue 'dispatch;
	}
	// 82EF6F20: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6F24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6F28: 4BFFFB61  bl 0x82ef6a88
	ctx.lr = 0x82EF6F2C;
	sub_82EF6A88(ctx, base);
	// 82EF6F2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF6F30: 483C2A25  bl 0x832b9954
	ctx.lr = 0x82EF6F34;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6F34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF6F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6F3C: 409A000C  bne cr6, 0x82ef6f48
	if !ctx.cr[6].eq {
	pc = 0x82EF6F48; continue 'dispatch;
	}
	// 82EF6F40: 48006901  bl 0x82efd840
	ctx.lr = 0x82EF6F44;
	sub_82EFD840(ctx, base);
	// 82EF6F44: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF6F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF6F4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF6F50: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF6F54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF6F58: 4E800421  bctrl
	ctx.lr = 0x82EF6F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF6F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF6F60: 4BDB24F8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6F68 size=4
    let mut pc: u32 = 0x82EF6F68;
    'dispatch: loop {
        match pc {
            0x82EF6F68 => {
    //   block [0x82EF6F68..0x82EF6F6C)
	// 82EF6F68: 4BFFFD28  b 0x82ef6c90
	sub_82EF6C90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF6F70 size=124
    let mut pc: u32 = 0x82EF6F70;
    'dispatch: loop {
        match pc {
            0x82EF6F70 => {
    //   block [0x82EF6F70..0x82EF6FEC)
	// 82EF6F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF6F74: 4BDB2485  bl 0x82ca93f8
	ctx.lr = 0x82EF6F78;
	sub_82CA93D0(ctx, base);
	// 82EF6F78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF6F7C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF6F80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF6F84: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF6F88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF6F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6F90: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF6F94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EF6F98: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EF6F9C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82EF6FA0: 483C29C5  bl 0x832b9964
	ctx.lr = 0x82EF6FA4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF6FA4: 3FA08336  lis r29, -0x7cca
	ctx.r[29].s64 = -2093613056;
	// 82EF6FA8: 807D8F88  lwz r3, -0x7078(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6FAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF6FB0: 409A000C  bne cr6, 0x82ef6fbc
	if !ctx.cr[6].eq {
	pc = 0x82EF6FBC; continue 'dispatch;
	}
	// 82EF6FB4: 4BFFF9DD  bl 0x82ef6990
	ctx.lr = 0x82EF6FB8;
	sub_82EF6990(ctx, base);
	// 82EF6FB8: 807D8F88  lwz r3, -0x7078(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 82EF6FBC: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82EF6FC0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82EF6FC4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82EF6FC8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82EF6FCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF6FD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF6FD4: 4BFFFDC5  bl 0x82ef6d98
	ctx.lr = 0x82EF6FD8;
	sub_82EF6D98(ctx, base);
	// 82EF6FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF6FDC: 483C2979  bl 0x832b9954
	ctx.lr = 0x82EF6FE0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF6FE0: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EF6FE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF6FE8: 4BDB2460  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF6FF0 size=40
    let mut pc: u32 = 0x82EF6FF0;
    'dispatch: loop {
        match pc {
            0x82EF6FF0 => {
    //   block [0x82EF6FF0..0x82EF7018)
	// 82EF6FF0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EF6FF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF6FF8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82EF6FFC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82EF7000: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82EF7004: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82EF7008: 409A0010  bne cr6, 0x82ef7018
	if !ctx.cr[6].eq {
		sub_82EF7018(ctx, base);
		return;
	}
	// 82EF700C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7010: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7014: 48000054  b 0x82ef7068
	sub_82EF702C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7018 size=20
    let mut pc: u32 = 0x82EF7018;
    'dispatch: loop {
        match pc {
            0x82EF7018 => {
    //   block [0x82EF7018..0x82EF702C)
	// 82EF7018: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF701C: 409A0010  bne cr6, 0x82ef702c
	if !ctx.cr[6].eq {
		sub_82EF702C(ctx, base);
		return;
	}
	// 82EF7020: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82EF7024: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF7028: 4BFFFF48  b 0x82ef6f70
	sub_82EF6F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF702C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF702C size=68
    let mut pc: u32 = 0x82EF702C;
    'dispatch: loop {
        match pc {
            0x82EF702C => {
    //   block [0x82EF702C..0x82EF7070)
	// 82EF702C: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82EF7030: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF7034: 394000CC  li r10, 0xcc
	ctx.r[10].s64 = 204;
	// 82EF7038: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82EF703C: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82EF7040: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EF7044: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82EF7048: 994B000D  stb r10, 0xd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82EF704C: 994B000E  stb r10, 0xe(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u8 ) };
	// 82EF7050: 994B000F  stb r10, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82EF7054: 99490004  stb r10, 4(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82EF7058: 99490005  stb r10, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82EF705C: 99490006  stb r10, 6(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 82EF7060: 39490004  addi r10, r9, 4
	ctx.r[10].s64 = ctx.r[9].s64 + 4;
	// 82EF7064: 99090007  stb r8, 7(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(7 as u32), ctx.r[8].u8 ) };
	// 82EF7068: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EF706C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7070 size=116
    let mut pc: u32 = 0x82EF7070;
    'dispatch: loop {
        match pc {
            0x82EF7070 => {
    //   block [0x82EF7070..0x82EF70E4)
	// 82EF7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7074: 4BDB238D  bl 0x82ca9400
	ctx.lr = 0x82EF7078;
	sub_82CA93D0(ctx, base);
	// 82EF7078: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF707C: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF7080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7084: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF7088: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF708C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF7090: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF7094: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF7098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF709C: 409A000C  bne cr6, 0x82ef70a8
	if !ctx.cr[6].eq {
	pc = 0x82EF70A8; continue 'dispatch;
	}
	// 82EF70A0: 480067A1  bl 0x82efd840
	ctx.lr = 0x82EF70A4;
	sub_82EFD840(ctx, base);
	// 82EF70A4: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	// 82EF70A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF70AC: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82EF70B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF70B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF70B8: 4E800421  bctrl
	ctx.lr = 0x82EF70BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF70BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF70C0: 4182001C  beq 0x82ef70dc
	if ctx.cr[0].eq {
	pc = 0x82EF70DC; continue 'dispatch;
	}
	// 82EF70C4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF70C8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF70CC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF70D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF70D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF70D8: 4BFFFE99  bl 0x82ef6f70
	ctx.lr = 0x82EF70DC;
	sub_82EF6F70(ctx, base);
	// 82EF70DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF70E0: 4BDB2370  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF70E8 size=284
    let mut pc: u32 = 0x82EF70E8;
    'dispatch: loop {
        match pc {
            0x82EF70E8 => {
    //   block [0x82EF70E8..0x82EF7204)
	// 82EF70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF70EC: 4BDB2311  bl 0x82ca93fc
	ctx.lr = 0x82EF70F0;
	sub_82CA93D0(ctx, base);
	// 82EF70F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF70F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF70F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF70FC: 3BAB8F88  addi r29, r11, -0x7078
	ctx.r[29].s64 = ctx.r[11].s64 + -28792;
	// 82EF7100: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EF7104: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82EF7108: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EF710C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EF7110: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7114: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82EF7118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF711C: 409A000C  bne cr6, 0x82ef7128
	if !ctx.cr[6].eq {
	pc = 0x82EF7128; continue 'dispatch;
	}
	// 82EF7120: 48006721  bl 0x82efd840
	ctx.lr = 0x82EF7124;
	sub_82EFD840(ctx, base);
	// 82EF7124: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EF7128: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF712C: 409A0034  bne cr6, 0x82ef7160
	if !ctx.cr[6].eq {
	pc = 0x82EF7160; continue 'dispatch;
	}
	// 82EF7130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7134: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82EF7138: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF713C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7140: 4E800421  bctrl
	ctx.lr = 0x82EF7144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7144: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82EF7148: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EF714C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EF7150: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EF7154: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7158: 4BFFFE19  bl 0x82ef6f70
	ctx.lr = 0x82EF715C;
	sub_82EF6F70(ctx, base);
	// 82EF715C: 480000A0  b 0x82ef71fc
	pc = 0x82EF71FC; continue 'dispatch;
	// 82EF7160: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7164: 409A0058  bne cr6, 0x82ef71bc
	if !ctx.cr[6].eq {
	pc = 0x82EF71BC; continue 'dispatch;
	}
	// 82EF7168: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF716C: 3BDEFFF0  addi r30, r30, -0x10
	ctx.r[30].s64 = ctx.r[30].s64 + -16;
	// 82EF7170: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 82EF7174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7178: 483C27ED  bl 0x832b9964
	ctx.lr = 0x82EF717C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF717C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7184: 419A0010  beq cr6, 0x82ef7194
	if ctx.cr[6].eq {
	pc = 0x82EF7194; continue 'dispatch;
	}
	// 82EF7188: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EF718C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7190: 4BFFF8F9  bl 0x82ef6a88
	ctx.lr = 0x82EF7194;
	sub_82EF6A88(ctx, base);
	// 82EF7194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7198: 483C27BD  bl 0x832b9954
	ctx.lr = 0x82EF719C;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF719C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF71A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF71A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF71A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF71AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF71B0: 4E800421  bctrl
	ctx.lr = 0x82EF71B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF71B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF71B8: 48000044  b 0x82ef71fc
	pc = 0x82EF71FC; continue 'dispatch;
	// 82EF71BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF71C0: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 82EF71C4: 83DEFFF0  lwz r30, -0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82EF71C8: 38BF0014  addi r5, r31, 0x14
	ctx.r[5].s64 = ctx.r[31].s64 + 20;
	// 82EF71CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF71D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF71D4: 4E800421  bctrl
	ctx.lr = 0x82EF71D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF71D8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EF71DC: 4182FFD8  beq 0x82ef71b4
	if ctx.cr[0].eq {
	pc = 0x82EF71B4; continue 'dispatch;
	}
	// 82EF71E0: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82EF71E4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EF71E8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EF71EC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82EF71F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF71F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF71F8: 4BFFFDF9  bl 0x82ef6ff0
	ctx.lr = 0x82EF71FC;
	sub_82EF6FF0(ctx, base);
	// 82EF71FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7200: 4BDB224C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7208 size=132
    let mut pc: u32 = 0x82EF7208;
    'dispatch: loop {
        match pc {
            0x82EF7208 => {
    //   block [0x82EF7208..0x82EF728C)
	// 82EF7208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF720C: 4BDB21ED  bl 0x82ca93f8
	ctx.lr = 0x82EF7210;
	sub_82CA93D0(ctx, base);
	// 82EF7210: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7214: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EF7218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF721C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EF7220: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EF7224: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EF7228: 807F8F8C  lwz r3, -0x7074(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28788 as u32) ) } as u64;
	// 82EF722C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82EF7230: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82EF7234: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 82EF7238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF723C: 409A000C  bne cr6, 0x82ef7248
	if !ctx.cr[6].eq {
	pc = 0x82EF7248; continue 'dispatch;
	}
	// 82EF7240: 48006601  bl 0x82efd840
	ctx.lr = 0x82EF7244;
	sub_82EFD840(ctx, base);
	// 82EF7244: 907F8F8C  stw r3, -0x7074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28788 as u32), ctx.r[3].u32 ) };
	// 82EF7248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF724C: 38DC0010  addi r6, r28, 0x10
	ctx.r[6].s64 = ctx.r[28].s64 + 16;
	// 82EF7250: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EF7254: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82EF7258: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF725C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7260: 4E800421  bctrl
	ctx.lr = 0x82EF7264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7264: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF7268: 4182001C  beq 0x82ef7284
	if ctx.cr[0].eq {
	pc = 0x82EF7284; continue 'dispatch;
	}
	// 82EF726C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82EF7270: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EF7274: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EF7278: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EF727C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7280: 4BFFFCF1  bl 0x82ef6f70
	ctx.lr = 0x82EF7284;
	sub_82EF6F70(ctx, base);
	// 82EF7284: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EF7288: 4BDB21C0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7290 size=64
    let mut pc: u32 = 0x82EF7290;
    'dispatch: loop {
        match pc {
            0x82EF7290 => {
    //   block [0x82EF7290..0x82EF72D0)
	// 82EF7290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF729C: 480065C5  bl 0x82efd860
	ctx.lr = 0x82EF72A0;
	sub_82EFD860(ctx, base);
	// 82EF72A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF72A4: 4182001C  beq 0x82ef72c0
	if ctx.cr[0].eq {
	pc = 0x82EF72C0; continue 'dispatch;
	}
	// 82EF72A8: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF72AC: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF72B0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF72B4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF72B8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF72BC: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF72C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF72C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF72C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF72CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF72D0 size=76
    let mut pc: u32 = 0x82EF72D0;
    'dispatch: loop {
        match pc {
            0x82EF72D0 => {
    //   block [0x82EF72D0..0x82EF731C)
	// 82EF72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF72D8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EF72DC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EF72E0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EF72E4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EF72E8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EF72EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF72F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF72F4: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82EF72F8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EF72FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF7300: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7304: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7308: 4BDB5431  bl 0x82cac738
	ctx.lr = 0x82EF730C;
	sub_82CAC738(ctx, base);
	// 82EF730C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF7310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7320 size=104
    let mut pc: u32 = 0x82EF7320;
    'dispatch: loop {
        match pc {
            0x82EF7320 => {
    //   block [0x82EF7320..0x82EF7388)
	// 82EF7320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF732C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF7338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF733C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7344: 419A0018  beq cr6, 0x82ef735c
	if ctx.cr[6].eq {
	pc = 0x82EF735C; continue 'dispatch;
	}
	// 82EF7348: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF734C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7350: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7358: 4E800421  bctrl
	ctx.lr = 0x82EF735C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF735C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7364: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7368: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF736C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF737C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7388 size=240
    let mut pc: u32 = 0x82EF7388;
    'dispatch: loop {
        match pc {
            0x82EF7388 => {
    //   block [0x82EF7388..0x82EF7478)
	// 82EF7388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7394: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF739C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF73A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF73A4: 409A0084  bne cr6, 0x82ef7428
	if !ctx.cr[6].eq {
	pc = 0x82EF7428; continue 'dispatch;
	}
	// 82EF73A8: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF73B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF73BC: 4E800421  bctrl
	ctx.lr = 0x82EF73C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF73C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF73C8: 419A0034  beq cr6, 0x82ef73fc
	if ctx.cr[6].eq {
	pc = 0x82EF73FC; continue 'dispatch;
	}
	// 82EF73CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF73D4: 419A0028  beq cr6, 0x82ef73fc
	if ctx.cr[6].eq {
	pc = 0x82EF73FC; continue 'dispatch;
	}
	// 82EF73D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF73DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF73E0: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF73E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EF73E8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF73EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF73F0: 4E800421  bctrl
	ctx.lr = 0x82EF73F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF73F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF73F8: 48000008  b 0x82ef7400
	pc = 0x82EF7400; continue 'dispatch;
	// 82EF73FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF7400: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF7404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7408: 419A0018  beq cr6, 0x82ef7420
	if ctx.cr[6].eq {
	pc = 0x82EF7420; continue 'dispatch;
	}
	// 82EF740C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7410: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF7414: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF741C: 4E800421  bctrl
	ctx.lr = 0x82EF7420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7424: 48000040  b 0x82ef7464
	pc = 0x82EF7464; continue 'dispatch;
	// 82EF7428: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF742C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF7430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7434: 419A002C  beq cr6, 0x82ef7460
	if ctx.cr[6].eq {
	pc = 0x82EF7460; continue 'dispatch;
	}
	// 82EF7438: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF743C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF7440: 419A0020  beq cr6, 0x82ef7460
	if ctx.cr[6].eq {
	pc = 0x82EF7460; continue 'dispatch;
	}
	// 82EF7444: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7448: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF744C: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF7450: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF7454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7458: 4E800421  bctrl
	ctx.lr = 0x82EF745C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF745C: 48000008  b 0x82ef7464
	pc = 0x82EF7464; continue 'dispatch;
	// 82EF7460: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF746C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7478 size=220
    let mut pc: u32 = 0x82EF7478;
    'dispatch: loop {
        match pc {
            0x82EF7478 => {
    //   block [0x82EF7478..0x82EF7554)
	// 82EF7478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF747C: 4BDB1F91  bl 0x82ca940c
	ctx.lr = 0x82EF7480;
	sub_82CA93D0(ctx, base);
	// 82EF7480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7484: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7488: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF748C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF7490: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF7494: 409A0028  bne cr6, 0x82ef74bc
	if !ctx.cr[6].eq {
	pc = 0x82EF74BC; continue 'dispatch;
	}
	// 82EF7498: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF749C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EF74A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF74A4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF74A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF74AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF74B0: 4E800421  bctrl
	ctx.lr = 0x82EF74B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF74B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF74B8: 4800000C  b 0x82ef74c4
	pc = 0x82EF74C4; continue 'dispatch;
	// 82EF74BC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF74C0: 388B001C  addi r4, r11, 0x1c
	ctx.r[4].s64 = ctx.r[11].s64 + 28;
	// 82EF74C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF74C8: 4BFFFE59  bl 0x82ef7320
	ctx.lr = 0x82EF74CC;
	sub_82EF7320(ctx, base);
	// 82EF74CC: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF74D0: 41820024  beq 0x82ef74f4
	if ctx.cr[0].eq {
	pc = 0x82EF74F4; continue 'dispatch;
	}
	// 82EF74D4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF74D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF74DC: 419A0018  beq cr6, 0x82ef74f4
	if ctx.cr[6].eq {
	pc = 0x82EF74F4; continue 'dispatch;
	}
	// 82EF74E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF74E4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF74E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF74EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF74F0: 4E800421  bctrl
	ctx.lr = 0x82EF74F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF74F4: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF74F8: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF74FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7500: 419A0024  beq cr6, 0x82ef7524
	if ctx.cr[6].eq {
	pc = 0x82EF7524; continue 'dispatch;
	}
	// 82EF7504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7508: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF750C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7510: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7518: 4E800421  bctrl
	ctx.lr = 0x82EF751C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF751C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7520: 48000008  b 0x82ef7528
	pc = 0x82EF7528; continue 'dispatch;
	// 82EF7524: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF7528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF752C: 419A001C  beq cr6, 0x82ef7548
	if ctx.cr[6].eq {
	pc = 0x82EF7548; continue 'dispatch;
	}
	// 82EF7530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7534: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF7538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF753C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7544: 4E800421  bctrl
	ctx.lr = 0x82EF7548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF754C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF7550: 4BDB1F0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7558 size=12
    let mut pc: u32 = 0x82EF7558;
    'dispatch: loop {
        match pc {
            0x82EF7558 => {
    //   block [0x82EF7558..0x82EF7564)
	// 82EF7558: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF755C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EF7560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7568 size=152
    let mut pc: u32 = 0x82EF7568;
    'dispatch: loop {
        match pc {
            0x82EF7568 => {
    //   block [0x82EF7568..0x82EF7600)
	// 82EF7568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF756C: 4BDB1E99  bl 0x82ca9404
	ctx.lr = 0x82EF7570;
	sub_82CA93D0(ctx, base);
	// 82EF7570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7574: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF7578: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF757C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EF7580: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7584: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EF7588: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF758C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7590: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7594: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7598: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF759C: 419A0058  beq cr6, 0x82ef75f4
	if ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	// 82EF75A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF75A4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF75A8: 409A004C  bne cr6, 0x82ef75f4
	if !ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	// 82EF75AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF75B0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF75B4: 409A0018  bne cr6, 0x82ef75cc
	if !ctx.cr[6].eq {
	pc = 0x82EF75CC; continue 'dispatch;
	}
	// 82EF75B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF75BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF75C0: 4BFFFDC9  bl 0x82ef7388
	ctx.lr = 0x82EF75C4;
	sub_82EF7388(ctx, base);
	// 82EF75C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF75C8: 40820024  bne 0x82ef75ec
	if !ctx.cr[0].eq {
	pc = 0x82EF75EC; continue 'dispatch;
	}
	// 82EF75CC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF75D0: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82EF75D4: 419A0020  beq cr6, 0x82ef75f4
	if ctx.cr[6].eq {
	pc = 0x82EF75F4; continue 'dispatch;
	}
	// 82EF75D8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF75DC: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF75E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF75E4: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF75E8: 4BFFFFC4  b 0x82ef75ac
	pc = 0x82EF75AC; continue 'dispatch;
	// 82EF75EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF75F0: 48000008  b 0x82ef75f8
	pc = 0x82EF75F8; continue 'dispatch;
	// 82EF75F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF75F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF75FC: 4BDB1E58  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7600 size=104
    let mut pc: u32 = 0x82EF7600;
    'dispatch: loop {
        match pc {
            0x82EF7600 => {
    //   block [0x82EF7600..0x82EF7668)
	// 82EF7600: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 82EF7604: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7608: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EF760C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7610: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF7614: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7618: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82EF761C: 419A0068  beq cr6, 0x82ef7684
	if ctx.cr[6].eq {
		sub_82EF7684(ctx, base);
		return;
	}
	// 82EF7620: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7624: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7628: 5547D1BE  srwi r7, r10, 6
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(6);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF762C: 7CEA5278  xor r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 ^ ctx.r[10].u64;
	// 82EF7630: 7D4A4038  and r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 82EF7634: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7638: 409A004C  bne cr6, 0x82ef7684
	if !ctx.cr[6].eq {
		sub_82EF7684(ctx, base);
		return;
	}
	// 82EF763C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7640: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82EF7644: 54E6D1BE  srwi r6, r7, 6
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82EF7648: 7CC73A78  xor r7, r6, r7
	ctx.r[7].u64 = ctx.r[6].u64 ^ ctx.r[7].u64;
	// 82EF764C: 7CE74038  and r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 82EF7650: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7654: 409A0014  bne cr6, 0x82ef7668
	if !ctx.cr[6].eq {
		sub_82EF7668(ctx, base);
		return;
	}
	// 82EF7658: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF765C: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7660: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF7664: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7668 size=28
    let mut pc: u32 = 0x82EF7668;
    'dispatch: loop {
        match pc {
            0x82EF7668 => {
    //   block [0x82EF7668..0x82EF7684)
	// 82EF7668: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF766C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EF7670: 419A0014  beq cr6, 0x82ef7684
	if ctx.cr[6].eq {
		sub_82EF7684(ctx, base);
		return;
	}
	// 82EF7674: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF7678: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF767C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EF7680: 4BFFFFBC  b 0x82ef763c
	sub_82EF7600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7684 size=8
    let mut pc: u32 = 0x82EF7684;
    'dispatch: loop {
        match pc {
            0x82EF7684 => {
    //   block [0x82EF7684..0x82EF768C)
	// 82EF7684: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF7688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7690 size=192
    let mut pc: u32 = 0x82EF7690;
    'dispatch: loop {
        match pc {
            0x82EF7690 => {
    //   block [0x82EF7690..0x82EF7750)
	// 82EF7690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7694: 4BDB1D71  bl 0x82ca9404
	ctx.lr = 0x82EF7698;
	sub_82CA93D0(ctx, base);
	// 82EF7698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF769C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF76A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF76A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF76A8: 480062A1  bl 0x82efd948
	ctx.lr = 0x82EF76AC;
	sub_82EFD948(ctx, base);
	// 82EF76AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF76B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF76B4: 396BCF9C  addi r11, r11, -0x3064
	ctx.r[11].s64 = ctx.r[11].s64 + -12388;
	// 82EF76B8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82EF76BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF76C0: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EF76C4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF76C8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EF76CC: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EF76D0: 48006899  bl 0x82efdf68
	ctx.lr = 0x82EF76D4;
	sub_82EFDF68(ctx, base);
	// 82EF76D4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF76D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF76DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF76E0: 480024A1  bl 0x82ef9b80
	ctx.lr = 0x82EF76E4;
	sub_82EF9B80(ctx, base);
	// 82EF76E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF76E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF76EC: 4800636D  bl 0x82efda58
	ctx.lr = 0x82EF76F0;
	sub_82EFDA58(ctx, base);
	// 82EF76F0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF76F4: 419A0018  beq cr6, 0x82ef770c
	if ctx.cr[6].eq {
	pc = 0x82EF770C; continue 'dispatch;
	}
	// 82EF76F8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF76FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF7700: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7708: 4E800421  bctrl
	ctx.lr = 0x82EF770C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF770C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7714: 419A0018  beq cr6, 0x82ef772c
	if ctx.cr[6].eq {
	pc = 0x82EF772C; continue 'dispatch;
	}
	// 82EF7718: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF771C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF7720: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7728: 4E800421  bctrl
	ctx.lr = 0x82EF772C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF772C: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82EF7730: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF7734: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EF7738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF773C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF7740: 4804DCE1  bl 0x82f45420
	ctx.lr = 0x82EF7744;
	sub_82F45420(ctx, base);
	// 82EF7744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF774C: 4BDB1D08  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7750 size=24
    let mut pc: u32 = 0x82EF7750;
    'dispatch: loop {
        match pc {
            0x82EF7750 => {
    //   block [0x82EF7750..0x82EF7768)
	// 82EF7750: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7754: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7758: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF775C: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7760: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7764: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7768 size=24
    let mut pc: u32 = 0x82EF7768;
    'dispatch: loop {
        match pc {
            0x82EF7768 => {
    //   block [0x82EF7768..0x82EF7780)
	// 82EF7768: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF776C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7770: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7774: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7778: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF777C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7780 size=28
    let mut pc: u32 = 0x82EF7780;
    'dispatch: loop {
        match pc {
            0x82EF7780 => {
    //   block [0x82EF7780..0x82EF779C)
	// 82EF7780: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7784: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7788: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82EF778C: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7790: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82EF7794: 2F09FFFE  cmpwi cr6, r9, -2
	ctx.cr[6].compare_i32(ctx.r[9].s32, -2, &mut ctx.xer);
	// 82EF7798: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF779C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF779C size=36
    let mut pc: u32 = 0x82EF779C;
    'dispatch: loop {
        match pc {
            0x82EF779C => {
    //   block [0x82EF779C..0x82EF77C0)
	// 82EF779C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF77A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF77A4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77B4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF77B8: 4099FFC8  ble cr6, 0x82ef7780
	if !ctx.cr[6].gt {
		sub_82EF7780(ctx, base);
		return;
	}
	// 82EF77BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF77C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF77C0 size=332
    let mut pc: u32 = 0x82EF77C0;
    'dispatch: loop {
        match pc {
            0x82EF77C0 => {
    //   block [0x82EF77C0..0x82EF790C)
	// 82EF77C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF77C4: 4BDB1C3D  bl 0x82ca9400
	ctx.lr = 0x82EF77C8;
	sub_82CA93D0(ctx, base);
	// 82EF77C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF77CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF77D0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EF77D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF77DC: 419A0128  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF77E0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF77E8: 419A001C  beq cr6, 0x82ef7804
	if ctx.cr[6].eq {
	pc = 0x82EF7804; continue 'dispatch;
	}
	// 82EF77EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF77F0: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF77F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF77F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF77FC: 4E800421  bctrl
	ctx.lr = 0x82EF7800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7800: 48000008  b 0x82ef7808
	pc = 0x82EF7808; continue 'dispatch;
	// 82EF7804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7808: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF780C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7810: 7D5E1838  and r30, r10, r3
	ctx.r[30].u64 = ctx.r[10].u64 & ctx.r[3].u64;
	// 82EF7814: 57CA2036  slwi r10, r30, 4
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7818: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF781C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7820: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7824: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF7828: 419A00DC  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF782C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7830: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF7834: 409A00D0  bne cr6, 0x82ef7904
	if !ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF7838: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EF783C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82EF7840: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7844: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82EF7848: 409A0018  bne cr6, 0x82ef7860
	if !ctx.cr[6].eq {
	pc = 0x82EF7860; continue 'dispatch;
	}
	// 82EF784C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EF7850: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82EF7854: 4BFFFB35  bl 0x82ef7388
	ctx.lr = 0x82EF7858;
	sub_82EF7388(ctx, base);
	// 82EF7858: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF785C: 40820028  bne 0x82ef7884
	if !ctx.cr[0].eq {
	pc = 0x82EF7884; continue 'dispatch;
	}
	// 82EF7860: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82EF7864: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7868: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF786C: 419A0098  beq cr6, 0x82ef7904
	if ctx.cr[6].eq {
	pc = 0x82EF7904; continue 'dispatch;
	}
	// 82EF7870: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7874: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7878: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF787C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF7880: 4BFFFFC0  b 0x82ef7840
	pc = 0x82EF7840; continue 'dispatch;
	// 82EF7884: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 82EF7888: 7F1BF000  cmpw cr6, r27, r30
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EF788C: 409A0050  bne cr6, 0x82ef78dc
	if !ctx.cr[6].eq {
	pc = 0x82EF78DC; continue 'dispatch;
	}
	// 82EF7890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7894: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82EF7898: 419A0058  beq cr6, 0x82ef78f0
	if ctx.cr[6].eq {
	pc = 0x82EF78F0; continue 'dispatch;
	}
	// 82EF789C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78A0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF78A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF78A8: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 82EF78AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF78B0: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 82EF78B4: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF78B8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF78BC: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF78C0: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF78C4: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 82EF78C8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF78CC: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF78D0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF78D4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF78D8: 48000018  b 0x82ef78f0
	pc = 0x82EF78F0; continue 'dispatch;
	// 82EF78DC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78E0: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF78E4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF78EC: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EF78F0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF78F4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78F8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF78FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF7900: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7904: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF7908: 4BDB1B48  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7910 size=140
    let mut pc: u32 = 0x82EF7910;
    'dispatch: loop {
        match pc {
            0x82EF7910 => {
    //   block [0x82EF7910..0x82EF799C)
	// 82EF7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF791C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF7928: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF792C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7934: 409A000C  bne cr6, 0x82ef7940
	if !ctx.cr[6].eq {
	pc = 0x82EF7940; continue 'dispatch;
	}
	// 82EF7938: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF793C: 48000048  b 0x82ef7984
	pc = 0x82EF7984; continue 'dispatch;
	// 82EF7940: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7944: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF7948: 419A0020  beq cr6, 0x82ef7968
	if ctx.cr[6].eq {
	pc = 0x82EF7968; continue 'dispatch;
	}
	// 82EF794C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7950: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7954: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF7958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF795C: 4E800421  bctrl
	ctx.lr = 0x82EF7960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF7964: 48000008  b 0x82ef796c
	pc = 0x82EF796C; continue 'dispatch;
	// 82EF7968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF796C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7970: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF7974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7978: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF797C: 7D455838  and r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82EF7980: 4BFFFBE9  bl 0x82ef7568
	ctx.lr = 0x82EF7984;
	sub_82EF7568(ctx, base);
	// 82EF7984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF798C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF79A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF79A0 size=392
    let mut pc: u32 = 0x82EF79A0;
    'dispatch: loop {
        match pc {
            0x82EF79A0 => {
    //   block [0x82EF79A0..0x82EF7B28)
	// 82EF79A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF79A4: 4BDB1A69  bl 0x82ca940c
	ctx.lr = 0x82EF79A8;
	sub_82CA93D0(ctx, base);
	// 82EF79A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF79AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF79B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF79B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF79B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF79BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF79C0: 409A000C  bne cr6, 0x82ef79cc
	if !ctx.cr[6].eq {
	pc = 0x82EF79CC; continue 'dispatch;
	}
	// 82EF79C4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EF79C8: 4800002C  b 0x82ef79f4
	pc = 0x82EF79F4; continue 'dispatch;
	// 82EF79CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF79D0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF79D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF79D8: 1D290005  mulli r9, r9, 5
	ctx.r[9].s64 = ctx.r[9].s64 * 5;
	// 82EF79DC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF79E0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF79E4: 40990018  ble cr6, 0x82ef79fc
	if !ctx.cr[6].gt {
	pc = 0x82EF79FC; continue 'dispatch;
	}
	// 82EF79E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF79EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF79F0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF79F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF79F8: 48000769  bl 0x82ef8160
	ctx.lr = 0x82EF79FC;
	sub_82EF8160(ctx, base);
	// 82EF79FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A00: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A04: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A08: 7D27F038  and r7, r9, r30
	ctx.r[7].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 82EF7A0C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EF7A10: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7A14: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7A18: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7A20: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF7A24: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A28: 2F05FFFE  cmpwi cr6, r5, -2
	ctx.cr[6].compare_i32(ctx.r[5].s32, -2, &mut ctx.xer);
	// 82EF7A2C: 409A001C  bne cr6, 0x82ef7a48
	if !ctx.cr[6].eq {
	pc = 0x82EF7A48; continue 'dispatch;
	}
	// 82EF7A30: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EF7A34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7A38: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7A3C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7A40: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A44: 48000074  b 0x82ef7ab8
	pc = 0x82EF7AB8; continue 'dispatch;
	// 82EF7A48: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A4C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82EF7A50: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 82EF7A54: 7D064838  and r6, r8, r9
	ctx.r[6].u64 = ctx.r[8].u64 & ctx.r[9].u64;
	// 82EF7A58: 54C82036  slwi r8, r6, 4
	ctx.r[8].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7A5C: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EF7A60: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7A64: 2F08FFFE  cmpwi cr6, r8, -2
	ctx.cr[6].compare_i32(ctx.r[8].s32, -2, &mut ctx.xer);
	// 82EF7A68: 409AFFE8  bne cr6, 0x82ef7a50
	if !ctx.cr[6].eq {
	pc = 0x82EF7A50; continue 'dispatch;
	}
	// 82EF7A6C: 54C92036  slwi r9, r6, 4
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7A70: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF7A74: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A78: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82EF7A7C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF7A80: 409A0044  bne cr6, 0x82ef7ac4
	if !ctx.cr[6].eq {
	pc = 0x82EF7AC4; continue 'dispatch;
	}
	// 82EF7A84: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7A88: 419A0020  beq cr6, 0x82ef7aa8
	if ctx.cr[6].eq {
	pc = 0x82EF7AA8; continue 'dispatch;
	}
	// 82EF7A8C: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EF7A90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7A94: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF7A98: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7A9C: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AA0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF7AA4: 9148000C  stw r10, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF7AA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7AAC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AB0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7AB4: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7AB8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF7ABC: 48000060  b 0x82ef7b1c
	pc = 0x82EF7B1C; continue 'dispatch;
	// 82EF7AC0: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7AC4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7AC8: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82EF7ACC: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EF7AD0: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7AD4: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EF7AD8: 409AFFE8  bne cr6, 0x82ef7ac0
	if !ctx.cr[6].eq {
	pc = 0x82EF7AC0; continue 'dispatch;
	}
	// 82EF7ADC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7AE0: 419A0020  beq cr6, 0x82ef7b00
	if ctx.cr[6].eq {
	pc = 0x82EF7B00; continue 'dispatch;
	}
	// 82EF7AE4: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EF7AE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7AEC: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EF7AF0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7AF4: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF7AF8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF7AFC: 9148000C  stw r10, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF7B00: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7B04: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EF7B08: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B0C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EF7B10: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B14: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF7B18: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82EF7B1C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EF7B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7B24: 4BDB1938  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7B28 size=372
    let mut pc: u32 = 0x82EF7B28;
    'dispatch: loop {
        match pc {
            0x82EF7B28 => {
    //   block [0x82EF7B28..0x82EF7C9C)
	// 82EF7B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7B2C: 4BDB18E1  bl 0x82ca940c
	ctx.lr = 0x82EF7B30;
	sub_82CA93D0(ctx, base);
	// 82EF7B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7B38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF7B3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF7B40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7B48: 409A000C  bne cr6, 0x82ef7b54
	if !ctx.cr[6].eq {
	pc = 0x82EF7B54; continue 'dispatch;
	}
	// 82EF7B4C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EF7B50: 4800002C  b 0x82ef7b7c
	pc = 0x82EF7B7C; continue 'dispatch;
	// 82EF7B54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B58: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B5C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF7B60: 1D290005  mulli r9, r9, 5
	ctx.r[9].s64 = ctx.r[9].s64 * 5;
	// 82EF7B64: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7B68: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF7B6C: 40990018  ble cr6, 0x82ef7b84
	if !ctx.cr[6].gt {
	pc = 0x82EF7B84; continue 'dispatch;
	}
	// 82EF7B70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF7B78: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF7B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7B80: 480004A1  bl 0x82ef8020
	ctx.lr = 0x82EF7B84;
	sub_82EF8020(ctx, base);
	// 82EF7B84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B88: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7B8C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7B90: 7D29F038  and r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[30].u64;
	// 82EF7B94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF7B98: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82EF7B9C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7BA0: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7BA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7BA8: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF7BAC: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF7BB0: 2F04FFFE  cmpwi cr6, r4, -2
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2, &mut ctx.xer);
	// 82EF7BB4: 409A0018  bne cr6, 0x82ef7bcc
	if !ctx.cr[6].eq {
	pc = 0x82EF7BCC; continue 'dispatch;
	}
	// 82EF7BB8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF7BBC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7BC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7BC4: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7BC8: 480000CC  b 0x82ef7c94
	pc = 0x82EF7C94; continue 'dispatch;
	// 82EF7BCC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7BD0: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 82EF7BD4: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 82EF7BD8: 7D464038  and r6, r10, r8
	ctx.r[6].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 82EF7BDC: 39460001  addi r10, r6, 1
	ctx.r[10].s64 = ctx.r[6].s64 + 1;
	// 82EF7BE0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7BE4: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF7BE8: 2F0AFFFE  cmpwi cr6, r10, -2
	ctx.cr[6].compare_i32(ctx.r[10].s32, -2, &mut ctx.xer);
	// 82EF7BEC: 409AFFE8  bne cr6, 0x82ef7bd4
	if !ctx.cr[6].eq {
	pc = 0x82EF7BD4; continue 'dispatch;
	}
	// 82EF7BF0: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7BF4: 38E60001  addi r7, r6, 1
	ctx.r[7].s64 = ctx.r[6].s64 + 1;
	// 82EF7BF8: 39450004  addi r10, r5, 4
	ctx.r[10].s64 = ctx.r[5].s64 + 4;
	// 82EF7BFC: 547FD1BE  srwi r31, r3, 6
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EF7C00: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EF7C04: 7FE31A78  xor r3, r31, r3
	ctx.r[3].u64 = ctx.r[31].u64 ^ ctx.r[3].u64;
	// 82EF7C08: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EF7C0C: 7C634038  and r3, r3, r8
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82EF7C10: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF7C14: 409A0028  bne cr6, 0x82ef7c3c
	if !ctx.cr[6].eq {
	pc = 0x82EF7C3C; continue 'dispatch;
	}
	// 82EF7C18: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF7C1C: 419A0010  beq cr6, 0x82ef7c2c
	if ctx.cr[6].eq {
	pc = 0x82EF7C2C; continue 'dispatch;
	}
	// 82EF7C20: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF7C24: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C28: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7C2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C30: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7C34: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7C38: 4800005C  b 0x82ef7c94
	pc = 0x82EF7C94; continue 'dispatch;
	// 82EF7C3C: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C40: 547FD1BE  srwi r31, r3, 6
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EF7C44: 7FE31A78  xor r3, r31, r3
	ctx.r[3].u64 = ctx.r[31].u64 ^ ctx.r[3].u64;
	// 82EF7C48: 7C684038  and r8, r3, r8
	ctx.r[8].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82EF7C4C: 48000008  b 0x82ef7c54
	pc = 0x82EF7C54; continue 'dispatch;
	// 82EF7C50: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C54: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82EF7C58: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EF7C5C: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82EF7C60: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C64: 7F034800  cmpw cr6, r3, r9
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF7C68: 409AFFE8  bne cr6, 0x82ef7c50
	if !ctx.cr[6].eq {
	pc = 0x82EF7C50; continue 'dispatch;
	}
	// 82EF7C6C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EF7C70: 419A0010  beq cr6, 0x82ef7c80
	if ctx.cr[6].eq {
	pc = 0x82EF7C80; continue 'dispatch;
	}
	// 82EF7C74: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82EF7C78: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C7C: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF7C80: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF7C84: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EF7C88: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7C8C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7C90: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7C94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7C98: 4BDB17C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7CA0 size=280
    let mut pc: u32 = 0x82EF7CA0;
    'dispatch: loop {
        match pc {
            0x82EF7CA0 => {
    //   block [0x82EF7CA0..0x82EF7DB8)
	// 82EF7CA0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82EF7CA4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82EF7CA8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CAC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82EF7CB0: 419A00FC  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CB4: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CB8: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7CBC: 54CBD1BE  srwi r11, r6, 6
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7CC0: 7D6B3278  xor r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[6].u64;
	// 82EF7CC4: 7D6A3838  and r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 82EF7CC8: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82EF7CCC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7CD0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF7CD4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7CD8: 2F09FFFE  cmpwi cr6, r9, -2
	ctx.cr[6].compare_i32(ctx.r[9].s32, -2, &mut ctx.xer);
	// 82EF7CDC: 419A00D0  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CE0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7CE4: 5525D1BE  srwi r5, r9, 6
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shr(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF7CE8: 7CA94A78  xor r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 ^ ctx.r[9].u64;
	// 82EF7CEC: 7D293838  and r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[7].u64;
	// 82EF7CF0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF7CF4: 409A00B8  bne cr6, 0x82ef7dac
	if !ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7CF8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82EF7CFC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EF7D00: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7D04: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82EF7D08: 57FED1BE  srwi r30, r31, 6
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shr(6);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EF7D0C: 7FDFFA78  xor r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 ^ ctx.r[31].u64;
	// 82EF7D10: 7FFF3838  and r31, r31, r7
	ctx.r[31].u64 = ctx.r[31].u64 & ctx.r[7].u64;
	// 82EF7D14: 7F1F2840  cmplw cr6, r31, r5
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82EF7D18: 409A0010  bne cr6, 0x82ef7d28
	if !ctx.cr[6].eq {
	pc = 0x82EF7D28; continue 'dispatch;
	}
	// 82EF7D1C: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D20: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82EF7D24: 419A0024  beq cr6, 0x82ef7d48
	if ctx.cr[6].eq {
	pc = 0x82EF7D48; continue 'dispatch;
	}
	// 82EF7D28: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82EF7D2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D30: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EF7D34: 419A0078  beq cr6, 0x82ef7dac
	if ctx.cr[6].eq {
	pc = 0x82EF7DAC; continue 'dispatch;
	}
	// 82EF7D38: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82EF7D3C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7D40: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF7D44: 4BFFFFBC  b 0x82ef7d00
	pc = 0x82EF7D00; continue 'dispatch;
	// 82EF7D48: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 82EF7D4C: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF7D50: 409A0038  bne cr6, 0x82ef7d88
	if !ctx.cr[6].eq {
	pc = 0x82EF7D88; continue 'dispatch;
	}
	// 82EF7D54: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D58: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82EF7D5C: 419A003C  beq cr6, 0x82ef7d98
	if ctx.cr[6].eq {
	pc = 0x82EF7D98; continue 'dispatch;
	}
	// 82EF7D60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF7D64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7D68: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7D6C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82EF7D70: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D74: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EF7D78: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7D7C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EF7D80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF7D84: 48000014  b 0x82ef7d98
	pc = 0x82EF7D98; continue 'dispatch;
	// 82EF7D88: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 82EF7D8C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7D90: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF7D94: 7CEA412E  stwx r7, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u32) };
	// 82EF7D98: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF7D9C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7DA0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7DA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF7DA8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7DAC: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7DB0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82EF7DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7DB8 size=40
    let mut pc: u32 = 0x82EF7DB8;
    'dispatch: loop {
        match pc {
            0x82EF7DB8 => {
    //   block [0x82EF7DB8..0x82EF7DE0)
	// 82EF7DB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF7DBC: 38E30004  addi r7, r3, 4
	ctx.r[7].s64 = ctx.r[3].s64 + 4;
	// 82EF7DC0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF7DC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DC8: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF7DCC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF7DD0: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7DD4: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DD8: 4082FFE8  bne 0x82ef7dc0
	if !ctx.cr[0].eq {
	pc = 0x82EF7DC0; continue 'dispatch;
	}
	// 82EF7DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7DE0 size=12
    let mut pc: u32 = 0x82EF7DE0;
    'dispatch: loop {
        match pc {
            0x82EF7DE0 => {
    //   block [0x82EF7DE0..0x82EF7DEC)
	// 82EF7DE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7DE4: 39430004  addi r10, r3, 4
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	// 82EF7DE8: 48000044  b 0x82ef7e2c
	sub_82EF7E14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7DEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7DEC size=40
    let mut pc: u32 = 0x82EF7DEC;
    'dispatch: loop {
        match pc {
            0x82EF7DEC => {
    //   block [0x82EF7DEC..0x82EF7E14)
	// 82EF7DEC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82EF7DF0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7DF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7DF8: 7D005028  lwarx r8, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 82EF7DFC: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF7E00: 409A0014  bne cr6, 0x82ef7e14
	if !ctx.cr[6].eq {
		sub_82EF7E14(ctx, base);
		return;
	}
	// 82EF7E04: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E08: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E0C: 4082FFE4  bne 0x82ef7df0
	if !ctx.cr[0].eq {
	pc = 0x82EF7DF0; continue 'dispatch;
	}
	// 82EF7E10: 4800000C  b 0x82ef7e1c
	sub_82EF7E14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7E14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7E14 size=40
    let mut pc: u32 = 0x82EF7E14;
    'dispatch: loop {
        match pc {
            0x82EF7E14 => {
    //   block [0x82EF7E14..0x82EF7E3C)
	// 82EF7E14: 7D00512D  stwcx. r8, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E18: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E1C: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82EF7E20: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF7E24: 419A0018  beq cr6, 0x82ef7e3c
	if ctx.cr[6].eq {
		sub_82EF7E3C(ctx, base);
		return;
	}
	// 82EF7E28: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7E2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF7E30: 409AFFBC  bne cr6, 0x82ef7dec
	if !ctx.cr[6].eq {
		sub_82EF7DEC(ctx, base);
		return;
	}
	// 82EF7E34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF7E3C size=8
    let mut pc: u32 = 0x82EF7E3C;
    'dispatch: loop {
        match pc {
            0x82EF7E3C => {
    //   block [0x82EF7E3C..0x82EF7E44)
	// 82EF7E3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7E48 size=108
    let mut pc: u32 = 0x82EF7E48;
    'dispatch: loop {
        match pc {
            0x82EF7E48 => {
    //   block [0x82EF7E48..0x82EF7EB4)
	// 82EF7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7E5C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EF7E60: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF7E64: 480010DD  bl 0x82ef8f40
	ctx.lr = 0x82EF7E68;
	sub_82EF8F40(ctx, base);
	// 82EF7E68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF7E70: 419A002C  beq cr6, 0x82ef7e9c
	if ctx.cr[6].eq {
	pc = 0x82EF7E9C; continue 'dispatch;
	}
	// 82EF7E74: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7E78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF7E7C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF7E80: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7E84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E88: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EF7E8C: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF7E90: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7E94: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7E98: 4082FFE8  bne 0x82ef7e80
	if !ctx.cr[0].eq {
	pc = 0x82EF7E80; continue 'dispatch;
	}
	// 82EF7E9C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF7EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF7EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7EAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7EB8 size=168
    let mut pc: u32 = 0x82EF7EB8;
    'dispatch: loop {
        match pc {
            0x82EF7EB8 => {
    //   block [0x82EF7EB8..0x82EF7F60)
	// 82EF7EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7EBC: 4BDB1551  bl 0x82ca940c
	ctx.lr = 0x82EF7EC0;
	sub_82CA93D0(ctx, base);
	// 82EF7EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7EC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7EC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7ECC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EF7ED0: 409A0034  bne cr6, 0x82ef7f04
	if !ctx.cr[6].eq {
	pc = 0x82EF7F04; continue 'dispatch;
	}
	// 82EF7ED4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7ED8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EF7EDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EF7EE0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF7EE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7EE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF7EEC: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EF7EF0: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF7EF4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF7EF8: 4082FFE8  bne 0x82ef7ee0
	if !ctx.cr[0].eq {
	pc = 0x82EF7EE0; continue 'dispatch;
	}
	// 82EF7EFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F00: 48000058  b 0x82ef7f58
	pc = 0x82EF7F58; continue 'dispatch;
	// 82EF7F04: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EF7F08: 419A004C  beq cr6, 0x82ef7f54
	if ctx.cr[6].eq {
	pc = 0x82EF7F54; continue 'dispatch;
	}
	// 82EF7F0C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7F14: 4BFFFF35  bl 0x82ef7e48
	ctx.lr = 0x82EF7F18;
	sub_82EF7E48(ctx, base);
	// 82EF7F18: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF7F1C: 41820030  beq 0x82ef7f4c
	if ctx.cr[0].eq {
	pc = 0x82EF7F4C; continue 'dispatch;
	}
	// 82EF7F20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF7F24: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF7F28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF7F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF7F34: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF7F38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF7F40: 4E800421  bctrl
	ctx.lr = 0x82EF7F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF7F44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF7F48: 48000010  b 0x82ef7f58
	pc = 0x82EF7F58; continue 'dispatch;
	// 82EF7F4C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82EF7F50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF7F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF7F58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7F5C: 4BDB1500  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7F60 size=88
    let mut pc: u32 = 0x82EF7F60;
    'dispatch: loop {
        match pc {
            0x82EF7F60 => {
    //   block [0x82EF7F60..0x82EF7FB8)
	// 82EF7F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF7F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7F74: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EF7F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF7F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF7F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF7F84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF7F88: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82EF7F8C: 4BFFF4ED  bl 0x82ef7478
	ctx.lr = 0x82EF7F90;
	sub_82EF7478(ctx, base);
	// 82EF7F90: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82EF7F94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF7F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF7F9C: 4BFFFA05  bl 0x82ef79a0
	ctx.lr = 0x82EF7FA0;
	sub_82EF79A0(ctx, base);
	// 82EF7FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF7FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF7FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF7FAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF7FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF7FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF7FB8 size=100
    let mut pc: u32 = 0x82EF7FB8;
    'dispatch: loop {
        match pc {
            0x82EF7FB8 => {
    //   block [0x82EF7FB8..0x82EF801C)
	// 82EF7FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF7FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF7FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF7FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF7FC8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7FCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF7FD0: 419A0034  beq cr6, 0x82ef8004
	if ctx.cr[6].eq {
	pc = 0x82EF8004; continue 'dispatch;
	}
	// 82EF7FD4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF7FD8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF7FDC: 5569D1BE  srwi r9, r11, 6
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EF7FE0: 7D2B5A78  xor r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 ^ ctx.r[11].u64;
	// 82EF7FE4: 7D655038  and r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82EF7FE8: 4BFFF619  bl 0x82ef7600
	ctx.lr = 0x82EF7FEC;
	sub_82EF7600(ctx, base);
	// 82EF7FEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF7FF0: 41800014  blt 0x82ef8004
	if ctx.cr[0].lt {
	pc = 0x82EF8004; continue 'dispatch;
	}
	// 82EF7FF4: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF7FF8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EF7FFC: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EF8000: 48000008  b 0x82ef8008
	pc = 0x82EF8008; continue 'dispatch;
	// 82EF8004: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF8008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF800C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF8020 size=320
    let mut pc: u32 = 0x82EF8020;
    'dispatch: loop {
        match pc {
            0x82EF8020 => {
    //   block [0x82EF8020..0x82EF8160)
	// 82EF8020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8024: 4BDB13DD  bl 0x82ca9400
	ctx.lr = 0x82EF8028;
	sub_82CA93D0(ctx, base);
	// 82EF8028: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF802C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF8030: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF8034: 409A0008  bne cr6, 0x82ef803c
	if !ctx.cr[6].eq {
	pc = 0x82EF803C; continue 'dispatch;
	}
	// 82EF8038: 4800011C  b 0x82ef8154
	pc = 0x82EF8154; continue 'dispatch;
	// 82EF803C: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EF8040: 4098000C  bge cr6, 0x82ef804c
	if !ctx.cr[6].lt {
	pc = 0x82EF804C; continue 'dispatch;
	}
	// 82EF8044: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EF8048: 4800004C  b 0x82ef8094
	pc = 0x82EF8094; continue 'dispatch;
	// 82EF804C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EF8050: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF8054: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EF8058: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF805C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EF8060: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EF8064: 4B2FBD4D  bl 0x821f3db0
	ctx.lr = 0x82EF8068;
	sub_821F3DB0(ctx, base);
	// 82EF8068: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EF806C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF8070: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF8074: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF8078: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF807C: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF8080: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EF8084: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF8088: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EF808C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8090: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF8094: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF8098: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82EF809C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF80A0: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF80A4: 480057BD  bl 0x82efd860
	ctx.lr = 0x82EF80A8;
	sub_82EFD860(ctx, base);
	// 82EF80A8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF80AC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EF80B0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EF80B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF80B8: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EF80BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF80C0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF80C4: 419A0020  beq cr6, 0x82ef80e4
	if ctx.cr[6].eq {
	pc = 0x82EF80E4; continue 'dispatch;
	}
	// 82EF80C8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82EF80CC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF80D0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EF80D4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF80D8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF80DC: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EF80E0: 4082FFEC  bne 0x82ef80cc
	if !ctx.cr[0].eq {
	pc = 0x82EF80CC; continue 'dispatch;
	}
	// 82EF80E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF80E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF80EC: 419A0058  beq cr6, 0x82ef8144
	if ctx.cr[6].eq {
	pc = 0x82EF8144; continue 'dispatch;
	}
	// 82EF80F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF80F4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EF80F8: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82EF80FC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8100: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EF8104: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF8108: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF810C: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF8110: 419A0020  beq cr6, 0x82ef8130
	if ctx.cr[6].eq {
	pc = 0x82EF8130; continue 'dispatch;
	}
	// 82EF8114: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8118: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82EF811C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8120: 556AD1BE  srwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8124: 7D455A78  xor r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82EF8128: 4BFFFA01  bl 0x82ef7b28
	ctx.lr = 0x82EF812C;
	sub_82EF7B28(ctx, base);
	// 82EF812C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EF8130: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF8134: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82EF8138: 4082FFC4  bne 0x82ef80fc
	if !ctx.cr[0].eq {
	pc = 0x82EF80FC; continue 'dispatch;
	}
	// 82EF813C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8140: 48005761  bl 0x82efd8a0
	ctx.lr = 0x82EF8144;
	sub_82EFD8A0(ctx, base);
	// 82EF8144: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF814C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF8150: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8154: 480754A5  bl 0x82f6d5f8
	ctx.lr = 0x82EF8158;
	sub_82F6D5F8(ctx, base);
	// 82EF8158: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF815C: 4BDB12F4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EF8160 size=308
    let mut pc: u32 = 0x82EF8160;
    'dispatch: loop {
        match pc {
            0x82EF8160 => {
    //   block [0x82EF8160..0x82EF8294)
	// 82EF8160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8164: 4BDB129D  bl 0x82ca9400
	ctx.lr = 0x82EF8168;
	sub_82CA93D0(ctx, base);
	// 82EF8168: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF816C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF8170: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF8174: 409A0008  bne cr6, 0x82ef817c
	if !ctx.cr[6].eq {
	pc = 0x82EF817C; continue 'dispatch;
	}
	// 82EF8178: 48000110  b 0x82ef8288
	pc = 0x82EF8288; continue 'dispatch;
	// 82EF817C: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EF8180: 4098000C  bge cr6, 0x82ef818c
	if !ctx.cr[6].lt {
	pc = 0x82EF818C; continue 'dispatch;
	}
	// 82EF8184: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EF8188: 4800004C  b 0x82ef81d4
	pc = 0x82EF81D4; continue 'dispatch;
	// 82EF818C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EF8190: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EF8194: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EF8198: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EF819C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EF81A0: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EF81A4: 4B2FBC0D  bl 0x821f3db0
	ctx.lr = 0x82EF81A8;
	sub_821F3DB0(ctx, base);
	// 82EF81A8: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EF81AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EF81B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EF81B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF81B8: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EF81BC: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EF81C0: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EF81C4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EF81C8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EF81CC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF81D0: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82EF81D4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF81D8: 57EB2036  slwi r11, r31, 4
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF81DC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF81E0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EF81E4: 4800567D  bl 0x82efd860
	ctx.lr = 0x82EF81E8;
	sub_82EFD860(ctx, base);
	// 82EF81E8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EF81EC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EF81F0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EF81F4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF81F8: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EF81FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8200: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8204: 419A0020  beq cr6, 0x82ef8224
	if ctx.cr[6].eq {
	pc = 0x82EF8224; continue 'dispatch;
	}
	// 82EF8208: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82EF820C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8210: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EF8214: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8218: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EF821C: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EF8220: 4082FFEC  bne 0x82ef820c
	if !ctx.cr[0].eq {
	pc = 0x82EF820C; continue 'dispatch;
	}
	// 82EF8224: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF822C: 419A004C  beq cr6, 0x82ef8278
	if ctx.cr[6].eq {
	pc = 0x82EF8278; continue 'dispatch;
	}
	// 82EF8230: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8234: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EF8238: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82EF823C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8240: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EF8244: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EF8248: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF824C: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EF8250: 419A0014  beq cr6, 0x82ef8264
	if ctx.cr[6].eq {
	pc = 0x82EF8264; continue 'dispatch;
	}
	// 82EF8254: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EF8258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF825C: 4BFFFD05  bl 0x82ef7f60
	ctx.lr = 0x82EF8260;
	sub_82EF7F60(ctx, base);
	// 82EF8260: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EF8264: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EF8268: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82EF826C: 4082FFD0  bne 0x82ef823c
	if !ctx.cr[0].eq {
	pc = 0x82EF823C; continue 'dispatch;
	}
	// 82EF8270: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8274: 4800562D  bl 0x82efd8a0
	ctx.lr = 0x82EF8278;
	sub_82EFD8A0(ctx, base);
	// 82EF8278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF827C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8280: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF8284: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8288: 480237C1  bl 0x82f1ba48
	ctx.lr = 0x82EF828C;
	sub_82F1BA48(ctx, base);
	// 82EF828C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8290: 4BDB11C0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8298 size=156
    let mut pc: u32 = 0x82EF8298;
    'dispatch: loop {
        match pc {
            0x82EF8298 => {
    //   block [0x82EF8298..0x82EF8334)
	// 82EF8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF82A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF82A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF82A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF82AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF82B0: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82EF82B4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF82B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF82BC: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF82C0: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF82C4: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF82C8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF82CC: 4082FFE8  bne 0x82ef82b4
	if !ctx.cr[0].eq {
	pc = 0x82EF82B4; continue 'dispatch;
	}
	// 82EF82D0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF82D4: 7C2004AC  lwsync
	// 82EF82D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF82DC: 40820044  bne 0x82ef8320
	if !ctx.cr[0].eq {
	pc = 0x82EF8320; continue 'dispatch;
	}
	// 82EF82E0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF82E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF82E8: 419A0020  beq cr6, 0x82ef8308
	if ctx.cr[6].eq {
	pc = 0x82EF8308; continue 'dispatch;
	}
	// 82EF82EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF82F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF82F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF82F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF82FC: 4E800421  bctrl
	ctx.lr = 0x82EF8300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF8304: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF8308: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF830C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8314: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF831C: 4E800421  bctrl
	ctx.lr = 0x82EF8320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF832C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8338 size=248
    let mut pc: u32 = 0x82EF8338;
    'dispatch: loop {
        match pc {
            0x82EF8338 => {
    //   block [0x82EF8338..0x82EF8430)
	// 82EF8338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF833C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF834C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8350: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF8354: 394ACF9C  addi r10, r10, -0x3064
	ctx.r[10].s64 = ctx.r[10].s64 + -12388;
	// 82EF8358: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF835C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8360: 3BCB0014  addi r30, r11, 0x14
	ctx.r[30].s64 = ctx.r[11].s64 + 20;
	// 82EF8364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8368: 483C15FD  bl 0x832b9964
	ctx.lr = 0x82EF836C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF836C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8374: 409A0014  bne cr6, 0x82ef8388
	if !ctx.cr[6].eq {
	pc = 0x82EF8388; continue 'dispatch;
	}
	// 82EF8378: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF837C: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8380: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82EF8384: 4BFFF43D  bl 0x82ef77c0
	ctx.lr = 0x82EF8388;
	sub_82EF77C0(ctx, base);
	// 82EF8388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF838C: 483C15C9  bl 0x832b9954
	ctx.lr = 0x82EF8390;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8390: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF8394: 48001875  bl 0x82ef9c08
	ctx.lr = 0x82EF8398;
	sub_82EF9C08(ctx, base);
	// 82EF8398: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EF839C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EF83A0: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EF83A4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF83A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF83AC: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EF83B0: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF83B4: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF83B8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF83BC: 4082FFE8  bne 0x82ef83a4
	if !ctx.cr[0].eq {
	pc = 0x82EF83A4; continue 'dispatch;
	}
	// 82EF83C0: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EF83C4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF83C8: 40820008  bne 0x82ef83d0
	if !ctx.cr[0].eq {
	pc = 0x82EF83D0; continue 'dispatch;
	}
	// 82EF83CC: 480054D5  bl 0x82efd8a0
	ctx.lr = 0x82EF83D0;
	sub_82EFD8A0(ctx, base);
	// 82EF83D0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF83D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF83D8: 419A0018  beq cr6, 0x82ef83f0
	if ctx.cr[6].eq {
	pc = 0x82EF83F0; continue 'dispatch;
	}
	// 82EF83DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF83E0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EF83E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF83E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF83EC: 4E800421  bctrl
	ctx.lr = 0x82EF83F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF83F0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF83F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF83F8: 419A0018  beq cr6, 0x82ef8410
	if ctx.cr[6].eq {
	pc = 0x82EF8410; continue 'dispatch;
	}
	// 82EF83FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8400: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8404: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF840C: 4E800421  bctrl
	ctx.lr = 0x82EF8410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8414: 48005605  bl 0x82efda18
	ctx.lr = 0x82EF8418;
	sub_82EFDA18(ctx, base);
	// 82EF8418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF841C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8430 size=144
    let mut pc: u32 = 0x82EF8430;
    'dispatch: loop {
        match pc {
            0x82EF8430 => {
    //   block [0x82EF8430..0x82EF84C0)
	// 82EF8430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8434: 4BDB0FD5  bl 0x82ca9408
	ctx.lr = 0x82EF8438;
	sub_82CA93D0(ctx, base);
	// 82EF8438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF843C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8440: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF8444: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8448: 3B8B0014  addi r28, r11, 0x14
	ctx.r[28].s64 = ctx.r[11].s64 + 20;
	// 82EF844C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8450: 483C1515  bl 0x832b9964
	ctx.lr = 0x82EF8454;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8454: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF8458: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82EF845C: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8460: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF8464: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8468: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 82EF846C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8470: 4BFFF4A1  bl 0x82ef7910
	ctx.lr = 0x82EF8474;
	sub_82EF7910(ctx, base);
	// 82EF8474: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8478: 41800018  blt 0x82ef8490
	if ctx.cr[0].lt {
	pc = 0x82EF8490; continue 'dispatch;
	}
	// 82EF847C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF8480: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8484: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8488: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF848C: 48000008  b 0x82ef8494
	pc = 0x82EF8494; continue 'dispatch;
	// 82EF8490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF8494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF8498: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EF849C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF84A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF84A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84A8: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF84AC: 480023AD  bl 0x82efa858
	ctx.lr = 0x82EF84B0;
	sub_82EFA858(ctx, base);
	// 82EF84B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF84B4: 483C14A1  bl 0x832b9954
	ctx.lr = 0x82EF84B8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF84B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF84BC: 4BDB0F9C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF84C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF84C0 size=96
    let mut pc: u32 = 0x82EF84C0;
    'dispatch: loop {
        match pc {
            0x82EF84C0 => {
    //   block [0x82EF84C0..0x82EF8520)
	// 82EF84C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF84C4: 4BDB0F49  bl 0x82ca940c
	ctx.lr = 0x82EF84C8;
	sub_82CA93D0(ctx, base);
	// 82EF84C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF84CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF84D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF84D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84D8: 3BCB0014  addi r30, r11, 0x14
	ctx.r[30].s64 = ctx.r[11].s64 + 20;
	// 82EF84DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF84E0: 483C1485  bl 0x832b9964
	ctx.lr = 0x82EF84E4;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF84E4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82EF84E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF84EC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF84F0: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82EF84F4: 480060A5  bl 0x82efe598
	ctx.lr = 0x82EF84F8;
	sub_82EFE598(ctx, base);
	// 82EF84F8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF84FC: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82EF8500: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82EF8504: 4BFFF2BD  bl 0x82ef77c0
	ctx.lr = 0x82EF8508;
	sub_82EF77C0(ctx, base);
	// 82EF8508: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EF850C: 4800234D  bl 0x82efa858
	ctx.lr = 0x82EF8510;
	sub_82EFA858(ctx, base);
	// 82EF8510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8514: 483C1441  bl 0x832b9954
	ctx.lr = 0x82EF8518;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF851C: 4BDB0F40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8520 size=108
    let mut pc: u32 = 0x82EF8520;
    'dispatch: loop {
        match pc {
            0x82EF8520 => {
    //   block [0x82EF8520..0x82EF858C)
	// 82EF8520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF852C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8538: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF853C: 4800540D  bl 0x82efd948
	ctx.lr = 0x82EF8540;
	sub_82EFD948(ctx, base);
	// 82EF8540: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8544: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82EF8548: 396BCFA0  addi r11, r11, -0x3060
	ctx.r[11].s64 = ctx.r[11].s64 + -12384;
	// 82EF854C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8550: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8554: 480007ED  bl 0x82ef8d40
	ctx.lr = 0x82EF8558;
	sub_82EF8D40(ctx, base);
	// 82EF8558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF855C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8560: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EF8564: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8568: 480054F1  bl 0x82efda58
	ctx.lr = 0x82EF856C;
	sub_82EFDA58(ctx, base);
	// 82EF856C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EF8570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF857C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8590 size=260
    let mut pc: u32 = 0x82EF8590;
    'dispatch: loop {
        match pc {
            0x82EF8590 => {
    //   block [0x82EF8590..0x82EF8694)
	// 82EF8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8594: 4BDB0E75  bl 0x82ca9408
	ctx.lr = 0x82EF8598;
	sub_82CA93D0(ctx, base);
	// 82EF8598: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF859C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF85A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF85A4: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82EF85A8: 396BCFA0  addi r11, r11, -0x3060
	ctx.r[11].s64 = ctx.r[11].s64 + -12384;
	// 82EF85AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF85B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF85B4: 483C13B1  bl 0x832b9964
	ctx.lr = 0x82EF85B8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF85B8: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EF85BC: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82EF85C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF85C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF85C8: 409A0010  bne cr6, 0x82ef85d8
	if !ctx.cr[6].eq {
	pc = 0x82EF85D8; continue 'dispatch;
	}
	// 82EF85CC: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82EF85D0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82EF85D4: 48000034  b 0x82ef8608
	pc = 0x82EF8608; continue 'dispatch;
	// 82EF85D8: 392A0008  addi r9, r10, 8
	ctx.r[9].s64 = ctx.r[10].s64 + 8;
	// 82EF85DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EF85E0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF85E4: 2F08FFFE  cmpwi cr6, r8, -2
	ctx.cr[6].compare_i32(ctx.r[8].s32, -2, &mut ctx.xer);
	// 82EF85E8: 409A0018  bne cr6, 0x82ef8600
	if !ctx.cr[6].eq {
	pc = 0x82EF8600; continue 'dispatch;
	}
	// 82EF85EC: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF85F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF85F4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82EF85F8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EF85FC: 4099FFE4  ble cr6, 0x82ef85e0
	if !ctx.cr[6].gt {
	pc = 0x82EF85E0; continue 'dispatch;
	}
	// 82EF8600: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82EF8604: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EF8608: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EF860C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF8610: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF8614: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8618: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82EF861C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82EF8620: 48042399  bl 0x82f3a9b8
	ctx.lr = 0x82EF8624;
	sub_82F3A9B8(ctx, base);
	// 82EF8624: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8628: 40820044  bne 0x82ef866c
	if !ctx.cr[0].eq {
	pc = 0x82EF866C; continue 'dispatch;
	}
	// 82EF862C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EF8630: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF8634: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8638: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF863C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF8640: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8644: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF864C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF8650: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EF8654: 4803A365  bl 0x82f329b8
	ctx.lr = 0x82EF8658;
	sub_82F329B8(ctx, base);
	// 82EF8658: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF865C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8660: 48042359  bl 0x82f3a9b8
	ctx.lr = 0x82EF8664;
	sub_82F3A9B8(ctx, base);
	// 82EF8664: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8668: 4182FFCC  beq 0x82ef8634
	if ctx.cr[0].eq {
	pc = 0x82EF8634; continue 'dispatch;
	}
	// 82EF866C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8670: 483C12E5  bl 0x832b9954
	ctx.lr = 0x82EF8674;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8674: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8678: 480233D1  bl 0x82f1ba48
	ctx.lr = 0x82EF867C;
	sub_82F1BA48(ctx, base);
	// 82EF867C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8680: 480029B9  bl 0x82efb038
	ctx.lr = 0x82EF8684;
	sub_82EFB038(ctx, base);
	// 82EF8684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8688: 48005391  bl 0x82efda18
	ctx.lr = 0x82EF868C;
	sub_82EFDA18(ctx, base);
	// 82EF868C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8690: 4BDB0DC8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8698 size=148
    let mut pc: u32 = 0x82EF8698;
    'dispatch: loop {
        match pc {
            0x82EF8698 => {
    //   block [0x82EF8698..0x82EF872C)
	// 82EF8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF869C: 4BDB0D71  bl 0x82ca940c
	ctx.lr = 0x82EF86A0;
	sub_82CA93D0(ctx, base);
	// 82EF86A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF86A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF86A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF86AC: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF86B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF86B4: 483C12B1  bl 0x832b9964
	ctx.lr = 0x82EF86B8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF86B8: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF86BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF86C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF86C4: 4BFFF24D  bl 0x82ef7910
	ctx.lr = 0x82EF86C8;
	sub_82EF7910(ctx, base);
	// 82EF86C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF86CC: 41800018  blt 0x82ef86e4
	if ctx.cr[0].lt {
	pc = 0x82EF86E4; continue 'dispatch;
	}
	// 82EF86D0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF86D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86D8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF86DC: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF86E0: 48000008  b 0x82ef86e8
	pc = 0x82EF86E8; continue 'dispatch;
	// 82EF86E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF86E8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EF86EC: 419A0038  beq cr6, 0x82ef8724
	if ctx.cr[6].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF86F0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF86F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF86F8: 409A002C  bne cr6, 0x82ef8724
	if !ctx.cr[6].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF86FC: 80660004  lwz r3, 4(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8700: 4BFFF6E1  bl 0x82ef7de0
	ctx.lr = 0x82EF8704;
	sub_82EF7DE0(ctx, base);
	// 82EF8704: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8708: 4182001C  beq 0x82ef8724
	if ctx.cr[0].eq {
	pc = 0x82EF8724; continue 'dispatch;
	}
	// 82EF870C: 83E60004  lwz r31, 4(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8714: 483C1241  bl 0x832b9954
	ctx.lr = 0x82EF8718;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF871C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8720: 4BDB0D3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82EF8724: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8728: 4BFFFFE8  b 0x82ef8710
	pc = 0x82EF8710; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8730 size=328
    let mut pc: u32 = 0x82EF8730;
    'dispatch: loop {
        match pc {
            0x82EF8730 => {
    //   block [0x82EF8730..0x82EF8878)
	// 82EF8730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8734: 4BDB0CD1  bl 0x82ca9404
	ctx.lr = 0x82EF8738;
	sub_82CA93D0(ctx, base);
	// 82EF8738: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF873C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8740: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EF8744: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 82EF8748: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EF874C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF8750: 483C1215  bl 0x832b9964
	ctx.lr = 0x82EF8754;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8754: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 82EF8758: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF875C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8760: 4BFFF1B1  bl 0x82ef7910
	ctx.lr = 0x82EF8764;
	sub_82EF7910(ctx, base);
	// 82EF8764: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8768: 41800018  blt 0x82ef8780
	if ctx.cr[0].lt {
	pc = 0x82EF8780; continue 'dispatch;
	}
	// 82EF876C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82EF8770: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8774: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8778: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF877C: 48000008  b 0x82ef8784
	pc = 0x82EF8784; continue 'dispatch;
	// 82EF8780: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EF8784: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82EF8788: 419A0050  beq cr6, 0x82ef87d8
	if ctx.cr[6].eq {
	pc = 0x82EF87D8; continue 'dispatch;
	}
	// 82EF878C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8794: 409A0080  bne cr6, 0x82ef8814
	if !ctx.cr[6].eq {
	pc = 0x82EF8814; continue 'dispatch;
	}
	// 82EF8798: 80660004  lwz r3, 4(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF879C: 4BFFF645  bl 0x82ef7de0
	ctx.lr = 0x82EF87A0;
	sub_82EF7DE0(ctx, base);
	// 82EF87A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF87A4: 41820028  beq 0x82ef87cc
	if ctx.cr[0].eq {
	pc = 0x82EF87CC; continue 'dispatch;
	}
	// 82EF87A8: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF87AC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82EF87B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF87B4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EF87B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF87BC: 483C1199  bl 0x832b9954
	ctx.lr = 0x82EF87C0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF87C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF87C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF87C8: 4BDB0C8C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82EF87CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF87D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF87D4: 4BFFEFED  bl 0x82ef77c0
	ctx.lr = 0x82EF87D8;
	sub_82EF77C0(ctx, base);
	// 82EF87D8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82EF87DC: 48005085  bl 0x82efd860
	ctx.lr = 0x82EF87E0;
	sub_82EFD860(ctx, base);
	// 82EF87E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF87E4: 4182005C  beq 0x82ef8840
	if ctx.cr[0].eq {
	pc = 0x82EF8840; continue 'dispatch;
	}
	// 82EF87E8: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF87EC: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF87F0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF87F4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF87F8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF87FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EF8800: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF8804: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8808: 4BFFEE89  bl 0x82ef7690
	ctx.lr = 0x82EF880C;
	sub_82EF7690(ctx, base);
	// 82EF880C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8810: 48000034  b 0x82ef8844
	pc = 0x82EF8844; continue 'dispatch;
	// 82EF8814: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8818: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82EF881C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8820: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF8824: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8828: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF882C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8834: 4E800421  bctrl
	ctx.lr = 0x82EF8838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8838: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF883C: 4BFFFF7C  b 0x82ef87b8
	pc = 0x82EF87B8; continue 'dispatch;
	// 82EF8840: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF8844: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8848: 409A000C  bne cr6, 0x82ef8854
	if !ctx.cr[6].eq {
	pc = 0x82EF8854; continue 'dispatch;
	}
	// 82EF884C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82EF8850: 4BFFFF68  b 0x82ef87b8
	pc = 0x82EF87B8; continue 'dispatch;
	// 82EF8854: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF8858: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82EF885C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF8860: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF8864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8868: 4BFFF6F9  bl 0x82ef7f60
	ctx.lr = 0x82EF886C;
	sub_82EF7F60(ctx, base);
	// 82EF886C: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82EF8870: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EF8874: 4BFFFF40  b 0x82ef87b4
	pc = 0x82EF87B4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8878 size=144
    let mut pc: u32 = 0x82EF8878;
    'dispatch: loop {
        match pc {
            0x82EF8878 => {
    //   block [0x82EF8878..0x82EF8908)
	// 82EF8878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF887C: 4BDB0B91  bl 0x82ca940c
	ctx.lr = 0x82EF8880;
	sub_82CA93D0(ctx, base);
	// 82EF8880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF888C: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82EF8890: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82EF8894: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8898: 483C10CD  bl 0x832b9964
	ctx.lr = 0x82EF889C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF889C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF88A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF88A4: 419A0054  beq cr6, 0x82ef88f8
	if ctx.cr[6].eq {
	pc = 0x82EF88F8; continue 'dispatch;
	}
	// 82EF88A8: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 82EF88AC: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF88B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF88B4: 4BFFF705  bl 0x82ef7fb8
	ctx.lr = 0x82EF88B8;
	sub_82EF7FB8(ctx, base);
	// 82EF88B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF88BC: 4082003C  bne 0x82ef88f8
	if !ctx.cr[0].eq {
	pc = 0x82EF88F8; continue 'dispatch;
	}
	// 82EF88C0: 57CBD1BE  srwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF88C4: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF88C8: 7D65F278  xor r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[30].u64;
	// 82EF88CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF88D0: 4BFFF259  bl 0x82ef7b28
	ctx.lr = 0x82EF88D4;
	sub_82EF7B28(ctx, base);
	// 82EF88D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF88D8: 391E0004  addi r8, r30, 4
	ctx.r[8].s64 = ctx.r[30].s64 + 4;
	// 82EF88DC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EF88E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF88E4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF88E8: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF88EC: 7D20412D  stwcx. r9, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF88F0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF88F4: 4082FFE8  bne 0x82ef88dc
	if !ctx.cr[0].eq {
	pc = 0x82EF88DC; continue 'dispatch;
	}
	// 82EF88F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF88FC: 483C1059  bl 0x832b9954
	ctx.lr = 0x82EF8900;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8900: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8904: 4BDB0B58  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8908 size=84
    let mut pc: u32 = 0x82EF8908;
    'dispatch: loop {
        match pc {
            0x82EF8908 => {
    //   block [0x82EF8908..0x82EF895C)
	// 82EF8908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF890C: 4BDB0B01  bl 0x82ca940c
	ctx.lr = 0x82EF8910;
	sub_82CA93D0(ctx, base);
	// 82EF8910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8918: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EF891C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF8920: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82EF8924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8928: 483C103D  bl 0x832b9964
	ctx.lr = 0x82EF892C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF892C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8934: 419A0018  beq cr6, 0x82ef894c
	if ctx.cr[6].eq {
	pc = 0x82EF894C; continue 'dispatch;
	}
	// 82EF8938: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EF893C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EF8940: 4BFFF361  bl 0x82ef7ca0
	ctx.lr = 0x82EF8944;
	sub_82EF7CA0(ctx, base);
	// 82EF8944: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8948: 4BFFF951  bl 0x82ef8298
	ctx.lr = 0x82EF894C;
	sub_82EF8298(ctx, base);
	// 82EF894C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8950: 483C1005  bl 0x832b9954
	ctx.lr = 0x82EF8954;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8958: 4BDB0B04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8960 size=240
    let mut pc: u32 = 0x82EF8960;
    'dispatch: loop {
        match pc {
            0x82EF8960 => {
    //   block [0x82EF8960..0x82EF8A50)
	// 82EF8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8964: 4BDB0AA9  bl 0x82ca940c
	ctx.lr = 0x82EF8968;
	sub_82CA93D0(ctx, base);
	// 82EF8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF896C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8970: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 82EF8974: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8978: 483C0FED  bl 0x832b9964
	ctx.lr = 0x82EF897C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF897C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF8984: 419A00BC  beq cr6, 0x82ef8a40
	if ctx.cr[6].eq {
	pc = 0x82EF8A40; continue 'dispatch;
	}
	// 82EF8988: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EF898C: 390B0014  addi r8, r11, 0x14
	ctx.r[8].s64 = ctx.r[11].s64 + 20;
	// 82EF8990: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8994: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF8998: 409A0010  bne cr6, 0x82ef89a8
	if !ctx.cr[6].eq {
	pc = 0x82EF89A8; continue 'dispatch;
	}
	// 82EF899C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82EF89A0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82EF89A4: 48000034  b 0x82ef89d8
	pc = 0x82EF89D8; continue 'dispatch;
	// 82EF89A8: 392A0008  addi r9, r10, 8
	ctx.r[9].s64 = ctx.r[10].s64 + 8;
	// 82EF89AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EF89B0: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF89B4: 2F07FFFE  cmpwi cr6, r7, -2
	ctx.cr[6].compare_i32(ctx.r[7].s32, -2, &mut ctx.xer);
	// 82EF89B8: 409A0018  bne cr6, 0x82ef89d0
	if !ctx.cr[6].eq {
	pc = 0x82EF89D0; continue 'dispatch;
	}
	// 82EF89BC: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF89C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF89C4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EF89C8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82EF89CC: 4099FFE4  ble cr6, 0x82ef89b0
	if !ctx.cr[6].gt {
	pc = 0x82EF89B0; continue 'dispatch;
	}
	// 82EF89D0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82EF89D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF89D8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EF89DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF89E0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82EF89E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF89E8: 48041FD1  bl 0x82f3a9b8
	ctx.lr = 0x82EF89EC;
	sub_82F3A9B8(ctx, base);
	// 82EF89EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF89F0: 40820044  bne 0x82ef8a34
	if !ctx.cr[0].eq {
	pc = 0x82EF8A34; continue 'dispatch;
	}
	// 82EF89F4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EF89F8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82EF89FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8A00: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8A04: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8A08: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF8A10: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8A14: 4BFFF885  bl 0x82ef8298
	ctx.lr = 0x82EF8A18;
	sub_82EF8298(ctx, base);
	// 82EF8A18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A1C: 4BFFED35  bl 0x82ef7750
	ctx.lr = 0x82EF8A20;
	sub_82EF7750(ctx, base);
	// 82EF8A20: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EF8A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A28: 48041F91  bl 0x82f3a9b8
	ctx.lr = 0x82EF8A2C;
	sub_82F3A9B8(ctx, base);
	// 82EF8A2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8A30: 4182FFCC  beq 0x82ef89fc
	if ctx.cr[0].eq {
	pc = 0x82EF89FC; continue 'dispatch;
	}
	// 82EF8A34: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8A38: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EF8A3C: 48074BBD  bl 0x82f6d5f8
	ctx.lr = 0x82EF8A40;
	sub_82F6D5F8(ctx, base);
	// 82EF8A40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8A44: 483C0F11  bl 0x832b9954
	ctx.lr = 0x82EF8A48;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8A4C: 4BDB0A10  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8A50 size=252
    let mut pc: u32 = 0x82EF8A50;
    'dispatch: loop {
        match pc {
            0x82EF8A50 => {
    //   block [0x82EF8A50..0x82EF8B4C)
	// 82EF8A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8A54: 4BDB09B5  bl 0x82ca9408
	ctx.lr = 0x82EF8A58;
	sub_82CA93D0(ctx, base);
	// 82EF8A58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8A60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8A64: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 82EF8A68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8A6C: 483C0EF9  bl 0x832b9964
	ctx.lr = 0x82EF8A70;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF8A70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8A74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF8A78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8A7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8A80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8A84: 4E800421  bctrl
	ctx.lr = 0x82EF8A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8A88: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82EF8A8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF8A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8A94: 4BFFEE7D  bl 0x82ef7910
	ctx.lr = 0x82EF8A98;
	sub_82EF7910(ctx, base);
	// 82EF8A98: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EF8A9C: 41800018  blt 0x82ef8ab4
	if ctx.cr[0].lt {
	pc = 0x82EF8AB4; continue 'dispatch;
	}
	// 82EF8AA0: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82EF8AA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AA8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF8AAC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF8AB0: 48000008  b 0x82ef8ab8
	pc = 0x82EF8AB8; continue 'dispatch;
	// 82EF8AB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF8AB8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF8ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8AC0: 419A0018  beq cr6, 0x82ef8ad8
	if ctx.cr[6].eq {
	pc = 0x82EF8AD8; continue 'dispatch;
	}
	// 82EF8AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AC8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF8ACC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8AD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8AD4: 4E800421  bctrl
	ctx.lr = 0x82EF8AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8AD8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EF8ADC: 419A0060  beq cr6, 0x82ef8b3c
	if ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AE0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8AE8: 409A0054  bne cr6, 0x82ef8b3c
	if !ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AEC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8AF0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8AF4: 409A0048  bne cr6, 0x82ef8b3c
	if !ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8AF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF8B00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EF8B04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8B0C: 4E800421  bctrl
	ctx.lr = 0x82EF8B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8B10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EF8B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8B18: 4BFFECA9  bl 0x82ef77c0
	ctx.lr = 0x82EF8B1C;
	sub_82EF77C0(ctx, base);
	// 82EF8B1C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EF8B20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8B24: 419A0018  beq cr6, 0x82ef8b3c
	if ctx.cr[6].eq {
	pc = 0x82EF8B3C; continue 'dispatch;
	}
	// 82EF8B28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8B2C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EF8B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8B38: 4E800421  bctrl
	ctx.lr = 0x82EF8B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8B3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF8B40: 483C0E15  bl 0x832b9954
	ctx.lr = 0x82EF8B44;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF8B44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF8B48: 4BDB0910  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8B50 size=140
    let mut pc: u32 = 0x82EF8B50;
    'dispatch: loop {
        match pc {
            0x82EF8B50 => {
    //   block [0x82EF8B50..0x82EF8BDC)
	// 82EF8B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8B64: 48004DE5  bl 0x82efd948
	ctx.lr = 0x82EF8B68;
	sub_82EFD948(ctx, base);
	// 82EF8B68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF8B70: 396BCFB0  addi r11, r11, -0x3050
	ctx.r[11].s64 = ctx.r[11].s64 + -12368;
	// 82EF8B74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF8B78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8B80: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF8B84: 48004ED5  bl 0x82efda58
	ctx.lr = 0x82EF8B88;
	sub_82EFDA58(ctx, base);
	// 82EF8B88: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82EF8B8C: 48004CD5  bl 0x82efd860
	ctx.lr = 0x82EF8B90;
	sub_82EFD860(ctx, base);
	// 82EF8B90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8B94: 41820028  beq 0x82ef8bbc
	if ctx.cr[0].eq {
	pc = 0x82EF8BBC; continue 'dispatch;
	}
	// 82EF8B98: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EF8B9C: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EF8BA0: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EF8BA4: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EF8BA8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF8BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EF8BB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EF8BB4: 4BFFF96D  bl 0x82ef8520
	ctx.lr = 0x82EF8BB8;
	sub_82EF8520(ctx, base);
	// 82EF8BB8: 48000008  b 0x82ef8bc0
	pc = 0x82EF8BC0; continue 'dispatch;
	// 82EF8BBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF8BC0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EF8BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8BD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8BE0 size=108
    let mut pc: u32 = 0x82EF8BE0;
    'dispatch: loop {
        match pc {
            0x82EF8BE0 => {
    //   block [0x82EF8BE0..0x82EF8C4C)
	// 82EF8BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8BF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8BF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF8BF8: 396BCFB0  addi r11, r11, -0x3050
	ctx.r[11].s64 = ctx.r[11].s64 + -12368;
	// 82EF8BFC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8C00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF8C04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF8C08: 419A0020  beq cr6, 0x82ef8c28
	if ctx.cr[6].eq {
	pc = 0x82EF8C28; continue 'dispatch;
	}
	// 82EF8C0C: 4BFFFD55  bl 0x82ef8960
	ctx.lr = 0x82EF8C10;
	sub_82EF8960(ctx, base);
	// 82EF8C10: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8C14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF8C18: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8C1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF8C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8C24: 4E800421  bctrl
	ctx.lr = 0x82EF8C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8C28: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82EF8C2C: 480749CD  bl 0x82f6d5f8
	ctx.lr = 0x82EF8C30;
	sub_82F6D5F8(ctx, base);
	// 82EF8C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C34: 48004DE5  bl 0x82efda18
	ctx.lr = 0x82EF8C38;
	sub_82EFDA18(ctx, base);
	// 82EF8C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8C50 size=76
    let mut pc: u32 = 0x82EF8C50;
    'dispatch: loop {
        match pc {
            0x82EF8C50 => {
    //   block [0x82EF8C50..0x82EF8C9C)
	// 82EF8C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8C6C: 4BFFF6CD  bl 0x82ef8338
	ctx.lr = 0x82EF8C70;
	sub_82EF8338(ctx, base);
	// 82EF8C70: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8C74: 4182000C  beq 0x82ef8c80
	if ctx.cr[0].eq {
	pc = 0x82EF8C80; continue 'dispatch;
	}
	// 82EF8C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C7C: 48004C25  bl 0x82efd8a0
	ctx.lr = 0x82EF8C80;
	sub_82EFD8A0(ctx, base);
	// 82EF8C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8CA0 size=76
    let mut pc: u32 = 0x82EF8CA0;
    'dispatch: loop {
        match pc {
            0x82EF8CA0 => {
    //   block [0x82EF8CA0..0x82EF8CEC)
	// 82EF8CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8CB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8CBC: 4BFFF8D5  bl 0x82ef8590
	ctx.lr = 0x82EF8CC0;
	sub_82EF8590(ctx, base);
	// 82EF8CC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8CC4: 4182000C  beq 0x82ef8cd0
	if ctx.cr[0].eq {
	pc = 0x82EF8CD0; continue 'dispatch;
	}
	// 82EF8CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8CCC: 48004BD5  bl 0x82efd8a0
	ctx.lr = 0x82EF8CD0;
	sub_82EFD8A0(ctx, base);
	// 82EF8CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8CD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8CF0 size=76
    let mut pc: u32 = 0x82EF8CF0;
    'dispatch: loop {
        match pc {
            0x82EF8CF0 => {
    //   block [0x82EF8CF0..0x82EF8D3C)
	// 82EF8CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8D08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8D0C: 4BFFFED5  bl 0x82ef8be0
	ctx.lr = 0x82EF8D10;
	sub_82EF8BE0(ctx, base);
	// 82EF8D10: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8D14: 4182000C  beq 0x82ef8d20
	if ctx.cr[0].eq {
	pc = 0x82EF8D20; continue 'dispatch;
	}
	// 82EF8D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D1C: 48004B85  bl 0x82efd8a0
	ctx.lr = 0x82EF8D20;
	sub_82EFD8A0(ctx, base);
	// 82EF8D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF8D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8D40 size=48
    let mut pc: u32 = 0x82EF8D40;
    'dispatch: loop {
        match pc {
            0x82EF8D40 => {
    //   block [0x82EF8D40..0x82EF8D70)
	// 82EF8D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8D54: 483C0F41  bl 0x832b9c94
	ctx.lr = 0x82EF8D58;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EF8D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8D5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF8D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF8D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF8D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF8D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8D70 size=116
    let mut pc: u32 = 0x82EF8D70;
    'dispatch: loop {
        match pc {
            0x82EF8D70 => {
    //   block [0x82EF8D70..0x82EF8DE4)
	// 82EF8D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8D74: 4BDB0695  bl 0x82ca9408
	ctx.lr = 0x82EF8D78;
	sub_82CA93D0(ctx, base);
	// 82EF8D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8D7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF8D80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF8D84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF8D88: 2B1E0020  cmplwi cr6, r30, 0x20
	ctx.cr[6].compare_u32(ctx.r[30].u32, 32 as u32, &mut ctx.xer);
	// 82EF8D8C: 40990010  ble cr6, 0x82ef8d9c
	if !ctx.cr[6].gt {
	pc = 0x82EF8D9C; continue 'dispatch;
	}
	// 82EF8D90: 57C3103A  slwi r3, r30, 2
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EF8D94: 48004ACD  bl 0x82efd860
	ctx.lr = 0x82EF8D98;
	sub_82EFD860(ctx, base);
	// 82EF8D98: 48000008  b 0x82ef8da0
	pc = 0x82EF8DA0; continue 'dispatch;
	// 82EF8D9C: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82EF8DA0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF8DA4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8DA8: 419A0030  beq cr6, 0x82ef8dd8
	if ctx.cr[6].eq {
	pc = 0x82EF8DD8; continue 'dispatch;
	}
	// 82EF8DAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8DB0: 7C7FE02E  lwzx r3, r31, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EF8DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8DB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8DC0: 4E800421  bctrl
	ctx.lr = 0x82EF8DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8DC4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8DC8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF8DCC: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82EF8DD0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EF8DD4: 4082FFDC  bne 0x82ef8db0
	if !ctx.cr[0].eq {
	pc = 0x82EF8DB0; continue 'dispatch;
	}
	// 82EF8DD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8DE0: 4BDB0678  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8DE8 size=204
    let mut pc: u32 = 0x82EF8DE8;
    'dispatch: loop {
        match pc {
            0x82EF8DE8 => {
    //   block [0x82EF8DE8..0x82EF8EB4)
	// 82EF8DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8DEC: 4BDB061D  bl 0x82ca9408
	ctx.lr = 0x82EF8DF0;
	sub_82CA93D0(ctx, base);
	// 82EF8DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8DF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8DF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EF8DFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8E00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E04: 419A0034  beq cr6, 0x82ef8e38
	if ctx.cr[6].eq {
	pc = 0x82EF8E38; continue 'dispatch;
	}
	// 82EF8E08: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82EF8E0C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8E18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E1C: 4E800421  bctrl
	ctx.lr = 0x82EF8E20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8E20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8E24: 41820014  beq 0x82ef8e38
	if ctx.cr[0].eq {
	pc = 0x82EF8E38; continue 'dispatch;
	}
	// 82EF8E28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF8E2C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF8E30: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8E34: 4198FFD8  blt cr6, 0x82ef8e0c
	if ctx.cr[6].lt {
	pc = 0x82EF8E0C; continue 'dispatch;
	}
	// 82EF8E38: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF8E3C: 409A0038  bne cr6, 0x82ef8e74
	if !ctx.cr[6].eq {
	pc = 0x82EF8E74; continue 'dispatch;
	}
	// 82EF8E40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E44: 419A0028  beq cr6, 0x82ef8e6c
	if ctx.cr[6].eq {
	pc = 0x82EF8E6C; continue 'dispatch;
	}
	// 82EF8E48: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82EF8E4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E54: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E5C: 4E800421  bctrl
	ctx.lr = 0x82EF8E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8E60: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF8E64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EF8E68: 4082FFE4  bne 0x82ef8e4c
	if !ctx.cr[0].eq {
	pc = 0x82EF8E4C; continue 'dispatch;
	}
	// 82EF8E6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF8E70: 4800003C  b 0x82ef8eac
	pc = 0x82EF8EAC; continue 'dispatch;
	// 82EF8E74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8E78: 419A0030  beq cr6, 0x82ef8ea8
	if ctx.cr[6].eq {
	pc = 0x82EF8EA8; continue 'dispatch;
	}
	// 82EF8E7C: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF8E80: 7FCBE214  add r30, r11, r28
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF8E84: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82EF8E88: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EF8E8C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8E94: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF8E98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8E9C: 4E800421  bctrl
	ctx.lr = 0x82EF8EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8EA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF8EA4: 409AFFE0  bne cr6, 0x82ef8e84
	if !ctx.cr[6].eq {
	pc = 0x82EF8E84; continue 'dispatch;
	}
	// 82EF8EA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF8EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8EB0: 4BDB05A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8EB8 size=136
    let mut pc: u32 = 0x82EF8EB8;
    'dispatch: loop {
        match pc {
            0x82EF8EB8 => {
    //   block [0x82EF8EB8..0x82EF8F40)
	// 82EF8EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8EBC: 4BDB054D  bl 0x82ca9408
	ctx.lr = 0x82EF8EC0;
	sub_82CA93D0(ctx, base);
	// 82EF8EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8EC4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF8EC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF8ECC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EF8ED0: 419A0048  beq cr6, 0x82ef8f18
	if ctx.cr[6].eq {
	pc = 0x82EF8F18; continue 'dispatch;
	}
	// 82EF8ED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF8ED8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8EE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8EE8: 4E800421  bctrl
	ctx.lr = 0x82EF8EEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8EEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EF8EF0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8EF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF8EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8EFC: 4E800421  bctrl
	ctx.lr = 0x82EF8F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8F00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF8F04: 40820020  bne 0x82ef8f24
	if !ctx.cr[0].eq {
	pc = 0x82EF8F24; continue 'dispatch;
	}
	// 82EF8F08: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF8F0C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EF8F10: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF8F14: 4198FFC4  blt cr6, 0x82ef8ed8
	if ctx.cr[6].lt {
	pc = 0x82EF8ED8; continue 'dispatch;
	}
	// 82EF8F18: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EF8F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF8F20: 4BDB0538  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82EF8F24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF8F28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8F2C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF8F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF8F34: 4E800421  bctrl
	ctx.lr = 0x82EF8F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF8F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF8F3C: 4BFFFFE0  b 0x82ef8f1c
	pc = 0x82EF8F1C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8F40 size=160
    let mut pc: u32 = 0x82EF8F40;
    'dispatch: loop {
        match pc {
            0x82EF8F40 => {
    //   block [0x82EF8F40..0x82EF8FE0)
	// 82EF8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8F44: 4BDB04C9  bl 0x82ca940c
	ctx.lr = 0x82EF8F48;
	sub_82CA93D0(ctx, base);
	// 82EF8F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8F50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF8F54: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EF8F58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8F5C: 48001BFD  bl 0x82efab58
	ctx.lr = 0x82EF8F60;
	sub_82EFAB58(ctx, base);
	// 82EF8F60: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF8F64: 419A004C  beq cr6, 0x82ef8fb0
	if ctx.cr[6].eq {
	pc = 0x82EF8FB0; continue 'dispatch;
	}
	// 82EF8F68: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8F6C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF8F70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8F74: 4082003C  bne 0x82ef8fb0
	if !ctx.cr[0].eq {
	pc = 0x82EF8FB0; continue 'dispatch;
	}
	// 82EF8F78: 419A0018  beq cr6, 0x82ef8f90
	if ctx.cr[6].eq {
	pc = 0x82EF8F90; continue 'dispatch;
	}
	// 82EF8F7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF8F80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF8F84: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF8F88: 48002079  bl 0x82efb000
	ctx.lr = 0x82EF8F8C;
	sub_82EFB000(ctx, base);
	// 82EF8F8C: 48000024  b 0x82ef8fb0
	pc = 0x82EF8FB0; continue 'dispatch;
	// 82EF8F90: 3BDF0038  addi r30, r31, 0x38
	ctx.r[30].s64 = ctx.r[31].s64 + 56;
	// 82EF8F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF8F98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF8F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8FA0: 48002061  bl 0x82efb000
	ctx.lr = 0x82EF8FA4;
	sub_82EFB000(ctx, base);
	// 82EF8FA4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8FA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8FAC: 4182FFE8  beq 0x82ef8f94
	if ctx.cr[0].eq {
	pc = 0x82EF8F94; continue 'dispatch;
	}
	// 82EF8FB0: 8BDF0018  lbz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF8FB4: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82EF8FB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF8FBC: 41820010  beq 0x82ef8fcc
	if ctx.cr[0].eq {
	pc = 0x82EF8FCC; continue 'dispatch;
	}
	// 82EF8FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF8FC4: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF8FC8: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EF8FCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF8FD0: 48002AD1  bl 0x82efbaa0
	ctx.lr = 0x82EF8FD4;
	sub_82EFBAA0(ctx, base);
	// 82EF8FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF8FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF8FDC: 4BDB0480  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF8FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF8FE0 size=80
    let mut pc: u32 = 0x82EF8FE0;
    'dispatch: loop {
        match pc {
            0x82EF8FE0 => {
    //   block [0x82EF8FE0..0x82EF9030)
	// 82EF8FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF8FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF8FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF8FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF8FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF8FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF8FF8: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EF8FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9000: 48001B59  bl 0x82efab58
	ctx.lr = 0x82EF9004;
	sub_82EFAB58(ctx, base);
	// 82EF9004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF900C: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EF9010: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9014: 48002A8D  bl 0x82efbaa0
	ctx.lr = 0x82EF9018;
	sub_82EFBAA0(ctx, base);
	// 82EF9018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF901C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9024: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF902C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9030 size=8
    let mut pc: u32 = 0x82EF9030;
    'dispatch: loop {
        match pc {
            0x82EF9030 => {
    //   block [0x82EF9030..0x82EF9038)
	// 82EF9030: 88630018  lbz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9038 size=12
    let mut pc: u32 = 0x82EF9038;
    'dispatch: loop {
        match pc {
            0x82EF9038 => {
    //   block [0x82EF9038..0x82EF9044)
	// 82EF9038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF903C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82EF9040: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9044 size=8
    let mut pc: u32 = 0x82EF9044;
    'dispatch: loop {
        match pc {
            0x82EF9044 => {
    //   block [0x82EF9044..0x82EF904C)
	// 82EF9044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF9048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9050 size=20
    let mut pc: u32 = 0x82EF9050;
    'dispatch: loop {
        match pc {
            0x82EF9050 => {
    //   block [0x82EF9050..0x82EF9064)
	// 82EF9050: 8163FFEC  lwz r11, -0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20 as u32) ) } as u64;
	// 82EF9054: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF9058: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF905C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF9060: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9068 size=96
    let mut pc: u32 = 0x82EF9068;
    'dispatch: loop {
        match pc {
            0x82EF9068 => {
    //   block [0x82EF9068..0x82EF90C8)
	// 82EF9068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF907C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9080: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82EF9084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9088: 48001AD1  bl 0x82efab58
	ctx.lr = 0x82EF908C;
	sub_82EFAB58(ctx, base);
	// 82EF908C: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82EF9090: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF9094: 41820010  beq 0x82ef90a4
	if ctx.cr[0].eq {
	pc = 0x82EF90A4; continue 'dispatch;
	}
	// 82EF9098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF909C: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82EF90A0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82EF90A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF90A8: 480029F9  bl 0x82efbaa0
	ctx.lr = 0x82EF90AC;
	sub_82EFBAA0(ctx, base);
	// 82EF90AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF90B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF90B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF90B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF90BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF90C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF90C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF90C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF90C8 size=288
    let mut pc: u32 = 0x82EF90C8;
    'dispatch: loop {
        match pc {
            0x82EF90C8 => {
    //   block [0x82EF90C8..0x82EF91E8)
	// 82EF90C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF90CC: 4BDB0335  bl 0x82ca9400
	ctx.lr = 0x82EF90D0;
	sub_82CA93D0(ctx, base);
	// 82EF90D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF90D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF90D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF90DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EF90E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF90E4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF90E8: 40990010  ble cr6, 0x82ef90f8
	if !ctx.cr[6].gt {
	pc = 0x82EF90F8; continue 'dispatch;
	}
	// 82EF90EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF90F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EF90F4: 4BDB035C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82EF90F8: 3B7F0020  addi r27, r31, 0x20
	ctx.r[27].s64 = ctx.r[31].s64 + 32;
	// 82EF90FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF9100: 48001A59  bl 0x82efab58
	ctx.lr = 0x82EF9104;
	sub_82EFAB58(ctx, base);
	// 82EF9104: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9108: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF910C: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EF9110: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF9114: 41990024  bgt cr6, 0x82ef9138
	if ctx.cr[6].gt {
	pc = 0x82EF9138; continue 'dispatch;
	}
	// 82EF9118: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF911C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82EF9120: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF9124: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9128: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EF912C: 48002975  bl 0x82efbaa0
	ctx.lr = 0x82EF9130;
	sub_82EFBAA0(ctx, base);
	// 82EF9130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9134: 4BFFFFBC  b 0x82ef90f0
	pc = 0x82EF90F0; continue 'dispatch;
	// 82EF9138: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF913C: 409A000C  bne cr6, 0x82ef9148
	if !ctx.cr[6].eq {
	pc = 0x82EF9148; continue 'dispatch;
	}
	// 82EF9140: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF9144: 4BFFFFE4  b 0x82ef9128
	pc = 0x82EF9128; continue 'dispatch;
	// 82EF9148: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EF914C: 409A0040  bne cr6, 0x82ef918c
	if !ctx.cr[6].eq {
	pc = 0x82EF918C; continue 'dispatch;
	}
	// 82EF9150: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9154: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82EF9158: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EF915C: 4099FFBC  ble cr6, 0x82ef9118
	if !ctx.cr[6].gt {
	pc = 0x82EF9118; continue 'dispatch;
	}
	// 82EF9160: 3BDF003C  addi r30, r31, 0x3c
	ctx.r[30].s64 = ctx.r[31].s64 + 60;
	// 82EF9164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF9168: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF916C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9170: 48001E91  bl 0x82efb000
	ctx.lr = 0x82EF9174;
	sub_82EFB000(ctx, base);
	// 82EF9174: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9178: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF917C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF9180: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9184: 4199FFE0  bgt cr6, 0x82ef9164
	if ctx.cr[6].gt {
	pc = 0x82EF9164; continue 'dispatch;
	}
	// 82EF9188: 4BFFFF90  b 0x82ef9118
	pc = 0x82EF9118; continue 'dispatch;
	// 82EF918C: 480063E5  bl 0x82eff570
	ctx.lr = 0x82EF9190;
	sub_82EFF570(ctx, base);
	// 82EF9190: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 82EF9194: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EF9198: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF919C: 48000034  b 0x82ef91d0
	pc = 0x82EF91D0; continue 'dispatch;
	// 82EF91A0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF91A4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF91A8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EF91AC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF91B0: 4099FF68  ble cr6, 0x82ef9118
	if !ctx.cr[6].gt {
	pc = 0x82EF9118; continue 'dispatch;
	}
	// 82EF91B4: 480063BD  bl 0x82eff570
	ctx.lr = 0x82EF91B8;
	sub_82EFF570(ctx, base);
	// 82EF91B8: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF91BC: 574A003E  slwi r10, r26, 0
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF91C0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF91C4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EF91C8: 4098FF78  bge cr6, 0x82ef9140
	if !ctx.cr[6].lt {
	pc = 0x82EF9140; continue 'dispatch;
	}
	// 82EF91CC: 7CABF050  subf r5, r11, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82EF91D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EF91D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF91D8: 48001E29  bl 0x82efb000
	ctx.lr = 0x82EF91DC;
	sub_82EFB000(ctx, base);
	// 82EF91DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF91E0: 4082FFC0  bne 0x82ef91a0
	if !ctx.cr[0].eq {
	pc = 0x82EF91A0; continue 'dispatch;
	}
	// 82EF91E4: 4BFFFF5C  b 0x82ef9140
	pc = 0x82EF9140; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF91E8 size=116
    let mut pc: u32 = 0x82EF91E8;
    'dispatch: loop {
        match pc {
            0x82EF91E8 => {
    //   block [0x82EF91E8..0x82EF925C)
	// 82EF91E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF91EC: 4BDB0221  bl 0x82ca940c
	ctx.lr = 0x82EF91F0;
	sub_82CA93D0(ctx, base);
	// 82EF91F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF91F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF91F8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EF91FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9200: 48001959  bl 0x82efab58
	ctx.lr = 0x82EF9204;
	sub_82EFAB58(ctx, base);
	// 82EF9204: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9208: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF920C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9210: 41980028  blt cr6, 0x82ef9238
	if ctx.cr[6].lt {
	pc = 0x82EF9238; continue 'dispatch;
	}
	// 82EF9214: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 82EF9218: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF921C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EF9220: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9224: 48001DDD  bl 0x82efb000
	ctx.lr = 0x82EF9228;
	sub_82EFB000(ctx, base);
	// 82EF9228: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF922C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9230: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9234: 4098FFE4  bge cr6, 0x82ef9218
	if !ctx.cr[6].lt {
	pc = 0x82EF9218; continue 'dispatch;
	}
	// 82EF9238: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF923C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9240: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EF9244: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9248: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF924C: 48002855  bl 0x82efbaa0
	ctx.lr = 0x82EF9250;
	sub_82EFBAA0(ctx, base);
	// 82EF9250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9258: 4BDB0204  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9260 size=128
    let mut pc: u32 = 0x82EF9260;
    'dispatch: loop {
        match pc {
            0x82EF9260 => {
    //   block [0x82EF9260..0x82EF92E0)
	// 82EF9260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9264: 4BDB01A5  bl 0x82ca9408
	ctx.lr = 0x82EF9268;
	sub_82CA93D0(ctx, base);
	// 82EF9268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF926C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9274: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF927C: 480018DD  bl 0x82efab58
	ctx.lr = 0x82EF9280;
	sub_82EFAB58(ctx, base);
	// 82EF9280: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9284: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9288: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF928C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF9290: 4099002C  ble cr6, 0x82ef92bc
	if !ctx.cr[6].gt {
	pc = 0x82EF92BC; continue 'dispatch;
	}
	// 82EF9294: 3B9F003C  addi r28, r31, 0x3c
	ctx.r[28].s64 = ctx.r[31].s64 + 60;
	// 82EF9298: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EF929C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF92A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EF92A4: 48001D5D  bl 0x82efb000
	ctx.lr = 0x82EF92A8;
	sub_82EFB000(ctx, base);
	// 82EF92A8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92AC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF92B0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF92B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EF92B8: 4199FFE0  bgt cr6, 0x82ef9298
	if ctx.cr[6].gt {
	pc = 0x82EF9298; continue 'dispatch;
	}
	// 82EF92BC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF92C4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EF92C8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF92CC: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF92D0: 480027D1  bl 0x82efbaa0
	ctx.lr = 0x82EF92D4;
	sub_82EFBAA0(ctx, base);
	// 82EF92D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF92D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF92DC: 4BDB017C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF92E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF92E0 size=32
    let mut pc: u32 = 0x82EF92E0;
    'dispatch: loop {
        match pc {
            0x82EF92E0 => {
    //   block [0x82EF92E0..0x82EF9300)
	// 82EF92E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF92E4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF92E8: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF92EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF92F0: 41810008  bgt 0x82ef92f8
	if ctx.cr[0].gt {
	pc = 0x82EF92F8; continue 'dispatch;
	}
	// 82EF92F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF92F8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF92FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9300 size=16
    let mut pc: u32 = 0x82EF9300;
    'dispatch: loop {
        match pc {
            0x82EF9300 => {
    //   block [0x82EF9300..0x82EF9310)
	// 82EF9300: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9304: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9308: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF930C: 4BFFFDBC  b 0x82ef90c8
	sub_82EF90C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9310 size=32
    let mut pc: u32 = 0x82EF9310;
    'dispatch: loop {
        match pc {
            0x82EF9310 => {
    //   block [0x82EF9310..0x82EF9330)
	// 82EF9310: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9314: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF9318: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF931C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF9320: 41810008  bgt 0x82ef9328
	if ctx.cr[0].gt {
	pc = 0x82EF9328; continue 'dispatch;
	}
	// 82EF9324: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9328: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF932C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9330 size=20
    let mut pc: u32 = 0x82EF9330;
    'dispatch: loop {
        match pc {
            0x82EF9330 => {
    //   block [0x82EF9330..0x82EF9344)
	// 82EF9330: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9338: 409A000C  bne cr6, 0x82ef9344
	if !ctx.cr[6].eq {
		sub_82EF9344(ctx, base);
		return;
	}
	// 82EF933C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF9340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9344 size=40
    let mut pc: u32 = 0x82EF9344;
    'dispatch: loop {
        match pc {
            0x82EF9344 => {
    //   block [0x82EF9344..0x82EF936C)
	// 82EF9344: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9348: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF934C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9350: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF9354: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF9358: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF935C: 40980008  bge cr6, 0x82ef9364
	if !ctx.cr[6].lt {
	pc = 0x82EF9364; continue 'dispatch;
	}
	// 82EF9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9364: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF9368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9370 size=20
    let mut pc: u32 = 0x82EF9370;
    'dispatch: loop {
        match pc {
            0x82EF9370 => {
    //   block [0x82EF9370..0x82EF9384)
	// 82EF9370: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EF9374: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF937C: 409A0008  bne cr6, 0x82ef9384
	if !ctx.cr[6].eq {
		sub_82EF9384(ctx, base);
		return;
	}
	// 82EF9380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9384 size=12
    let mut pc: u32 = 0x82EF9384;
    'dispatch: loop {
        match pc {
            0x82EF9384 => {
    //   block [0x82EF9384..0x82EF9390)
	// 82EF9384: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9388: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF938C: 4BFFFD3C  b 0x82ef90c8
	sub_82EF90C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9390 size=20
    let mut pc: u32 = 0x82EF9390;
    'dispatch: loop {
        match pc {
            0x82EF9390 => {
    //   block [0x82EF9390..0x82EF93A4)
	// 82EF9390: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9394: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EF9398: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EF939C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82EF93A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF93A8 size=20
    let mut pc: u32 = 0x82EF93A8;
    'dispatch: loop {
        match pc {
            0x82EF93A8 => {
    //   block [0x82EF93A8..0x82EF93BC)
	// 82EF93A8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF93AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF93B0: 409A000C  bne cr6, 0x82ef93bc
	if !ctx.cr[6].eq {
		sub_82EF93BC(ctx, base);
		return;
	}
	// 82EF93B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF93B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF93BC size=40
    let mut pc: u32 = 0x82EF93BC;
    'dispatch: loop {
        match pc {
            0x82EF93BC => {
    //   block [0x82EF93BC..0x82EF93E4)
	// 82EF93BC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF93C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EF93C4: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF93C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EF93CC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EF93D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF93D4: 40980008  bge cr6, 0x82ef93dc
	if !ctx.cr[6].lt {
	pc = 0x82EF93DC; continue 'dispatch;
	}
	// 82EF93D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF93DC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EF93E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF93E8 size=96
    let mut pc: u32 = 0x82EF93E8;
    'dispatch: loop {
        match pc {
            0x82EF93E8 => {
    //   block [0x82EF93E8..0x82EF9448)
	// 82EF93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF93F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF93F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF93F8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 82EF93FC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EF9400: 3BEB8FB0  addi r31, r11, -0x7050
	ctx.r[31].s64 = ctx.r[11].s64 + -28752;
	// 82EF9404: 816A8FB4  lwz r11, -0x704c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28748 as u32) ) } as u64;
	// 82EF9408: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF940C: 40820024  bne 0x82ef9430
	if !ctx.cr[0].eq {
	pc = 0x82EF9430; continue 'dispatch;
	}
	// 82EF9410: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9414: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82EF9418: 3929C558  addi r9, r9, -0x3aa8
	ctx.r[9].s64 = ctx.r[9].s64 + -15016;
	// 82EF941C: 916A8FB4  stw r11, -0x704c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28748 as u32), ctx.r[11].u32 ) };
	// 82EF9420: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82EF9424: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EF9428: 386B8F98  addi r3, r11, -0x7068
	ctx.r[3].s64 = ctx.r[11].s64 + -28776;
	// 82EF942C: 4BDB0AF5  bl 0x82ca9f20
	ctx.lr = 0x82EF9430;
	sub_82CA9F20(ctx, base);
	// 82EF9430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9448 size=196
    let mut pc: u32 = 0x82EF9448;
    'dispatch: loop {
        match pc {
            0x82EF9448 => {
    //   block [0x82EF9448..0x82EF950C)
	// 82EF9448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF945C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9460: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9464: 556A17BF  rlwinm. r10, r11, 2, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9468: 41820028  beq 0x82ef9490
	if ctx.cr[0].eq {
	pc = 0x82EF9490; continue 'dispatch;
	}
	// 82EF946C: 55690FFF  rlwinm. r9, r11, 1, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF9470: 41820010  beq 0x82ef9480
	if ctx.cr[0].eq {
	pc = 0x82EF9480; continue 'dispatch;
	}
	// 82EF9474: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EF9478: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF947C: 41980078  blt cr6, 0x82ef94f4
	if ctx.cr[6].lt {
	pc = 0x82EF94F4; continue 'dispatch;
	}
	// 82EF9480: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EF9484: 4182000C  beq 0x82ef9490
	if ctx.cr[0].eq {
	pc = 0x82EF9490; continue 'dispatch;
	}
	// 82EF9488: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF948C: 419A0068  beq cr6, 0x82ef94f4
	if ctx.cr[6].eq {
	pc = 0x82EF94F4; continue 'dispatch;
	}
	// 82EF9490: 556B0002  rlwinm r11, r11, 0, 0, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EF9494: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9498: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EF949C: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82EF94A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF94A4: 409A001C  bne cr6, 0x82ef94c0
	if !ctx.cr[6].eq {
	pc = 0x82EF94C0; continue 'dispatch;
	}
	// 82EF94A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF94AC: 419A0008  beq cr6, 0x82ef94b4
	if ctx.cr[6].eq {
	pc = 0x82EF94B4; continue 'dispatch;
	}
	// 82EF94B0: 480043F1  bl 0x82efd8a0
	ctx.lr = 0x82EF94B4;
	sub_82EFD8A0(ctx, base);
	// 82EF94B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF94B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF94BC: 48000038  b 0x82ef94f4
	pc = 0x82EF94F4; continue 'dispatch;
	// 82EF94C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF94C4: 419A0014  beq cr6, 0x82ef94d8
	if ctx.cr[6].eq {
	pc = 0x82EF94D8; continue 'dispatch;
	}
	// 82EF94C8: 54841838  slwi r4, r4, 3
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EF94CC: 480043B5  bl 0x82efd880
	ctx.lr = 0x82EF94D0;
	sub_82EFD880(ctx, base);
	// 82EF94D0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF94D4: 48000020  b 0x82ef94f4
	pc = 0x82EF94F4; continue 'dispatch;
	// 82EF94D8: 549E1838  slwi r30, r4, 3
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EF94DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF94E0: 48004381  bl 0x82efd860
	ctx.lr = 0x82EF94E4;
	sub_82EFD860(ctx, base);
	// 82EF94E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF94E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF94EC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EF94F0: 4BDB04C1  bl 0x82ca99b0
	ctx.lr = 0x82EF94F4;
	sub_82CA99B0(ctx, base);
	// 82EF94F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF94F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF94FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9500: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9510 size=4
    let mut pc: u32 = 0x82EF9510;
    'dispatch: loop {
        match pc {
            0x82EF9510 => {
    //   block [0x82EF9510..0x82EF9514)
	// 82EF9510: 4BFFFED8  b 0x82ef93e8
	sub_82EF93E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9518 size=132
    let mut pc: u32 = 0x82EF9518;
    'dispatch: loop {
        match pc {
            0x82EF9518 => {
    //   block [0x82EF9518..0x82EF959C)
	// 82EF9518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF951C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF952C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9534: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9538: 48039E51  bl 0x82f33388
	ctx.lr = 0x82EF953C;
	sub_82F33388(ctx, base);
	// 82EF953C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EF9544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9548: 4099003C  ble cr6, 0x82ef9584
	if !ctx.cr[6].gt {
	pc = 0x82EF9584; continue 'dispatch;
	}
	// 82EF954C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9550: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9554: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EF9558: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF955C: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EF9560: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82EF9564: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF9568: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF956C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82EF9570: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9574: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82EF9578: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF957C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EF9580: 4198FFD0  blt cr6, 0x82ef9550
	if ctx.cr[6].lt {
	pc = 0x82EF9550; continue 'dispatch;
	}
	// 82EF9584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF95A0 size=128
    let mut pc: u32 = 0x82EF95A0;
    'dispatch: loop {
        match pc {
            0x82EF95A0 => {
    //   block [0x82EF95A0..0x82EF9620)
	// 82EF95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF95A4: 4BDAFE61  bl 0x82ca9404
	ctx.lr = 0x82EF95A8;
	sub_82CA93D0(ctx, base);
	// 82EF95A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF95AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF95B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF95B4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF95B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF95BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF95C0: 409A000C  bne cr6, 0x82ef95cc
	if !ctx.cr[6].eq {
	pc = 0x82EF95CC; continue 'dispatch;
	}
	// 82EF95C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF95C8: 48000050  b 0x82ef9618
	pc = 0x82EF9618; continue 'dispatch;
	// 82EF95CC: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	// 82EF95D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF95D4: 483C0391  bl 0x832b9964
	ctx.lr = 0x82EF95D8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF95D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF95DC: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82EF95E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF95E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF95E8: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82EF95EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EF95F0: 48039D99  bl 0x82f33388
	ctx.lr = 0x82EF95F4;
	sub_82F33388(ctx, base);
	// 82EF95F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF95F8: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EF95FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9600: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF9604: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
	// 82EF9608: 938BFFF8  stw r28, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 82EF960C: 936BFFFC  stw r27, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[27].u32 ) };
	// 82EF9610: 483C0345  bl 0x832b9954
	ctx.lr = 0x82EF9614;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9614: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF9618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF961C: 4BDAFE38  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9620 size=108
    let mut pc: u32 = 0x82EF9620;
    'dispatch: loop {
        match pc {
            0x82EF9620 => {
    //   block [0x82EF9620..0x82EF968C)
	// 82EF9620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF962C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9634: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9638: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82EF963C: 409A0010  bne cr6, 0x82ef964c
	if !ctx.cr[6].eq {
	pc = 0x82EF964C; continue 'dispatch;
	}
	// 82EF9640: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9644: 48039D45  bl 0x82f33388
	ctx.lr = 0x82EF9648;
	sub_82F33388(ctx, base);
	// 82EF9648: 48000030  b 0x82ef9678
	pc = 0x82EF9678; continue 'dispatch;
	// 82EF964C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9650: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EF9654: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82EF9658: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EF965C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9660: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 82EF9664: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EF9668: 4BDB74C9  bl 0x82cb0b30
	ctx.lr = 0x82EF966C;
	sub_82CB0B30(ctx, base);
	// 82EF966C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9670: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9674: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF9678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9690 size=176
    let mut pc: u32 = 0x82EF9690;
    'dispatch: loop {
        match pc {
            0x82EF9690 => {
    //   block [0x82EF9690..0x82EF9740)
	// 82EF9690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9694: 4BDAFD71  bl 0x82ca9404
	ctx.lr = 0x82EF9698;
	sub_82CA93D0(ctx, base);
	// 82EF9698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF969C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF96A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF96A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EF96A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF96AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF96B0: 409A000C  bne cr6, 0x82ef96bc
	if !ctx.cr[6].eq {
	pc = 0x82EF96BC; continue 'dispatch;
	}
	// 82EF96B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF96B8: 48000080  b 0x82ef9738
	pc = 0x82EF9738; continue 'dispatch;
	// 82EF96BC: 3BAB0010  addi r29, r11, 0x10
	ctx.r[29].s64 = ctx.r[11].s64 + 16;
	// 82EF96C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF96C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF96C8: 483C029D  bl 0x832b9964
	ctx.lr = 0x82EF96CC;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF96CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF96D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF96D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF96D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EF96DC: 419A0050  beq cr6, 0x82ef972c
	if ctx.cr[6].eq {
	pc = 0x82EF972C; continue 'dispatch;
	}
	// 82EF96E0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82EF96E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF96E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF96EC: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82EF96F0: 409A0014  bne cr6, 0x82ef9704
	if !ctx.cr[6].eq {
	pc = 0x82EF9704; continue 'dispatch;
	}
	// 82EF96F4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF96F8: 7F09D840  cmplw cr6, r9, r27
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82EF96FC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF9700: 419A0008  beq cr6, 0x82ef9708
	if ctx.cr[6].eq {
	pc = 0x82EF9708; continue 'dispatch;
	}
	// 82EF9704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EF9708: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EF970C: 40820018  bne 0x82ef9724
	if !ctx.cr[0].eq {
	pc = 0x82EF9724; continue 'dispatch;
	}
	// 82EF9710: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82EF9714: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EF9718: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EF971C: 4198FFCC  blt cr6, 0x82ef96e8
	if ctx.cr[6].lt {
	pc = 0x82EF96E8; continue 'dispatch;
	}
	// 82EF9720: 4800000C  b 0x82ef972c
	pc = 0x82EF972C; continue 'dispatch;
	// 82EF9724: 4BFFFEFD  bl 0x82ef9620
	ctx.lr = 0x82EF9728;
	sub_82EF9620(ctx, base);
	// 82EF9728: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82EF972C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9730: 483C0225  bl 0x832b9954
	ctx.lr = 0x82EF9734;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF973C: 4BDAFD18  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9740 size=156
    let mut pc: u32 = 0x82EF9740;
    'dispatch: loop {
        match pc {
            0x82EF9740 => {
    //   block [0x82EF9740..0x82EF97DC)
	// 82EF9740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9744: 4BDAFCC5  bl 0x82ca9408
	ctx.lr = 0x82EF9748;
	sub_82CA93D0(ctx, base);
	// 82EF9748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF974C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EF9750: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF9754: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EF9758: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF975C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9760: 40990038  ble cr6, 0x82ef9798
	if !ctx.cr[6].gt {
	pc = 0x82EF9798; continue 'dispatch;
	}
	// 82EF9764: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF9768: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF976C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF9770: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF9774: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9778: 4BFFFE29  bl 0x82ef95a0
	ctx.lr = 0x82EF977C;
	sub_82EF95A0(ctx, base);
	// 82EF977C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9780: 41820024  beq 0x82ef97a4
	if ctx.cr[0].eq {
	pc = 0x82EF97A4; continue 'dispatch;
	}
	// 82EF9784: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9788: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EF978C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF9790: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF9794: 4198FFD4  blt cr6, 0x82ef9768
	if ctx.cr[6].lt {
	pc = 0x82EF9768; continue 'dispatch;
	}
	// 82EF9798: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF979C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF97A0: 4BDAFCB8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82EF97A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF97A8: 419A002C  beq cr6, 0x82ef97d4
	if ctx.cr[6].eq {
	pc = 0x82EF97D4; continue 'dispatch;
	}
	// 82EF97AC: 57FD103A  slwi r29, r31, 2
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EF97B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF97B4: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82EF97B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EF97BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF97C0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EF97C4: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF97C8: 4BFFFEC9  bl 0x82ef9690
	ctx.lr = 0x82EF97CC;
	sub_82EF9690(ctx, base);
	// 82EF97CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF97D0: 409AFFE0  bne cr6, 0x82ef97b0
	if !ctx.cr[6].eq {
	pc = 0x82EF97B0; continue 'dispatch;
	}
	// 82EF97D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF97D8: 4BFFFFC4  b 0x82ef979c
	pc = 0x82EF979C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF97E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF97E0 size=88
    let mut pc: u32 = 0x82EF97E0;
    'dispatch: loop {
        match pc {
            0x82EF97E0 => {
    //   block [0x82EF97E0..0x82EF9838)
	// 82EF97E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF97E4: 4BDAFC25  bl 0x82ca9408
	ctx.lr = 0x82EF97E8;
	sub_82CA93D0(ctx, base);
	// 82EF97E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF97EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF97F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EF97F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EF97F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF97FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9800: 40990030  ble cr6, 0x82ef9830
	if !ctx.cr[6].gt {
	pc = 0x82EF9830; continue 'dispatch;
	}
	// 82EF9804: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EF9808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF980C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EF9810: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EF9814: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EF9818: 4BFFFE79  bl 0x82ef9690
	ctx.lr = 0x82EF981C;
	sub_82EF9690(ctx, base);
	// 82EF981C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF9820: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EF9824: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EF9828: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EF982C: 4198FFDC  blt cr6, 0x82ef9808
	if ctx.cr[6].lt {
	pc = 0x82EF9808; continue 'dispatch;
	}
	// 82EF9830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9834: 4BDAFC24  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9838 size=80
    let mut pc: u32 = 0x82EF9838;
    'dispatch: loop {
        match pc {
            0x82EF9838 => {
    //   block [0x82EF9838..0x82EF9888)
	// 82EF9838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF984C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9850: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EF9854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9858: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EF985C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EF9860: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EF9864: 4BFFF4DD  bl 0x82ef8d40
	ctx.lr = 0x82EF9868;
	sub_82EF8D40(ctx, base);
	// 82EF9868: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF986C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9870: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9888 size=152
    let mut pc: u32 = 0x82EF9888;
    'dispatch: loop {
        match pc {
            0x82EF9888 => {
    //   block [0x82EF9888..0x82EF9920)
	// 82EF9888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF989C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF98A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF98A4: 480040A5  bl 0x82efd948
	ctx.lr = 0x82EF98A8;
	sub_82EFD948(ctx, base);
	// 82EF98A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF98AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF98B0: 396BCFB4  addi r11, r11, -0x304c
	ctx.r[11].s64 = ctx.r[11].s64 + -12364;
	// 82EF98B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF98B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF98BC: 4800419D  bl 0x82efda58
	ctx.lr = 0x82EF98C0;
	sub_82EFDA58(ctx, base);
	// 82EF98C0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF98C4: 4182001C  beq 0x82ef98e0
	if ctx.cr[0].eq {
	pc = 0x82EF98E0; continue 'dispatch;
	}
	// 82EF98C8: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82EF98CC: 48003F95  bl 0x82efd860
	ctx.lr = 0x82EF98D0;
	sub_82EFD860(ctx, base);
	// 82EF98D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EF98D4: 4182000C  beq 0x82ef98e0
	if ctx.cr[0].eq {
	pc = 0x82EF98E0; continue 'dispatch;
	}
	// 82EF98D8: 4BFFFF61  bl 0x82ef9838
	ctx.lr = 0x82EF98DC;
	sub_82EF9838(ctx, base);
	// 82EF98DC: 48000008  b 0x82ef98e4
	pc = 0x82EF98E4; continue 'dispatch;
	// 82EF98E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EF98E4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82EF98E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF98EC: 419A0018  beq cr6, 0x82ef9904
	if ctx.cr[6].eq {
	pc = 0x82EF9904; continue 'dispatch;
	}
	// 82EF98F0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EF98F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EF98F8: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82EF98FC: 512AF002  rlwimi r10, r9, 0x1e, 0, 1
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(30) as u64) & 0x00000000C0000000) | (ctx.r[10].u64 & 0xFFFFFFFF3FFFFFFF);
	// 82EF9900: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EF9904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF990C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF991C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9920 size=76
    let mut pc: u32 = 0x82EF9920;
    'dispatch: loop {
        match pc {
            0x82EF9920 => {
    //   block [0x82EF9920..0x82EF996C)
	// 82EF9920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF992C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9930: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9934: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EF9938: 48001701  bl 0x82efb038
	ctx.lr = 0x82EF993C;
	sub_82EFB038(ctx, base);
	// 82EF993C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82EF9940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9944: 48039A45  bl 0x82f33388
	ctx.lr = 0x82EF9948;
	sub_82F33388(ctx, base);
	// 82EF9948: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF994C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9950: 419A0008  beq cr6, 0x82ef9958
	if ctx.cr[6].eq {
	pc = 0x82EF9958; continue 'dispatch;
	}
	// 82EF9954: 48003F4D  bl 0x82efd8a0
	ctx.lr = 0x82EF9958;
	sub_82EFD8A0(ctx, base);
	// 82EF9958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF995C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9970 size=188
    let mut pc: u32 = 0x82EF9970;
    'dispatch: loop {
        match pc {
            0x82EF9970 => {
    //   block [0x82EF9970..0x82EF9A2C)
	// 82EF9970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9974: 4BDAFA99  bl 0x82ca940c
	ctx.lr = 0x82EF9978;
	sub_82CA93D0(ctx, base);
	// 82EF9978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF997C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9980: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 82EF9984: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9988: 483BFFDD  bl 0x832b9964
	ctx.lr = 0x82EF998C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EF998C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EF9990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EF9994: 419A0088  beq cr6, 0x82ef9a1c
	if ctx.cr[6].eq {
	pc = 0x82EF9A1C; continue 'dispatch;
	}
	// 82EF9998: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82EF999C: 409A001C  bne cr6, 0x82ef99b8
	if !ctx.cr[6].eq {
	pc = 0x82EF99B8; continue 'dispatch;
	}
	// 82EF99A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99A4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF99AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF99B0: 4E800421  bctrl
	ctx.lr = 0x82EF99B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF99B4: 48000068  b 0x82ef9a1c
	pc = 0x82EF9A1C; continue 'dispatch;
	// 82EF99B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF99BC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82EF99C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF99C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF99C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EF99CC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EF99D0: 4BFFFB49  bl 0x82ef9518
	ctx.lr = 0x82EF99D4;
	sub_82EF9518(ctx, base);
	// 82EF99D4: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EF99D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF99DC: 419A0024  beq cr6, 0x82ef9a00
	if ctx.cr[6].eq {
	pc = 0x82EF9A00; continue 'dispatch;
	}
	// 82EF99E0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF99E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EF99E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF99EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EF99F0: 4E800421  bctrl
	ctx.lr = 0x82EF99F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EF99F4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF99F8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82EF99FC: 4082FFE8  bne 0x82ef99e4
	if !ctx.cr[0].eq {
	pc = 0x82EF99E4; continue 'dispatch;
	}
	// 82EF9A00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EF9A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EF9A08: 48039981  bl 0x82f33388
	ctx.lr = 0x82EF9A0C;
	sub_82F33388(ctx, base);
	// 82EF9A0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9A10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A14: 419A0008  beq cr6, 0x82ef9a1c
	if ctx.cr[6].eq {
	pc = 0x82EF9A1C; continue 'dispatch;
	}
	// 82EF9A18: 48003E89  bl 0x82efd8a0
	ctx.lr = 0x82EF9A1C;
	sub_82EFD8A0(ctx, base);
	// 82EF9A1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9A20: 483BFF35  bl 0x832b9954
	ctx.lr = 0x82EF9A24;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EF9A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9A28: 4BDAFA34  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9A30 size=12
    let mut pc: u32 = 0x82EF9A30;
    'dispatch: loop {
        match pc {
            0x82EF9A30 => {
    //   block [0x82EF9A30..0x82EF9A3C)
	// 82EF9A30: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9A34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9A3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9A3C size=8
    let mut pc: u32 = 0x82EF9A3C;
    'dispatch: loop {
        match pc {
            0x82EF9A3C => {
    //   block [0x82EF9A3C..0x82EF9A44)
	// 82EF9A3C: 4BFFFF34  b 0x82ef9970
	sub_82EF9970(ctx, base);
	return;
	// 82EF9A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9A48 size=108
    let mut pc: u32 = 0x82EF9A48;
    'dispatch: loop {
        match pc {
            0x82EF9A48 => {
    //   block [0x82EF9A48..0x82EF9AB4)
	// 82EF9A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9A5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EF9A60: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF9A64: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9A68: 7D40F828  lwarx r10, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EF9A6C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EF9A70: 7D20F92D  stwcx. r9, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF9A74: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9A78: 4082FFE8  bne 0x82ef9a60
	if !ctx.cr[0].eq {
	pc = 0x82EF9A60; continue 'dispatch;
	}
	// 82EF9A7C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EF9A80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9A84: 4082001C  bne 0x82ef9aa0
	if !ctx.cr[0].eq {
	pc = 0x82EF9AA0; continue 'dispatch;
	}
	// 82EF9A88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9A8C: 419A0014  beq cr6, 0x82ef9aa0
	if ctx.cr[6].eq {
	pc = 0x82EF9AA0; continue 'dispatch;
	}
	// 82EF9A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9A94: 4BFFFE8D  bl 0x82ef9920
	ctx.lr = 0x82EF9A98;
	sub_82EF9920(ctx, base);
	// 82EF9A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9A9C: 48003E05  bl 0x82efd8a0
	ctx.lr = 0x82EF9AA0;
	sub_82EFD8A0(ctx, base);
	// 82EF9AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9AB8 size=120
    let mut pc: u32 = 0x82EF9AB8;
    'dispatch: loop {
        match pc {
            0x82EF9AB8 => {
    //   block [0x82EF9AB8..0x82EF9B30)
	// 82EF9AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9ACC: 83E30010  lwz r31, 0x10(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9AD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9AD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9AD8: 419A0040  beq cr6, 0x82ef9b18
	if ctx.cr[6].eq {
	pc = 0x82EF9B18; continue 'dispatch;
	}
	// 82EF9ADC: 7C2004AC  lwsync
	// 82EF9AE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EF9AE4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EF9AE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9AEC: 7D60F828  lwarx r11, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EF9AF0: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EF9AF4: 7D20F92D  stwcx. r9, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EF9AF8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EF9AFC: 4082FFE8  bne 0x82ef9ae4
	if !ctx.cr[0].eq {
	pc = 0x82EF9AE4; continue 'dispatch;
	}
	// 82EF9B00: 7C2004AC  lwsync
	// 82EF9B04: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EF9B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9B0C: 419A0008  beq cr6, 0x82ef9b14
	if ctx.cr[6].eq {
	pc = 0x82EF9B14; continue 'dispatch;
	}
	// 82EF9B10: 4BFFFF39  bl 0x82ef9a48
	ctx.lr = 0x82EF9B14;
	sub_82EF9A48(ctx, base);
	// 82EF9B14: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EF9B18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9B24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9B30 size=76
    let mut pc: u32 = 0x82EF9B30;
    'dispatch: loop {
        match pc {
            0x82EF9B30 => {
    //   block [0x82EF9B30..0x82EF9B7C)
	// 82EF9B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9B48: 396BCFB4  addi r11, r11, -0x304c
	ctx.r[11].s64 = ctx.r[11].s64 + -12364;
	// 82EF9B4C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EF9B50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EF9B58: 419A0008  beq cr6, 0x82ef9b60
	if ctx.cr[6].eq {
	pc = 0x82EF9B60; continue 'dispatch;
	}
	// 82EF9B5C: 4BFFFEED  bl 0x82ef9a48
	ctx.lr = 0x82EF9B60;
	sub_82EF9A48(ctx, base);
	// 82EF9B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9B64: 48003EB5  bl 0x82efda18
	ctx.lr = 0x82EF9B68;
	sub_82EFDA18(ctx, base);
	// 82EF9B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EF9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9B74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9B80 size=136
    let mut pc: u32 = 0x82EF9B80;
    'dispatch: loop {
        match pc {
            0x82EF9B80 => {
    //   block [0x82EF9B80..0x82EF9C08)
	// 82EF9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9B88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9B8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9B94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9B98: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF9B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9BA0: 4BFFFCE9  bl 0x82ef9888
	ctx.lr = 0x82EF9BA4;
	sub_82EF9888(ctx, base);
	// 82EF9BA4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9BA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9BAC: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9BB0: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9BB4: 394ACFD4  addi r10, r10, -0x302c
	ctx.r[10].s64 = ctx.r[10].s64 + -12332;
	// 82EF9BB8: 3929CFC0  addi r9, r9, -0x3040
	ctx.r[9].s64 = ctx.r[9].s64 + -12352;
	// 82EF9BBC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9BC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF9BC4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82EF9BC8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF9BCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9BD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9BD4: 48000EF5  bl 0x82efaac8
	ctx.lr = 0x82EF9BD8;
	sub_82EFAAC8(ctx, base);
	// 82EF9BD8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9BDC: 480013D5  bl 0x82efafb0
	ctx.lr = 0x82EF9BE0;
	sub_82EFAFB0(ctx, base);
	// 82EF9BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9BE4: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82EF9BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9BEC: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9BF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9BFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9C08 size=112
    let mut pc: u32 = 0x82EF9C08;
    'dispatch: loop {
        match pc {
            0x82EF9C08 => {
    //   block [0x82EF9C08..0x82EF9C78)
	// 82EF9C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9C20: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9C24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9C28: 396BCFD4  addi r11, r11, -0x302c
	ctx.r[11].s64 = ctx.r[11].s64 + -12332;
	// 82EF9C2C: 394ACFC0  addi r10, r10, -0x3040
	ctx.r[10].s64 = ctx.r[10].s64 + -12352;
	// 82EF9C30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9C34: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9C38: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF9C3C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF9C40: 48001501  bl 0x82efb140
	ctx.lr = 0x82EF9C44;
	sub_82EFB140(ctx, base);
	// 82EF9C44: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82EF9C48: 48001491  bl 0x82efb0d8
	ctx.lr = 0x82EF9C4C;
	sub_82EFB0D8(ctx, base);
	// 82EF9C4C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9C54: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9C58: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9C5C: 4BFFFED5  bl 0x82ef9b30
	ctx.lr = 0x82EF9C60;
	sub_82EF9B30(ctx, base);
	// 82EF9C60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9C6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9C70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9C78 size=136
    let mut pc: u32 = 0x82EF9C78;
    'dispatch: loop {
        match pc {
            0x82EF9C78 => {
    //   block [0x82EF9C78..0x82EF9D00)
	// 82EF9C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9C80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9C84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9C90: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EF9C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9C98: 48000EC1  bl 0x82efab58
	ctx.lr = 0x82EF9C9C;
	sub_82EFAB58(ctx, base);
	// 82EF9C9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EF9CA0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EF9CA4: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EF9CA8: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82EF9CAC: 4800135D  bl 0x82efb008
	ctx.lr = 0x82EF9CB0;
	sub_82EFB008(ctx, base);
	// 82EF9CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9CB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9CB8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF9CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CC0: 4BFFFDF9  bl 0x82ef9ab8
	ctx.lr = 0x82EF9CC4;
	sub_82EF9AB8(ctx, base);
	// 82EF9CC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9CC8: 48001DD9  bl 0x82efbaa0
	ctx.lr = 0x82EF9CCC;
	sub_82EFBAA0(ctx, base);
	// 82EF9CCC: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9CD4: 419A0014  beq cr6, 0x82ef9ce8
	if ctx.cr[6].eq {
	pc = 0x82EF9CE8; continue 'dispatch;
	}
	// 82EF9CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CDC: 4BFFFC95  bl 0x82ef9970
	ctx.lr = 0x82EF9CE0;
	sub_82EF9970(ctx, base);
	// 82EF9CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9CE4: 4BFFFD65  bl 0x82ef9a48
	ctx.lr = 0x82EF9CE8;
	sub_82EF9A48(ctx, base);
	// 82EF9CE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9CF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


