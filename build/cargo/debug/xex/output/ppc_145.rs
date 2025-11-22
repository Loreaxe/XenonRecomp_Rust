pub fn sub_83004FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83004FF0 size=128
	// 83004FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83004FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83004FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83004FFC: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005000: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83005004: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 83005008: 4BFFE3D9  bl 0x830033e0
	ctx.lr = 0x8300500C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 8300500C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83005010: 419A0034  beq cr6, 0x83005044
	if ctx.cr[6].eq {
	pc = 0x83005044; continue 'dispatch;
	}
	// 83005014: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83005018: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 8300501C: 4BFFE3C5  bl 0x830033e0
	ctx.lr = 0x83005020;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 83005020: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83005024: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005028: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300502C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83005030: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005034: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300503C: 4E800421  bctrl
	ctx.lr = 0x83005040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83005040: 4800001C  b 0x8300505c
	pc = 0x8300505C; continue 'dispatch;
	// 83005044: 3D6020FF  lis r11, 0x20ff
	ctx.r[11].s64 = 553582592;
	// 83005048: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 8300504C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83005050: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005054: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005058: 4B1EFE61  bl 0x821f4eb8
	ctx.lr = 0x8300505C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F4EB8);
	// 8300505C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005068: 4E800020  blr
	return;
}

pub fn sub_83005070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005070 size=184
	// 83005070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300507C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005080: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83005084: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 83005088: 4BFFE359  bl 0x830033e0
	ctx.lr = 0x8300508C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 8300508C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83005090: 419A0034  beq cr6, 0x830050c4
	if ctx.cr[6].eq {
	pc = 0x830050C4; continue 'dispatch;
	}
	// 83005094: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83005098: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 8300509C: 4BFFE345  bl 0x830033e0
	ctx.lr = 0x830050A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 830050A0: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830050A4: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830050A8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830050AC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830050B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830050B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830050B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830050BC: 4E800421  bctrl
	ctx.lr = 0x830050C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830050C0: 4800001C  b 0x830050dc
	pc = 0x830050DC; continue 'dispatch;
	// 830050C4: 3D6020FF  lis r11, 0x20ff
	ctx.r[11].s64 = 553582592;
	// 830050C8: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 830050CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830050D0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830050D4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830050D8: 4B1F0F71  bl 0x821f6048
	ctx.lr = 0x830050DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F6048);
	// 830050DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830050E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830050E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830050E8: 4E800020  blr
	return;
}

pub fn sub_83005128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005128 size=72
	// 83005128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005134: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005138: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 8300513C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005140: 48000031  bl 0x83005170
	ctx.lr = 0x83005144;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005170);
	// 83005144: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005148: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8300514C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005150: 419A000C  beq cr6, 0x8300515c
	if ctx.cr[6].eq {
	pc = 0x8300515C; continue 'dispatch;
	}
	// 83005154: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005158: 4B840659  bl 0x828457b0
	ctx.lr = 0x8300515C;
	crate::recompiler::externs::call(&mut ctx, base, 0x828457B0);
	// 8300515C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300516C: 4E800020  blr
	return;
}

pub fn sub_83005170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005170 size=112
	// 83005170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300517C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005180: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83005184: 396BD5A0  addi r11, r11, -0x2a60
	ctx.r[11].s64 = ctx.r[11].s64 + -10848;
	// 83005188: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300518C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005190: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005194: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300519C: 419A002C  beq cr6, 0x830051c8
	if ctx.cr[6].eq {
	pc = 0x830051C8; continue 'dispatch;
	}
	// 830051A0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830051A4: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 830051A8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830051AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830051B0: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830051B4: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830051B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830051BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830051C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830051C4: 4E800421  bctrl
	ctx.lr = 0x830051C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830051C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830051CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830051D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830051D4: 4E800020  blr
	return;
	// 830051D8: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 830051DC: 8204D5B0  lwz r16, -0x2a50(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-10832 as u32) ) } as u64;
}

pub fn sub_830051E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830051E0 size=2060
	// 830051E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830051E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830051E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830051EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830051F0: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830051F4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830051F8: 907F00E4  stw r3, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[3].u32 ) };
	// 830051FC: 909F00EC  stw r4, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 83005200: F8BF00F0  std r5, 0xf0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[5].u64 ) };
	// 83005204: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83005208: 396BF240  addi r11, r11, -0xdc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3520;
	// 8300520C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005210: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83005214: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83005218: 4BFFE1C9  bl 0x830033e0
	ctx.lr = 0x8300521C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 8300521C: 4800080D  bl 0x83005a28
	ctx.lr = 0x83005220;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005220: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005228: 419A0038  beq cr6, 0x83005260
	if ctx.cr[6].eq {
	pc = 0x83005260; continue 'dispatch;
	}
	// 8300522C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83005230: 388B16C0  addi r4, r11, 0x16c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5824;
	// 83005234: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83005238: 4B2ECD09  bl 0x822f1f40
	ctx.lr = 0x8300523C;
	crate::recompiler::externs::call(&mut ctx, base, 0x822F1F40);
	// 8300523C: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 83005240: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83005244: 4B2ECB2D  bl 0x822f1d70
	ctx.lr = 0x83005248;
	crate::recompiler::externs::call(&mut ctx, base, 0x822F1D70);
	// 83005248: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 8300524C: 388BB5E0  addi r4, r11, -0x4a20
	ctx.r[4].s64 = ctx.r[11].s64 + -18976;
	// 83005250: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83005254: 4BCA7FAD  bl 0x82cad200
	ctx.lr = 0x83005258;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAD200);
	// 83005258: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300525C: 4B16C5B5  bl 0x82171810
	ctx.lr = 0x83005260;
	crate::recompiler::externs::call(&mut ctx, base, 0x82171810);
	// 83005260: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83005264: 4BFFE17D  bl 0x830033e0
	ctx.lr = 0x83005268;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 83005268: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 8300526C: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83005270: 48000B39  bl 0x83005da8
	ctx.lr = 0x83005274;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005DA8);
	// 83005274: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005278: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8300527C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005280: 4BFFF929  bl 0x83004ba8
	ctx.lr = 0x83005284;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005284: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005288: 480007A1  bl 0x83005a28
	ctx.lr = 0x8300528C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 8300528C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005294: 419A0018  beq cr6, 0x830052ac
	if ctx.cr[6].eq {
	pc = 0x830052AC; continue 'dispatch;
	}
	// 83005298: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300529C: 4BEE329D  bl 0x82ee8538
	ctx.lr = 0x830052A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830052A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830052A4: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830052A8: 48000050  b 0x830052f8
	pc = 0x830052F8; continue 'dispatch;
	// 830052AC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830052B0: 4BEE3289  bl 0x82ee8538
	ctx.lr = 0x830052B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830052B4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830052B8: 48000771  bl 0x83005a28
	ctx.lr = 0x830052BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 830052BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830052C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830052C4: 419A0018  beq cr6, 0x830052dc
	if ctx.cr[6].eq {
	pc = 0x830052DC; continue 'dispatch;
	}
	// 830052C8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830052CC: 4BFFF8DD  bl 0x83004ba8
	ctx.lr = 0x830052D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830052D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830052D4: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830052D8: 48000020  b 0x830052f8
	pc = 0x830052F8; continue 'dispatch;
	// 830052DC: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 830052E0: 4BFFE101  bl 0x830033e0
	ctx.lr = 0x830052E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830033E0);
	// 830052E4: 907F0080  stw r3, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 830052E8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830052EC: 4BEE324D  bl 0x82ee8538
	ctx.lr = 0x830052F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830052F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830052F4: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830052F8: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830052FC: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005300: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005304: 409A014C  bne cr6, 0x83005450
	if !ctx.cr[6].eq {
	pc = 0x83005450; continue 'dispatch;
	}
	// 83005308: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300530C: 4BFFFAB5  bl 0x83004dc0
	ctx.lr = 0x83005310;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005314: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83005318: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300531C: 4800070D  bl 0x83005a28
	ctx.lr = 0x83005320;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005320: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005328: 409A0014  bne cr6, 0x8300533c
	if !ctx.cr[6].eq {
	pc = 0x8300533C; continue 'dispatch;
	}
	// 8300532C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005330: 4BFFFA91  bl 0x83004dc0
	ctx.lr = 0x83005334;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005334: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005338: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300533C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005340: 4BFFF049  bl 0x83004388
	ctx.lr = 0x83005344;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005348: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300534C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005350: 409A0018  bne cr6, 0x83005368
	if !ctx.cr[6].eq {
	pc = 0x83005368; continue 'dispatch;
	}
	// 83005354: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005358: 4BFFF031  bl 0x83004388
	ctx.lr = 0x8300535C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 8300535C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005360: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005364: 48000040  b 0x830053a4
	pc = 0x830053A4; continue 'dispatch;
	// 83005368: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300536C: 4BFFF83D  bl 0x83004ba8
	ctx.lr = 0x83005370;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005370: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005374: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005378: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300537C: 409A0018  bne cr6, 0x83005394
	if !ctx.cr[6].eq {
	pc = 0x83005394; continue 'dispatch;
	}
	// 83005380: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005384: 4BFFF825  bl 0x83004ba8
	ctx.lr = 0x83005388;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005388: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300538C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005390: 48000014  b 0x830053a4
	pc = 0x830053A4; continue 'dispatch;
	// 83005394: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005398: 4BEE31A1  bl 0x82ee8538
	ctx.lr = 0x8300539C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 8300539C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830053A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830053A4: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830053A8: 4BFFF999  bl 0x83004d40
	ctx.lr = 0x830053AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004D40);
	// 830053AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830053B0: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830053B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830053B8: 409A0040  bne cr6, 0x830053f8
	if !ctx.cr[6].eq {
	pc = 0x830053F8; continue 'dispatch;
	}
	// 830053BC: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830053C0: 48000669  bl 0x83005a28
	ctx.lr = 0x830053C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 830053C4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830053C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830053CC: 419A0010  beq cr6, 0x830053dc
	if ctx.cr[6].eq {
	pc = 0x830053DC; continue 'dispatch;
	}
	// 830053D0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830053D4: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 830053D8: 48000010  b 0x830053e8
	pc = 0x830053E8; continue 'dispatch;
	// 830053DC: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830053E0: 480007F9  bl 0x83005bd8
	ctx.lr = 0x830053E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005BD8);
	// 830053E4: 907F00B0  stw r3, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 830053E8: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830053EC: 4BFFF955  bl 0x83004d40
	ctx.lr = 0x830053F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004D40);
	// 830053F0: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830053F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830053F8: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830053FC: 4BFFEF5D  bl 0x83004358
	ctx.lr = 0x83005400;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004358);
	// 83005400: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005404: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005408: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300540C: 409A0040  bne cr6, 0x8300544c
	if !ctx.cr[6].eq {
	pc = 0x8300544C; continue 'dispatch;
	}
	// 83005410: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005414: 48000615  bl 0x83005a28
	ctx.lr = 0x83005418;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005418: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300541C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005420: 419A0010  beq cr6, 0x83005430
	if ctx.cr[6].eq {
	pc = 0x83005430; continue 'dispatch;
	}
	// 83005424: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005428: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8300542C: 48000010  b 0x8300543c
	pc = 0x8300543C; continue 'dispatch;
	// 83005430: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005434: 4800074D  bl 0x83005b80
	ctx.lr = 0x83005438;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005B80);
	// 83005438: 907F00B4  stw r3, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 8300543C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005440: 4BFFEF19  bl 0x83004358
	ctx.lr = 0x83005444;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004358);
	// 83005444: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83005448: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300544C: 4800018C  b 0x830055d8
	pc = 0x830055D8; continue 'dispatch;
	// 83005450: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005454: 4BFFF755  bl 0x83004ba8
	ctx.lr = 0x83005458;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005458: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300545C: 4BFFF965  bl 0x83004dc0
	ctx.lr = 0x83005460;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005460: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005464: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005468: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300546C: 4BFFF73D  bl 0x83004ba8
	ctx.lr = 0x83005470;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005474: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005478: 4BFFF731  bl 0x83004ba8
	ctx.lr = 0x8300547C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 8300547C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005480: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005484: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005488: 4BEE30B1  bl 0x82ee8538
	ctx.lr = 0x8300548C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 8300548C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005490: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005494: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005498: 409A0010  bne cr6, 0x830054a8
	if !ctx.cr[6].eq {
	pc = 0x830054A8; continue 'dispatch;
	}
	// 8300549C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830054A0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830054A4: 4800007C  b 0x83005520
	pc = 0x83005520; continue 'dispatch;
	// 830054A8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830054AC: 4BFFF915  bl 0x83004dc0
	ctx.lr = 0x830054B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830054B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830054B4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830054B8: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830054BC: 4800056D  bl 0x83005a28
	ctx.lr = 0x830054C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 830054C0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830054C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830054C8: 409A0014  bne cr6, 0x830054dc
	if !ctx.cr[6].eq {
	pc = 0x830054DC; continue 'dispatch;
	}
	// 830054CC: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830054D0: 4BFFF8F1  bl 0x83004dc0
	ctx.lr = 0x830054D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830054D4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830054D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830054DC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830054E0: 4BFFF6C9  bl 0x83004ba8
	ctx.lr = 0x830054E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830054E4: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830054E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830054EC: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830054F0: 4BEE3049  bl 0x82ee8538
	ctx.lr = 0x830054F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830054F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830054F8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830054FC: 4BEE303D  bl 0x82ee8538
	ctx.lr = 0x83005500;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005500: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005504: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005508: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300550C: 4BEE302D  bl 0x82ee8538
	ctx.lr = 0x83005510;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005510: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005514: 4BFFF8AD  bl 0x83004dc0
	ctx.lr = 0x83005518;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005518: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300551C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005520: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005524: 4BFFEE65  bl 0x83004388
	ctx.lr = 0x83005528;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005528: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300552C: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005530: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005534: 409A0018  bne cr6, 0x8300554c
	if !ctx.cr[6].eq {
	pc = 0x8300554C; continue 'dispatch;
	}
	// 83005538: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300553C: 4BFFEE4D  bl 0x83004388
	ctx.lr = 0x83005540;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005540: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005544: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005548: 48000058  b 0x830055a0
	pc = 0x830055A0; continue 'dispatch;
	// 8300554C: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005550: 4BFFF871  bl 0x83004dc0
	ctx.lr = 0x83005554;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005554: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005558: 4BFFF651  bl 0x83004ba8
	ctx.lr = 0x8300555C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 8300555C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005560: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005564: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005568: 409A0020  bne cr6, 0x83005588
	if !ctx.cr[6].eq {
	pc = 0x83005588; continue 'dispatch;
	}
	// 8300556C: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005570: 4BFFF851  bl 0x83004dc0
	ctx.lr = 0x83005574;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005574: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005578: 4BFFF631  bl 0x83004ba8
	ctx.lr = 0x8300557C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 8300557C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005580: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005584: 4800001C  b 0x830055a0
	pc = 0x830055A0; continue 'dispatch;
	// 83005588: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300558C: 4BFFF835  bl 0x83004dc0
	ctx.lr = 0x83005590;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005590: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005594: 4BEE2FA5  bl 0x82ee8538
	ctx.lr = 0x83005598;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005598: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300559C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830055A0: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830055A4: 4BFFF81D  bl 0x83004dc0
	ctx.lr = 0x830055A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830055A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830055AC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830055B0: 4BFFF811  bl 0x83004dc0
	ctx.lr = 0x830055B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830055B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830055B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830055BC: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830055C0: 48000459  bl 0x83005a18
	ctx.lr = 0x830055C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830055C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830055C8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830055CC: 4800044D  bl 0x83005a18
	ctx.lr = 0x830055D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830055D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830055D4: 4B2EC5AD  bl 0x822f1b80
	ctx.lr = 0x830055D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x822F1B80);
	// 830055D8: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830055DC: 4800043D  bl 0x83005a18
	ctx.lr = 0x830055E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830055E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830055E4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830055E8: 409A0384  bne cr6, 0x8300596c
	if !ctx.cr[6].eq {
	pc = 0x8300596C; continue 'dispatch;
	}
	// 830055EC: 48000014  b 0x83005600
	pc = 0x83005600; continue 'dispatch;
	// 830055F0: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830055F4: 4BFFF7CD  bl 0x83004dc0
	ctx.lr = 0x830055F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830055F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830055FC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83005600: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005604: 4BFFED85  bl 0x83004388
	ctx.lr = 0x83005608;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005608: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300560C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005610: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005614: 419A0348  beq cr6, 0x8300595c
	if ctx.cr[6].eq {
	pc = 0x8300595C; continue 'dispatch;
	}
	// 83005618: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300561C: 480003FD  bl 0x83005a18
	ctx.lr = 0x83005620;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005620: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005624: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83005628: 409A0334  bne cr6, 0x8300595c
	if !ctx.cr[6].eq {
	pc = 0x8300595C; continue 'dispatch;
	}
	// 8300562C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005630: 4BFFF579  bl 0x83004ba8
	ctx.lr = 0x83005634;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005634: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005638: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300563C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005640: 409A0190  bne cr6, 0x830057d0
	if !ctx.cr[6].eq {
	pc = 0x830057D0; continue 'dispatch;
	}
	// 83005644: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005648: 4BEE2EF1  bl 0x82ee8538
	ctx.lr = 0x8300564C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 8300564C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005650: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83005654: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005658: 480003C1  bl 0x83005a18
	ctx.lr = 0x8300565C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300565C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005664: 409A0040  bne cr6, 0x830056a4
	if !ctx.cr[6].eq {
	pc = 0x830056A4; continue 'dispatch;
	}
	// 83005668: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300566C: 480003AD  bl 0x83005a18
	ctx.lr = 0x83005670;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005670: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83005674: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005678: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300567C: 4800039D  bl 0x83005a18
	ctx.lr = 0x83005680;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83005684: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005688: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300568C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005690: 480003A9  bl 0x83005a38
	ctx.lr = 0x83005694;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A38);
	// 83005694: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005698: 4BEE2EA1  bl 0x82ee8538
	ctx.lr = 0x8300569C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 8300569C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056A0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830056A4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830056A8: 48000381  bl 0x83005a28
	ctx.lr = 0x830056AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 830056AC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830056B4: 419A0010  beq cr6, 0x830056c4
	if ctx.cr[6].eq {
	pc = 0x830056C4; continue 'dispatch;
	}
	// 830056B8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830056BC: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830056C0: 4800010C  b 0x830057cc
	pc = 0x830057CC; continue 'dispatch;
	// 830056C4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830056C8: 4BFFF4E1  bl 0x83004ba8
	ctx.lr = 0x830056CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830056CC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056D0: 48000349  bl 0x83005a18
	ctx.lr = 0x830056D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830056D4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830056DC: 409A003C  bne cr6, 0x83005718
	if !ctx.cr[6].eq {
	pc = 0x83005718; continue 'dispatch;
	}
	// 830056E0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830056E4: 4BEE2E55  bl 0x82ee8538
	ctx.lr = 0x830056E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830056E8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056EC: 4800032D  bl 0x83005a18
	ctx.lr = 0x830056F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830056F0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830056F4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830056F8: 409A0020  bne cr6, 0x83005718
	if !ctx.cr[6].eq {
	pc = 0x83005718; continue 'dispatch;
	}
	// 830056FC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005700: 48000319  bl 0x83005a18
	ctx.lr = 0x83005704;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83005708: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300570C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005710: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83005714: 480000B8  b 0x830057cc
	pc = 0x830057CC; continue 'dispatch;
	// 83005718: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300571C: 4BEE2E1D  bl 0x82ee8538
	ctx.lr = 0x83005720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005720: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005724: 480002F5  bl 0x83005a18
	ctx.lr = 0x83005728;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005728: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300572C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83005730: 409A0048  bne cr6, 0x83005778
	if !ctx.cr[6].eq {
	pc = 0x83005778; continue 'dispatch;
	}
	// 83005734: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005738: 4BFFF471  bl 0x83004ba8
	ctx.lr = 0x8300573C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 8300573C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005740: 480002D9  bl 0x83005a18
	ctx.lr = 0x83005744;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005744: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83005748: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300574C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005750: 480002C9  bl 0x83005a18
	ctx.lr = 0x83005754;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83005758: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300575C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005760: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005764: 480004CD  bl 0x83005c30
	ctx.lr = 0x83005768;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005C30);
	// 83005768: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300576C: 4BEE2DCD  bl 0x82ee8538
	ctx.lr = 0x83005770;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005770: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005774: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83005778: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300577C: 4800029D  bl 0x83005a18
	ctx.lr = 0x83005780;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005780: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005784: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005788: 48000291  bl 0x83005a18
	ctx.lr = 0x8300578C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300578C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005790: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005794: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005798: 48000281  bl 0x83005a18
	ctx.lr = 0x8300579C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300579C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830057A0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830057A4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830057A8: 4BEE2D91  bl 0x82ee8538
	ctx.lr = 0x830057AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830057AC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830057B0: 48000269  bl 0x83005a18
	ctx.lr = 0x830057B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830057B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830057B8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830057BC: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830057C0: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830057C4: 48000275  bl 0x83005a38
	ctx.lr = 0x830057C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A38);
	// 830057C8: 48000194  b 0x8300595c
	pc = 0x8300595C; continue 'dispatch;
	// 830057CC: 4800018C  b 0x83005958
	pc = 0x83005958; continue 'dispatch;
	// 830057D0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830057D4: 4BFFF3D5  bl 0x83004ba8
	ctx.lr = 0x830057D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830057D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830057DC: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830057E0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830057E4: 48000235  bl 0x83005a18
	ctx.lr = 0x830057E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830057E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830057EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830057F0: 409A0040  bne cr6, 0x83005830
	if !ctx.cr[6].eq {
	pc = 0x83005830; continue 'dispatch;
	}
	// 830057F4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830057F8: 48000221  bl 0x83005a18
	ctx.lr = 0x830057FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830057FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83005800: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005804: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005808: 48000211  bl 0x83005a18
	ctx.lr = 0x8300580C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300580C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83005810: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005814: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005818: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300581C: 48000415  bl 0x83005c30
	ctx.lr = 0x83005820;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005C30);
	// 83005820: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005824: 4BFFF385  bl 0x83004ba8
	ctx.lr = 0x83005828;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005828: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300582C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83005830: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005834: 480001F5  bl 0x83005a28
	ctx.lr = 0x83005838;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005838: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300583C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005840: 419A0010  beq cr6, 0x83005850
	if ctx.cr[6].eq {
	pc = 0x83005850; continue 'dispatch;
	}
	// 83005844: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005848: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8300584C: 4800010C  b 0x83005958
	pc = 0x83005958; continue 'dispatch;
	// 83005850: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005854: 4BEE2CE5  bl 0x82ee8538
	ctx.lr = 0x83005858;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005858: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300585C: 480001BD  bl 0x83005a18
	ctx.lr = 0x83005860;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005860: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005864: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83005868: 409A003C  bne cr6, 0x830058a4
	if !ctx.cr[6].eq {
	pc = 0x830058A4; continue 'dispatch;
	}
	// 8300586C: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005870: 4BFFF339  bl 0x83004ba8
	ctx.lr = 0x83005874;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005874: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005878: 480001A1  bl 0x83005a18
	ctx.lr = 0x8300587C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300587C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005880: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83005884: 409A0020  bne cr6, 0x830058a4
	if !ctx.cr[6].eq {
	pc = 0x830058A4; continue 'dispatch;
	}
	// 83005888: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300588C: 4800018D  bl 0x83005a18
	ctx.lr = 0x83005890;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83005894: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005898: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300589C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830058A0: 480000B8  b 0x83005958
	pc = 0x83005958; continue 'dispatch;
	// 830058A4: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830058A8: 4BFFF301  bl 0x83004ba8
	ctx.lr = 0x830058AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830058AC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830058B0: 48000169  bl 0x83005a18
	ctx.lr = 0x830058B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830058B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830058B8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830058BC: 409A0048  bne cr6, 0x83005904
	if !ctx.cr[6].eq {
	pc = 0x83005904; continue 'dispatch;
	}
	// 830058C0: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830058C4: 4BEE2C75  bl 0x82ee8538
	ctx.lr = 0x830058C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 830058C8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830058CC: 4800014D  bl 0x83005a18
	ctx.lr = 0x830058D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830058D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830058D4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830058D8: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830058DC: 4800013D  bl 0x83005a18
	ctx.lr = 0x830058E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 830058E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830058E4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830058E8: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 830058EC: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830058F0: 48000149  bl 0x83005a38
	ctx.lr = 0x830058F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A38);
	// 830058F4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830058F8: 4BFFF2B1  bl 0x83004ba8
	ctx.lr = 0x830058FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830058FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005900: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83005904: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005908: 48000111  bl 0x83005a18
	ctx.lr = 0x8300590C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300590C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83005910: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005914: 48000105  bl 0x83005a18
	ctx.lr = 0x83005918;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005918: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300591C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005920: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005924: 480000F5  bl 0x83005a18
	ctx.lr = 0x83005928;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005928: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300592C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005930: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83005934: 4BFFF275  bl 0x83004ba8
	ctx.lr = 0x83005938;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005938: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300593C: 480000DD  bl 0x83005a18
	ctx.lr = 0x83005940;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005940: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83005944: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83005948: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300594C: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005950: 480002E1  bl 0x83005c30
	ctx.lr = 0x83005954;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005C30);
	// 83005954: 48000008  b 0x8300595c
	pc = 0x8300595C; continue 'dispatch;
	// 83005958: 4BFFFC98  b 0x830055f0
	pc = 0x830055F0; continue 'dispatch;
	// 8300595C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005960: 480000B9  bl 0x83005a18
	ctx.lr = 0x83005964;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 83005964: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83005968: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300596C: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005970: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005974: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 83005978: 4BFFEA41  bl 0x830043b8
	ctx.lr = 0x8300597C;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043B8);
	// 8300597C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83005980: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83005984: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005988: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8300598C: 480003ED  bl 0x83005d78
	ctx.lr = 0x83005990;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005D78);
	// 83005990: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 83005994: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83005998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300599C: 40990018  ble cr6, 0x830059b4
	if !ctx.cr[6].gt {
	pc = 0x830059B4; continue 'dispatch;
	}
	// 830059A0: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830059A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830059A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830059AC: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 830059B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830059B4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830059B8: 815F00F0  lwz r10, 0xf0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 830059BC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830059C0: 815F00F4  lwz r10, 0xf4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 830059C4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830059C8: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 830059CC: 48004245  bl 0x83009c10
	ctx.lr = 0x830059D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009C10);
	// 830059D0: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830059D4: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830059D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830059DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830059E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830059E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830059E8: 4E800020  blr
	return;
}

pub fn sub_830059EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830059EC size=76
	// 830059EC: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830059F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830059F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830059F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830059FC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83005A00: 4B16BE11  bl 0x82171810
	ctx.lr = 0x83005A04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82171810);
	// 83005A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005A10: 4E800020  blr
	return;
}

pub fn sub_83005A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005A38 size=328
	// 83005A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005A44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005A48: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 83005A4C: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83005A50: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005A54: 4BEE2AE5  bl 0x82ee8538
	ctx.lr = 0x83005A58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005A58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83005A60: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005A64: 4BFFF145  bl 0x83004ba8
	ctx.lr = 0x83005A68;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005A6C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005A70: 4BEE2AC9  bl 0x82ee8538
	ctx.lr = 0x83005A74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005A74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005A7C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005A80: 4BFFF129  bl 0x83004ba8
	ctx.lr = 0x83005A84;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005A84: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A88: 4BFFFFA1  bl 0x83005a28
	ctx.lr = 0x83005A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005A8C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005A94: 409A001C  bne cr6, 0x83005ab0
	if !ctx.cr[6].eq {
	pc = 0x83005AB0; continue 'dispatch;
	}
	// 83005A98: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005A9C: 4BFFF10D  bl 0x83004ba8
	ctx.lr = 0x83005AA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005AA0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005AA4: 4BFFF31D  bl 0x83004dc0
	ctx.lr = 0x83005AA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005AA8: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005AAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005AB0: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005AB4: 4BFFF30D  bl 0x83004dc0
	ctx.lr = 0x83005AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005ABC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005AC0: 4BFFF301  bl 0x83004dc0
	ctx.lr = 0x83005AC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005AC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005AC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005ACC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005AD0: 4BFFE8B9  bl 0x83004388
	ctx.lr = 0x83005AD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005AD4: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005AD8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005ADC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005AE0: 409A0018  bne cr6, 0x83005af8
	if !ctx.cr[6].eq {
	pc = 0x83005AF8; continue 'dispatch;
	}
	// 83005AE4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005AE8: 4BFFE8A1  bl 0x83004388
	ctx.lr = 0x83005AEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005AEC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005AF0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005AF4: 48000058  b 0x83005b4c
	pc = 0x83005B4C; continue 'dispatch;
	// 83005AF8: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005AFC: 4BFFF2C5  bl 0x83004dc0
	ctx.lr = 0x83005B00;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005B00: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B04: 4BFFF0A5  bl 0x83004ba8
	ctx.lr = 0x83005B08;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005B08: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005B0C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005B14: 409A0020  bne cr6, 0x83005b34
	if !ctx.cr[6].eq {
	pc = 0x83005B34; continue 'dispatch;
	}
	// 83005B18: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005B1C: 4BFFF2A5  bl 0x83004dc0
	ctx.lr = 0x83005B20;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005B20: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B24: 4BFFF085  bl 0x83004ba8
	ctx.lr = 0x83005B28;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005B28: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005B2C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005B30: 4800001C  b 0x83005b4c
	pc = 0x83005B4C; continue 'dispatch;
	// 83005B34: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005B38: 4BFFF289  bl 0x83004dc0
	ctx.lr = 0x83005B3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005B3C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B40: 4BEE29F9  bl 0x82ee8538
	ctx.lr = 0x83005B44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005B44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005B48: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005B4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005B50: 4BFFF059  bl 0x83004ba8
	ctx.lr = 0x83005B54;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005B54: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005B58: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005B5C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005B60: 4BFFF261  bl 0x83004dc0
	ctx.lr = 0x83005B64;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005B64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005B68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005B6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83005B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005B7C: 4E800020  blr
	return;
}

pub fn sub_83005B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005B80 size=88
	// 83005B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005B8C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005B90: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005B94: 4BEE29A5  bl 0x82ee8538
	ctx.lr = 0x83005B98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005B98: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005B9C: 4BFFFE8D  bl 0x83005a28
	ctx.lr = 0x83005BA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005BA0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005BA8: 409A0018  bne cr6, 0x83005bc0
	if !ctx.cr[6].eq {
	pc = 0x83005BC0; continue 'dispatch;
	}
	// 83005BAC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005BB0: 4BEE2989  bl 0x82ee8538
	ctx.lr = 0x83005BB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005BB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83005BBC: 4BFFFFD4  b 0x83005b90
	pc = 0x83005B90; continue 'dispatch;
	// 83005BC0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005BC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005BD0: 4E800020  blr
	return;
}

pub fn sub_83005BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005BD8 size=88
	// 83005BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005BE4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005BE8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005BEC: 4BFFEFBD  bl 0x83004ba8
	ctx.lr = 0x83005BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005BF0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005BF4: 4BFFFE35  bl 0x83005a28
	ctx.lr = 0x83005BF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005BF8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005C00: 409A0018  bne cr6, 0x83005c18
	if !ctx.cr[6].eq {
	pc = 0x83005C18; continue 'dispatch;
	}
	// 83005C04: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005C08: 4BFFEFA1  bl 0x83004ba8
	ctx.lr = 0x83005C0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83005C14: 4BFFFFD4  b 0x83005be8
	pc = 0x83005BE8; continue 'dispatch;
	// 83005C18: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005C1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005C28: 4E800020  blr
	return;
}

pub fn sub_83005C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005C30 size=328
	// 83005C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005C3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005C40: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 83005C44: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83005C48: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005C4C: 4BFFEF5D  bl 0x83004ba8
	ctx.lr = 0x83005C50;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83005C58: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005C5C: 4BEE28DD  bl 0x82ee8538
	ctx.lr = 0x83005C60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005C64: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005C68: 4BFFEF41  bl 0x83004ba8
	ctx.lr = 0x83005C6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005C6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C70: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005C74: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005C78: 4BEE28C1  bl 0x82ee8538
	ctx.lr = 0x83005C7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005C7C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C80: 4BFFFDA9  bl 0x83005a28
	ctx.lr = 0x83005C84;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005C84: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005C8C: 409A001C  bne cr6, 0x83005ca8
	if !ctx.cr[6].eq {
	pc = 0x83005CA8; continue 'dispatch;
	}
	// 83005C90: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005C94: 4BEE28A5  bl 0x82ee8538
	ctx.lr = 0x83005C98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005C98: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005C9C: 4BFFF125  bl 0x83004dc0
	ctx.lr = 0x83005CA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005CA0: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005CA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005CA8: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005CAC: 4BFFF115  bl 0x83004dc0
	ctx.lr = 0x83005CB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83005CB4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005CB8: 4BFFF109  bl 0x83004dc0
	ctx.lr = 0x83005CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005CBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005CC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005CC4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005CC8: 4BFFE6C1  bl 0x83004388
	ctx.lr = 0x83005CCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005CCC: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005CD0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005CD4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005CD8: 409A0018  bne cr6, 0x83005cf0
	if !ctx.cr[6].eq {
	pc = 0x83005CF0; continue 'dispatch;
	}
	// 83005CDC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005CE0: 4BFFE6A9  bl 0x83004388
	ctx.lr = 0x83005CE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83005CE4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005CE8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005CEC: 48000058  b 0x83005d44
	pc = 0x83005D44; continue 'dispatch;
	// 83005CF0: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005CF4: 4BFFF0CD  bl 0x83004dc0
	ctx.lr = 0x83005CF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005CF8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005CFC: 4BEE283D  bl 0x82ee8538
	ctx.lr = 0x83005D00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005D00: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005D04: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005D08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005D0C: 409A0020  bne cr6, 0x83005d2c
	if !ctx.cr[6].eq {
	pc = 0x83005D2C; continue 'dispatch;
	}
	// 83005D10: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005D14: 4BFFF0AD  bl 0x83004dc0
	ctx.lr = 0x83005D18;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005D18: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005D1C: 4BEE281D  bl 0x82ee8538
	ctx.lr = 0x83005D20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005D20: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005D24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005D28: 4800001C  b 0x83005d44
	pc = 0x83005D44; continue 'dispatch;
	// 83005D2C: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005D30: 4BFFF091  bl 0x83004dc0
	ctx.lr = 0x83005D34;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005D34: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005D38: 4BFFEE71  bl 0x83004ba8
	ctx.lr = 0x83005D3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005D3C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005D40: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005D44: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005D48: 4BEE27F1  bl 0x82ee8538
	ctx.lr = 0x83005D4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005D4C: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005D50: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005D54: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83005D58: 4BFFF069  bl 0x83004dc0
	ctx.lr = 0x83005D5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005D5C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005D60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83005D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83005D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005D70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005D74: 4E800020  blr
	return;
}

pub fn sub_83005D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005D78 size=48
	// 83005D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005D84: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005D88: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83005D8C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83005D90: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005D94: 4BFFF2DD  bl 0x83005070
	ctx.lr = 0x83005D98;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005070);
	// 83005D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005DA4: 4E800020  blr
	return;
}

pub fn sub_83005DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005DA8 size=48
	// 83005DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005DB4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005DB8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005DBC: 4800001D  bl 0x83005dd8
	ctx.lr = 0x83005DC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005DD8);
	// 83005DC0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005DD0: 4E800020  blr
	return;
}

pub fn sub_83005DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005DD8 size=48
	// 83005DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005DE4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005DE8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005DEC: 48000055  bl 0x83005e40
	ctx.lr = 0x83005DF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005E40);
	// 83005DF0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005DF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005E00: 4E800020  blr
	return;
}

pub fn sub_83005E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E08 size=56
	// 83005E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E14: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005E18: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83005E1C: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83005E20: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005E24: 1C6B0018  mulli r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 * 24;
	// 83005E28: 4BFFF1C9  bl 0x83004ff0
	ctx.lr = 0x83005E2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004FF0);
	// 83005E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005E38: 4E800020  blr
	return;
}

pub fn sub_83005E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005E40 size=256
	// 83005E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005E4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005E50: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 83005E54: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005E60: 409A0008  bne cr6, 0x83005e68
	if !ctx.cr[6].eq {
	pc = 0x83005E68; continue 'dispatch;
	}
	// 83005E64: 0FE00016  twui r0, 0x16
	// 83005E68: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E6C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005E70: 4BFFFBB9  bl 0x83005a28
	ctx.lr = 0x83005E74;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005E74: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005E7C: 419A000C  beq cr6, 0x83005e88
	if ctx.cr[6].eq {
	pc = 0x83005E88; continue 'dispatch;
	}
	// 83005E80: 0FE00016  twui r0, 0x16
	// 83005E84: 480000A4  b 0x83005f28
	pc = 0x83005F28; continue 'dispatch;
	// 83005E88: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005E8C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005E90: 4BEE26A9  bl 0x82ee8538
	ctx.lr = 0x83005E94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005E94: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005E98: 4BFFFB91  bl 0x83005a28
	ctx.lr = 0x83005E9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005E9C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005EA4: 409A0024  bne cr6, 0x83005ec8
	if !ctx.cr[6].eq {
	pc = 0x83005EC8; continue 'dispatch;
	}
	// 83005EA8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005EAC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005EB0: 4BEE2689  bl 0x82ee8538
	ctx.lr = 0x83005EB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005EB4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005EB8: 4BFFFD21  bl 0x83005bd8
	ctx.lr = 0x83005EBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005BD8);
	// 83005EBC: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005EC0: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83005EC4: 48000064  b 0x83005f28
	pc = 0x83005F28; continue 'dispatch;
	// 83005EC8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005ECC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005ED0: 4BFFEEF1  bl 0x83004dc0
	ctx.lr = 0x83005ED4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83005ED4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005ED8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83005EDC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005EE0: 4BFFFB49  bl 0x83005a28
	ctx.lr = 0x83005EE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83005EE4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83005EEC: 409A0030  bne cr6, 0x83005f1c
	if !ctx.cr[6].eq {
	pc = 0x83005F1C; continue 'dispatch;
	}
	// 83005EF0: 83E10084  lwz r31, 0x84(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005EF4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005EF8: 4BEE2641  bl 0x82ee8538
	ctx.lr = 0x83005EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83005EFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83005F00: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83005F04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83005F08: 409A0014  bne cr6, 0x83005f1c
	if !ctx.cr[6].eq {
	pc = 0x83005F1C; continue 'dispatch;
	}
	// 83005F0C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005F10: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005F14: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83005F18: 4BFFFFB0  b 0x83005ec8
	pc = 0x83005EC8; continue 'dispatch;
	// 83005F1C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005F20: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83005F24: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83005F28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83005F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83005F38: 4E800020  blr
	return;
}

pub fn sub_83005F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005F40 size=64
	// 83005F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005F4C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005F50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83005F54: 4BFFEC55  bl 0x83004ba8
	ctx.lr = 0x83005F58;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83005F58: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83005F5C: 38810051  addi r4, r1, 0x51
	ctx.r[4].s64 = ctx.r[1].s64 + 81;
	// 83005F60: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005F64: 4800001D  bl 0x83005f80
	ctx.lr = 0x83005F68;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005F80);
	// 83005F68: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005F78: 4E800020  blr
	return;
}

pub fn sub_83005F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005F80 size=88
	// 83005F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005F8C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005F90: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83005F94: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83005F98: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83005F9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83005FA0: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83005FA4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83005FA8: 4BEE77E9  bl 0x82eed790
	ctx.lr = 0x83005FAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EED790);
	// 83005FAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83005FB0: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83005FB4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005FB8: 4BFFE519  bl 0x830044d0
	ctx.lr = 0x83005FBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x830044D0);
	// 83005FBC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005FC0: 48000019  bl 0x83005fd8
	ctx.lr = 0x83005FC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005FD8);
	// 83005FC4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83005FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83005FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83005FD4: 4E800020  blr
	return;
}

pub fn sub_83005FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83005FD8 size=160
	// 83005FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83005FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83005FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83005FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83005FE8: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83005FEC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005FF0: 48000089  bl 0x83006078
	ctx.lr = 0x83005FF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006078);
	// 83005FF4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83005FF8: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83005FFC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006000: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006004: 4BFFFA25  bl 0x83005a28
	ctx.lr = 0x83006008;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83006008: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300600C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83006010: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006014: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006018: 4BFFE371  bl 0x83004388
	ctx.lr = 0x8300601C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 8300601C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006020: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83006024: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006028: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300602C: 4BFFED15  bl 0x83004d40
	ctx.lr = 0x83006030;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004D40);
	// 83006030: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006034: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83006038: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300603C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006040: 4BFFE319  bl 0x83004358
	ctx.lr = 0x83006044;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004358);
	// 83006044: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006048: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300604C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83006054: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83006058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006068: 4E800020  blr
	return;
}

pub fn sub_83006078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006078 size=300
	// 83006078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006080: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83006084: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83006088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300608C: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83006090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006094: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 83006098: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300609C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830060A0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830060A4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830060A8: 4BFFFD61  bl 0x83005e08
	ctx.lr = 0x830060AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005E08);
	// 830060AC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830060B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830060B4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830060B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830060BC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830060C0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830060C4: 4BFFEAE5  bl 0x83004ba8
	ctx.lr = 0x830060C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830060C8: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 830060CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830060D0: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 830060D4: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830060D8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830060DC: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 830060E0: 4BFFEAD9  bl 0x83004bb8
	ctx.lr = 0x830060E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BB8);
	// 830060E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830060E8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830060EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830060F0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830060F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830060F8: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830060FC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006100: 4BFFECC1  bl 0x83004dc0
	ctx.lr = 0x83006104;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 83006104: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83006108: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300610C: 38BF005C  addi r5, r31, 0x5c
	ctx.r[5].s64 = ctx.r[31].s64 + 92;
	// 83006110: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83006114: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006118: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8300611C: 4BFFEA9D  bl 0x83004bb8
	ctx.lr = 0x83006120;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BB8);
	// 83006120: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83006124: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83006128: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300612C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83006130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83006134: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83006138: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300613C: 4BEE23FD  bl 0x82ee8538
	ctx.lr = 0x83006140;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83006140: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 83006144: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83006148: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 8300614C: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83006150: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006154: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83006158: 4BFFEA61  bl 0x83004bb8
	ctx.lr = 0x8300615C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BB8);
	// 8300615C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83006160: 48000004  b 0x83006164
	pc = 0x83006164; continue 'dispatch;
	// 83006164: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006168: 4BFFF8B1  bl 0x83005a18
	ctx.lr = 0x8300616C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A18);
	// 8300616C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83006170: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83006174: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006178: 4BFFF8B1  bl 0x83005a28
	ctx.lr = 0x8300617C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 8300617C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83006180: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83006184: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006188: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300618C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83006198: 4E800020  blr
	return;
	// 8300619C: 82CAE348  lwz r22, -0x1cb8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7352 as u32) ) } as u64;
	// 830061A0: 8204D61C  lwz r16, -0x29e4(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-10724 as u32) ) } as u64;
}

pub fn sub_830061A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830061A4 size=148
	// 830061A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830061A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830061AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830061B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830061B4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830061B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830061BC: 4099001C  ble cr6, 0x830061d8
	if !ctx.cr[6].gt {
	pc = 0x830061D8; continue 'dispatch;
	}
	// 830061C0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830061C4: 4BFFEBFD  bl 0x83004dc0
	ctx.lr = 0x830061C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004DC0);
	// 830061C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830061CC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830061D0: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 830061D4: 4BFFE1E5  bl 0x830043b8
	ctx.lr = 0x830061D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043B8);
	// 830061D8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830061DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830061E0: 4099001C  ble cr6, 0x830061fc
	if !ctx.cr[6].gt {
	pc = 0x830061FC; continue 'dispatch;
	}
	// 830061E4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830061E8: 4BFFE9C1  bl 0x83004ba8
	ctx.lr = 0x830061EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 830061EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830061F0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830061F4: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 830061F8: 4BFFE1C1  bl 0x830043b8
	ctx.lr = 0x830061FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043B8);
	// 830061FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83006200: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006204: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006208: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8300620C: 4BFFFB6D  bl 0x83005d78
	ctx.lr = 0x83006210;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005D78);
	// 83006210: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83006214: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006218: 4BCA6FE9  bl 0x82cad200
	ctx.lr = 0x8300621C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAD200);
	// 8300621C: 3C608300  lis r3, -0x7d00
	ctx.r[3].s64 = -2097152000;
	// 83006220: 38636164  addi r3, r3, 0x6164
	ctx.r[3].s64 = ctx.r[3].s64 + 24932;
	// 83006224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300622C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006230: 4E800020  blr
	return;
}

pub fn sub_83006238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006238 size=40
	// 83006238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006244: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83006248: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300624C: 48000015  bl 0x83006260
	ctx.lr = 0x83006250;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006260);
	// 83006250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300625C: 4E800020  blr
	return;
}

pub fn sub_83006260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006260 size=40
	// 83006260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300626C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83006270: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006274: 48000015  bl 0x83006288
	ctx.lr = 0x83006278;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006288);
	// 83006278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300627C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006284: 4E800020  blr
	return;
}

pub fn sub_83006288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006288 size=12
	// 83006288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300628C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83006368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006368 size=232
	// 83006368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300636C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006374: 90610094  stw r3, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 83006378: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 8300637C: F8A100A0  std r5, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[5].u64 ) };
	// 83006380: F8C100A8  std r6, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[6].u64 ) };
	// 83006384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006388: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 8300638C: 4BFFE05D  bl 0x830043e8
	ctx.lr = 0x83006390;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043E8);
	// 83006390: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83006394: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 83006398: 4BFFE8C9  bl 0x83004c60
	ctx.lr = 0x8300639C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004C60);
	// 8300639C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830063A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830063A4: 419A0048  beq cr6, 0x830063ec
	if ctx.cr[6].eq {
	pc = 0x830063EC; continue 'dispatch;
	}
	// 830063A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830063AC: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 830063B0: 4BFFE3D9  bl 0x83004788
	ctx.lr = 0x830063B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004788);
	// 830063B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830063B8: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 830063BC: 4BFFE8A5  bl 0x83004c60
	ctx.lr = 0x830063C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004C60);
	// 830063C0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830063C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830063C8: 419A0024  beq cr6, 0x830063ec
	if ctx.cr[6].eq {
	pc = 0x830063EC; continue 'dispatch;
	}
	// 830063CC: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 830063D0: 48000081  bl 0x83006450
	ctx.lr = 0x830063D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006450);
	// 830063D4: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 830063D8: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 830063DC: 4BFFE00D  bl 0x830043e8
	ctx.lr = 0x830063E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043E8);
	// 830063E0: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 830063E4: 4800005C  b 0x83006440
	pc = 0x83006440; continue 'dispatch;
	// 830063E8: 48000058  b 0x83006440
	pc = 0x83006440; continue 'dispatch;
	// 830063EC: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 830063F0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830063F4: 4BFFE045  bl 0x83004438
	ctx.lr = 0x830063F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004438);
	// 830063F8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830063FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83006400: 419A0028  beq cr6, 0x83006428
	if ctx.cr[6].eq {
	pc = 0x83006428; continue 'dispatch;
	}
	// 83006404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006408: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300640C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 83006410: 480000C9  bl 0x830064d8
	ctx.lr = 0x83006414;
	crate::recompiler::externs::call(&mut ctx, base, 0x830064D8);
	// 83006414: E8A30000  ld r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 83006418: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8300641C: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 83006420: 4BFFEDC1  bl 0x830051e0
	ctx.lr = 0x83006424;
	crate::recompiler::externs::call(&mut ctx, base, 0x830051E0);
	// 83006424: 4BFFFFC8  b 0x830063ec
	pc = 0x830063EC; continue 'dispatch;
	// 83006428: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300642C: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 83006430: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83006434: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 83006438: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300643C: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83006440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83006444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300644C: 4E800020  blr
	return;
}

pub fn sub_83006450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006450 size=136
	// 83006450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300645C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006460: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83006464: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006468: 4BFFDF21  bl 0x83004388
	ctx.lr = 0x8300646C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 8300646C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006470: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006474: 480000BD  bl 0x83006530
	ctx.lr = 0x83006478;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006530);
	// 83006478: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300647C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006480: 4BFFDF09  bl 0x83004388
	ctx.lr = 0x83006484;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004388);
	// 83006484: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83006488: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300648C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83006494: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83006498: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300649C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830064A0: 4BFFE8A1  bl 0x83004d40
	ctx.lr = 0x830064A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004D40);
	// 830064A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830064A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830064AC: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830064B0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830064B4: 4BFFDEA5  bl 0x83004358
	ctx.lr = 0x830064B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004358);
	// 830064B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830064BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830064C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830064C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830064C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830064CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830064D0: 4E800020  blr
	return;
}

pub fn sub_830064D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830064D8 size=88
	// 830064D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830064DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830064E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830064E4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830064E8: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830064EC: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 830064F0: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830064F4: E96B0000  ld r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 830064F8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 830064FC: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83006500: 4BFFF8A9  bl 0x83005da8
	ctx.lr = 0x83006504;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005DA8);
	// 83006504: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006508: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300650C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83006510: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83006514: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83006518: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300651C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83006520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83006524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83006528: 4E800020  blr
	return;
}

pub fn sub_83006530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006530 size=264
	// 83006530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300653C: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83006540: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83006544: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83006548: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300654C: 4800000C  b 0x83006558
	pc = 0x83006558; continue 'dispatch;
	// 83006550: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83006554: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83006558: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300655C: 4BFFF4CD  bl 0x83005a28
	ctx.lr = 0x83006560;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005A28);
	// 83006560: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006564: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83006568: 409A0050  bne cr6, 0x830065b8
	if !ctx.cr[6].eq {
	pc = 0x830065B8; continue 'dispatch;
	}
	// 8300656C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83006570: 4BEE1FC9  bl 0x82ee8538
	ctx.lr = 0x83006574;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EE8538);
	// 83006574: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83006578: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8300657C: 4BFFFFB5  bl 0x83006530
	ctx.lr = 0x83006580;
	crate::recompiler::externs::call(&mut ctx, base, 0x83006530);
	// 83006580: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83006584: 4BFFE625  bl 0x83004ba8
	ctx.lr = 0x83006588;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004BA8);
	// 83006588: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300658C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83006590: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83006594: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83006598: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8300659C: 4BFFDE1D  bl 0x830043b8
	ctx.lr = 0x830065A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830043B8);
	// 830065A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830065A4: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 830065A8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830065AC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830065B0: 4BFFF7C9  bl 0x83005d78
	ctx.lr = 0x830065B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005D78);
	// 830065B4: 4BFFFF9C  b 0x83006550
	pc = 0x83006550; continue 'dispatch;
	// 830065B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830065BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830065C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830065C4: 4E800020  blr
	return;
}

pub fn sub_83006638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006638 size=296
	// 83006638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300663C: 4BCA2DBD  bl 0x82ca93f8
	ctx.lr = 0x83006640;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 83006640: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006644: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83006648: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 8300664C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83006650: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 83006654: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83006658: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300665C: 7F0B2214  add r24, r11, r4
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83006660: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83006664: 409A003C  bne cr6, 0x830066a0
	if !ctx.cr[6].eq {
	pc = 0x830066A0; continue 'dispatch;
	}
	// 83006668: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8300666C: 419A0034  beq cr6, 0x830066a0
	if ctx.cr[6].eq {
	pc = 0x830066A0; continue 'dispatch;
	}
	// 83006670: 4BCAB9E9  bl 0x82cb2058
	ctx.lr = 0x83006674;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83006674: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83006678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300667C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83006680: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83006684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006688: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300668C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006690: 4BCAB889  bl 0x82cb1f18
	ctx.lr = 0x83006694;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83006694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006698: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300669C: 4BCA2DAC  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
	// 830066A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830066A4: 419AFFCC  beq cr6, 0x83006670
	if ctx.cr[6].eq {
	pc = 0x83006670; continue 'dispatch;
	}
	// 830066A8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830066AC: 419AFFC4  beq cr6, 0x83006670
	if ctx.cr[6].eq {
	pc = 0x83006670; continue 'dispatch;
	}
	// 830066B0: 7F04C040  cmplw cr6, r4, r24
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[24].u32, &mut ctx.xer);
	// 830066B4: 4199FFE0  bgt cr6, 0x83006694
	if ctx.cr[6].gt {
	pc = 0x83006694; continue 'dispatch;
	}
	// 830066B8: 54BCF87F  rlwinm. r28, r5, 0x1f, 1, 0x1f
	ctx.r[28].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830066BC: 4182006C  beq 0x83006728
	if ctx.cr[0].eq {
	pc = 0x83006728; continue 'dispatch;
	}
	// 830066C0: 54BB07FF  clrlwi. r27, r5, 0x1f
	ctx.r[27].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830066C4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830066C8: 40820008  bne 0x830066d0
	if !ctx.cr[0].eq {
	pc = 0x830066D0; continue 'dispatch;
	}
	// 830066CC: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 830066D0: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 830066D4: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830066D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830066DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830066E0: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 830066E4: 4E800421  bctrl
	ctx.lr = 0x830066E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830066E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830066EC: 41820034  beq 0x83006720
	if ctx.cr[0].eq {
	pc = 0x83006720; continue 'dispatch;
	}
	// 830066F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830066F4: 40980018  bge cr6, 0x8300670c
	if !ctx.cr[6].lt {
	pc = 0x8300670C; continue 'dispatch;
	}
	// 830066F8: 7F1EF850  subf r24, r30, r31
	ctx.r[24].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 830066FC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83006700: 409A0010  bne cr6, 0x83006710
	if !ctx.cr[6].eq {
	pc = 0x83006710; continue 'dispatch;
	}
	// 83006704: 38BCFFFF  addi r5, r28, -1
	ctx.r[5].s64 = ctx.r[28].s64 + -1;
	// 83006708: 4800000C  b 0x83006714
	pc = 0x83006714; continue 'dispatch;
	// 8300670C: 7FBFF214  add r29, r31, r30
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83006710: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83006714: 7F1DC040  cmplw cr6, r29, r24
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83006718: 4099FFA0  ble cr6, 0x830066b8
	if !ctx.cr[6].gt {
	pc = 0x830066B8; continue 'dispatch;
	}
	// 8300671C: 4BFFFF78  b 0x83006694
	pc = 0x83006694; continue 'dispatch;
	// 83006720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83006724: 4BFFFF74  b 0x83006698
	pc = 0x83006698; continue 'dispatch;
	// 83006728: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8300672C: 419AFF68  beq cr6, 0x83006694
	if ctx.cr[6].eq {
	pc = 0x83006694; continue 'dispatch;
	}
	// 83006730: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83006734: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83006738: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 8300673C: 4E800421  bctrl
	ctx.lr = 0x83006740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83006740: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83006744: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006748: 4082FF50  bne 0x83006698
	if !ctx.cr[0].eq {
	pc = 0x83006698; continue 'dispatch;
	}
	// 8300674C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83006750: 4BFFFF48  b 0x83006698
	pc = 0x83006698; continue 'dispatch;
}

pub fn sub_83006760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006760 size=8
	// 83006760: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 83006764: 7DCB61CE  stvx v14, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[14].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[14].u8[first + i]); }
	}
}

pub fn sub_83006768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006768 size=8
	// 83006768: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 8300676C: 7DEB61CE  stvx v15, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[15].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[15].u8[first + i]); }
	}
}

pub fn sub_83006770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006770 size=8
	// 83006770: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 83006774: 7E0B61CE  stvx v16, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[16].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[16].u8[first + i]); }
	}
}

pub fn sub_83006778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006778 size=8
	// 83006778: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 8300677C: 7E2B61CE  stvx v17, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[17].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[17].u8[first + i]); }
	}
}

pub fn sub_83006780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006780 size=8
	// 83006780: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 83006784: 7E4B61CE  stvx v18, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[18].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[18].u8[first + i]); }
	}
}

pub fn sub_83006788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006788 size=8
	// 83006788: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 8300678C: 7E6B61CE  stvx v19, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[19].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[19].u8[first + i]); }
	}
}

pub fn sub_83006790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006790 size=8
	// 83006790: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 83006794: 7E8B61CE  stvx v20, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[20].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[20].u8[first + i]); }
	}
}

pub fn sub_83006798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006798 size=8
	// 83006798: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 8300679C: 7EAB61CE  stvx v21, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[21].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[21].u8[first + i]); }
	}
}

pub fn sub_830067A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067A0 size=8
	// 830067A0: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 830067A4: 7ECB61CE  stvx v22, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[22].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[22].u8[first + i]); }
	}
}

pub fn sub_830067A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067A8 size=8
	// 830067A8: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 830067AC: 7EEB61CE  stvx v23, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[23].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[23].u8[first + i]); }
	}
}

pub fn sub_830067B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067B0 size=8
	// 830067B0: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 830067B4: 7F0B61CE  stvx v24, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[24].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[24].u8[first + i]); }
	}
}

pub fn sub_830067B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067B8 size=8
	// 830067B8: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 830067BC: 7F2B61CE  stvx v25, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[25].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[25].u8[first + i]); }
	}
}

pub fn sub_830067C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067C0 size=8
	// 830067C0: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 830067C4: 7F4B61CE  stvx v26, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[26].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[26].u8[first + i]); }
	}
}

pub fn sub_830067C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067C8 size=8
	// 830067C8: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 830067CC: 7F6B61CE  stvx v27, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[27].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[27].u8[first + i]); }
	}
}

pub fn sub_830067D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067D0 size=8
	// 830067D0: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 830067D4: 7F8B61CE  stvx v28, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[28].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[28].u8[first + i]); }
	}
}

pub fn sub_830067D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067D8 size=8
	// 830067D8: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 830067DC: 7FAB61CE  stvx v29, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[29].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[29].u8[first + i]); }
	}
}

pub fn sub_830067E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067E0 size=8
	// 830067E0: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 830067E4: 7FCB61CE  stvx v30, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[30].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[30].u8[first + i]); }
	}
}

pub fn sub_830067E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067E8 size=12
	// 830067E8: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 830067EC: 7FEB61CE  stvx v31, r11, r12
	let addr = ctx.r[11].u32 + ctx.r[12].u32;
	let off = (addr & 0xFu32) as usize;
	let base_aligned = addr & !0xFu32;
	let first = 16usize - off;
	for i in 0..first {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add((off + i) as u32), ctx.v[31].u8[i]); }
	}
	for i in 0..off {
		unsafe { crate::rt::store_u8(base as *mut u8, base_aligned.wrapping_add(16).wrapping_add(i as u32), ctx.v[31].u8[first + i]); }
	}
	// 830067F0: 4E800020  blr
	return;
}

pub fn sub_830067F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067F4 size=8
	// 830067F4: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
}

pub fn sub_830067FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830067FC size=8
	// 830067FC: 3960FC10  li r11, -0x3f0
	ctx.r[11].s64 = -1008;
}

pub fn sub_83006804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006804 size=8
	// 83006804: 3960FC20  li r11, -0x3e0
	ctx.r[11].s64 = -992;
}

pub fn sub_8300680C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300680C size=8
	// 8300680C: 3960FC30  li r11, -0x3d0
	ctx.r[11].s64 = -976;
}

pub fn sub_83006814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006814 size=8
	// 83006814: 3960FC40  li r11, -0x3c0
	ctx.r[11].s64 = -960;
}

pub fn sub_8300681C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300681C size=8
	// 8300681C: 3960FC50  li r11, -0x3b0
	ctx.r[11].s64 = -944;
}

pub fn sub_83006824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006824 size=8
	// 83006824: 3960FC60  li r11, -0x3a0
	ctx.r[11].s64 = -928;
}

pub fn sub_8300682C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300682C size=8
	// 8300682C: 3960FC70  li r11, -0x390
	ctx.r[11].s64 = -912;
}

pub fn sub_83006834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006834 size=8
	// 83006834: 3960FC80  li r11, -0x380
	ctx.r[11].s64 = -896;
}

pub fn sub_8300683C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300683C size=8
	// 8300683C: 3960FC90  li r11, -0x370
	ctx.r[11].s64 = -880;
}

pub fn sub_83006844(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006844 size=8
	// 83006844: 3960FCA0  li r11, -0x360
	ctx.r[11].s64 = -864;
}

pub fn sub_8300684C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300684C size=8
	// 8300684C: 3960FCB0  li r11, -0x350
	ctx.r[11].s64 = -848;
}

pub fn sub_83006854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006854 size=8
	// 83006854: 3960FCC0  li r11, -0x340
	ctx.r[11].s64 = -832;
}

pub fn sub_8300685C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300685C size=8
	// 8300685C: 3960FCD0  li r11, -0x330
	ctx.r[11].s64 = -816;
}

pub fn sub_83006864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006864 size=8
	// 83006864: 3960FCE0  li r11, -0x320
	ctx.r[11].s64 = -800;
}

pub fn sub_8300686C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300686C size=8
	// 8300686C: 3960FCF0  li r11, -0x310
	ctx.r[11].s64 = -784;
}

pub fn sub_83006874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006874 size=8
	// 83006874: 3960FD00  li r11, -0x300
	ctx.r[11].s64 = -768;
}

pub fn sub_8300687C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300687C size=8
	// 8300687C: 3960FD10  li r11, -0x2f0
	ctx.r[11].s64 = -752;
}

pub fn sub_83006884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006884 size=8
	// 83006884: 3960FD20  li r11, -0x2e0
	ctx.r[11].s64 = -736;
}

pub fn sub_8300688C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300688C size=8
	// 8300688C: 3960FD30  li r11, -0x2d0
	ctx.r[11].s64 = -720;
}

pub fn sub_83006894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006894 size=8
	// 83006894: 3960FD40  li r11, -0x2c0
	ctx.r[11].s64 = -704;
}

pub fn sub_8300689C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300689C size=8
	// 8300689C: 3960FD50  li r11, -0x2b0
	ctx.r[11].s64 = -688;
}

pub fn sub_830068A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068A4 size=8
	// 830068A4: 3960FD60  li r11, -0x2a0
	ctx.r[11].s64 = -672;
}

pub fn sub_830068AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068AC size=8
	// 830068AC: 3960FD70  li r11, -0x290
	ctx.r[11].s64 = -656;
}

pub fn sub_830068B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068B4 size=8
	// 830068B4: 3960FD80  li r11, -0x280
	ctx.r[11].s64 = -640;
}

pub fn sub_830068BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068BC size=8
	// 830068BC: 3960FD90  li r11, -0x270
	ctx.r[11].s64 = -624;
}

pub fn sub_830068C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068C4 size=8
	// 830068C4: 3960FDA0  li r11, -0x260
	ctx.r[11].s64 = -608;
}

pub fn sub_830068CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068CC size=8
	// 830068CC: 3960FDB0  li r11, -0x250
	ctx.r[11].s64 = -592;
}

pub fn sub_830068D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068D4 size=8
	// 830068D4: 3960FDC0  li r11, -0x240
	ctx.r[11].s64 = -576;
}

pub fn sub_830068DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068DC size=8
	// 830068DC: 3960FDD0  li r11, -0x230
	ctx.r[11].s64 = -560;
}

pub fn sub_830068E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068E4 size=8
	// 830068E4: 3960FDE0  li r11, -0x220
	ctx.r[11].s64 = -544;
}

pub fn sub_830068EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068EC size=8
	// 830068EC: 3960FDF0  li r11, -0x210
	ctx.r[11].s64 = -528;
}

pub fn sub_830068F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068F4 size=8
	// 830068F4: 3960FE00  li r11, -0x200
	ctx.r[11].s64 = -512;
}

pub fn sub_830068FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830068FC size=8
	// 830068FC: 3960FE10  li r11, -0x1f0
	ctx.r[11].s64 = -496;
}

pub fn sub_83006904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006904 size=8
	// 83006904: 3960FE20  li r11, -0x1e0
	ctx.r[11].s64 = -480;
}

pub fn sub_8300690C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300690C size=8
	// 8300690C: 3960FE30  li r11, -0x1d0
	ctx.r[11].s64 = -464;
}

pub fn sub_83006914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006914 size=8
	// 83006914: 3960FE40  li r11, -0x1c0
	ctx.r[11].s64 = -448;
}

pub fn sub_8300691C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300691C size=8
	// 8300691C: 3960FE50  li r11, -0x1b0
	ctx.r[11].s64 = -432;
}

pub fn sub_83006924(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006924 size=8
	// 83006924: 3960FE60  li r11, -0x1a0
	ctx.r[11].s64 = -416;
}

pub fn sub_8300692C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300692C size=8
	// 8300692C: 3960FE70  li r11, -0x190
	ctx.r[11].s64 = -400;
}

pub fn sub_83006934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006934 size=8
	// 83006934: 3960FE80  li r11, -0x180
	ctx.r[11].s64 = -384;
}

pub fn sub_8300693C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300693C size=8
	// 8300693C: 3960FE90  li r11, -0x170
	ctx.r[11].s64 = -368;
}

pub fn sub_83006944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006944 size=8
	// 83006944: 3960FEA0  li r11, -0x160
	ctx.r[11].s64 = -352;
}

pub fn sub_8300694C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300694C size=8
	// 8300694C: 3960FEB0  li r11, -0x150
	ctx.r[11].s64 = -336;
}

pub fn sub_83006954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006954 size=8
	// 83006954: 3960FEC0  li r11, -0x140
	ctx.r[11].s64 = -320;
}

pub fn sub_8300695C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300695C size=8
	// 8300695C: 3960FED0  li r11, -0x130
	ctx.r[11].s64 = -304;
}

pub fn sub_83006964(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006964 size=8
	// 83006964: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
}

pub fn sub_8300696C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300696C size=8
	// 8300696C: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
}

pub fn sub_83006974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006974 size=8
	// 83006974: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
}

pub fn sub_8300697C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300697C size=8
	// 8300697C: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
}

pub fn sub_83006984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006984 size=8
	// 83006984: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
}

pub fn sub_8300698C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300698C size=8
	// 8300698C: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
}

pub fn sub_83006994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006994 size=8
	// 83006994: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
}

pub fn sub_8300699C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300699C size=8
	// 8300699C: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
}

pub fn sub_830069A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069A4 size=8
	// 830069A4: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
}

pub fn sub_830069AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069AC size=8
	// 830069AC: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
}

pub fn sub_830069B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069B4 size=8
	// 830069B4: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
}

pub fn sub_830069BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069BC size=8
	// 830069BC: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
}

pub fn sub_830069C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069C4 size=8
	// 830069C4: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
}

pub fn sub_830069CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069CC size=8
	// 830069CC: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
}

pub fn sub_830069D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069D4 size=8
	// 830069D4: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
}

pub fn sub_830069DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069DC size=8
	// 830069DC: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
}

pub fn sub_830069E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069E4 size=8
	// 830069E4: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
}

pub fn sub_830069EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830069EC size=12
	// 830069EC: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
}

pub fn sub_830069F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830069F8 size=8
	// 830069F8: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
	// 830069FC: 7DCB60CE  lvx v14, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[14] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A00 size=8
	// 83006A00: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
	// 83006A04: 7DEB60CE  lvx v15, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[15] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A08 size=8
	// 83006A08: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
	// 83006A0C: 7E0B60CE  lvx v16, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[16] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A10 size=8
	// 83006A10: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
	// 83006A14: 7E2B60CE  lvx v17, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[17] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A18 size=8
	// 83006A18: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
	// 83006A1C: 7E4B60CE  lvx v18, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[18] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A20 size=8
	// 83006A20: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
	// 83006A24: 7E6B60CE  lvx v19, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[19] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A28 size=8
	// 83006A28: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
	// 83006A2C: 7E8B60CE  lvx v20, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[20] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A30 size=8
	// 83006A30: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
	// 83006A34: 7EAB60CE  lvx v21, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[21] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A38 size=8
	// 83006A38: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
	// 83006A3C: 7ECB60CE  lvx v22, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[22] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A40 size=8
	// 83006A40: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
	// 83006A44: 7EEB60CE  lvx v23, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[23] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A48 size=8
	// 83006A48: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 83006A4C: 7F0B60CE  lvx v24, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[24] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A50 size=8
	// 83006A50: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
	// 83006A54: 7F2B60CE  lvx v25, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[25] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A58 size=8
	// 83006A58: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
	// 83006A5C: 7F4B60CE  lvx v26, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[26] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A60 size=8
	// 83006A60: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
	// 83006A64: 7F6B60CE  lvx v27, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[27] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A68 size=8
	// 83006A68: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
	// 83006A6C: 7F8B60CE  lvx v28, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[28] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A70 size=8
	// 83006A70: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
	// 83006A74: 7FAB60CE  lvx v29, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[29] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A78 size=8
	// 83006A78: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
	// 83006A7C: 7FCB60CE  lvx v30, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[30] using VectorMaskL[(tmp.u32 & 0xF)]
}

pub fn sub_83006A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83006A80 size=12
	// 83006A80: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
	// 83006A84: 7FEB60CE  lvx v31, r11, r12
	tmp.u32 = ctx.r[11].u32 + ctx.r[12].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[31] using VectorMaskL[(tmp.u32 & 0xF)]
	// 83006A88: 4E800020  blr
	return;
}

pub fn sub_83006A8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006A8C size=8
	// 83006A8C: 3960FC00  li r11, -0x400
	ctx.r[11].s64 = -1024;
}

pub fn sub_83006A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006A94 size=8
	// 83006A94: 3960FC10  li r11, -0x3f0
	ctx.r[11].s64 = -1008;
}

pub fn sub_83006A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006A9C size=8
	// 83006A9C: 3960FC20  li r11, -0x3e0
	ctx.r[11].s64 = -992;
}

pub fn sub_83006AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AA4 size=8
	// 83006AA4: 3960FC30  li r11, -0x3d0
	ctx.r[11].s64 = -976;
}

pub fn sub_83006AAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AAC size=8
	// 83006AAC: 3960FC40  li r11, -0x3c0
	ctx.r[11].s64 = -960;
}

pub fn sub_83006AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AB4 size=8
	// 83006AB4: 3960FC50  li r11, -0x3b0
	ctx.r[11].s64 = -944;
}

pub fn sub_83006ABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006ABC size=8
	// 83006ABC: 3960FC60  li r11, -0x3a0
	ctx.r[11].s64 = -928;
}

pub fn sub_83006AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AC4 size=8
	// 83006AC4: 3960FC70  li r11, -0x390
	ctx.r[11].s64 = -912;
}

pub fn sub_83006ACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006ACC size=8
	// 83006ACC: 3960FC80  li r11, -0x380
	ctx.r[11].s64 = -896;
}

pub fn sub_83006AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AD4 size=8
	// 83006AD4: 3960FC90  li r11, -0x370
	ctx.r[11].s64 = -880;
}

pub fn sub_83006ADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006ADC size=8
	// 83006ADC: 3960FCA0  li r11, -0x360
	ctx.r[11].s64 = -864;
}

pub fn sub_83006AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AE4 size=8
	// 83006AE4: 3960FCB0  li r11, -0x350
	ctx.r[11].s64 = -848;
}

pub fn sub_83006AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AEC size=8
	// 83006AEC: 3960FCC0  li r11, -0x340
	ctx.r[11].s64 = -832;
}

pub fn sub_83006AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AF4 size=8
	// 83006AF4: 3960FCD0  li r11, -0x330
	ctx.r[11].s64 = -816;
}

pub fn sub_83006AFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006AFC size=8
	// 83006AFC: 3960FCE0  li r11, -0x320
	ctx.r[11].s64 = -800;
}

pub fn sub_83006B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B04 size=8
	// 83006B04: 3960FCF0  li r11, -0x310
	ctx.r[11].s64 = -784;
}

pub fn sub_83006B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B0C size=8
	// 83006B0C: 3960FD00  li r11, -0x300
	ctx.r[11].s64 = -768;
}

pub fn sub_83006B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B14 size=8
	// 83006B14: 3960FD10  li r11, -0x2f0
	ctx.r[11].s64 = -752;
}

pub fn sub_83006B1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B1C size=8
	// 83006B1C: 3960FD20  li r11, -0x2e0
	ctx.r[11].s64 = -736;
}

pub fn sub_83006B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B24 size=8
	// 83006B24: 3960FD30  li r11, -0x2d0
	ctx.r[11].s64 = -720;
}

pub fn sub_83006B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B2C size=8
	// 83006B2C: 3960FD40  li r11, -0x2c0
	ctx.r[11].s64 = -704;
}

pub fn sub_83006B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B34 size=8
	// 83006B34: 3960FD50  li r11, -0x2b0
	ctx.r[11].s64 = -688;
}

pub fn sub_83006B3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B3C size=8
	// 83006B3C: 3960FD60  li r11, -0x2a0
	ctx.r[11].s64 = -672;
}

pub fn sub_83006B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B44 size=8
	// 83006B44: 3960FD70  li r11, -0x290
	ctx.r[11].s64 = -656;
}

pub fn sub_83006B4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B4C size=8
	// 83006B4C: 3960FD80  li r11, -0x280
	ctx.r[11].s64 = -640;
}

pub fn sub_83006B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B54 size=8
	// 83006B54: 3960FD90  li r11, -0x270
	ctx.r[11].s64 = -624;
}

pub fn sub_83006B5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B5C size=8
	// 83006B5C: 3960FDA0  li r11, -0x260
	ctx.r[11].s64 = -608;
}

pub fn sub_83006B64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B64 size=8
	// 83006B64: 3960FDB0  li r11, -0x250
	ctx.r[11].s64 = -592;
}

pub fn sub_83006B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B6C size=8
	// 83006B6C: 3960FDC0  li r11, -0x240
	ctx.r[11].s64 = -576;
}

pub fn sub_83006B74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B74 size=8
	// 83006B74: 3960FDD0  li r11, -0x230
	ctx.r[11].s64 = -560;
}

pub fn sub_83006B7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B7C size=8
	// 83006B7C: 3960FDE0  li r11, -0x220
	ctx.r[11].s64 = -544;
}

pub fn sub_83006B84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B84 size=8
	// 83006B84: 3960FDF0  li r11, -0x210
	ctx.r[11].s64 = -528;
}

pub fn sub_83006B8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B8C size=8
	// 83006B8C: 3960FE00  li r11, -0x200
	ctx.r[11].s64 = -512;
}

pub fn sub_83006B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B94 size=8
	// 83006B94: 3960FE10  li r11, -0x1f0
	ctx.r[11].s64 = -496;
}

pub fn sub_83006B9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006B9C size=8
	// 83006B9C: 3960FE20  li r11, -0x1e0
	ctx.r[11].s64 = -480;
}

pub fn sub_83006BA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BA4 size=8
	// 83006BA4: 3960FE30  li r11, -0x1d0
	ctx.r[11].s64 = -464;
}

pub fn sub_83006BAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BAC size=8
	// 83006BAC: 3960FE40  li r11, -0x1c0
	ctx.r[11].s64 = -448;
}

pub fn sub_83006BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BB4 size=8
	// 83006BB4: 3960FE50  li r11, -0x1b0
	ctx.r[11].s64 = -432;
}

pub fn sub_83006BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BBC size=8
	// 83006BBC: 3960FE60  li r11, -0x1a0
	ctx.r[11].s64 = -416;
}

pub fn sub_83006BC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BC4 size=8
	// 83006BC4: 3960FE70  li r11, -0x190
	ctx.r[11].s64 = -400;
}

pub fn sub_83006BCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BCC size=8
	// 83006BCC: 3960FE80  li r11, -0x180
	ctx.r[11].s64 = -384;
}

pub fn sub_83006BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BD4 size=8
	// 83006BD4: 3960FE90  li r11, -0x170
	ctx.r[11].s64 = -368;
}

pub fn sub_83006BDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BDC size=8
	// 83006BDC: 3960FEA0  li r11, -0x160
	ctx.r[11].s64 = -352;
}

pub fn sub_83006BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BE4 size=8
	// 83006BE4: 3960FEB0  li r11, -0x150
	ctx.r[11].s64 = -336;
}

pub fn sub_83006BEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BEC size=8
	// 83006BEC: 3960FEC0  li r11, -0x140
	ctx.r[11].s64 = -320;
}

pub fn sub_83006BF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BF4 size=8
	// 83006BF4: 3960FED0  li r11, -0x130
	ctx.r[11].s64 = -304;
}

pub fn sub_83006BFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006BFC size=8
	// 83006BFC: 3960FEE0  li r11, -0x120
	ctx.r[11].s64 = -288;
}

pub fn sub_83006C04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C04 size=8
	// 83006C04: 3960FEF0  li r11, -0x110
	ctx.r[11].s64 = -272;
}

pub fn sub_83006C0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C0C size=8
	// 83006C0C: 3960FF00  li r11, -0x100
	ctx.r[11].s64 = -256;
}

pub fn sub_83006C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C14 size=8
	// 83006C14: 3960FF10  li r11, -0xf0
	ctx.r[11].s64 = -240;
}

pub fn sub_83006C1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C1C size=8
	// 83006C1C: 3960FF20  li r11, -0xe0
	ctx.r[11].s64 = -224;
}

pub fn sub_83006C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C24 size=8
	// 83006C24: 3960FF30  li r11, -0xd0
	ctx.r[11].s64 = -208;
}

pub fn sub_83006C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C2C size=8
	// 83006C2C: 3960FF40  li r11, -0xc0
	ctx.r[11].s64 = -192;
}

pub fn sub_83006C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C34 size=8
	// 83006C34: 3960FF50  li r11, -0xb0
	ctx.r[11].s64 = -176;
}

pub fn sub_83006C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C3C size=8
	// 83006C3C: 3960FF60  li r11, -0xa0
	ctx.r[11].s64 = -160;
}

pub fn sub_83006C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C44 size=8
	// 83006C44: 3960FF70  li r11, -0x90
	ctx.r[11].s64 = -144;
}

pub fn sub_83006C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C4C size=8
	// 83006C4C: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
}

pub fn sub_83006C54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C54 size=8
	// 83006C54: 3960FF90  li r11, -0x70
	ctx.r[11].s64 = -112;
}

pub fn sub_83006C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C5C size=8
	// 83006C5C: 3960FFA0  li r11, -0x60
	ctx.r[11].s64 = -96;
}

pub fn sub_83006C64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C64 size=8
	// 83006C64: 3960FFB0  li r11, -0x50
	ctx.r[11].s64 = -80;
}

pub fn sub_83006C6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C6C size=8
	// 83006C6C: 3960FFC0  li r11, -0x40
	ctx.r[11].s64 = -64;
}

pub fn sub_83006C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C74 size=8
	// 83006C74: 3960FFD0  li r11, -0x30
	ctx.r[11].s64 = -48;
}

pub fn sub_83006C7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C7C size=8
	// 83006C7C: 3960FFE0  li r11, -0x20
	ctx.r[11].s64 = -32;
}

pub fn sub_83006C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83006C84 size=12
	// 83006C84: 3960FFF0  li r11, -0x10
	ctx.r[11].s64 = -16;
}

pub fn sub_83006F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83006F60 size=264
	// 83006F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83006F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83006F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83006F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83006F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83006F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83006F78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83006F7C: 409A0030  bne cr6, 0x83006fac
	if !ctx.cr[6].eq {
	pc = 0x83006FAC; continue 'dispatch;
	}
	// 83006F80: 4BCAB0D9  bl 0x82cb2058
	ctx.lr = 0x83006F84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83006F84: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83006F88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83006F8C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83006F90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83006F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83006F98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83006F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006FA0: 4BCAAF79  bl 0x82cb1f18
	ctx.lr = 0x83006FA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83006FA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83006FA8: 480000A8  b 0x83007050
	pc = 0x83007050; continue 'dispatch;
	// 83006FAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83006FB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83006FB4: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 83006FB8: 4BCBFCC9  bl 0x82cc6c80
	ctx.lr = 0x83006FBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6C80);
	// 83006FBC: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 83006FC0: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 83006FC4: A1010052  lhz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83006FC8: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 83006FCC: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 83006FD0: 995F0002  stb r10, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 83006FD4: A0C10056  lhz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83006FD8: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 83006FDC: A1410050  lhz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83006FE0: 7D2A4BD6  divw r9, r10, r9
	ctx.r[9].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 83006FE4: 1D290064  mulli r9, r9, 0x64
	ctx.r[9].s64 = ctx.r[9].s64 * 100;
	// 83006FE8: 7CA95050  subf r5, r9, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83006FEC: 7D485BD6  divw r10, r8, r11
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 83006FF0: 7D265BD6  divw r9, r6, r11
	ctx.r[9].s32 = ctx.r[6].s32 / ctx.r[11].s32;
	// 83006FF4: 7CE55BD6  divw r7, r5, r11
	ctx.r[7].s32 = ctx.r[5].s32 / ctx.r[11].s32;
	// 83006FF8: 1C69000A  mulli r3, r9, 0xa
	ctx.r[3].s64 = ctx.r[9].s64 * 10;
	// 83006FFC: 1C8A000A  mulli r4, r10, 0xa
	ctx.r[4].s64 = ctx.r[10].s64 * 10;
	// 83007000: 1FC7000A  mulli r30, r7, 0xa
	ctx.r[30].s64 = ctx.r[7].s64 * 10;
	// 83007004: 7D285BD6  divw r9, r8, r11
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	// 83007008: 7CE65BD6  divw r7, r6, r11
	ctx.r[7].s32 = ctx.r[6].s32 / ctx.r[11].s32;
	// 8300700C: 7D455BD6  divw r10, r5, r11
	ctx.r[10].s32 = ctx.r[5].s32 / ctx.r[11].s32;
	// 83007010: 7CC33050  subf r6, r3, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[3].s64;
	// 83007014: 7D044050  subf r8, r4, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	// 83007018: 7D7E2850  subf r11, r30, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[30].s64;
	// 8300701C: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 83007020: 39080030  addi r8, r8, 0x30
	ctx.r[8].s64 = ctx.r[8].s64 + 48;
	// 83007024: 38E70030  addi r7, r7, 0x30
	ctx.r[7].s64 = ctx.r[7].s64 + 48;
	// 83007028: 993F0000  stb r9, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8300702C: 38C60030  addi r6, r6, 0x30
	ctx.r[6].s64 = ctx.r[6].s64 + 48;
	// 83007030: 991F0001  stb r8, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[8].u8 ) };
	// 83007034: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 83007038: 98FF0003  stb r7, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 8300703C: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 83007040: 98DF0004  stb r6, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u8 ) };
	// 83007044: 995F0006  stb r10, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 83007048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300704C: 997F0007  stb r11, 7(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7 as u32), ctx.r[11].u8 ) };
	// 83007050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83007054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300705C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83007060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007064: 4E800020  blr
	return;
}

pub fn sub_83007068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007068 size=336
	// 83007068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300706C: 4BCA239D  bl 0x82ca9408
	ctx.lr = 0x83007070;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83007070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007074: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83007078: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300707C: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 83007080: 41980038  blt cr6, 0x830070b8
	if ctx.cr[6].lt {
	pc = 0x830070B8; continue 'dispatch;
	}
	// 83007084: 419A0020  beq cr6, 0x830070a4
	if ctx.cr[6].eq {
	pc = 0x830070A4; continue 'dispatch;
	}
	// 83007088: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 8300708C: 4098003C  bge cr6, 0x830070c8
	if !ctx.cr[6].lt {
	pc = 0x830070C8; continue 'dispatch;
	}
	// 83007090: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83007094: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 83007098: 396B9B74  addi r11, r11, -0x648c
	ctx.r[11].s64 = ctx.r[11].s64 + -25740;
	// 8300709C: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 830070A0: 4800002C  b 0x830070cc
	pc = 0x830070CC; continue 'dispatch;
	// 830070A4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830070A8: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 830070AC: 396B9B74  addi r11, r11, -0x648c
	ctx.r[11].s64 = ctx.r[11].s64 + -25740;
	// 830070B0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 830070B4: 48000018  b 0x830070cc
	pc = 0x830070CC; continue 'dispatch;
	// 830070B8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830070BC: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 830070C0: 3BEB9B74  addi r31, r11, -0x648c
	ctx.r[31].s64 = ctx.r[11].s64 + -25740;
	// 830070C4: 48000008  b 0x830070cc
	pc = 0x830070CC; continue 'dispatch;
	// 830070C8: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830070CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 830070D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830070D4: 38AB97EC  addi r5, r11, -0x6814
	ctx.r[5].s64 = ctx.r[11].s64 + -26644;
	// 830070D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830070DC: 4B169BED  bl 0x82170cc8
	ctx.lr = 0x830070E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82170CC8);
	// 830070E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830070E4: 4182001C  beq 0x83007100
	if ctx.cr[0].eq {
	pc = 0x83007100; continue 'dispatch;
	}
	// 830070E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830070EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830070F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830070F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830070F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830070FC: 4BCAAE65  bl 0x82cb1f60
	ctx.lr = 0x83007100;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83007100: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83007104: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 83007108: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8300710C: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 83007110: 419A0018  beq cr6, 0x83007128
	if ctx.cr[6].eq {
	pc = 0x83007128; continue 'dispatch;
	}
	// 83007114: 2F0A002F  cmpwi cr6, r10, 0x2f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 47, &mut ctx.xer);
	// 83007118: 419A0010  beq cr6, 0x83007128
	if ctx.cr[6].eq {
	pc = 0x83007128; continue 'dispatch;
	}
	// 8300711C: 3940005C  li r10, 0x5c
	ctx.r[10].s64 = 92;
	// 83007120: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83007124: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83007128: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8300712C: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 83007130: 39400074  li r10, 0x74
	ctx.r[10].s64 = 116;
	// 83007134: 419A0008  beq cr6, 0x8300713c
	if ctx.cr[6].eq {
	pc = 0x8300713C; continue 'dispatch;
	}
	// 83007138: 39400073  li r10, 0x73
	ctx.r[10].s64 = 115;
	// 8300713C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83007140: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83007144: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83007148: 4B217BC1  bl 0x8221ed08
	ctx.lr = 0x8300714C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221ED08);
	// 8300714C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83007150: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83007154: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83007158: 48002C91  bl 0x83009de8
	ctx.lr = 0x8300715C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009DE8);
	// 8300715C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007160: 4182001C  beq 0x8300717c
	if ctx.cr[0].eq {
	pc = 0x8300717C; continue 'dispatch;
	}
	// 83007164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007168: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300716C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007170: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007178: 4BCAADE9  bl 0x82cb1f60
	ctx.lr = 0x8300717C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 8300717C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83007180: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83007184: 38AB230C  addi r5, r11, 0x230c
	ctx.r[5].s64 = ctx.r[11].s64 + 8972;
	// 83007188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300718C: 4BCA63BD  bl 0x82cad548
	ctx.lr = 0x83007190;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAD548);
	// 83007190: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007194: 4182001C  beq 0x830071b0
	if ctx.cr[0].eq {
	pc = 0x830071B0; continue 'dispatch;
	}
	// 83007198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300719C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830071A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830071A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830071A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830071AC: 4BCAADB5  bl 0x82cb1f60
	ctx.lr = 0x830071B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 830071B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830071B4: 4BCA22A4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_830071B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830071B8 size=192
	// 830071B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830071BC: 4BCA224D  bl 0x82ca9408
	ctx.lr = 0x830071C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 830071C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830071C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830071C8: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 830071CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830071D0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830071D4: 4BCAA58D  bl 0x82cb1760
	ctx.lr = 0x830071D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1760);
	// 830071D8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 830071DC: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 830071E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830071E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830071E8: 4BCA5941  bl 0x82cacb28
	ctx.lr = 0x830071EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CACB28);
	// 830071EC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 830071F0: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830071F4: 4198000C  blt cr6, 0x83007200
	if ctx.cr[6].lt {
	pc = 0x83007200; continue 'dispatch;
	}
	// 830071F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830071FC: 4800006C  b 0x83007268
	pc = 0x83007268; continue 'dispatch;
	// 83007200: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83007204: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83007208: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8300720C: 48002BDD  bl 0x83009de8
	ctx.lr = 0x83007210;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009DE8);
	// 83007210: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007214: 4182001C  beq 0x83007230
	if ctx.cr[0].eq {
	pc = 0x83007230; continue 'dispatch;
	}
	// 83007218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300721C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007220: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007224: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007228: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300722C: 4BCAAD35  bl 0x82cb1f60
	ctx.lr = 0x83007230;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83007230: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 83007234: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83007238: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8300723C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007240: 4B169A89  bl 0x82170cc8
	ctx.lr = 0x83007244;
	crate::recompiler::externs::call(&mut ctx, base, 0x82170CC8);
	// 83007244: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007248: 4182001C  beq 0x83007264
	if ctx.cr[0].eq {
	pc = 0x83007264; continue 'dispatch;
	}
	// 8300724C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007250: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007258: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300725C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007260: 4BCAAD01  bl 0x82cb1f60
	ctx.lr = 0x83007264;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83007264: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300726C: 4BCA21EC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 83007270: 832B9CA4  lwz r25, -0x635c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25436 as u32) ) } as u64;
	// 83007274: 8210B298  lwz r16, -0x4d68(r16)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-19816 as u32) ) } as u64;
}

pub fn sub_83007278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007278 size=524
	// 83007278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300727C: 4BCA217D  bl 0x82ca93f8
	ctx.lr = 0x83007280;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 83007280: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83007284: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007288: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8300728C: 933F00B4  stw r25, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[25].u32 ) };
	// 83007290: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83007294: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83007298: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 8300729C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 830072A0: 931F0050  stw r24, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 830072A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830072A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830072AC: 4BCAADAD  bl 0x82cb2058
	ctx.lr = 0x830072B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830072B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830072B4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830072B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830072BC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830072C0: 4BCAE779  bl 0x82cb5a38
	ctx.lr = 0x830072C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5A38);
	// 830072C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830072C8: 40820014  bne 0x830072dc
	if !ctx.cr[0].eq {
	pc = 0x830072DC; continue 'dispatch;
	}
	// 830072CC: 931E0000  stw r24, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 830072D0: 4BCAAD89  bl 0x82cb2058
	ctx.lr = 0x830072D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830072D4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830072D8: 480001A4  b 0x8300747c
	pc = 0x8300747C; continue 'dispatch;
	// 830072DC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830072E0: 4BCAE899  bl 0x82cb5b78
	ctx.lr = 0x830072E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5B78);
	// 830072E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830072E8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830072EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 830072F0: 409A0010  bne cr6, 0x83007300
	if !ctx.cr[6].eq {
	pc = 0x83007300; continue 'dispatch;
	}
	// 830072F4: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 830072F8: 3BCB9B74  addi r30, r11, -0x648c
	ctx.r[30].s64 = ctx.r[11].s64 + -25740;
	// 830072FC: 48000010  b 0x8300730c
	pc = 0x8300730C; continue 'dispatch;
	// 83007300: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 83007304: 396B9B74  addi r11, r11, -0x648c
	ctx.r[11].s64 = ctx.r[11].s64 + -25740;
	// 83007308: 3BCB0024  addi r30, r11, 0x24
	ctx.r[30].s64 = ctx.r[11].s64 + 36;
	// 8300730C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83007314: 409A0010  bne cr6, 0x83007324
	if !ctx.cr[6].eq {
	pc = 0x83007324; continue 'dispatch;
	}
	// 83007318: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300731C: 4BFFFD4D  bl 0x83007068
	ctx.lr = 0x83007320;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007068);
	// 83007320: 4800001C  b 0x8300733c
	pc = 0x8300733C; continue 'dispatch;
	// 83007324: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83007328: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300732C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007330: 4BFFFE89  bl 0x830071b8
	ctx.lr = 0x83007334;
	crate::recompiler::externs::call(&mut ctx, base, 0x830071B8);
	// 83007334: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007338: 40820104  bne 0x8300743c
	if !ctx.cr[0].eq {
	pc = 0x8300743C; continue 'dispatch;
	}
	// 8300733C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007344: 48002C45  bl 0x83009f88
	ctx.lr = 0x83007348;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009F88);
	// 83007348: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300734C: 40820024  bne 0x83007370
	if !ctx.cr[0].eq {
	pc = 0x83007370; continue 'dispatch;
	}
	// 83007350: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83007354: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83007358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300735C: 4BFFFE5D  bl 0x830071b8
	ctx.lr = 0x83007360;
	crate::recompiler::externs::call(&mut ctx, base, 0x830071B8);
	// 83007360: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007364: 408200D8  bne 0x8300743c
	if !ctx.cr[0].eq {
	pc = 0x8300743C; continue 'dispatch;
	}
	// 83007368: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300736C: 4BFFFFD0  b 0x8300733c
	pc = 0x8300733C; continue 'dispatch;
	// 83007370: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83007374: 409A0050  bne cr6, 0x830073c4
	if !ctx.cr[6].eq {
	pc = 0x830073C4; continue 'dispatch;
	}
	// 83007378: 4BCABA91  bl 0x82cb2e08
	ctx.lr = 0x8300737C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2E08);
	// 8300737C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83007380: 40820010  bne 0x83007390
	if !ctx.cr[0].eq {
	pc = 0x83007390; continue 'dispatch;
	}
	// 83007384: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 83007388: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300738C: 480000B0  b 0x8300743c
	pc = 0x8300743C; continue 'dispatch;
	// 83007390: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83007394: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83007398: 409A001C  bne cr6, 0x830073b4
	if !ctx.cr[6].eq {
	pc = 0x830073B4; continue 'dispatch;
	}
	// 8300739C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830073A0: 4BCA30E9  bl 0x82caa488
	ctx.lr = 0x830073A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAA488);
	// 830073A4: 907D002C  stw r3, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 830073A8: 546B003E  slwi r11, r3, 0
	// 830073AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830073B0: 419AFFD4  beq cr6, 0x83007384
	if ctx.cr[6].eq {
	pc = 0x83007384; continue 'dispatch;
	}
	// 830073B4: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 830073B8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830073BC: 907F00B4  stw r3, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 830073C0: 48000054  b 0x83007414
	pc = 0x83007414; continue 'dispatch;
	// 830073C4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830073C8: 419A0044  beq cr6, 0x8300740c
	if ctx.cr[6].eq {
	pc = 0x8300740C; continue 'dispatch;
	}
	// 830073CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830073D0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830073D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830073D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830073DC: 409AFFF4  bne cr6, 0x830073d0
	if !ctx.cr[6].eq {
	pc = 0x830073D0; continue 'dispatch;
	}
	// 830073E0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830073E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830073E8: 556B003E  slwi r11, r11, 0
	// 830073EC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830073F0: 4198001C  blt cr6, 0x8300740c
	if ctx.cr[6].lt {
	pc = 0x8300740C; continue 'dispatch;
	}
	// 830073F4: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 830073F8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830073FC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83007400: 419A003C  beq cr6, 0x8300743c
	if ctx.cr[6].eq {
	pc = 0x8300743C; continue 'dispatch;
	}
	// 83007404: 9B190000  stb r24, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[24].u8 ) };
	// 83007408: 48000034  b 0x8300743c
	pc = 0x8300743C; continue 'dispatch;
	// 8300740C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83007410: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83007414: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83007418: 4B1698B1  bl 0x82170cc8
	ctx.lr = 0x8300741C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82170CC8);
	// 8300741C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007420: 4182001C  beq 0x8300743c
	if ctx.cr[0].eq {
	pc = 0x8300743C; continue 'dispatch;
	}
	// 83007424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007428: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300742C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007430: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007438: 4BCAAB29  bl 0x82cb1f60
	ctx.lr = 0x8300743C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 8300743C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007440: 399F00A0  addi r12, r31, 0xa0
	ctx.r[12].s64 = ctx.r[31].s64 + 160;
	// 83007444: 48000041  bl 0x83007484
	ctx.lr = 0x83007448;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007484);
	// 83007448: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8300744C: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83007450: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83007454: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83007458: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300745C: 419A0010  beq cr6, 0x8300746c
	if ctx.cr[6].eq {
	pc = 0x8300746C; continue 'dispatch;
	}
	// 83007460: 4BCAABF9  bl 0x82cb2058
	ctx.lr = 0x83007464;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007464: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83007468: 48000010  b 0x83007478
	pc = 0x83007478; continue 'dispatch;
	// 8300746C: 4BCAABED  bl 0x82cb2058
	ctx.lr = 0x83007470;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007470: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83007474: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300747C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83007480: 4BCA1FC8  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
}

pub fn sub_83007484(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007484 size=36
	// 83007484: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007488: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300748C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007490: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83007494: 4BCAE585  bl 0x82cb5a18
	ctx.lr = 0x83007498;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5A18);
	// 83007498: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300749C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830074A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830074A4: 4E800020  blr
	return;
}

pub fn sub_830074A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830074A8 size=72
	// 830074A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830074AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830074B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830074B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830074B8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830074BC: 38C07FFF  li r6, 0x7fff
	ctx.r[6].s64 = 32767;
	// 830074C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830074C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830074C8: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 830074CC: 4BFFFDAD  bl 0x83007278
	ctx.lr = 0x830074D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007278);
	// 830074D0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830074D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830074D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830074DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830074E0: 4E800020  blr
	return;
}

pub fn sub_830074F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830074F0 size=16
	// 830074F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830074F4: 4BCA1F09  bl 0x82ca93fc
	ctx.lr = 0x830074F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 830074F8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830074FC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_830077B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830077B0 size=112
	// 830077B0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830077B4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830077B8: FBA1FFF0  std r29, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[29].u64 ) };
	// 830077BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830077C0: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 830077C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830077C8: 83BF0058  lwz r29, 0x58(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830077CC: 4800001C  b 0x830077e8
	pc = 0x830077E8; continue 'dispatch;
	// 830077D0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830077D4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830077D8: FBA1FFF0  std r29, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[29].u64 ) };
	// 830077DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830077E0: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 830077E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830077E8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830077EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830077F0: 419A000C  beq cr6, 0x830077fc
	if ctx.cr[6].eq {
	pc = 0x830077FC; continue 'dispatch;
	}
	// 830077F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830077F8: 4BCA7769  bl 0x82caef60
	ctx.lr = 0x830077FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 830077FC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83007800: 4BCAE219  bl 0x82cb5a18
	ctx.lr = 0x83007804;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5A18);
	// 83007804: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007808: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8300780C: EBA1FFF0  ld r29, -0x10(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007810: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 83007814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007818: 4E800020  blr
	return;
}

pub fn sub_83007820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007820 size=80
	// 83007820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300782C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83007830: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83007834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007838: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300783C: 4BFFFCB5  bl 0x830074f0
	ctx.lr = 0x83007840;
	crate::recompiler::externs::call(&mut ctx, base, 0x830074F0);
	// 83007840: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83007844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300784C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007850: 4E800020  blr
	return;
}

pub fn sub_83007870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007870 size=144
	// 83007870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300787C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007880: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 83007884: 7D6B2039  and. r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007888: 41820030  beq 0x830078b8
	if ctx.cr[0].eq {
	pc = 0x830078B8; continue 'dispatch;
	}
	// 8300788C: 4BCAA7CD  bl 0x82cb2058
	ctx.lr = 0x83007890;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007890: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007898: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300789C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830078A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830078A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830078A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830078AC: 4BCAA66D  bl 0x82cb1f18
	ctx.lr = 0x830078B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 830078B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830078B4: 48000038  b 0x830078ec
	pc = 0x830078EC; continue 'dispatch;
	// 830078B8: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 830078BC: 41990008  bgt cr6, 0x830078c4
	if ctx.cr[6].gt {
	pc = 0x830078C4; continue 'dispatch;
	}
	// 830078C0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830078C4: 3BE4FFFF  addi r31, r4, -1
	ctx.r[31].s64 = ctx.r[4].s64 + -1;
	// 830078C8: 7D7F1A14  add r11, r31, r3
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 830078CC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830078D0: 4BCA2BB9  bl 0x82caa488
	ctx.lr = 0x830078D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAA488);
	// 830078D4: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830078D8: 4182FFD8  beq 0x830078b0
	if ctx.cr[0].eq {
	pc = 0x830078B0; continue 'dispatch;
	}
	// 830078DC: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830078E0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830078E4: 7D43F878  andc r3, r10, r31
	ctx.r[3].u64 = ctx.r[10].u64 & !ctx.r[31].u64;
	// 830078E8: 9163FFFC  stw r11, -4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 830078EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830078F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830078F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830078F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830078FC: 4E800020  blr
	return;
}

pub fn sub_83007900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007900 size=104
	// 83007900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300790C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007914: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83007918: 4B180DE1  bl 0x821886f8
	ctx.lr = 0x8300791C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821886F8);
	// 8300791C: 3D80FE62  lis r12, -0x19e
	ctx.r[12].s64 = -27131904;
	// 83007920: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83007924: 3D600098  lis r11, 0x98
	ctx.r[11].s64 = 9961472;
	// 83007928: 618C4E21  ori r12, r12, 0x4e21
	ctx.r[12].u64 = ctx.r[12].u64 | 20001;
	// 8300792C: 616B9680  ori r11, r11, 0x9680
	ctx.r[11].u64 = ctx.r[11].u64 | 38528;
	// 83007930: 798C07C6  sldi r12, r12, 0x20
	// 83007934: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83007938: 658C2AC1  oris r12, r12, 0x2ac1
	ctx.r[12].u64 = ctx.r[12].u64 | 717291520;
	// 8300793C: 618C8000  ori r12, r12, 0x8000
	ctx.r[12].u64 = ctx.r[12].u64 | 32768;
	// 83007940: 7D4A6214  add r10, r10, r12
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[12].u64;
	// 83007944: 7C6A5B92  divdu r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 / ctx.r[11].u64;
	// 83007948: 419A0008  beq cr6, 0x83007950
	if ctx.cr[6].eq {
	pc = 0x83007950; continue 'dispatch;
	}
	// 8300794C: F87F0000  std r3, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 83007950: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83007954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300795C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007960: 4E800020  blr
	return;
}

pub fn sub_83007968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007968 size=88
	// 83007968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300796C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83007974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300797C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83007980: 41980014  blt cr6, 0x83007994
	if ctx.cr[6].lt {
	pc = 0x83007994; continue 'dispatch;
	}
	// 83007984: 4800275D  bl 0x8300a0e0
	ctx.lr = 0x83007988;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A0E0);
	// 83007988: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300798C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83007990: 4198000C  blt cr6, 0x8300799c
	if ctx.cr[6].lt {
	pc = 0x8300799C; continue 'dispatch;
	}
	// 83007994: 4800274D  bl 0x8300a0e0
	ctx.lr = 0x83007998;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A0E0);
	// 83007998: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300799C: 48002755  bl 0x8300a0f0
	ctx.lr = 0x830079A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A0F0);
	// 830079A0: 57EB103A  slwi r11, r31, 2
	// 830079A4: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830079A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830079AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830079B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830079B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830079B8: 4E800020  blr
	return;
}

pub fn sub_830079C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830079C0 size=160
	// 830079C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830079C4: 4BCA1A49  bl 0x82ca940c
	ctx.lr = 0x830079C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 830079C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830079CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830079D0: 4BCAB439  bl 0x82cb2e08
	ctx.lr = 0x830079D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2E08);
	// 830079D4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830079D8: 40820010  bne 0x830079e8
	if !ctx.cr[0].eq {
	pc = 0x830079E8; continue 'dispatch;
	}
	// 830079DC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 830079E0: 386BD658  addi r3, r11, -0x29a8
	ctx.r[3].s64 = ctx.r[11].s64 + -10664;
	// 830079E4: 48000068  b 0x83007a4c
	pc = 0x83007A4C; continue 'dispatch;
	// 830079E8: 83FE0024  lwz r31, 0x24(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830079EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830079F0: 409A0020  bne cr6, 0x83007a10
	if !ctx.cr[6].eq {
	pc = 0x83007A10; continue 'dispatch;
	}
	// 830079F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830079F8: 38600086  li r3, 0x86
	ctx.r[3].s64 = 134;
	// 830079FC: 4BCA9C6D  bl 0x82cb1668
	ctx.lr = 0x83007A00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1668);
	// 83007A00: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83007A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83007A08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83007A0C: 4182FFD0  beq 0x830079dc
	if ctx.cr[0].eq {
	pc = 0x830079DC; continue 'dispatch;
	}
	// 83007A10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83007A14: 4BFFFF55  bl 0x83007968
	ctx.lr = 0x83007A18;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007968);
	// 83007A18: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83007A1C: 38800086  li r4, 0x86
	ctx.r[4].s64 = 134;
	// 83007A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007A24: 4B1692A5  bl 0x82170cc8
	ctx.lr = 0x83007A28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82170CC8);
	// 83007A28: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83007A2C: 4182001C  beq 0x83007a48
	if ctx.cr[0].eq {
	pc = 0x83007A48; continue 'dispatch;
	}
	// 83007A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007A38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007A3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007A40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007A44: 4BCAA51D  bl 0x82cb1f60
	ctx.lr = 0x83007A48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83007A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83007A4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83007A50: 4BCA1A0C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_83007A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007A60 size=460
	// 83007A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007A64: 4BCA1991  bl 0x82ca93f4
	ctx.lr = 0x83007A68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
	// 83007A68: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83007A6C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007A70: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83007A74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83007A78: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 83007A7C: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 83007A80: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83007A84: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007A88: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 83007A8C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83007A90: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007A94: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83007A98: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83007A9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007AA0: 409A0030  bne cr6, 0x83007ad0
	if !ctx.cr[6].eq {
	pc = 0x83007AD0; continue 'dispatch;
	}
	// 83007AA4: 4BCAA5B5  bl 0x82cb2058
	ctx.lr = 0x83007AA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007AA8: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007AB0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007AB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007ABC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007AC4: 4BCAA455  bl 0x82cb1f18
	ctx.lr = 0x83007AC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83007AC8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83007ACC: 48000158  b 0x83007c24
	pc = 0x83007C24; continue 'dispatch;
	// 83007AD0: 7F6B0034  cntlzw r11, r27
	ctx.r[11].u64 = if ctx.r[27].u32 == 0 { 32 } else { ctx.r[27].u32.leading_zeros() as u64 };
	// 83007AD4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007AD8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007ADC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007AE0: 419AFFC4  beq cr6, 0x83007aa4
	if ctx.cr[6].eq {
	pc = 0x83007AA4; continue 'dispatch;
	}
	// 83007AE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007AE8: 4BCA73B9  bl 0x82caeea0
	ctx.lr = 0x83007AEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEEA0);
	// 83007AEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007AF0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007AF4: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007AF8: 408200F8  bne 0x83007bf0
	if !ctx.cr[0].eq {
	pc = 0x83007BF0; continue 'dispatch;
	}
	// 83007AFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B00: 4BCAD7F9  bl 0x82cb52f8
	ctx.lr = 0x83007B04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B04: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83007B08: 419A004C  beq cr6, 0x83007b54
	if ctx.cr[6].eq {
	pc = 0x83007B54; continue 'dispatch;
	}
	// 83007B0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B10: 4BCAD7E9  bl 0x82cb52f8
	ctx.lr = 0x83007B14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B14: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 83007B18: 419A003C  beq cr6, 0x83007b54
	if ctx.cr[6].eq {
	pc = 0x83007B54; continue 'dispatch;
	}
	// 83007B1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B20: 4BCAD7D9  bl 0x82cb52f8
	ctx.lr = 0x83007B24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B24: 7C6A2E70  srawi r10, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 83007B28: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83007B2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B30: 555C103A  slwi r28, r10, 2
	// 83007B34: 3BAB7760  addi r29, r11, 0x7760
	ctx.r[29].s64 = ctx.r[11].s64 + 30560;
	// 83007B38: 4BCAD7C1  bl 0x82cb52f8
	ctx.lr = 0x83007B3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B3C: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83007B40: 546A3572  rlwinm r10, r3, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 83007B44: 3D20832F  lis r9, -0x7cd1
	ctx.r[9].s64 = -2094071808;
	// 83007B48: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83007B4C: 3B89F6E8  addi r28, r9, -0x918
	ctx.r[28].s64 = ctx.r[9].s64 + -2328;
	// 83007B50: 48000018  b 0x83007b68
	pc = 0x83007B68; continue 'dispatch;
	// 83007B54: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83007B58: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83007B5C: 3B8AF6E8  addi r28, r10, -0x918
	ctx.r[28].s64 = ctx.r[10].s64 + -2328;
	// 83007B60: 3BAB7760  addi r29, r11, 0x7760
	ctx.r[29].s64 = ctx.r[11].s64 + 30560;
	// 83007B64: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83007B68: 896B0028  lbz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007B6C: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007B70: 40820054  bne 0x83007bc4
	if !ctx.cr[0].eq {
	pc = 0x83007BC4; continue 'dispatch;
	}
	// 83007B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B78: 4BCAD781  bl 0x82cb52f8
	ctx.lr = 0x83007B7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B7C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83007B80: 419A0038  beq cr6, 0x83007bb8
	if ctx.cr[6].eq {
	pc = 0x83007BB8; continue 'dispatch;
	}
	// 83007B84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B88: 4BCAD771  bl 0x82cb52f8
	ctx.lr = 0x83007B8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B8C: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 83007B90: 419A0028  beq cr6, 0x83007bb8
	if ctx.cr[6].eq {
	pc = 0x83007BB8; continue 'dispatch;
	}
	// 83007B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007B98: 4BCAD761  bl 0x82cb52f8
	ctx.lr = 0x83007B9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007B9C: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 83007BA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007BA4: 557C103A  slwi r28, r11, 2
	// 83007BA8: 4BCAD751  bl 0x82cb52f8
	ctx.lr = 0x83007BAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007BAC: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83007BB0: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 83007BB4: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83007BB8: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 83007BBC: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007BC0: 41820030  beq 0x83007bf0
	if ctx.cr[0].eq {
	pc = 0x83007BF0; continue 'dispatch;
	}
	// 83007BC4: 4BCAA495  bl 0x82cb2058
	ctx.lr = 0x83007BC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007BC8: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007BD0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007BD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007BDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007BE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007BE4: 4BCAA335  bl 0x82cb1f18
	ctx.lr = 0x83007BE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83007BE8: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 83007BEC: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 83007BF0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83007BF4: 409A0020  bne cr6, 0x83007c14
	if !ctx.cr[6].eq {
	pc = 0x83007C14; continue 'dispatch;
	}
	// 83007BF8: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83007BFC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83007C00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83007C04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007C08: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 83007C0C: 4E800421  bctrl
	ctx.lr = 0x83007C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83007C10: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83007C14: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007C18: 399F00B0  addi r12, r31, 0xb0
	ctx.r[12].s64 = ctx.r[31].s64 + 176;
	// 83007C1C: 48000031  bl 0x83007c4c
	ctx.lr = 0x83007C20;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007C4C);
	// 83007C20: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83007C24: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83007C28: 4BCA181C  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
}

pub fn sub_83007C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007C2C size=92
	// 83007C2C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83007C30: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83007C34: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83007C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007C3C: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 83007C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007C44: 83DF00CC  lwz r30, 0xcc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 83007C48: 4800001C  b 0x83007c64
	pc = 0x83007C64; continue 'dispatch;
	// 83007C4C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83007C50: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83007C54: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83007C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007C5C: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 83007C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007C68: 4BCA72F9  bl 0x82caef60
	ctx.lr = 0x83007C6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 83007C6C: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007C70: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83007C74: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83007C78: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 83007C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007C80: 4E800020  blr
	return;
}

pub fn sub_83007C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007C88 size=104
	// 83007C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007C90: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 83007C94: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 83007C98: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 83007C9C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 83007CA0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 83007CA4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 83007CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007CAC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83007CB0: 3D4082CC  lis r10, -0x7d34
	ctx.r[10].s64 = -2100559872;
	// 83007CB4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83007CB8: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 83007CBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83007CC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007CC4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83007CC8: 386A8888  addi r3, r10, -0x7778
	ctx.r[3].s64 = ctx.r[10].s64 + -30584;
	// 83007CCC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83007CD0: 4BFFFD91  bl 0x83007a60
	ctx.lr = 0x83007CD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83007A60);
	// 83007CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007CE0: 4E800020  blr
	return;
}

pub fn sub_83007CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007CF0 size=16
	// 83007CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007CF4: 4BCA170D  bl 0x82ca9400
	ctx.lr = 0x83007CF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 83007CF8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83007CFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83007E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007E50 size=48
	// 83007E50: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83007E54: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83007E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007E5C: 9181FFF0  stw r12, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[12].u32 ) };
	// 83007E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007E64: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83007E68: 4BCA70F9  bl 0x82caef60
	ctx.lr = 0x83007E6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 83007E6C: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83007E70: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83007E74: 8181FFF0  lwz r12, -0x10(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 83007E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007E7C: 4E800020  blr
	return;
}

pub fn sub_83007E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007E80 size=88
	// 83007E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007E8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83007E90: 409A0030  bne cr6, 0x83007ec0
	if !ctx.cr[6].eq {
	pc = 0x83007EC0; continue 'dispatch;
	}
	// 83007E94: 4BCAA1C5  bl 0x82cb2058
	ctx.lr = 0x83007E98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007E98: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007EA0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007EA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007EAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007EB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007EB4: 4BCAA065  bl 0x82cb1f18
	ctx.lr = 0x83007EB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83007EB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007EBC: 4800000C  b 0x83007ec8
	pc = 0x83007EC8; continue 'dispatch;
	// 83007EC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007EC4: 556306F6  rlwinm r3, r11, 0, 0x1b, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83007EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007ED4: 4E800020  blr
	return;
}

pub fn sub_83007ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007ED8 size=96
	// 83007ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83007EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007EE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83007EE8: 409A0030  bne cr6, 0x83007f18
	if !ctx.cr[6].eq {
	pc = 0x83007F18; continue 'dispatch;
	}
	// 83007EEC: 4BCAA16D  bl 0x82cb2058
	ctx.lr = 0x83007EF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007EF0: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007EF8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007F00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007F04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007F08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007F0C: 4BCAA00D  bl 0x82cb1f18
	ctx.lr = 0x83007F10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83007F10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007F14: 4800000C  b 0x83007f20
	pc = 0x83007F20; continue 'dispatch;
	// 83007F18: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007F1C: 556306B4  rlwinm r3, r11, 0, 0x1a, 0x1a
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83007F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83007F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83007F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83007F2C: 4E800020  blr
	return;
	// 83007F30: 832B9CA4  lwz r25, -0x635c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25436 as u32) ) } as u64;
	// 83007F34: 8210B2F8  lwz r16, -0x4d08(r16)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(-19720 as u32) ) } as u64;
}

pub fn sub_83007F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83007F38 size=444
	// 83007F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83007F3C: 4BCA14C9  bl 0x82ca9404
	ctx.lr = 0x83007F40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 83007F40: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83007F44: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83007F48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83007F4C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83007F50: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 83007F54: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83007F58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83007F5C: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83007F60: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83007F64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007F68: 409A0030  bne cr6, 0x83007f98
	if !ctx.cr[6].eq {
	pc = 0x83007F98; continue 'dispatch;
	}
	// 83007F6C: 4BCAA0ED  bl 0x82cb2058
	ctx.lr = 0x83007F70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83007F70: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83007F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83007F78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83007F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83007F80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83007F84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83007F88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83007F8C: 4BCA9F8D  bl 0x82cb1f18
	ctx.lr = 0x83007F90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83007F90: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83007F94: 48000158  b 0x830080ec
	pc = 0x830080EC; continue 'dispatch;
	// 83007F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007F9C: 4BCA6F05  bl 0x82caeea0
	ctx.lr = 0x83007FA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEEA0);
	// 83007FA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83007FA4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83007FA8: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83007FAC: 408200F8  bne 0x830080a4
	if !ctx.cr[0].eq {
	pc = 0x830080A4; continue 'dispatch;
	}
	// 83007FB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FB4: 4BCAD345  bl 0x82cb52f8
	ctx.lr = 0x83007FB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007FB8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83007FBC: 419A004C  beq cr6, 0x83008008
	if ctx.cr[6].eq {
	pc = 0x83008008; continue 'dispatch;
	}
	// 83007FC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FC4: 4BCAD335  bl 0x82cb52f8
	ctx.lr = 0x83007FC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007FC8: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 83007FCC: 419A003C  beq cr6, 0x83008008
	if ctx.cr[6].eq {
	pc = 0x83008008; continue 'dispatch;
	}
	// 83007FD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FD4: 4BCAD325  bl 0x82cb52f8
	ctx.lr = 0x83007FD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007FD8: 7C6A2E70  srawi r10, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 83007FDC: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83007FE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83007FE4: 555C103A  slwi r28, r10, 2
	// 83007FE8: 3BAB7760  addi r29, r11, 0x7760
	ctx.r[29].s64 = ctx.r[11].s64 + 30560;
	// 83007FEC: 4BCAD30D  bl 0x82cb52f8
	ctx.lr = 0x83007FF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83007FF0: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83007FF4: 54693572  rlwinm r9, r3, 6, 0x15, 0x19
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 83007FF8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83007FFC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83008000: 3B8BF6E8  addi r28, r11, -0x918
	ctx.r[28].s64 = ctx.r[11].s64 + -2328;
	// 83008004: 48000018  b 0x8300801c
	pc = 0x8300801C; continue 'dispatch;
	// 83008008: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8300800C: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83008010: 3B8AF6E8  addi r28, r10, -0x918
	ctx.r[28].s64 = ctx.r[10].s64 + -2328;
	// 83008014: 3BAB7760  addi r29, r11, 0x7760
	ctx.r[29].s64 = ctx.r[11].s64 + 30560;
	// 83008018: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8300801C: 896A0028  lbz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008020: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008024: 40820054  bne 0x83008078
	if !ctx.cr[0].eq {
	pc = 0x83008078; continue 'dispatch;
	}
	// 83008028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300802C: 4BCAD2CD  bl 0x82cb52f8
	ctx.lr = 0x83008030;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83008030: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83008034: 419A0038  beq cr6, 0x8300806c
	if ctx.cr[6].eq {
	pc = 0x8300806C; continue 'dispatch;
	}
	// 83008038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300803C: 4BCAD2BD  bl 0x82cb52f8
	ctx.lr = 0x83008040;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83008040: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 83008044: 419A0028  beq cr6, 0x8300806c
	if ctx.cr[6].eq {
	pc = 0x8300806C; continue 'dispatch;
	}
	// 83008048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300804C: 4BCAD2AD  bl 0x82cb52f8
	ctx.lr = 0x83008050;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83008050: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 83008054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008058: 557C103A  slwi r28, r11, 2
	// 8300805C: 4BCAD29D  bl 0x82cb52f8
	ctx.lr = 0x83008060;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83008060: 7D5CE82E  lwzx r10, r28, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83008064: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 83008068: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8300806C: 897C0028  lbz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008070: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008074: 41820030  beq 0x830080a4
	if ctx.cr[0].eq {
	pc = 0x830080A4; continue 'dispatch;
	}
	// 83008078: 4BCA9FE1  bl 0x82cb2058
	ctx.lr = 0x8300807C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300807C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83008080: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008084: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008088: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300808C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83008090: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83008094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008098: 4BCA9E81  bl 0x82cb1f18
	ctx.lr = 0x8300809C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 8300809C: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 830080A0: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 830080A4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830080A8: 409A0034  bne cr6, 0x830080dc
	if !ctx.cr[6].eq {
	pc = 0x830080DC; continue 'dispatch;
	}
	// 830080AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830080B0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830080B4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830080B8: 41800018  blt 0x830080d0
	if ctx.cr[0].lt {
	pc = 0x830080D0; continue 'dispatch;
	}
	// 830080BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830080C0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 830080C4: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830080C8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830080CC: 4800000C  b 0x830080d8
	pc = 0x830080D8; continue 'dispatch;
	// 830080D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830080D4: 4BCB40D5  bl 0x82cbc1a8
	ctx.lr = 0x830080D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC1A8);
	// 830080D8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830080DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830080E0: 399F0090  addi r12, r31, 0x90
	ctx.r[12].s64 = ctx.r[31].s64 + 144;
	// 830080E4: 48000031  bl 0x83008114
	ctx.lr = 0x830080E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83008114);
	// 830080E8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830080EC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830080F0: 4BCA1364  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_830080F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830080F4 size=100
	// 830080F4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830080F8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830080FC: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83008100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008104: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 83008108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300810C: 83DF00A4  lwz r30, 0xa4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008110: 4800001C  b 0x8300812c
	pc = 0x8300812C; continue 'dispatch;
	// 83008114: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83008118: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300811C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83008120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008124: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 83008128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300812C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008130: 4BCA6E31  bl 0x82caef60
	ctx.lr = 0x83008134;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 83008134: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008138: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8300813C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008140: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 83008144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008148: 4E800020  blr
	return;
}

pub fn sub_83008158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008158 size=240
	// 83008158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300815C: 4BCA12AD  bl 0x82ca9408
	ctx.lr = 0x83008160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83008160: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83008164: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008168: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300816C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83008170: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 83008174: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83008178: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300817C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008180: 409A0030  bne cr6, 0x830081b0
	if !ctx.cr[6].eq {
	pc = 0x830081B0; continue 'dispatch;
	}
	// 83008184: 4BCA9ED5  bl 0x82cb2058
	ctx.lr = 0x83008188;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008188: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8300818C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008190: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83008198: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300819C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830081A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830081A4: 4BCA9D75  bl 0x82cb1f18
	ctx.lr = 0x830081A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 830081A8: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 830081AC: 48000094  b 0x83008240
	pc = 0x83008240; continue 'dispatch;
	// 830081B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830081B4: 4BCA6CED  bl 0x82caeea0
	ctx.lr = 0x830081B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEEA0);
	// 830081B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830081BC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830081C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830081C4: 556B0732  rlwinm r11, r11, 0, 0x1c, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830081C8: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830081CC: 4BCAD12D  bl 0x82cb52f8
	ctx.lr = 0x830081D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 830081D0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830081D4: 419A0044  beq cr6, 0x83008218
	if ctx.cr[6].eq {
	pc = 0x83008218; continue 'dispatch;
	}
	// 830081D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830081DC: 4BCAD11D  bl 0x82cb52f8
	ctx.lr = 0x830081E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 830081E0: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 830081E4: 419A0034  beq cr6, 0x83008218
	if ctx.cr[6].eq {
	pc = 0x83008218; continue 'dispatch;
	}
	// 830081E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830081EC: 4BCAD10D  bl 0x82cb52f8
	ctx.lr = 0x830081F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 830081F0: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 830081F4: 3D40834F  lis r10, -0x7cb1
	ctx.r[10].s64 = -2091974656;
	// 830081F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830081FC: 557C103A  slwi r28, r11, 2
	// 83008200: 3BAA7760  addi r29, r10, 0x7760
	ctx.r[29].s64 = ctx.r[10].s64 + 30560;
	// 83008204: 4BCAD0F5  bl 0x82cb52f8
	ctx.lr = 0x83008208;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83008208: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300820C: 546A3572  rlwinm r10, r3, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 83008210: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83008214: 4800000C  b 0x83008220
	pc = 0x83008220; continue 'dispatch;
	// 83008218: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8300821C: 396BF6E8  addi r11, r11, -0x918
	ctx.r[11].s64 = ctx.r[11].s64 + -2328;
	// 83008220: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83008224: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83008228: 554A07FA  rlwinm r10, r10, 0, 0x1f, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8300822C: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 83008230: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83008234: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 83008238: 48000031  bl 0x83008268
	ctx.lr = 0x8300823C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83008268);
	// 8300823C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008240: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83008244: 4BCA1214  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_83008248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008248 size=96
	// 83008248: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8300824C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83008250: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83008254: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008258: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 8300825C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008260: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83008264: 4800001C  b 0x83008280
	pc = 0x83008280; continue 'dispatch;
	// 83008268: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8300826C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83008270: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83008274: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008278: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 8300827C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008284: 4BCA6CDD  bl 0x82caef60
	ctx.lr = 0x83008288;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 83008288: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300828C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83008290: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008294: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 83008298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300829C: 4E800020  blr
	return;
	// 830082A0: 4BFFFEB8  b 0x83008158
	crate::recompiler::externs::call(&mut ctx, base, 0x83008158);
	return;
}

pub fn sub_830082A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830082A8 size=880
	// 830082A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830082AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830082B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830082B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830082B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830082BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830082C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830082C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830082C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830082CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830082D0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830082D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830082D8: 409A0030  bne cr6, 0x83008308
	if !ctx.cr[6].eq {
	pc = 0x83008308; continue 'dispatch;
	}
	// 830082DC: 4BCA9D7D  bl 0x82cb2058
	ctx.lr = 0x830082E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830082E0: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 830082E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830082E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830082EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830082F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830082F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830082F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830082FC: 4BCA9C1D  bl 0x82cb1f18
	ctx.lr = 0x83008300;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83008300: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83008304: 480002F8  b 0x830085fc
	pc = 0x830085FC; continue 'dispatch;
	// 83008308: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 8300830C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 83008310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008314: 4BCA169D  bl 0x82ca99b0
	ctx.lr = 0x83008318;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA99B0);
	// 83008318: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300831C: 419AFFC0  beq cr6, 0x830082dc
	if ctx.cr[6].eq {
	pc = 0x830082DC; continue 'dispatch;
	}
	// 83008320: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 83008324: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83008328: 4098001C  bge cr6, 0x83008344
	if !ctx.cr[6].lt {
	pc = 0x83008344; continue 'dispatch;
	}
	// 8300832C: 4BCA9D2D  bl 0x82cb2058
	ctx.lr = 0x83008330;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008330: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83008334: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 83008338: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 8300833C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83008340: 480002BC  b 0x830085fc
	pc = 0x830085FC; continue 'dispatch;
	// 83008344: 3D409340  lis r10, -0x6cc0
	ctx.r[10].s64 = -1824522240;
	// 83008348: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8300834C: 614A6FFF  ori r10, r10, 0x6fff
	ctx.r[10].u64 = ctx.r[10].u64 | 28671;
	// 83008350: 792A000E  rldimi r10, r9, 0x20, 0
	ctx.r[10].u64 = ((ctx.r[9].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[10].u64 & 0x00000000FFFFFFFF);
	// 83008354: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83008358: 4199FF84  bgt cr6, 0x830082dc
	if ctx.cr[6].gt {
	pc = 0x830082DC; continue 'dispatch;
	}
	// 8300835C: 4BCB932D  bl 0x82cc1688
	ctx.lr = 0x83008360;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1688);
	// 83008360: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83008364: 4BCB9445  bl 0x82cc17a8
	ctx.lr = 0x83008368;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC17A8);
	// 83008368: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300836C: 4182001C  beq 0x83008388
	if ctx.cr[0].eq {
	pc = 0x83008388; continue 'dispatch;
	}
	// 83008370: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83008378: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300837C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83008380: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008384: 4BCA9BDD  bl 0x82cb1f60
	ctx.lr = 0x83008388;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83008388: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8300838C: 4BCB9485  bl 0x82cc1810
	ctx.lr = 0x83008390;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1810);
	// 83008390: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008394: 4182001C  beq 0x830083b0
	if ctx.cr[0].eq {
	pc = 0x830083B0; continue 'dispatch;
	}
	// 83008398: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300839C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830083A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830083A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830083A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830083AC: 4BCA9BB5  bl 0x82cb1f60
	ctx.lr = 0x830083B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 830083B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830083B4: 4BCB94C5  bl 0x82cc1878
	ctx.lr = 0x830083B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1878);
	// 830083B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830083BC: 4182001C  beq 0x830083d8
	if ctx.cr[0].eq {
	pc = 0x830083D8; continue 'dispatch;
	}
	// 830083C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830083C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830083C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830083CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830083D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830083D4: 4BCA9B8D  bl 0x82cb1f60
	ctx.lr = 0x830083D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 830083D8: 3D400003  lis r10, 3
	ctx.r[10].s64 = 196608;
	// 830083DC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 830083E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830083E4: 614AF480  ori r10, r10, 0xf480
	ctx.r[10].u64 = ctx.r[10].u64 | 62592;
	// 830083E8: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 830083EC: 4099006C  ble cr6, 0x83008458
	if !ctx.cr[6].gt {
	pc = 0x83008458; continue 'dispatch;
	}
	// 830083F0: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 830083F4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830083F8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830083FC: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 83008400: 480005C9  bl 0x830089c8
	ctx.lr = 0x83008404;
	crate::recompiler::externs::call(&mut ctx, base, 0x830089C8);
	// 83008404: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008408: 408201F4  bne 0x830085fc
	if !ctx.cr[0].eq {
	pc = 0x830085FC; continue 'dispatch;
	}
	// 8300840C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008410: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008414: 419A01E4  beq cr6, 0x830085f8
	if ctx.cr[6].eq {
	pc = 0x830085F8; continue 'dispatch;
	}
	// 83008418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300841C: 4BCB930D  bl 0x82cc1728
	ctx.lr = 0x83008420;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1728);
	// 83008420: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008424: 418201D4  beq 0x830085f8
	if ctx.cr[0].eq {
	pc = 0x830085F8; continue 'dispatch;
	}
	// 83008428: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8300842C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83008430: E961005A  lwa r11, 0x58(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 83008434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008438: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8300843C: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 83008440: 48000589  bl 0x830089c8
	ctx.lr = 0x83008444;
	crate::recompiler::externs::call(&mut ctx, base, 0x830089C8);
	// 83008444: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008448: 408201B4  bne 0x830085fc
	if !ctx.cr[0].eq {
	pc = 0x830085FC; continue 'dispatch;
	}
	// 8300844C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83008450: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83008454: 480001A4  b 0x830085f8
	pc = 0x830085F8; continue 'dispatch;
	// 83008458: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300845C: 4800056D  bl 0x830089c8
	ctx.lr = 0x83008460;
	crate::recompiler::externs::call(&mut ctx, base, 0x830089C8);
	// 83008460: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008464: 40820198  bne 0x830085fc
	if !ctx.cr[0].eq {
	pc = 0x830085FC; continue 'dispatch;
	}
	// 83008468: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300846C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008470: 419A0038  beq cr6, 0x830084a8
	if ctx.cr[6].eq {
	pc = 0x830084A8; continue 'dispatch;
	}
	// 83008474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008478: 4BCB92B1  bl 0x82cc1728
	ctx.lr = 0x8300847C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1728);
	// 8300847C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008480: 41820028  beq 0x830084a8
	if ctx.cr[0].eq {
	pc = 0x830084A8; continue 'dispatch;
	}
	// 83008484: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83008488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8300848C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008490: E91F0002  lwa r8, 0(r31)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 83008494: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83008498: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8300849C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 830084A0: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 830084A4: 48000010  b 0x830084b4
	pc = 0x830084B4; continue 'dispatch;
	// 830084A8: E97F0002  lwa r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 830084AC: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 830084B0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830084B4: 3920003C  li r9, 0x3c
	ctx.r[9].s64 = 60;
	// 830084B8: 7D4B4BD2  divd r10, r11, r9
	ctx.r[10].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 830084BC: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 830084C0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830084C4: 7D4A07B5  extsw. r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830084C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830084CC: 40800010  bge 0x830084dc
	if !ctx.cr[0].lt {
	pc = 0x830084DC; continue 'dispatch;
	}
	// 830084D0: 394A003C  addi r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 + 60;
	// 830084D4: 396BFFC4  addi r11, r11, -0x3c
	ctx.r[11].s64 = ctx.r[11].s64 + -60;
	// 830084D8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830084DC: E91F0006  lwa r8, 4(r31)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as i32) as i64;
	// 830084E0: 7D4B4BD2  divd r10, r11, r9
	ctx.r[10].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 830084E4: 7D6A4214  add r11, r10, r8
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 830084E8: 7D4B4BD2  divd r10, r11, r9
	ctx.r[10].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 830084EC: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 830084F0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830084F4: 7D4A07B5  extsw. r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830084F8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830084FC: 40800010  bge 0x8300850c
	if !ctx.cr[0].lt {
	pc = 0x8300850C; continue 'dispatch;
	}
	// 83008500: 394A003C  addi r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 + 60;
	// 83008504: 396BFFC4  addi r11, r11, -0x3c
	ctx.r[11].s64 = ctx.r[11].s64 + -60;
	// 83008508: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300850C: E95F000A  lwa r10, 8(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as i32) as i64;
	// 83008510: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 83008514: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83008518: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 8300851C: 7D4B43D2  divd r10, r11, r8
	ctx.r[10].s64 = ctx.r[11].s64 / ctx.r[8].s64;
	// 83008520: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 83008524: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83008528: 7D4A07B5  extsw. r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300852C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83008530: 40800010  bge 0x83008540
	if !ctx.cr[0].lt {
	pc = 0x83008540; continue 'dispatch;
	}
	// 83008534: 394A0018  addi r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 + 24;
	// 83008538: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 8300853C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83008540: 7D6B43D2  divd r11, r11, r8
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[8].s64;
	// 83008544: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 83008548: 40990040  ble cr6, 0x83008588
	if !ctx.cr[6].gt {
	pc = 0x83008588; continue 'dispatch;
	}
	// 8300854C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83008550: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83008554: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 83008558: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300855C: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83008560: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83008564: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83008568: 7D0743D6  divw r8, r7, r8
	ctx.r[8].s32 = ctx.r[7].s32 / ctx.r[8].s32;
	// 8300856C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83008570: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83008574: 1D480007  mulli r10, r8, 7
	ctx.r[10].s64 = ctx.r[8].s64 * 7;
	// 83008578: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8300857C: 7D6A3850  subf r11, r10, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 83008580: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83008584: 48000074  b 0x830085f8
	pc = 0x830085F8; continue 'dispatch;
	// 83008588: 40980070  bge cr6, 0x830085f8
	if !ctx.cr[6].lt {
	pc = 0x830085F8; continue 'dispatch;
	}
	// 8300858C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83008590: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83008594: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 83008598: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8300859C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830085A0: 39290007  addi r9, r9, 7
	ctx.r[9].s64 = ctx.r[9].s64 + 7;
	// 830085A4: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830085A8: 7D0943D6  divw r8, r9, r8
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[8].s32;
	// 830085AC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830085B0: 1D080007  mulli r8, r8, 7
	ctx.r[8].s64 = ctx.r[8].s64 * 7;
	// 830085B4: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 830085B8: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 830085BC: 41810030  bgt 0x830085ec
	if ctx.cr[0].gt {
	pc = 0x830085EC; continue 'dispatch;
	}
	// 830085C0: 390A001F  addi r8, r10, 0x1f
	ctx.r[8].s64 = ctx.r[10].s64 + 31;
	// 830085C4: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830085C8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830085CC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 830085D0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830085D4: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830085D8: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 830085DC: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 830085E0: 396B016D  addi r11, r11, 0x16d
	ctx.r[11].s64 = ctx.r[11].s64 + 365;
	// 830085E4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830085E8: 4800000C  b 0x830085f4
	pc = 0x830085F4; continue 'dispatch;
	// 830085EC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830085F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830085F4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830085F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830085FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83008600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008608: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300860C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008610: 4E800020  blr
	return;
}

pub fn sub_83008618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008618 size=20
	// 83008618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300861C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83008678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008678 size=88
	// 83008678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300867C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008684: 2F240000  cmpdi cr6, r4, 0
	ctx.cr[6].compare_i64(ctx.r[4].s64, 0, &mut ctx.xer);
	// 83008688: 41980020  blt cr6, 0x830086a8
	if ctx.cr[6].lt {
	pc = 0x830086A8; continue 'dispatch;
	}
	// 8300868C: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 83008690: 41980018  blt cr6, 0x830086a8
	if ctx.cr[6].lt {
	pc = 0x830086A8; continue 'dispatch;
	}
	// 83008694: 7D641850  subf r11, r4, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 83008698: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8300869C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 830086A0: FC20069C  fcfid f1, f0
	ctx.f[1].f64 = (ctx.f[0].s64 as f64);
	// 830086A4: 48000018  b 0x830086bc
	pc = 0x830086BC; continue 'dispatch;
	// 830086A8: 4BCA99B1  bl 0x82cb2058
	ctx.lr = 0x830086AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830086AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830086B0: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 830086B4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830086B8: C82B0D38  lfd f1, 0xd38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3384 as u32) ) };
	// 830086BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830086C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830086C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830086C8: 4E800020  blr
	return;
}

pub fn sub_830086D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830086D0 size=80
	// 830086D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830086D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830086D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830086DC: 48003325  bl 0x8300ba00
	ctx.lr = 0x830086E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300BA00);
	// 830086E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830086E4: 4082000C  bne 0x830086f0
	if !ctx.cr[0].eq {
	pc = 0x830086F0; continue 'dispatch;
	}
	// 830086E8: 4BCBE8C9  bl 0x82cc6fb0
	ctx.lr = 0x830086EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 830086EC: 48000008  b 0x830086f4
	pc = 0x830086F4; continue 'dispatch;
	// 830086F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830086F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830086F8: 419A0010  beq cr6, 0x83008708
	if ctx.cr[6].eq {
	pc = 0x83008708; continue 'dispatch;
	}
	// 830086FC: 4B237E05  bl 0x82240500
	ctx.lr = 0x83008700;
	crate::recompiler::externs::call(&mut ctx, base, 0x82240500);
	// 83008700: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83008704: 48000008  b 0x8300870c
	pc = 0x8300870C; continue 'dispatch;
	// 83008708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300870C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008718: 4E800020  blr
	return;
}

pub fn sub_83008720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83008720 size=680
	// 83008720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008724: 4BCA0CE5  bl 0x82ca9408
	ctx.lr = 0x83008728;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83008728: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300872C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83008730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008734: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83008738: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8300873C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83008740: 409A002C  bne cr6, 0x8300876c
	if !ctx.cr[6].eq {
	pc = 0x8300876C; continue 'dispatch;
	}
	// 83008744: 4BCA9915  bl 0x82cb2058
	ctx.lr = 0x83008748;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008748: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8300874C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008750: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83008758: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300875C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83008760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008764: 4BCA97B5  bl 0x82cb1f18
	ctx.lr = 0x83008768;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83008768: 48000248  b 0x830089b0
	pc = 0x830089B0; continue 'dispatch;
	// 8300876C: E97F0016  lwa r11, 0x14(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as i32) as i64;
	// 83008770: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 83008774: 2F2B0045  cmpdi cr6, r11, 0x45
	ctx.cr[6].compare_i64(ctx.r[11].s64, 69, &mut ctx.xer);
	// 83008778: 41980228  blt cr6, 0x830089a0
	if ctx.cr[6].lt {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 8300877C: 2F2B044D  cmpdi cr6, r11, 0x44d
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1101, &mut ctx.xer);
	// 83008780: 41990220  bgt cr6, 0x830089a0
	if ctx.cr[6].gt {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 83008784: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83008788: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300878C: 4198000C  blt cr6, 0x83008798
	if ctx.cr[6].lt {
	pc = 0x83008798; continue 'dispatch;
	}
	// 83008790: 2F0A000B  cmpwi cr6, r10, 0xb
	ctx.cr[6].compare_i32(ctx.r[10].s32, 11, &mut ctx.xer);
	// 83008794: 4099004C  ble cr6, 0x830087e0
	if !ctx.cr[6].gt {
	pc = 0x830087E0; continue 'dispatch;
	}
	// 83008798: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8300879C: 7D0A4BD6  divw r8, r10, r9
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 830087A0: 7D2A4BD6  divw r9, r10, r9
	ctx.r[9].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 830087A4: 1D08000C  mulli r8, r8, 0xc
	ctx.r[8].s64 = ctx.r[8].s64 * 12;
	// 830087A8: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 830087AC: 7D485051  subf. r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830087B0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 830087B4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830087B8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 830087BC: 40800014  bge 0x830087d0
	if !ctx.cr[0].lt {
	pc = 0x830087D0; continue 'dispatch;
	}
	// 830087C0: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 830087C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830087C8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830087CC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 830087D0: 2F2B0045  cmpdi cr6, r11, 0x45
	ctx.cr[6].compare_i64(ctx.r[11].s64, 69, &mut ctx.xer);
	// 830087D4: 419801CC  blt cr6, 0x830089a0
	if ctx.cr[6].lt {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 830087D8: 2F2B044D  cmpdi cr6, r11, 0x44d
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1101, &mut ctx.xer);
	// 830087DC: 419901C4  bgt cr6, 0x830089a0
	if ctx.cr[6].gt {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 830087E0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830087E4: 3D20832F  lis r9, -0x7cd1
	ctx.r[9].s64 = -2094071808;
	// 830087E8: 7D681674  sradi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 2) - 1)) != 0);
	ctx.r[8].s64 = ctx.r[11].s64 >> 2;
	// 830087EC: 5547103A  slwi r7, r10, 2
	// 830087F0: 3929FF04  addi r9, r9, -0xfc
	ctx.r[9].s64 = ctx.r[9].s64 + -252;
	// 830087F4: 7CC80194  addze r6, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[6].s64 = tmp.s64;
	// 830087F8: 39000190  li r8, 0x190
	ctx.r[8].s64 = 400;
	// 830087FC: 78C61764  sldi r6, r6, 2
	// 83008800: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 83008804: 7D274AAA  lwax r9, r7, r9
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as i32) as i64;
	// 83008808: 7CE65850  subf r7, r6, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 8300880C: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 83008810: 2F270000  cmpdi cr6, r7, 0
	ctx.cr[6].compare_i64(ctx.r[7].s64, 0, &mut ctx.xer);
	// 83008814: 409A0018  bne cr6, 0x8300882c
	if !ctx.cr[6].eq {
	pc = 0x8300882C; continue 'dispatch;
	}
	// 83008818: 7D2B2BD2  divd r9, r11, r5
	ctx.r[9].s64 = ctx.r[11].s64 / ctx.r[5].s64;
	// 8300881C: 1D290064  mulli r9, r9, 0x64
	ctx.r[9].s64 = ctx.r[9].s64 * 100;
	// 83008820: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83008824: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 83008828: 409A001C  bne cr6, 0x83008844
	if !ctx.cr[6].eq {
	pc = 0x83008844; continue 'dispatch;
	}
	// 8300882C: 392B076C  addi r9, r11, 0x76c
	ctx.r[9].s64 = ctx.r[11].s64 + 1900;
	// 83008830: 7CE943D2  divd r7, r9, r8
	ctx.r[7].s64 = ctx.r[9].s64 / ctx.r[8].s64;
	// 83008834: 1CE70190  mulli r7, r7, 0x190
	ctx.r[7].s64 = ctx.r[7].s64 * 400;
	// 83008838: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 8300883C: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 83008840: 409A0010  bne cr6, 0x83008850
	if !ctx.cr[6].eq {
	pc = 0x83008850; continue 'dispatch;
	}
	// 83008844: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83008848: 40990008  ble cr6, 0x83008850
	if !ctx.cr[6].gt {
	pc = 0x83008850; continue 'dispatch;
	}
	// 8300884C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 83008850: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 83008854: E93F000E  lwa r9, 0xc(r31)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as i32) as i64;
	// 83008858: 38EB012B  addi r7, r11, 0x12b
	ctx.r[7].s64 = ctx.r[11].s64 + 299;
	// 8300885C: E87F000A  lwa r3, 8(r31)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as i32) as i64;
	// 83008860: 7D5E0E74  sradi r30, r10, 1
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 1) - 1)) != 0);
	ctx.r[30].s64 = ctx.r[10].s64 >> 1;
	// 83008864: EBBF0006  lwa r29, 4(r31)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as i32) as i64;
	// 83008868: 7D0743D2  divd r8, r7, r8
	ctx.r[8].s64 = ctx.r[7].s64 / ctx.r[8].s64;
	// 8300886C: EB9F0002  lwa r28, 0(r31)
	ctx.r[28].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 83008870: 7BC717A0  rldicl r7, r30, 2, 0x3e
	ctx.r[7].u64 = ctx.r[30].u64 & 0x3FFFFFFFFFFFFFFFu64;
	// 83008874: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83008878: 7D0A2BD2  divd r8, r10, r5
	ctx.r[8].s64 = ctx.r[10].s64 / ctx.r[5].s64;
	// 8300887C: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 83008880: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83008884: 7D4A1674  sradi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s64 < 0) && ((ctx.r[10].u64 & ((1u64 << 2) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[10].s64 >> 2;
	// 83008888: 1D6B016D  mulli r11, r11, 0x16d
	ctx.r[11].s64 = ctx.r[11].s64 * 365;
	// 8300888C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83008890: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83008894: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83008898: 7D0B3214  add r8, r11, r6
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8300889C: 39089C21  addi r8, r8, -0x63df
	ctx.r[8].s64 = ctx.r[8].s64 + -25567;
	// 830088A0: 1D080018  mulli r8, r8, 0x18
	ctx.r[8].s64 = ctx.r[8].s64 * 24;
	// 830088A4: 7D234214  add r9, r3, r8
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[8].u64;
	// 830088A8: 1D29003C  mulli r9, r9, 0x3c
	ctx.r[9].s64 = ctx.r[9].s64 * 60;
	// 830088AC: 7D5D4A14  add r10, r29, r9
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 830088B0: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 830088B4: 7D7C5214  add r11, r28, r10
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 830088B8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 830088BC: 419A00B8  beq cr6, 0x83008974
	if ctx.cr[6].eq {
	pc = 0x83008974; continue 'dispatch;
	}
	// 830088C0: 4BCB8DC9  bl 0x82cc1688
	ctx.lr = 0x830088C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1688);
	// 830088C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830088C8: 4BCB8F49  bl 0x82cc1810
	ctx.lr = 0x830088CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1810);
	// 830088CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830088D0: 4182001C  beq 0x830088ec
	if ctx.cr[0].eq {
	pc = 0x830088EC; continue 'dispatch;
	}
	// 830088D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830088D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830088DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830088E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830088E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830088E8: 4BCA9679  bl 0x82cb1f60
	ctx.lr = 0x830088EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 830088EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830088F0: 4BCB8F89  bl 0x82cc1878
	ctx.lr = 0x830088F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC1878);
	// 830088F4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830088F8: 4182001C  beq 0x83008914
	if ctx.cr[0].eq {
	pc = 0x83008914; continue 'dispatch;
	}
	// 830088FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83008904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83008908: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300890C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008910: 4BCA9651  bl 0x82cb1f60
	ctx.lr = 0x83008914;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 83008914: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83008918: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8300891C: E9610052  lwa r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 83008920: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83008924: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83008928: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8300892C: 4BFFF97D  bl 0x830082a8
	ctx.lr = 0x83008930;
	crate::recompiler::externs::call(&mut ctx, base, 0x830082A8);
	// 83008930: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008934: 4082006C  bne 0x830089a0
	if !ctx.cr[0].eq {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 83008938: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300893C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008940: 41990014  bgt cr6, 0x83008954
	if ctx.cr[6].gt {
	pc = 0x83008954; continue 'dispatch;
	}
	// 83008944: 40980044  bge cr6, 0x83008988
	if !ctx.cr[6].lt {
	pc = 0x83008988; continue 'dispatch;
	}
	// 83008948: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300894C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008950: 40990038  ble cr6, 0x83008988
	if !ctx.cr[6].gt {
	pc = 0x83008988; continue 'dispatch;
	}
	// 83008954: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83008958: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8300895C: E9610056  lwa r11, 0x54(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as i32) as i64;
	// 83008960: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83008964: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83008968: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8300896C: 4BFFF93D  bl 0x830082a8
	ctx.lr = 0x83008970;
	crate::recompiler::externs::call(&mut ctx, base, 0x830082A8);
	// 83008970: 48000010  b 0x83008980
	pc = 0x83008980; continue 'dispatch;
	// 83008974: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83008978: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300897C: 4800004D  bl 0x830089c8
	ctx.lr = 0x83008980;
	crate::recompiler::externs::call(&mut ctx, base, 0x830089C8);
	// 83008980: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008984: 4082001C  bne 0x830089a0
	if !ctx.cr[0].eq {
	pc = 0x830089A0; continue 'dispatch;
	}
	// 83008988: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8300898C: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 83008990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008994: 4BCA0AED  bl 0x82ca9480
	ctx.lr = 0x83008998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9480);
	// 83008998: E8610058  ld r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8300899C: 48000018  b 0x830089b4
	pc = 0x830089B4; continue 'dispatch;
	// 830089A0: 4BCA96B9  bl 0x82cb2058
	ctx.lr = 0x830089A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830089A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830089A8: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 830089AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830089B0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830089B4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830089B8: 4BCA0AA0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_830089C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830089C8 size=648
	// 830089C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830089CC: 4BCA0A3D  bl 0x82ca9408
	ctx.lr = 0x830089D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 830089D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830089D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830089D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830089DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830089E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830089E4: 409A0030  bne cr6, 0x83008a14
	if !ctx.cr[6].eq {
	pc = 0x83008A14; continue 'dispatch;
	}
	// 830089E8: 4BCA9671  bl 0x82cb2058
	ctx.lr = 0x830089EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830089EC: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 830089F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830089F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830089F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830089FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83008A00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83008A04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008A08: 4BCA9511  bl 0x82cb1f18
	ctx.lr = 0x83008A0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83008A0C: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83008A10: 48000234  b 0x83008c44
	pc = 0x83008C44; continue 'dispatch;
	// 83008A14: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 83008A18: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 83008A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008A20: 4BCA0F91  bl 0x82ca99b0
	ctx.lr = 0x83008A24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA99B0);
	// 83008A24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83008A28: 419AFFC0  beq cr6, 0x830089e8
	if ctx.cr[6].eq {
	pc = 0x830089E8; continue 'dispatch;
	}
	// 83008A2C: E91D0000  ld r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 83008A30: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 83008A34: 4098001C  bge cr6, 0x83008a50
	if !ctx.cr[6].lt {
	pc = 0x83008A50; continue 'dispatch;
	}
	// 83008A38: 4BCA9621  bl 0x82cb2058
	ctx.lr = 0x83008A3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008A3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83008A40: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 83008A44: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83008A48: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83008A4C: 480001F8  b 0x83008c44
	pc = 0x83008C44; continue 'dispatch;
	// 83008A50: 3D609340  lis r11, -0x6cc0
	ctx.r[11].s64 = -1824522240;
	// 83008A54: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 83008A58: 616B6FFF  ori r11, r11, 0x6fff
	ctx.r[11].u64 = ctx.r[11].u64 | 28671;
	// 83008A5C: 794B000E  rldimi r11, r10, 0x20, 0
	ctx.r[11].u64 = ((ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[11].u64 & 0x00000000FFFFFFFF);
	// 83008A60: 7F285800  cmpd cr6, r8, r11
	ctx.cr[6].compare_i64(ctx.r[8].s64, ctx.r[11].s64, &mut ctx.xer);
	// 83008A64: 4199FF84  bgt cr6, 0x830089e8
	if ctx.cr[6].gt {
	pc = 0x830089E8; continue 'dispatch;
	}
	// 83008A68: 3D6001E1  lis r11, 0x1e1
	ctx.r[11].s64 = 31522816;
	// 83008A6C: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 83008A70: 61673380  ori r7, r11, 0x3380
	ctx.r[7].u64 = ctx.r[11].u64 | 13184;
	// 83008A74: 61495180  ori r9, r10, 0x5180
	ctx.r[9].u64 = ctx.r[10].u64 | 20864;
	// 83008A78: 7D683BD2  divd r11, r8, r7
	ctx.r[11].s64 = ctx.r[8].s64 / ctx.r[7].s64;
	// 83008A7C: 38600190  li r3, 0x190
	ctx.r[3].s64 = 400;
	// 83008A80: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83008A84: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 83008A88: 396B0046  addi r11, r11, 0x46
	ctx.r[11].s64 = ctx.r[11].s64 + 70;
	// 83008A8C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 83008A90: 38CB012B  addi r6, r11, 0x12b
	ctx.r[6].s64 = ctx.r[11].s64 + 299;
	// 83008A94: 7CAA23D6  divw r5, r10, r4
	ctx.r[5].s32 = ctx.r[10].s32 / ctx.r[4].s32;
	// 83008A98: 7CC61BD6  divw r6, r6, r3
	ctx.r[6].s32 = ctx.r[6].s32 / ctx.r[3].s32;
	// 83008A9C: 7D5C1670  srawi r28, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 83008AA0: 7CA53050  subf r5, r5, r6
	ctx.r[5].s64 = ctx.r[6].s64 - ctx.r[5].s64;
	// 83008AA4: 7CDC0194  addze r6, r28
	tmp.s64 = ctx.r[28].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[28].u32);
	ctx.r[6].s64 = tmp.s64;
	// 83008AA8: 3B8BFFBA  addi r28, r11, -0x46
	ctx.r[28].s64 = ctx.r[11].s64 + -70;
	// 83008AAC: 7CC53214  add r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 83008AB0: 7F8507B4  extsw r5, r28
	ctx.r[5].s64 = ctx.r[28].s32 as i64;
	// 83008AB4: 3B86FFEF  addi r28, r6, -0x11
	ctx.r[28].s64 = ctx.r[6].s64 + -17;
	// 83008AB8: 1CC5016D  mulli r6, r5, 0x16d
	ctx.r[6].s64 = ctx.r[5].s64 * 365;
	// 83008ABC: 7F8507B4  extsw r5, r28
	ctx.r[5].s64 = ctx.r[28].s32 as i64;
	// 83008AC0: 7CC53214  add r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 83008AC4: 7CC649D2  mulld r6, r6, r9
	ctx.r[6].s64 = ctx.r[6].s64 * ctx.r[9].s64;
	// 83008AC8: 7D064050  subf r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 83008ACC: 2F280000  cmpdi cr6, r8, 0
	ctx.cr[6].compare_i64(ctx.r[8].s64, 0, &mut ctx.xer);
	// 83008AD0: 4098004C  bge cr6, 0x83008b1c
	if !ctx.cr[6].lt {
	pc = 0x83008B1C; continue 'dispatch;
	}
	// 83008AD4: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 83008AD8: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 83008ADC: 7CEB0194  addze r7, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[7].s64 = tmp.s64;
	// 83008AE0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83008AE4: 54E7103A  slwi r7, r7, 2
	// 83008AE8: 7D475051  subf. r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008AEC: 40820014  bne 0x83008b00
	if !ctx.cr[0].eq {
	pc = 0x83008B00; continue 'dispatch;
	}
	// 83008AF0: 7D4B23D6  divw r10, r11, r4
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	// 83008AF4: 1D4A0064  mulli r10, r10, 0x64
	ctx.r[10].s64 = ctx.r[10].s64 * 100;
	// 83008AF8: 7D4A5851  subf. r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008AFC: 40820018  bne 0x83008b14
	if !ctx.cr[0].eq {
	pc = 0x83008B14; continue 'dispatch;
	}
	// 83008B00: 394B076C  addi r10, r11, 0x76c
	ctx.r[10].s64 = ctx.r[11].s64 + 1900;
	// 83008B04: 7CEA1BD6  divw r7, r10, r3
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 83008B08: 1CE70190  mulli r7, r7, 0x190
	ctx.r[7].s64 = ctx.r[7].s64 * 400;
	// 83008B0C: 7D475051  subf. r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008B10: 40820048  bne 0x83008b58
	if !ctx.cr[0].eq {
	pc = 0x83008B58; continue 'dispatch;
	}
	// 83008B14: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83008B18: 4800003C  b 0x83008b54
	pc = 0x83008B54; continue 'dispatch;
	// 83008B1C: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 83008B20: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83008B24: 554A103A  slwi r10, r10, 2
	// 83008B28: 7D4A5851  subf. r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008B2C: 40820014  bne 0x83008b40
	if !ctx.cr[0].eq {
	pc = 0x83008B40; continue 'dispatch;
	}
	// 83008B30: 7D4B23D6  divw r10, r11, r4
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	// 83008B34: 1D4A0064  mulli r10, r10, 0x64
	ctx.r[10].s64 = ctx.r[10].s64 * 100;
	// 83008B38: 7D4A5851  subf. r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008B3C: 40820018  bne 0x83008b54
	if !ctx.cr[0].eq {
	pc = 0x83008B54; continue 'dispatch;
	}
	// 83008B40: 394B076C  addi r10, r11, 0x76c
	ctx.r[10].s64 = ctx.r[11].s64 + 1900;
	// 83008B44: 7CEA1BD6  divw r7, r10, r3
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 83008B48: 1CE70190  mulli r7, r7, 0x190
	ctx.r[7].s64 = ctx.r[7].s64 * 400;
	// 83008B4C: 7D475051  subf. r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83008B50: 40820008  bne 0x83008b58
	if !ctx.cr[0].eq {
	pc = 0x83008B58; continue 'dispatch;
	}
	// 83008B54: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83008B58: 7D484BD2  divd r10, r8, r9
	ctx.r[10].s64 = ctx.r[8].s64 / ctx.r[9].s64;
	// 83008B5C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83008B60: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83008B64: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 83008B68: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83008B6C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83008B70: 7D6A49D2  mulld r11, r10, r9
	ctx.r[11].s64 = ctx.r[10].s64 * ctx.r[9].s64;
	// 83008B74: 7CCB4050  subf r6, r11, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 83008B78: 419A0010  beq cr6, 0x83008b88
	if ctx.cr[6].eq {
	pc = 0x83008B88; continue 'dispatch;
	}
	// 83008B7C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83008B80: 38EBFED0  addi r7, r11, -0x130
	ctx.r[7].s64 = ctx.r[11].s64 + -304;
	// 83008B84: 4800000C  b 0x83008b90
	pc = 0x83008B90; continue 'dispatch;
	// 83008B88: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83008B8C: 38EBFF04  addi r7, r11, -0xfc
	ctx.r[7].s64 = ctx.r[11].s64 + -252;
	// 83008B90: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83008B94: 39470004  addi r10, r7, 4
	ctx.r[10].s64 = ctx.r[7].s64 + 4;
	// 83008B98: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 83008B9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83008BA0: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83008BA4: 4098001C  bge cr6, 0x83008bc0
	if !ctx.cr[6].lt {
	pc = 0x83008BC0; continue 'dispatch;
	}
	// 83008BA8: 5505003E  slwi r5, r8, 0
	// 83008BAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83008BB0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83008BB4: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008BB8: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83008BBC: 4198FFF0  blt cr6, 0x83008bac
	if ctx.cr[6].lt {
	pc = 0x83008BAC; continue 'dispatch;
	}
	// 83008BC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83008BC4: 39400E10  li r10, 0xe10
	ctx.r[10].s64 = 3600;
	// 83008BC8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83008BCC: 556B103A  slwi r11, r11, 2
	// 83008BD0: 7D4653D2  divd r10, r6, r10
	ctx.r[10].s64 = ctx.r[6].s64 / ctx.r[10].s64;
	// 83008BD4: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 83008BD8: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 83008BDC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83008BE0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83008BE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83008BE8: 1C630E10  mulli r3, r3, 0xe10
	ctx.r[3].s64 = ctx.r[3].s64 * 3600;
	// 83008BEC: 7CC33050  subf r6, r3, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[3].s64;
	// 83008BF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008BF4: 7CA62BD2  divd r5, r6, r5
	ctx.r[5].s64 = ctx.r[6].s64 / ctx.r[5].s64;
	// 83008BF8: 7CC607B4  extsw r6, r6
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 83008BFC: 7CA507B4  extsw r5, r5
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 83008C00: 7D6B382E  lwzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 83008C04: 1CE5003C  mulli r7, r5, 0x3c
	ctx.r[7].s64 = ctx.r[5].s64 * 60;
	// 83008C08: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 83008C0C: 7D073050  subf r8, r7, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 83008C10: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83008C14: E97D0000  ld r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 83008C18: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 83008C1C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83008C20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83008C24: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83008C28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83008C2C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83008C30: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83008C34: 7D4B23D6  divw r10, r11, r4
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[4].s32;
	// 83008C38: 1D4A0007  mulli r10, r10, 7
	ctx.r[10].s64 = ctx.r[10].s64 * 7;
	// 83008C3C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83008C40: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83008C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83008C48: 4BCA0810  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_83008C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008C50 size=20
	// 83008C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83008CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008CB0 size=208
	// 83008CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008CBC: 4BCBEAA5  bl 0x82cc7760
	ctx.lr = 0x83008CC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7760);
	// 83008CC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008CC4: 4082000C  bne 0x83008cd0
	if !ctx.cr[0].eq {
	pc = 0x83008CD0; continue 'dispatch;
	}
	// 83008CC8: 4BCBE2E9  bl 0x82cc6fb0
	ctx.lr = 0x83008CCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 83008CCC: 48000008  b 0x83008cd4
	pc = 0x83008CD4; continue 'dispatch;
	// 83008CD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008CD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83008CD8: 419A0010  beq cr6, 0x83008ce8
	if ctx.cr[6].eq {
	pc = 0x83008CE8; continue 'dispatch;
	}
	// 83008CDC: 4B237825  bl 0x82240500
	ctx.lr = 0x83008CE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82240500);
	// 83008CE0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83008CE4: 48000008  b 0x83008cec
	pc = 0x83008CEC; continue 'dispatch;
	// 83008CE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008CEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008CF8: 4E800020  blr
	return;
}

pub fn sub_83008D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008D80 size=1056
	// 83008D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008D84: 4BCA0671  bl 0x82ca93f4
	ctx.lr = 0x83008D88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
	// 83008D88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008D8C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 83008D90: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83008D94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83008D98: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83008D9C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83008DA0: 419A0008  beq cr6, 0x83008da8
	if ctx.cr[6].eq {
	pc = 0x83008DA8; continue 'dispatch;
	}
	// 83008DA4: 93370000  stw r25, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 83008DA8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83008DAC: 409A002C  bne cr6, 0x83008dd8
	if !ctx.cr[6].eq {
	pc = 0x83008DD8; continue 'dispatch;
	}
	// 83008DB0: 4BCA92A9  bl 0x82cb2058
	ctx.lr = 0x83008DB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008DB4: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83008DB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83008DBC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008DC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83008DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83008DC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83008DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83008DD0: 4BCA9149  bl 0x82cb1f18
	ctx.lr = 0x83008DD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83008DD4: 48000278  b 0x8300904c
	pc = 0x8300904C; continue 'dispatch;
	// 83008DD8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83008DDC: 419A0014  beq cr6, 0x83008df0
	if ctx.cr[6].eq {
	pc = 0x83008DF0; continue 'dispatch;
	}
	// 83008DE0: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 83008DE4: 4198FFCC  blt cr6, 0x83008db0
	if ctx.cr[6].lt {
	pc = 0x83008DB0; continue 'dispatch;
	}
	// 83008DE8: 2F1A0024  cmpwi cr6, r26, 0x24
	ctx.cr[6].compare_i32(ctx.r[26].s32, 36, &mut ctx.xer);
	// 83008DEC: 4199FFC4  bgt cr6, 0x83008db0
	if ctx.cr[6].gt {
	pc = 0x83008DB0; continue 'dispatch;
	}
	// 83008DF0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83008DF4: A3F90000  lhz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008DF8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83008DFC: 3BCBF3F0  addi r30, r11, -0xc10
	ctx.r[30].s64 = ctx.r[11].s64 + -3088;
	// 83008E00: 3BB90002  addi r29, r25, 2
	ctx.r[29].s64 = ctx.r[25].s64 + 2;
	// 83008E04: 4800000C  b 0x83008e10
	pc = 0x83008E10; continue 'dispatch;
	// 83008E08: A3FD0000  lhz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008E0C: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 83008E10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83008E14: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83008E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008E1C: 4BCAED15  bl 0x82cb7b30
	ctx.lr = 0x83008E20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB7B30);
	// 83008E20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008E24: 4082FFE4  bne 0x83008e08
	if !ctx.cr[0].eq {
	pc = 0x83008E08; continue 'dispatch;
	}
	// 83008E28: 57EB043E  clrlwi r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 83008E2C: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83008E30: 409A000C  bne cr6, 0x83008e3c
	if !ctx.cr[6].eq {
	pc = 0x83008E3C; continue 'dispatch;
	}
	// 83008E34: 63180002  ori r24, r24, 2
	ctx.r[24].u64 = ctx.r[24].u64 | 2;
	// 83008E38: 4800000C  b 0x83008e44
	pc = 0x83008E44; continue 'dispatch;
	// 83008E3C: 2B0B002B  cmplwi cr6, r11, 0x2b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 43 as u32, &mut ctx.xer);
	// 83008E40: 409A000C  bne cr6, 0x83008e4c
	if !ctx.cr[6].eq {
	pc = 0x83008E4C; continue 'dispatch;
	}
	// 83008E44: A3FD0000  lhz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008E48: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 83008E4C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83008E50: 419801F0  blt cr6, 0x83009040
	if ctx.cr[6].lt {
	pc = 0x83009040; continue 'dispatch;
	}
	// 83008E54: 2F1A0001  cmpwi cr6, r26, 1
	ctx.cr[6].compare_i32(ctx.r[26].s32, 1, &mut ctx.xer);
	// 83008E58: 419A01E8  beq cr6, 0x83009040
	if ctx.cr[6].eq {
	pc = 0x83009040; continue 'dispatch;
	}
	// 83008E5C: 2F1A0024  cmpwi cr6, r26, 0x24
	ctx.cr[6].compare_i32(ctx.r[26].s32, 36, &mut ctx.xer);
	// 83008E60: 419901E0  bgt cr6, 0x83009040
	if ctx.cr[6].gt {
	pc = 0x83009040; continue 'dispatch;
	}
	// 83008E64: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83008E68: 409A003C  bne cr6, 0x83008ea4
	if !ctx.cr[6].eq {
	pc = 0x83008EA4; continue 'dispatch;
	}
	// 83008E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008E70: 4BCAEB01  bl 0x82cb7970
	ctx.lr = 0x83008E74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB7970);
	// 83008E74: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008E78: 4182000C  beq 0x83008e84
	if ctx.cr[0].eq {
	pc = 0x83008E84; continue 'dispatch;
	}
	// 83008E7C: 3B40000A  li r26, 0xa
	ctx.r[26].s64 = 10;
	// 83008E80: 4800005C  b 0x83008edc
	pc = 0x83008EDC; continue 'dispatch;
	// 83008E84: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008E88: 2B0B0078  cmplwi cr6, r11, 0x78
	ctx.cr[6].compare_u32(ctx.r[11].u32, 120 as u32, &mut ctx.xer);
	// 83008E8C: 419A0014  beq cr6, 0x83008ea0
	if ctx.cr[6].eq {
	pc = 0x83008EA0; continue 'dispatch;
	}
	// 83008E90: 2B0B0058  cmplwi cr6, r11, 0x58
	ctx.cr[6].compare_u32(ctx.r[11].u32, 88 as u32, &mut ctx.xer);
	// 83008E94: 419A000C  beq cr6, 0x83008ea0
	if ctx.cr[6].eq {
	pc = 0x83008EA0; continue 'dispatch;
	}
	// 83008E98: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 83008E9C: 48000040  b 0x83008edc
	pc = 0x83008EDC; continue 'dispatch;
	// 83008EA0: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 83008EA4: 2F1A0010  cmpwi cr6, r26, 0x10
	ctx.cr[6].compare_i32(ctx.r[26].s32, 16, &mut ctx.xer);
	// 83008EA8: 409A0034  bne cr6, 0x83008edc
	if !ctx.cr[6].eq {
	pc = 0x83008EDC; continue 'dispatch;
	}
	// 83008EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008EB0: 4BCAEAC1  bl 0x82cb7970
	ctx.lr = 0x83008EB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB7970);
	// 83008EB4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83008EB8: 40820024  bne 0x83008edc
	if !ctx.cr[0].eq {
	pc = 0x83008EDC; continue 'dispatch;
	}
	// 83008EBC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008EC0: 2B0B0078  cmplwi cr6, r11, 0x78
	ctx.cr[6].compare_u32(ctx.r[11].u32, 120 as u32, &mut ctx.xer);
	// 83008EC4: 419A000C  beq cr6, 0x83008ed0
	if ctx.cr[6].eq {
	pc = 0x83008ED0; continue 'dispatch;
	}
	// 83008EC8: 2B0B0058  cmplwi cr6, r11, 0x58
	ctx.cr[6].compare_u32(ctx.r[11].u32, 88 as u32, &mut ctx.xer);
	// 83008ECC: 409A0010  bne cr6, 0x83008edc
	if !ctx.cr[6].eq {
	pc = 0x83008EDC; continue 'dispatch;
	}
	// 83008ED0: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83008ED4: A3FD0002  lhz r31, 2(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 83008ED8: 3BAB0002  addi r29, r11, 2
	ctx.r[29].s64 = ctx.r[11].s64 + 2;
	// 83008EDC: 7F5E07B4  extsw r30, r26
	ctx.r[30].s64 = ctx.r[26].s32 as i64;
	// 83008EE0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83008EE4: 08DE0000  tdi 6, r30, 0
	// 83008EE8: 7F8BF392  divdu r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 / ctx.r[30].u64;
	// 83008EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008EF0: 4BCAEA81  bl 0x82cb7970
	ctx.lr = 0x83008EF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB7970);
	// 83008EF4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83008EF8: 409A0040  bne cr6, 0x83008f38
	if !ctx.cr[6].eq {
	pc = 0x83008F38; continue 'dispatch;
	}
	// 83008EFC: 57EB043E  clrlwi r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 83008F00: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83008F04: 4198000C  blt cr6, 0x83008f10
	if ctx.cr[6].lt {
	pc = 0x83008F10; continue 'dispatch;
	}
	// 83008F08: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 83008F0C: 40990014  ble cr6, 0x83008f20
	if !ctx.cr[6].gt {
	pc = 0x83008F20; continue 'dispatch;
	}
	// 83008F10: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 83008F14: 41980068  blt cr6, 0x83008f7c
	if ctx.cr[6].lt {
	pc = 0x83008F7C; continue 'dispatch;
	}
	// 83008F18: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 83008F1C: 41990060  bgt cr6, 0x83008f7c
	if ctx.cr[6].gt {
	pc = 0x83008F7C; continue 'dispatch;
	}
	// 83008F20: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 83008F24: 41980010  blt cr6, 0x83008f34
	if ctx.cr[6].lt {
	pc = 0x83008F34; continue 'dispatch;
	}
	// 83008F28: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 83008F2C: 41990008  bgt cr6, 0x83008f34
	if ctx.cr[6].gt {
	pc = 0x83008F34; continue 'dispatch;
	}
	// 83008F30: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83008F34: 386BFFC9  addi r3, r11, -0x37
	ctx.r[3].s64 = ctx.r[11].s64 + -55;
	// 83008F38: 7F03D040  cmplw cr6, r3, r26
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[26].u32, &mut ctx.xer);
	// 83008F3C: 40980040  bge cr6, 0x83008f7c
	if !ctx.cr[6].lt {
	pc = 0x83008F7C; continue 'dispatch;
	}
	// 83008F40: 63180008  ori r24, r24, 8
	ctx.r[24].u64 = ctx.r[24].u64 | 8;
	// 83008F44: 7F3BE040  cmpld cr6, r27, r28
	ctx.cr[6].compare_u64(ctx.r[27].u64, ctx.r[28].u64, &mut ctx.xer);
	// 83008F48: 41980054  blt cr6, 0x83008f9c
	if ctx.cr[6].lt {
	pc = 0x83008F9C; continue 'dispatch;
	}
	// 83008F4C: 409A0024  bne cr6, 0x83008f70
	if !ctx.cr[6].eq {
	pc = 0x83008F70; continue 'dispatch;
	}
	// 83008F50: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83008F54: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 83008F58: 7D2BF392  divdu r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 / ctx.r[30].u64;
	// 83008F5C: 08DE0000  tdi 6, r30, 0
	// 83008F60: 7D29F1D2  mulld r9, r9, r30
	ctx.r[9].s64 = ctx.r[9].s64 * ctx.r[30].s64;
	// 83008F64: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83008F68: 7F2A5840  cmpld cr6, r10, r11
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[11].u64, &mut ctx.xer);
	// 83008F6C: 40990030  ble cr6, 0x83008f9c
	if !ctx.cr[6].gt {
	pc = 0x83008F9C; continue 'dispatch;
	}
	// 83008F70: 63180004  ori r24, r24, 4
	ctx.r[24].u64 = ctx.r[24].u64 | 4;
	// 83008F74: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83008F78: 409A0030  bne cr6, 0x83008fa8
	if !ctx.cr[6].eq {
	pc = 0x83008FA8; continue 'dispatch;
	}
	// 83008F7C: 570B0739  rlwinm. r11, r24, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008F80: 3BBDFFFE  addi r29, r29, -2
	ctx.r[29].s64 = ctx.r[29].s64 + -2;
	// 83008F84: 40820030  bne 0x83008fb4
	if !ctx.cr[0].eq {
	pc = 0x83008FB4; continue 'dispatch;
	}
	// 83008F88: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83008F8C: 419A0008  beq cr6, 0x83008f94
	if ctx.cr[6].eq {
	pc = 0x83008F94; continue 'dispatch;
	}
	// 83008F90: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 83008F94: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83008F98: 48000088  b 0x83009020
	pc = 0x83009020; continue 'dispatch;
	// 83008F9C: 7D5ED9D2  mulld r10, r30, r27
	ctx.r[10].s64 = ctx.r[30].s64 * ctx.r[27].s64;
	// 83008FA0: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 83008FA4: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83008FA8: A3FD0000  lhz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008FAC: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 83008FB0: 4BFFFF3C  b 0x83008eec
	pc = 0x83008EEC; continue 'dispatch;
	// 83008FB4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83008FB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83008FBC: 570B077B  rlwinm. r11, r24, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008FC0: 795F0040  clrldi r31, r10, 1
	ctx.r[31].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 83008FC4: 793EFFE6  rldicr r30, r9, 0x3f, 0x3f
	ctx.r[30].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 83008FC8: 4082002C  bne 0x83008ff4
	if !ctx.cr[0].eq {
	pc = 0x83008FF4; continue 'dispatch;
	}
	// 83008FCC: 570B07FF  clrlwi. r11, r24, 0x1f
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008FD0: 40820050  bne 0x83009020
	if !ctx.cr[0].eq {
	pc = 0x83009020; continue 'dispatch;
	}
	// 83008FD4: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008FD8: 4182000C  beq 0x83008fe4
	if ctx.cr[0].eq {
	pc = 0x83008FE4; continue 'dispatch;
	}
	// 83008FDC: 7F3BF040  cmpld cr6, r27, r30
	ctx.cr[6].compare_u64(ctx.r[27].u64, ctx.r[30].u64, &mut ctx.xer);
	// 83008FE0: 41990014  bgt cr6, 0x83008ff4
	if ctx.cr[6].gt {
	pc = 0x83008FF4; continue 'dispatch;
	}
	// 83008FE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83008FE8: 409A0038  bne cr6, 0x83009020
	if !ctx.cr[6].eq {
	pc = 0x83009020; continue 'dispatch;
	}
	// 83008FEC: 7F3BF840  cmpld cr6, r27, r31
	ctx.cr[6].compare_u64(ctx.r[27].u64, ctx.r[31].u64, &mut ctx.xer);
	// 83008FF0: 40990030  ble cr6, 0x83009020
	if !ctx.cr[6].gt {
	pc = 0x83009020; continue 'dispatch;
	}
	// 83008FF4: 4BCA9065  bl 0x82cb2058
	ctx.lr = 0x83008FF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83008FF8: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 83008FFC: 570A07FF  clrlwi. r10, r24, 0x1f
	ctx.r[10].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83009000: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009004: 4182000C  beq 0x83009010
	if ctx.cr[0].eq {
	pc = 0x83009010; continue 'dispatch;
	}
	// 83009008: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 8300900C: 48000014  b 0x83009020
	pc = 0x83009020; continue 'dispatch;
	// 83009010: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009014: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 83009018: 40820008  bne 0x83009020
	if !ctx.cr[0].eq {
	pc = 0x83009020; continue 'dispatch;
	}
	// 8300901C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 83009020: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83009024: 419A0008  beq cr6, 0x8300902c
	if ctx.cr[6].eq {
	pc = 0x8300902C; continue 'dispatch;
	}
	// 83009028: 93B70000  stw r29, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8300902C: 570B07BD  rlwinm. r11, r24, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009030: 41820008  beq 0x83009038
	if ctx.cr[0].eq {
	pc = 0x83009038; continue 'dispatch;
	}
	// 83009034: 7F7B00D0  neg r27, r27
	ctx.r[27].s64 = -ctx.r[27].s64;
	// 83009038: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300903C: 48000014  b 0x83009050
	pc = 0x83009050; continue 'dispatch;
	// 83009040: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83009044: 419A0008  beq cr6, 0x8300904c
	if ctx.cr[6].eq {
	pc = 0x8300904C; continue 'dispatch;
	}
	// 83009048: 93370000  stw r25, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8300904C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009050: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83009054: 4BCA03F0  b 0x82ca9444
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9444);
	return;
	// 83009058: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300905C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83009060: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83009064: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83009068: 386AF3F0  addi r3, r10, -0xc10
	ctx.r[3].s64 = ctx.r[10].s64 + -3088;
	// 8300906C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009070: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83009074: 4BFFFD0C  b 0x83008d80
	pc = 0x83008D80; continue 'dispatch;
	// 83009078: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300907C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83009080: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83009084: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83009088: 386AF3F0  addi r3, r10, -0xc10
	ctx.r[3].s64 = ctx.r[10].s64 + -3088;
	// 8300908C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83009090: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83009094: 4BFFFCEC  b 0x83008d80
	pc = 0x83008D80; continue 'dispatch;
}

pub fn sub_830091A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830091A0 size=200
	// 830091A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830091A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830091A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830091AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830091B0: 409A0034  bne cr6, 0x830091e4
	if !ctx.cr[6].eq {
	pc = 0x830091E4; continue 'dispatch;
	}
	// 830091B4: 4BCA8EA5  bl 0x82cb2058
	ctx.lr = 0x830091B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830091B8: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 830091BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830091C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830091C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830091C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830091CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830091D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830091D4: 4BCA8D45  bl 0x82cb1f18
	ctx.lr = 0x830091D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 830091D8: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 830091DC: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 830091E0: 48000074  b 0x83009254
	pc = 0x83009254; continue 'dispatch;
	// 830091E4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830091E8: 419AFFCC  beq cr6, 0x830091b4
	if ctx.cr[6].eq {
	pc = 0x830091B4; continue 'dispatch;
	}
	// 830091EC: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830091F0: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830091F4: 41980010  blt cr6, 0x83009204
	if ctx.cr[6].lt {
	pc = 0x83009204; continue 'dispatch;
	}
	// 830091F8: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 830091FC: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 83009200: 40990008  ble cr6, 0x83009208
	if !ctx.cr[6].gt {
	pc = 0x83009208; continue 'dispatch;
	}
	// 83009204: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83009208: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300920C: 5549043E  clrlwi r9, r10, 0x10
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83009210: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83009214: 41980010  blt cr6, 0x83009224
	if ctx.cr[6].lt {
	pc = 0x83009224; continue 'dispatch;
	}
	// 83009218: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 8300921C: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 83009220: 40990008  ble cr6, 0x83009228
	if !ctx.cr[6].gt {
	pc = 0x83009228; continue 'dispatch;
	}
	// 83009224: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83009228: 552B043F  clrlwi. r11, r9, 0x10
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300922C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83009230: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 83009234: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 83009238: 41820010  beq 0x83009248
	if ctx.cr[0].eq {
	pc = 0x83009248; continue 'dispatch;
	}
	// 8300923C: 5548043E  clrlwi r8, r10, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83009240: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83009244: 419AFFA8  beq cr6, 0x830091ec
	if ctx.cr[6].eq {
	pc = 0x830091EC; continue 'dispatch;
	}
	// 83009248: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300924C: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83009250: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83009254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300925C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009260: 4E800020  blr
	return;
}

pub fn sub_83009268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009268 size=16
	// 83009268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300926C: 4BCA019D  bl 0x82ca9408
	ctx.lr = 0x83009270;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83009270: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 83009274: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_830093E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830093E0 size=84
	// 830093E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830093E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830093E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830093EC: 3BE1FFA0  addi r31, r1, -0x60
	ctx.r[31].s64 = ctx.r[1].s64 + -96;
	// 830093F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830093F4: 4BCA9B05  bl 0x82cb2ef8
	ctx.lr = 0x830093F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2EF8);
	// 830093F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830093FC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009400: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009404: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 83009408: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300940C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009410: 4E800421  bctrl
	ctx.lr = 0x83009414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009414: 4BCBDB6D  bl 0x82cc6f80
	ctx.lr = 0x83009418;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6F80);
	// 83009418: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300941C: 4BCA6B5D  bl 0x82caff78
	ctx.lr = 0x83009420;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAFF78);
	// 83009420: 383F0060  addi r1, r31, 0x60
	ctx.r[1].s64 = ctx.r[31].s64 + 96;
	// 83009424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300942C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009430: 4E800020  blr
	return;
}

pub fn sub_83009434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009434 size=52
	// 83009434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300943C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009444: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83009448: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300944C: 4BCA9D65  bl 0x82cb31b0
	ctx.lr = 0x83009450;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB31B0);
	// 83009450: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009454: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300945C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009460: 4E800020  blr
	return;
}

pub fn sub_83009468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009468 size=144
	// 83009468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300947C: 4BCA9895  bl 0x82cb2d10
	ctx.lr = 0x83009480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2D10);
	// 83009480: 4BCA9881  bl 0x82cb2d00
	ctx.lr = 0x83009484;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2D00);
	// 83009484: 4BCA9835  bl 0x82cb2cb8
	ctx.lr = 0x83009488;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2CB8);
	// 83009488: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300948C: 4082002C  bne 0x830094b8
	if !ctx.cr[0].eq {
	pc = 0x830094B8; continue 'dispatch;
	}
	// 83009490: 4BCA9871  bl 0x82cb2d00
	ctx.lr = 0x83009494;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2D00);
	// 83009494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83009498: 4BCA98D1  bl 0x82cb2d68
	ctx.lr = 0x8300949C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2D68);
	// 8300949C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830094A0: 4082000C  bne 0x830094ac
	if !ctx.cr[0].eq {
	pc = 0x830094AC; continue 'dispatch;
	}
	// 830094A4: 4BCBDB0D  bl 0x82cc6fb0
	ctx.lr = 0x830094A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 830094A8: 4BCBDAD9  bl 0x82cc6f80
	ctx.lr = 0x830094AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6F80);
	// 830094AC: 4B21585D  bl 0x8221ed08
	ctx.lr = 0x830094B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221ED08);
	// 830094B0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830094B4: 48000024  b 0x830094d8
	pc = 0x830094D8; continue 'dispatch;
	// 830094B8: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830094BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830094C0: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830094C4: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830094C8: 914B0058  stw r10, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 830094CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830094D0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830094D4: 4BCA9A65  bl 0x82cb2f38
	ctx.lr = 0x830094D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2F38);
	// 830094D8: 4BFFFF09  bl 0x830093e0
	ctx.lr = 0x830094DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x830093E0);
	// 830094DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830094E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830094E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830094E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830094EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830094F0: 4E800020  blr
	return;
}

pub fn sub_830094F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830094F8 size=280
	// 830094F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830094FC: 4BC9FEFD  bl 0x82ca93f8
	ctx.lr = 0x83009500;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 83009500: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009504: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83009508: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300950C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83009510: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83009514: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83009518: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8300951C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83009520: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83009524: 409A002C  bne cr6, 0x83009550
	if !ctx.cr[6].eq {
	pc = 0x83009550; continue 'dispatch;
	}
	// 83009528: 4BCA8B31  bl 0x82cb2058
	ctx.lr = 0x8300952C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300952C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83009530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009534: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009538: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300953C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009548: 4BCA89D1  bl 0x82cb1f18
	ctx.lr = 0x8300954C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 8300954C: 4800008C  b 0x830095d8
	pc = 0x830095D8; continue 'dispatch;
	// 83009550: 4BCA97C1  bl 0x82cb2d10
	ctx.lr = 0x83009554;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2D10);
	// 83009554: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 83009558: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300955C: 4BCA810D  bl 0x82cb1668
	ctx.lr = 0x83009560;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1668);
	// 83009560: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83009564: 4182005C  beq 0x830095c0
	if ctx.cr[0].eq {
	pc = 0x830095C0; continue 'dispatch;
	}
	// 83009568: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300956C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009570: 4BCA9881  bl 0x82cb2df0
	ctx.lr = 0x83009574;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2DF0);
	// 83009574: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83009578: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8300957C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83009580: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 83009584: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83009588: 409A0008  bne cr6, 0x83009590
	if !ctx.cr[6].eq {
	pc = 0x83009590; continue 'dispatch;
	}
	// 8300958C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 83009590: 3D608301  lis r11, -0x7cff
	ctx.r[11].s64 = -2097086464;
	// 83009594: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83009598: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8300959C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830095A0: 38AB9468  addi r5, r11, -0x6b98
	ctx.r[5].s64 = ctx.r[11].s64 + -27544;
	// 830095A4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830095A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830095AC: 4BCBA66D  bl 0x82cc3c18
	ctx.lr = 0x830095B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC3C18);
	// 830095B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830095B4: 40820028  bne 0x830095dc
	if !ctx.cr[0].eq {
	pc = 0x830095DC; continue 'dispatch;
	}
	// 830095B8: 4BCBD9F9  bl 0x82cc6fb0
	ctx.lr = 0x830095BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 830095BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830095C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830095C4: 4BCA2B7D  bl 0x82cac140
	ctx.lr = 0x830095C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAC140);
	// 830095C8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830095CC: 419A000C  beq cr6, 0x830095d8
	if ctx.cr[6].eq {
	pc = 0x830095D8; continue 'dispatch;
	}
	// 830095D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830095D4: 4B236F2D  bl 0x82240500
	ctx.lr = 0x830095D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82240500);
	// 830095D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830095DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830095E0: 4BC9FE68  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
}

pub fn sub_83009610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009610 size=664
	// 83009610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009614: 4BC9FDF1  bl 0x82ca9404
	ctx.lr = 0x83009618;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 83009618: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300961C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83009620: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83009624: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83009628: 409A0038  bne cr6, 0x83009660
	if !ctx.cr[6].eq {
	pc = 0x83009660; continue 'dispatch;
	}
	// 8300962C: 4BCA8A65  bl 0x82cb2090
	ctx.lr = 0x83009630;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2090);
	// 83009630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83009634: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009638: 4BCA8A21  bl 0x82cb2058
	ctx.lr = 0x8300963C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300963C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83009640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009644: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009648: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300964C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009650: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009658: 4BCA88C1  bl 0x82cb1f18
	ctx.lr = 0x8300965C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 8300965C: 48000234  b 0x83009890
	pc = 0x83009890; continue 'dispatch;
	// 83009660: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83009664: 419AFFC8  beq cr6, 0x8300962c
	if ctx.cr[6].eq {
	pc = 0x8300962C; continue 'dispatch;
	}
	// 83009668: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300966C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009670: 388BD698  addi r4, r11, -0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + -10600;
	// 83009674: 48000CF5  bl 0x8300a368
	ctx.lr = 0x83009678;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A368);
	// 83009678: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300967C: 41820020  beq 0x8300969c
	if ctx.cr[0].eq {
	pc = 0x8300969C; continue 'dispatch;
	}
	// 83009680: 4BCA89D9  bl 0x82cb2058
	ctx.lr = 0x83009684;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009684: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83009688: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8300968C: 4BCA8A05  bl 0x82cb2090
	ctx.lr = 0x83009690;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2090);
	// 83009690: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83009694: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83009698: 480001F8  b 0x83009890
	pc = 0x83009890; continue 'dispatch;
	// 8300969C: 897C0001  lbz r11, 1(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(1 as u32) ) } as u64;
	// 830096A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830096A4: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 830096A8: 409A0028  bne cr6, 0x830096d0
	if !ctx.cr[6].eq {
	pc = 0x830096D0; continue 'dispatch;
	}
	// 830096AC: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830096B0: 7D630775  extsb. r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830096B4: 41820010  beq 0x830096c4
	if ctx.cr[0].eq {
	pc = 0x830096C4; continue 'dispatch;
	}
	// 830096B8: 897C0002  lbz r11, 2(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(2 as u32) ) } as u64;
	// 830096BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830096C0: 419AFFC0  beq cr6, 0x83009680
	if ctx.cr[6].eq {
	pc = 0x83009680; continue 'dispatch;
	}
	// 830096C4: 4BCA23AD  bl 0x82caba70
	ctx.lr = 0x830096C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CABA70);
	// 830096C8: 3B63FFA0  addi r27, r3, -0x60
	ctx.r[27].s64 = ctx.r[3].s64 + -96;
	// 830096CC: 48000008  b 0x830096d4
	pc = 0x830096D4; continue 'dispatch;
	// 830096D0: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 830096D4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 830096D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830096DC: 4BCBDD8D  bl 0x82cc7468
	ctx.lr = 0x830096E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7468);
	// 830096E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830096E4: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 830096E8: 419AFF98  beq cr6, 0x83009680
	if ctx.cr[6].eq {
	pc = 0x83009680; continue 'dispatch;
	}
	// 830096EC: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 830096F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830096F4: 409A0018  bne cr6, 0x8300970c
	if !ctx.cr[6].eq {
	pc = 0x8300970C; continue 'dispatch;
	}
	// 830096F8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 830096FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83009700: 409A000C  bne cr6, 0x8300970c
	if !ctx.cr[6].eq {
	pc = 0x8300970C; continue 'dispatch;
	}
	// 83009704: FBDF0020  std r30, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u64 ) };
	// 83009708: 48000050  b 0x83009758
	pc = 0x83009758; continue 'dispatch;
	// 8300970C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83009710: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 83009714: 4BCBE795  bl 0x82cc7ea8
	ctx.lr = 0x83009718;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7EA8);
	// 83009718: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300971C: 41820164  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 83009720: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83009724: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83009728: 4BCBE3C1  bl 0x82cc7ae8
	ctx.lr = 0x8300972C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7AE8);
	// 8300972C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83009730: 41820150  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 83009734: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 83009738: A101005C  lhz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8300973C: A0E1005A  lhz r7, 0x5a(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 83009740: A0C10058  lhz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83009744: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83009748: A0810052  lhz r4, 0x52(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8300974C: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83009750: 4BCB2F11  bl 0x82cbc660
	ctx.lr = 0x83009754;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC660);
	// 83009754: F87F0020  std r3, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 83009758: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300975C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83009760: 409A001C  bne cr6, 0x8300977c
	if !ctx.cr[6].eq {
	pc = 0x8300977C; continue 'dispatch;
	}
	// 83009764: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 83009768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300976C: 409A0010  bne cr6, 0x8300977c
	if !ctx.cr[6].eq {
	pc = 0x8300977C; continue 'dispatch;
	}
	// 83009770: E97F0020  ld r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 83009774: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 83009778: 48000050  b 0x830097c8
	pc = 0x830097C8; continue 'dispatch;
	// 8300977C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83009780: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 83009784: 4BCBE725  bl 0x82cc7ea8
	ctx.lr = 0x83009788;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7EA8);
	// 83009788: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300978C: 418200F4  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 83009790: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83009794: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83009798: 4BCBE351  bl 0x82cc7ae8
	ctx.lr = 0x8300979C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7AE8);
	// 8300979C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830097A0: 418200E0  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 830097A4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830097A8: A101005C  lhz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830097AC: A0E1005A  lhz r7, 0x5a(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 830097B0: A0C10058  lhz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830097B4: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 830097B8: A0810052  lhz r4, 0x52(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 830097BC: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830097C0: 4BCB2EA1  bl 0x82cbc660
	ctx.lr = 0x830097C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC660);
	// 830097C4: F87F0018  std r3, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u64 ) };
	// 830097C8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 830097CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830097D0: 409A001C  bne cr6, 0x830097ec
	if !ctx.cr[6].eq {
	pc = 0x830097EC; continue 'dispatch;
	}
	// 830097D4: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830097D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830097DC: 409A0010  bne cr6, 0x830097ec
	if !ctx.cr[6].eq {
	pc = 0x830097EC; continue 'dispatch;
	}
	// 830097E0: E97F0020  ld r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 830097E4: F97F0028  std r11, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 830097E8: 48000050  b 0x83009838
	pc = 0x83009838; continue 'dispatch;
	// 830097EC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830097F0: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 830097F4: 4BCBE6B5  bl 0x82cc7ea8
	ctx.lr = 0x830097F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7EA8);
	// 830097F8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830097FC: 41820084  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 83009800: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83009804: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83009808: 4BCBE2E1  bl 0x82cc7ae8
	ctx.lr = 0x8300980C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC7AE8);
	// 8300980C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83009810: 41820070  beq 0x83009880
	if ctx.cr[0].eq {
	pc = 0x83009880; continue 'dispatch;
	}
	// 83009814: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 83009818: A101005C  lhz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8300981C: A0E1005A  lhz r7, 0x5a(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 83009820: A0C10058  lhz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83009824: A0A10056  lhz r5, 0x56(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83009828: A0810052  lhz r4, 0x52(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8300982C: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83009830: 4BCB2E31  bl 0x82cbc660
	ctx.lr = 0x83009834;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC660);
	// 83009834: F87F0028  std r3, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u64 ) };
	// 83009838: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300983C: 4BCB8F75  bl 0x82cc27b0
	ctx.lr = 0x83009840;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC27B0);
	// 83009840: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83009844: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 83009848: 480009F1  bl 0x8300a238
	ctx.lr = 0x8300984C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A238);
	// 8300984C: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83009850: 397BFFFF  addi r11, r27, -1
	ctx.r[11].s64 = ctx.r[27].s64 + -1;
	// 83009854: B07F0006  sth r3, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[3].u16 ) };
	// 83009858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8300985C: B3DF0004  sth r30, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 83009860: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009868: B13F0008  sth r9, 8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u16 ) };
	// 8300986C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83009870: B3DF000C  sth r30, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 83009874: B3DF000A  sth r30, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[30].u16 ) };
	// 83009878: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8300987C: 48000018  b 0x83009894
	pc = 0x83009894; continue 'dispatch;
	// 83009880: 4BCBD731  bl 0x82cc6fb0
	ctx.lr = 0x83009884;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 83009884: 4B236C7D  bl 0x82240500
	ctx.lr = 0x83009888;
	crate::recompiler::externs::call(&mut ctx, base, 0x82240500);
	// 83009888: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300988C: 4BCB8F25  bl 0x82cc27b0
	ctx.lr = 0x83009890;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC27B0);
	// 83009890: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83009894: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 83009898: 4BC9FBBC  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_830098A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830098A8 size=276
	// 830098A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830098AC: 4BC9FB61  bl 0x82ca940c
	ctx.lr = 0x830098B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 830098B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830098B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830098B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830098BC: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 830098C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830098C4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830098C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830098CC: 409A002C  bne cr6, 0x830098f8
	if !ctx.cr[6].eq {
	pc = 0x830098F8; continue 'dispatch;
	}
	// 830098D0: 4BCA8789  bl 0x82cb2058
	ctx.lr = 0x830098D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 830098D4: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 830098D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830098DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830098E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830098E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830098E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830098EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830098F0: 4BCA8629  bl 0x82cb1f18
	ctx.lr = 0x830098F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 830098F4: 480000C0  b 0x830099b4
	pc = 0x830099B4; continue 'dispatch;
	// 830098F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830098FC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83009900: 4BCAB9F9  bl 0x82cb52f8
	ctx.lr = 0x83009904;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 83009904: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83009908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300990C: 4BCA5595  bl 0x82caeea0
	ctx.lr = 0x83009910;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEEA0);
	// 83009910: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009918: 4BCA1761  bl 0x82cab078
	ctx.lr = 0x8300991C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAB078);
	// 8300991C: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83009920: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009924: 556B0732  rlwinm r11, r11, 0, 0x1c, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83009928: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8300992C: 419A002C  beq cr6, 0x83009958
	if ctx.cr[6].eq {
	pc = 0x83009958; continue 'dispatch;
	}
	// 83009930: 2F1DFFFE  cmpwi cr6, r29, -2
	ctx.cr[6].compare_i32(ctx.r[29].s32, -2, &mut ctx.xer);
	// 83009934: 419A0024  beq cr6, 0x83009958
	if ctx.cr[6].eq {
	pc = 0x83009958; continue 'dispatch;
	}
	// 83009938: 7FAB2E70  srawi r11, r29, 5
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 5) as i64;
	// 8300993C: 3D40834F  lis r10, -0x7cb1
	ctx.r[10].s64 = -2091974656;
	// 83009940: 5569103A  slwi r9, r11, 2
	// 83009944: 394A7760  addi r10, r10, 0x7760
	ctx.r[10].s64 = ctx.r[10].s64 + 30560;
	// 83009948: 57AB3572  rlwinm r11, r29, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x03FFFFFFu64;
	// 8300994C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83009950: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83009954: 4800000C  b 0x83009960
	pc = 0x83009960; continue 'dispatch;
	// 83009958: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8300995C: 396BF6E8  addi r11, r11, -0x918
	ctx.r[11].s64 = ctx.r[11].s64 + -2328;
	// 83009960: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83009964: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83009968: 554A07FA  rlwinm r10, r10, 0, 0x1f, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8300996C: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 83009970: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009974: 556A0631  rlwinm. r10, r11, 0, 0x18, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83009978: 4182000C  beq 0x83009984
	if ctx.cr[0].eq {
	pc = 0x83009984; continue 'dispatch;
	}
	// 8300997C: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83009980: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83009984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009988: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300998C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83009990: 4BCAC309  bl 0x82cb5c98
	ctx.lr = 0x83009994;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5C98);
	// 83009994: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83009998: 409A0010  bne cr6, 0x830099a8
	if !ctx.cr[6].eq {
	pc = 0x830099A8; continue 'dispatch;
	}
	// 8300999C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830099A0: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 830099A4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830099A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830099AC: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 830099B0: 4800000D  bl 0x830099bc
	ctx.lr = 0x830099B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x830099BC);
	// 830099B4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830099B8: 4BC9FAA4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_830099BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830099BC size=236
	// 830099BC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830099C0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830099C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830099C8: 9181FFF0  stw r12, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[12].u32 ) };
	// 830099CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830099D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830099D4: 4BCA558D  bl 0x82caef60
	ctx.lr = 0x830099D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 830099D8: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 830099DC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 830099E0: 8181FFF0  lwz r12, -0x10(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 830099E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830099E8: 4E800020  blr
	return;
}

pub fn sub_83009AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009AA8 size=12
	// 83009AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009AAC: 4BC9F95D  bl 0x82ca9408
	ctx.lr = 0x83009AB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83009AB0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83009C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009C70 size=392
	// 83009C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009C80: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83009C84: 409A0030  bne cr6, 0x83009cb4
	if !ctx.cr[6].eq {
	pc = 0x83009CB4; continue 'dispatch;
	}
	// 83009C88: 4BCA83D1  bl 0x82cb2058
	ctx.lr = 0x83009C8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009C8C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83009C90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009C94: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009C98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83009C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009CA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009CA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009CA8: 4BCA8271  bl 0x82cb1f18
	ctx.lr = 0x83009CAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83009CAC: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83009CB0: 48000120  b 0x83009dd0
	pc = 0x83009DD0; continue 'dispatch;
	// 83009CB4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83009CB8: 409A0030  bne cr6, 0x83009ce8
	if !ctx.cr[6].eq {
	pc = 0x83009CE8; continue 'dispatch;
	}
	// 83009CBC: 4BCA839D  bl 0x82cb2058
	ctx.lr = 0x83009CC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009CC0: 3BE00016  li r31, 0x16
	ctx.r[31].s64 = 22;
	// 83009CC4: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83009CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83009CD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009CD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009CD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009CDC: 4BCA823D  bl 0x82cb1f18
	ctx.lr = 0x83009CE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83009CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009CE4: 480000EC  b 0x83009dd0
	pc = 0x83009DD0; continue 'dispatch;
	// 83009CE8: 7CEB0034  cntlzw r11, r7
	ctx.r[11].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 83009CEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83009CF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83009CF4: 9BE40000  stb r31, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83009CF8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83009CFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83009D00: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83009D04: 41990010  bgt cr6, 0x83009d14
	if ctx.cr[6].gt {
	pc = 0x83009D14; continue 'dispatch;
	}
	// 83009D08: 4BCA8351  bl 0x82cb2058
	ctx.lr = 0x83009D0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009D0C: 3BE00022  li r31, 0x22
	ctx.r[31].s64 = 34;
	// 83009D10: 4BFFFFB4  b 0x83009cc4
	pc = 0x83009CC4; continue 'dispatch;
	// 83009D14: 2B060002  cmplwi cr6, r6, 2
	ctx.cr[6].compare_u32(ctx.r[6].u32, 2 as u32, &mut ctx.xer);
	// 83009D18: 4198FFA4  blt cr6, 0x83009cbc
	if ctx.cr[6].lt {
	pc = 0x83009CBC; continue 'dispatch;
	}
	// 83009D1C: 2B060024  cmplwi cr6, r6, 0x24
	ctx.cr[6].compare_u32(ctx.r[6].u32, 36 as u32, &mut ctx.xer);
	// 83009D20: 4199FF9C  bgt cr6, 0x83009cbc
	if ctx.cr[6].gt {
	pc = 0x83009CBC; continue 'dispatch;
	}
	// 83009D24: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83009D28: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83009D2C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83009D30: 419A0018  beq cr6, 0x83009d48
	if ctx.cr[6].eq {
	pc = 0x83009D48; continue 'dispatch;
	}
	// 83009D34: 3940002D  li r10, 0x2d
	ctx.r[10].s64 = 45;
	// 83009D38: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 83009D3C: 99440000  stb r10, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83009D40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83009D44: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 83009D48: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83009D4C: 7D433396  divwu r10, r3, r6
	ctx.r[10].u32 = ctx.r[3].u32 / ctx.r[6].u32;
	// 83009D50: 0CC60000  twi 6, r6, 0
	// 83009D54: 7D4A31D6  mullw r10, r10, r6
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[6].s32 as i64);
	// 83009D58: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 83009D5C: 7C633396  divwu r3, r3, r6
	ctx.r[3].u32 = ctx.r[3].u32 / ctx.r[6].u32;
	// 83009D60: 0CC60000  twi 6, r6, 0
	// 83009D64: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 83009D68: 4099000C  ble cr6, 0x83009d74
	if !ctx.cr[6].gt {
	pc = 0x83009D74; continue 'dispatch;
	}
	// 83009D6C: 394A0057  addi r10, r10, 0x57
	ctx.r[10].s64 = ctx.r[10].s64 + 87;
	// 83009D70: 48000008  b 0x83009d78
	pc = 0x83009D78; continue 'dispatch;
	// 83009D74: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 83009D78: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83009D7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83009D80: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83009D84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83009D88: 419A000C  beq cr6, 0x83009d94
	if ctx.cr[6].eq {
	pc = 0x83009D94; continue 'dispatch;
	}
	// 83009D8C: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 83009D90: 4198FFBC  blt cr6, 0x83009d4c
	if ctx.cr[6].lt {
	pc = 0x83009D4C; continue 'dispatch;
	}
	// 83009D94: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 83009D98: 4198000C  blt cr6, 0x83009da4
	if ctx.cr[6].lt {
	pc = 0x83009DA4; continue 'dispatch;
	}
	// 83009D9C: 9BE40000  stb r31, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83009DA0: 4BFFFF68  b 0x83009d08
	pc = 0x83009D08; continue 'dispatch;
	// 83009DA4: 9BEB0000  stb r31, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83009DA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83009DAC: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009DB0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009DB4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83009DB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83009DBC: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83009DC0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83009DC4: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83009DC8: 4198FFE4  blt cr6, 0x83009dac
	if ctx.cr[6].lt {
	pc = 0x83009DAC; continue 'dispatch;
	}
	// 83009DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009DE0: 4E800020  blr
	return;
}

pub fn sub_83009DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009DF8 size=220
	// 83009DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009DFC: 4BC9F60D  bl 0x82ca9408
	ctx.lr = 0x83009E00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83009E00: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83009E04: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009E08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83009E0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83009E10: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 83009E14: 4BCABD65  bl 0x82cb5b78
	ctx.lr = 0x83009E18;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5B78);
	// 83009E18: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009E1C: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83009E20: 3D40834F  lis r10, -0x7cb1
	ctx.r[10].s64 = -2091974656;
	// 83009E24: 3BCB7870  addi r30, r11, 0x7870
	ctx.r[30].s64 = ctx.r[11].s64 + 30832;
	// 83009E28: 394A7874  addi r10, r10, 0x7874
	ctx.r[10].s64 = ctx.r[10].s64 + 30836;
	// 83009E2C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83009E30: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009E34: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83009E38: 40980084  bge cr6, 0x83009ebc
	if !ctx.cr[6].lt {
	pc = 0x83009EBC; continue 'dispatch;
	}
	// 83009E3C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009E40: 579D103A  slwi r29, r28, 2
	// 83009E44: 7D3D582E  lwzx r9, r29, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83009E48: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83009E4C: 419A0064  beq cr6, 0x83009eb0
	if ctx.cr[6].eq {
	pc = 0x83009EB0; continue 'dispatch;
	}
	// 83009E50: 5524003E  slwi r4, r9, 0
	// 83009E54: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009E58: 716B0083  andi. r11, r11, 0x83
	ctx.r[11].u64 = ctx.r[11].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009E5C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83009E60: 41820050  beq 0x83009eb0
	if ctx.cr[0].eq {
	pc = 0x83009EB0; continue 'dispatch;
	}
	// 83009E64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009E68: 4BCA50A9  bl 0x82caef10
	ctx.lr = 0x83009E6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF10);
	// 83009E6C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009E70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009E74: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83009E78: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009E7C: 716B0083  andi. r11, r11, 0x83
	ctx.r[11].u64 = ctx.r[11].u64 & 131;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009E80: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83009E84: 41820020  beq 0x83009ea4
	if ctx.cr[0].eq {
	pc = 0x83009EA4; continue 'dispatch;
	}
	// 83009E88: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83009E8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83009E90: 419A0014  beq cr6, 0x83009ea4
	if ctx.cr[6].eq {
	pc = 0x83009EA4; continue 'dispatch;
	}
	// 83009E94: 4BCA0C9D  bl 0x82caab30
	ctx.lr = 0x83009E98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAAB30);
	// 83009E98: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83009E9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83009EA0: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83009EA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009EA8: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 83009EAC: 48000079  bl 0x83009f24
	ctx.lr = 0x83009EB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009F24);
	// 83009EB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009EB4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83009EB8: 4BFFFF74  b 0x83009e2c
	pc = 0x83009E2C; continue 'dispatch;
	// 83009EBC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83009EC0: 399F0080  addi r12, r31, 0x80
	ctx.r[12].s64 = ctx.r[31].s64 + 128;
	// 83009EC4: 48000011  bl 0x83009ed4
	ctx.lr = 0x83009EC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83009ED4);
	// 83009EC8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83009ECC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83009ED0: 4BC9F588  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_83009ED4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009ED4 size=36
	// 83009ED4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009ED8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009EE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83009EE4: 4BCABB35  bl 0x82cb5a18
	ctx.lr = 0x83009EE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB5A18);
	// 83009EE8: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009EF4: 4E800020  blr
	return;
}

pub fn sub_83009EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009EF8 size=144
	// 83009EF8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83009EFC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83009F00: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83009F04: FB81FFE8  std r28, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[28].u64 ) };
	// 83009F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009F0C: 9181FFE0  stw r12, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[12].u32 ) };
	// 83009F10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009F14: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83009F18: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83009F1C: 3BCB7870  addi r30, r11, 0x7870
	ctx.r[30].s64 = ctx.r[11].s64 + 30832;
	// 83009F20: 48000020  b 0x83009f40
	pc = 0x83009F40; continue 'dispatch;
	// 83009F24: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83009F28: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83009F2C: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83009F30: FB81FFE8  std r28, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[28].u64 ) };
	// 83009F34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009F38: 9181FFE0  stw r12, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[12].u32 ) };
	// 83009F3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009F40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F44: 578A103A  slwi r10, r28, 2
	// 83009F48: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009F4C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83009F50: 4BCA5051  bl 0x82caefa0
	ctx.lr = 0x83009F54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEFA0);
	// 83009F54: 3D40834F  lis r10, -0x7cb1
	ctx.r[10].s64 = -2091974656;
	// 83009F58: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83009F5C: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 83009F60: 3BCA7870  addi r30, r10, 0x7870
	ctx.r[30].s64 = ctx.r[10].s64 + 30832;
	// 83009F64: 394B7874  addi r10, r11, 0x7874
	ctx.r[10].s64 = ctx.r[11].s64 + 30836;
	// 83009F68: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F6C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83009F70: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009F74: EB81FFE8  ld r28, -0x18(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009F78: 8181FFE0  lwz r12, -0x20(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 83009F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009F80: 4E800020  blr
	return;
}

pub fn sub_83009F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009F88 size=200
	// 83009F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009F98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83009F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83009FA0: 409A003C  bne cr6, 0x83009fdc
	if !ctx.cr[6].eq {
	pc = 0x83009FDC; continue 'dispatch;
	}
	// 83009FA4: 4BCA80ED  bl 0x82cb2090
	ctx.lr = 0x83009FA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2090);
	// 83009FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83009FAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009FB0: 4BCA80A9  bl 0x82cb2058
	ctx.lr = 0x83009FB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009FB4: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 83009FB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83009FBC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009FC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83009FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009FC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009FCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83009FD0: 4BCA7F49  bl 0x82cb1f18
	ctx.lr = 0x83009FD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 83009FD4: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83009FD8: 48000064  b 0x8300a03c
	pc = 0x8300A03C; continue 'dispatch;
	// 83009FDC: 57EB07F9  rlwinm. r11, r31, 0, 0x1f, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009FE0: 4082FFC4  bne 0x83009fa4
	if !ctx.cr[0].eq {
	pc = 0x83009FA4; continue 'dispatch;
	}
	// 83009FE4: 4BCB9BCD  bl 0x82cc3bb0
	ctx.lr = 0x83009FE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC3BB0);
	// 83009FE8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83009FEC: 409A0018  bne cr6, 0x8300a004
	if !ctx.cr[6].eq {
	pc = 0x8300A004; continue 'dispatch;
	}
	// 83009FF0: 4BCBCFC1  bl 0x82cc6fb0
	ctx.lr = 0x83009FF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC6FB0);
	// 83009FF4: 4B23650D  bl 0x82240500
	ctx.lr = 0x83009FF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82240500);
	// 83009FF8: 4BCA8061  bl 0x82cb2058
	ctx.lr = 0x83009FFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 83009FFC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A000: 4800003C  b 0x8300a03c
	pc = 0x8300A03C; continue 'dispatch;
	// 8300A004: 546B06F7  rlwinm. r11, r3, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A008: 40820030  bne 0x8300a038
	if !ctx.cr[0].eq {
	pc = 0x8300A038; continue 'dispatch;
	}
	// 8300A00C: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A010: 41820028  beq 0x8300a038
	if ctx.cr[0].eq {
	pc = 0x8300A038; continue 'dispatch;
	}
	// 8300A014: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A018: 41820020  beq 0x8300a038
	if ctx.cr[0].eq {
	pc = 0x8300A038; continue 'dispatch;
	}
	// 8300A01C: 4BCA8075  bl 0x82cb2090
	ctx.lr = 0x8300A020;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2090);
	// 8300A020: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8300A024: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A028: 4BCA8031  bl 0x82cb2058
	ctx.lr = 0x8300A02C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300A02C: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 8300A030: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A034: 4BFFFFC4  b 0x83009ff8
	pc = 0x83009FF8; continue 'dispatch;
	// 8300A038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A04C: 4E800020  blr
	return;
}

pub fn sub_8300A050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A050 size=176
	// 8300A050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A054: 4BC9F3B9  bl 0x82ca940c
	ctx.lr = 0x8300A058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 8300A058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A05C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300A060: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8300A064: 419A0070  beq cr6, 0x8300a0d4
	if ctx.cr[6].eq {
	pc = 0x8300A0D4; continue 'dispatch;
	}
	// 8300A068: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300A06C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A070: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300A074: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300A078: 409AFFF4  bne cr6, 0x8300a06c
	if !ctx.cr[6].eq {
	pc = 0x8300A06C; continue 'dispatch;
	}
	// 8300A07C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8300A080: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300A084: 556B003E  slwi r11, r11, 0
	// 8300A088: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 8300A08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A090: 4BCA03F9  bl 0x82caa488
	ctx.lr = 0x8300A094;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAA488);
	// 8300A094: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8300A098: 4182003C  beq 0x8300a0d4
	if ctx.cr[0].eq {
	pc = 0x8300A0D4; continue 'dispatch;
	}
	// 8300A09C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300A0A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300A0A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A0A8: 4B166C21  bl 0x82170cc8
	ctx.lr = 0x8300A0AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82170CC8);
	// 8300A0AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300A0B0: 4182001C  beq 0x8300a0cc
	if ctx.cr[0].eq {
	pc = 0x8300A0CC; continue 'dispatch;
	}
	// 8300A0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300A0B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300A0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A0C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300A0C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A0C8: 4BCA7E99  bl 0x82cb1f60
	ctx.lr = 0x8300A0CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F60);
	// 8300A0CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A0D0: 48000008  b 0x8300a0d8
	pc = 0x8300A0D8; continue 'dispatch;
	// 8300A0D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A0D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300A0DC: 4BC9F380  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 8300A0E0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8300A0E4: 386BF8C8  addi r3, r11, -0x738
	ctx.r[3].s64 = ctx.r[11].s64 + -1848;
	// 8300A0E8: 4E800020  blr
	return;
}

pub fn sub_8300A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A100 size=16
	// 8300A100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A10C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8300A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A180 size=184
	// 8300A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A184: 4BC9F281  bl 0x82ca9404
	ctx.lr = 0x8300A188;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8300A188: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A18C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300A190: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300A194: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8300A198: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8300A19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8300A1A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8300A1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300A1A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300A1AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300A1B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8300A1B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300A1B8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8300A1BC: 48000315  bl 0x8300a4d0
	ctx.lr = 0x8300A1C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300A4D0);
	// 8300A1C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300A1C4: 57EB077B  rlwinm. r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A1C8: 41820014  beq 0x8300a1dc
	if ctx.cr[0].eq {
	pc = 0x8300A1DC; continue 'dispatch;
	}
	// 8300A1CC: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 8300A1D0: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8300A1D4: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 8300A1D8: 48000038  b 0x8300a210
	pc = 0x8300A210; continue 'dispatch;
	// 8300A1DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8300A1E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8300A1E4: 4BCB5705  bl 0x82cbf8e8
	ctx.lr = 0x8300A1E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBF8E8);
	// 8300A1E8: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A1EC: 4082000C  bne 0x8300a1f8
	if !ctx.cr[0].eq {
	pc = 0x8300A1F8; continue 'dispatch;
	}
	// 8300A1F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8300A1F4: 409A0008  bne cr6, 0x8300a1fc
	if !ctx.cr[6].eq {
	pc = 0x8300A1FC; continue 'dispatch;
	}
	// 8300A1F8: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 8300A1FC: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A200: 4082000C  bne 0x8300a20c
	if !ctx.cr[0].eq {
	pc = 0x8300A20C; continue 'dispatch;
	}
	// 8300A204: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8300A208: 409A0008  bne cr6, 0x8300a210
	if !ctx.cr[6].eq {
	pc = 0x8300A210; continue 'dispatch;
	}
	// 8300A20C: 63DE0100  ori r30, r30, 0x100
	ctx.r[30].u64 = ctx.r[30].u64 | 256;
	// 8300A210: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300A214: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300A218: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8300A21C: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8300A220: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8300A224: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300A228: F95D0010  std r10, 0x10(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8300A22C: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300A230: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300A234: 4BC9F220  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8300A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A238 size=312
	// 8300A238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A248: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A24C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8300A250: 5468063E  clrlwi r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8300A254: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8300A258: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8300A25C: 2B09003A  cmplwi cr6, r9, 0x3a
	ctx.cr[6].compare_u32(ctx.r[9].u32, 58 as u32, &mut ctx.xer);
	// 8300A260: 409A0008  bne cr6, 0x8300a268
	if !ctx.cr[6].eq {
	pc = 0x8300A268; continue 'dispatch;
	}
	// 8300A264: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8300A268: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A26C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8300A270: 2F09005C  cmpwi cr6, r9, 0x5c
	ctx.cr[6].compare_i32(ctx.r[9].s32, 92, &mut ctx.xer);
	// 8300A274: 419A000C  beq cr6, 0x8300a280
	if ctx.cr[6].eq {
	pc = 0x8300A280; continue 'dispatch;
	}
	// 8300A278: 2F09002F  cmpwi cr6, r9, 0x2f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 47, &mut ctx.xer);
	// 8300A27C: 409A0010  bne cr6, 0x8300a28c
	if !ctx.cr[6].eq {
	pc = 0x8300A28C; continue 'dispatch;
	}
	// 8300A280: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8300A284: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300A288: 419A0020  beq cr6, 0x8300a2a8
	if ctx.cr[6].eq {
	pc = 0x8300A2A8; continue 'dispatch;
	}
	// 8300A28C: 550A06F7  rlwinm. r10, r8, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300A290: 40820018  bne 0x8300a2a8
	if !ctx.cr[0].eq {
	pc = 0x8300A2A8; continue 'dispatch;
	}
	// 8300A294: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300A298: 419A0010  beq cr6, 0x8300a2a8
	if ctx.cr[6].eq {
	pc = 0x8300A2A8; continue 'dispatch;
	}
	// 8300A29C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8300A2A0: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 8300A2A4: 48000008  b 0x8300a2ac
	pc = 0x8300A2AC; continue 'dispatch;
	// 8300A2A8: 39404040  li r10, 0x4040
	ctx.r[10].s64 = 16448;
	// 8300A2AC: 7D0940F8  nor r9, r8, r8
	ctx.r[9].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 8300A2B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8300A2B4: 55293E30  rlwinm r9, r9, 7, 0x18, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x01FFFFFFu64;
	// 8300A2B8: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 8300A2BC: 7D2B5378  or r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 8300A2C0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300A2C4: 617E0100  ori r30, r11, 0x100
	ctx.r[30].u64 = ctx.r[11].u64 | 256;
	// 8300A2C8: 4BCA7499  bl 0x82cb1760
	ctx.lr = 0x8300A2CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1760);
	// 8300A2CC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8300A2D0: 4182006C  beq 0x8300a33c
	if ctx.cr[0].eq {
	pc = 0x8300A33C; continue 'dispatch;
	}
	// 8300A2D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300A2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A2DC: 388BD9EC  addi r4, r11, -0x2614
	ctx.r[4].s64 = ctx.r[11].s64 + -9748;
	// 8300A2E0: 4BCA74F1  bl 0x82cb17d0
	ctx.lr = 0x8300A2E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB17D0);
	// 8300A2E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300A2E8: 4182004C  beq 0x8300a334
	if ctx.cr[0].eq {
	pc = 0x8300A334; continue 'dispatch;
	}
	// 8300A2EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300A2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A2F4: 388BD9E4  addi r4, r11, -0x261c
	ctx.r[4].s64 = ctx.r[11].s64 + -9756;
	// 8300A2F8: 4BCA74D9  bl 0x82cb17d0
	ctx.lr = 0x8300A2FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB17D0);
	// 8300A2FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300A300: 41820034  beq 0x8300a334
	if ctx.cr[0].eq {
	pc = 0x8300A334; continue 'dispatch;
	}
	// 8300A304: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300A308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A30C: 388BD9DC  addi r4, r11, -0x2624
	ctx.r[4].s64 = ctx.r[11].s64 + -9764;
	// 8300A310: 4BCA74C1  bl 0x82cb17d0
	ctx.lr = 0x8300A314;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB17D0);
	// 8300A314: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300A318: 4182001C  beq 0x8300a334
	if ctx.cr[0].eq {
	pc = 0x8300A334; continue 'dispatch;
	}
	// 8300A31C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300A320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A324: 388BD9D4  addi r4, r11, -0x262c
	ctx.r[4].s64 = ctx.r[11].s64 + -9772;
	// 8300A328: 4BCA74A9  bl 0x82cb17d0
	ctx.lr = 0x8300A32C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB17D0);
	// 8300A32C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300A330: 4082000C  bne 0x8300a33c
	if !ctx.cr[0].eq {
	pc = 0x8300A33C; continue 'dispatch;
	}
	// 8300A334: 57CB043E  clrlwi r11, r30, 0x10
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8300A338: 617E0040  ori r30, r11, 0x40
	ctx.r[30].u64 = ctx.r[11].u64 | 64;
	// 8300A33C: 57CBEEB8  rlwinm r11, r30, 0x1d, 0x1a, 0x1c
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 8300A340: 7D6BF378  or r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 8300A344: 556AD77E  rlwinm r10, r11, 0x1a, 0x1d, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 8300A348: 7D435B78  or r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300A34C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300A350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A358: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A35C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A360: 4E800020  blr
	return;
}

pub fn sub_8300A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A370 size=12
	// 8300A370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A374: 4BC9F099  bl 0x82ca940c
	ctx.lr = 0x8300A378;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 8300A378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8300A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A410 size=192
	// 8300A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A41C: F8610070  std r3, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u64 ) };
	// 8300A420: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8300A424: 386B9C18  addi r3, r11, -0x63e8
	ctx.r[3].s64 = ctx.r[11].s64 + -25576;
	// 8300A428: 482AF8FD  bl 0x832b9d24
	ctx.lr = 0x8300A42C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9D24);
	// 8300A42C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300A430: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8300A434: 394A9BC0  addi r10, r10, -0x6440
	ctx.r[10].s64 = ctx.r[10].s64 + -25664;
	// 8300A438: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8300A43C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8300A440: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 8300A444: 614A0409  ori r10, r10, 0x409
	ctx.r[10].u64 = ctx.r[10].u64 | 1033;
	// 8300A448: 914B9BC0  stw r10, -0x6440(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25664 as u32), ctx.r[10].u32 ) };
	// 8300A44C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8300A450: 396B9BC0  addi r11, r11, -0x6440
	ctx.r[11].s64 = ctx.r[11].s64 + -25664;
	// 8300A454: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8300A458: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300A45C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8300A460: 816BF240  lwz r11, -0xdc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3520 as u32) ) } as u64;
	// 8300A464: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300A468: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8300A46C: 816BF244  lwz r11, -0xdbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3516 as u32) ) } as u64;
	// 8300A470: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8300A474: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8300A478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300A47C: 914B9C10  stw r10, -0x63f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25584 as u32), ctx.r[10].u32 ) };
	// 8300A480: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300A484: 4BCB2E5D  bl 0x82cbd2e0
	ctx.lr = 0x8300A488;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBD2E0);
	// 8300A488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A48C: 4BCB81CD  bl 0x82cc2658
	ctx.lr = 0x8300A490;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC2658);
	// 8300A490: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8300A494: 386BD9F4  addi r3, r11, -0x260c
	ctx.r[3].s64 = ctx.r[11].s64 + -9740;
	// 8300A498: 4BCB8289  bl 0x82cc2720
	ctx.lr = 0x8300A49C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC2720);
	// 8300A49C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8300A4A0: 816B9C10  lwz r11, -0x63f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25584 as u32) ) } as u64;
	// 8300A4A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A4A8: 409A000C  bne cr6, 0x8300a4b4
	if !ctx.cr[6].eq {
	pc = 0x8300A4B4; continue 'dispatch;
	}
	// 8300A4AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300A4B0: 4BCB2E31  bl 0x82cbd2e0
	ctx.lr = 0x8300A4B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBD2E0);
	// 8300A4B4: 386000F2  li r3, 0xf2
	ctx.r[3].s64 = 242;
	// 8300A4B8: 482AF80D  bl 0x832b9cc4
	ctx.lr = 0x8300A4BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9CC4);
	// 8300A4BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A4C8: 4E800020  blr
	return;
}

pub fn sub_8300A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A4D0 size=2448
	// 8300A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A4D4: 4BC9EF09  bl 0x82ca93dc
	ctx.lr = 0x8300A4D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93DC);
	// 8300A4D8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A4DC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8300A4E0: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 8300A4E4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8300A4E8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8300A4EC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 8300A4F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8300A4F4: 7EB2AB78  mr r18, r21
	ctx.r[18].u64 = ctx.r[21].u64;
	// 8300A4F8: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 8300A4FC: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 8300A500: 7EBAAB78  mr r26, r21
	ctx.r[26].u64 = ctx.r[21].u64;
	// 8300A504: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8300A508: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8300A50C: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 8300A510: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8300A514: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300A518: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300A51C: 409A0030  bne cr6, 0x8300a54c
	if !ctx.cr[6].eq {
	pc = 0x8300A54C; continue 'dispatch;
	}
	// 8300A520: 4BCA7B39  bl 0x82cb2058
	ctx.lr = 0x8300A524;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300A524: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8300A528: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300A52C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A530: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300A534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A538: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300A53C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A540: 4BCA79D9  bl 0x82cb1f18
	ctx.lr = 0x8300A544;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB1F18);
	// 8300A544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300A548: 4800090C  b 0x8300ae54
	pc = 0x8300AE54; continue 'dispatch;
	// 8300A54C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8300A550: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A554: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 8300A558: 419A001C  beq cr6, 0x8300a574
	if ctx.cr[6].eq {
	pc = 0x8300A574; continue 'dispatch;
	}
	// 8300A55C: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 8300A560: 419A0014  beq cr6, 0x8300a574
	if ctx.cr[6].eq {
	pc = 0x8300A574; continue 'dispatch;
	}
	// 8300A564: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 8300A568: 419A000C  beq cr6, 0x8300a574
	if ctx.cr[6].eq {
	pc = 0x8300A574; continue 'dispatch;
	}
	// 8300A56C: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 8300A570: 409A000C  bne cr6, 0x8300a57c
	if !ctx.cr[6].eq {
	pc = 0x8300A57C; continue 'dispatch;
	}
	// 8300A574: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A578: 4BFFFFD8  b 0x8300a550
	pc = 0x8300A550; continue 'dispatch;
	// 8300A57C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8300A580: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 8300A584: 60F48000  ori r20, r7, 0x8000
	ctx.r[20].u64 = ctx.r[7].u64 | 32768;
	// 8300A588: 80EAED94  lwz r7, -0x126c(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4716 as u32) ) } as u64;
	// 8300A58C: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A590: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A594: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 8300A598: 4199037C  bgt cr6, 0x8300a914
	if ctx.cr[6].gt {
	pc = 0x8300A914; continue 'dispatch;
	}
	// 8300A59C: 3D808205  lis r12, -0x7dfb
	ctx.r[12].s64 = -2113601536;
	// 8300A5A0: 398CDA70  addi r12, r12, -0x2590
	ctx.r[12].s64 = ctx.r[12].s64 + -9616;
	// 8300A5A4: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8300A5A8: 5400103A  slwi r0, r0, 2
	// 8300A5AC: 3D808301  lis r12, -0x7cff
	ctx.r[12].s64 = -2097086464;
	// 8300A5B0: 398CA5C4  addi r12, r12, -0x5a3c
	ctx.r[12].s64 = ctx.r[12].s64 + -23100;
	// 8300A5B4: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 8300A5B8: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 8300A5BC: 60000000  nop
	// 8300A5C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 8300A5C4: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A5C8: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A5CC: 41980018  blt cr6, 0x8300a5e4
	if ctx.cr[6].lt {
	pc = 0x8300A5E4; continue 'dispatch;
	}
	// 8300A5D0: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A5D4: 41990010  bgt cr6, 0x8300a5e4
	if ctx.cr[6].gt {
	pc = 0x8300A5E4; continue 'dispatch;
	}
	// 8300A5D8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8300A5DC: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8300A5E0: 4BFFFFAC  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A5E4: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A5E8: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A5EC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8300A5F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8300A5F4: 409A000C  bne cr6, 0x8300a600
	if !ctx.cr[6].eq {
	pc = 0x8300A600; continue 'dispatch;
	}
	// 8300A5F8: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8300A5FC: 4BFFFF90  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A600: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8300A604: 419A0028  beq cr6, 0x8300a62c
	if ctx.cr[6].eq {
	pc = 0x8300A62C; continue 'dispatch;
	}
	// 8300A608: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8300A60C: 419A0014  beq cr6, 0x8300a620
	if ctx.cr[6].eq {
	pc = 0x8300A620; continue 'dispatch;
	}
	// 8300A610: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8300A614: 409A02AC  bne cr6, 0x8300a8c0
	if !ctx.cr[6].eq {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A618: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300A61C: 4BFFFF70  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A620: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8300A624: 7E92A378  mr r18, r20
	ctx.r[18].u64 = ctx.r[20].u64;
	// 8300A628: 4BFFFF64  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A62C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8300A630: 7EB2AB78  mr r18, r21
	ctx.r[18].u64 = ctx.r[21].u64;
	// 8300A634: 4BFFFF58  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A638: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A63C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8300A640: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A644: 4198000C  blt cr6, 0x8300a650
	if ctx.cr[6].lt {
	pc = 0x8300A650; continue 'dispatch;
	}
	// 8300A648: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A64C: 4099FF8C  ble cr6, 0x8300a5d8
	if !ctx.cr[6].gt {
	pc = 0x8300A5D8; continue 'dispatch;
	}
	// 8300A650: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A654: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A658: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8300A65C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8300A660: 409A000C  bne cr6, 0x8300a66c
	if !ctx.cr[6].eq {
	pc = 0x8300A66C; continue 'dispatch;
	}
	// 8300A664: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8300A668: 4BFFFF24  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A66C: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8300A670: 419A003C  beq cr6, 0x8300a6ac
	if ctx.cr[6].eq {
	pc = 0x8300A6AC; continue 'dispatch;
	}
	// 8300A674: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8300A678: 419A0034  beq cr6, 0x8300a6ac
	if ctx.cr[6].eq {
	pc = 0x8300A6AC; continue 'dispatch;
	}
	// 8300A67C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8300A680: 419AFF98  beq cr6, 0x8300a618
	if ctx.cr[6].eq {
	pc = 0x8300A618; continue 'dispatch;
	}
	// 8300A684: 2F0B0043  cmpwi cr6, r11, 0x43
	ctx.cr[6].compare_i32(ctx.r[11].s32, 67, &mut ctx.xer);
	// 8300A688: 40990238  ble cr6, 0x8300a8c0
	if !ctx.cr[6].gt {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A68C: 2F0B0045  cmpwi cr6, r11, 0x45
	ctx.cr[6].compare_i32(ctx.r[11].s32, 69, &mut ctx.xer);
	// 8300A690: 40990014  ble cr6, 0x8300a6a4
	if !ctx.cr[6].gt {
	pc = 0x8300A6A4; continue 'dispatch;
	}
	// 8300A694: 2F0B0063  cmpwi cr6, r11, 0x63
	ctx.cr[6].compare_i32(ctx.r[11].s32, 99, &mut ctx.xer);
	// 8300A698: 40990228  ble cr6, 0x8300a8c0
	if !ctx.cr[6].gt {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A69C: 2F0B0065  cmpwi cr6, r11, 0x65
	ctx.cr[6].compare_i32(ctx.r[11].s32, 101, &mut ctx.xer);
	// 8300A6A0: 41990220  bgt cr6, 0x8300a8c0
	if ctx.cr[6].gt {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A6A4: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8300A6A8: 4BFFFEE4  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A6AC: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8300A6B0: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 8300A6B4: 4BFFFED8  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A6B8: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A6BC: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A6C0: 4198000C  blt cr6, 0x8300a6cc
	if ctx.cr[6].lt {
	pc = 0x8300A6CC; continue 'dispatch;
	}
	// 8300A6C4: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A6C8: 4099FF10  ble cr6, 0x8300a5d8
	if !ctx.cr[6].gt {
	pc = 0x8300A5D8; continue 'dispatch;
	}
	// 8300A6CC: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A6D0: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A6D4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8300A6D8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8300A6DC: 419AFF1C  beq cr6, 0x8300a5f8
	if ctx.cr[6].eq {
	pc = 0x8300A5F8; continue 'dispatch;
	}
	// 8300A6E0: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8300A6E4: 419AFF34  beq cr6, 0x8300a618
	if ctx.cr[6].eq {
	pc = 0x8300A618; continue 'dispatch;
	}
	// 8300A6E8: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8300A6EC: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8300A6F0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300A6F4: 419A06FC  beq cr6, 0x8300adf0
	if ctx.cr[6].eq {
	pc = 0x8300ADF0; continue 'dispatch;
	}
	// 8300A6F8: 2B060018  cmplwi cr6, r6, 0x18
	ctx.cr[6].compare_u32(ctx.r[6].u32, 24 as u32, &mut ctx.xer);
	// 8300A6FC: 40990028  ble cr6, 0x8300a724
	if !ctx.cr[6].gt {
	pc = 0x8300A724; continue 'dispatch;
	}
	// 8300A700: 89610097  lbz r11, 0x97(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(151 as u32) ) } as u64;
	// 8300A704: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8300A708: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8300A70C: 4198000C  blt cr6, 0x8300a718
	if ctx.cr[6].lt {
	pc = 0x8300A718; continue 'dispatch;
	}
	// 8300A710: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300A714: 99610097  stb r11, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[11].u8 ) };
	// 8300A718: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8300A71C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8300A720: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8300A724: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8300A728: 419A06B4  beq cr6, 0x8300addc
	if ctx.cr[6].eq {
	pc = 0x8300ADDC; continue 'dispatch;
	}
	// 8300A72C: 8943FFFF  lbz r10, -1(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8300A730: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8300A734: 4800026C  b 0x8300a9a0
	pc = 0x8300A9A0; continue 'dispatch;
	// 8300A738: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A73C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8300A740: 48000038  b 0x8300a778
	pc = 0x8300A778; continue 'dispatch;
	// 8300A744: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A748: 41990038  bgt cr6, 0x8300a780
	if ctx.cr[6].gt {
	pc = 0x8300A780; continue 'dispatch;
	}
	// 8300A74C: 2B060019  cmplwi cr6, r6, 0x19
	ctx.cr[6].compare_u32(ctx.r[6].u32, 25 as u32, &mut ctx.xer);
	// 8300A750: 40980018  bge cr6, 0x8300a768
	if !ctx.cr[6].lt {
	pc = 0x8300A768; continue 'dispatch;
	}
	// 8300A754: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 8300A758: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8300A75C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300A760: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8300A764: 48000008  b 0x8300a76c
	pc = 0x8300A76C; continue 'dispatch;
	// 8300A768: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8300A76C: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A770: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A774: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8300A778: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A77C: 4098FFC8  bge cr6, 0x8300a744
	if !ctx.cr[6].lt {
	pc = 0x8300A744; continue 'dispatch;
	}
	// 8300A780: 82C70000  lwz r22, 0(r7)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A784: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A788: 89560000  lbz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A78C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8300A790: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8300A794: 419AFED0  beq cr6, 0x8300a664
	if ctx.cr[6].eq {
	pc = 0x8300A664; continue 'dispatch;
	}
	// 8300A798: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8300A79C: 419AFF10  beq cr6, 0x8300a6ac
	if ctx.cr[6].eq {
	pc = 0x8300A6AC; continue 'dispatch;
	}
	// 8300A7A0: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8300A7A4: 419AFF08  beq cr6, 0x8300a6ac
	if ctx.cr[6].eq {
	pc = 0x8300A6AC; continue 'dispatch;
	}
	// 8300A7A8: 4BFFFEDC  b 0x8300a684
	pc = 0x8300A684; continue 'dispatch;
	// 8300A7AC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8300A7B0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8300A7B4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8300A7B8: 409A0024  bne cr6, 0x8300a7dc
	if !ctx.cr[6].eq {
	pc = 0x8300A7DC; continue 'dispatch;
	}
	// 8300A7BC: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A7C0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A7C4: 409A0018  bne cr6, 0x8300a7dc
	if !ctx.cr[6].eq {
	pc = 0x8300A7DC; continue 'dispatch;
	}
	// 8300A7C8: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A7CC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8300A7D0: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A7D4: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8300A7D8: 419AFFF0  beq cr6, 0x8300a7c8
	if ctx.cr[6].eq {
	pc = 0x8300A7C8; continue 'dispatch;
	}
	// 8300A7DC: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A7E0: 48000034  b 0x8300a814
	pc = 0x8300A814; continue 'dispatch;
	// 8300A7E4: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A7E8: 41990034  bgt cr6, 0x8300a81c
	if ctx.cr[6].gt {
	pc = 0x8300A81C; continue 'dispatch;
	}
	// 8300A7EC: 2B060019  cmplwi cr6, r6, 0x19
	ctx.cr[6].compare_u32(ctx.r[6].u32, 25 as u32, &mut ctx.xer);
	// 8300A7F0: 40980018  bge cr6, 0x8300a808
	if !ctx.cr[6].lt {
	pc = 0x8300A808; continue 'dispatch;
	}
	// 8300A7F4: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 8300A7F8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8300A7FC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8300A800: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8300A804: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8300A808: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A80C: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A810: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8300A814: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A818: 4098FFCC  bge cr6, 0x8300a7e4
	if !ctx.cr[6].lt {
	pc = 0x8300A7E4; continue 'dispatch;
	}
	// 8300A81C: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A820: 4BFFFF78  b 0x8300a798
	pc = 0x8300A798; continue 'dispatch;
	// 8300A824: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A828: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8300A82C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A830: 4198FEB8  blt cr6, 0x8300a6e8
	if ctx.cr[6].lt {
	pc = 0x8300A6E8; continue 'dispatch;
	}
	// 8300A834: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A838: 4199FEB0  bgt cr6, 0x8300a6e8
	if ctx.cr[6].gt {
	pc = 0x8300A6E8; continue 'dispatch;
	}
	// 8300A83C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8300A840: 4BFFFD9C  b 0x8300a5dc
	pc = 0x8300A5DC; continue 'dispatch;
	// 8300A844: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A848: 38A8FFFC  addi r5, r8, -4
	ctx.r[5].s64 = ctx.r[8].s64 + -4;
	// 8300A84C: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A850: 41980014  blt cr6, 0x8300a864
	if ctx.cr[6].lt {
	pc = 0x8300A864; continue 'dispatch;
	}
	// 8300A854: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A858: 4199000C  bgt cr6, 0x8300a864
	if ctx.cr[6].gt {
	pc = 0x8300A864; continue 'dispatch;
	}
	// 8300A85C: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8300A860: 4BFFFD7C  b 0x8300a5dc
	pc = 0x8300A5DC; continue 'dispatch;
	// 8300A864: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8300A868: 419A001C  beq cr6, 0x8300a884
	if ctx.cr[6].eq {
	pc = 0x8300A884; continue 'dispatch;
	}
	// 8300A86C: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8300A870: 419A0090  beq cr6, 0x8300a900
	if ctx.cr[6].eq {
	pc = 0x8300A900; continue 'dispatch;
	}
	// 8300A874: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8300A878: 409AFE70  bne cr6, 0x8300a6e8
	if !ctx.cr[6].eq {
	pc = 0x8300A6E8; continue 'dispatch;
	}
	// 8300A87C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8300A880: 4BFFFD0C  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A884: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8300A888: 4BFFFD04  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A88C: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A890: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8300A894: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A898: 409A0014  bne cr6, 0x8300a8ac
	if !ctx.cr[6].eq {
	pc = 0x8300A8AC; continue 'dispatch;
	}
	// 8300A89C: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A8A0: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A8A4: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8300A8A8: 419AFFF4  beq cr6, 0x8300a89c
	if ctx.cr[6].eq {
	pc = 0x8300A89C; continue 'dispatch;
	}
	// 8300A8AC: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A8B0: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A8B4: 4198000C  blt cr6, 0x8300a8c0
	if ctx.cr[6].lt {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A8B8: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A8BC: 4099FFA0  ble cr6, 0x8300a85c
	if !ctx.cr[6].gt {
	pc = 0x8300A85C; continue 'dispatch;
	}
	// 8300A8C0: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8300A8C4: 4BFFFE28  b 0x8300a6ec
	pc = 0x8300A6EC; continue 'dispatch;
	// 8300A8C8: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A8CC: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 8300A8D0: 4198FFA4  blt cr6, 0x8300a874
	if ctx.cr[6].lt {
	pc = 0x8300A874; continue 'dispatch;
	}
	// 8300A8D4: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A8D8: 4099FF84  ble cr6, 0x8300a85c
	if !ctx.cr[6].gt {
	pc = 0x8300A85C; continue 'dispatch;
	}
	// 8300A8DC: 4BFFFF98  b 0x8300a874
	pc = 0x8300A874; continue 'dispatch;
	// 8300A8E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300A8E4: 419A0028  beq cr6, 0x8300a90c
	if ctx.cr[6].eq {
	pc = 0x8300A90C; continue 'dispatch;
	}
	// 8300A8E8: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A8EC: 38A8FFFE  addi r5, r8, -2
	ctx.r[5].s64 = ctx.r[8].s64 + -2;
	// 8300A8F0: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 8300A8F4: 419AFF90  beq cr6, 0x8300a884
	if ctx.cr[6].eq {
	pc = 0x8300A884; continue 'dispatch;
	}
	// 8300A8F8: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 8300A8FC: 409AFDEC  bne cr6, 0x8300a6e8
	if !ctx.cr[6].eq {
	pc = 0x8300A6E8; continue 'dispatch;
	}
	// 8300A900: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8300A904: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 8300A908: 4BFFFC84  b 0x8300a58c
	pc = 0x8300A58C; continue 'dispatch;
	// 8300A90C: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8300A910: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8300A914: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8300A918: 409AFC74  bne cr6, 0x8300a58c
	if !ctx.cr[6].eq {
	pc = 0x8300A58C; continue 'dispatch;
	}
	// 8300A91C: 4BFFFDD0  b 0x8300a6ec
	pc = 0x8300A6EC; continue 'dispatch;
	// 8300A920: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A924: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8300A928: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8300A92C: 4800002C  b 0x8300a958
	pc = 0x8300A958; continue 'dispatch;
	// 8300A930: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A934: 41990034  bgt cr6, 0x8300a968
	if ctx.cr[6].gt {
	pc = 0x8300A968; continue 'dispatch;
	}
	// 8300A938: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8300A93C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8300A940: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 8300A944: 2F091450  cmpwi cr6, r9, 0x1450
	ctx.cr[6].compare_i32(ctx.r[9].s32, 5200, &mut ctx.xer);
	// 8300A948: 4199001C  bgt cr6, 0x8300a964
	if ctx.cr[6].gt {
	pc = 0x8300A964; continue 'dispatch;
	}
	// 8300A94C: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A950: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A954: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8300A958: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A95C: 4098FFD4  bge cr6, 0x8300a930
	if !ctx.cr[6].lt {
	pc = 0x8300A930; continue 'dispatch;
	}
	// 8300A960: 48000008  b 0x8300a968
	pc = 0x8300A968; continue 'dispatch;
	// 8300A964: 39201451  li r9, 0x1451
	ctx.r[9].s64 = 5201;
	// 8300A968: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300A96C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 8300A970: 48000014  b 0x8300a984
	pc = 0x8300A984; continue 'dispatch;
	// 8300A974: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 8300A978: 4199FF48  bgt cr6, 0x8300a8c0
	if ctx.cr[6].gt {
	pc = 0x8300A8C0; continue 'dispatch;
	}
	// 8300A97C: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A980: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 8300A984: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8300A988: 4098FFEC  bge cr6, 0x8300a974
	if !ctx.cr[6].lt {
	pc = 0x8300A974; continue 'dispatch;
	}
	// 8300A98C: 4BFFFF34  b 0x8300a8c0
	pc = 0x8300A8C0; continue 'dispatch;
	// 8300A990: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300A994: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8300A998: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8300A99C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A9A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300A9A4: 419AFFEC  beq cr6, 0x8300a990
	if ctx.cr[6].eq {
	pc = 0x8300A990; continue 'dispatch;
	}
	// 8300A9A8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8300A9AC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8300A9B0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8300A9B4: 4BCB5D25  bl 0x82cc06d8
	ctx.lr = 0x8300A9B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC06D8);
	// 8300A9B8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8300A9BC: 40980008  bge cr6, 0x8300a9c4
	if !ctx.cr[6].lt {
	pc = 0x8300A9C4; continue 'dispatch;
	}
	// 8300A9C0: 7FBD00D0  neg r29, r29
	ctx.r[29].s64 = -ctx.r[29].s64;
	// 8300A9C4: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8300A9C8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300A9CC: 409A0008  bne cr6, 0x8300a9d4
	if !ctx.cr[6].eq {
	pc = 0x8300A9D4; continue 'dispatch;
	}
	// 8300A9D0: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8300A9D4: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8300A9D8: 409A0008  bne cr6, 0x8300a9e0
	if !ctx.cr[6].eq {
	pc = 0x8300A9E0; continue 'dispatch;
	}
	// 8300A9DC: 7D785850  subf r11, r24, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[24].s64;
	// 8300A9E0: 2F0B1450  cmpwi cr6, r11, 0x1450
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5200, &mut ctx.xer);
	// 8300A9E4: 41990424  bgt cr6, 0x8300ae08
	if ctx.cr[6].gt {
	pc = 0x8300AE08; continue 'dispatch;
	}
	// 8300A9E8: 2F0BEBB0  cmpwi cr6, r11, -0x1450
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5200, &mut ctx.xer);
	// 8300A9EC: 41980434  blt cr6, 0x8300ae20
	if ctx.cr[6].lt {
	pc = 0x8300AE20; continue 'dispatch;
	}
	// 8300A9F0: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8300A9F4: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 8300A9F8: 394AFB58  addi r10, r10, -0x4a8
	ctx.r[10].s64 = ctx.r[10].s64 + -1192;
	// 8300A9FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AA00: 3B2AFFA0  addi r25, r10, -0x60
	ctx.r[25].s64 = ctx.r[10].s64 + -96;
	// 8300AA04: 419A03C4  beq cr6, 0x8300adc8
	if ctx.cr[6].eq {
	pc = 0x8300ADC8; continue 'dispatch;
	}
	// 8300AA08: 40980014  bge cr6, 0x8300aa1c
	if !ctx.cr[6].lt {
	pc = 0x8300AA1C; continue 'dispatch;
	}
	// 8300AA0C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8300AA10: 7F0B00D0  neg r24, r11
	ctx.r[24].s64 = -ctx.r[11].s64;
	// 8300AA14: 396AFCB8  addi r11, r10, -0x348
	ctx.r[11].s64 = ctx.r[10].s64 + -840;
	// 8300AA18: 3B2BFFA0  addi r25, r11, -0x60
	ctx.r[25].s64 = ctx.r[11].s64 + -96;
	// 8300AA1C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8300AA20: 409A0008  bne cr6, 0x8300aa28
	if !ctx.cr[6].eq {
	pc = 0x8300AA28; continue 'dispatch;
	}
	// 8300AA24: B2A1006A  sth r21, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[21].u16 ) };
	// 8300AA28: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8300AA2C: 419A039C  beq cr6, 0x8300adc8
	if ctx.cr[6].eq {
	pc = 0x8300ADC8; continue 'dispatch;
	}
	// 8300AA30: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8300AA34: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8300AA38: 617AFFFF  ori r26, r11, 0xffff
	ctx.r[26].u64 = ctx.r[11].u64 | 65535;
	// 8300AA3C: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 8300AA40: 3AE08000  li r23, -0x8000
	ctx.r[23].s64 = -32768;
	// 8300AA44: 61518000  ori r17, r10, 0x8000
	ctx.r[17].u64 = ctx.r[10].u64 | 32768;
	// 8300AA48: 570B077F  clrlwi. r11, r24, 0x1d
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AA4C: 3B390054  addi r25, r25, 0x54
	ctx.r[25].s64 = ctx.r[25].s64 + 84;
	// 8300AA50: 7F181E70  srawi r24, r24, 3
	ctx.xer.ca = (ctx.r[24].s32 < 0) && ((ctx.r[24].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[24].s32 >> 3) as i64;
	// 8300AA54: 4182036C  beq 0x8300adc0
	if ctx.cr[0].eq {
	pc = 0x8300ADC0; continue 'dispatch;
	}
	// 8300AA58: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 8300AA5C: 7C8BCA14  add r4, r11, r25
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8300AA60: A164000A  lhz r11, 0xa(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(10 as u32) ) } as u64;
	// 8300AA64: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8300AA68: 41980020  blt cr6, 0x8300aa88
	if ctx.cr[6].lt {
	pc = 0x8300AA88; continue 'dispatch;
	}
	// 8300AA6C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8300AA70: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8300AA74: 4BC9EA0D  bl 0x82ca9480
	ctx.lr = 0x8300AA78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9480);
	// 8300AA78: 81610076  lwz r11, 0x76(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(118 as u32) ) } as u64;
	// 8300AA7C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8300AA80: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300AA84: 91610076  stw r11, 0x76(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[11].u32 ) };
	// 8300AA88: A1410060  lhz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8300AA8C: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8300AA90: 92A10058  stw r21, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[21].u32 ) };
	// 8300AA94: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 8300AA98: 554B047E  clrlwi r11, r10, 0x11
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00007FFFu64;
	// 8300AA9C: 92A10050  stw r21, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[21].u32 ) };
	// 8300AAA0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8300AAA4: A1040000  lhz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AAA8: 550A047E  clrlwi r10, r8, 0x11
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x00007FFFu64;
	// 8300AAAC: 7D284278  xor r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 ^ ctx.r[8].u64;
	// 8300AAB0: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8300AAB4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8300AAB8: 2B0B7FFF  cmplwi cr6, r11, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32767 as u32, &mut ctx.xer);
	// 8300AABC: 551B0420  rlwinm r27, r8, 0, 0x10, 0x10
	ctx.r[27].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 8300AAC0: 54FD043E  clrlwi r29, r7, 0x10
	ctx.r[29].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 8300AAC4: 409802E4  bge cr6, 0x8300ada8
	if !ctx.cr[6].lt {
	pc = 0x8300ADA8; continue 'dispatch;
	}
	// 8300AAC8: 2B0A7FFF  cmplwi cr6, r10, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32767 as u32, &mut ctx.xer);
	// 8300AACC: 409802DC  bge cr6, 0x8300ada8
	if !ctx.cr[6].lt {
	pc = 0x8300ADA8; continue 'dispatch;
	}
	// 8300AAD0: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8300AAD4: 2B0BBFFD  cmplwi cr6, r11, 0xbffd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49149 as u32, &mut ctx.xer);
	// 8300AAD8: 419902D0  bgt cr6, 0x8300ada8
	if ctx.cr[6].gt {
	pc = 0x8300ADA8; continue 'dispatch;
	}
	// 8300AADC: 2B0B3FBF  cmplwi cr6, r11, 0x3fbf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16319 as u32, &mut ctx.xer);
	// 8300AAE0: 4199000C  bgt cr6, 0x8300aaec
	if ctx.cr[6].gt {
	pc = 0x8300AAEC; continue 'dispatch;
	}
	// 8300AAE4: 92A10060  stw r21, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[21].u32 ) };
	// 8300AAE8: 480002D0  b 0x8300adb8
	pc = 0x8300ADB8; continue 'dispatch;
	// 8300AAEC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8300AAF0: 409A0038  bne cr6, 0x8300ab28
	if !ctx.cr[6].eq {
	pc = 0x8300AB28; continue 'dispatch;
	}
	// 8300AAF4: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8300AAF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300AAFC: 5529007F  clrlwi. r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AB00: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AB04: 40820024  bne 0x8300ab28
	if !ctx.cr[0].eq {
	pc = 0x8300AB28; continue 'dispatch;
	}
	// 8300AB08: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8300AB0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300AB10: 409A0018  bne cr6, 0x8300ab28
	if !ctx.cr[6].eq {
	pc = 0x8300AB28; continue 'dispatch;
	}
	// 8300AB14: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300AB18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300AB1C: 409A000C  bne cr6, 0x8300ab28
	if !ctx.cr[6].eq {
	pc = 0x8300AB28; continue 'dispatch;
	}
	// 8300AB20: B2A10060  sth r21, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[21].u16 ) };
	// 8300AB24: 4800029C  b 0x8300adc0
	pc = 0x8300ADC0; continue 'dispatch;
	// 8300AB28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8300AB2C: 409A0034  bne cr6, 0x8300ab60
	if !ctx.cr[6].eq {
	pc = 0x8300AB60; continue 'dispatch;
	}
	// 8300AB30: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8300AB34: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AB38: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300AB3C: 554A007F  clrlwi. r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300AB40: 557D043E  clrlwi r29, r11, 0x10
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AB44: 4082001C  bne 0x8300ab60
	if !ctx.cr[0].eq {
	pc = 0x8300AB60; continue 'dispatch;
	}
	// 8300AB48: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AB4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300AB50: 409A0010  bne cr6, 0x8300ab60
	if !ctx.cr[6].eq {
	pc = 0x8300AB60; continue 'dispatch;
	}
	// 8300AB54: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300AB58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300AB5C: 419AFF88  beq cr6, 0x8300aae4
	if ctx.cr[6].eq {
	pc = 0x8300AAE4; continue 'dispatch;
	}
	// 8300AB60: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 8300AB64: 39010056  addi r8, r1, 0x56
	ctx.r[8].s64 = ctx.r[1].s64 + 86;
	// 8300AB68: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8300AB6C: 57CB083C  slwi r11, r30, 1
	// 8300AB70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300AB74: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8300AB78: 40990064  ble cr6, 0x8300abdc
	if !ctx.cr[6].gt {
	pc = 0x8300ABDC; continue 'dispatch;
	}
	// 8300AB7C: 3941006A  addi r10, r1, 0x6a
	ctx.r[10].s64 = ctx.r[1].s64 + 106;
	// 8300AB80: 38A40002  addi r5, r4, 2
	ctx.r[5].s64 = ctx.r[4].s64 + 2;
	// 8300AB84: 7CCB5050  subf r6, r11, r10
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8300AB88: A1460000  lhz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AB8C: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 8300AB90: A1250000  lhz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AB94: 81680002  lwz r11, 2(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 8300AB98: 7D2A49D6  mullw r9, r10, r9
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8300AB9C: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8300ABA0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300ABA4: 4198000C  blt cr6, 0x8300abb0
	if ctx.cr[6].lt {
	pc = 0x8300ABB0; continue 'dispatch;
	}
	// 8300ABA8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300ABAC: 40980008  bge cr6, 0x8300abb4
	if !ctx.cr[6].lt {
	pc = 0x8300ABB4; continue 'dispatch;
	}
	// 8300ABB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300ABB4: 91480002  stw r10, 2(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[10].u32 ) };
	// 8300ABB8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8300ABBC: 419A0010  beq cr6, 0x8300abcc
	if ctx.cr[6].eq {
	pc = 0x8300ABCC; continue 'dispatch;
	}
	// 8300ABC0: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ABC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300ABC8: B1680000  sth r11, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8300ABCC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8300ABD0: 38C6FFFE  addi r6, r6, -2
	ctx.r[6].s64 = ctx.r[6].s64 + -2;
	// 8300ABD4: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 8300ABD8: 4181FFB0  bgt 0x8300ab88
	if ctx.cr[0].gt {
	pc = 0x8300AB88; continue 'dispatch;
	}
	// 8300ABDC: 3463FFFF  addic. r3, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8300ABE0: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8300ABE4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8300ABE8: 4181FF84  bgt 0x8300ab6c
	if ctx.cr[0].gt {
	pc = 0x8300AB6C; continue 'dispatch;
	}
	// 8300ABEC: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8300ABF0: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8300ABF4: 396BC002  addi r11, r11, -0x3ffe
	ctx.r[11].s64 = ctx.r[11].s64 + -16382;
	// 8300ABF8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300ABFC: 7D6A0735  extsh. r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300AC00: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300AC04: 40810050  ble 0x8300ac54
	if !ctx.cr[0].gt {
	pc = 0x8300AC54; continue 'dispatch;
	}
	// 8300AC08: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AC0C: 55090001  rlwinm. r9, r8, 0, 0, 0
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC10: 40820044  bne 0x8300ac54
	if !ctx.cr[0].eq {
	pc = 0x8300AC54; continue 'dispatch;
	}
	// 8300AC14: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300AC18: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AC1C: 55470FFE  srwi r7, r10, 0x1f
	// 8300AC20: 55260FFE  srwi r6, r9, 0x1f
	// 8300AC24: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8300AC28: 5529083C  slwi r9, r9, 1
	// 8300AC2C: 5508083C  slwi r8, r8, 1
	// 8300AC30: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AC34: 554A083C  slwi r10, r10, 1
	// 8300AC38: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8300AC3C: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 8300AC40: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8300AC44: 7D650735  extsh. r5, r11
	ctx.r[5].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8300AC48: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8300AC4C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8300AC50: 4181FFB8  bgt 0x8300ac08
	if ctx.cr[0].gt {
	pc = 0x8300AC08; continue 'dispatch;
	}
	// 8300AC54: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC58: 4181009C  bgt 0x8300acf4
	if ctx.cr[0].gt {
	pc = 0x8300ACF4; continue 'dispatch;
	}
	// 8300AC5C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AC60: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 8300AC64: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AC68: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC6C: 40800088  bge 0x8300acf4
	if !ctx.cr[0].lt {
	pc = 0x8300ACF4; continue 'dispatch;
	}
	// 8300AC70: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300AC74: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AC78: A121005A  lhz r9, 0x5a(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8300AC7C: 552907FF  clrlwi. r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC80: 41820008  beq 0x8300ac88
	if ctx.cr[0].eq {
	pc = 0x8300AC88; continue 'dispatch;
	}
	// 8300AC84: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8300AC88: 54C907FF  clrlwi. r9, r6, 0x1f
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC8C: 7EC8B378  mr r8, r22
	ctx.r[8].u64 = ctx.r[22].u64;
	// 8300AC90: 40820008  bne 0x8300ac98
	if !ctx.cr[0].eq {
	pc = 0x8300AC98; continue 'dispatch;
	}
	// 8300AC94: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8300AC98: 54E907FF  clrlwi. r9, r7, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AC9C: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 8300ACA0: 40820008  bne 0x8300aca8
	if !ctx.cr[0].eq {
	pc = 0x8300ACA8; continue 'dispatch;
	}
	// 8300ACA4: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8300ACA8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300ACAC: 554AF87E  srwi r10, r10, 1
	// 8300ACB0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300ACB4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8300ACB8: 54E7F87E  srwi r7, r7, 1
	// 8300ACBC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300ACC0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8300ACC4: 54C6F87E  srwi r6, r6, 1
	// 8300ACC8: 7D690735  extsh. r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300ACCC: 7CE74378  or r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8300ACD0: 4180FFA8  blt 0x8300ac78
	if ctx.cr[0].lt {
	pc = 0x8300AC78; continue 'dispatch;
	}
	// 8300ACD4: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8300ACD8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300ACDC: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 8300ACE0: 419A0014  beq cr6, 0x8300acf4
	if ctx.cr[6].eq {
	pc = 0x8300ACF4; continue 'dispatch;
	}
	// 8300ACE4: A141005A  lhz r10, 0x5a(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8300ACE8: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8300ACEC: B141005A  sth r10, 0x5a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[10].u16 ) };
	// 8300ACF0: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300ACF4: A121005A  lhz r9, 0x5a(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 8300ACF8: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 8300ACFC: 41990018  bgt cr6, 0x8300ad14
	if ctx.cr[6].gt {
	pc = 0x8300AD14; continue 'dispatch;
	}
	// 8300AD00: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 8300AD04: 554A03FE  clrlwi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0001FFFFu64;
	// 8300AD08: 61298000  ori r9, r9, 0x8000
	ctx.r[9].u64 = ctx.r[9].u64 | 32768;
	// 8300AD0C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300AD10: 409A0064  bne cr6, 0x8300ad74
	if !ctx.cr[6].eq {
	pc = 0x8300AD74; continue 'dispatch;
	}
	// 8300AD14: 81410056  lwz r10, 0x56(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8300AD18: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8300AD1C: 409A0050  bne cr6, 0x8300ad6c
	if !ctx.cr[6].eq {
	pc = 0x8300AD6C; continue 'dispatch;
	}
	// 8300AD20: 81410052  lwz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8300AD24: 92A10056  stw r21, 0x56(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[21].u32 ) };
	// 8300AD28: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8300AD2C: 409A0034  bne cr6, 0x8300ad60
	if !ctx.cr[6].eq {
	pc = 0x8300AD60; continue 'dispatch;
	}
	// 8300AD30: A1410050  lhz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AD34: 92A10052  stw r21, 0x52(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[21].u32 ) };
	// 8300AD38: 2B0AFFFF  cmplwi cr6, r10, 0xffff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65535 as u32, &mut ctx.xer);
	// 8300AD3C: 409A0018  bne cr6, 0x8300ad54
	if !ctx.cr[6].eq {
	pc = 0x8300AD54; continue 'dispatch;
	}
	// 8300AD40: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AD44: B2810050  sth r20, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[20].u16 ) };
	// 8300AD48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300AD4C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AD50: 48000024  b 0x8300ad74
	pc = 0x8300AD74; continue 'dispatch;
	// 8300AD54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300AD58: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8300AD5C: 48000018  b 0x8300ad74
	pc = 0x8300AD74; continue 'dispatch;
	// 8300AD60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300AD64: 91410052  stw r10, 0x52(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u32 ) };
	// 8300AD68: 4800000C  b 0x8300ad74
	pc = 0x8300AD74; continue 'dispatch;
	// 8300AD6C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300AD70: 91410056  stw r10, 0x56(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u32 ) };
	// 8300AD74: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8300AD78: 2B0B7FFF  cmplwi cr6, r11, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32767 as u32, &mut ctx.xer);
	// 8300AD7C: 4098002C  bge cr6, 0x8300ada8
	if !ctx.cr[6].lt {
	pc = 0x8300ADA8; continue 'dispatch;
	}
	// 8300AD80: A1210058  lhz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300AD84: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8300AD88: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300AD8C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AD90: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300AD94: B1610060  sth r11, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 8300AD98: B121006A  sth r9, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[9].u16 ) };
	// 8300AD9C: 91010066  stw r8, 0x66(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[8].u32 ) };
	// 8300ADA0: 90E10062  stw r7, 0x62(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[7].u32 ) };
	// 8300ADA4: 4800001C  b 0x8300adc0
	pc = 0x8300ADC0; continue 'dispatch;
	// 8300ADA8: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 8300ADAC: 576B043F  clrlwi. r11, r27, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300ADB0: 40820008  bne 0x8300adb8
	if !ctx.cr[0].eq {
	pc = 0x8300ADB8; continue 'dispatch;
	}
	// 8300ADB4: 92210060  stw r17, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[17].u32 ) };
	// 8300ADB8: 92A10068  stw r21, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[21].u32 ) };
	// 8300ADBC: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8300ADC0: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8300ADC4: 409AFC84  bne cr6, 0x8300aa48
	if !ctx.cr[6].eq {
	pc = 0x8300AA48; continue 'dispatch;
	}
	// 8300ADC8: A161006A  lhz r11, 0x6a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(106 as u32) ) } as u64;
	// 8300ADCC: 81010066  lwz r8, 0x66(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 8300ADD0: 81210062  lwz r9, 0x62(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 8300ADD4: A1410060  lhz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8300ADD8: 4800005C  b 0x8300ae34
	pc = 0x8300AE34; continue 'dispatch;
	// 8300ADDC: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300ADE0: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8300ADE4: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8300ADE8: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8300ADEC: 48000048  b 0x8300ae34
	pc = 0x8300AE34; continue 'dispatch;
	// 8300ADF0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300ADF4: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8300ADF8: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8300ADFC: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8300AE00: 3AA00004  li r21, 4
	ctx.r[21].s64 = 4;
	// 8300AE04: 48000030  b 0x8300ae34
	pc = 0x8300AE34; continue 'dispatch;
	// 8300AE08: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8300AE0C: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300AE10: 39407FFF  li r10, 0x7fff
	ctx.r[10].s64 = 32767;
	// 8300AE14: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8300AE18: 3AA00002  li r21, 2
	ctx.r[21].s64 = 2;
	// 8300AE1C: 48000018  b 0x8300ae34
	pc = 0x8300AE34; continue 'dispatch;
	// 8300AE20: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300AE24: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8300AE28: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8300AE2C: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 8300AE30: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 8300AE34: B173000A  sth r11, 0xa(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 8300AE38: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8300AE3C: 564B043E  clrlwi r11, r18, 0x10
	ctx.r[11].u64 = ctx.r[18].u32 as u64 & 0x0000FFFFu64;
	// 8300AE40: 91130006  stw r8, 6(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[8].u32 ) };
	// 8300AE44: 91330002  stw r9, 2(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(2 as u32), ctx.r[9].u32 ) };
	// 8300AE48: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8300AE4C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300AE50: B1730000  sth r11, 0(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8300AE54: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8300AE58: 4BC9E5D4  b 0x82ca942c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA942C);
	return;
}

pub fn sub_8300AE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AE60 size=632
	// 8300AE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AE64: 4BC9E5A5  bl 0x82ca9408
	ctx.lr = 0x8300AE68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 8300AE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AE6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300AE70: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8300AE74: 3D40834F  lis r10, -0x7cb1
	ctx.r[10].s64 = -2091974656;
	// 8300AE78: 3B8BF6E8  addi r28, r11, -0x918
	ctx.r[28].s64 = ctx.r[11].s64 + -2328;
	// 8300AE7C: 3BAA7760  addi r29, r10, 0x7760
	ctx.r[29].s64 = ctx.r[10].s64 + 30560;
	// 8300AE80: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300AE84: 55290673  rlwinm. r9, r9, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8300AE88: 408200D8  bne 0x8300af60
	if !ctx.cr[0].eq {
	pc = 0x8300AF60; continue 'dispatch;
	}
	// 8300AE8C: 4BCAA46D  bl 0x82cb52f8
	ctx.lr = 0x8300AE90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AE90: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300AE94: 419A003C  beq cr6, 0x8300aed0
	if ctx.cr[6].eq {
	pc = 0x8300AED0; continue 'dispatch;
	}
	// 8300AE98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AE9C: 4BCAA45D  bl 0x82cb52f8
	ctx.lr = 0x8300AEA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AEA0: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 8300AEA4: 419A002C  beq cr6, 0x8300aed0
	if ctx.cr[6].eq {
	pc = 0x8300AED0; continue 'dispatch;
	}
	// 8300AEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AEAC: 4BCAA44D  bl 0x82cb52f8
	ctx.lr = 0x8300AEB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AEB0: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 8300AEB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AEB8: 557E103A  slwi r30, r11, 2
	// 8300AEBC: 4BCAA43D  bl 0x82cb52f8
	ctx.lr = 0x8300AEC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AEC0: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300AEC4: 546A3572  rlwinm r10, r3, 6, 0x15, 0x19
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 8300AEC8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8300AECC: 48000008  b 0x8300aed4
	pc = 0x8300AED4; continue 'dispatch;
	// 8300AED0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300AED4: 896B0028  lbz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300AED8: 556B003D  rlwinm. r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AEDC: 41820084  beq 0x8300af60
	if ctx.cr[0].eq {
	pc = 0x8300AF60; continue 'dispatch;
	}
	// 8300AEE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AEE4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AEE8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300AEEC: 41800018  blt 0x8300af04
	if ctx.cr[0].lt {
	pc = 0x8300AF04; continue 'dispatch;
	}
	// 8300AEF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEF4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8300AEF8: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEFC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300AF00: 4800000C  b 0x8300af0c
	pc = 0x8300AF0C; continue 'dispatch;
	// 8300AF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF08: 4BCB12A1  bl 0x82cbc1a8
	ctx.lr = 0x8300AF0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC1A8);
	// 8300AF0C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300AF10: 409A000C  bne cr6, 0x8300af1c
	if !ctx.cr[6].eq {
	pc = 0x8300AF1C; continue 'dispatch;
	}
	// 8300AF14: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8300AF18: 480001AC  b 0x8300b0c4
	pc = 0x8300B0C4; continue 'dispatch;
	// 8300AF1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AF20: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 8300AF24: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AF28: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300AF2C: 41800018  blt 0x8300af44
	if ctx.cr[0].lt {
	pc = 0x8300AF44; continue 'dispatch;
	}
	// 8300AF30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AF34: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8300AF38: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AF3C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300AF40: 4800000C  b 0x8300af4c
	pc = 0x8300AF4C; continue 'dispatch;
	// 8300AF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF48: 4BCB1261  bl 0x82cbc1a8
	ctx.lr = 0x8300AF4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC1A8);
	// 8300AF4C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300AF50: 419AFFC4  beq cr6, 0x8300af14
	if ctx.cr[6].eq {
	pc = 0x8300AF14; continue 'dispatch;
	}
	// 8300AF54: 98610051  stb r3, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[3].u8 ) };
	// 8300AF58: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AF5C: 48000168  b 0x8300b0c4
	pc = 0x8300B0C4; continue 'dispatch;
	// 8300AF60: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300AF64: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AF68: 4082012C  bne 0x8300b094
	if !ctx.cr[0].eq {
	pc = 0x8300B094; continue 'dispatch;
	}
	// 8300AF6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF70: 4BCAA389  bl 0x82cb52f8
	ctx.lr = 0x8300AF74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AF74: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300AF78: 419A003C  beq cr6, 0x8300afb4
	if ctx.cr[6].eq {
	pc = 0x8300AFB4; continue 'dispatch;
	}
	// 8300AF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF80: 4BCAA379  bl 0x82cb52f8
	ctx.lr = 0x8300AF84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AF84: 2F03FFFE  cmpwi cr6, r3, -2
	ctx.cr[6].compare_i32(ctx.r[3].s32, -2, &mut ctx.xer);
	// 8300AF88: 419A002C  beq cr6, 0x8300afb4
	if ctx.cr[6].eq {
	pc = 0x8300AFB4; continue 'dispatch;
	}
	// 8300AF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF90: 4BCAA369  bl 0x82cb52f8
	ctx.lr = 0x8300AF94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AF94: 7C6B2E70  srawi r11, r3, 5
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 5) as i64;
	// 8300AF98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF9C: 557E103A  slwi r30, r11, 2
	// 8300AFA0: 4BCAA359  bl 0x82cb52f8
	ctx.lr = 0x8300AFA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB52F8);
	// 8300AFA4: 7D5EE82E  lwzx r10, r30, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300AFA8: 546B3572  rlwinm r11, r3, 6, 0x15, 0x19
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x03FFFFFFu64;
	// 8300AFAC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8300AFB0: 48000008  b 0x8300afb8
	pc = 0x8300AFB8; continue 'dispatch;
	// 8300AFB4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300AFB8: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AFBC: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AFC0: 418200D4  beq 0x8300b094
	if ctx.cr[0].eq {
	pc = 0x8300B094; continue 'dispatch;
	}
	// 8300AFC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AFC8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8300AFCC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AFD0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300AFD4: 41800018  blt 0x8300afec
	if ctx.cr[0].lt {
	pc = 0x8300AFEC; continue 'dispatch;
	}
	// 8300AFD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AFDC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8300AFE0: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AFE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300AFE8: 4800000C  b 0x8300aff4
	pc = 0x8300AFF4; continue 'dispatch;
	// 8300AFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AFF0: 4BCB11B9  bl 0x82cbc1a8
	ctx.lr = 0x8300AFF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC1A8);
	// 8300AFF4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300AFF8: 419AFF1C  beq cr6, 0x8300af14
	if ctx.cr[6].eq {
	pc = 0x8300AF14; continue 'dispatch;
	}
	// 8300AFFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300B000: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8300B004: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8300B008: 4BCA23F9  bl 0x82cad400
	ctx.lr = 0x8300B00C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAD400);
	// 8300B00C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8300B010: 41820054  beq 0x8300b064
	if ctx.cr[0].eq {
	pc = 0x8300B064; continue 'dispatch;
	}
	// 8300B014: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B018: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300B01C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300B020: 41800018  blt 0x8300b038
	if ctx.cr[0].lt {
	pc = 0x8300B038; continue 'dispatch;
	}
	// 8300B024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B028: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8300B02C: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B030: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300B034: 4800000C  b 0x8300b040
	pc = 0x8300B040; continue 'dispatch;
	// 8300B038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B03C: 4BCB116D  bl 0x82cbc1a8
	ctx.lr = 0x8300B040;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBC1A8);
	// 8300B040: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300B044: 409A0018  bne cr6, 0x8300b05c
	if !ctx.cr[6].eq {
	pc = 0x8300B05C; continue 'dispatch;
	}
	// 8300B048: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300B04C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300B050: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 8300B054: 4BCB5AF5  bl 0x82cc0b48
	ctx.lr = 0x8300B058;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CC0B48);
	// 8300B058: 4BFFFEBC  b 0x8300af14
	pc = 0x8300AF14; continue 'dispatch;
	// 8300B05C: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8300B060: 98610055  stb r3, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[3].u8 ) };
	// 8300B064: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300B068: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8300B06C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B070: 4BCB39A9  bl 0x82cbea18
	ctx.lr = 0x8300B074;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CBEA18);
	// 8300B074: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300B078: 409AFEE0  bne cr6, 0x8300af58
	if !ctx.cr[6].eq {
	pc = 0x8300AF58; continue 'dispatch;
	}
	// 8300B07C: 4BCA6FDD  bl 0x82cb2058
	ctx.lr = 0x8300B080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CB2058);
	// 8300B080: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300B084: 3940002A  li r10, 0x2a
	ctx.r[10].s64 = 42;
	// 8300B088: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8300B08C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300B090: 48000034  b 0x8300b0c4
	pc = 0x8300B0C4; continue 'dispatch;
	// 8300B094: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B098: 356BFFFE  addic. r11, r11, -2
	ctx.xer.ca = (ctx.r[11].u32 > (!(-2 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300B09C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300B0A0: 41800018  blt 0x8300b0b8
	if ctx.cr[0].lt {
	pc = 0x8300B0B8; continue 'dispatch;
	}
	// 8300B0A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B0A8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8300B0AC: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B0B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300B0B4: 48000010  b 0x8300b0c4
	pc = 0x8300B0C4; continue 'dispatch;
	// 8300B0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B0BC: 48000115  bl 0x8300b1d0
	ctx.lr = 0x8300B0C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8300B1D0);
	// 8300B0C0: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8300B0C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300B0C8: 4BC9E390  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_8300B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B0D8 size=24
	// 8300B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B0E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300B0E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300B0E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300B0EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8300B178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B178 size=88
	// 8300B178: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8300B17C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300B180: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8300B184: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B188: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 8300B18C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B190: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300B194: 4800001C  b 0x8300b1b0
	pc = 0x8300B1B0; continue 'dispatch;
	// 8300B198: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8300B19C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300B1A0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8300B1A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B1A8: 9181FFE8  stw r12, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[12].u32 ) };
	// 8300B1AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B1B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B1B4: 4BCA3DAD  bl 0x82caef60
	ctx.lr = 0x8300B1B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CAEF60);
	// 8300B1B8: 80210000  lwz r1, 0(r1)
	ctx.r[1].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B1BC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8300B1C0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300B1C4: 8181FFE8  lwz r12, -0x18(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) } as u64;
	// 8300B1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B1CC: 4E800020  blr
	return;
}

