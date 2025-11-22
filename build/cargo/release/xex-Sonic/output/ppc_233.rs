pub fn sub_82FF2048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2048 size=8
    let mut pc: u32 = 0x82FF2048;
    'dispatch: loop {
        match pc {
            0x82FF2048 => {
    //   block [0x82FF2048..0x82FF2050)
	// 82FF2048: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF204C: 8213E890  lwz r16, -0x1770(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-6000 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2050 size=72
    let mut pc: u32 = 0x82FF2050;
    'dispatch: loop {
        match pc {
            0x82FF2050 => {
    //   block [0x82FF2050..0x82FF2098)
	// 82FF2050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF2054: 481B6119  bl 0x831a816c
	ctx.lr = 0x82FF2058;
	sub_831A8130(ctx, base);
	// 82FF2058: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF205C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2060: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF2064: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FF2068: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FF206C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF2070: 4BFE6EC1  bl 0x82fd8f30
	ctx.lr = 0x82FF2074;
	sub_82FD8F30(ctx, base);
	// 82FF2074: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2078: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF207C: 396BD9E8  addi r11, r11, -0x2618
	ctx.r[11].s64 = ctx.r[11].s64 + -9752;
	// 82FF2080: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2084: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF2088: 4BFE7231  bl 0x82fd92b8
	ctx.lr = 0x82FF208C;
	sub_82FD92B8(ctx, base);
	// 82FF208C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2090: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF2094: 481B6128  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2098 size=40
    let mut pc: u32 = 0x82FF2098;
    'dispatch: loop {
        match pc {
            0x82FF2098 => {
    //   block [0x82FF2098..0x82FF20C0)
	// 82FF2098: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF209C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF20A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF20A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF20A8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF20AC: 4BFE6DCD  bl 0x82fd8e78
	ctx.lr = 0x82FF20B0;
	sub_82FD8E78(ctx, base);
	// 82FF20B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF20B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF20B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF20BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF20C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF20C0 size=68
    let mut pc: u32 = 0x82FF20C0;
    'dispatch: loop {
        match pc {
            0x82FF20C0 => {
    //   block [0x82FF20C0..0x82FF2104)
	// 82FF20C0: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF20C4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF20C8: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82FF20CC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF20D0: 4098002C  bge cr6, 0x82ff20fc
	if !ctx.cr[6].lt {
	pc = 0x82FF20FC; continue 'dispatch;
	}
	// 82FF20D4: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF20D8: 61298054  ori r9, r9, 0x8054
	ctx.r[9].u64 = ctx.r[9].u64 | 32852;
	// 82FF20DC: 7D23482E  lwzx r9, r3, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF20E0: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF20E4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF20E8: 7D0848AE  lbzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF20EC: 55080031  rlwinm. r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FF20F0: 41820014  beq 0x82ff2104
	if ctx.cr[0].eq {
		sub_82FF2104(ctx, base);
		return;
	}
	// 82FF20F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF20F8: 4198FFE8  blt cr6, 0x82ff20e0
	if ctx.cr[6].lt {
	pc = 0x82FF20E0; continue 'dispatch;
	}
	// 82FF20FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF2100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2104 size=8
    let mut pc: u32 = 0x82FF2104;
    'dispatch: loop {
        match pc {
            0x82FF2104 => {
    //   block [0x82FF2104..0x82FF210C)
	// 82FF2104: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF2108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2110 size=68
    let mut pc: u32 = 0x82FF2110;
    'dispatch: loop {
        match pc {
            0x82FF2110 => {
    //   block [0x82FF2110..0x82FF2154)
	// 82FF2110: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF2114: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF2118: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82FF211C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF2120: 4098002C  bge cr6, 0x82ff214c
	if !ctx.cr[6].lt {
	pc = 0x82FF214C; continue 'dispatch;
	}
	// 82FF2124: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF2128: 61298054  ori r9, r9, 0x8054
	ctx.r[9].u64 = ctx.r[9].u64 | 32852;
	// 82FF212C: 7D23482E  lwzx r9, r3, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF2130: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2134: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF2138: 7D0848AE  lbzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF213C: 55080031  rlwinm. r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FF2140: 40820014  bne 0x82ff2154
	if !ctx.cr[0].eq {
		sub_82FF2154(ctx, base);
		return;
	}
	// 82FF2144: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF2148: 4198FFE8  blt cr6, 0x82ff2130
	if ctx.cr[6].lt {
	pc = 0x82FF2130; continue 'dispatch;
	}
	// 82FF214C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF2150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2154 size=8
    let mut pc: u32 = 0x82FF2154;
    'dispatch: loop {
        match pc {
            0x82FF2154 => {
    //   block [0x82FF2154..0x82FF215C)
	// 82FF2154: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF2158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2160 size=36
    let mut pc: u32 = 0x82FF2160;
    'dispatch: loop {
        match pc {
            0x82FF2160 => {
    //   block [0x82FF2160..0x82FF2184)
	// 82FF2160: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2164: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF2168: 614A805C  ori r10, r10, 0x805c
	ctx.r[10].u64 = ctx.r[10].u64 | 32860;
	// 82FF216C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF2170: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF2174: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF2178: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF217C: 409A0008  bne cr6, 0x82ff2184
	if !ctx.cr[6].eq {
		sub_82FF2184(ctx, base);
		return;
	}
	// 82FF2180: 4BFE7650  b 0x82fd97d0
	sub_82FD97D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF2184 size=4
    let mut pc: u32 = 0x82FF2184;
    'dispatch: loop {
        match pc {
            0x82FF2184 => {
    //   block [0x82FF2184..0x82FF2188)
	// 82FF2184: 4BFE764C  b 0x82fd97d0
	sub_82FD97D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2188 size=232
    let mut pc: u32 = 0x82FF2188;
    'dispatch: loop {
        match pc {
            0x82FF2188 => {
    //   block [0x82FF2188..0x82FF2270)
	// 82FF2188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF2190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF2194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF2198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF219C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF21A0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF21A4: 3FDF0003  addis r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 196608;
	// 82FF21A8: 616BC014  ori r11, r11, 0xc014
	ctx.r[11].u64 = ctx.r[11].u64 | 49172;
	// 82FF21AC: 3BDE8060  addi r30, r30, -0x7fa0
	ctx.r[30].s64 = ctx.r[30].s64 + -32672;
	// 82FF21B0: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF21B4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF21C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF21C4: 4E800421  bctrl
	ctx.lr = 0x82FF21C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF21C8: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF21CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21D0: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF21D4: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF21D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF21E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF21E4: 4E800421  bctrl
	ctx.lr = 0x82FF21E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF21E8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF21EC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21F0: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FF21F4: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF21F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF21FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF2200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF2204: 4E800421  bctrl
	ctx.lr = 0x82FF2208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2208: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF220C: 616B8044  ori r11, r11, 0x8044
	ctx.r[11].u64 = ctx.r[11].u64 | 32836;
	// 82FF2210: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2214: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2218: 41820018  beq 0x82ff2230
	if ctx.cr[0].eq {
	pc = 0x82FF2230; continue 'dispatch;
	}
	// 82FF221C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2220: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF2224: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF222C: 4E800421  bctrl
	ctx.lr = 0x82FF2230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2230: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2234: 616B804C  ori r11, r11, 0x804c
	ctx.r[11].u64 = ctx.r[11].u64 | 32844;
	// 82FF2238: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF223C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2240: 41820018  beq 0x82ff2258
	if ctx.cr[0].eq {
	pc = 0x82FF2258; continue 'dispatch;
	}
	// 82FF2244: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2248: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF224C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF2254: 4E800421  bctrl
	ctx.lr = 0x82FF2258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF2260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF2264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF2268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF226C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2270 size=248
    let mut pc: u32 = 0x82FF2270;
    'dispatch: loop {
        match pc {
            0x82FF2270 => {
    //   block [0x82FF2270..0x82FF2368)
	// 82FF2270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF2274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF2278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF227C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2280: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF2284: 614A803C  ori r10, r10, 0x803c
	ctx.r[10].u64 = ctx.r[10].u64 | 32828;
	// 82FF2288: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF228C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2290: 418200A4  beq 0x82ff2334
	if ctx.cr[0].eq {
	pc = 0x82FF2334; continue 'dispatch;
	}
	// 82FF2294: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2298: 614A803D  ori r10, r10, 0x803d
	ctx.r[10].u64 = ctx.r[10].u64 | 32829;
	// 82FF229C: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF22A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF22A4: 41820090  beq 0x82ff2334
	if ctx.cr[0].eq {
	pc = 0x82FF2334; continue 'dispatch;
	}
	// 82FF22A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF22AC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF22B0: 40820020  bne 0x82ff22d0
	if !ctx.cr[0].eq {
	pc = 0x82FF22D0; continue 'dispatch;
	}
	// 82FF22B4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF22B8: 614A8038  ori r10, r10, 0x8038
	ctx.r[10].u64 = ctx.r[10].u64 | 32824;
	// 82FF22BC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF22C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF22C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF22C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF22CC: 4E800020  blr
	return;
	// 82FF22D0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82FF22D4: 61298004  ori r9, r9, 0x8004
	ctx.r[9].u64 = ctx.r[9].u64 | 32772;
	// 82FF22D8: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF22DC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF22E0: 40980024  bge cr6, 0x82ff2304
	if !ctx.cr[6].lt {
	pc = 0x82FF2304; continue 'dispatch;
	}
	// 82FF22E4: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF22E8: 394A3002  addi r10, r10, 0x3002
	ctx.r[10].s64 = ctx.r[10].s64 + 12290;
	// 82FF22EC: 61298038  ori r9, r9, 0x8038
	ctx.r[9].u64 = ctx.r[9].u64 | 32824;
	// 82FF22F0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FF22F4: 7D4B482E  lwzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF22F8: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF22FC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FF2300: 4BFFFFC0  b 0x82ff22c0
	pc = 0x82FF22C0; continue 'dispatch;
	// 82FF2304: 392A3001  addi r9, r10, 0x3001
	ctx.r[9].s64 = ctx.r[10].s64 + 12289;
	// 82FF2308: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 82FF230C: 3CE00002  lis r7, 2
	ctx.r[7].s64 = 131072;
	// 82FF2310: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF2314: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF2318: 61088007  ori r8, r8, 0x8007
	ctx.r[8].u64 = ctx.r[8].u64 | 32775;
	// 82FF231C: 60E78038  ori r7, r7, 0x8038
	ctx.r[7].u64 = ctx.r[7].u64 | 32824;
	// 82FF2320: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2324: 7D0A40AE  lbzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FF2328: 7D4B382E  lwzx r10, r11, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FF232C: 7D694214  add r11, r9, r8
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FF2330: 4BFFFFCC  b 0x82ff22fc
	pc = 0x82FF22FC; continue 'dispatch;
	// 82FF2334: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2338: 38C0004A  li r6, 0x4a
	ctx.r[6].s64 = 74;
	// 82FF233C: 61498060  ori r9, r10, 0x8060
	ctx.r[9].u64 = ctx.r[10].u64 | 32864;
	// 82FF2340: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF2344: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 82FF2348: 388AE8C0  addi r4, r10, -0x1740
	ctx.r[4].s64 = ctx.r[10].s64 + -5952;
	// 82FF234C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF2350: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF2354: 4BFDED05  bl 0x82fd1058
	ctx.lr = 0x82FF2358;
	sub_82FD1058(ctx, base);
	// 82FF2358: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF235C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF2360: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FF2364: 481BE8C5  bl 0x831b0c28
	ctx.lr = 0x82FF2368;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2368 size=784
    let mut pc: u32 = 0x82FF2368;
    'dispatch: loop {
        match pc {
            0x82FF2368 => {
    //   block [0x82FF2368..0x82FF2678)
	// 82FF2368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF236C: 481B5DED  bl 0x831a8158
	ctx.lr = 0x82FF2370;
	sub_831A8130(ctx, base);
	// 82FF2370: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2374: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2378: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FF237C: 616BC018  ori r11, r11, 0xc018
	ctx.r[11].u64 = ctx.r[11].u64 | 49176;
	// 82FF2380: 7D7C58AE  lbzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2384: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2388: 408202E4  bne 0x82ff266c
	if !ctx.cr[0].eq {
	pc = 0x82FF266C; continue 'dispatch;
	}
	// 82FF238C: 3FBC0003  addis r29, r28, 3
	ctx.r[29].s64 = ctx.r[28].s64 + 196608;
	// 82FF2390: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF2394: 3BBD8060  addi r29, r29, -0x7fa0
	ctx.r[29].s64 = ctx.r[29].s64 + -32672;
	// 82FF2398: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF239C: 4BFDE7E5  bl 0x82fd0b80
	ctx.lr = 0x82FF23A0;
	sub_82FD0B80(ctx, base);
	// 82FF23A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF23A4: 4BFDFCA5  bl 0x82fd2048
	ctx.lr = 0x82FF23A8;
	sub_82FD2048(ctx, base);
	// 82FF23A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF23AC: 480438BD  bl 0x83035c68
	ctx.lr = 0x82FF23B0;
	sub_83035C68(ctx, base);
	// 82FF23B0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF23B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FF23B8: 6179804C  ori r25, r11, 0x804c
	ctx.r[25].u64 = ctx.r[11].u64 | 32844;
	// 82FF23BC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF23C0: 2F1B03E7  cmpwi cr6, r27, 0x3e7
	ctx.cr[6].compare_i32(ctx.r[27].s32, 999, &mut ctx.xer);
	// 82FF23C4: 617AC010  ori r26, r11, 0xc010
	ctx.r[26].u64 = ctx.r[11].u64 | 49168;
	// 82FF23C8: 3F008339  lis r24, -0x7cc7
	ctx.r[24].s64 = -2093416448;
	// 82FF23CC: 409A0204  bne cr6, 0x82ff25d0
	if !ctx.cr[6].eq {
	pc = 0x82FF25D0; continue 'dispatch;
	}
	// 82FF23D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF23D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF23D8: 388B7DE0  addi r4, r11, 0x7de0
	ctx.r[4].s64 = ctx.r[11].s64 + 32224;
	// 82FF23DC: 4BFDF5ED  bl 0x82fd19c8
	ctx.lr = 0x82FF23E0;
	sub_82FD19C8(ctx, base);
	// 82FF23E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF23E4: 41820164  beq 0x82ff2548
	if ctx.cr[0].eq {
	pc = 0x82FF2548; continue 'dispatch;
	}
	// 82FF23E8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF23EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF23F0: 388B7DF0  addi r4, r11, 0x7df0
	ctx.r[4].s64 = ctx.r[11].s64 + 32240;
	// 82FF23F4: 4BFDF5D5  bl 0x82fd19c8
	ctx.lr = 0x82FF23F8;
	sub_82FD19C8(ctx, base);
	// 82FF23F8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF23FC: 4182014C  beq 0x82ff2548
	if ctx.cr[0].eq {
	pc = 0x82FF2548; continue 'dispatch;
	}
	// 82FF2400: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF2404: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2408: 388B7DFC  addi r4, r11, 0x7dfc
	ctx.r[4].s64 = ctx.r[11].s64 + 32252;
	// 82FF240C: 4BFDF5BD  bl 0x82fd19c8
	ctx.lr = 0x82FF2410;
	sub_82FD19C8(ctx, base);
	// 82FF2410: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF2414: 41820134  beq 0x82ff2548
	if ctx.cr[0].eq {
	pc = 0x82FF2548; continue 'dispatch;
	}
	// 82FF2418: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF241C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2420: 388B7E0C  addi r4, r11, 0x7e0c
	ctx.r[4].s64 = ctx.r[11].s64 + 32268;
	// 82FF2424: 4BFDF5A5  bl 0x82fd19c8
	ctx.lr = 0x82FF2428;
	sub_82FD19C8(ctx, base);
	// 82FF2428: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF242C: 4182011C  beq 0x82ff2548
	if ctx.cr[0].eq {
	pc = 0x82FF2548; continue 'dispatch;
	}
	// 82FF2430: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF2434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2438: 388B7E20  addi r4, r11, 0x7e20
	ctx.r[4].s64 = ctx.r[11].s64 + 32288;
	// 82FF243C: 4BFDF58D  bl 0x82fd19c8
	ctx.lr = 0x82FF2440;
	sub_82FD19C8(ctx, base);
	// 82FF2440: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF2444: 41820104  beq 0x82ff2548
	if ctx.cr[0].eq {
	pc = 0x82FF2548; continue 'dispatch;
	}
	// 82FF2448: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF244C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2450: 388B7D10  addi r4, r11, 0x7d10
	ctx.r[4].s64 = ctx.r[11].s64 + 32016;
	// 82FF2454: 4BFDF575  bl 0x82fd19c8
	ctx.lr = 0x82FF2458;
	sub_82FD19C8(ctx, base);
	// 82FF2458: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF245C: 41820078  beq 0x82ff24d4
	if ctx.cr[0].eq {
	pc = 0x82FF24D4; continue 'dispatch;
	}
	// 82FF2460: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF2464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2468: 388B7D1C  addi r4, r11, 0x7d1c
	ctx.r[4].s64 = ctx.r[11].s64 + 32028;
	// 82FF246C: 4BFDF55D  bl 0x82fd19c8
	ctx.lr = 0x82FF2470;
	sub_82FD19C8(ctx, base);
	// 82FF2470: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF2474: 41820060  beq 0x82ff24d4
	if ctx.cr[0].eq {
	pc = 0x82FF24D4; continue 'dispatch;
	}
	// 82FF2478: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF247C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF2480: 388B7D28  addi r4, r11, 0x7d28
	ctx.r[4].s64 = ctx.r[11].s64 + 32040;
	// 82FF2484: 4BFDF545  bl 0x82fd19c8
	ctx.lr = 0x82FF2488;
	sub_82FD19C8(ctx, base);
	// 82FF2488: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF248C: 41820048  beq 0x82ff24d4
	if ctx.cr[0].eq {
	pc = 0x82FF24D4; continue 'dispatch;
	}
	// 82FF2490: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2494: 3FFC0002  addis r31, r28, 2
	ctx.r[31].s64 = ctx.r[28].s64 + 131072;
	// 82FF2498: 3BFFC014  addi r31, r31, -0x3fec
	ctx.r[31].s64 = ctx.r[31].s64 + -16364;
	// 82FF249C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF24A0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF24A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF24A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF24AC: 4E800421  bctrl
	ctx.lr = 0x82FF24B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF24B0: 38C04000  li r6, 0x4000
	ctx.r[6].s64 = 16384;
	// 82FF24B4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FF24B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF24BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF24C0: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF24C4: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF24C8: 48003D09  bl 0x82ff61d0
	ctx.lr = 0x82FF24CC;
	sub_82FF61D0(ctx, base);
	// 82FF24CC: 7C7CC92E  stwx r3, r28, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[25].u32), ctx.r[3].u32) };
	// 82FF24D0: 48000124  b 0x82ff25f4
	pc = 0x82FF25F4; continue 'dispatch;
	// 82FF24D4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF24D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF24DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF24E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF24E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF24E8: 4E800421  bctrl
	ctx.lr = 0x82FF24EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF24EC: 7D7CD02E  lwzx r11, r28, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FF24F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FF24F4: 419A000C  beq cr6, 0x82ff2500
	if ctx.cr[6].eq {
	pc = 0x82FF2500; continue 'dispatch;
	}
	// 82FF24F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF24FC: 409A0078  bne cr6, 0x82ff2574
	if !ctx.cr[6].eq {
	pc = 0x82FF2574; continue 'dispatch;
	}
	// 82FF2500: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2504: 3FFC0002  addis r31, r28, 2
	ctx.r[31].s64 = ctx.r[28].s64 + 131072;
	// 82FF2508: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82FF250C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FF2510: 3BFFC014  addi r31, r31, -0x3fec
	ctx.r[31].s64 = ctx.r[31].s64 + -16364;
	// 82FF2514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2518: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF251C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF2520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF2524: 409A0014  bne cr6, 0x82ff2538
	if !ctx.cr[6].eq {
	pc = 0x82FF2538; continue 'dispatch;
	}
	// 82FF2528: 4E800421  bctrl
	ctx.lr = 0x82FF252C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF252C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF2530: 386B7D5C  addi r3, r11, 0x7d5c
	ctx.r[3].s64 = ctx.r[11].s64 + 32092;
	// 82FF2534: 4800008C  b 0x82ff25c0
	pc = 0x82FF25C0; continue 'dispatch;
	// 82FF2538: 4E800421  bctrl
	ctx.lr = 0x82FF253C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF253C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF2540: 386B7D34  addi r3, r11, 0x7d34
	ctx.r[3].s64 = ctx.r[11].s64 + 32052;
	// 82FF2544: 4800007C  b 0x82ff25c0
	pc = 0x82FF25C0; continue 'dispatch;
	// 82FF2548: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF254C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF2550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2554: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF2558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF255C: 4E800421  bctrl
	ctx.lr = 0x82FF2560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2560: 7D7CD02E  lwzx r11, r28, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FF2564: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FF2568: 419A0014  beq cr6, 0x82ff257c
	if ctx.cr[6].eq {
	pc = 0x82FF257C; continue 'dispatch;
	}
	// 82FF256C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FF2570: 419A000C  beq cr6, 0x82ff257c
	if ctx.cr[6].eq {
	pc = 0x82FF257C; continue 'dispatch;
	}
	// 82FF2574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF2578: 480000F8  b 0x82ff2670
	pc = 0x82FF2670; continue 'dispatch;
	// 82FF257C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2580: 3FFC0002  addis r31, r28, 2
	ctx.r[31].s64 = ctx.r[28].s64 + 131072;
	// 82FF2584: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82FF2588: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FF258C: 3BFFC014  addi r31, r31, -0x3fec
	ctx.r[31].s64 = ctx.r[31].s64 + -16364;
	// 82FF2590: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2594: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2598: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF259C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF25A0: 409A0014  bne cr6, 0x82ff25b4
	if !ctx.cr[6].eq {
	pc = 0x82FF25B4; continue 'dispatch;
	}
	// 82FF25A4: 4E800421  bctrl
	ctx.lr = 0x82FF25A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF25A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF25AC: 386B7E58  addi r3, r11, 0x7e58
	ctx.r[3].s64 = ctx.r[11].s64 + 32344;
	// 82FF25B0: 48000010  b 0x82ff25c0
	pc = 0x82FF25C0; continue 'dispatch;
	// 82FF25B4: 4E800421  bctrl
	ctx.lr = 0x82FF25B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF25B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF25BC: 386B7E2C  addi r3, r11, 0x7e2c
	ctx.r[3].s64 = ctx.r[11].s64 + 32300;
	// 82FF25C0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF25C4: 4BFDE5BD  bl 0x82fd0b80
	ctx.lr = 0x82FF25C8;
	sub_82FD0B80(ctx, base);
	// 82FF25C8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF25CC: 48000028  b 0x82ff25f4
	pc = 0x82FF25F4; continue 'dispatch;
	// 82FF25D0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF25D4: 3FFC0002  addis r31, r28, 2
	ctx.r[31].s64 = ctx.r[28].s64 + 131072;
	// 82FF25D8: 3BFFC014  addi r31, r31, -0x3fec
	ctx.r[31].s64 = ctx.r[31].s64 + -16364;
	// 82FF25DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF25E0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF25E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF25E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF25EC: 4E800421  bctrl
	ctx.lr = 0x82FF25F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF25F0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF25F4: 7FDCCA14  add r30, r28, r25
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[25].u64;
	// 82FF25F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF25FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF2600: 409A0068  bne cr6, 0x82ff2668
	if !ctx.cr[6].eq {
	pc = 0x82FF2668; continue 'dispatch;
	}
	// 82FF2604: 38C04000  li r6, 0x4000
	ctx.r[6].s64 = 16384;
	// 82FF2608: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF260C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FF2610: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF2614: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF2618: 48003AB9  bl 0x82ff60d0
	ctx.lr = 0x82FF261C;
	sub_82FF60D0(ctx, base);
	// 82FF261C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2620: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2624: 40820044  bne 0x82ff2668
	if !ctx.cr[0].eq {
	pc = 0x82FF2668; continue 'dispatch;
	}
	// 82FF2628: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF262C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2634: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2638: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF263C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF2640: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF2644: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82FF2648: 38C00062  li r6, 0x62
	ctx.r[6].s64 = 98;
	// 82FF264C: 38A0044B  li r5, 0x44b
	ctx.r[5].s64 = 1099;
	// 82FF2650: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2654: 4BFFA655  bl 0x82fecca8
	ctx.lr = 0x82FF2658;
	sub_82FECCA8(ctx, base);
	// 82FF2658: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF265C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2660: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF2664: 481BE5C5  bl 0x831b0c28
	ctx.lr = 0x82FF2668;
	sub_831B0C28(ctx, base);
	// 82FF2668: 7F7CD12E  stwx r27, r28, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[26].u32), ctx.r[27].u32) };
	// 82FF266C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF2670: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82FF2674: 481B5B34  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2678 size=164
    let mut pc: u32 = 0x82FF2678;
    'dispatch: loop {
        match pc {
            0x82FF2678 => {
    //   block [0x82FF2678..0x82FF271C)
	// 82FF2678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF267C: 481B5AF1  bl 0x831a816c
	ctx.lr = 0x82FF2680;
	sub_831A8130(ctx, base);
	// 82FF2680: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2684: 3FC30002  addis r30, r3, 2
	ctx.r[30].s64 = ctx.r[3].s64 + 131072;
	// 82FF2688: 3FA30003  addis r29, r3, 3
	ctx.r[29].s64 = ctx.r[3].s64 + 196608;
	// 82FF268C: 3BDEC020  addi r30, r30, -0x3fe0
	ctx.r[30].s64 = ctx.r[30].s64 + -16352;
	// 82FF2690: 3BBD8024  addi r29, r29, -0x7fdc
	ctx.r[29].s64 = ctx.r[29].s64 + -32732;
	// 82FF2694: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF2698: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF269C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF26A0: 7FEA4851  subf. r31, r10, r9
	ctx.r[31].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FF26A4: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF26A8: 614AC024  ori r10, r10, 0xc024
	ctx.r[10].u64 = ctx.r[10].u64 | 49188;
	// 82FF26AC: 41820028  beq 0x82ff26d4
	if ctx.cr[0].eq {
	pc = 0x82FF26D4; continue 'dispatch;
	}
	// 82FF26B0: 7CE35214  add r7, r3, r10
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 82FF26B4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF26B8: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FF26BC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FF26C0: 7D2950AE  lbzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF26C4: 7D2759AE  stbx r9, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82FF26C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF26CC: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FF26D0: 4198FFE4  blt cr6, 0x82ff26b4
	if ctx.cr[6].lt {
	pc = 0x82FF26B4; continue 'dispatch;
	}
	// 82FF26D4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF26D8: 7D1F1A14  add r8, r31, r3
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 82FF26DC: 616B8044  ori r11, r11, 0x8044
	ctx.r[11].u64 = ctx.r[11].u64 | 32836;
	// 82FF26E0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82FF26E4: 7C885214  add r4, r8, r10
	ctx.r[4].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82FF26E8: 6129C000  ori r9, r9, 0xc000
	ctx.r[9].u64 = ctx.r[9].u64 | 49152;
	// 82FF26EC: 7C63582E  lwzx r3, r3, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF26F0: 7CBF4850  subf r5, r31, r9
	ctx.r[5].s64 = ctx.r[9].s64 - ctx.r[31].s64;
	// 82FF26F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF26F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF26FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF2700: 4E800421  bctrl
	ctx.lr = 0x82FF2704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2704: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82FF2708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF270C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF2710: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF2714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF2718: 481B5AA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2720 size=188
    let mut pc: u32 = 0x82FF2720;
    'dispatch: loop {
        match pc {
            0x82FF2720 => {
    //   block [0x82FF2720..0x82FF27DC)
	// 82FF2720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF2724: 481B5A3D  bl 0x831a8160
	ctx.lr = 0x82FF2728;
	sub_831A8130(ctx, base);
	// 82FF2728: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF272C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF2730: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF2734: 3FBF0003  addis r29, r31, 3
	ctx.r[29].s64 = ctx.r[31].s64 + 196608;
	// 82FF2738: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FF273C: 3BBD8024  addi r29, r29, -0x7fdc
	ctx.r[29].s64 = ctx.r[29].s64 + -32732;
	// 82FF2740: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FF2744: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2748: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF274C: 4082000C  bne 0x82ff2758
	if !ctx.cr[0].eq {
	pc = 0x82FF2758; continue 'dispatch;
	}
	// 82FF2750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF2754: 48000080  b 0x82ff27d4
	pc = 0x82FF27D4; continue 'dispatch;
	// 82FF2758: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 82FF275C: 3BDEC020  addi r30, r30, -0x3fe0
	ctx.r[30].s64 = ctx.r[30].s64 + -16352;
	// 82FF2760: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2764: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FF2768: 2B0B0064  cmplwi cr6, r11, 0x64
	ctx.cr[6].compare_u32(ctx.r[11].u32, 100 as u32, &mut ctx.xer);
	// 82FF276C: 40980018  bge cr6, 0x82ff2784
	if !ctx.cr[6].lt {
	pc = 0x82FF2784; continue 'dispatch;
	}
	// 82FF2770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2774: 4BFFFF05  bl 0x82ff2678
	ctx.lr = 0x82FF2778;
	sub_82FF2678(ctx, base);
	// 82FF2778: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF277C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2780: 4182FFD0  beq 0x82ff2750
	if ctx.cr[0].eq {
	pc = 0x82FF2750; continue 'dispatch;
	}
	// 82FF2784: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2788: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82FF278C: 6166804C  ori r6, r11, 0x804c
	ctx.r[6].u64 = ctx.r[11].u64 | 32844;
	// 82FF2790: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2794: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82FF2798: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF279C: 7CAB5050  subf r5, r11, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FF27A0: 3C840002  addis r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 131072;
	// 82FF27A4: 7C7F302E  lwzx r3, r31, r6
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82FF27A8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FF27AC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FF27B0: 3884C024  addi r4, r4, -0x3fdc
	ctx.r[4].s64 = ctx.r[4].s64 + -16348;
	// 82FF27B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF27B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF27BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF27C0: 4E800421  bctrl
	ctx.lr = 0x82FF27C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF27C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF27C8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF27CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FF27D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF27D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF27D8: 481B59D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF27E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF27E0 size=720
    let mut pc: u32 = 0x82FF27E0;
    'dispatch: loop {
        match pc {
            0x82FF27E0 => {
    //   block [0x82FF27E0..0x82FF2AB0)
	// 82FF27E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF27E4: 481B5959  bl 0x831a813c
	ctx.lr = 0x82FF27E8;
	sub_831A8130(ctx, base);
	// 82FF27E8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF27EC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF27F0: 8301017C  lwz r24, 0x17c(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 82FF27F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF27F8: 616BC008  ori r11, r11, 0xc008
	ctx.r[11].u64 = ctx.r[11].u64 | 49160;
	// 82FF27FC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FF2800: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FF2804: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 82FF2808: 3EDF0001  addis r22, r31, 1
	ctx.r[22].s64 = ctx.r[31].s64 + 65536;
	// 82FF280C: 3F3F0002  addis r25, r31, 2
	ctx.r[25].s64 = ctx.r[31].s64 + 131072;
	// 82FF2810: 7F5F592E  stwx r26, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[26].u32) };
	// 82FF2814: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 82FF2818: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF281C: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 82FF2820: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2824: 7D324B78  mr r18, r9
	ctx.r[18].u64 = ctx.r[9].u64;
	// 82FF2828: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF282C: 614AC00C  ori r10, r10, 0xc00c
	ctx.r[10].u64 = ctx.r[10].u64 | 49164;
	// 82FF2830: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82FF2834: 6108C018  ori r8, r8, 0xc018
	ctx.r[8].u64 = ctx.r[8].u64 | 49176;
	// 82FF2838: 616BC019  ori r11, r11, 0xc019
	ctx.r[11].u64 = ctx.r[11].u64 | 49177;
	// 82FF283C: 3AD68004  addi r22, r22, -0x7ffc
	ctx.r[22].s64 = ctx.r[22].s64 + -32764;
	// 82FF2840: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2844: 3B39C010  addi r25, r25, -0x3ff0
	ctx.r[25].s64 = ctx.r[25].s64 + -16368;
	// 82FF2848: 7F5F512E  stwx r26, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 82FF284C: 3BBDC014  addi r29, r29, -0x3fec
	ctx.r[29].s64 = ctx.r[29].s64 + -16364;
	// 82FF2850: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF2854: 7F5F41AE  stbx r26, r31, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[26].u8) };
	// 82FF2858: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FF285C: 7FDF59AE  stbx r30, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u8) };
	// 82FF2860: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF2864: 93D60000  stw r30, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2868: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82FF286C: 91390000  stw r9, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FF2870: 7CF33B78  mr r19, r7
	ctx.r[19].u64 = ctx.r[7].u64;
	// 82FF2874: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2878: 4BFDE309  bl 0x82fd0b80
	ctx.lr = 0x82FF287C;
	sub_82FD0B80(ctx, base);
	// 82FF287C: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF2880: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2884: 614AC020  ori r10, r10, 0xc020
	ctx.r[10].u64 = ctx.r[10].u64 | 49184;
	// 82FF2888: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF288C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF2890: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82FF2894: 61298024  ori r9, r9, 0x8024
	ctx.r[9].u64 = ctx.r[9].u64 | 32804;
	// 82FF2898: 7FDF512E  stwx r30, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FF289C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF28A0: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF28A4: 61088028  ori r8, r8, 0x8028
	ctx.r[8].u64 = ctx.r[8].u64 | 32808;
	// 82FF28A8: 614A8030  ori r10, r10, 0x8030
	ctx.r[10].u64 = ctx.r[10].u64 | 32816;
	// 82FF28AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FF28B0: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82FF28B4: 3E3F0003  addis r17, r31, 3
	ctx.r[17].s64 = ctx.r[31].s64 + 196608;
	// 82FF28B8: 3EFF0003  addis r23, r31, 3
	ctx.r[23].s64 = ctx.r[31].s64 + 196608;
	// 82FF28BC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF28C0: 7FDF51AE  stbx r30, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF28C4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF28C8: 7D7F412E  stwx r11, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 82FF28CC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF28D0: 8901016F  lbz r8, 0x16f(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(367 as u32) ) } as u64;
	// 82FF28D4: 61298034  ori r9, r9, 0x8034
	ctx.r[9].u64 = ctx.r[9].u64 | 32820;
	// 82FF28D8: 616B8038  ori r11, r11, 0x8038
	ctx.r[11].u64 = ctx.r[11].u64 | 32824;
	// 82FF28DC: 614A803D  ori r10, r10, 0x803d
	ctx.r[10].u64 = ctx.r[10].u64 | 32829;
	// 82FF28E0: 3A31802C  addi r17, r17, -0x7fd4
	ctx.r[17].s64 = ctx.r[17].s64 + -32724;
	// 82FF28E4: 3AF7803C  addi r23, r23, -0x7fc4
	ctx.r[23].s64 = ctx.r[23].s64 + -32708;
	// 82FF28E8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FF28EC: 7EBF492E  stwx r21, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[21].u32) };
	// 82FF28F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF28F4: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 82FF28F8: 7D1F51AE  stbx r8, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u8) };
	// 82FF28FC: 93710000  stw r27, 0(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF2900: 9BD70000  stb r30, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF2904: 4BFDE27D  bl 0x82fd0b80
	ctx.lr = 0x82FF2908;
	sub_82FD0B80(ctx, base);
	// 82FF2908: 3F7F0003  addis r27, r31, 3
	ctx.r[27].s64 = ctx.r[31].s64 + 196608;
	// 82FF290C: 3EBF0003  addis r21, r31, 3
	ctx.r[21].s64 = ctx.r[31].s64 + 196608;
	// 82FF2910: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2914: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2918: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF291C: 3B7B8048  addi r27, r27, -0x7fb8
	ctx.r[27].s64 = ctx.r[27].s64 + -32696;
	// 82FF2920: 3AB5804C  addi r21, r21, -0x7fb4
	ctx.r[21].s64 = ctx.r[21].s64 + -32692;
	// 82FF2924: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FF2928: 614A8044  ori r10, r10, 0x8044
	ctx.r[10].u64 = ctx.r[10].u64 | 32836;
	// 82FF292C: 61298049  ori r9, r9, 0x8049
	ctx.r[9].u64 = ctx.r[9].u64 | 32841;
	// 82FF2930: 7E9F512E  stwx r20, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[20].u32) };
	// 82FF2934: 3F9F0003  addis r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 196608;
	// 82FF2938: 3E9F0003  addis r20, r31, 3
	ctx.r[20].s64 = ctx.r[31].s64 + 196608;
	// 82FF293C: 89010167  lbz r8, 0x167(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(359 as u32) ) } as u64;
	// 82FF2940: 3B9C8060  addi r28, r28, -0x7fa0
	ctx.r[28].s64 = ctx.r[28].s64 + -32672;
	// 82FF2944: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF2948: 3A948050  addi r20, r20, -0x7fb0
	ctx.r[20].s64 = ctx.r[20].s64 + -32688;
	// 82FF294C: 80810174  lwz r4, 0x174(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FF2950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2954: 9BDB0000  stb r30, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF2958: 93D50000  stw r30, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF295C: 7D1F49AE  stbx r8, r31, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82FF2960: 931C0000  stw r24, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82FF2964: 92540000  stw r18, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 82FF2968: 4BFF5301  bl 0x82fe7c68
	ctx.lr = 0x82FF296C;
	sub_82FE7C68(ctx, base);
	// 82FF296C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2970: 4BFFFD09  bl 0x82ff2678
	ctx.lr = 0x82FF2974;
	sub_82FF2678(ctx, base);
	// 82FF2974: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82FF2978: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF297C: 4BFDE205  bl 0x82fd0b80
	ctx.lr = 0x82FF2980;
	sub_82FD0B80(ctx, base);
	// 82FF2980: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2984: 4BFDF6C5  bl 0x82fd2048
	ctx.lr = 0x82FF2988;
	sub_82FD2048(ctx, base);
	// 82FF2988: 3F008339  lis r24, -0x7cc7
	ctx.r[24].s64 = -2093416448;
	// 82FF298C: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF2990: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2994: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF2998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF299C: 4E800421  bctrl
	ctx.lr = 0x82FF29A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF29A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF29A4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF29A8: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FF29AC: 480432BD  bl 0x83035c68
	ctx.lr = 0x82FF29B0;
	sub_83035C68(ctx, base);
	// 82FF29B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF29B4: 9BDB0000  stb r30, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF29B8: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 82FF29BC: 90990000  stw r4, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FF29C0: 419A000C  beq cr6, 0x82ff29cc
	if ctx.cr[6].eq {
	pc = 0x82FF29CC; continue 'dispatch;
	}
	// 82FF29C4: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82FF29C8: 409A0008  bne cr6, 0x82ff29d0
	if !ctx.cr[6].eq {
	pc = 0x82FF29D0; continue 'dispatch;
	}
	// 82FF29CC: 9B5B0000  stb r26, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 82FF29D0: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF29D4: 2F0403E7  cmpwi cr6, r4, 0x3e7
	ctx.cr[6].compare_i32(ctx.r[4].s32, 999, &mut ctx.xer);
	// 82FF29D8: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF29DC: 38C04000  li r6, 0x4000
	ctx.r[6].s64 = 16384;
	// 82FF29E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FF29E4: 409A0010  bne cr6, 0x82ff29f4
	if !ctx.cr[6].eq {
	pc = 0x82FF29F4; continue 'dispatch;
	}
	// 82FF29E8: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF29EC: 480037E5  bl 0x82ff61d0
	ctx.lr = 0x82FF29F0;
	sub_82FF61D0(ctx, base);
	// 82FF29F0: 48000008  b 0x82ff29f8
	pc = 0x82FF29F8; continue 'dispatch;
	// 82FF29F4: 480036DD  bl 0x82ff60d0
	ctx.lr = 0x82FF29F8;
	sub_82FF60D0(ctx, base);
	// 82FF29F8: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF29FC: 90750000  stw r3, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF2A04: 409A0044  bne cr6, 0x82ff2a48
	if !ctx.cr[6].eq {
	pc = 0x82FF2A48; continue 'dispatch;
	}
	// 82FF2A08: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2A14: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A18: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF2A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF2A20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF2A24: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82FF2A28: 38C00062  li r6, 0x62
	ctx.r[6].s64 = 98;
	// 82FF2A2C: 38A0010B  li r5, 0x10b
	ctx.r[5].s64 = 267;
	// 82FF2A30: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2A34: 4BFFA275  bl 0x82fecca8
	ctx.lr = 0x82FF2A38;
	sub_82FECCA8(ctx, base);
	// 82FF2A38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF2A3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2A40: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF2A44: 481BE1E5  bl 0x831b0c28
	ctx.lr = 0x82FF2A48;
	sub_831B0C28(ctx, base);
	// 82FF2A48: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF2A50: 409A0054  bne cr6, 0x82ff2aa4
	if !ctx.cr[6].eq {
	pc = 0x82FF2AA4; continue 'dispatch;
	}
	// 82FF2A54: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A58: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF2A5C: 409A0048  bne cr6, 0x82ff2aa4
	if !ctx.cr[6].eq {
	pc = 0x82FF2AA4; continue 'dispatch;
	}
	// 82FF2A60: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A64: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2A68: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82FF2A6C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF2A70: 614A8008  ori r10, r10, 0x8008
	ctx.r[10].u64 = ctx.r[10].u64 | 32776;
	// 82FF2A74: 7FCB51AE  stbx r30, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF2A78: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A7C: 396B3002  addi r11, r11, 0x3002
	ctx.r[11].s64 = ctx.r[11].s64 + 12290;
	// 82FF2A80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF2A84: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 82FF2A88: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A8C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF2A90: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF2A94: 7D2BFB2E  sthx r9, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u16) };
	// 82FF2A98: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2A9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF2AA0: 91760000  stw r11, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF2AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2AA8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82FF2AAC: 481B56E0  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2AB0 size=696
    let mut pc: u32 = 0x82FF2AB0;
    'dispatch: loop {
        match pc {
            0x82FF2AB0 => {
    //   block [0x82FF2AB0..0x82FF2D68)
	// 82FF2AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF2AB4: 481B5685  bl 0x831a8138
	ctx.lr = 0x82FF2AB8;
	sub_831A8130(ctx, base);
	// 82FF2AB8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2ABC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2AC0: 82E1017C  lwz r23, 0x17c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 82FF2AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF2AC8: 616BC008  ori r11, r11, 0xc008
	ctx.r[11].u64 = ctx.r[11].u64 | 49160;
	// 82FF2ACC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FF2AD0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FF2AD4: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 82FF2AD8: 3F1F0001  addis r24, r31, 1
	ctx.r[24].s64 = ctx.r[31].s64 + 65536;
	// 82FF2ADC: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 82FF2AE0: 7F5F592E  stwx r26, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[26].u32) };
	// 82FF2AE4: 3EBF0002  addis r21, r31, 2
	ctx.r[21].s64 = ctx.r[31].s64 + 131072;
	// 82FF2AE8: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF2AEC: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 82FF2AF0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2AF4: 7D324B78  mr r18, r9
	ctx.r[18].u64 = ctx.r[9].u64;
	// 82FF2AF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF2AFC: 614AC00C  ori r10, r10, 0xc00c
	ctx.r[10].u64 = ctx.r[10].u64 | 49164;
	// 82FF2B00: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82FF2B04: 6108C018  ori r8, r8, 0xc018
	ctx.r[8].u64 = ctx.r[8].u64 | 49176;
	// 82FF2B08: 616BC019  ori r11, r11, 0xc019
	ctx.r[11].u64 = ctx.r[11].u64 | 49177;
	// 82FF2B0C: 3B188004  addi r24, r24, -0x7ffc
	ctx.r[24].s64 = ctx.r[24].s64 + -32764;
	// 82FF2B10: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2B14: 3BBDC010  addi r29, r29, -0x3ff0
	ctx.r[29].s64 = ctx.r[29].s64 + -16368;
	// 82FF2B18: 7F5F512E  stwx r26, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 82FF2B1C: 3AB5C014  addi r21, r21, -0x3fec
	ctx.r[21].s64 = ctx.r[21].s64 + -16364;
	// 82FF2B20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF2B24: 7F5F41AE  stbx r26, r31, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[26].u8) };
	// 82FF2B28: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FF2B2C: 7FDF59AE  stbx r30, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u8) };
	// 82FF2B30: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF2B34: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2B38: 7CD33378  mr r19, r6
	ctx.r[19].u64 = ctx.r[6].u64;
	// 82FF2B3C: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FF2B40: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FF2B44: 93D50000  stw r30, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2B48: 4BFDE039  bl 0x82fd0b80
	ctx.lr = 0x82FF2B4C;
	sub_82FD0B80(ctx, base);
	// 82FF2B4C: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF2B50: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2B54: 614AC020  ori r10, r10, 0xc020
	ctx.r[10].u64 = ctx.r[10].u64 | 49184;
	// 82FF2B58: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF2B5C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF2B60: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82FF2B64: 61298024  ori r9, r9, 0x8024
	ctx.r[9].u64 = ctx.r[9].u64 | 32804;
	// 82FF2B68: 7FDF512E  stwx r30, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FF2B6C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2B70: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF2B74: 61088028  ori r8, r8, 0x8028
	ctx.r[8].u64 = ctx.r[8].u64 | 32808;
	// 82FF2B78: 614A8030  ori r10, r10, 0x8030
	ctx.r[10].u64 = ctx.r[10].u64 | 32816;
	// 82FF2B7C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FF2B80: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82FF2B84: 3E1F0003  addis r16, r31, 3
	ctx.r[16].s64 = ctx.r[31].s64 + 196608;
	// 82FF2B88: 3EDF0003  addis r22, r31, 3
	ctx.r[22].s64 = ctx.r[31].s64 + 196608;
	// 82FF2B8C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF2B90: 7FDF51AE  stbx r30, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF2B94: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2B98: 7D7F412E  stwx r11, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 82FF2B9C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2BA0: 8901016F  lbz r8, 0x16f(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(367 as u32) ) } as u64;
	// 82FF2BA4: 61298034  ori r9, r9, 0x8034
	ctx.r[9].u64 = ctx.r[9].u64 | 32820;
	// 82FF2BA8: 616B8038  ori r11, r11, 0x8038
	ctx.r[11].u64 = ctx.r[11].u64 | 32824;
	// 82FF2BAC: 614A803D  ori r10, r10, 0x803d
	ctx.r[10].u64 = ctx.r[10].u64 | 32829;
	// 82FF2BB0: 3A10802C  addi r16, r16, -0x7fd4
	ctx.r[16].s64 = ctx.r[16].s64 + -32724;
	// 82FF2BB4: 3AD6803C  addi r22, r22, -0x7fc4
	ctx.r[22].s64 = ctx.r[22].s64 + -32708;
	// 82FF2BB8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FF2BBC: 7E9F492E  stwx r20, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[20].u32) };
	// 82FF2BC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF2BC4: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 82FF2BC8: 7D1F51AE  stbx r8, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u8) };
	// 82FF2BCC: 93700000  stw r27, 0(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF2BD0: 9BD60000  stb r30, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF2BD4: 4BFDDFAD  bl 0x82fd0b80
	ctx.lr = 0x82FF2BD8;
	sub_82FD0B80(ctx, base);
	// 82FF2BD8: 3F7F0003  addis r27, r31, 3
	ctx.r[27].s64 = ctx.r[31].s64 + 196608;
	// 82FF2BDC: 3E9F0003  addis r20, r31, 3
	ctx.r[20].s64 = ctx.r[31].s64 + 196608;
	// 82FF2BE0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2BE4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF2BE8: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF2BEC: 3B7B8048  addi r27, r27, -0x7fb8
	ctx.r[27].s64 = ctx.r[27].s64 + -32696;
	// 82FF2BF0: 3A94804C  addi r20, r20, -0x7fb4
	ctx.r[20].s64 = ctx.r[20].s64 + -32692;
	// 82FF2BF4: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FF2BF8: 614A8044  ori r10, r10, 0x8044
	ctx.r[10].u64 = ctx.r[10].u64 | 32836;
	// 82FF2BFC: 61298049  ori r9, r9, 0x8049
	ctx.r[9].u64 = ctx.r[9].u64 | 32841;
	// 82FF2C00: 3E3F0003  addis r17, r31, 3
	ctx.r[17].s64 = ctx.r[31].s64 + 196608;
	// 82FF2C04: 89010167  lbz r8, 0x167(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(359 as u32) ) } as u64;
	// 82FF2C08: 3F9F0003  addis r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 196608;
	// 82FF2C0C: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF2C10: 3A318050  addi r17, r17, -0x7fb0
	ctx.r[17].s64 = ctx.r[17].s64 + -32688;
	// 82FF2C14: 80810174  lwz r4, 0x174(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FF2C18: 3B9C8060  addi r28, r28, -0x7fa0
	ctx.r[28].s64 = ctx.r[28].s64 + -32672;
	// 82FF2C1C: 7E7F512E  stwx r19, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[19].u32) };
	// 82FF2C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2C24: 9BDB0000  stb r30, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF2C28: 7D1F49AE  stbx r8, r31, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82FF2C2C: 93D40000  stw r30, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF2C30: 92510000  stw r18, 0(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 82FF2C34: 92FC0000  stw r23, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82FF2C38: 4BFF5031  bl 0x82fe7c68
	ctx.lr = 0x82FF2C3C;
	sub_82FE7C68(ctx, base);
	// 82FF2C3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2C40: 4BFFFA39  bl 0x82ff2678
	ctx.lr = 0x82FF2C44;
	sub_82FF2678(ctx, base);
	// 82FF2C44: 3E608339  lis r19, -0x7cc7
	ctx.r[19].s64 = -2093416448;
	// 82FF2C48: 8073B7DC  lwz r3, -0x4824(r19)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF2C4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2C50: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF2C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF2C58: 4E800421  bctrl
	ctx.lr = 0x82FF2C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF2C5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF2C60: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2C64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FF2C68: 933D0000  stw r25, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82FF2C6C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FF2C70: 99760000  stb r11, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FF2C74: 480431C5  bl 0x83035e38
	ctx.lr = 0x82FF2C78;
	sub_83035E38(ctx, base);
	// 82FF2C78: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FF2C7C: 4BFDDF05  bl 0x82fd0b80
	ctx.lr = 0x82FF2C80;
	sub_82FD0B80(ctx, base);
	// 82FF2C80: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2C84: 90750000  stw r3, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2C88: 9BDB0000  stb r30, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF2C8C: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 82FF2C90: 419A000C  beq cr6, 0x82ff2c9c
	if ctx.cr[6].eq {
	pc = 0x82FF2C9C; continue 'dispatch;
	}
	// 82FF2C94: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82FF2C98: 409A0008  bne cr6, 0x82ff2ca0
	if !ctx.cr[6].eq {
	pc = 0x82FF2CA0; continue 'dispatch;
	}
	// 82FF2C9C: 9B5B0000  stb r26, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 82FF2CA0: 38C04000  li r6, 0x4000
	ctx.r[6].s64 = 16384;
	// 82FF2CA4: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2CA8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FF2CAC: 8073B7DC  lwz r3, -0x4824(r19)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF2CB0: 48003421  bl 0x82ff60d0
	ctx.lr = 0x82FF2CB4;
	sub_82FF60D0(ctx, base);
	// 82FF2CB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2CB8: 90740000  stw r3, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2CBC: 40820044  bne 0x82ff2d00
	if !ctx.cr[0].eq {
	pc = 0x82FF2D00; continue 'dispatch;
	}
	// 82FF2CC0: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2CC4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2CCC: 80F50000  lwz r7, 0(r21)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2CD0: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF2CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF2CD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF2CDC: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82FF2CE0: 38C00062  li r6, 0x62
	ctx.r[6].s64 = 98;
	// 82FF2CE4: 38A0016B  li r5, 0x16b
	ctx.r[5].s64 = 363;
	// 82FF2CE8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2CEC: 4BFF9FBD  bl 0x82fecca8
	ctx.lr = 0x82FF2CF0;
	sub_82FECCA8(ctx, base);
	// 82FF2CF0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF2CF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2CF8: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF2CFC: 481BDF2D  bl 0x831b0c28
	ctx.lr = 0x82FF2D00;
	sub_831B0C28(ctx, base);
	// 82FF2D00: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF2D08: 409A0054  bne cr6, 0x82ff2d5c
	if !ctx.cr[6].eq {
	pc = 0x82FF2D5C; continue 'dispatch;
	}
	// 82FF2D0C: 81700000  lwz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D10: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF2D14: 409A0048  bne cr6, 0x82ff2d5c
	if !ctx.cr[6].eq {
	pc = 0x82FF2D5C; continue 'dispatch;
	}
	// 82FF2D18: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D1C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2D20: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82FF2D24: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF2D28: 614A8008  ori r10, r10, 0x8008
	ctx.r[10].u64 = ctx.r[10].u64 | 32776;
	// 82FF2D2C: 7FCB51AE  stbx r30, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF2D30: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D34: 396B3002  addi r11, r11, 0x3002
	ctx.r[11].s64 = ctx.r[11].s64 + 12290;
	// 82FF2D38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF2D3C: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 82FF2D40: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D44: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF2D48: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF2D4C: 7D2BFB2E  sthx r9, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u16) };
	// 82FF2D50: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF2D58: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF2D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2D60: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82FF2D64: 481B5424  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF2D68 size=924
    let mut pc: u32 = 0x82FF2D68;
    'dispatch: loop {
        match pc {
            0x82FF2D68 => {
    //   block [0x82FF2D68..0x82FF3104)
	// 82FF2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF2D6C: 481B53E5  bl 0x831a8150
	ctx.lr = 0x82FF2D70;
	sub_831A8130(ctx, base);
	// 82FF2D70: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF2D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF2D78: 3EDF0002  addis r22, r31, 2
	ctx.r[22].s64 = ctx.r[31].s64 + 131072;
	// 82FF2D7C: 3AD6C019  addi r22, r22, -0x3fe7
	ctx.r[22].s64 = ctx.r[22].s64 + -16359;
	// 82FF2D80: 89760000  lbz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2D88: 4182000C  beq 0x82ff2d94
	if ctx.cr[0].eq {
	pc = 0x82FF2D94; continue 'dispatch;
	}
	// 82FF2D8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF2D90: 4800036C  b 0x82ff30fc
	pc = 0x82FF30FC; continue 'dispatch;
	// 82FF2D94: 3F5F0001  addis r26, r31, 1
	ctx.r[26].s64 = ctx.r[31].s64 + 65536;
	// 82FF2D98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2D9C: 3B5A8004  addi r26, r26, -0x7ffc
	ctx.r[26].s64 = ctx.r[26].s64 + -32764;
	// 82FF2DA0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2DA4: 7F6B5050  subf r27, r11, r10
	ctx.r[27].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FF2DA8: 2B1B4000  cmplwi cr6, r27, 0x4000
	ctx.cr[6].compare_u32(ctx.r[27].u32, 16384 as u32, &mut ctx.xer);
	// 82FF2DAC: 409A000C  bne cr6, 0x82ff2db8
	if !ctx.cr[6].eq {
	pc = 0x82FF2DB8; continue 'dispatch;
	}
	// 82FF2DB0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF2DB4: 48000348  b 0x82ff30fc
	pc = 0x82FF30FC; continue 'dispatch;
	// 82FF2DB8: 3F9F0003  addis r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 196608;
	// 82FF2DBC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF2DC0: 3B9C804C  addi r28, r28, -0x7fb4
	ctx.r[28].s64 = ctx.r[28].s64 + -32692;
	// 82FF2DC4: 6158C010  ori r24, r10, 0xc010
	ctx.r[24].u64 = ctx.r[10].u64 | 49168;
	// 82FF2DC8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF2DD0: 409A00BC  bne cr6, 0x82ff2e8c
	if !ctx.cr[6].eq {
	pc = 0x82FF2E8C; continue 'dispatch;
	}
	// 82FF2DD4: 7D7FC02E  lwzx r11, r31, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FF2DD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF2DDC: 409A0038  bne cr6, 0x82ff2e14
	if !ctx.cr[6].eq {
	pc = 0x82FF2E14; continue 'dispatch;
	}
	// 82FF2DE0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2DE4: 38C0004B  li r6, 0x4b
	ctx.r[6].s64 = 75;
	// 82FF2DE8: 616A8060  ori r10, r11, 0x8060
	ctx.r[10].u64 = ctx.r[11].u64 | 32864;
	// 82FF2DEC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2DF0: 38A001BD  li r5, 0x1bd
	ctx.r[5].s64 = 445;
	// 82FF2DF4: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF2DF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2DFC: 7CFF502E  lwzx r7, r31, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF2E00: 4BFDE259  bl 0x82fd1058
	ctx.lr = 0x82FF2E04;
	sub_82FD1058(ctx, base);
	// 82FF2E04: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF2E08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2E0C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FF2E10: 481BDE19  bl 0x831b0c28
	ctx.lr = 0x82FF2E14;
	sub_831B0C28(ctx, base);
	// 82FF2E14: 3FDF0003  addis r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 196608;
	// 82FF2E18: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 82FF2E1C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF2E20: 3BDE8060  addi r30, r30, -0x7fa0
	ctx.r[30].s64 = ctx.r[30].s64 + -32672;
	// 82FF2E24: 3BBDC014  addi r29, r29, -0x3fec
	ctx.r[29].s64 = ctx.r[29].s64 + -16364;
	// 82FF2E28: 38C04000  li r6, 0x4000
	ctx.r[6].s64 = 16384;
	// 82FF2E2C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FF2E30: 806BB7DC  lwz r3, -0x4824(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF2E34: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2E38: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2E3C: 48003395  bl 0x82ff61d0
	ctx.lr = 0x82FF2E40;
	sub_82FF61D0(ctx, base);
	// 82FF2E40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2E44: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF2E48: 40820044  bne 0x82ff2e8c
	if !ctx.cr[0].eq {
	pc = 0x82FF2E8C; continue 'dispatch;
	}
	// 82FF2E4C: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2E50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF2E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2E58: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2E5C: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF2E60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF2E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF2E68: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82FF2E6C: 38C00062  li r6, 0x62
	ctx.r[6].s64 = 98;
	// 82FF2E70: 38A001D1  li r5, 0x1d1
	ctx.r[5].s64 = 465;
	// 82FF2E74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2E78: 4BFF9E31  bl 0x82fecca8
	ctx.lr = 0x82FF2E7C;
	sub_82FECCA8(ctx, base);
	// 82FF2E7C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF2E80: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF2E84: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF2E88: 481BDDA1  bl 0x831b0c28
	ctx.lr = 0x82FF2E8C;
	sub_831B0C28(ctx, base);
	// 82FF2E8C: 3EFF0003  addis r23, r31, 3
	ctx.r[23].s64 = ctx.r[31].s64 + 196608;
	// 82FF2E90: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FF2E94: 3AF7803D  addi r23, r23, -0x7fc3
	ctx.r[23].s64 = ctx.r[23].s64 + -32707;
	// 82FF2E98: 61478008  ori r7, r10, 0x8008
	ctx.r[7].u64 = ctx.r[10].u64 | 32776;
	// 82FF2E9C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FF2EA0: 89770000  lbz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2EA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2EA8: 41820040  beq 0x82ff2ee8
	if ctx.cr[0].eq {
	pc = 0x82FF2EE8; continue 'dispatch;
	}
	// 82FF2EAC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2EB0: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82FF2EB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FF2EB8: 40990030  ble cr6, 0x82ff2ee8
	if !ctx.cr[6].gt {
	pc = 0x82FF2EE8; continue 'dispatch;
	}
	// 82FF2EBC: 3D5F0003  addis r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 196608;
	// 82FF2EC0: 7CDF3A14  add r6, r31, r7
	ctx.r[6].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82FF2EC4: 394A8038  addi r10, r10, -0x7fc8
	ctx.r[10].s64 = ctx.r[10].s64 + -32712;
	// 82FF2EC8: 7D2658AE  lbzx r9, r6, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2ECC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF2ED0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2ED4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FF2ED8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FF2EDC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2EE0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF2EE4: 4198FFE4  blt cr6, 0x82ff2ec8
	if ctx.cr[6].lt {
	pc = 0x82FF2EC8; continue 'dispatch;
	}
	// 82FF2EE8: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82FF2EEC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FF2EF0: 419A0054  beq cr6, 0x82ff2f44
	if ctx.cr[6].eq {
	pc = 0x82FF2F44; continue 'dispatch;
	}
	// 82FF2EF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2EF8: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2EFC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF2F00: 40980044  bge cr6, 0x82ff2f44
	if !ctx.cr[6].lt {
	pc = 0x82FF2F44; continue 'dispatch;
	}
	// 82FF2F04: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF2F08: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 82FF2F0C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF2F10: 7D1F3A14  add r8, r31, r7
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82FF2F14: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82FF2F18: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2F1C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF2F20: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82FF2F24: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FF2F28: 7CC858AE  lbzx r6, r8, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF2F30: 7CC8F1AE  stbx r6, r8, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32), ctx.r[6].u8) };
	// 82FF2F34: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FF2F38: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2F3C: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82FF2F40: 4198FFD8  blt cr6, 0x82ff2f18
	if ctx.cr[6].lt {
	pc = 0x82FF2F18; continue 'dispatch;
	}
	// 82FF2F44: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FF2F48: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82FF2F4C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF2F50: 20DB4000  subfic r6, r27, 0x4000
	ctx.xer.ca = ctx.r[27].u32 <= 16384 as u32;
	ctx.r[6].s64 = (16384 as i64) - ctx.r[27].s64;
	// 82FF2F54: 7CAA3A14  add r5, r10, r7
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82FF2F58: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF2F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF2F60: 4BFFF7C1  bl 0x82ff2720
	ctx.lr = 0x82FF2F64;
	sub_82FF2720(ctx, base);
	// 82FF2F64: 7D63DA15  add. r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF2F68: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82FF2F6C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82FF2F70: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF2F74: 40820050  bne 0x82ff2fc4
	if !ctx.cr[0].eq {
	pc = 0x82FF2FC4; continue 'dispatch;
	}
	// 82FF2F78: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2F7C: 616B8050  ori r11, r11, 0x8050
	ctx.r[11].u64 = ctx.r[11].u64 | 32848;
	// 82FF2F80: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2F84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF2F88: 409A003C  bne cr6, 0x82ff2fc4
	if !ctx.cr[6].eq {
	pc = 0x82FF2FC4; continue 'dispatch;
	}
	// 82FF2F8C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF2F90: 616B802C  ori r11, r11, 0x802c
	ctx.r[11].u64 = ctx.r[11].u64 | 32812;
	// 82FF2F94: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2F98: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF2F9C: 409A0028  bne cr6, 0x82ff2fc4
	if !ctx.cr[6].eq {
	pc = 0x82FF2FC4; continue 'dispatch;
	}
	// 82FF2FA0: 3D7F0003  addis r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 196608;
	// 82FF2FA4: 396B8030  addi r11, r11, -0x7fd0
	ctx.r[11].s64 = ctx.r[11].s64 + -32720;
	// 82FF2FA8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2FAC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF2FB0: 40820014  bne 0x82ff2fc4
	if !ctx.cr[0].eq {
	pc = 0x82FF2FC4; continue 'dispatch;
	}
	// 82FF2FB4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82FF2FB8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FF2FBC: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82FF2FC0: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FF2FC4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF2FC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF2FCC: 419A00B4  beq cr6, 0x82ff3080
	if ctx.cr[6].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF2FD0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2FD4: 616BC00C  ori r11, r11, 0xc00c
	ctx.r[11].u64 = ctx.r[11].u64 | 49164;
	// 82FF2FD8: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2FDC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF2FE0: 409A00A0  bne cr6, 0x82ff3080
	if !ctx.cr[6].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF2FE4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF2FE8: 616BC008  ori r11, r11, 0xc008
	ctx.r[11].u64 = ctx.r[11].u64 | 49160;
	// 82FF2FEC: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF2FF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF2FF4: 409A008C  bne cr6, 0x82ff3080
	if !ctx.cr[6].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF2FF8: 7D7FC02E  lwzx r11, r31, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82FF2FFC: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FF3000: 419A000C  beq cr6, 0x82ff300c
	if ctx.cr[6].eq {
	pc = 0x82FF300C; continue 'dispatch;
	}
	// 82FF3004: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FF3008: 409A0028  bne cr6, 0x82ff3030
	if !ctx.cr[6].eq {
	pc = 0x82FF3030; continue 'dispatch;
	}
	// 82FF300C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF3010: 409A0020  bne cr6, 0x82ff3030
	if !ctx.cr[6].eq {
	pc = 0x82FF3030; continue 'dispatch;
	}
	// 82FF3014: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF3018: 2B0BFEFF  cmplwi cr6, r11, 0xfeff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65279 as u32, &mut ctx.xer);
	// 82FF301C: 419A000C  beq cr6, 0x82ff3028
	if ctx.cr[6].eq {
	pc = 0x82FF3028; continue 'dispatch;
	}
	// 82FF3020: 2B0BFFFE  cmplwi cr6, r11, 0xfffe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65534 as u32, &mut ctx.xer);
	// 82FF3024: 409A005C  bne cr6, 0x82ff3080
	if !ctx.cr[6].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF3028: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FF302C: 48000054  b 0x82ff3080
	pc = 0x82FF3080; continue 'dispatch;
	// 82FF3030: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF3034: 3FA08215  lis r29, -0x7deb
	ctx.r[29].s64 = -2112552960;
	// 82FF3038: 616B8024  ori r11, r11, 0x8024
	ctx.r[11].u64 = ctx.r[11].u64 | 32804;
	// 82FF303C: 80BD8E0C  lwz r5, -0x71f4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-29172 as u32) ) } as u64;
	// 82FF3040: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3044: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FF3048: 40990038  ble cr6, 0x82ff3080
	if !ctx.cr[6].gt {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF304C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FF3050: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 82FF3054: 388B8E08  addi r4, r11, -0x71f8
	ctx.r[4].s64 = ctx.r[11].s64 + -29176;
	// 82FF3058: 3863C024  addi r3, r3, -0x3fdc
	ctx.r[3].s64 = ctx.r[3].s64 + -16348;
	// 82FF305C: 4BFDE1BD  bl 0x82fd1218
	ctx.lr = 0x82FF3060;
	sub_82FD1218(ctx, base);
	// 82FF3060: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF3064: 4082001C  bne 0x82ff3080
	if !ctx.cr[0].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF3068: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF306C: 409A0014  bne cr6, 0x82ff3080
	if !ctx.cr[6].eq {
	pc = 0x82FF3080; continue 'dispatch;
	}
	// 82FF3070: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3074: 817D8E0C  lwz r11, -0x71f4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-29172 as u32) ) } as u64;
	// 82FF3078: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF307C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3080: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3084: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3088: 40820008  bne 0x82ff3090
	if !ctx.cr[0].eq {
	pc = 0x82FF3090; continue 'dispatch;
	}
	// 82FF308C: 9B960000  stb r28, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82FF3090: 89570000  lbz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3094: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3098: 41820050  beq 0x82ff30e8
	if ctx.cr[0].eq {
	pc = 0x82FF30E8; continue 'dispatch;
	}
	// 82FF309C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FF30A0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FF30A4: 6149C008  ori r9, r10, 0xc008
	ctx.r[9].u64 = ctx.r[10].u64 | 49160;
	// 82FF30A8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82FF30AC: 7F3F492E  stwx r25, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u32) };
	// 82FF30B0: 40990038  ble cr6, 0x82ff30e8
	if !ctx.cr[6].gt {
	pc = 0x82FF30E8; continue 'dispatch;
	}
	// 82FF30B4: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 82FF30B8: 3CFF0001  addis r7, r31, 1
	ctx.r[7].s64 = ctx.r[31].s64 + 65536;
	// 82FF30BC: 396BC00C  addi r11, r11, -0x3ff4
	ctx.r[11].s64 = ctx.r[11].s64 + -16372;
	// 82FF30C0: 38E78007  addi r7, r7, -0x7ff9
	ctx.r[7].s64 = ctx.r[7].s64 + -32761;
	// 82FF30C4: 7D2750AE  lbzx r9, r7, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF30C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF30CC: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FF30D0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82FF30D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FF30D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FF30DC: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF30E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF30E4: 4198FFE0  blt cr6, 0x82ff30c4
	if ctx.cr[6].lt {
	pc = 0x82FF30C4; continue 'dispatch;
	}
	// 82FF30E8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF30EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF30F0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF30F4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF30F8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FF30FC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82FF3100: 481B50A0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3108 size=632
    let mut pc: u32 = 0x82FF3108;
    'dispatch: loop {
        match pc {
            0x82FF3108 => {
    //   block [0x82FF3108..0x82FF3380)
	// 82FF3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF310C: 481B504D  bl 0x831a8158
	ctx.lr = 0x82FF3110;
	sub_831A8130(ctx, base);
	// 82FF3110: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF3118: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF311C: 3F1F0001  addis r24, r31, 1
	ctx.r[24].s64 = ctx.r[31].s64 + 65536;
	// 82FF3120: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF3124: 3B188004  addi r24, r24, -0x7ffc
	ctx.r[24].s64 = ctx.r[24].s64 + -32764;
	// 82FF3128: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF312C: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3130: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3134: 409A0018  bne cr6, 0x82ff314c
	if !ctx.cr[6].eq {
	pc = 0x82FF314C; continue 'dispatch;
	}
	// 82FF3138: 4BFFFC31  bl 0x82ff2d68
	ctx.lr = 0x82FF313C;
	sub_82FF2D68(ctx, base);
	// 82FF313C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF3140: 4082000C  bne 0x82ff314c
	if !ctx.cr[0].eq {
	pc = 0x82FF314C; continue 'dispatch;
	}
	// 82FF3144: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF3148: 48000230  b 0x82ff3378
	pc = 0x82FF3378; continue 'dispatch;
	// 82FF314C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF3150: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF3154: 6159805C  ori r25, r10, 0x805c
	ctx.r[25].u64 = ctx.r[10].u64 | 32860;
	// 82FF3158: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF315C: 615DC008  ori r29, r10, 0xc008
	ctx.r[29].u64 = ctx.r[10].u64 | 49160;
	// 82FF3160: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF3164: 615A8054  ori r26, r10, 0x8054
	ctx.r[26].u64 = ctx.r[10].u64 | 32852;
	// 82FF3168: 408200E4  bne 0x82ff324c
	if !ctx.cr[0].eq {
	pc = 0x82FF324C; continue 'dispatch;
	}
	// 82FF316C: 7D7FC82E  lwzx r11, r31, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82FF3170: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF3174: 409A0094  bne cr6, 0x82ff3208
	if !ctx.cr[6].eq {
	pc = 0x82FF3208; continue 'dispatch;
	}
	// 82FF3178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF317C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF3180: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3184: 7C8AFA2E  lhzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF3188: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FF318C: 2B0AD800  cmplwi cr6, r10, 0xd800
	ctx.cr[6].compare_u32(ctx.r[10].u32, 55296 as u32, &mut ctx.xer);
	// 82FF3190: 41980078  blt cr6, 0x82ff3208
	if ctx.cr[6].lt {
	pc = 0x82FF3208; continue 'dispatch;
	}
	// 82FF3194: 2B0ADB7F  cmplwi cr6, r10, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56191 as u32, &mut ctx.xer);
	// 82FF3198: 41990070  bgt cr6, 0x82ff3208
	if ctx.cr[6].gt {
	pc = 0x82FF3208; continue 'dispatch;
	}
	// 82FF319C: 394B0003  addi r10, r11, 3
	ctx.r[10].s64 = ctx.r[11].s64 + 3;
	// 82FF31A0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF31A4: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF31A8: 2B0ADC00  cmplwi cr6, r10, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56320 as u32, &mut ctx.xer);
	// 82FF31AC: 4198FF98  blt cr6, 0x82ff3144
	if ctx.cr[6].lt {
	pc = 0x82FF3144; continue 'dispatch;
	}
	// 82FF31B0: 2B0ADFFF  cmplwi cr6, r10, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57343 as u32, &mut ctx.xer);
	// 82FF31B4: 4199FF90  bgt cr6, 0x82ff3144
	if ctx.cr[6].gt {
	pc = 0x82FF3144; continue 'dispatch;
	}
	// 82FF31B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF31BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF31C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF31C4: 4BFDD955  bl 0x82fd0b18
	ctx.lr = 0x82FF31C8;
	sub_82FD0B18(ctx, base);
	// 82FF31C8: 7FDFEA14  add r30, r31, r29
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FF31CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF31D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF31D4: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FF31D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF31DC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF31E0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF31E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF31E8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF31EC: 7C89FA2E  lhzx r4, r9, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF31F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF31F4: 4BFDD925  bl 0x82fd0b18
	ctx.lr = 0x82FF31F8;
	sub_82FD0B18(ctx, base);
	// 82FF31F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF31FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3200: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3204: 48000048  b 0x82ff324c
	pc = 0x82FF324C; continue 'dispatch;
	// 82FF3208: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF320C: 7D5FD02E  lwzx r10, r31, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FF3210: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FF3214: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF3218: 7C89FA2E  lhzx r4, r9, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF321C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FF3220: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3224: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF3228: 4182FF1C  beq 0x82ff3144
	if ctx.cr[0].eq {
	pc = 0x82FF3144; continue 'dispatch;
	}
	// 82FF322C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3230: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF3234: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3238: 4BFDD8E1  bl 0x82fd0b18
	ctx.lr = 0x82FF323C;
	sub_82FD0B18(ctx, base);
	// 82FF323C: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FF3240: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3244: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3248: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF324C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3250: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3254: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3258: 409800F8  bge cr6, 0x82ff3350
	if !ctx.cr[6].lt {
	pc = 0x82FF3350; continue 'dispatch;
	}
	// 82FF325C: 7F7FCA14  add r27, r31, r25
	ctx.r[27].u64 = ctx.r[31].u64 + ctx.r[25].u64;
	// 82FF3260: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3264: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF3268: 409A0094  bne cr6, 0x82ff32fc
	if !ctx.cr[6].eq {
	pc = 0x82FF32FC; continue 'dispatch;
	}
	// 82FF326C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3270: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF3274: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3278: 7C8AFA2E  lhzx r4, r10, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF327C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FF3280: 2B0AD800  cmplwi cr6, r10, 0xd800
	ctx.cr[6].compare_u32(ctx.r[10].u32, 55296 as u32, &mut ctx.xer);
	// 82FF3284: 41980078  blt cr6, 0x82ff32fc
	if ctx.cr[6].lt {
	pc = 0x82FF32FC; continue 'dispatch;
	}
	// 82FF3288: 2B0ADB7F  cmplwi cr6, r10, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56191 as u32, &mut ctx.xer);
	// 82FF328C: 41990070  bgt cr6, 0x82ff32fc
	if ctx.cr[6].gt {
	pc = 0x82FF32FC; continue 'dispatch;
	}
	// 82FF3290: 394B0003  addi r10, r11, 3
	ctx.r[10].s64 = ctx.r[11].s64 + 3;
	// 82FF3294: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3298: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF329C: 2B0ADC00  cmplwi cr6, r10, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56320 as u32, &mut ctx.xer);
	// 82FF32A0: 419800C0  blt cr6, 0x82ff3360
	if ctx.cr[6].lt {
	pc = 0x82FF3360; continue 'dispatch;
	}
	// 82FF32A4: 2B0ADFFF  cmplwi cr6, r10, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57343 as u32, &mut ctx.xer);
	// 82FF32A8: 419900B8  bgt cr6, 0x82ff3360
	if ctx.cr[6].gt {
	pc = 0x82FF3360; continue 'dispatch;
	}
	// 82FF32AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF32B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF32B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF32B8: 4BFDD861  bl 0x82fd0b18
	ctx.lr = 0x82FF32BC;
	sub_82FD0B18(ctx, base);
	// 82FF32BC: 7FDFEA14  add r30, r31, r29
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FF32C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF32C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF32C8: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FF32CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF32D0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF32D4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF32D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF32DC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF32E0: 7C89FA2E  lhzx r4, r9, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF32E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF32E8: 4BFDD831  bl 0x82fd0b18
	ctx.lr = 0x82FF32EC;
	sub_82FD0B18(ctx, base);
	// 82FF32EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF32F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF32F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF32F8: 48000048  b 0x82ff3340
	pc = 0x82FF3340; continue 'dispatch;
	// 82FF32FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3300: 7D5FD02E  lwzx r10, r31, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82FF3304: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FF3308: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF330C: 7C89FA2E  lhzx r4, r9, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF3310: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82FF3314: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3318: 554A077B  rlwinm. r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF331C: 41820044  beq 0x82ff3360
	if ctx.cr[0].eq {
	pc = 0x82FF3360; continue 'dispatch;
	}
	// 82FF3320: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3324: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF3328: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF332C: 4BFDD7ED  bl 0x82fd0b18
	ctx.lr = 0x82FF3330;
	sub_82FD0B18(ctx, base);
	// 82FF3330: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FF3334: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3338: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF333C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3344: 81580000  lwz r10, 0(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3348: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF334C: 4198FF14  blt cr6, 0x82ff3260
	if ctx.cr[6].lt {
	pc = 0x82FF3260; continue 'dispatch;
	}
	// 82FF3350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3354: 4BFFFA15  bl 0x82ff2d68
	ctx.lr = 0x82FF3358;
	sub_82FF2D68(ctx, base);
	// 82FF3358: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF335C: 4082FEF0  bne 0x82ff324c
	if !ctx.cr[0].eq {
	pc = 0x82FF324C; continue 'dispatch;
	}
	// 82FF3360: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF3364: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF3368: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF336C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF3370: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF3374: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF3378: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FF337C: 481B4E2C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3380 size=172
    let mut pc: u32 = 0x82FF3380;
    'dispatch: loop {
        match pc {
            0x82FF3380 => {
    //   block [0x82FF3380..0x82FF342C)
	// 82FF3380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF3384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF3388: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF338C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF3390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3394: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF3398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF339C: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FF33A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF33A4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF33A8: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF33AC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF33B0: 409A0018  bne cr6, 0x82ff33c8
	if !ctx.cr[6].eq {
	pc = 0x82FF33C8; continue 'dispatch;
	}
	// 82FF33B4: 4BFFF9B5  bl 0x82ff2d68
	ctx.lr = 0x82FF33B8;
	sub_82FF2D68(ctx, base);
	// 82FF33B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF33BC: 4082000C  bne 0x82ff33c8
	if !ctx.cr[0].eq {
	pc = 0x82FF33C8; continue 'dispatch;
	}
	// 82FF33C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF33C4: 48000050  b 0x82ff3414
	pc = 0x82FF3414; continue 'dispatch;
	// 82FF33C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF33CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF33D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF33D4: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF33D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82FF33DC: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 82FF33E0: 419A000C  beq cr6, 0x82ff33ec
	if ctx.cr[6].eq {
	pc = 0x82FF33EC; continue 'dispatch;
	}
	// 82FF33E4: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 82FF33E8: 409AFFD8  bne cr6, 0x82ff33c0
	if !ctx.cr[6].eq {
	pc = 0x82FF33C0; continue 'dispatch;
	}
	// 82FF33EC: B15E0000  sth r10, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FF33F0: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FF33F4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF33F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF33FC: 396BC008  addi r11, r11, -0x3ff8
	ctx.r[11].s64 = ctx.r[11].s64 + -16376;
	// 82FF3400: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3404: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3408: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF340C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3410: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF3418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF341C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF3420: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF3424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF3428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3430 size=156
    let mut pc: u32 = 0x82FF3430;
    'dispatch: loop {
        match pc {
            0x82FF3430 => {
    //   block [0x82FF3430..0x82FF34CC)
	// 82FF3430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF3434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF3438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF343C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF3440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3444: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF3448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF344C: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FF3450: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF3454: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3458: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF345C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF3460: 409A0010  bne cr6, 0x82ff3470
	if !ctx.cr[6].eq {
	pc = 0x82FF3470; continue 'dispatch;
	}
	// 82FF3464: 4BFFF905  bl 0x82ff2d68
	ctx.lr = 0x82FF3468;
	sub_82FF2D68(ctx, base);
	// 82FF3468: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF346C: 41820044  beq 0x82ff34b0
	if ctx.cr[0].eq {
	pc = 0x82FF34B0; continue 'dispatch;
	}
	// 82FF3470: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3474: 57CA043E  clrlwi r10, r30, 0x10
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 82FF3478: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FF347C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF3480: 7D29FA2E  lhzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF3484: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3488: 409A0028  bne cr6, 0x82ff34b0
	if !ctx.cr[6].eq {
	pc = 0x82FF34B0; continue 'dispatch;
	}
	// 82FF348C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FF3490: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FF3494: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF3498: 396BC008  addi r11, r11, -0x3ff8
	ctx.r[11].s64 = ctx.r[11].s64 + -16376;
	// 82FF349C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF34A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF34A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF34A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF34AC: 48000008  b 0x82ff34b4
	pc = 0x82FF34B4; continue 'dispatch;
	// 82FF34B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF34B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF34B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF34BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF34C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF34C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF34C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF34D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF34D0 size=236
    let mut pc: u32 = 0x82FF34D0;
    'dispatch: loop {
        match pc {
            0x82FF34D0 => {
    //   block [0x82FF34D0..0x82FF35BC)
	// 82FF34D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF34D4: 481B4C91  bl 0x831a8164
	ctx.lr = 0x82FF34D8;
	sub_831A8130(ctx, base);
	// 82FF34D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF34DC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FF34E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF34E4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FF34E8: 419A0034  beq cr6, 0x82ff351c
	if ctx.cr[6].eq {
	pc = 0x82FF351C; continue 'dispatch;
	}
	// 82FF34EC: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF34F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF34F4: 41820028  beq 0x82ff351c
	if ctx.cr[0].eq {
	pc = 0x82FF351C; continue 'dispatch;
	}
	// 82FF34F8: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FF34FC: 48000008  b 0x82ff3504
	pc = 0x82FF3504; continue 'dispatch;
	// 82FF3500: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF3504: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3508: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF350C: 4082FFF4  bne 0x82ff3500
	if !ctx.cr[0].eq {
	pc = 0x82FF3500; continue 'dispatch;
	}
	// 82FF3510: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FF3514: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF3518: 48000008  b 0x82ff3520
	pc = 0x82FF3520; continue 'dispatch;
	// 82FF351C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF3520: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 82FF3524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3528: 3B9C8004  addi r28, r28, -0x7ffc
	ctx.r[28].s64 = ctx.r[28].s64 + -32764;
	// 82FF352C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3530: 7FAB5050  subf r29, r11, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FF3534: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF3538: 4098002C  bge cr6, 0x82ff3564
	if !ctx.cr[6].lt {
	pc = 0x82FF3564; continue 'dispatch;
	}
	// 82FF353C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3540: 4BFFF829  bl 0x82ff2d68
	ctx.lr = 0x82FF3544;
	sub_82FF2D68(ctx, base);
	// 82FF3544: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3548: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF354C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FF3550: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FF3554: 419A0060  beq cr6, 0x82ff35b4
	if ctx.cr[6].eq {
	pc = 0x82FF35B4; continue 'dispatch;
	}
	// 82FF3558: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82FF355C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF3560: 4198FFDC  blt cr6, 0x82ff353c
	if ctx.cr[6].lt {
	pc = 0x82FF353C; continue 'dispatch;
	}
	// 82FF3564: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3568: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF356C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF3570: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF3574: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF3578: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF357C: 4BFDE3C5  bl 0x82fd1940
	ctx.lr = 0x82FF3580;
	sub_82FD1940(ctx, base);
	// 82FF3580: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF3584: 40820030  bne 0x82ff35b4
	if !ctx.cr[0].eq {
	pc = 0x82FF35B4; continue 'dispatch;
	}
	// 82FF3588: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF358C: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 82FF3590: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF3594: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82FF3598: 396BC008  addi r11, r11, -0x3ff8
	ctx.r[11].s64 = ctx.r[11].s64 + -16376;
	// 82FF359C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF35A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF35A4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82FF35A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF35AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF35B0: 481B4C04  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FF35B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF35B8: 4BFFFFF4  b 0x82ff35ac
	pc = 0x82FF35AC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF35C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF35C0 size=200
    let mut pc: u32 = 0x82FF35C0;
    'dispatch: loop {
        match pc {
            0x82FF35C0 => {
    //   block [0x82FF35C0..0x82FF3688)
	// 82FF35C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF35C4: 481B4BA1  bl 0x831a8164
	ctx.lr = 0x82FF35C8;
	sub_831A8130(ctx, base);
	// 82FF35C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF35CC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FF35D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF35D4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FF35D8: 419A0034  beq cr6, 0x82ff360c
	if ctx.cr[6].eq {
	pc = 0x82FF360C; continue 'dispatch;
	}
	// 82FF35DC: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF35E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF35E4: 41820028  beq 0x82ff360c
	if ctx.cr[0].eq {
	pc = 0x82FF360C; continue 'dispatch;
	}
	// 82FF35E8: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FF35EC: 48000008  b 0x82ff35f4
	pc = 0x82FF35F4; continue 'dispatch;
	// 82FF35F0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF35F4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF35F8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF35FC: 4082FFF4  bne 0x82ff35f0
	if !ctx.cr[0].eq {
	pc = 0x82FF35F0; continue 'dispatch;
	}
	// 82FF3600: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FF3604: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF3608: 48000008  b 0x82ff3610
	pc = 0x82FF3610; continue 'dispatch;
	// 82FF360C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FF3610: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 82FF3614: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3618: 3B9C8004  addi r28, r28, -0x7ffc
	ctx.r[28].s64 = ctx.r[28].s64 + -32764;
	// 82FF361C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3620: 7FCB5050  subf r30, r11, r10
	ctx.r[30].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FF3624: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FF3628: 4098002C  bge cr6, 0x82ff3654
	if !ctx.cr[6].lt {
	pc = 0x82FF3654; continue 'dispatch;
	}
	// 82FF362C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3630: 4BFFF739  bl 0x82ff2d68
	ctx.lr = 0x82FF3634;
	sub_82FF2D68(ctx, base);
	// 82FF3634: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3638: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF363C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FF3640: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF3644: 419A003C  beq cr6, 0x82ff3680
	if ctx.cr[6].eq {
	pc = 0x82FF3680; continue 'dispatch;
	}
	// 82FF3648: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82FF364C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FF3650: 4198FFDC  blt cr6, 0x82ff362c
	if ctx.cr[6].lt {
	pc = 0x82FF362C; continue 'dispatch;
	}
	// 82FF3654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3658: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FF365C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF3660: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF3664: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF3668: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FF366C: 4BFDE2D5  bl 0x82fd1940
	ctx.lr = 0x82FF3670;
	sub_82FD1940(ctx, base);
	// 82FF3670: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FF3674: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF3678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF367C: 481B4B38  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FF3680: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF3684: 4BFFFFF4  b 0x82ff3678
	pc = 0x82FF3678; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3688 size=176
    let mut pc: u32 = 0x82FF3688;
    'dispatch: loop {
        match pc {
            0x82FF3688 => {
    //   block [0x82FF3688..0x82FF3738)
	// 82FF3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF368C: 481B4ADD  bl 0x831a8168
	ctx.lr = 0x82FF3690;
	sub_831A8130(ctx, base);
	// 82FF3690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF3698: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF369C: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 82FF36A0: 3BBD8004  addi r29, r29, -0x7ffc
	ctx.r[29].s64 = ctx.r[29].s64 + -32764;
	// 82FF36A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF36A8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF36AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF36B0: 40980064  bge cr6, 0x82ff3714
	if !ctx.cr[6].lt {
	pc = 0x82FF3714; continue 'dispatch;
	}
	// 82FF36B4: 3FDF0003  addis r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 196608;
	// 82FF36B8: 3BDE8054  addi r30, r30, -0x7fac
	ctx.r[30].s64 = ctx.r[30].s64 + -32684;
	// 82FF36BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF36C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF36C4: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF36C8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF36CC: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF36D0: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82FF36D4: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF36D8: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF36DC: 41820054  beq 0x82ff3730
	if ctx.cr[0].eq {
	pc = 0x82FF3730; continue 'dispatch;
	}
	// 82FF36E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF36E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF36E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF36EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF36F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF36F4: 4BFF9865  bl 0x82fecf58
	ctx.lr = 0x82FF36F8;
	sub_82FECF58(ctx, base);
	// 82FF36F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF36FC: A0810050  lhz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF3700: 4BFDD419  bl 0x82fd0b18
	ctx.lr = 0x82FF3704;
	sub_82FD0B18(ctx, base);
	// 82FF3704: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3708: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF370C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3710: 4198FFAC  blt cr6, 0x82ff36bc
	if ctx.cr[6].lt {
	pc = 0x82FF36BC; continue 'dispatch;
	}
	// 82FF3714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3718: 4BFFF651  bl 0x82ff2d68
	ctx.lr = 0x82FF371C;
	sub_82FF2D68(ctx, base);
	// 82FF371C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF3720: 4082FF84  bne 0x82ff36a4
	if !ctx.cr[0].eq {
	pc = 0x82FF36A4; continue 'dispatch;
	}
	// 82FF3724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF3728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF372C: 481B4A8C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FF3730: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF3734: 4BFFFFF4  b 0x82ff3728
	pc = 0x82FF3728; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3738 size=192
    let mut pc: u32 = 0x82FF3738;
    'dispatch: loop {
        match pc {
            0x82FF3738 => {
    //   block [0x82FF3738..0x82FF37F8)
	// 82FF3738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF373C: 481B4A29  bl 0x831a8164
	ctx.lr = 0x82FF3740;
	sub_831A8130(ctx, base);
	// 82FF3740: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF3748: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF374C: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 82FF3750: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FF3754: 3BBD8004  addi r29, r29, -0x7ffc
	ctx.r[29].s64 = ctx.r[29].s64 + -32764;
	// 82FF3758: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF375C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3760: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3764: 40980070  bge cr6, 0x82ff37d4
	if !ctx.cr[6].lt {
	pc = 0x82FF37D4; continue 'dispatch;
	}
	// 82FF3768: 3FDF0003  addis r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 196608;
	// 82FF376C: 3BDE8054  addi r30, r30, -0x7fac
	ctx.r[30].s64 = ctx.r[30].s64 + -32684;
	// 82FF3770: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3774: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3778: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF377C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3780: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF3784: 7D2950AE  lbzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3788: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82FF378C: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FF3790: 40820060  bne 0x82ff37f0
	if !ctx.cr[0].eq {
	pc = 0x82FF37F0; continue 'dispatch;
	}
	// 82FF3794: 5769043E  clrlwi r9, r27, 0x10
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 82FF3798: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF379C: 419A0054  beq cr6, 0x82ff37f0
	if ctx.cr[6].eq {
	pc = 0x82FF37F0; continue 'dispatch;
	}
	// 82FF37A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF37A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF37A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF37AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF37B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF37B4: 4BFF97A5  bl 0x82fecf58
	ctx.lr = 0x82FF37B8;
	sub_82FECF58(ctx, base);
	// 82FF37B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF37BC: A0810050  lhz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF37C0: 4BFDD359  bl 0x82fd0b18
	ctx.lr = 0x82FF37C4;
	sub_82FD0B18(ctx, base);
	// 82FF37C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF37C8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF37CC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF37D0: 4198FFA0  blt cr6, 0x82ff3770
	if ctx.cr[6].lt {
	pc = 0x82FF3770; continue 'dispatch;
	}
	// 82FF37D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF37D8: 4BFFF591  bl 0x82ff2d68
	ctx.lr = 0x82FF37DC;
	sub_82FF2D68(ctx, base);
	// 82FF37DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF37E0: 4082FF78  bne 0x82ff3758
	if !ctx.cr[0].eq {
	pc = 0x82FF3758; continue 'dispatch;
	}
	// 82FF37E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF37E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF37EC: 481B49C8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FF37F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF37F4: 4BFFFFF4  b 0x82ff37e8
	pc = 0x82FF37E8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF37F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF37F8 size=264
    let mut pc: u32 = 0x82FF37F8;
    'dispatch: loop {
        match pc {
            0x82FF37F8 => {
    //   block [0x82FF37F8..0x82FF3900)
	// 82FF37F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF37FC: 481B4959  bl 0x831a8154
	ctx.lr = 0x82FF3800;
	sub_831A8130(ctx, base);
	// 82FF3800: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF3808: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82FF380C: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 82FF3810: 3F7F0002  addis r27, r31, 2
	ctx.r[27].s64 = ctx.r[31].s64 + 131072;
	// 82FF3814: 3BBDC00C  addi r29, r29, -0x3ff4
	ctx.r[29].s64 = ctx.r[29].s64 + -16372;
	// 82FF3818: 3B7BC008  addi r27, r27, -0x3ff8
	ctx.r[27].s64 = ctx.r[27].s64 + -16376;
	// 82FF381C: 3FDF0001  addis r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 65536;
	// 82FF3820: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF3824: 3BDE8004  addi r30, r30, -0x7ffc
	ctx.r[30].s64 = ctx.r[30].s64 + -32764;
	// 82FF3828: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF382C: 831B0000  lwz r24, 0(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3834: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3838: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF383C: 4098005C  bge cr6, 0x82ff3898
	if !ctx.cr[6].lt {
	pc = 0x82FF3898; continue 'dispatch;
	}
	// 82FF3840: 3F5F0003  addis r26, r31, 3
	ctx.r[26].s64 = ctx.r[31].s64 + 196608;
	// 82FF3844: 3B5A8054  addi r26, r26, -0x7fac
	ctx.r[26].s64 = ctx.r[26].s64 + -32684;
	// 82FF3848: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF384C: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3850: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF3854: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3858: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF385C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82FF3860: 7D2848AE  lbzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3864: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FF3868: 41820070  beq 0x82ff38d8
	if ctx.cr[0].eq {
	pc = 0x82FF38D8; continue 'dispatch;
	}
	// 82FF386C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3870: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82FF3874: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF3878: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF387C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3880: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3884: 4BFF96D5  bl 0x82fecf58
	ctx.lr = 0x82FF3888;
	sub_82FECF58(ctx, base);
	// 82FF3888: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF388C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3890: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3894: 4198FFB4  blt cr6, 0x82ff3848
	if ctx.cr[6].lt {
	pc = 0x82FF3848; continue 'dispatch;
	}
	// 82FF3898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF389C: 4BFFF4CD  bl 0x82ff2d68
	ctx.lr = 0x82FF38A0;
	sub_82FF2D68(ctx, base);
	// 82FF38A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF38A4: 4082FF8C  bne 0x82ff3830
	if !ctx.cr[0].eq {
	pc = 0x82FF3830; continue 'dispatch;
	}
	// 82FF38A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF38AC: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF38B0: 409A0014  bne cr6, 0x82ff38c4
	if !ctx.cr[6].eq {
	pc = 0x82FF38C4; continue 'dispatch;
	}
	// 82FF38B4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF38B8: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF38BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF38C0: 419A0008  beq cr6, 0x82ff38c8
	if ctx.cr[6].eq {
	pc = 0x82FF38C8; continue 'dispatch;
	}
	// 82FF38C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF38C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF38CC: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FF38D0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FF38D4: 481B48D0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 82FF38D8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF38DC: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF38E0: 409A0014  bne cr6, 0x82ff38f4
	if !ctx.cr[6].eq {
	pc = 0x82FF38F4; continue 'dispatch;
	}
	// 82FF38E4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF38E8: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF38EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF38F0: 419A0008  beq cr6, 0x82ff38f8
	if ctx.cr[6].eq {
	pc = 0x82FF38F8; continue 'dispatch;
	}
	// 82FF38F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF38F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF38FC: 4BFFFFD0  b 0x82ff38cc
	pc = 0x82FF38CC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF3900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF3900 size=156
    let mut pc: u32 = 0x82FF3900;
    'dispatch: loop {
        match pc {
            0x82FF3900 => {
    //   block [0x82FF3900..0x82FF399C)
	// 82FF3900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF3904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF3908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF390C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF3910: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF3914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF3918: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FF391C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3920: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3924: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF3928: 409A0010  bne cr6, 0x82ff3938
	if !ctx.cr[6].eq {
	pc = 0x82FF3938; continue 'dispatch;
	}
	// 82FF392C: 4BFFF43D  bl 0x82ff2d68
	ctx.lr = 0x82FF3930;
	sub_82FF2D68(ctx, base);
	// 82FF3930: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF3934: 41820050  beq 0x82ff3984
	if ctx.cr[0].eq {
	pc = 0x82FF3984; continue 'dispatch;
	}
	// 82FF3938: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF393C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3940: 614A8054  ori r10, r10, 0x8054
	ctx.r[10].u64 = ctx.r[10].u64 | 32852;
	// 82FF3944: 7D3F502E  lwzx r9, r31, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3948: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FF394C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3950: 7D4AFA2E  lhzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FF3954: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82FF3958: 7D4950AE  lbzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF395C: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF3960: 41820024  beq 0x82ff3984
	if ctx.cr[0].eq {
	pc = 0x82FF3984; continue 'dispatch;
	}
	// 82FF3964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3968: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF396C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF3970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF3974: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3978: 4BFF95E1  bl 0x82fecf58
	ctx.lr = 0x82FF397C;
	sub_82FECF58(ctx, base);
	// 82FF397C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF3980: 48000008  b 0x82ff3988
	pc = 0x82FF3988; continue 'dispatch;
	// 82FF3984: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF3988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF398C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF3990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF3994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF3998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF39A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF39A0 size=8
    let mut pc: u32 = 0x82FF39A0;
    'dispatch: loop {
        match pc {
            0x82FF39A0 => {
    //   block [0x82FF39A0..0x82FF39A8)
	// 82FF39A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF39A4: 8213E908  lwz r16, -0x16f8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF39A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF39A8 size=2032
    let mut pc: u32 = 0x82FF39A8;
    'dispatch: loop {
        match pc {
            0x82FF39A8 => {
    //   block [0x82FF39A8..0x82FF4198)
	// 82FF39A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF39AC: 481B47A5  bl 0x831a8150
	ctx.lr = 0x82FF39B0;
	sub_831A8130(ctx, base);
	// 82FF39B0: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 82FF39B4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF39B8: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF39BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF39C0: 616BC010  ori r11, r11, 0xc010
	ctx.r[11].u64 = ctx.r[11].u64 | 49168;
	// 82FF39C4: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FF39C8: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82FF39CC: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF39D0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF39D4: 61788004  ori r24, r11, 0x8004
	ctx.r[24].u64 = ctx.r[11].u64 | 32772;
	// 82FF39D8: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF39DC: 41820634  beq 0x82ff4010
	if ctx.cr[0].eq {
	pc = 0x82FF4010; continue 'dispatch;
	}
	// 82FF39E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF39E4: 40990598  ble cr6, 0x82ff3f7c
	if !ctx.cr[6].gt {
	pc = 0x82FF3F7C; continue 'dispatch;
	}
	// 82FF39E8: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82FF39EC: 40990360  ble cr6, 0x82ff3d4c
	if !ctx.cr[6].gt {
	pc = 0x82FF3D4C; continue 'dispatch;
	}
	// 82FF39F0: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82FF39F4: 419A01A0  beq cr6, 0x82ff3b94
	if ctx.cr[6].eq {
	pc = 0x82FF3B94; continue 'dispatch;
	}
	// 82FF39F8: 396AFFFB  addi r11, r10, -5
	ctx.r[11].s64 = ctx.r[10].s64 + -5;
	// 82FF39FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FF3A00: 4199057C  bgt cr6, 0x82ff3f7c
	if ctx.cr[6].gt {
	pc = 0x82FF3F7C; continue 'dispatch;
	}
	// 82FF3A04: 3FBE0003  addis r29, r30, 3
	ctx.r[29].s64 = ctx.r[30].s64 + 196608;
	// 82FF3A08: 3BBD8024  addi r29, r29, -0x7fdc
	ctx.r[29].s64 = ctx.r[29].s64 + -32732;
	// 82FF3A0C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3A10: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 82FF3A14: 419806C8  blt cr6, 0x82ff40dc
	if ctx.cr[6].lt {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3A18: 3CBE0002  addis r5, r30, 2
	ctx.r[5].s64 = ctx.r[30].s64 + 131072;
	// 82FF3A1C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FF3A20: 38A5C020  addi r5, r5, -0x3fe0
	ctx.r[5].s64 = ctx.r[5].s64 + -16352;
	// 82FF3A24: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3A28: 7D66F214  add r11, r6, r30
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[30].u64;
	// 82FF3A2C: 3D0B0002  addis r8, r11, 2
	ctx.r[8].s64 = ctx.r[11].s64 + 131072;
	// 82FF3A30: 3908C024  addi r8, r8, -0x3fdc
	ctx.r[8].s64 = ctx.r[8].s64 + -16348;
	// 82FF3A34: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3A38: 2B0BFEFF  cmplwi cr6, r11, 0xfeff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65279 as u32, &mut ctx.xer);
	// 82FF3A3C: 419A000C  beq cr6, 0x82ff3a48
	if ctx.cr[6].eq {
	pc = 0x82FF3A48; continue 'dispatch;
	}
	// 82FF3A40: 2B0BFFFE  cmplwi cr6, r11, 0xfffe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65534 as u32, &mut ctx.xer);
	// 82FF3A44: 409A0018  bne cr6, 0x82ff3a5c
	if !ctx.cr[6].eq {
	pc = 0x82FF3A5C; continue 'dispatch;
	}
	// 82FF3A48: 39660002  addi r11, r6, 2
	ctx.r[11].s64 = ctx.r[6].s64 + 2;
	// 82FF3A4C: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82FF3A50: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82FF3A54: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FF3A58: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3A5C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FF3A60: 7D261850  subf r9, r6, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[6].s64;
	// 82FF3A64: 816B8DD0  lwz r11, -0x7230(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29232 as u32) ) } as u64;
	// 82FF3A68: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF3A6C: 4098000C  bge cr6, 0x82ff3a78
	if !ctx.cr[6].lt {
	pc = 0x82FF3A78; continue 'dispatch;
	}
	// 82FF3A70: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FF3A74: 48000668  b 0x82ff40dc
	pc = 0x82FF40DC; continue 'dispatch;
	// 82FF3A78: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 82FF3A7C: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82FF3A80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3A84: 409A00D8  bne cr6, 0x82ff3b5c
	if !ctx.cr[6].eq {
	pc = 0x82FF3B5C; continue 'dispatch;
	}
	// 82FF3A88: 3D408215  lis r10, -0x7deb
	ctx.r[10].s64 = -2112552960;
	// 82FF3A8C: 392A8DB8  addi r9, r10, -0x7248
	ctx.r[9].s64 = ctx.r[10].s64 + -29256;
	// 82FF3A90: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82FF3A94: 41820028  beq 0x82ff3abc
	if ctx.cr[0].eq {
	pc = 0x82FF3ABC; continue 'dispatch;
	}
	// 82FF3A98: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF3A9C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3AA0: 8B890000  lbz r28, 0(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3AA4: 7CFC3851  subf. r7, r28, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FF3AA8: 40820014  bne 0x82ff3abc
	if !ctx.cr[0].eq {
	pc = 0x82FF3ABC; continue 'dispatch;
	}
	// 82FF3AAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3AB0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FF3AB4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF3AB8: 409AFFE4  bne cr6, 0x82ff3a9c
	if !ctx.cr[6].eq {
	pc = 0x82FF3A9C; continue 'dispatch;
	}
	// 82FF3ABC: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF3AC0: 4082FFB0  bne 0x82ff3a70
	if !ctx.cr[0].eq {
	pc = 0x82FF3A70; continue 'dispatch;
	}
	// 82FF3AC4: 7F061840  cmplw cr6, r6, r3
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FF3AC8: 40980614  bge cr6, 0x82ff40dc
	if !ctx.cr[6].lt {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3ACC: 3CFE0003  addis r7, r30, 3
	ctx.r[7].s64 = ctx.r[30].s64 + 196608;
	// 82FF3AD0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82FF3AD4: 38E78048  addi r7, r7, -0x7fb8
	ctx.r[7].s64 = ctx.r[7].s64 + -32696;
	// 82FF3AD8: 7D7EC214  add r11, r30, r24
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF3ADC: 615C8008  ori r28, r10, 0x8008
	ctx.r[28].u64 = ctx.r[10].u64 | 32776;
	// 82FF3AE0: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3AE4: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3AE8: 38CA0002  addi r6, r10, 2
	ctx.r[6].s64 = ctx.r[10].s64 + 2;
	// 82FF3AEC: A1480000  lhz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3AF0: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82FF3AF4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3AF8: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82FF3AFC: 41820014  beq 0x82ff3b10
	if ctx.cr[0].eq {
	pc = 0x82FF3B10; continue 'dispatch;
	}
	// 82FF3B00: 5549422E  rlwinm r9, r10, 8, 8, 0x17
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82FF3B04: 554AC63E  rlwinm r10, r10, 0x18, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82FF3B08: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82FF3B0C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FF3B10: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B14: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82FF3B18: 5544043E  clrlwi r4, r10, 0x10
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FF3B1C: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82FF3B20: 2B04003E  cmplwi cr6, r4, 0x3e
	ctx.cr[6].compare_u32(ctx.r[4].u32, 62 as u32, &mut ctx.xer);
	// 82FF3B24: 7CC9E1AE  stbx r6, r9, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32), ctx.r[6].u8) };
	// 82FF3B28: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B2C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FF3B30: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FF3B34: 7D49F32E  sthx r10, r9, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u16) };
	// 82FF3B38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3B40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3B44: 419A0598  beq cr6, 0x82ff40dc
	if ctx.cr[6].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3B48: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B4C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B50: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF3B54: 4198FF8C  blt cr6, 0x82ff3ae0
	if ctx.cr[6].lt {
	pc = 0x82FF3AE0; continue 'dispatch;
	}
	// 82FF3B58: 48000584  b 0x82ff40dc
	pc = 0x82FF40DC; continue 'dispatch;
	// 82FF3B5C: 3D408215  lis r10, -0x7deb
	ctx.r[10].s64 = -2112552960;
	// 82FF3B60: 392A8DC4  addi r9, r10, -0x723c
	ctx.r[9].s64 = ctx.r[10].s64 + -29244;
	// 82FF3B64: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82FF3B68: 4182FF54  beq 0x82ff3abc
	if ctx.cr[0].eq {
	pc = 0x82FF3ABC; continue 'dispatch;
	}
	// 82FF3B6C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF3B70: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B74: 8B890000  lbz r28, 0(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3B78: 7CFC3851  subf. r7, r28, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[28].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FF3B7C: 4082FF40  bne 0x82ff3abc
	if !ctx.cr[0].eq {
	pc = 0x82FF3ABC; continue 'dispatch;
	}
	// 82FF3B80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3B84: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FF3B88: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF3B8C: 409AFFE4  bne cr6, 0x82ff3b70
	if !ctx.cr[6].eq {
	pc = 0x82FF3B70; continue 'dispatch;
	}
	// 82FF3B90: 4BFFFF2C  b 0x82ff3abc
	pc = 0x82FF3ABC; continue 'dispatch;
	// 82FF3B94: 3F5E0003  addis r26, r30, 3
	ctx.r[26].s64 = ctx.r[30].s64 + 196608;
	// 82FF3B98: 3F808215  lis r28, -0x7deb
	ctx.r[28].s64 = -2112552960;
	// 82FF3B9C: 3B5A8024  addi r26, r26, -0x7fdc
	ctx.r[26].s64 = ctx.r[26].s64 + -32732;
	// 82FF3BA0: 3FBE0002  addis r29, r30, 2
	ctx.r[29].s64 = ctx.r[30].s64 + 131072;
	// 82FF3BA4: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF3BA8: 3BBDC024  addi r29, r29, -0x3fdc
	ctx.r[29].s64 = ctx.r[29].s64 + -16348;
	// 82FF3BAC: 80BC8E0C  lwz r5, -0x71f4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-29172 as u32) ) } as u64;
	// 82FF3BB0: 615BC020  ori r27, r10, 0xc020
	ctx.r[27].u64 = ctx.r[10].u64 | 49184;
	// 82FF3BB4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3BB8: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FF3BBC: 40990038  ble cr6, 0x82ff3bf4
	if !ctx.cr[6].gt {
	pc = 0x82FF3BF4; continue 'dispatch;
	}
	// 82FF3BC0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FF3BC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF3BC8: 388B8E08  addi r4, r11, -0x71f8
	ctx.r[4].s64 = ctx.r[11].s64 + -29176;
	// 82FF3BCC: 4BFDD64D  bl 0x82fd1218
	ctx.lr = 0x82FF3BD0;
	sub_82FD1218(ctx, base);
	// 82FF3BD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF3BD4: 40820020  bne 0x82ff3bf4
	if !ctx.cr[0].eq {
	pc = 0x82FF3BF4; continue 'dispatch;
	}
	// 82FF3BD8: 7D7EDA14  add r11, r30, r27
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 82FF3BDC: 815C8E0C  lwz r10, -0x71f4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-29172 as u32) ) } as u64;
	// 82FF3BE0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3BE4: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82FF3BE8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3BEC: 817C8E0C  lwz r11, -0x71f4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-29172 as u32) ) } as u64;
	// 82FF3BF0: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FF3BF4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FF3BF8: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3BFC: 80AB8DA8  lwz r5, -0x7258(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29272 as u32) ) } as u64;
	// 82FF3C00: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FF3C04: 419804D8  blt cr6, 0x82ff40dc
	if ctx.cr[6].lt {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3C08: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FF3C0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF3C10: 388B8DA0  addi r4, r11, -0x7260
	ctx.r[4].s64 = ctx.r[11].s64 + -29280;
	// 82FF3C14: 4BFDD605  bl 0x82fd1218
	ctx.lr = 0x82FF3C18;
	sub_82FD1218(ctx, base);
	// 82FF3C18: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF3C1C: 408204C0  bne 0x82ff40dc
	if !ctx.cr[0].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3C20: 7D5EDA14  add r10, r30, r27
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 82FF3C24: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C28: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C2C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF3C30: 409804AC  bge cr6, 0x82ff40dc
	if !ctx.cr[6].lt {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3C34: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82FF3C38: 7D7EC214  add r11, r30, r24
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF3C3C: 613C8008  ori r28, r9, 0x8008
	ctx.r[28].u64 = ctx.r[9].u64 | 32776;
	// 82FF3C40: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C44: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C48: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82FF3C4C: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C50: 7D08F214  add r8, r8, r30
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 82FF3C54: 7D260774  extsb r6, r9
	ctx.r[6].s64 = ctx.r[9].s8 as i64;
	// 82FF3C58: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FF3C5C: 2F06003E  cmpwi cr6, r6, 0x3e
	ctx.cr[6].compare_i32(ctx.r[6].s32, 62, &mut ctx.xer);
	// 82FF3C60: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FF3C64: 7D270774  extsb r7, r9
	ctx.r[7].s64 = ctx.r[9].s8 as i64;
	// 82FF3C68: 7EC8E1AE  stbx r22, r8, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[28].u32), ctx.r[22].u8) };
	// 82FF3C6C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C70: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82FF3C74: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FF3C78: 7CE8F32E  sthx r7, r8, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u16) };
	// 82FF3C7C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C80: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82FF3C84: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FF3C88: 419A0454  beq cr6, 0x82ff40dc
	if ctx.cr[6].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3C8C: 55290631  rlwinm. r9, r9, 0, 0x18, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FF3C90: 40820018  bne 0x82ff3ca8
	if !ctx.cr[0].eq {
	pc = 0x82FF3CA8; continue 'dispatch;
	}
	// 82FF3C94: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C98: 811A0000  lwz r8, 0(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3C9C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FF3CA0: 4198FFA0  blt cr6, 0x82ff3c40
	if ctx.cr[6].lt {
	pc = 0x82FF3C40; continue 'dispatch;
	}
	// 82FF3CA4: 48000438  b 0x82ff40dc
	pc = 0x82FF40DC; continue 'dispatch;
	// 82FF3CA8: 3FBE0003  addis r29, r30, 3
	ctx.r[29].s64 = ctx.r[30].s64 + 196608;
	// 82FF3CAC: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FF3CB0: 3BBD8060  addi r29, r29, -0x7fa0
	ctx.r[29].s64 = ctx.r[29].s64 + -32672;
	// 82FF3CB4: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 82FF3CB8: 6129C01C  ori r9, r9, 0xc01c
	ctx.r[9].u64 = ctx.r[9].u64 | 49180;
	// 82FF3CBC: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82FF3CC0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3CC4: 92EA0000  stw r23, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82FF3CC8: 7C9E482E  lwzx r4, r30, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3CCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3CD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3CD8: 4E800421  bctrl
	ctx.lr = 0x82FF3CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3CDC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF3CE0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3CE4: 616BC014  ori r11, r11, 0xc014
	ctx.r[11].u64 = ctx.r[11].u64 | 49172;
	// 82FF3CE8: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3CF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3CF8: 4E800421  bctrl
	ctx.lr = 0x82FF3CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3CFC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF3D00: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3D04: 614A8040  ori r10, r10, 0x8040
	ctx.r[10].u64 = ctx.r[10].u64 | 32832;
	// 82FF3D08: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FF3D0C: 7CFE502E  lwzx r7, r30, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3D10: 90FF0060  stw r7, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82FF3D14: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF3D18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF3D1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FF3D20: 388AE8C0  addi r4, r10, -0x1740
	ctx.r[4].s64 = ctx.r[10].s64 + -5952;
	// 82FF3D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF3D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF3D2C: 38C00047  li r6, 0x47
	ctx.r[6].s64 = 71;
	// 82FF3D30: 38A004FB  li r5, 0x4fb
	ctx.r[5].s64 = 1275;
	// 82FF3D34: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FF3D38: 4BFF8F71  bl 0x82fecca8
	ctx.lr = 0x82FF3D3C;
	sub_82FECCA8(ctx, base);
	// 82FF3D3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF3D40: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FF3D44: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF3D48: 481BCEE1  bl 0x831b0c28
	ctx.lr = 0x82FF3D4C;
	sub_831B0C28(ctx, base);
	// 82FF3D4C: 3D1E0002  addis r8, r30, 2
	ctx.r[8].s64 = ctx.r[30].s64 + 131072;
	// 82FF3D50: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF3D54: 3908C024  addi r8, r8, -0x3fdc
	ctx.r[8].s64 = ctx.r[8].s64 + -16348;
	// 82FF3D58: 61468024  ori r6, r10, 0x8024
	ctx.r[6].u64 = ctx.r[10].u64 | 32804;
	// 82FF3D5C: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF3D60: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 82FF3D64: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 82FF3D68: 89680000  lbz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3D6C: 614AC025  ori r10, r10, 0xc025
	ctx.r[10].u64 = ctx.r[10].u64 | 49189;
	// 82FF3D70: 6129C026  ori r9, r9, 0xc026
	ctx.r[9].u64 = ctx.r[9].u64 | 49190;
	// 82FF3D74: 60E7C027  ori r7, r7, 0xc027
	ctx.r[7].u64 = ctx.r[7].u64 | 49191;
	// 82FF3D78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3D7C: 40820028  bne 0x82ff3da4
	if !ctx.cr[0].eq {
	pc = 0x82FF3DA4; continue 'dispatch;
	}
	// 82FF3D80: 7CBE50AE  lbzx r5, r30, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3D84: 28050000  cmplwi r5, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3D88: 4082001C  bne 0x82ff3da4
	if !ctx.cr[0].eq {
	pc = 0x82FF3DA4; continue 'dispatch;
	}
	// 82FF3D8C: 7CBE48AE  lbzx r5, r30, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3D90: 2B0500FE  cmplwi cr6, r5, 0xfe
	ctx.cr[6].compare_u32(ctx.r[5].u32, 254 as u32, &mut ctx.xer);
	// 82FF3D94: 409A0010  bne cr6, 0x82ff3da4
	if !ctx.cr[6].eq {
	pc = 0x82FF3DA4; continue 'dispatch;
	}
	// 82FF3D98: 7CBE38AE  lbzx r5, r30, r7
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FF3D9C: 2B0500FF  cmplwi cr6, r5, 0xff
	ctx.cr[6].compare_u32(ctx.r[5].u32, 255 as u32, &mut ctx.xer);
	// 82FF3DA0: 419A0030  beq cr6, 0x82ff3dd0
	if ctx.cr[6].eq {
	pc = 0x82FF3DD0; continue 'dispatch;
	}
	// 82FF3DA4: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 82FF3DA8: 409A0070  bne cr6, 0x82ff3e18
	if !ctx.cr[6].eq {
	pc = 0x82FF3E18; continue 'dispatch;
	}
	// 82FF3DAC: 7D7E50AE  lbzx r11, r30, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3DB0: 2B0B00FE  cmplwi cr6, r11, 0xfe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 254 as u32, &mut ctx.xer);
	// 82FF3DB4: 409A0064  bne cr6, 0x82ff3e18
	if !ctx.cr[6].eq {
	pc = 0x82FF3E18; continue 'dispatch;
	}
	// 82FF3DB8: 7D7E48AE  lbzx r11, r30, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FF3DBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3DC0: 40820058  bne 0x82ff3e18
	if !ctx.cr[0].eq {
	pc = 0x82FF3E18; continue 'dispatch;
	}
	// 82FF3DC4: 7D7E38AE  lbzx r11, r30, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FF3DC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3DCC: 4082004C  bne 0x82ff3e18
	if !ctx.cr[0].eq {
	pc = 0x82FF3E18; continue 'dispatch;
	}
	// 82FF3DD0: 7D5E3214  add r10, r30, r6
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82FF3DD4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3DD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF3DDC: 40990030  ble cr6, 0x82ff3e0c
	if !ctx.cr[6].gt {
	pc = 0x82FF3E0C; continue 'dispatch;
	}
	// 82FF3DE0: 3D60FFFE  lis r11, -2
	ctx.r[11].s64 = -131072;
	// 82FF3DE4: 61693FDC  ori r9, r11, 0x3fdc
	ctx.r[9].u64 = ctx.r[11].u64 | 16348;
	// 82FF3DE8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82FF3DEC: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82FF3DF0: 88EB0004  lbz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF3DF4: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82FF3DF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF3DFC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E00: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FF3E04: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82FF3E08: 4198FFE8  blt cr6, 0x82ff3df0
	if ctx.cr[6].lt {
	pc = 0x82FF3DF0; continue 'dispatch;
	}
	// 82FF3E0C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E10: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82FF3E14: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3E18: 3D3E0002  addis r9, r30, 2
	ctx.r[9].s64 = ctx.r[30].s64 + 131072;
	// 82FF3E1C: 7CFE3214  add r7, r30, r6
	ctx.r[7].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82FF3E20: 3929C020  addi r9, r9, -0x3fe0
	ctx.r[9].s64 = ctx.r[9].s64 + -16352;
	// 82FF3E24: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E28: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E2C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3E30: 409802AC  bge cr6, 0x82ff40dc
	if !ctx.cr[6].lt {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3E34: 3CDE0003  addis r6, r30, 3
	ctx.r[6].s64 = ctx.r[30].s64 + 196608;
	// 82FF3E38: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF3E3C: 38C68048  addi r6, r6, -0x7fb8
	ctx.r[6].s64 = ctx.r[6].s64 + -32696;
	// 82FF3E40: 617C8008  ori r28, r11, 0x8008
	ctx.r[28].u64 = ctx.r[11].u64 | 32776;
	// 82FF3E44: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E48: 88A60000  lbz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E4C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FF3E50: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E54: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82FF3E58: 28050000  cmplwi r5, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF3E5C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF3E60: 41820020  beq 0x82ff3e80
	if ctx.cr[0].eq {
	pc = 0x82FF3E80; continue 'dispatch;
	}
	// 82FF3E64: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82FF3E68: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82FF3E6C: 514B843E  rlwimi r11, r10, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82FF3E70: 5145801E  rlwimi r5, r10, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82FF3E74: 556BC43E  rlwinm r11, r11, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF3E78: 54AA401E  rlwinm r10, r5, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82FF3E7C: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82FF3E80: 2B0AFFFF  cmplwi cr6, r10, 0xffff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65535 as u32, &mut ctx.xer);
	// 82FF3E84: 41990054  bgt cr6, 0x82ff3ed8
	if ctx.cr[6].gt {
	pc = 0x82FF3ED8; continue 'dispatch;
	}
	// 82FF3E88: 7D7EC214  add r11, r30, r24
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF3E8C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82FF3E90: 2B0A003E  cmplwi cr6, r10, 0x3e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 62 as u32, &mut ctx.xer);
	// 82FF3E94: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FF3E98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3E9C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82FF3EA0: 7CAAE1AE  stbx r5, r10, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[5].u8) };
	// 82FF3EA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3EA8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF3EAC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF3EB0: 7C8AF32E  sthx r4, r10, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[4].u16) };
	// 82FF3EB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3EB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF3EBC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF3EC0: 419A021C  beq cr6, 0x82ff40dc
	if ctx.cr[6].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF3EC4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3EC8: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3ECC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF3ED0: 4198FF74  blt cr6, 0x82ff3e44
	if ctx.cr[6].lt {
	pc = 0x82FF3E44; continue 'dispatch;
	}
	// 82FF3ED4: 48000208  b 0x82ff40dc
	pc = 0x82FF40DC; continue 'dispatch;
	// 82FF3ED8: 3FBE0003  addis r29, r30, 3
	ctx.r[29].s64 = ctx.r[30].s64 + 196608;
	// 82FF3EDC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF3EE0: 3BBD8060  addi r29, r29, -0x7fa0
	ctx.r[29].s64 = ctx.r[29].s64 + -32672;
	// 82FF3EE4: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF3EE8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FF3EEC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3EF0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3EF4: 7EFEC12E  stwx r23, r30, r24
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[24].u32), ctx.r[23].u32) };
	// 82FF3EF8: 92E90000  stw r23, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82FF3EFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3F04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3F08: 4E800421  bctrl
	ctx.lr = 0x82FF3F0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3F0C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF3F10: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F14: 616BC014  ori r11, r11, 0xc014
	ctx.r[11].u64 = ctx.r[11].u64 | 49172;
	// 82FF3F18: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3F1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3F28: 4E800421  bctrl
	ctx.lr = 0x82FF3F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3F2C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF3F30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F34: 614A8040  ori r10, r10, 0x8040
	ctx.r[10].u64 = ctx.r[10].u64 | 32832;
	// 82FF3F38: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FF3F3C: 7CFE502E  lwzx r7, r30, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FF3F40: 90FF0060  stw r7, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82FF3F44: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF3F48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF3F4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FF3F50: 388AE8C0  addi r4, r10, -0x1740
	ctx.r[4].s64 = ctx.r[10].s64 + -5952;
	// 82FF3F54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF3F58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF3F5C: 38C00047  li r6, 0x47
	ctx.r[6].s64 = 71;
	// 82FF3F60: 38A004AC  li r5, 0x4ac
	ctx.r[5].s64 = 1196;
	// 82FF3F64: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82FF3F68: 4BFF8D41  bl 0x82fecca8
	ctx.lr = 0x82FF3F6C;
	sub_82FECCA8(ctx, base);
	// 82FF3F6C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF3F70: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82FF3F74: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF3F78: 481BCCB1  bl 0x831b0c28
	ctx.lr = 0x82FF3F7C;
	sub_831B0C28(ctx, base);
	// 82FF3F7C: 3FBE0003  addis r29, r30, 3
	ctx.r[29].s64 = ctx.r[30].s64 + 196608;
	// 82FF3F80: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF3F84: 3BBD8060  addi r29, r29, -0x7fa0
	ctx.r[29].s64 = ctx.r[29].s64 + -32672;
	// 82FF3F88: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF3F8C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F90: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3F94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3F98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3F9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3FA0: 4E800421  bctrl
	ctx.lr = 0x82FF3FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3FA4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF3FA8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3FAC: 616BC014  ori r11, r11, 0xc014
	ctx.r[11].u64 = ctx.r[11].u64 | 49172;
	// 82FF3FB0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3FB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3FB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3FC0: 4E800421  bctrl
	ctx.lr = 0x82FF3FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3FC4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF3FC8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3FCC: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FF3FD0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF3FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3FD8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF3FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF3FE0: 4E800421  bctrl
	ctx.lr = 0x82FF3FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF3FE4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF3FE8: 38C00046  li r6, 0x46
	ctx.r[6].s64 = 70;
	// 82FF3FEC: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF3FF0: 388BE8C0  addi r4, r11, -0x1740
	ctx.r[4].s64 = ctx.r[11].s64 + -5952;
	// 82FF3FF4: 38A0056F  li r5, 0x56f
	ctx.r[5].s64 = 1391;
	// 82FF3FF8: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82FF3FFC: 4BFFE055  bl 0x82ff2050
	ctx.lr = 0x82FF4000;
	sub_82FF2050(ctx, base);
	// 82FF4000: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4004: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82FF4008: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 82FF400C: 481BCC1D  bl 0x831b0c28
	ctx.lr = 0x82FF4010;
	sub_831B0C28(ctx, base);
	// 82FF4010: 3F5E0002  addis r26, r30, 2
	ctx.r[26].s64 = ctx.r[30].s64 + 131072;
	// 82FF4014: 3B5AC024  addi r26, r26, -0x3fdc
	ctx.r[26].s64 = ctx.r[26].s64 + -16348;
	// 82FF4018: 887A0000  lbz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF401C: 48041E7D  bl 0x83035e98
	ctx.lr = 0x82FF4020;
	sub_83035E98(ctx, base);
	// 82FF4020: 3F7E0002  addis r27, r30, 2
	ctx.r[27].s64 = ctx.r[30].s64 + 131072;
	// 82FF4024: 7FBEC214  add r29, r30, r24
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF4028: 3B7BC020  addi r27, r27, -0x3fe0
	ctx.r[27].s64 = ctx.r[27].s64 + -16352;
	// 82FF402C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF4030: 5469043E  clrlwi r9, r3, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FF4034: 617C8008  ori r28, r11, 0x8008
	ctx.r[28].u64 = ctx.r[11].u64 | 32776;
	// 82FF4038: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF403C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FF4040: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4044: 2B09003E  cmplwi cr6, r9, 0x3e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 62 as u32, &mut ctx.xer);
	// 82FF4048: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF404C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF4050: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF4054: 7ECBE1AE  stbx r22, r11, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[22].u8) };
	// 82FF4058: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF405C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF4060: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF4064: 7C6BF32E  sthx r3, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u16) };
	// 82FF4068: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF406C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF4070: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4074: 419A0068  beq cr6, 0x82ff40dc
	if ctx.cr[6].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF4078: 3F3E0003  addis r25, r30, 3
	ctx.r[25].s64 = ctx.r[30].s64 + 196608;
	// 82FF407C: 3B398024  addi r25, r25, -0x7fdc
	ctx.r[25].s64 = ctx.r[25].s64 + -32732;
	// 82FF4080: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4084: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4088: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF408C: 419A0050  beq cr6, 0x82ff40dc
	if ctx.cr[6].eq {
	pc = 0x82FF40DC; continue 'dispatch;
	}
	// 82FF4090: 887A0000  lbz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4094: 48041E05  bl 0x83035e98
	ctx.lr = 0x82FF4098;
	sub_83035E98(ctx, base);
	// 82FF4098: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF409C: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FF40A0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FF40A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF40A8: 2B0A003E  cmplwi cr6, r10, 0x3e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 62 as u32, &mut ctx.xer);
	// 82FF40AC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF40B0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF40B4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF40B8: 7ECBE1AE  stbx r22, r11, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[22].u8) };
	// 82FF40BC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF40C0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF40C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF40C8: 7C6BF32E  sthx r3, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u16) };
	// 82FF40CC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF40D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF40D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF40D8: 409AFFA8  bne cr6, 0x82ff4080
	if !ctx.cr[6].eq {
	pc = 0x82FF4080; continue 'dispatch;
	}
	// 82FF40DC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF40E0: 616B8050  ori r11, r11, 0x8050
	ctx.r[11].u64 = ctx.r[11].u64 | 32848;
	// 82FF40E4: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF40E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF40EC: 409A003C  bne cr6, 0x82ff4128
	if !ctx.cr[6].eq {
	pc = 0x82FF4128; continue 'dispatch;
	}
	// 82FF40F0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF40F4: 616B802C  ori r11, r11, 0x802c
	ctx.r[11].u64 = ctx.r[11].u64 | 32812;
	// 82FF40F8: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF40FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FF4100: 409A0028  bne cr6, 0x82ff4128
	if !ctx.cr[6].eq {
	pc = 0x82FF4128; continue 'dispatch;
	}
	// 82FF4104: 7D7EC214  add r11, r30, r24
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF4108: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82FF410C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4110: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF4114: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF4118: 7D2AF32E  sthx r9, r10, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u16) };
	// 82FF411C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4120: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FF4124: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF4128: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF412C: 616B803D  ori r11, r11, 0x803d
	ctx.r[11].u64 = ctx.r[11].u64 | 32829;
	// 82FF4130: 7D7E58AE  lbzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF4134: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4138: 41820058  beq 0x82ff4190
	if ctx.cr[0].eq {
	pc = 0x82FF4190; continue 'dispatch;
	}
	// 82FF413C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF4140: 7D3EC214  add r9, r30, r24
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 82FF4144: 616AC008  ori r10, r11, 0xc008
	ctx.r[10].u64 = ctx.r[11].u64 | 49160;
	// 82FF4148: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 82FF414C: 7EFE512E  stwx r23, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u32) };
	// 82FF4150: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4154: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82FF4158: 40990038  ble cr6, 0x82ff4190
	if !ctx.cr[6].gt {
	pc = 0x82FF4190; continue 'dispatch;
	}
	// 82FF415C: 3D5E0001  addis r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 65536;
	// 82FF4160: 3CDE0001  addis r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 65536;
	// 82FF4164: 394AC00C  addi r10, r10, -0x3ff4
	ctx.r[10].s64 = ctx.r[10].s64 + -16372;
	// 82FF4168: 38C68007  addi r6, r6, -0x7ff9
	ctx.r[6].s64 = ctx.r[6].s64 + -32761;
	// 82FF416C: 7D0658AE  lbzx r8, r6, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FF4170: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF4174: 80EAFFFC  lwz r7, -4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FF4178: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82FF417C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FF4180: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FF4184: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4188: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FF418C: 4198FFE0  blt cr6, 0x82ff416c
	if ctx.cr[6].lt {
	pc = 0x82FF416C; continue 'dispatch;
	}
	// 82FF4190: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 82FF4194: 481B400C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4198 size=40
    let mut pc: u32 = 0x82FF4198;
    'dispatch: loop {
        match pc {
            0x82FF4198 => {
    //   block [0x82FF4198..0x82FF41C0)
	// 82FF4198: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FF419C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF41A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF41A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF41A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF41AC: 4BFDECAD  bl 0x82fd2e58
	ctx.lr = 0x82FF41B0;
	sub_82FD2E58(ctx, base);
	// 82FF41B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF41B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF41B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF41BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF41C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF41C0 size=40
    let mut pc: u32 = 0x82FF41C0;
    'dispatch: loop {
        match pc {
            0x82FF41C0 => {
    //   block [0x82FF41C0..0x82FF41E8)
	// 82FF41C0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FF41C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF41C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF41CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF41D0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FF41D4: 4BFDEC85  bl 0x82fd2e58
	ctx.lr = 0x82FF41D8;
	sub_82FD2E58(ctx, base);
	// 82FF41D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF41DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF41E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF41E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF41E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF41E8 size=516
    let mut pc: u32 = 0x82FF41E8;
    'dispatch: loop {
        match pc {
            0x82FF41E8 => {
    //   block [0x82FF41E8..0x82FF43EC)
	// 82FF41E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF41EC: 481B3F59  bl 0x831a8144
	ctx.lr = 0x82FF41F0;
	sub_831A8130(ctx, base);
	// 82FF41F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF41F4: 7D535378  mr r19, r10
	ctx.r[19].u64 = ctx.r[10].u64;
	// 82FF41F8: 83610124  lwz r27, 0x124(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82FF41FC: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FF4200: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF4204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4208: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF420C: 616B8004  ori r11, r11, 0x8004
	ctx.r[11].u64 = ctx.r[11].u64 | 32772;
	// 82FF4210: 614AC008  ori r10, r10, 0xc008
	ctx.r[10].u64 = ctx.r[10].u64 | 49160;
	// 82FF4214: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82FF4218: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82FF421C: 3F9F0002  addis r28, r31, 2
	ctx.r[28].s64 = ctx.r[31].s64 + 131072;
	// 82FF4220: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF4224: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 82FF4228: 3D200001  lis r9, 1
	ctx.r[9].s64 = 65536;
	// 82FF422C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF4230: 7EFF512E  stwx r23, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u32) };
	// 82FF4234: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF4238: 6129C00C  ori r9, r9, 0xc00c
	ctx.r[9].u64 = ctx.r[9].u64 | 49164;
	// 82FF423C: 616BC018  ori r11, r11, 0xc018
	ctx.r[11].u64 = ctx.r[11].u64 | 49176;
	// 82FF4240: 614AC019  ori r10, r10, 0xc019
	ctx.r[10].u64 = ctx.r[10].u64 | 49177;
	// 82FF4244: 3B9CC014  addi r28, r28, -0x3fec
	ctx.r[28].s64 = ctx.r[28].s64 + -16364;
	// 82FF4248: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF424C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF4250: 7EFF492E  stwx r23, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[23].u32) };
	// 82FF4254: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF4258: 7FDF59AE  stbx r30, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u8) };
	// 82FF425C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FF4260: 7FDF51AE  stbx r30, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF4264: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82FF4268: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF426C: 7D154378  mr r21, r8
	ctx.r[21].u64 = ctx.r[8].u64;
	// 82FF4270: 4BFDC911  bl 0x82fd0b80
	ctx.lr = 0x82FF4274;
	sub_82FD0B80(ctx, base);
	// 82FF4274: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82FF4278: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FF427C: 616BC01C  ori r11, r11, 0xc01c
	ctx.r[11].u64 = ctx.r[11].u64 | 49180;
	// 82FF4280: 614AC020  ori r10, r10, 0xc020
	ctx.r[10].u64 = ctx.r[10].u64 | 49184;
	// 82FF4284: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF4288: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82FF428C: 61298028  ori r9, r9, 0x8028
	ctx.r[9].u64 = ctx.r[9].u64 | 32808;
	// 82FF4290: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF4294: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF4298: 7FDF512E  stwx r30, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FF429C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF42A0: 616B802C  ori r11, r11, 0x802c
	ctx.r[11].u64 = ctx.r[11].u64 | 32812;
	// 82FF42A4: 614A8030  ori r10, r10, 0x8030
	ctx.r[10].u64 = ctx.r[10].u64 | 32816;
	// 82FF42A8: 7D1F492E  stwx r8, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FF42AC: 3F5F0003  addis r26, r31, 3
	ctx.r[26].s64 = ctx.r[31].s64 + 196608;
	// 82FF42B0: 3F3F0003  addis r25, r31, 3
	ctx.r[25].s64 = ctx.r[31].s64 + 196608;
	// 82FF42B4: 3CE00002  lis r7, 2
	ctx.r[7].s64 = 131072;
	// 82FF42B8: 7EDF592E  stwx r22, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[22].u32) };
	// 82FF42BC: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF42C0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF42C4: 7FDF51AE  stbx r30, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u8) };
	// 82FF42C8: 89410117  lbz r10, 0x117(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(279 as u32) ) } as u64;
	// 82FF42CC: 60E78034  ori r7, r7, 0x8034
	ctx.r[7].u64 = ctx.r[7].u64 | 32820;
	// 82FF42D0: 61298038  ori r9, r9, 0x8038
	ctx.r[9].u64 = ctx.r[9].u64 | 32824;
	// 82FF42D4: 616B803D  ori r11, r11, 0x803d
	ctx.r[11].u64 = ctx.r[11].u64 | 32829;
	// 82FF42D8: 3B5A8024  addi r26, r26, -0x7fdc
	ctx.r[26].s64 = ctx.r[26].s64 + -32732;
	// 82FF42DC: 3B39803C  addi r25, r25, -0x7fc4
	ctx.r[25].s64 = ctx.r[25].s64 + -32708;
	// 82FF42E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF42E4: 7E9F392E  stwx r20, r31, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[20].u32) };
	// 82FF42E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF42EC: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82FF42F0: 7D5F59AE  stbx r10, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82FF42F4: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF42F8: 9BD90000  stb r30, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF42FC: 4BFDC885  bl 0x82fd0b80
	ctx.lr = 0x82FF4300;
	sub_82FD0B80(ctx, base);
	// 82FF4300: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF4304: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF4308: 616B8040  ori r11, r11, 0x8040
	ctx.r[11].u64 = ctx.r[11].u64 | 32832;
	// 82FF430C: 614A8044  ori r10, r10, 0x8044
	ctx.r[10].u64 = ctx.r[10].u64 | 32836;
	// 82FF4310: 3FBF0003  addis r29, r31, 3
	ctx.r[29].s64 = ctx.r[31].s64 + 196608;
	// 82FF4314: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FF4318: 3BBD8048  addi r29, r29, -0x7fb8
	ctx.r[29].s64 = ctx.r[29].s64 + -32696;
	// 82FF431C: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FF4320: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FF4324: 7F1F512E  stwx r24, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[24].u32) };
	// 82FF4328: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FF432C: 61298049  ori r9, r9, 0x8049
	ctx.r[9].u64 = ctx.r[9].u64 | 32841;
	// 82FF4330: 616B804C  ori r11, r11, 0x804c
	ctx.r[11].u64 = ctx.r[11].u64 | 32844;
	// 82FF4334: 614A8050  ori r10, r10, 0x8050
	ctx.r[10].u64 = ctx.r[10].u64 | 32848;
	// 82FF4338: 3F1F0003  addis r24, r31, 3
	ctx.r[24].s64 = ctx.r[31].s64 + 196608;
	// 82FF433C: 8081011C  lwz r4, 0x11c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 82FF4340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4344: 7E7F49AE  stbx r19, r31, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[19].u8) };
	// 82FF4348: 3B188060  addi r24, r24, -0x7fa0
	ctx.r[24].s64 = ctx.r[24].s64 + -32672;
	// 82FF434C: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 82FF4350: 7EBF512E  stwx r21, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u32) };
	// 82FF4354: 9BDD0000  stb r30, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF4358: 93780000  stw r27, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF435C: 4BFF390D  bl 0x82fe7c68
	ctx.lr = 0x82FF4360;
	sub_82FE7C68(ctx, base);
	// 82FF4360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4364: 4BFFE315  bl 0x82ff2678
	ctx.lr = 0x82FF4368;
	sub_82FF2678(ctx, base);
	// 82FF4368: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF436C: 806BB7DC  lwz r3, -0x4824(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FF4370: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4374: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF4378: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF437C: 4E800421  bctrl
	ctx.lr = 0x82FF4380;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4380: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF4384: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 82FF4388: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF438C: 3863C024  addi r3, r3, -0x3fdc
	ctx.r[3].s64 = ctx.r[3].s64 + -16348;
	// 82FF4390: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FF4394: 48041665  bl 0x830359f8
	ctx.lr = 0x82FF4398;
	sub_830359F8(ctx, base);
	// 82FF4398: 3F5F0002  addis r26, r31, 2
	ctx.r[26].s64 = ctx.r[31].s64 + 131072;
	// 82FF439C: 83780000  lwz r27, 0(r24)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF43A0: 3B5AC010  addi r26, r26, -0x3ff0
	ctx.r[26].s64 = ctx.r[26].s64 + -16368;
	// 82FF43A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF43A8: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF43AC: 48041A8D  bl 0x83035e38
	ctx.lr = 0x82FF43B0;
	sub_83035E38(ctx, base);
	// 82FF43B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF43B4: 4BFDC7CD  bl 0x82fd0b80
	ctx.lr = 0x82FF43B8;
	sub_82FD0B80(ctx, base);
	// 82FF43B8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF43BC: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FF43C0: 9BDD0000  stb r30, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF43C4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FF43C8: 419A000C  beq cr6, 0x82ff43d4
	if ctx.cr[6].eq {
	pc = 0x82FF43D4; continue 'dispatch;
	}
	// 82FF43CC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FF43D0: 409A0008  bne cr6, 0x82ff43d8
	if !ctx.cr[6].eq {
	pc = 0x82FF43D8; continue 'dispatch;
	}
	// 82FF43D4: 9AFD0000  stb r23, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 82FF43D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF43DC: 4BFFF5CD  bl 0x82ff39a8
	ctx.lr = 0x82FF43E0;
	sub_82FF39A8(ctx, base);
	// 82FF43E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF43E4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FF43E8: 481B3DAC  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF43F0 size=92
    let mut pc: u32 = 0x82FF43F0;
    'dispatch: loop {
        match pc {
            0x82FF43F0 => {
    //   block [0x82FF43F0..0x82FF444C)
	// 82FF43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF43F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF43FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4404: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF4408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF440C: 396B7838  addi r11, r11, 0x7838
	ctx.r[11].s64 = ctx.r[11].s64 + 30776;
	// 82FF4410: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF4414: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF4418: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF441C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4420: 4BFDC761  bl 0x82fd0b80
	ctx.lr = 0x82FF4424;
	sub_82FD0B80(ctx, base);
	// 82FF4424: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF4428: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FF442C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4430: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FF4434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF4438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF4444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4450 size=8
    let mut pc: u32 = 0x82FF4450;
    'dispatch: loop {
        match pc {
            0x82FF4450 => {
    //   block [0x82FF4450..0x82FF4458)
	// 82FF4450: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF4454: 8213EA58  lwz r16, -0x15a8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4458 size=120
    let mut pc: u32 = 0x82FF4458;
    'dispatch: loop {
        match pc {
            0x82FF4458 => {
    //   block [0x82FF4458..0x82FF44D0)
	// 82FF4458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF445C: 481B3D01  bl 0x831a815c
	ctx.lr = 0x82FF4460;
	sub_831A8130(ctx, base);
	// 82FF4460: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF4464: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4468: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF446C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82FF4470: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF4474: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FF4478: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FF447C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF4480: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82FF4484: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82FF4488: 4BFFFF69  bl 0x82ff43f0
	ctx.lr = 0x82FF448C;
	sub_82FF43F0(ctx, base);
	// 82FF448C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4490: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF4494: 933E000C  stw r25, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 82FF4498: 396BEA48  addi r11, r11, -0x15b8
	ctx.r[11].s64 = ctx.r[11].s64 + -5560;
	// 82FF449C: 935E0010  stw r26, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82FF44A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF44A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF44A8: 4BFDC6D9  bl 0x82fd0b80
	ctx.lr = 0x82FF44AC;
	sub_82FD0B80(ctx, base);
	// 82FF44AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF44B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF44B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF44B8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FF44BC: 4BFDC6C5  bl 0x82fd0b80
	ctx.lr = 0x82FF44C0;
	sub_82FD0B80(ctx, base);
	// 82FF44C0: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FF44C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF44C8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF44CC: 481B3CE0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF44D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF44D0 size=40
    let mut pc: u32 = 0x82FF44D0;
    'dispatch: loop {
        match pc {
            0x82FF44D0 => {
    //   block [0x82FF44D0..0x82FF44F8)
	// 82FF44D0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF44D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF44D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF44DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF44E0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF44E4: 4BFE3705  bl 0x82fd7be8
	ctx.lr = 0x82FF44E8;
	sub_82FD7BE8(ctx, base);
	// 82FF44E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF44EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF44F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF44F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF44F8 size=8
    let mut pc: u32 = 0x82FF44F8;
    'dispatch: loop {
        match pc {
            0x82FF44F8 => {
    //   block [0x82FF44F8..0x82FF4500)
	// 82FF44F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF44FC: 8213EA90  lwz r16, -0x1570(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4500 size=116
    let mut pc: u32 = 0x82FF4500;
    'dispatch: loop {
        match pc {
            0x82FF4500 => {
    //   block [0x82FF4500..0x82FF4574)
	// 82FF4500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4504: 481B3C69  bl 0x831a816c
	ctx.lr = 0x82FF4508;
	sub_831A8130(ctx, base);
	// 82FF4508: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF450C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4510: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4514: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF4518: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF451C: 4BFE366D  bl 0x82fd7b88
	ctx.lr = 0x82FF4520;
	sub_82FD7B88(ctx, base);
	// 82FF4520: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4524: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF4528: 396BEA48  addi r11, r11, -0x15b8
	ctx.r[11].s64 = ctx.r[11].s64 + -5560;
	// 82FF452C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4530: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF4534: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FF4538: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF453C: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82FF4540: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FF4544: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FF4548: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF454C: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF4550: 4BFDC631  bl 0x82fd0b80
	ctx.lr = 0x82FF4554;
	sub_82FD0B80(ctx, base);
	// 82FF4554: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82FF4558: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF455C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF4560: 4BFDC621  bl 0x82fd0b80
	ctx.lr = 0x82FF4564;
	sub_82FD0B80(ctx, base);
	// 82FF4564: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FF4568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF456C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF4570: 481B3C4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4574(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4574 size=40
    let mut pc: u32 = 0x82FF4574;
    'dispatch: loop {
        match pc {
            0x82FF4574 => {
    //   block [0x82FF4574..0x82FF459C)
	// 82FF4574: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF4578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF457C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4584: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF4588: 4BFE3661  bl 0x82fd7be8
	ctx.lr = 0x82FF458C;
	sub_82FD7BE8(ctx, base);
	// 82FF458C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF4590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF45A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF45A0 size=8
    let mut pc: u32 = 0x82FF45A0;
    'dispatch: loop {
        match pc {
            0x82FF45A0 => {
    //   block [0x82FF45A0..0x82FF45A8)
	// 82FF45A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF45A4: 8213EAC8  lwz r16, -0x1538(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5432 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF45A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF45A8 size=152
    let mut pc: u32 = 0x82FF45A8;
    'dispatch: loop {
        match pc {
            0x82FF45A8 => {
    //   block [0x82FF45A8..0x82FF4640)
	// 82FF45A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF45AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF45B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF45B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF45B8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF45BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF45C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF45C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF45C8: 396BEA48  addi r11, r11, -0x15b8
	ctx.r[11].s64 = ctx.r[11].s64 + -5560;
	// 82FF45CC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF45D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF45D4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF45D8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF45DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF45E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF45E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF45E8: 4E800421  bctrl
	ctx.lr = 0x82FF45EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF45EC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF45F0: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF45F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF45F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF45FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4600: 4E800421  bctrl
	ctx.lr = 0x82FF4604;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4604: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF4608: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF460C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF4610: 396B7838  addi r11, r11, 0x7838
	ctx.r[11].s64 = ctx.r[11].s64 + 30776;
	// 82FF4614: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF461C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4624: 4E800421  bctrl
	ctx.lr = 0x82FF4628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4628: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF462C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF4638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF463C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4640 size=40
    let mut pc: u32 = 0x82FF4640;
    'dispatch: loop {
        match pc {
            0x82FF4640 => {
    //   block [0x82FF4640..0x82FF4668)
	// 82FF4640: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF4644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF464C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4650: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF4654: 4BFE3595  bl 0x82fd7be8
	ctx.lr = 0x82FF4658;
	sub_82FD7BE8(ctx, base);
	// 82FF4658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF465C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4668 size=76
    let mut pc: u32 = 0x82FF4668;
    'dispatch: loop {
        match pc {
            0x82FF4668 => {
    //   block [0x82FF4668..0x82FF46B4)
	// 82FF4668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF4674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF467C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF4684: 4BFFFF25  bl 0x82ff45a8
	ctx.lr = 0x82FF4688;
	sub_82FF45A8(ctx, base);
	// 82FF4688: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF468C: 4182000C  beq 0x82ff4698
	if ctx.cr[0].eq {
	pc = 0x82FF4698; continue 'dispatch;
	}
	// 82FF4690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4694: 4BFE3C4D  bl 0x82fd82e0
	ctx.lr = 0x82FF4698;
	sub_82FD82E0(ctx, base);
	// 82FF4698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF469C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF46A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF46A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF46A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF46AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF46B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF46B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF46B8 size=68
    let mut pc: u32 = 0x82FF46B8;
    'dispatch: loop {
        match pc {
            0x82FF46B8 => {
    //   block [0x82FF46B8..0x82FF46FC)
	// 82FF46B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF46BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF46C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF46C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF46C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF46CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF46D0: 48041891  bl 0x83035f60
	ctx.lr = 0x82FF46D4;
	sub_83035F60(ctx, base);
	// 82FF46D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FF46D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF46DC: 3BEBBE64  addi r31, r11, -0x419c
	ctx.r[31].s64 = ctx.r[11].s64 + -16796;
	// 82FF46E0: 481BBD11  bl 0x831b03f0
	ctx.lr = 0x82FF46E4;
	sub_831B03F0(ctx, base);
	// 82FF46E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF46E8: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 82FF46EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF46F0: 481BCA11  bl 0x831b1100
	ctx.lr = 0x82FF46F4;
	sub_831B1100(ctx, base);
	// 82FF46F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FF46F8: 481B8489  bl 0x831acb80
	ctx.lr = 0x82FF46FC;
	sub_831ACB80(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4700 size=8
    let mut pc: u32 = 0x82FF4700;
    'dispatch: loop {
        match pc {
            0x82FF4700 => {
    //   block [0x82FF4700..0x82FF4708)
	// 82FF4700: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF4704: 8213EB64  lwz r16, -0x149c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5276 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4708 size=84
    let mut pc: u32 = 0x82FF4708;
    'dispatch: loop {
        match pc {
            0x82FF4708 => {
    //   block [0x82FF4708..0x82FF475C)
	// 82FF4708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF470C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4710: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FF4714: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FF4718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF471C: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF4720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4724: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF4728: 4B2CC211  bl 0x822c0938
	ctx.lr = 0x82FF472C;
	sub_822C0938(ctx, base);
	// 82FF472C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FF4730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FF4734: 409A0014  bne cr6, 0x82ff4748
	if !ctx.cr[6].eq {
	pc = 0x82FF4748; continue 'dispatch;
	}
	// 82FF4738: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF473C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FF4740: 388BC7AC  addi r4, r11, -0x3854
	ctx.r[4].s64 = ctx.r[11].s64 + -14420;
	// 82FF4744: 481BC4E5  bl 0x831b0c28
	ctx.lr = 0x82FF4748;
	sub_831B0C28(ctx, base);
	// 82FF4748: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF474C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4754: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4764 size=32
    let mut pc: u32 = 0x82FF4764;
    'dispatch: loop {
        match pc {
            0x82FF4764 => {
    //   block [0x82FF4764..0x82FF4784)
	// 82FF4764: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF4768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4774: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4778: 387F0051  addi r3, r31, 0x51
	ctx.r[3].s64 = ctx.r[31].s64 + 81;
	// 82FF477C: 388BC7AC  addi r4, r11, -0x3854
	ctx.r[4].s64 = ctx.r[11].s64 + -14420;
	// 82FF4780: 481BC4A9  bl 0x831b0c28
	ctx.lr = 0x82FF4784;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4788 size=8
    let mut pc: u32 = 0x82FF4788;
    'dispatch: loop {
        match pc {
            0x82FF4788 => {
    //   block [0x82FF4788..0x82FF4790)
	// 82FF4788: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF478C: 4B2CBADC  b 0x822c0268
	sub_822C0268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4790 size=128
    let mut pc: u32 = 0x82FF4790;
    'dispatch: loop {
        match pc {
            0x82FF4790 => {
    //   block [0x82FF4790..0x82FF4810)
	// 82FF4790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4794: 481B39D9  bl 0x831a816c
	ctx.lr = 0x82FF4798;
	sub_831A8130(ctx, base);
	// 82FF4798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF479C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 82FF47A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF47A4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF47A8: 809DB8EC  lwz r4, -0x4714(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-18196 as u32) ) } as u64;
	// 82FF47AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FF47B0: 419A0020  beq cr6, 0x82ff47d0
	if ctx.cr[6].eq {
	pc = 0x82FF47D0; continue 'dispatch;
	}
	// 82FF47B4: 807EB7E8  lwz r3, -0x4818(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF47B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF47BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF47C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF47C4: 4E800421  bctrl
	ctx.lr = 0x82FF47C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF47C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF47CC: 917DB8EC  stw r11, -0x4714(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-18196 as u32), ctx.r[11].u32 ) };
	// 82FF47D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FF47D4: 419A0034  beq cr6, 0x82ff4808
	if ctx.cr[6].eq {
	pc = 0x82FF4808; continue 'dispatch;
	}
	// 82FF47D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF47DC: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF47E0: 4BFDCB29  bl 0x82fd1308
	ctx.lr = 0x82FF47E4;
	sub_82FD1308(ctx, base);
	// 82FF47E4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FF47E8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FF47EC: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF47F0: 3BEB3140  addi r31, r11, 0x3140
	ctx.r[31].s64 = ctx.r[11].s64 + 12608;
	// 82FF47F4: 907DB8EC  stw r3, -0x4714(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-18196 as u32), ctx.r[3].u32 ) };
	// 82FF47F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF47FC: 4BFDCC3D  bl 0x82fd1438
	ctx.lr = 0x82FF4800;
	sub_82FD1438(ctx, base);
	// 82FF4800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF4804: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FF4808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF480C: 481B39B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4810 size=96
    let mut pc: u32 = 0x82FF4810;
    'dispatch: loop {
        match pc {
            0x82FF4810 => {
    //   block [0x82FF4810..0x82FF4870)
	// 82FF4810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4814: 481B3959  bl 0x831a816c
	ctx.lr = 0x82FF4818;
	sub_831A8130(ctx, base);
	// 82FF4818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF481C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FF4820: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF4824: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF4828: 809FB8F0  lwz r4, -0x4710(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18192 as u32) ) } as u64;
	// 82FF482C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FF4830: 419A0020  beq cr6, 0x82ff4850
	if ctx.cr[6].eq {
	pc = 0x82FF4850; continue 'dispatch;
	}
	// 82FF4834: 807EB7E8  lwz r3, -0x4818(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF4838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF483C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4844: 4E800421  bctrl
	ctx.lr = 0x82FF4848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF484C: 917FB8F0  stw r11, -0x4710(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18192 as u32), ctx.r[11].u32 ) };
	// 82FF4850: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF4854: 419A0014  beq cr6, 0x82ff4868
	if ctx.cr[6].eq {
	pc = 0x82FF4868; continue 'dispatch;
	}
	// 82FF4858: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF485C: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF4860: 4BFDCAA9  bl 0x82fd1308
	ctx.lr = 0x82FF4864;
	sub_82FD1308(ctx, base);
	// 82FF4864: 907FB8F0  stw r3, -0x4710(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18192 as u32), ctx.r[3].u32 ) };
	// 82FF4868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF486C: 481B3950  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4870 size=12
    let mut pc: u32 = 0x82FF4870;
    'dispatch: loop {
        match pc {
            0x82FF4870 => {
    //   block [0x82FF4870..0x82FF487C)
	// 82FF4870: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FF4874: 386B3140  addi r3, r11, 0x3140
	ctx.r[3].s64 = ctx.r[11].s64 + 12608;
	// 82FF4878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4880 size=8
    let mut pc: u32 = 0x82FF4880;
    'dispatch: loop {
        match pc {
            0x82FF4880 => {
    //   block [0x82FF4880..0x82FF4888)
	// 82FF4880: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF4884: 8213EBB8  lwz r16, -0x1448(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4888 size=72
    let mut pc: u32 = 0x82FF4888;
    'dispatch: loop {
        match pc {
            0x82FF4888 => {
    //   block [0x82FF4888..0x82FF48D0)
	// 82FF4888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF488C: 481B38E1  bl 0x831a816c
	ctx.lr = 0x82FF4890;
	sub_831A8130(ctx, base);
	// 82FF4890: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF4894: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF489C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FF48A0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FF48A4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF48A8: 4BFE4689  bl 0x82fd8f30
	ctx.lr = 0x82FF48AC;
	sub_82FD8F30(ctx, base);
	// 82FF48AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF48B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF48B4: 396BEBA0  addi r11, r11, -0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + -5216;
	// 82FF48B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF48BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF48C0: 4BFE49F9  bl 0x82fd92b8
	ctx.lr = 0x82FF48C4;
	sub_82FD92B8(ctx, base);
	// 82FF48C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF48C8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF48CC: 481B38F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF48D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF48D0 size=40
    let mut pc: u32 = 0x82FF48D0;
    'dispatch: loop {
        match pc {
            0x82FF48D0 => {
    //   block [0x82FF48D0..0x82FF48F8)
	// 82FF48D0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF48D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF48D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF48DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF48E0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF48E4: 4BFE4595  bl 0x82fd8e78
	ctx.lr = 0x82FF48E8;
	sub_82FD8E78(ctx, base);
	// 82FF48E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF48EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF48F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF48F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF48F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF48F8 size=60
    let mut pc: u32 = 0x82FF48F8;
    'dispatch: loop {
        match pc {
            0x82FF48F8 => {
    //   block [0x82FF48F8..0x82FF4934)
	// 82FF48F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF48FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF490C: 4BFE469D  bl 0x82fd8fa8
	ctx.lr = 0x82FF4910;
	sub_82FD8FA8(ctx, base);
	// 82FF4910: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4918: 396BEBA0  addi r11, r11, -0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + -5216;
	// 82FF491C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF4924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF492C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4938 size=16
    let mut pc: u32 = 0x82FF4938;
    'dispatch: loop {
        match pc {
            0x82FF4938 => {
    //   block [0x82FF4938..0x82FF4948)
	// 82FF4938: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF493C: 396BEBA0  addi r11, r11, -0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + -5216;
	// 82FF4940: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4944: 4BFE4534  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4948 size=8
    let mut pc: u32 = 0x82FF4948;
    'dispatch: loop {
        match pc {
            0x82FF4948 => {
    //   block [0x82FF4948..0x82FF4950)
	// 82FF4948: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF494C: 8213EBF0  lwz r16, -0x1410(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4950 size=92
    let mut pc: u32 = 0x82FF4950;
    'dispatch: loop {
        match pc {
            0x82FF4950 => {
    //   block [0x82FF4950..0x82FF49AC)
	// 82FF4950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4954: 481B3819  bl 0x831a816c
	ctx.lr = 0x82FF4958;
	sub_831A8130(ctx, base);
	// 82FF4958: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF495C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4960: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF4964: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FF4968: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF496C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FF4970: 4BFE3929  bl 0x82fd8298
	ctx.lr = 0x82FF4974;
	sub_82FD8298(ctx, base);
	// 82FF4974: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF4978: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF497C: 41820024  beq 0x82ff49a0
	if ctx.cr[0].eq {
	pc = 0x82FF49A0; continue 'dispatch;
	}
	// 82FF4980: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF4984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF4988: 4BFE4621  bl 0x82fd8fa8
	ctx.lr = 0x82FF498C;
	sub_82FD8FA8(ctx, base);
	// 82FF498C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF4994: 396BEBA0  addi r11, r11, -0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + -5216;
	// 82FF4998: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF499C: 48000008  b 0x82ff49a4
	pc = 0x82FF49A4; continue 'dispatch;
	// 82FF49A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF49A4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF49A8: 481B3814  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF49AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF49AC size=48
    let mut pc: u32 = 0x82FF49AC;
    'dispatch: loop {
        match pc {
            0x82FF49AC => {
    //   block [0x82FF49AC..0x82FF49DC)
	// 82FF49AC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF49B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF49B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF49B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF49BC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FF49C0: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF49C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF49C8: 4BFE3919  bl 0x82fd82e0
	ctx.lr = 0x82FF49CC;
	sub_82FD82E0(ctx, base);
	// 82FF49CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF49D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF49D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF49D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF49E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF49E0 size=12
    let mut pc: u32 = 0x82FF49E0;
    'dispatch: loop {
        match pc {
            0x82FF49E0 => {
    //   block [0x82FF49E0..0x82FF49EC)
	// 82FF49E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF49E4: 386B837C  addi r3, r11, -0x7c84
	ctx.r[3].s64 = ctx.r[11].s64 + -31876;
	// 82FF49E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF49F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF49F0 size=88
    let mut pc: u32 = 0x82FF49F0;
    'dispatch: loop {
        match pc {
            0x82FF49F0 => {
    //   block [0x82FF49F0..0x82FF4A48)
	// 82FF49F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF49F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF49F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF49FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4A04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4A0C: 396BEBA0  addi r11, r11, -0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + -5216;
	// 82FF4A10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF4A14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF4A18: 4BFE4461  bl 0x82fd8e78
	ctx.lr = 0x82FF4A1C;
	sub_82FD8E78(ctx, base);
	// 82FF4A1C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF4A20: 4182000C  beq 0x82ff4a2c
	if ctx.cr[0].eq {
	pc = 0x82FF4A2C; continue 'dispatch;
	}
	// 82FF4A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4A28: 4BFE38B9  bl 0x82fd82e0
	ctx.lr = 0x82FF4A2C;
	sub_82FD82E0(ctx, base);
	// 82FF4A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4A30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF4A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4A3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF4A40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4A48 size=40
    let mut pc: u32 = 0x82FF4A48;
    'dispatch: loop {
        match pc {
            0x82FF4A48 => {
    //   block [0x82FF4A48..0x82FF4A70)
	// 82FF4A48: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF4A4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF4A50: 816BB7E0  lwz r11, -0x4820(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18464 as u32) ) } as u64;
	// 82FF4A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF4A58: 419A0018  beq cr6, 0x82ff4a70
	if ctx.cr[6].eq {
		sub_82FF4A70(ctx, base);
		return;
	}
	// 82FF4A5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4A60: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF4A64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF4A68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4A6C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4A70 size=28
    let mut pc: u32 = 0x82FF4A70;
    'dispatch: loop {
        match pc {
            0x82FF4A70 => {
    //   block [0x82FF4A70..0x82FF4A8C)
	// 82FF4A70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF4A74: 816BB7E4  lwz r11, -0x481c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18460 as u32) ) } as u64;
	// 82FF4A78: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF4A7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4A80: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF4A84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4A88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4A90 size=96
    let mut pc: u32 = 0x82FF4A90;
    'dispatch: loop {
        match pc {
            0x82FF4A90 => {
    //   block [0x82FF4A90..0x82FF4AF0)
	// 82FF4A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4A9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF4AA4: 481BCE8D  bl 0x831b1930
	ctx.lr = 0x82FF4AA8;
	sub_831B1930(ctx, base);
	// 82FF4AA8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FF4AAC: 409A0030  bne cr6, 0x82ff4adc
	if !ctx.cr[6].eq {
	pc = 0x82FF4ADC; continue 'dispatch;
	}
	// 82FF4AB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4AB4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82FF4AB8: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 82FF4ABC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82FF4AC0: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82FF4AC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4AC8: 4BFFFDC1  bl 0x82ff4888
	ctx.lr = 0x82FF4ACC;
	sub_82FF4888(ctx, base);
	// 82FF4ACC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4AD4: 388BC800  addi r4, r11, -0x3800
	ctx.r[4].s64 = ctx.r[11].s64 + -14336;
	// 82FF4AD8: 481BC151  bl 0x831b0c28
	ctx.lr = 0x82FF4ADC;
	sub_831B0C28(ctx, base);
	// 82FF4ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF4AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4AF0 size=96
    let mut pc: u32 = 0x82FF4AF0;
    'dispatch: loop {
        match pc {
            0x82FF4AF0 => {
    //   block [0x82FF4AF0..0x82FF4B50)
	// 82FF4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4AFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4B00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF4B04: 481BABBD  bl 0x831af6c0
	ctx.lr = 0x82FF4B08;
	sub_831AF6C0(ctx, base);
	// 82FF4B08: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FF4B0C: 409A0030  bne cr6, 0x82ff4b3c
	if !ctx.cr[6].eq {
	pc = 0x82FF4B3C; continue 'dispatch;
	}
	// 82FF4B10: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4B14: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82FF4B18: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 82FF4B1C: 38C00021  li r6, 0x21
	ctx.r[6].s64 = 33;
	// 82FF4B20: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 82FF4B24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4B28: 4BFFFD61  bl 0x82ff4888
	ctx.lr = 0x82FF4B2C;
	sub_82FF4888(ctx, base);
	// 82FF4B2C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4B30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4B34: 388BC800  addi r4, r11, -0x3800
	ctx.r[4].s64 = ctx.r[11].s64 + -14336;
	// 82FF4B38: 481BC0F1  bl 0x831b0c28
	ctx.lr = 0x82FF4B3C;
	sub_831B0C28(ctx, base);
	// 82FF4B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF4B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4B48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4B50 size=392
    let mut pc: u32 = 0x82FF4B50;
    'dispatch: loop {
        match pc {
            0x82FF4B50 => {
    //   block [0x82FF4B50..0x82FF4CD8)
	// 82FF4B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4B54: 481B3615  bl 0x831a8168
	ctx.lr = 0x82FF4B58;
	sub_831A8130(ctx, base);
	// 82FF4B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4B5C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF4B60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FF4B64: 419A016C  beq cr6, 0x82ff4cd0
	if ctx.cr[6].eq {
	pc = 0x82FF4CD0; continue 'dispatch;
	}
	// 82FF4B68: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4B6C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF4B70: 409A0084  bne cr6, 0x82ff4bf4
	if !ctx.cr[6].eq {
	pc = 0x82FF4BF4; continue 'dispatch;
	}
	// 82FF4B74: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 82FF4B78: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82FF4B7C: 48000008  b 0x82ff4b84
	pc = 0x82FF4B84; continue 'dispatch;
	// 82FF4B80: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF4B84: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4B88: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4B8C: 4082FFF4  bne 0x82ff4b80
	if !ctx.cr[0].eq {
	pc = 0x82FF4B80; continue 'dispatch;
	}
	// 82FF4B90: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FF4B94: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF4B98: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82FF4B9C: 40990058  ble cr6, 0x82ff4bf4
	if !ctx.cr[6].gt {
	pc = 0x82FF4BF4; continue 'dispatch;
	}
	// 82FF4BA0: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF4BA4: 2B09003A  cmplwi cr6, r9, 0x3a
	ctx.cr[6].compare_u32(ctx.r[9].u32, 58 as u32, &mut ctx.xer);
	// 82FF4BA8: 409A002C  bne cr6, 0x82ff4bd4
	if !ctx.cr[6].eq {
	pc = 0x82FF4BD4; continue 'dispatch;
	}
	// 82FF4BAC: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4BB0: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 82FF4BB4: 4198000C  blt cr6, 0x82ff4bc0
	if ctx.cr[6].lt {
	pc = 0x82FF4BC0; continue 'dispatch;
	}
	// 82FF4BB8: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 82FF4BBC: 40990014  ble cr6, 0x82ff4bd0
	if !ctx.cr[6].gt {
	pc = 0x82FF4BD0; continue 'dispatch;
	}
	// 82FF4BC0: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 82FF4BC4: 41980010  blt cr6, 0x82ff4bd4
	if ctx.cr[6].lt {
	pc = 0x82FF4BD4; continue 'dispatch;
	}
	// 82FF4BC8: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 82FF4BCC: 41990008  bgt cr6, 0x82ff4bd4
	if ctx.cr[6].gt {
	pc = 0x82FF4BD4; continue 'dispatch;
	}
	// 82FF4BD0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82FF4BD4: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4BD8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FF4BDC: 409A0018  bne cr6, 0x82ff4bf4
	if !ctx.cr[6].eq {
	pc = 0x82FF4BF4; continue 'dispatch;
	}
	// 82FF4BE0: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF4BE4: 419A000C  beq cr6, 0x82ff4bf0
	if ctx.cr[6].eq {
	pc = 0x82FF4BF0; continue 'dispatch;
	}
	// 82FF4BE8: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF4BEC: 409A0008  bne cr6, 0x82ff4bf4
	if !ctx.cr[6].eq {
	pc = 0x82FF4BF4; continue 'dispatch;
	}
	// 82FF4BF0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82FF4BF4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4BF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF4BFC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FF4C00: 4800001C  b 0x82ff4c1c
	pc = 0x82FF4C1C; continue 'dispatch;
	// 82FF4C04: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF4C08: 419A001C  beq cr6, 0x82ff4c24
	if ctx.cr[6].eq {
	pc = 0x82FF4C24; continue 'dispatch;
	}
	// 82FF4C0C: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF4C10: 419A0014  beq cr6, 0x82ff4c24
	if ctx.cr[6].eq {
	pc = 0x82FF4C24; continue 'dispatch;
	}
	// 82FF4C14: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF4C18: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4C20: 4082FFE4  bne 0x82ff4c04
	if !ctx.cr[0].eq {
	pc = 0x82FF4C04; continue 'dispatch;
	}
	// 82FF4C24: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C28: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4C2C: 4182004C  beq 0x82ff4c78
	if ctx.cr[0].eq {
	pc = 0x82FF4C78; continue 'dispatch;
	}
	// 82FF4C30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF4C34: 4BFDBF4D  bl 0x82fd0b80
	ctx.lr = 0x82FF4C38;
	sub_82FD0B80(ctx, base);
	// 82FF4C38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4C3C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FF4C40: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C44: 48000028  b 0x82ff4c6c
	pc = 0x82FF4C6C; continue 'dispatch;
	// 82FF4C48: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C4C: 2B0A00A5  cmplwi cr6, r10, 0xa5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 165 as u32, &mut ctx.xer);
	// 82FF4C50: 419A000C  beq cr6, 0x82ff4c5c
	if ctx.cr[6].eq {
	pc = 0x82FF4C5C; continue 'dispatch;
	}
	// 82FF4C54: 2B0A20A9  cmplwi cr6, r10, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8361 as u32, &mut ctx.xer);
	// 82FF4C58: 409A000C  bne cr6, 0x82ff4c64
	if !ctx.cr[6].eq {
	pc = 0x82FF4C64; continue 'dispatch;
	}
	// 82FF4C5C: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82FF4C60: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FF4C64: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF4C68: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C6C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4C70: 4082FFD8  bne 0x82ff4c48
	if !ctx.cr[0].eq {
	pc = 0x82FF4C48; continue 'dispatch;
	}
	// 82FF4C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF4C78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF4C7C: 4BFDC74D  bl 0x82fd13c8
	ctx.lr = 0x82FF4C80;
	sub_82FD13C8(ctx, base);
	// 82FF4C80: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FF4C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4C88: 388B08A4  addi r4, r11, 0x8a4
	ctx.r[4].s64 = ctx.r[11].s64 + 2212;
	// 82FF4C8C: 481BC9C5  bl 0x831b1650
	ctx.lr = 0x82FF4C90;
	sub_831B1650(ctx, base);
	// 82FF4C90: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4C94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF4C98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF4C9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF4CA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4CA8: 4E800421  bctrl
	ctx.lr = 0x82FF4CAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4CAC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF4CB0: 419A001C  beq cr6, 0x82ff4ccc
	if ctx.cr[6].eq {
	pc = 0x82FF4CCC; continue 'dispatch;
	}
	// 82FF4CB4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4CB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF4CBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF4CC0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4CC8: 4E800421  bctrl
	ctx.lr = 0x82FF4CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4CCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF4CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF4CD4: 481B34E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4CD8 size=252
    let mut pc: u32 = 0x82FF4CD8;
    'dispatch: loop {
        match pc {
            0x82FF4CD8 => {
    //   block [0x82FF4CD8..0x82FF4DD4)
	// 82FF4CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4CDC: 481B348D  bl 0x831a8168
	ctx.lr = 0x82FF4CE0;
	sub_831A8130(ctx, base);
	// 82FF4CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4CE4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF4CE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FF4CEC: 419A00E0  beq cr6, 0x82ff4dcc
	if ctx.cr[6].eq {
	pc = 0x82FF4DCC; continue 'dispatch;
	}
	// 82FF4CF0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4CF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF4CF8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FF4CFC: 4800001C  b 0x82ff4d18
	pc = 0x82FF4D18; continue 'dispatch;
	// 82FF4D00: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF4D04: 419A001C  beq cr6, 0x82ff4d20
	if ctx.cr[6].eq {
	pc = 0x82FF4D20; continue 'dispatch;
	}
	// 82FF4D08: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF4D0C: 419A0014  beq cr6, 0x82ff4d20
	if ctx.cr[6].eq {
	pc = 0x82FF4D20; continue 'dispatch;
	}
	// 82FF4D10: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF4D14: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4D1C: 4082FFE4  bne 0x82ff4d00
	if !ctx.cr[0].eq {
	pc = 0x82FF4D00; continue 'dispatch;
	}
	// 82FF4D20: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4D28: 4182004C  beq 0x82ff4d74
	if ctx.cr[0].eq {
	pc = 0x82FF4D74; continue 'dispatch;
	}
	// 82FF4D2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF4D30: 4BFDBE51  bl 0x82fd0b80
	ctx.lr = 0x82FF4D34;
	sub_82FD0B80(ctx, base);
	// 82FF4D34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4D38: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FF4D3C: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D40: 48000028  b 0x82ff4d68
	pc = 0x82FF4D68; continue 'dispatch;
	// 82FF4D44: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D48: 2B0A00A5  cmplwi cr6, r10, 0xa5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 165 as u32, &mut ctx.xer);
	// 82FF4D4C: 419A000C  beq cr6, 0x82ff4d58
	if ctx.cr[6].eq {
	pc = 0x82FF4D58; continue 'dispatch;
	}
	// 82FF4D50: 2B0A20A9  cmplwi cr6, r10, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8361 as u32, &mut ctx.xer);
	// 82FF4D54: 409A000C  bne cr6, 0x82ff4d60
	if !ctx.cr[6].eq {
	pc = 0x82FF4D60; continue 'dispatch;
	}
	// 82FF4D58: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82FF4D5C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FF4D60: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF4D64: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D68: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4D6C: 4082FFD8  bne 0x82ff4d44
	if !ctx.cr[0].eq {
	pc = 0x82FF4D44; continue 'dispatch;
	}
	// 82FF4D70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF4D74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF4D78: 4BFDC651  bl 0x82fd13c8
	ctx.lr = 0x82FF4D7C;
	sub_82FD13C8(ctx, base);
	// 82FF4D7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82FF4D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4D84: 388B0930  addi r4, r11, 0x930
	ctx.r[4].s64 = ctx.r[11].s64 + 2352;
	// 82FF4D88: 481BC8C9  bl 0x831b1650
	ctx.lr = 0x82FF4D8C;
	sub_831B1650(ctx, base);
	// 82FF4D8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4D90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FF4D94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF4D98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF4D9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4DA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4DA4: 4E800421  bctrl
	ctx.lr = 0x82FF4DA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4DA8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF4DAC: 419A001C  beq cr6, 0x82ff4dc8
	if ctx.cr[6].eq {
	pc = 0x82FF4DC8; continue 'dispatch;
	}
	// 82FF4DB0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF4DB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF4DB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF4DBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF4DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF4DC4: 4E800421  bctrl
	ctx.lr = 0x82FF4DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF4DC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF4DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF4DD0: 481B33E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4DD8 size=128
    let mut pc: u32 = 0x82FF4DD8;
    'dispatch: loop {
        match pc {
            0x82FF4DD8 => {
    //   block [0x82FF4DD8..0x82FF4E58)
	// 82FF4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4DDC: 481B338D  bl 0x831a8168
	ctx.lr = 0x82FF4DE0;
	sub_831A8130(ctx, base);
	// 82FF4DE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4DE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4DE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF4DEC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FF4DF0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FF4DF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF4DF8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FF4DFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF4E00: 481BA299  bl 0x831af098
	ctx.lr = 0x82FF4E04;
	sub_831AF098(ctx, base);
	// 82FF4E04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF4E08: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FF4E0C: 40980040  bge cr6, 0x82ff4e4c
	if !ctx.cr[6].lt {
	pc = 0x82FF4E4C; continue 'dispatch;
	}
	// 82FF4E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF4E14: 481BC64D  bl 0x831b1460
	ctx.lr = 0x82FF4E18;
	sub_831B1460(ctx, base);
	// 82FF4E18: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF4E1C: 41820030  beq 0x82ff4e4c
	if ctx.cr[0].eq {
	pc = 0x82FF4E4C; continue 'dispatch;
	}
	// 82FF4E20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4E24: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FF4E28: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 82FF4E2C: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82FF4E30: 38A0015D  li r5, 0x15d
	ctx.r[5].s64 = 349;
	// 82FF4E34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4E38: 4BFFFA51  bl 0x82ff4888
	ctx.lr = 0x82FF4E3C;
	sub_82FF4888(ctx, base);
	// 82FF4E3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4E40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4E44: 388BC800  addi r4, r11, -0x3800
	ctx.r[4].s64 = ctx.r[11].s64 + -14336;
	// 82FF4E48: 481BBDE1  bl 0x831b0c28
	ctx.lr = 0x82FF4E4C;
	sub_831B0C28(ctx, base);
	// 82FF4E4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF4E50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF4E54: 481B3364  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4E58 size=132
    let mut pc: u32 = 0x82FF4E58;
    'dispatch: loop {
        match pc {
            0x82FF4E58 => {
    //   block [0x82FF4E58..0x82FF4EDC)
	// 82FF4E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4E5C: 481B330D  bl 0x831a8168
	ctx.lr = 0x82FF4E60;
	sub_831A8130(ctx, base);
	// 82FF4E60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4E64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4E68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF4E6C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF4E70: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FF4E74: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF4E78: 419A005C  beq cr6, 0x82ff4ed4
	if ctx.cr[6].eq {
	pc = 0x82FF4ED4; continue 'dispatch;
	}
	// 82FF4E7C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FF4E80: 40990054  ble cr6, 0x82ff4ed4
	if !ctx.cr[6].gt {
	pc = 0x82FF4ED4; continue 'dispatch;
	}
	// 82FF4E84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FF4E88: 419A004C  beq cr6, 0x82ff4ed4
	if ctx.cr[6].eq {
	pc = 0x82FF4ED4; continue 'dispatch;
	}
	// 82FF4E8C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF4E90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FF4E94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF4E98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF4E9C: 481BA665  bl 0x831af500
	ctx.lr = 0x82FF4EA0;
	sub_831AF500(ctx, base);
	// 82FF4EA0: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FF4EA4: 4098FFE8  bge cr6, 0x82ff4e8c
	if !ctx.cr[6].lt {
	pc = 0x82FF4E8C; continue 'dispatch;
	}
	// 82FF4EA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF4EAC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FF4EB0: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 82FF4EB4: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82FF4EB8: 38A00174  li r5, 0x174
	ctx.r[5].s64 = 372;
	// 82FF4EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4EC0: 4BFFF9C9  bl 0x82ff4888
	ctx.lr = 0x82FF4EC4;
	sub_82FF4888(ctx, base);
	// 82FF4EC4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF4EC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF4ECC: 388BC800  addi r4, r11, -0x3800
	ctx.r[4].s64 = ctx.r[11].s64 + -14336;
	// 82FF4ED0: 481BBD59  bl 0x831b0c28
	ctx.lr = 0x82FF4ED4;
	sub_831B0C28(ctx, base);
	// 82FF4ED4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF4ED8: 481B32E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4EE0 size=16
    let mut pc: u32 = 0x82FF4EE0;
    'dispatch: loop {
        match pc {
            0x82FF4EE0 => {
    //   block [0x82FF4EE0..0x82FF4EF0)
	// 82FF4EE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82FF4EE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF4EE8: 386B9BC9  addi r3, r11, -0x6437
	ctx.r[3].s64 = ctx.r[11].s64 + -25655;
	// 82FF4EEC: 4BFDC524  b 0x82fd1410
	sub_82FD1410(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4EF0 size=4
    let mut pc: u32 = 0x82FF4EF0;
    'dispatch: loop {
        match pc {
            0x82FF4EF0 => {
    //   block [0x82FF4EF0..0x82FF4EF4)
	// 82FF4EF0: 4B2CB378  b 0x822c0268
	sub_822C0268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4EF8 size=4
    let mut pc: u32 = 0x82FF4EF8;
    'dispatch: loop {
        match pc {
            0x82FF4EF8 => {
    //   block [0x82FF4EF8..0x82FF4EFC)
	// 82FF4EF8: 4824DA74  b 0x8324296c
	sub_8324296C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4F00 size=56
    let mut pc: u32 = 0x82FF4F00;
    'dispatch: loop {
        match pc {
            0x82FF4F00 => {
    //   block [0x82FF4F00..0x82FF4F38)
	// 82FF4F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4F10: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF4F14: 4B2CBA25  bl 0x822c0938
	ctx.lr = 0x82FF4F18;
	sub_822C0938(ctx, base);
	// 82FF4F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF4F1C: 4824DAC1  bl 0x832429dc
	ctx.lr = 0x82FF4F20;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 82FF4F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF4F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF4F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4F38 size=4
    let mut pc: u32 = 0x82FF4F38;
    'dispatch: loop {
        match pc {
            0x82FF4F38 => {
    //   block [0x82FF4F38..0x82FF4F3C)
	// 82FF4F38: 4824DA24  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4F40 size=36
    let mut pc: u32 = 0x82FF4F40;
    'dispatch: loop {
        match pc {
            0x82FF4F40 => {
    //   block [0x82FF4F40..0x82FF4F64)
	// 82FF4F40: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 82FF4F44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82FF4F48: 7D601828  lwarx r11, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82FF4F4C: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82FF4F50: 409A0014  bne cr6, 0x82ff4f64
	if !ctx.cr[6].eq {
		sub_82FF4F64(ctx, base);
		return;
	}
	// 82FF4F54: 7C80192D  stwcx. r4, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82FF4F58: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82FF4F5C: 4082FFE4  bne 0x82ff4f40
	if !ctx.cr[0].eq {
	pc = 0x82FF4F40; continue 'dispatch;
	}
	// 82FF4F60: 4800000C  b 0x82ff4f6c
	sub_82FF4F64(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4F64 size=16
    let mut pc: u32 = 0x82FF4F64;
    'dispatch: loop {
        match pc {
            0x82FF4F64 => {
    //   block [0x82FF4F64..0x82FF4F74)
	// 82FF4F64: 7D60192D  stwcx. r11, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82FF4F68: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82FF4F6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF4F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF4F78 size=8
    let mut pc: u32 = 0x82FF4F78;
    'dispatch: loop {
        match pc {
            0x82FF4F78 => {
    //   block [0x82FF4F78..0x82FF4F80)
	// 82FF4F78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF4F7C: 8213EC70  lwz r16, -0x1390(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-5008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4F80 size=88
    let mut pc: u32 = 0x82FF4F80;
    'dispatch: loop {
        match pc {
            0x82FF4F80 => {
    //   block [0x82FF4F80..0x82FF4FD8)
	// 82FF4F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4F88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF4F8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF4F90: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF4F94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4F98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF4F9C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FF4FA0: 4BFE32A9  bl 0x82fd8248
	ctx.lr = 0x82FF4FA4;
	sub_82FD8248(ctx, base);
	// 82FF4FA4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF4FA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF4FAC: 41820010  beq 0x82ff4fbc
	if ctx.cr[0].eq {
	pc = 0x82FF4FBC; continue 'dispatch;
	}
	// 82FF4FB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF4FB4: 4804109D  bl 0x83036050
	ctx.lr = 0x82FF4FB8;
	sub_83036050(ctx, base);
	// 82FF4FB8: 48000008  b 0x82ff4fc0
	pc = 0x82FF4FC0; continue 'dispatch;
	// 82FF4FBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF4FC0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF4FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4FCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF4FD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF4FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF4FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF4FD8 size=40
    let mut pc: u32 = 0x82FF4FD8;
    'dispatch: loop {
        match pc {
            0x82FF4FD8 => {
    //   block [0x82FF4FD8..0x82FF5000)
	// 82FF4FD8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF4FDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF4FE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF4FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF4FE8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF4FEC: 4BFE32F5  bl 0x82fd82e0
	ctx.lr = 0x82FF4FF0;
	sub_82FD82E0(ctx, base);
	// 82FF4FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF4FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF4FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF4FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5000 size=8
    let mut pc: u32 = 0x82FF5000;
    'dispatch: loop {
        match pc {
            0x82FF5000 => {
    //   block [0x82FF5000..0x82FF5008)
	// 82FF5000: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5004: 8213ECA8  lwz r16, -0x1358(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5008 size=72
    let mut pc: u32 = 0x82FF5008;
    'dispatch: loop {
        match pc {
            0x82FF5008 => {
    //   block [0x82FF5008..0x82FF5050)
	// 82FF5008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF500C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5014: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF5018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF501C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF5020: 4BFE3229  bl 0x82fd8248
	ctx.lr = 0x82FF5024;
	sub_82FD8248(ctx, base);
	// 82FF5024: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5028: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF502C: 4182000C  beq 0x82ff5038
	if ctx.cr[0].eq {
	pc = 0x82FF5038; continue 'dispatch;
	}
	// 82FF5030: 48041B39  bl 0x83036b68
	ctx.lr = 0x82FF5034;
	sub_83036B68(ctx, base);
	// 82FF5034: 48000008  b 0x82ff503c
	pc = 0x82FF503C; continue 'dispatch;
	// 82FF5038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF503C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF5040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF504C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5050 size=40
    let mut pc: u32 = 0x82FF5050;
    'dispatch: loop {
        match pc {
            0x82FF5050 => {
    //   block [0x82FF5050..0x82FF5078)
	// 82FF5050: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF5054: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5058: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF505C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5060: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5064: 4BFE327D  bl 0x82fd82e0
	ctx.lr = 0x82FF5068;
	sub_82FD82E0(ctx, base);
	// 82FF5068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF506C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5078 size=36
    let mut pc: u32 = 0x82FF5078;
    'dispatch: loop {
        match pc {
            0x82FF5078 => {
    //   block [0x82FF5078..0x82FF509C)
	// 82FF5078: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FF507C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FF5080: 419A00E4  beq cr6, 0x82ff5164
	if ctx.cr[6].eq {
		sub_82FF5164(ctx, base);
		return;
	}
	// 82FF5084: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5088: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF508C: 418200D8  beq 0x82ff5164
	if ctx.cr[0].eq {
		sub_82FF5164(ctx, base);
		return;
	}
	// 82FF5090: 39690002  addi r11, r9, 2
	ctx.r[11].s64 = ctx.r[9].s64 + 2;
	// 82FF5094: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82FF5098: 48000008  b 0x82ff50a0
	sub_82FF509C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF509C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF509C size=36
    let mut pc: u32 = 0x82FF509C;
    'dispatch: loop {
        match pc {
            0x82FF509C => {
    //   block [0x82FF509C..0x82FF50C0)
	// 82FF509C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF50A0: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF50A4: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF50A8: 4082FFF4  bne 0x82ff509c
	if !ctx.cr[0].eq {
	pc = 0x82FF509C; continue 'dispatch;
	}
	// 82FF50AC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82FF50B0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FF50B4: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF50B8: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82FF50BC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF50C0 size=152
    let mut pc: u32 = 0x82FF50C0;
    'dispatch: loop {
        match pc {
            0x82FF50C0 => {
    //   block [0x82FF50C0..0x82FF5158)
	// 82FF50C0: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF50C4: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF50C8: 419A0020  beq cr6, 0x82ff50e8
	if ctx.cr[6].eq {
	pc = 0x82FF50E8; continue 'dispatch;
	}
	// 82FF50CC: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF50D0: 419A0018  beq cr6, 0x82ff50e8
	if ctx.cr[6].eq {
	pc = 0x82FF50E8; continue 'dispatch;
	}
	// 82FF50D4: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF50D8: 419A0010  beq cr6, 0x82ff50e8
	if ctx.cr[6].eq {
	pc = 0x82FF50E8; continue 'dispatch;
	}
	// 82FF50DC: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF50E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF50E4: 409A0008  bne cr6, 0x82ff50ec
	if !ctx.cr[6].eq {
	pc = 0x82FF50EC; continue 'dispatch;
	}
	// 82FF50E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF50EC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF50F0: 41820050  beq 0x82ff5140
	if ctx.cr[0].eq {
	pc = 0x82FF5140; continue 'dispatch;
	}
	// 82FF50F4: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FF50F8: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FF50FC: 409A0044  bne cr6, 0x82ff5140
	if !ctx.cr[6].eq {
	pc = 0x82FF5140; continue 'dispatch;
	}
	// 82FF5100: A16A0004  lhz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5104: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FF5108: 409A0038  bne cr6, 0x82ff5140
	if !ctx.cr[6].eq {
	pc = 0x82FF5140; continue 'dispatch;
	}
	// 82FF510C: A16A0006  lhz r11, 6(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82FF5110: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF5114: 419A0020  beq cr6, 0x82ff5134
	if ctx.cr[6].eq {
	pc = 0x82FF5134; continue 'dispatch;
	}
	// 82FF5118: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF511C: 419A0018  beq cr6, 0x82ff5134
	if ctx.cr[6].eq {
	pc = 0x82FF5134; continue 'dispatch;
	}
	// 82FF5120: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF5124: 419A0010  beq cr6, 0x82ff5134
	if ctx.cr[6].eq {
	pc = 0x82FF5134; continue 'dispatch;
	}
	// 82FF5128: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF512C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5130: 409A0008  bne cr6, 0x82ff5138
	if !ctx.cr[6].eq {
	pc = 0x82FF5138; continue 'dispatch;
	}
	// 82FF5134: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF5138: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF513C: 4082001C  bne 0x82ff5158
	if !ctx.cr[0].eq {
		sub_82FF5158(ctx, base);
		return;
	}
	// 82FF5140: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF5144: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82FF5148: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF514C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5150: 4082FF68  bne 0x82ff50b8
	if !ctx.cr[0].eq {
		sub_82FF509C(ctx, base);
		return;
	}
	// 82FF5154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5158 size=12
    let mut pc: u32 = 0x82FF5158;
    'dispatch: loop {
        match pc {
            0x82FF5158 => {
    //   block [0x82FF5158..0x82FF5164)
	// 82FF5158: 7D695050  subf r11, r9, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82FF515C: 7D630E70  srawi r3, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF5160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5164 size=8
    let mut pc: u32 = 0x82FF5164;
    'dispatch: loop {
        match pc {
            0x82FF5164 => {
    //   block [0x82FF5164..0x82FF516C)
	// 82FF5164: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FF5168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5170 size=392
    let mut pc: u32 = 0x82FF5170;
    'dispatch: loop {
        match pc {
            0x82FF5170 => {
    //   block [0x82FF5170..0x82FF52F8)
	// 82FF5170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF517C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF518C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FF5190: 419A0150  beq cr6, 0x82ff52e0
	if ctx.cr[6].eq {
	pc = 0x82FF52E0; continue 'dispatch;
	}
	// 82FF5194: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5198: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF519C: 41820144  beq 0x82ff52e0
	if ctx.cr[0].eq {
	pc = 0x82FF52E0; continue 'dispatch;
	}
	// 82FF51A0: 4BFDB9E1  bl 0x82fd0b80
	ctx.lr = 0x82FF51A4;
	sub_82FD0B80(ctx, base);
	// 82FF51A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF51A8: 41820034  beq 0x82ff51dc
	if ctx.cr[0].eq {
	pc = 0x82FF51DC; continue 'dispatch;
	}
	// 82FF51AC: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF51B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF51B4: 41820028  beq 0x82ff51dc
	if ctx.cr[0].eq {
	pc = 0x82FF51DC; continue 'dispatch;
	}
	// 82FF51B8: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FF51BC: 48000008  b 0x82ff51c4
	pc = 0x82FF51C4; continue 'dispatch;
	// 82FF51C0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF51C4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF51C8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF51CC: 4082FFF4  bne 0x82ff51c0
	if !ctx.cr[0].eq {
	pc = 0x82FF51C0; continue 'dispatch;
	}
	// 82FF51D0: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FF51D4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF51D8: 48000008  b 0x82ff51e0
	pc = 0x82FF51E0; continue 'dispatch;
	// 82FF51DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF51E0: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82FF51E4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF51E8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82FF51EC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82FF51F0: 480000D0  b 0x82ff52c0
	pc = 0x82FF52C0; continue 'dispatch;
	// 82FF51F4: 2F070003  cmpwi cr6, r7, 3
	ctx.cr[6].compare_i32(ctx.r[7].s32, 3, &mut ctx.xer);
	// 82FF51F8: 41980094  blt cr6, 0x82ff528c
	if ctx.cr[6].lt {
	pc = 0x82FF528C; continue 'dispatch;
	}
	// 82FF51FC: 552B043E  clrlwi r11, r9, 0x10
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82FF5200: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF5204: 419A0020  beq cr6, 0x82ff5224
	if ctx.cr[6].eq {
	pc = 0x82FF5224; continue 'dispatch;
	}
	// 82FF5208: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF520C: 419A0018  beq cr6, 0x82ff5224
	if ctx.cr[6].eq {
	pc = 0x82FF5224; continue 'dispatch;
	}
	// 82FF5210: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF5214: 419A0010  beq cr6, 0x82ff5224
	if ctx.cr[6].eq {
	pc = 0x82FF5224; continue 'dispatch;
	}
	// 82FF5218: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF521C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5220: 409A0008  bne cr6, 0x82ff5228
	if !ctx.cr[6].eq {
	pc = 0x82FF5228; continue 'dispatch;
	}
	// 82FF5224: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF5228: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF522C: 41820054  beq 0x82ff5280
	if ctx.cr[0].eq {
	pc = 0x82FF5280; continue 'dispatch;
	}
	// 82FF5230: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FF5234: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FF5238: 409A0048  bne cr6, 0x82ff5280
	if !ctx.cr[6].eq {
	pc = 0x82FF5280; continue 'dispatch;
	}
	// 82FF523C: 39030004  addi r8, r3, 4
	ctx.r[8].s64 = ctx.r[3].s64 + 4;
	// 82FF5240: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5244: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF5248: 419A0020  beq cr6, 0x82ff5268
	if ctx.cr[6].eq {
	pc = 0x82FF5268; continue 'dispatch;
	}
	// 82FF524C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF5250: 419A0018  beq cr6, 0x82ff5268
	if ctx.cr[6].eq {
	pc = 0x82FF5268; continue 'dispatch;
	}
	// 82FF5254: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF5258: 419A0010  beq cr6, 0x82ff5268
	if ctx.cr[6].eq {
	pc = 0x82FF5268; continue 'dispatch;
	}
	// 82FF525C: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF5260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5264: 409A0008  bne cr6, 0x82ff526c
	if !ctx.cr[6].eq {
	pc = 0x82FF526C; continue 'dispatch;
	}
	// 82FF5268: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF526C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF5270: 41820010  beq 0x82ff5280
	if ctx.cr[0].eq {
	pc = 0x82FF5280; continue 'dispatch;
	}
	// 82FF5274: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82FF5278: 38E7FFFE  addi r7, r7, -2
	ctx.r[7].s64 = ctx.r[7].s64 + -2;
	// 82FF527C: 48000044  b 0x82ff52c0
	pc = 0x82FF52C0; continue 'dispatch;
	// 82FF5280: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 82FF5284: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82FF5288: 48000030  b 0x82ff52b8
	pc = 0x82FF52B8; continue 'dispatch;
	// 82FF528C: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 82FF5290: 409A000C  bne cr6, 0x82ff529c
	if !ctx.cr[6].eq {
	pc = 0x82FF529C; continue 'dispatch;
	}
	// 82FF5294: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 82FF5298: 48000020  b 0x82ff52b8
	pc = 0x82FF52B8; continue 'dispatch;
	// 82FF529C: 2F070002  cmpwi cr6, r7, 2
	ctx.cr[6].compare_i32(ctx.r[7].s32, 2, &mut ctx.xer);
	// 82FF52A0: 409A0020  bne cr6, 0x82ff52c0
	if !ctx.cr[6].eq {
	pc = 0x82FF52C0; continue 'dispatch;
	}
	// 82FF52A4: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FF52A8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82FF52AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF52B0: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 82FF52B4: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF52B8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82FF52BC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FF52C0: A1230000  lhz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF52C4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF52C8: 4082FF2C  bne 0x82ff51f4
	if !ctx.cr[0].eq {
	pc = 0x82FF51F4; continue 'dispatch;
	}
	// 82FF52CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF52D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF52D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF52D8: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FF52DC: 4BFDD7E5  bl 0x82fd2ac0
	ctx.lr = 0x82FF52E0;
	sub_82FD2AC0(ctx, base);
	// 82FF52E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF52E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF52E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF52EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF52F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF52F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF52F8 size=8
    let mut pc: u32 = 0x82FF52F8;
    'dispatch: loop {
        match pc {
            0x82FF52F8 => {
    //   block [0x82FF52F8..0x82FF5300)
	// 82FF52F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF52FC: 8213ECE8  lwz r16, -0x1318(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5300 size=576
    let mut pc: u32 = 0x82FF5300;
    'dispatch: loop {
        match pc {
            0x82FF5300 => {
    //   block [0x82FF5300..0x82FF5540)
	// 82FF5300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5304: 481B2E51  bl 0x831a8154
	ctx.lr = 0x82FF5308;
	sub_831A8130(ctx, base);
	// 82FF5308: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FF530C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5310: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF5314: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FF5318: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF531C: 419A0034  beq cr6, 0x82ff5350
	if ctx.cr[6].eq {
	pc = 0x82FF5350; continue 'dispatch;
	}
	// 82FF5320: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5324: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5328: 41820028  beq 0x82ff5350
	if ctx.cr[0].eq {
	pc = 0x82FF5350; continue 'dispatch;
	}
	// 82FF532C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FF5330: 48000008  b 0x82ff5338
	pc = 0x82FF5338; continue 'dispatch;
	// 82FF5334: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF5338: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF533C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5340: 4082FFF4  bne 0x82ff5334
	if !ctx.cr[0].eq {
	pc = 0x82FF5334; continue 'dispatch;
	}
	// 82FF5344: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FF5348: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF534C: 48000008  b 0x82ff5354
	pc = 0x82FF5354; continue 'dispatch;
	// 82FF5350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5354: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5358: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF535C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF5360: 557C083C  slwi r28, r11, 1
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FF5364: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF5368: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF536C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF5370: 4E800421  bctrl
	ctx.lr = 0x82FF5374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF5374: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FF5378: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FF537C: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82FF5380: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5384: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF5388: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF538C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF5394: 4E800421  bctrl
	ctx.lr = 0x82FF5398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF5398: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FF539C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FF53A0: 931F0050  stw r24, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82FF53A4: 3AFE0002  addi r23, r30, 2
	ctx.r[23].s64 = ctx.r[30].s64 + 2;
	// 82FF53A8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FF53AC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FF53B0: 4BFFFCC9  bl 0x82ff5078
	ctx.lr = 0x82FF53B4;
	sub_82FF5078(ctx, base);
	// 82FF53B4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FF53B8: 419A0168  beq cr6, 0x82ff5520
	if ctx.cr[6].eq {
	pc = 0x82FF5520; continue 'dispatch;
	}
	// 82FF53BC: 7F3A1A14  add r25, r26, r3
	ctx.r[25].u64 = ctx.r[26].u64 + ctx.r[3].u64;
	// 82FF53C0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FF53C4: 3B99FFFF  addi r28, r25, -1
	ctx.r[28].s64 = ctx.r[25].s64 + -1;
	// 82FF53C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF53CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF53D0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF53D4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FF53D8: 4BFDCCB1  bl 0x82fd2088
	ctx.lr = 0x82FF53DC;
	sub_82FD2088(ctx, base);
	// 82FF53DC: 2C1C0000  cmpwi r28, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FF53E0: 41800128  blt 0x82ff5508
	if ctx.cr[0].lt {
	pc = 0x82FF5508; continue 'dispatch;
	}
	// 82FF53E4: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF53E8: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82FF53EC: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF53F0: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82FF53F4: 419A0020  beq cr6, 0x82ff5414
	if ctx.cr[6].eq {
	pc = 0x82FF5414; continue 'dispatch;
	}
	// 82FF53F8: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FF53FC: 419A0018  beq cr6, 0x82ff5414
	if ctx.cr[6].eq {
	pc = 0x82FF5414; continue 'dispatch;
	}
	// 82FF5400: 2B0B00A5  cmplwi cr6, r11, 0xa5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 165 as u32, &mut ctx.xer);
	// 82FF5404: 419A0010  beq cr6, 0x82ff5414
	if ctx.cr[6].eq {
	pc = 0x82FF5414; continue 'dispatch;
	}
	// 82FF5408: 2B0B20A9  cmplwi cr6, r11, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8361 as u32, &mut ctx.xer);
	// 82FF540C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5410: 409A0008  bne cr6, 0x82ff5418
	if !ctx.cr[6].eq {
	pc = 0x82FF5418; continue 'dispatch;
	}
	// 82FF5414: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF5418: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF541C: 40820010  bne 0x82ff542c
	if !ctx.cr[0].eq {
	pc = 0x82FF542C; continue 'dispatch;
	}
	// 82FF5420: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FF5424: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82FF5428: 4080FFC4  bge 0x82ff53ec
	if !ctx.cr[0].lt {
	pc = 0x82FF53EC; continue 'dispatch;
	}
	// 82FF542C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FF5430: 419800D8  blt cr6, 0x82ff5508
	if ctx.cr[6].lt {
	pc = 0x82FF5508; continue 'dispatch;
	}
	// 82FF5434: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82FF5438: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF543C: 7D6BF22E  lhzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FF5440: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FF5444: 409A0024  bne cr6, 0x82ff5468
	if !ctx.cr[6].eq {
	pc = 0x82FF5468; continue 'dispatch;
	}
	// 82FF5448: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FF544C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF5450: 7D6BF22E  lhzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FF5454: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FF5458: 409A0010  bne cr6, 0x82ff5468
	if !ctx.cr[6].eq {
	pc = 0x82FF5468; continue 'dispatch;
	}
	// 82FF545C: 397C0003  addi r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 + 3;
	// 82FF5460: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FF5464: 419A00A4  beq cr6, 0x82ff5508
	if ctx.cr[6].eq {
	pc = 0x82FF5508; continue 'dispatch;
	}
	// 82FF5468: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FF546C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FF5470: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF5474: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5478: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF547C: 4BFDCC0D  bl 0x82fd2088
	ctx.lr = 0x82FF5480;
	sub_82FD2088(ctx, base);
	// 82FF5480: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF5484: 419A0034  beq cr6, 0x82ff54b8
	if ctx.cr[6].eq {
	pc = 0x82FF54B8; continue 'dispatch;
	}
	// 82FF5488: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF548C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5490: 41820028  beq 0x82ff54b8
	if ctx.cr[0].eq {
	pc = 0x82FF54B8; continue 'dispatch;
	}
	// 82FF5494: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82FF5498: 48000008  b 0x82ff54a0
	pc = 0x82FF54A0; continue 'dispatch;
	// 82FF549C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF54A0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF54A4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF54A8: 4082FFF4  bne 0x82ff549c
	if !ctx.cr[0].eq {
	pc = 0x82FF549C; continue 'dispatch;
	}
	// 82FF54AC: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FF54B0: 7D660E70  srawi r6, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF54B4: 48000008  b 0x82ff54bc
	pc = 0x82FF54BC; continue 'dispatch;
	// 82FF54B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FF54BC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FF54C0: 38B90003  addi r5, r25, 3
	ctx.r[5].s64 = ctx.r[25].s64 + 3;
	// 82FF54C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF54C8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FF54CC: 4BFDCBBD  bl 0x82fd2088
	ctx.lr = 0x82FF54D0;
	sub_82FD2088(ctx, base);
	// 82FF54D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF54D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF54D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF54DC: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FF54E0: 4BFDC3B9  bl 0x82fd1898
	ctx.lr = 0x82FF54E4;
	sub_82FD1898(ctx, base);
	// 82FF54E4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FF54E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF54EC: 4BFDC3AD  bl 0x82fd1898
	ctx.lr = 0x82FF54F0;
	sub_82FD1898(ctx, base);
	// 82FF54F0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FF54F4: 409A000C  bne cr6, 0x82ff5500
	if !ctx.cr[6].eq {
	pc = 0x82FF5500; continue 'dispatch;
	}
	// 82FF54F8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FF54FC: 48000010  b 0x82ff550c
	pc = 0x82FF550C; continue 'dispatch;
	// 82FF5500: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82FF5504: 48000008  b 0x82ff550c
	pc = 0x82FF550C; continue 'dispatch;
	// 82FF5508: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82FF550C: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF5510: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF5514: 4BFFFB65  bl 0x82ff5078
	ctx.lr = 0x82FF5518;
	sub_82FF5078(ctx, base);
	// 82FF5518: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FF551C: 409AFEA0  bne cr6, 0x82ff53bc
	if !ctx.cr[6].eq {
	pc = 0x82FF53BC; continue 'dispatch;
	}
	// 82FF5520: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF5524: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FF5528: 4BFDD599  bl 0x82fd2ac0
	ctx.lr = 0x82FF552C;
	sub_82FD2AC0(ctx, base);
	// 82FF552C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF5530: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FF5534: 4BFDD58D  bl 0x82fd2ac0
	ctx.lr = 0x82FF5538;
	sub_82FD2AC0(ctx, base);
	// 82FF5538: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FF553C: 481B2C68  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5540 size=40
    let mut pc: u32 = 0x82FF5540;
    'dispatch: loop {
        match pc {
            0x82FF5540 => {
    //   block [0x82FF5540..0x82FF5568)
	// 82FF5540: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FF5544: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5548: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF554C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5550: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FF5554: 4BFDD905  bl 0x82fd2e58
	ctx.lr = 0x82FF5558;
	sub_82FD2E58(ctx, base);
	// 82FF5558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5568 size=40
    let mut pc: u32 = 0x82FF5568;
    'dispatch: loop {
        match pc {
            0x82FF5568 => {
    //   block [0x82FF5568..0x82FF5590)
	// 82FF5568: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FF556C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5570: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5578: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FF557C: 4BFDD8DD  bl 0x82fd2e58
	ctx.lr = 0x82FF5580;
	sub_82FD2E58(ctx, base);
	// 82FF5580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF558C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5590 size=440
    let mut pc: u32 = 0x82FF5590;
    'dispatch: loop {
        match pc {
            0x82FF5590 => {
    //   block [0x82FF5590..0x82FF5748)
	// 82FF5590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5594: 481B2BCD  bl 0x831a8160
	ctx.lr = 0x82FF5598;
	sub_831A8130(ctx, base);
	// 82FF5598: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF559C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF55A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FF55A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF55A8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FF55AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF55B0: 419A0034  beq cr6, 0x82ff55e4
	if ctx.cr[6].eq {
	pc = 0x82FF55E4; continue 'dispatch;
	}
	// 82FF55B4: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF55B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF55BC: 41820028  beq 0x82ff55e4
	if ctx.cr[0].eq {
	pc = 0x82FF55E4; continue 'dispatch;
	}
	// 82FF55C0: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FF55C4: 48000008  b 0x82ff55cc
	pc = 0x82FF55CC; continue 'dispatch;
	// 82FF55C8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF55CC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF55D0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF55D4: 4082FFF4  bne 0x82ff55c8
	if !ctx.cr[0].eq {
	pc = 0x82FF55C8; continue 'dispatch;
	}
	// 82FF55D8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FF55DC: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF55E0: 48000008  b 0x82ff55e8
	pc = 0x82FF55E8; continue 'dispatch;
	// 82FF55E4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FF55E8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FF55EC: 419A0034  beq cr6, 0x82ff5620
	if ctx.cr[6].eq {
	pc = 0x82FF5620; continue 'dispatch;
	}
	// 82FF55F0: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF55F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF55F8: 41820028  beq 0x82ff5620
	if ctx.cr[0].eq {
	pc = 0x82FF5620; continue 'dispatch;
	}
	// 82FF55FC: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FF5600: 48000008  b 0x82ff5608
	pc = 0x82FF5608; continue 'dispatch;
	// 82FF5604: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF5608: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF560C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5610: 4082FFF4  bne 0x82ff5604
	if !ctx.cr[0].eq {
	pc = 0x82FF5604; continue 'dispatch;
	}
	// 82FF5614: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FF5618: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF561C: 48000008  b 0x82ff5624
	pc = 0x82FF5624; continue 'dispatch;
	// 82FF5620: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FF5624: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FF5628: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF562C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF5630: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF5634: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF5638: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF563C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF5640: 4E800421  bctrl
	ctx.lr = 0x82FF5644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF5644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5648: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF564C: B35F0000  sth r26, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 82FF5650: 419A00E0  beq cr6, 0x82ff5730
	if ctx.cr[6].eq {
	pc = 0x82FF5730; continue 'dispatch;
	}
	// 82FF5654: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5658: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF565C: 418200D4  beq 0x82ff5730
	if ctx.cr[0].eq {
	pc = 0x82FF5730; continue 'dispatch;
	}
	// 82FF5660: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FF5664: 48000008  b 0x82ff566c
	pc = 0x82FF566C; continue 'dispatch;
	// 82FF5668: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF566C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5670: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5674: 4082FFF4  bne 0x82ff5668
	if !ctx.cr[0].eq {
	pc = 0x82FF5668; continue 'dispatch;
	}
	// 82FF5678: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FF567C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF5680: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF5684: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF5688: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82FF568C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF5690: 419800A0  blt cr6, 0x82ff5730
	if ctx.cr[6].lt {
	pc = 0x82FF5730; continue 'dispatch;
	}
	// 82FF5694: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5698: 2B0A005C  cmplwi cr6, r10, 0x5c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 92 as u32, &mut ctx.xer);
	// 82FF569C: 419A0020  beq cr6, 0x82ff56bc
	if ctx.cr[6].eq {
	pc = 0x82FF56BC; continue 'dispatch;
	}
	// 82FF56A0: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82FF56A4: 419A0018  beq cr6, 0x82ff56bc
	if ctx.cr[6].eq {
	pc = 0x82FF56BC; continue 'dispatch;
	}
	// 82FF56A8: 2B0A00A5  cmplwi cr6, r10, 0xa5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 165 as u32, &mut ctx.xer);
	// 82FF56AC: 419A0010  beq cr6, 0x82ff56bc
	if ctx.cr[6].eq {
	pc = 0x82FF56BC; continue 'dispatch;
	}
	// 82FF56B0: 2B0A20A9  cmplwi cr6, r10, 0x20a9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8361 as u32, &mut ctx.xer);
	// 82FF56B4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FF56B8: 409A0008  bne cr6, 0x82ff56c0
	if !ctx.cr[6].eq {
	pc = 0x82FF56C0; continue 'dispatch;
	}
	// 82FF56BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FF56C0: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF56C4: 40820010  bne 0x82ff56d4
	if !ctx.cr[0].eq {
	pc = 0x82FF56D4; continue 'dispatch;
	}
	// 82FF56C8: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82FF56CC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF56D0: 4098FFC4  bge cr6, 0x82ff5694
	if !ctx.cr[6].lt {
	pc = 0x82FF5694; continue 'dispatch;
	}
	// 82FF56D4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FF56D8: 41980058  blt cr6, 0x82ff5730
	if ctx.cr[6].lt {
	pc = 0x82FF5730; continue 'dispatch;
	}
	// 82FF56DC: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FF56E0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FF56E4: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF56E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF56EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF56F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF56F4: 38DD0001  addi r6, r29, 1
	ctx.r[6].s64 = ctx.r[29].s64 + 1;
	// 82FF56F8: 4BFDC991  bl 0x82fd2088
	ctx.lr = 0x82FF56FC;
	sub_82FD2088(ctx, base);
	// 82FF56FC: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82FF5700: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF5704: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF5708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF570C: 7F4BFB2E  sthx r26, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[26].u16) };
	// 82FF5710: 4BFDC189  bl 0x82fd1898
	ctx.lr = 0x82FF5714;
	sub_82FD1898(ctx, base);
	// 82FF5714: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF5718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF571C: 4BFFFA55  bl 0x82ff5170
	ctx.lr = 0x82FF5720;
	sub_82FF5170(ctx, base);
	// 82FF5720: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF5724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5728: 4BFFFBD9  bl 0x82ff5300
	ctx.lr = 0x82FF572C;
	sub_82FF5300(ctx, base);
	// 82FF572C: 48000010  b 0x82ff573c
	pc = 0x82FF573C; continue 'dispatch;
	// 82FF5730: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF5734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5738: 4BFDC431  bl 0x82fd1b68
	ctx.lr = 0x82FF573C;
	sub_82FD1B68(ctx, base);
	// 82FF573C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5740: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF5744: 481B2A6C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5748 size=64
    let mut pc: u32 = 0x82FF5748;
    'dispatch: loop {
        match pc {
            0x82FF5748 => {
    //   block [0x82FF5748..0x82FF5788)
	// 82FF5748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF575C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF5760: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5764: 4BFFF79D  bl 0x82ff4f00
	ctx.lr = 0x82FF5768;
	sub_82FF4F00(ctx, base);
	// 82FF5768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF576C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5770: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF577C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF5784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5788 size=64
    let mut pc: u32 = 0x82FF5788;
    'dispatch: loop {
        match pc {
            0x82FF5788 => {
    //   block [0x82FF5788..0x82FF57C8)
	// 82FF5788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF578C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF579C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF57A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF57A4: 41820010  beq 0x82ff57b4
	if ctx.cr[0].eq {
	pc = 0x82FF57B4; continue 'dispatch;
	}
	// 82FF57A8: 4BFFF749  bl 0x82ff4ef0
	ctx.lr = 0x82FF57AC;
	sub_82FF4EF0(ctx, base);
	// 82FF57AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF57B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF57B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF57B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF57BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF57C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF57C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF57C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF57C8 size=8
    let mut pc: u32 = 0x82FF57C8;
    'dispatch: loop {
        match pc {
            0x82FF57C8 => {
    //   block [0x82FF57C8..0x82FF57D0)
	// 82FF57C8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF57CC: 4BFFF72C  b 0x82ff4ef8
	sub_82FF4EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF57D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF57D0 size=8
    let mut pc: u32 = 0x82FF57D0;
    'dispatch: loop {
        match pc {
            0x82FF57D0 => {
    //   block [0x82FF57D0..0x82FF57D8)
	// 82FF57D0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF57D4: 4BFFF764  b 0x82ff4f38
	sub_82FF4F38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF57D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF57D8 size=56
    let mut pc: u32 = 0x82FF57D8;
    'dispatch: loop {
        match pc {
            0x82FF57D8 => {
    //   block [0x82FF57D8..0x82FF5810)
	// 82FF57D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF57DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF57E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF57E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF57E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF57EC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FF57F0: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF57F4: 4BFFF705  bl 0x82ff4ef8
	ctx.lr = 0x82FF57F8;
	sub_82FF4EF8(ctx, base);
	// 82FF57F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF57FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF580C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5810 size=12
    let mut pc: u32 = 0x82FF5810;
    'dispatch: loop {
        match pc {
            0x82FF5810 => {
    //   block [0x82FF5810..0x82FF581C)
	// 82FF5810: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5814: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5818: 4BFFF720  b 0x82ff4f38
	sub_82FF4F38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5820 size=80
    let mut pc: u32 = 0x82FF5820;
    'dispatch: loop {
        match pc {
            0x82FF5820 => {
    //   block [0x82FF5820..0x82FF5870)
	// 82FF5820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF582C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5830: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF5834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5838: 396BED58  addi r11, r11, -0x12a8
	ctx.r[11].s64 = ctx.r[11].s64 + -4776;
	// 82FF583C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF5840: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5844: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF5848: 808BB7E8  lwz r4, -0x4818(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF584C: 4BFDB335  bl 0x82fd0b80
	ctx.lr = 0x82FF5850;
	sub_82FD0B80(ctx, base);
	// 82FF5850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF5854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5858: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FF585C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF586C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5870 size=80
    let mut pc: u32 = 0x82FF5870;
    'dispatch: loop {
        match pc {
            0x82FF5870 => {
    //   block [0x82FF5870..0x82FF58C0)
	// 82FF5870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF587C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5880: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FF5884: 807FB8FC  lwz r3, -0x4704(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF5888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FF588C: 419A0018  beq cr6, 0x82ff58a4
	if ctx.cr[6].eq {
	pc = 0x82FF58A4; continue 'dispatch;
	}
	// 82FF5890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF5894: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF5898: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF589C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF58A0: 4E800421  bctrl
	ctx.lr = 0x82FF58A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF58A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF58A8: 917FB8FC  stw r11, -0x4704(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18180 as u32), ctx.r[11].u32 ) };
	// 82FF58AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF58B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF58B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF58B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF58BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF58C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF58C0 size=16
    let mut pc: u32 = 0x82FF58C0;
    'dispatch: loop {
        match pc {
            0x82FF58C0 => {
    //   block [0x82FF58C0..0x82FF58D0)
	// 82FF58C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF58C4: 396BED60  addi r11, r11, -0x12a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4768;
	// 82FF58C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF58CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF58D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF58D0 size=40
    let mut pc: u32 = 0x82FF58D0;
    'dispatch: loop {
        match pc {
            0x82FF58D0 => {
    //   block [0x82FF58D0..0x82FF58F8)
	// 82FF58D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF58D4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF58D8: 394AED8C  addi r10, r10, -0x1274
	ctx.r[10].s64 = ctx.r[10].s64 + -4724;
	// 82FF58DC: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF58E0: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF58E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF58E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF58EC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF58F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF58F4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF58F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF58F8 size=108
    let mut pc: u32 = 0x82FF58F8;
    'dispatch: loop {
        match pc {
            0x82FF58F8 => {
    //   block [0x82FF58F8..0x82FF5964)
	// 82FF58F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF58FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF5904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF590C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5910: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF5914: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF5918: 396BED8C  addi r11, r11, -0x1274
	ctx.r[11].s64 = ctx.r[11].s64 + -4724;
	// 82FF591C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF5920: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF5924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF592C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF5930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF5934: 4E800421  bctrl
	ctx.lr = 0x82FF5938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF5938: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF593C: 4182000C  beq 0x82ff5948
	if ctx.cr[0].eq {
	pc = 0x82FF5948; continue 'dispatch;
	}
	// 82FF5940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5944: 4BFE299D  bl 0x82fd82e0
	ctx.lr = 0x82FF5948;
	sub_82FD82E0(ctx, base);
	// 82FF5948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF594C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF5950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5958: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF595C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF5960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5968 size=92
    let mut pc: u32 = 0x82FF5968;
    'dispatch: loop {
        match pc {
            0x82FF5968 => {
    //   block [0x82FF5968..0x82FF59C4)
	// 82FF5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5978: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF597C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5980: 396BED8C  addi r11, r11, -0x1274
	ctx.r[11].s64 = ctx.r[11].s64 + -4724;
	// 82FF5984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF5988: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF598C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FF5990: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FF5994: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5998: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82FF599C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FF59A0: 4BFDB1E1  bl 0x82fd0b80
	ctx.lr = 0x82FF59A4;
	sub_82FD0B80(ctx, base);
	// 82FF59A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF59A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF59AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF59B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF59B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF59B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF59BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF59C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF59C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF59C8 size=16
    let mut pc: u32 = 0x82FF59C8;
    'dispatch: loop {
        match pc {
            0x82FF59C8 => {
    //   block [0x82FF59C8..0x82FF59D8)
	// 82FF59C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF59CC: 396BED9C  addi r11, r11, -0x1264
	ctx.r[11].s64 = ctx.r[11].s64 + -4708;
	// 82FF59D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF59D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF59D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF59D8 size=8
    let mut pc: u32 = 0x82FF59D8;
    'dispatch: loop {
        match pc {
            0x82FF59D8 => {
    //   block [0x82FF59D8..0x82FF59E0)
	// 82FF59D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF59DC: 8213EDD0  lwz r16, -0x1230(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF59E0 size=88
    let mut pc: u32 = 0x82FF59E0;
    'dispatch: loop {
        match pc {
            0x82FF59E0 => {
    //   block [0x82FF59E0..0x82FF5A38)
	// 82FF59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF59E4: 481B2785  bl 0x831a8168
	ctx.lr = 0x82FF59E8;
	sub_831A8130(ctx, base);
	// 82FF59E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF59EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF59F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF59F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF59F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF59FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5A00: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FF5A04: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5A08: 4BFE2891  bl 0x82fd8298
	ctx.lr = 0x82FF5A0C;
	sub_82FD8298(ctx, base);
	// 82FF5A0C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5A10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5A14: 41820018  beq 0x82ff5a2c
	if ctx.cr[0].eq {
	pc = 0x82FF5A2C; continue 'dispatch;
	}
	// 82FF5A18: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5A1C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5A20: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5A24: 4804134D  bl 0x83036d70
	ctx.lr = 0x82FF5A28;
	sub_83036D70(ctx, base);
	// 82FF5A28: 48000008  b 0x82ff5a30
	pc = 0x82FF5A30; continue 'dispatch;
	// 82FF5A2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5A30: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5A34: 481B2784  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5A38 size=44
    let mut pc: u32 = 0x82FF5A38;
    'dispatch: loop {
        match pc {
            0x82FF5A38 => {
    //   block [0x82FF5A38..0x82FF5A64)
	// 82FF5A38: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5A3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5A40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5A48: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5A4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5A50: 4BFE2891  bl 0x82fd82e0
	ctx.lr = 0x82FF5A54;
	sub_82FD82E0(ctx, base);
	// 82FF5A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5A68 size=8
    let mut pc: u32 = 0x82FF5A68;
    'dispatch: loop {
        match pc {
            0x82FF5A68 => {
    //   block [0x82FF5A68..0x82FF5A70)
	// 82FF5A68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5A6C: 8213EE10  lwz r16, -0x11f0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5A70 size=88
    let mut pc: u32 = 0x82FF5A70;
    'dispatch: loop {
        match pc {
            0x82FF5A70 => {
    //   block [0x82FF5A70..0x82FF5AC8)
	// 82FF5A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5A74: 481B26F5  bl 0x831a8168
	ctx.lr = 0x82FF5A78;
	sub_831A8130(ctx, base);
	// 82FF5A78: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5A7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5A80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5A84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5A88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5A8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5A90: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FF5A94: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5A98: 4BFE2801  bl 0x82fd8298
	ctx.lr = 0x82FF5A9C;
	sub_82FD8298(ctx, base);
	// 82FF5A9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5AA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5AA4: 41820018  beq 0x82ff5abc
	if ctx.cr[0].eq {
	pc = 0x82FF5ABC; continue 'dispatch;
	}
	// 82FF5AA8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5AAC: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5AB0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5AB4: 480413ED  bl 0x83036ea0
	ctx.lr = 0x82FF5AB8;
	sub_83036EA0(ctx, base);
	// 82FF5AB8: 48000008  b 0x82ff5ac0
	pc = 0x82FF5AC0; continue 'dispatch;
	// 82FF5ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5AC0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5AC4: 481B26F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5AC8 size=44
    let mut pc: u32 = 0x82FF5AC8;
    'dispatch: loop {
        match pc {
            0x82FF5AC8 => {
    //   block [0x82FF5AC8..0x82FF5AF4)
	// 82FF5AC8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5ACC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5AD0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5AD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5AD8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5ADC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5AE0: 4BFE2801  bl 0x82fd82e0
	ctx.lr = 0x82FF5AE4;
	sub_82FD82E0(ctx, base);
	// 82FF5AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5AF8 size=8
    let mut pc: u32 = 0x82FF5AF8;
    'dispatch: loop {
        match pc {
            0x82FF5AF8 => {
    //   block [0x82FF5AF8..0x82FF5B00)
	// 82FF5AF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5AFC: 8213EE50  lwz r16, -0x11b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5B00 size=88
    let mut pc: u32 = 0x82FF5B00;
    'dispatch: loop {
        match pc {
            0x82FF5B00 => {
    //   block [0x82FF5B00..0x82FF5B58)
	// 82FF5B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5B04: 481B2665  bl 0x831a8168
	ctx.lr = 0x82FF5B08;
	sub_831A8130(ctx, base);
	// 82FF5B08: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5B0C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5B10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5B14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5B18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5B1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5B20: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FF5B24: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5B28: 4BFE2771  bl 0x82fd8298
	ctx.lr = 0x82FF5B2C;
	sub_82FD8298(ctx, base);
	// 82FF5B2C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5B30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5B34: 41820018  beq 0x82ff5b4c
	if ctx.cr[0].eq {
	pc = 0x82FF5B4C; continue 'dispatch;
	}
	// 82FF5B38: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5B3C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5B40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5B44: 4804185D  bl 0x830373a0
	ctx.lr = 0x82FF5B48;
	sub_830373A0(ctx, base);
	// 82FF5B48: 48000008  b 0x82ff5b50
	pc = 0x82FF5B50; continue 'dispatch;
	// 82FF5B4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5B50: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5B54: 481B2664  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5B58 size=44
    let mut pc: u32 = 0x82FF5B58;
    'dispatch: loop {
        match pc {
            0x82FF5B58 => {
    //   block [0x82FF5B58..0x82FF5B84)
	// 82FF5B58: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5B5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5B60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5B68: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5B6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5B70: 4BFE2771  bl 0x82fd82e0
	ctx.lr = 0x82FF5B74;
	sub_82FD82E0(ctx, base);
	// 82FF5B74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5B88 size=8
    let mut pc: u32 = 0x82FF5B88;
    'dispatch: loop {
        match pc {
            0x82FF5B88 => {
    //   block [0x82FF5B88..0x82FF5B90)
	// 82FF5B88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5B8C: 8213EE90  lwz r16, -0x1170(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5B90 size=88
    let mut pc: u32 = 0x82FF5B90;
    'dispatch: loop {
        match pc {
            0x82FF5B90 => {
    //   block [0x82FF5B90..0x82FF5BE8)
	// 82FF5B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5B94: 481B25D5  bl 0x831a8168
	ctx.lr = 0x82FF5B98;
	sub_831A8130(ctx, base);
	// 82FF5B98: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5B9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5BA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5BA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5BA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5BAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5BB0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FF5BB4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5BB8: 4BFE26E1  bl 0x82fd8298
	ctx.lr = 0x82FF5BBC;
	sub_82FD8298(ctx, base);
	// 82FF5BBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5BC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5BC4: 41820018  beq 0x82ff5bdc
	if ctx.cr[0].eq {
	pc = 0x82FF5BDC; continue 'dispatch;
	}
	// 82FF5BC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5BCC: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5BD0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5BD4: 48041F75  bl 0x83037b48
	ctx.lr = 0x82FF5BD8;
	sub_83037B48(ctx, base);
	// 82FF5BD8: 48000008  b 0x82ff5be0
	pc = 0x82FF5BE0; continue 'dispatch;
	// 82FF5BDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5BE0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5BE4: 481B25D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5BE8 size=44
    let mut pc: u32 = 0x82FF5BE8;
    'dispatch: loop {
        match pc {
            0x82FF5BE8 => {
    //   block [0x82FF5BE8..0x82FF5C14)
	// 82FF5BE8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5BEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5BF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5BF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5BF8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5BFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5C00: 4BFE26E1  bl 0x82fd82e0
	ctx.lr = 0x82FF5C04;
	sub_82FD82E0(ctx, base);
	// 82FF5C04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5C18 size=76
    let mut pc: u32 = 0x82FF5C18;
    'dispatch: loop {
        match pc {
            0x82FF5C18 => {
    //   block [0x82FF5C18..0x82FF5C64)
	// 82FF5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5C20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF5C24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5C28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5C30: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5C34: 4BFFFBED  bl 0x82ff5820
	ctx.lr = 0x82FF5C38;
	sub_82FF5820(ctx, base);
	// 82FF5C38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF5C3C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82FF5C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5C44: 396BEEC0  addi r11, r11, -0x1140
	ctx.r[11].s64 = ctx.r[11].s64 + -4416;
	// 82FF5C48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF5C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF5C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF5C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5C68 size=8
    let mut pc: u32 = 0x82FF5C68;
    'dispatch: loop {
        match pc {
            0x82FF5C68 => {
    //   block [0x82FF5C68..0x82FF5C70)
	// 82FF5C68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5C6C: 8213EED0  lwz r16, -0x1130(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4400 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5C70 size=92
    let mut pc: u32 = 0x82FF5C70;
    'dispatch: loop {
        match pc {
            0x82FF5C70 => {
    //   block [0x82FF5C70..0x82FF5CCC)
	// 82FF5C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5C74: 481B24F5  bl 0x831a8168
	ctx.lr = 0x82FF5C78;
	sub_831A8130(ctx, base);
	// 82FF5C78: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5C7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5C80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF5C84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF5C88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5C8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF5C90: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FF5C94: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82FF5C98: 4BFE2601  bl 0x82fd8298
	ctx.lr = 0x82FF5C9C;
	sub_82FD8298(ctx, base);
	// 82FF5C9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5CA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5CA4: 4182001C  beq 0x82ff5cc0
	if ctx.cr[0].eq {
	pc = 0x82FF5CC0; continue 'dispatch;
	}
	// 82FF5CA8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FF5CAC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5CB0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5CB4: 88DE0008  lbz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF5CB8: 48042099  bl 0x83037d50
	ctx.lr = 0x82FF5CBC;
	sub_83037D50(ctx, base);
	// 82FF5CBC: 48000008  b 0x82ff5cc4
	pc = 0x82FF5CC4; continue 'dispatch;
	// 82FF5CC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5CC4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5CC8: 481B24F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5CCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5CCC size=44
    let mut pc: u32 = 0x82FF5CCC;
    'dispatch: loop {
        match pc {
            0x82FF5CCC => {
    //   block [0x82FF5CCC..0x82FF5CF8)
	// 82FF5CCC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5CDC: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5CE0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5CE4: 4BFE25FD  bl 0x82fd82e0
	ctx.lr = 0x82FF5CE8;
	sub_82FD82E0(ctx, base);
	// 82FF5CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5CF8 size=76
    let mut pc: u32 = 0x82FF5CF8;
    'dispatch: loop {
        match pc {
            0x82FF5CF8 => {
    //   block [0x82FF5CF8..0x82FF5D44)
	// 82FF5CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF5D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF5D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF5D10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5D14: 4BFFFB0D  bl 0x82ff5820
	ctx.lr = 0x82FF5D18;
	sub_82FF5820(ctx, base);
	// 82FF5D18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF5D1C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82FF5D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF5D24: 396BEF00  addi r11, r11, -0x1100
	ctx.r[11].s64 = ctx.r[11].s64 + -4352;
	// 82FF5D28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF5D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF5D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF5D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5D48 size=8
    let mut pc: u32 = 0x82FF5D48;
    'dispatch: loop {
        match pc {
            0x82FF5D48 => {
    //   block [0x82FF5D48..0x82FF5D50)
	// 82FF5D48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5D4C: 8213EF10  lwz r16, -0x10f0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4336 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5D50 size=92
    let mut pc: u32 = 0x82FF5D50;
    'dispatch: loop {
        match pc {
            0x82FF5D50 => {
    //   block [0x82FF5D50..0x82FF5DAC)
	// 82FF5D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5D54: 481B2415  bl 0x831a8168
	ctx.lr = 0x82FF5D58;
	sub_831A8130(ctx, base);
	// 82FF5D58: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5D5C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5D60: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF5D64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF5D68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5D6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF5D70: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FF5D74: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82FF5D78: 4BFE2521  bl 0x82fd8298
	ctx.lr = 0x82FF5D7C;
	sub_82FD8298(ctx, base);
	// 82FF5D7C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5D80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5D84: 4182001C  beq 0x82ff5da0
	if ctx.cr[0].eq {
	pc = 0x82FF5DA0; continue 'dispatch;
	}
	// 82FF5D88: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FF5D8C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5D90: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5D94: 88DE0008  lbz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF5D98: 480421A1  bl 0x83037f38
	ctx.lr = 0x82FF5D9C;
	sub_83037F38(ctx, base);
	// 82FF5D9C: 48000008  b 0x82ff5da4
	pc = 0x82FF5DA4; continue 'dispatch;
	// 82FF5DA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5DA4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5DA8: 481B2410  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5DAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5DAC size=44
    let mut pc: u32 = 0x82FF5DAC;
    'dispatch: loop {
        match pc {
            0x82FF5DAC => {
    //   block [0x82FF5DAC..0x82FF5DD8)
	// 82FF5DAC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5DBC: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5DC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5DC4: 4BFE251D  bl 0x82fd82e0
	ctx.lr = 0x82FF5DC8;
	sub_82FD82E0(ctx, base);
	// 82FF5DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5DD8 size=8
    let mut pc: u32 = 0x82FF5DD8;
    'dispatch: loop {
        match pc {
            0x82FF5DD8 => {
    //   block [0x82FF5DD8..0x82FF5DE0)
	// 82FF5DD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5DDC: 8213EF50  lwz r16, -0x10b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5DE0 size=88
    let mut pc: u32 = 0x82FF5DE0;
    'dispatch: loop {
        match pc {
            0x82FF5DE0 => {
    //   block [0x82FF5DE0..0x82FF5E38)
	// 82FF5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5DE4: 481B2385  bl 0x831a8168
	ctx.lr = 0x82FF5DE8;
	sub_831A8130(ctx, base);
	// 82FF5DE8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5DEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5DF0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5DF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5DF8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5DFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5E00: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF5E04: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5E08: 4BFE2491  bl 0x82fd8298
	ctx.lr = 0x82FF5E0C;
	sub_82FD8298(ctx, base);
	// 82FF5E0C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5E14: 41820018  beq 0x82ff5e2c
	if ctx.cr[0].eq {
	pc = 0x82FF5E2C; continue 'dispatch;
	}
	// 82FF5E18: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5E1C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5E20: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5E24: 4804008D  bl 0x83035eb0
	ctx.lr = 0x82FF5E28;
	sub_83035EB0(ctx, base);
	// 82FF5E28: 48000008  b 0x82ff5e30
	pc = 0x82FF5E30; continue 'dispatch;
	// 82FF5E2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5E30: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5E34: 481B2384  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5E38 size=44
    let mut pc: u32 = 0x82FF5E38;
    'dispatch: loop {
        match pc {
            0x82FF5E38 => {
    //   block [0x82FF5E38..0x82FF5E64)
	// 82FF5E38: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5E3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5E40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5E48: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5E4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5E50: 4BFE2491  bl 0x82fd82e0
	ctx.lr = 0x82FF5E54;
	sub_82FD82E0(ctx, base);
	// 82FF5E54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5E68 size=8
    let mut pc: u32 = 0x82FF5E68;
    'dispatch: loop {
        match pc {
            0x82FF5E68 => {
    //   block [0x82FF5E68..0x82FF5E70)
	// 82FF5E68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5E6C: 8213EF90  lwz r16, -0x1070(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4208 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5E70 size=88
    let mut pc: u32 = 0x82FF5E70;
    'dispatch: loop {
        match pc {
            0x82FF5E70 => {
    //   block [0x82FF5E70..0x82FF5EC8)
	// 82FF5E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5E74: 481B22F5  bl 0x831a8168
	ctx.lr = 0x82FF5E78;
	sub_831A8130(ctx, base);
	// 82FF5E78: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5E7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5E80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5E84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5E88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5E8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5E90: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF5E94: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5E98: 4BFE2401  bl 0x82fd8298
	ctx.lr = 0x82FF5E9C;
	sub_82FD8298(ctx, base);
	// 82FF5E9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5EA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5EA4: 41820018  beq 0x82ff5ebc
	if ctx.cr[0].eq {
	pc = 0x82FF5EBC; continue 'dispatch;
	}
	// 82FF5EA8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5EAC: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5EB0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5EB4: 48042335  bl 0x830381e8
	ctx.lr = 0x82FF5EB8;
	sub_830381E8(ctx, base);
	// 82FF5EB8: 48000008  b 0x82ff5ec0
	pc = 0x82FF5EC0; continue 'dispatch;
	// 82FF5EBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5EC0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5EC4: 481B22F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5EC8 size=44
    let mut pc: u32 = 0x82FF5EC8;
    'dispatch: loop {
        match pc {
            0x82FF5EC8 => {
    //   block [0x82FF5EC8..0x82FF5EF4)
	// 82FF5EC8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5ECC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5ED0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5ED8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5EDC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5EE0: 4BFE2401  bl 0x82fd82e0
	ctx.lr = 0x82FF5EE4;
	sub_82FD82E0(ctx, base);
	// 82FF5EE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5EF8 size=8
    let mut pc: u32 = 0x82FF5EF8;
    'dispatch: loop {
        match pc {
            0x82FF5EF8 => {
    //   block [0x82FF5EF8..0x82FF5F00)
	// 82FF5EF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5EFC: 8213EFD0  lwz r16, -0x1030(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5F00 size=88
    let mut pc: u32 = 0x82FF5F00;
    'dispatch: loop {
        match pc {
            0x82FF5F00 => {
    //   block [0x82FF5F00..0x82FF5F58)
	// 82FF5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5F04: 481B2265  bl 0x831a8168
	ctx.lr = 0x82FF5F08;
	sub_831A8130(ctx, base);
	// 82FF5F08: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5F0C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5F10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5F14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5F18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5F1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5F20: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF5F24: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5F28: 4BFE2371  bl 0x82fd8298
	ctx.lr = 0x82FF5F2C;
	sub_82FD8298(ctx, base);
	// 82FF5F2C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5F30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5F34: 41820018  beq 0x82ff5f4c
	if ctx.cr[0].eq {
	pc = 0x82FF5F4C; continue 'dispatch;
	}
	// 82FF5F38: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5F3C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5F40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5F44: 480422FD  bl 0x83038240
	ctx.lr = 0x82FF5F48;
	sub_83038240(ctx, base);
	// 82FF5F48: 48000008  b 0x82ff5f50
	pc = 0x82FF5F50; continue 'dispatch;
	// 82FF5F4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5F50: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5F54: 481B2264  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5F58 size=44
    let mut pc: u32 = 0x82FF5F58;
    'dispatch: loop {
        match pc {
            0x82FF5F58 => {
    //   block [0x82FF5F58..0x82FF5F84)
	// 82FF5F58: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5F5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5F60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5F64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5F68: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5F6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF5F70: 4BFE2371  bl 0x82fd82e0
	ctx.lr = 0x82FF5F74;
	sub_82FD82E0(ctx, base);
	// 82FF5F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF5F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF5F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF5F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF5F88 size=8
    let mut pc: u32 = 0x82FF5F88;
    'dispatch: loop {
        match pc {
            0x82FF5F88 => {
    //   block [0x82FF5F88..0x82FF5F90)
	// 82FF5F88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF5F8C: 8213F010  lwz r16, -0xff0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-4080 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5F90 size=88
    let mut pc: u32 = 0x82FF5F90;
    'dispatch: loop {
        match pc {
            0x82FF5F90 => {
    //   block [0x82FF5F90..0x82FF5FE8)
	// 82FF5F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5F94: 481B21D5  bl 0x831a8168
	ctx.lr = 0x82FF5F98;
	sub_831A8130(ctx, base);
	// 82FF5F98: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF5F9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5FA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF5FA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF5FA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF5FAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF5FB0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF5FB4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF5FB8: 4BFE22E1  bl 0x82fd8298
	ctx.lr = 0x82FF5FBC;
	sub_82FD8298(ctx, base);
	// 82FF5FBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF5FC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF5FC4: 41820018  beq 0x82ff5fdc
	if ctx.cr[0].eq {
	pc = 0x82FF5FDC; continue 'dispatch;
	}
	// 82FF5FC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FF5FCC: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF5FD0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF5FD4: 480422C5  bl 0x83038298
	ctx.lr = 0x82FF5FD8;
	sub_83038298(ctx, base);
	// 82FF5FD8: 48000008  b 0x82ff5fe0
	pc = 0x82FF5FE0; continue 'dispatch;
	// 82FF5FDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF5FE0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF5FE4: 481B21D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF5FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF5FE8 size=44
    let mut pc: u32 = 0x82FF5FE8;
    'dispatch: loop {
        match pc {
            0x82FF5FE8 => {
    //   block [0x82FF5FE8..0x82FF6014)
	// 82FF5FE8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF5FEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF5FF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF5FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF5FF8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF5FFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF6000: 4BFE22E1  bl 0x82fd82e0
	ctx.lr = 0x82FF6004;
	sub_82FD82E0(ctx, base);
	// 82FF6004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF6008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF6010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF6018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF6018 size=112
    let mut pc: u32 = 0x82FF6018;
    'dispatch: loop {
        match pc {
            0x82FF6018 => {
    //   block [0x82FF6018..0x82FF6088)
	// 82FF6018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF6020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF6024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF6028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF602C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF6034: 396BED58  addi r11, r11, -0x12a8
	ctx.r[11].s64 = ctx.r[11].s64 + -4776;
	// 82FF6038: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF603C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6040: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF6044: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF6048: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF604C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6050: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF6054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6058: 4E800421  bctrl
	ctx.lr = 0x82FF605C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF605C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF6060: 4182000C  beq 0x82ff606c
	if ctx.cr[0].eq {
	pc = 0x82FF606C; continue 'dispatch;
	}
	// 82FF6064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF6068: 4BFE2279  bl 0x82fd82e0
	ctx.lr = 0x82FF606C;
	sub_82FD82E0(ctx, base);
	// 82FF606C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF6070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF6074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF6078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF607C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF6080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF6084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF6088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF6088 size=68
    let mut pc: u32 = 0x82FF6088;
    'dispatch: loop {
        match pc {
            0x82FF6088 => {
    //   block [0x82FF6088..0x82FF60CC)
	// 82FF6088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF608C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF6090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF6094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF6098: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF609C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF60A0: 396BED60  addi r11, r11, -0x12a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4768;
	// 82FF60A4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF60A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF60AC: 41820008  beq 0x82ff60b4
	if ctx.cr[0].eq {
	pc = 0x82FF60B4; continue 'dispatch;
	}
	// 82FF60B0: 4BFE2231  bl 0x82fd82e0
	ctx.lr = 0x82FF60B4;
	sub_82FD82E0(ctx, base);
	// 82FF60B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF60B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF60BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF60C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF60C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF60C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF60D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF60D0 size=180
    let mut pc: u32 = 0x82FF60D0;
    'dispatch: loop {
        match pc {
            0x82FF60D0 => {
    //   block [0x82FF60D0..0x82FF6184)
	// 82FF60D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF60D4: 481B208D  bl 0x831a8160
	ctx.lr = 0x82FF60D8;
	sub_831A8130(ctx, base);
	// 82FF60D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF60DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF60E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF60E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FF60E8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FF60EC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FF60F0: 2B1F0007  cmplwi cr6, r31, 7
	ctx.cr[6].compare_u32(ctx.r[31].u32, 7 as u32, &mut ctx.xer);
	// 82FF60F4: 4199007C  bgt cr6, 0x82ff6170
	if ctx.cr[6].gt {
	pc = 0x82FF6170; continue 'dispatch;
	}
	// 82FF60F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF60FC: 806BB8FC  lwz r3, -0x4704(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6100: 48036771  bl 0x8302c870
	ctx.lr = 0x82FF6104;
	sub_8302C870(ctx, base);
	// 82FF6104: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6108: 41820028  beq 0x82ff6130
	if ctx.cr[0].eq {
	pc = 0x82FF6130; continue 'dispatch;
	}
	// 82FF610C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6110: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6114: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FF6118: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF611C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6120: 4E800421  bctrl
	ctx.lr = 0x82FF6124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6124: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FF6128: 556BE7BC  rlwinm r11, r11, 0x1c, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82FF612C: 4800004C  b 0x82ff6178
	pc = 0x82FF6178; continue 'dispatch;
	// 82FF6130: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6134: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF613C: 4803FCFD  bl 0x83035e38
	ctx.lr = 0x82FF6140;
	sub_83035E38(ctx, base);
	// 82FF6140: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF6144: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF6148: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF614C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF6150: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82FF6154: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82FF6158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF615C: 4E800421  bctrl
	ctx.lr = 0x82FF6160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6160: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6164: 41820018  beq 0x82ff617c
	if ctx.cr[0].eq {
	pc = 0x82FF617C; continue 'dispatch;
	}
	// 82FF6168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF616C: 4800000C  b 0x82ff6178
	pc = 0x82FF6178; continue 'dispatch;
	// 82FF6170: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82FF6174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF6178: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF617C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FF6180: 481B2030  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF6188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF6188 size=68
    let mut pc: u32 = 0x82FF6188;
    'dispatch: loop {
        match pc {
            0x82FF6188 => {
    //   block [0x82FF6188..0x82FF61CC)
	// 82FF6188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF618C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF6190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF6194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF6198: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF619C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF61A0: 396BED9C  addi r11, r11, -0x1264
	ctx.r[11].s64 = ctx.r[11].s64 + -4708;
	// 82FF61A4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF61A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF61AC: 41820008  beq 0x82ff61b4
	if ctx.cr[0].eq {
	pc = 0x82FF61B4; continue 'dispatch;
	}
	// 82FF61B0: 4BFE2131  bl 0x82fd82e0
	ctx.lr = 0x82FF61B4;
	sub_82FD82E0(ctx, base);
	// 82FF61B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF61B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF61BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF61C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF61C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF61C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF61D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF61D0 size=260
    let mut pc: u32 = 0x82FF61D0;
    'dispatch: loop {
        match pc {
            0x82FF61D0 => {
    //   block [0x82FF61D0..0x82FF62D4)
	// 82FF61D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF61D4: 481B1F91  bl 0x831a8164
	ctx.lr = 0x82FF61D8;
	sub_831A8130(ctx, base);
	// 82FF61D8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FF61DC: 9421EF60  stwu r1, -0x10a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF61E0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF61E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF61E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FF61EC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FF61F0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FF61F4: 896BB8F4  lbz r11, -0x470c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-18188 as u32) ) } as u64;
	// 82FF61F8: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FF61FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6200: 41820024  beq 0x82ff6224
	if ctx.cr[0].eq {
	pc = 0x82FF6224; continue 'dispatch;
	}
	// 82FF6204: 4804261D  bl 0x83038820
	ctx.lr = 0x82FF6208;
	sub_83038820(ctx, base);
	// 82FF6208: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF620C: 4804246D  bl 0x83038678
	ctx.lr = 0x82FF6210;
	sub_83038678(ctx, base);
	// 82FF6210: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF6214: 40820010  bne 0x82ff6224
	if !ctx.cr[0].eq {
	pc = 0x82FF6224; continue 'dispatch;
	}
	// 82FF6218: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF621C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF6220: 480000A8  b 0x82ff62c8
	pc = 0x82FF62C8; continue 'dispatch;
	// 82FF6224: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 82FF6228: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF622C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF6230: 4BFDB979  bl 0x82fd1ba8
	ctx.lr = 0x82FF6234;
	sub_82FD1BA8(ctx, base);
	// 82FF6234: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF6238: 4082000C  bne 0x82ff6244
	if !ctx.cr[0].eq {
	pc = 0x82FF6244; continue 'dispatch;
	}
	// 82FF623C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82FF6240: 4BFFFFDC  b 0x82ff621c
	pc = 0x82FF621C; continue 'dispatch;
	// 82FF6244: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF6248: 4BFDBE01  bl 0x82fd2048
	ctx.lr = 0x82FF624C;
	sub_82FD2048(ctx, base);
	// 82FF624C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF6250: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FF6254: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82FF6258: 806BB8F8  lwz r3, -0x4708(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF625C: 48004205  bl 0x82ffa460
	ctx.lr = 0x82FF6260;
	sub_82FFA460(ctx, base);
	// 82FF6260: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6264: 41820034  beq 0x82ff6298
	if ctx.cr[0].eq {
	pc = 0x82FF6298; continue 'dispatch;
	}
	// 82FF6268: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF626C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6270: 41820028  beq 0x82ff6298
	if ctx.cr[0].eq {
	pc = 0x82FF6298; continue 'dispatch;
	}
	// 82FF6274: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6278: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF627C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6280: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6288: 4E800421  bctrl
	ctx.lr = 0x82FF628C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF628C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FF6290: 556BE7BC  rlwinm r11, r11, 0x1c, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82FF6294: 48000034  b 0x82ff62c8
	pc = 0x82FF62C8; continue 'dispatch;
	// 82FF6298: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF629C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FF62A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FF62A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FF62A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FF62AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF62B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF62B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF62B8: 4E800421  bctrl
	ctx.lr = 0x82FF62BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF62BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF62C0: 4182000C  beq 0x82ff62cc
	if ctx.cr[0].eq {
	pc = 0x82FF62CC; continue 'dispatch;
	}
	// 82FF62C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF62C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF62CC: 382110A0  addi r1, r1, 0x10a0
	ctx.r[1].s64 = ctx.r[1].s64 + 4256;
	// 82FF62D0: 481B1EE4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF62D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF62D8 size=8
    let mut pc: u32 = 0x82FF62D8;
    'dispatch: loop {
        match pc {
            0x82FF62D8 => {
    //   block [0x82FF62D8..0x82FF62E0)
	// 82FF62D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF62DC: 8213F1E0  lwz r16, -0xe20(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-3616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF62E0 size=3628
    let mut pc: u32 = 0x82FF62E0;
    'dispatch: loop {
        match pc {
            0x82FF62E0 => {
    //   block [0x82FF62E0..0x82FF710C)
	// 82FF62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF62E4: 481B1E7D  bl 0x831a8160
	ctx.lr = 0x82FF62E8;
	sub_831A8130(ctx, base);
	// 82FF62E8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF62EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF62F0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82FF62F4: 3F408339  lis r26, -0x7cc7
	ctx.r[26].s64 = -2093416448;
	// 82FF62F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF62FC: 807AB8FC  lwz r3, -0x4704(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6300: 48044E51  bl 0x8303b150
	ctx.lr = 0x82FF6304;
	sub_8303B150(ctx, base);
	// 82FF6304: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6308: 4082FFF0  bne 0x82ff62f8
	if !ctx.cr[0].eq {
	pc = 0x82FF62F8; continue 'dispatch;
	}
	// 82FF630C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6310: 4BFE1F39  bl 0x82fd8248
	ctx.lr = 0x82FF6314;
	sub_82FD8248(ctx, base);
	// 82FF6314: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6318: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF631C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6320: 3B8BEDC0  addi r28, r11, -0x1240
	ctx.r[28].s64 = ctx.r[11].s64 + -4672;
	// 82FF6324: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6328: 3BAB7F08  addi r29, r11, 0x7f08
	ctx.r[29].s64 = ctx.r[11].s64 + 32520;
	// 82FF632C: 4182001C  beq 0x82ff6348
	if ctx.cr[0].eq {
	pc = 0x82FF6348; continue 'dispatch;
	}
	// 82FF6330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6334: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6338: 4BFFF4E9  bl 0x82ff5820
	ctx.lr = 0x82FF633C;
	sub_82FF5820(ctx, base);
	// 82FF633C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6340: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FF6344: 48000008  b 0x82ff634c
	pc = 0x82FF634C; continue 'dispatch;
	// 82FF6348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF634C: 807AB8FC  lwz r3, -0x4704(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6350: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82FF6354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6358: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF635C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6360: 4E800421  bctrl
	ctx.lr = 0x82FF6364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6364: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6368: 4BFE1EE1  bl 0x82fd8248
	ctx.lr = 0x82FF636C;
	sub_82FD8248(ctx, base);
	// 82FF636C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6370: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6374: 4182001C  beq 0x82ff6390
	if ctx.cr[0].eq {
	pc = 0x82FF6390; continue 'dispatch;
	}
	// 82FF6378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF637C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6380: 4BFFF4A1  bl 0x82ff5820
	ctx.lr = 0x82FF6384;
	sub_82FF5820(ctx, base);
	// 82FF6384: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6388: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FF638C: 48000008  b 0x82ff6394
	pc = 0x82FF6394; continue 'dispatch;
	// 82FF6390: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6394: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 82FF6398: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF639C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF63A0: 48036AA9  bl 0x8302ce48
	ctx.lr = 0x82FF63A4;
	sub_8302CE48(ctx, base);
	// 82FF63A4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF63A8: 4BFE1EA1  bl 0x82fd8248
	ctx.lr = 0x82FF63AC;
	sub_82FD8248(ctx, base);
	// 82FF63AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF63B0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF63B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF63B8: 3B6BEE00  addi r27, r11, -0x1200
	ctx.r[27].s64 = ctx.r[11].s64 + -4608;
	// 82FF63BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF63C0: 3BAB7D84  addi r29, r11, 0x7d84
	ctx.r[29].s64 = ctx.r[11].s64 + 32132;
	// 82FF63C4: 4182001C  beq 0x82ff63e0
	if ctx.cr[0].eq {
	pc = 0x82FF63E0; continue 'dispatch;
	}
	// 82FF63C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF63CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF63D0: 4BFFF451  bl 0x82ff5820
	ctx.lr = 0x82FF63D4;
	sub_82FF5820(ctx, base);
	// 82FF63D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF63D8: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF63DC: 48000008  b 0x82ff63e4
	pc = 0x82FF63E4; continue 'dispatch;
	// 82FF63E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF63E4: 807AB8FC  lwz r3, -0x4704(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF63E8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82FF63EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF63F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF63F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF63F8: 4E800421  bctrl
	ctx.lr = 0x82FF63FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF63FC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6400: 4BFE1E49  bl 0x82fd8248
	ctx.lr = 0x82FF6404;
	sub_82FD8248(ctx, base);
	// 82FF6404: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6408: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF640C: 4182001C  beq 0x82ff6428
	if ctx.cr[0].eq {
	pc = 0x82FF6428; continue 'dispatch;
	}
	// 82FF6410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6418: 4BFFF409  bl 0x82ff5820
	ctx.lr = 0x82FF641C;
	sub_82FF5820(ctx, base);
	// 82FF641C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6420: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6424: 48000008  b 0x82ff642c
	pc = 0x82FF642C; continue 'dispatch;
	// 82FF6428: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF642C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6430: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6434: 48036A15  bl 0x8302ce48
	ctx.lr = 0x82FF6438;
	sub_8302CE48(ctx, base);
	// 82FF6438: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF643C: 4BFE1E0D  bl 0x82fd8248
	ctx.lr = 0x82FF6440;
	sub_82FD8248(ctx, base);
	// 82FF6440: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6444: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6448: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF644C: 3BAB7D98  addi r29, r11, 0x7d98
	ctx.r[29].s64 = ctx.r[11].s64 + 32152;
	// 82FF6450: 4182001C  beq 0x82ff646c
	if ctx.cr[0].eq {
	pc = 0x82FF646C; continue 'dispatch;
	}
	// 82FF6454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6458: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF645C: 4BFFF3C5  bl 0x82ff5820
	ctx.lr = 0x82FF6460;
	sub_82FF5820(ctx, base);
	// 82FF6460: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6464: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6468: 48000008  b 0x82ff6470
	pc = 0x82FF6470; continue 'dispatch;
	// 82FF646C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6470: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6474: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6478: 480369D1  bl 0x8302ce48
	ctx.lr = 0x82FF647C;
	sub_8302CE48(ctx, base);
	// 82FF647C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6480: 4BFE1DC9  bl 0x82fd8248
	ctx.lr = 0x82FF6484;
	sub_82FD8248(ctx, base);
	// 82FF6484: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6488: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF648C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6490: 3BAB7DA8  addi r29, r11, 0x7da8
	ctx.r[29].s64 = ctx.r[11].s64 + 32168;
	// 82FF6494: 4182001C  beq 0x82ff64b0
	if ctx.cr[0].eq {
	pc = 0x82FF64B0; continue 'dispatch;
	}
	// 82FF6498: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF649C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF64A0: 4BFFF381  bl 0x82ff5820
	ctx.lr = 0x82FF64A4;
	sub_82FF5820(ctx, base);
	// 82FF64A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF64A8: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF64AC: 48000008  b 0x82ff64b4
	pc = 0x82FF64B4; continue 'dispatch;
	// 82FF64B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF64B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF64B8: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF64BC: 4803698D  bl 0x8302ce48
	ctx.lr = 0x82FF64C0;
	sub_8302CE48(ctx, base);
	// 82FF64C0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF64C4: 4BFE1D85  bl 0x82fd8248
	ctx.lr = 0x82FF64C8;
	sub_82FD8248(ctx, base);
	// 82FF64C8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF64CC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF64D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF64D4: 3BAB7DB4  addi r29, r11, 0x7db4
	ctx.r[29].s64 = ctx.r[11].s64 + 32180;
	// 82FF64D8: 4182001C  beq 0x82ff64f4
	if ctx.cr[0].eq {
	pc = 0x82FF64F4; continue 'dispatch;
	}
	// 82FF64DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF64E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF64E4: 4BFFF33D  bl 0x82ff5820
	ctx.lr = 0x82FF64E8;
	sub_82FF5820(ctx, base);
	// 82FF64E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF64EC: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF64F0: 48000008  b 0x82ff64f8
	pc = 0x82FF64F8; continue 'dispatch;
	// 82FF64F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF64F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF64FC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6500: 48036949  bl 0x8302ce48
	ctx.lr = 0x82FF6504;
	sub_8302CE48(ctx, base);
	// 82FF6504: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6508: 4BFE1D41  bl 0x82fd8248
	ctx.lr = 0x82FF650C;
	sub_82FD8248(ctx, base);
	// 82FF650C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6510: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6514: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6518: 3B6BEE40  addi r27, r11, -0x11c0
	ctx.r[27].s64 = ctx.r[11].s64 + -4544;
	// 82FF651C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6520: 3BAB7DC8  addi r29, r11, 0x7dc8
	ctx.r[29].s64 = ctx.r[11].s64 + 32200;
	// 82FF6524: 4182001C  beq 0x82ff6540
	if ctx.cr[0].eq {
	pc = 0x82FF6540; continue 'dispatch;
	}
	// 82FF6528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF652C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6530: 4BFFF2F1  bl 0x82ff5820
	ctx.lr = 0x82FF6534;
	sub_82FF5820(ctx, base);
	// 82FF6534: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6538: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF653C: 48000008  b 0x82ff6544
	pc = 0x82FF6544; continue 'dispatch;
	// 82FF6540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6544: 807AB8FC  lwz r3, -0x4704(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6548: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FF654C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6550: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6558: 4E800421  bctrl
	ctx.lr = 0x82FF655C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF655C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6560: 4BFE1CE9  bl 0x82fd8248
	ctx.lr = 0x82FF6564;
	sub_82FD8248(ctx, base);
	// 82FF6564: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6568: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF656C: 4182001C  beq 0x82ff6588
	if ctx.cr[0].eq {
	pc = 0x82FF6588; continue 'dispatch;
	}
	// 82FF6570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6574: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6578: 4BFFF2A9  bl 0x82ff5820
	ctx.lr = 0x82FF657C;
	sub_82FF5820(ctx, base);
	// 82FF657C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6580: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6584: 48000008  b 0x82ff658c
	pc = 0x82FF658C; continue 'dispatch;
	// 82FF6588: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF658C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6590: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6594: 480368B5  bl 0x8302ce48
	ctx.lr = 0x82FF6598;
	sub_8302CE48(ctx, base);
	// 82FF6598: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF659C: 4BFE1CAD  bl 0x82fd8248
	ctx.lr = 0x82FF65A0;
	sub_82FD8248(ctx, base);
	// 82FF65A0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF65A4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF65A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF65AC: 3BAB7DD4  addi r29, r11, 0x7dd4
	ctx.r[29].s64 = ctx.r[11].s64 + 32212;
	// 82FF65B0: 4182001C  beq 0x82ff65cc
	if ctx.cr[0].eq {
	pc = 0x82FF65CC; continue 'dispatch;
	}
	// 82FF65B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF65B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF65BC: 4BFFF265  bl 0x82ff5820
	ctx.lr = 0x82FF65C0;
	sub_82FF5820(ctx, base);
	// 82FF65C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF65C4: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF65C8: 48000008  b 0x82ff65d0
	pc = 0x82FF65D0; continue 'dispatch;
	// 82FF65CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF65D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF65D4: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF65D8: 48036871  bl 0x8302ce48
	ctx.lr = 0x82FF65DC;
	sub_8302CE48(ctx, base);
	// 82FF65DC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF65E0: 4BFE1C69  bl 0x82fd8248
	ctx.lr = 0x82FF65E4;
	sub_82FD8248(ctx, base);
	// 82FF65E4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF65E8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF65EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF65F0: 3B6BEE80  addi r27, r11, -0x1180
	ctx.r[27].s64 = ctx.r[11].s64 + -4480;
	// 82FF65F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF65F8: 3BAB7B1C  addi r29, r11, 0x7b1c
	ctx.r[29].s64 = ctx.r[11].s64 + 31516;
	// 82FF65FC: 4182001C  beq 0x82ff6618
	if ctx.cr[0].eq {
	pc = 0x82FF6618; continue 'dispatch;
	}
	// 82FF6600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6604: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6608: 4BFFF219  bl 0x82ff5820
	ctx.lr = 0x82FF660C;
	sub_82FF5820(ctx, base);
	// 82FF660C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6610: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6614: 48000008  b 0x82ff661c
	pc = 0x82FF661C; continue 'dispatch;
	// 82FF6618: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF661C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6620: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6624: 48036825  bl 0x8302ce48
	ctx.lr = 0x82FF6628;
	sub_8302CE48(ctx, base);
	// 82FF6628: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF662C: 4BFE1C1D  bl 0x82fd8248
	ctx.lr = 0x82FF6630;
	sub_82FD8248(ctx, base);
	// 82FF6630: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6634: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6638: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF663C: 3BAB7B30  addi r29, r11, 0x7b30
	ctx.r[29].s64 = ctx.r[11].s64 + 31536;
	// 82FF6640: 4182001C  beq 0x82ff665c
	if ctx.cr[0].eq {
	pc = 0x82FF665C; continue 'dispatch;
	}
	// 82FF6644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6648: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF664C: 4BFFF1D5  bl 0x82ff5820
	ctx.lr = 0x82FF6650;
	sub_82FF5820(ctx, base);
	// 82FF6650: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6654: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6658: 48000008  b 0x82ff6660
	pc = 0x82FF6660; continue 'dispatch;
	// 82FF665C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6660: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6664: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6668: 480367E1  bl 0x8302ce48
	ctx.lr = 0x82FF666C;
	sub_8302CE48(ctx, base);
	// 82FF666C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6670: 4BFE1BD9  bl 0x82fd8248
	ctx.lr = 0x82FF6674;
	sub_82FD8248(ctx, base);
	// 82FF6674: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6678: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF667C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6680: 3BAB7B48  addi r29, r11, 0x7b48
	ctx.r[29].s64 = ctx.r[11].s64 + 31560;
	// 82FF6684: 4182001C  beq 0x82ff66a0
	if ctx.cr[0].eq {
	pc = 0x82FF66A0; continue 'dispatch;
	}
	// 82FF6688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF668C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6690: 4BFFF191  bl 0x82ff5820
	ctx.lr = 0x82FF6694;
	sub_82FF5820(ctx, base);
	// 82FF6694: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6698: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF669C: 48000008  b 0x82ff66a4
	pc = 0x82FF66A4; continue 'dispatch;
	// 82FF66A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF66A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF66A8: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF66AC: 4803679D  bl 0x8302ce48
	ctx.lr = 0x82FF66B0;
	sub_8302CE48(ctx, base);
	// 82FF66B0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF66B4: 4BFE1B95  bl 0x82fd8248
	ctx.lr = 0x82FF66B8;
	sub_82FD8248(ctx, base);
	// 82FF66B8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF66BC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF66C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF66C4: 3BAB7B60  addi r29, r11, 0x7b60
	ctx.r[29].s64 = ctx.r[11].s64 + 31584;
	// 82FF66C8: 4182001C  beq 0x82ff66e4
	if ctx.cr[0].eq {
	pc = 0x82FF66E4; continue 'dispatch;
	}
	// 82FF66CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF66D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF66D4: 4BFFF14D  bl 0x82ff5820
	ctx.lr = 0x82FF66D8;
	sub_82FF5820(ctx, base);
	// 82FF66D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF66DC: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF66E0: 48000008  b 0x82ff66e8
	pc = 0x82FF66E8; continue 'dispatch;
	// 82FF66E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF66E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF66EC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF66F0: 48036759  bl 0x8302ce48
	ctx.lr = 0x82FF66F4;
	sub_8302CE48(ctx, base);
	// 82FF66F4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF66F8: 4BFE1B51  bl 0x82fd8248
	ctx.lr = 0x82FF66FC;
	sub_82FD8248(ctx, base);
	// 82FF66FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6700: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6704: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6708: 3BAB7B70  addi r29, r11, 0x7b70
	ctx.r[29].s64 = ctx.r[11].s64 + 31600;
	// 82FF670C: 4182001C  beq 0x82ff6728
	if ctx.cr[0].eq {
	pc = 0x82FF6728; continue 'dispatch;
	}
	// 82FF6710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6714: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6718: 4BFFF109  bl 0x82ff5820
	ctx.lr = 0x82FF671C;
	sub_82FF5820(ctx, base);
	// 82FF671C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6720: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6724: 48000008  b 0x82ff672c
	pc = 0x82FF672C; continue 'dispatch;
	// 82FF6728: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF672C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6730: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6734: 48036715  bl 0x8302ce48
	ctx.lr = 0x82FF6738;
	sub_8302CE48(ctx, base);
	// 82FF6738: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF673C: 4BFE1B0D  bl 0x82fd8248
	ctx.lr = 0x82FF6740;
	sub_82FD8248(ctx, base);
	// 82FF6740: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6744: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6748: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF674C: 3BAB7B80  addi r29, r11, 0x7b80
	ctx.r[29].s64 = ctx.r[11].s64 + 31616;
	// 82FF6750: 4182001C  beq 0x82ff676c
	if ctx.cr[0].eq {
	pc = 0x82FF676C; continue 'dispatch;
	}
	// 82FF6754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6758: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF675C: 4BFFF0C5  bl 0x82ff5820
	ctx.lr = 0x82FF6760;
	sub_82FF5820(ctx, base);
	// 82FF6760: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6764: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6768: 48000008  b 0x82ff6770
	pc = 0x82FF6770; continue 'dispatch;
	// 82FF676C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6770: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6774: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6778: 480366D1  bl 0x8302ce48
	ctx.lr = 0x82FF677C;
	sub_8302CE48(ctx, base);
	// 82FF677C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6780: 4BFE1AC9  bl 0x82fd8248
	ctx.lr = 0x82FF6784;
	sub_82FD8248(ctx, base);
	// 82FF6784: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6788: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF678C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6790: 3BAB7B90  addi r29, r11, 0x7b90
	ctx.r[29].s64 = ctx.r[11].s64 + 31632;
	// 82FF6794: 4182001C  beq 0x82ff67b0
	if ctx.cr[0].eq {
	pc = 0x82FF67B0; continue 'dispatch;
	}
	// 82FF6798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF679C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF67A0: 4BFFF081  bl 0x82ff5820
	ctx.lr = 0x82FF67A4;
	sub_82FF5820(ctx, base);
	// 82FF67A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF67A8: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF67AC: 48000008  b 0x82ff67b4
	pc = 0x82FF67B4; continue 'dispatch;
	// 82FF67B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF67B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF67B8: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF67BC: 4803668D  bl 0x8302ce48
	ctx.lr = 0x82FF67C0;
	sub_8302CE48(ctx, base);
	// 82FF67C0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF67C4: 4BFE1A85  bl 0x82fd8248
	ctx.lr = 0x82FF67C8;
	sub_82FD8248(ctx, base);
	// 82FF67C8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF67CC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF67D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF67D4: 3BAB7BA0  addi r29, r11, 0x7ba0
	ctx.r[29].s64 = ctx.r[11].s64 + 31648;
	// 82FF67D8: 4182001C  beq 0x82ff67f4
	if ctx.cr[0].eq {
	pc = 0x82FF67F4; continue 'dispatch;
	}
	// 82FF67DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF67E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF67E4: 4BFFF03D  bl 0x82ff5820
	ctx.lr = 0x82FF67E8;
	sub_82FF5820(ctx, base);
	// 82FF67E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF67EC: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF67F0: 48000008  b 0x82ff67f8
	pc = 0x82FF67F8; continue 'dispatch;
	// 82FF67F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF67F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF67FC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6800: 48036649  bl 0x8302ce48
	ctx.lr = 0x82FF6804;
	sub_8302CE48(ctx, base);
	// 82FF6804: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6808: 4BFE1A41  bl 0x82fd8248
	ctx.lr = 0x82FF680C;
	sub_82FD8248(ctx, base);
	// 82FF680C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6810: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6814: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6818: 3BAB7BB0  addi r29, r11, 0x7bb0
	ctx.r[29].s64 = ctx.r[11].s64 + 31664;
	// 82FF681C: 4182001C  beq 0x82ff6838
	if ctx.cr[0].eq {
	pc = 0x82FF6838; continue 'dispatch;
	}
	// 82FF6820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6824: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6828: 4BFFEFF9  bl 0x82ff5820
	ctx.lr = 0x82FF682C;
	sub_82FF5820(ctx, base);
	// 82FF682C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6830: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6834: 48000008  b 0x82ff683c
	pc = 0x82FF683C; continue 'dispatch;
	// 82FF6838: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF683C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6840: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6844: 48036605  bl 0x8302ce48
	ctx.lr = 0x82FF6848;
	sub_8302CE48(ctx, base);
	// 82FF6848: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF684C: 4BFE19FD  bl 0x82fd8248
	ctx.lr = 0x82FF6850;
	sub_82FD8248(ctx, base);
	// 82FF6850: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6854: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6858: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF685C: 3BAB7BBC  addi r29, r11, 0x7bbc
	ctx.r[29].s64 = ctx.r[11].s64 + 31676;
	// 82FF6860: 4182001C  beq 0x82ff687c
	if ctx.cr[0].eq {
	pc = 0x82FF687C; continue 'dispatch;
	}
	// 82FF6864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF686C: 4BFFEFB5  bl 0x82ff5820
	ctx.lr = 0x82FF6870;
	sub_82FF5820(ctx, base);
	// 82FF6870: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6874: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6878: 48000008  b 0x82ff6880
	pc = 0x82FF6880; continue 'dispatch;
	// 82FF687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6884: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6888: 480365C1  bl 0x8302ce48
	ctx.lr = 0x82FF688C;
	sub_8302CE48(ctx, base);
	// 82FF688C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6890: 4BFE19B9  bl 0x82fd8248
	ctx.lr = 0x82FF6894;
	sub_82FD8248(ctx, base);
	// 82FF6894: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6898: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF689C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF68A0: 3BAB7BD4  addi r29, r11, 0x7bd4
	ctx.r[29].s64 = ctx.r[11].s64 + 31700;
	// 82FF68A4: 4182001C  beq 0x82ff68c0
	if ctx.cr[0].eq {
	pc = 0x82FF68C0; continue 'dispatch;
	}
	// 82FF68A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF68AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF68B0: 4BFFEF71  bl 0x82ff5820
	ctx.lr = 0x82FF68B4;
	sub_82FF5820(ctx, base);
	// 82FF68B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF68B8: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF68BC: 48000008  b 0x82ff68c4
	pc = 0x82FF68C4; continue 'dispatch;
	// 82FF68C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF68C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF68C8: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF68CC: 4803657D  bl 0x8302ce48
	ctx.lr = 0x82FF68D0;
	sub_8302CE48(ctx, base);
	// 82FF68D0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF68D4: 4BFE1975  bl 0x82fd8248
	ctx.lr = 0x82FF68D8;
	sub_82FD8248(ctx, base);
	// 82FF68D8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF68DC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF68E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF68E4: 3BAB7BEC  addi r29, r11, 0x7bec
	ctx.r[29].s64 = ctx.r[11].s64 + 31724;
	// 82FF68E8: 4182001C  beq 0x82ff6904
	if ctx.cr[0].eq {
	pc = 0x82FF6904; continue 'dispatch;
	}
	// 82FF68EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF68F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF68F4: 4BFFEF2D  bl 0x82ff5820
	ctx.lr = 0x82FF68F8;
	sub_82FF5820(ctx, base);
	// 82FF68F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF68FC: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6900: 48000008  b 0x82ff6908
	pc = 0x82FF6908; continue 'dispatch;
	// 82FF6904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6908: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF690C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6910: 48036539  bl 0x8302ce48
	ctx.lr = 0x82FF6914;
	sub_8302CE48(ctx, base);
	// 82FF6914: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6918: 4BFE1931  bl 0x82fd8248
	ctx.lr = 0x82FF691C;
	sub_82FD8248(ctx, base);
	// 82FF691C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6920: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6924: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6928: 3BCB7E58  addi r30, r11, 0x7e58
	ctx.r[30].s64 = ctx.r[11].s64 + 32344;
	// 82FF692C: 41820018  beq 0x82ff6944
	if ctx.cr[0].eq {
	pc = 0x82FF6944; continue 'dispatch;
	}
	// 82FF6930: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF6934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6938: 4BFFF2E1  bl 0x82ff5c18
	ctx.lr = 0x82FF693C;
	sub_82FF5C18(ctx, base);
	// 82FF693C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF6940: 48000008  b 0x82ff6948
	pc = 0x82FF6948; continue 'dispatch;
	// 82FF6944: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6948: 817AB8FC  lwz r11, -0x4704(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF694C: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82FF6950: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF6954: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6958: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF695C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6960: 4E800421  bctrl
	ctx.lr = 0x82FF6964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6964: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6968: 4BFE18E1  bl 0x82fd8248
	ctx.lr = 0x82FF696C;
	sub_82FD8248(ctx, base);
	// 82FF696C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6970: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6974: 41820018  beq 0x82ff698c
	if ctx.cr[0].eq {
	pc = 0x82FF698C; continue 'dispatch;
	}
	// 82FF6978: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF697C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6980: 4BFFF299  bl 0x82ff5c18
	ctx.lr = 0x82FF6984;
	sub_82FF5C18(ctx, base);
	// 82FF6984: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6988: 48000008  b 0x82ff6990
	pc = 0x82FF6990; continue 'dispatch;
	// 82FF698C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6990: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6994: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6998: 480364B1  bl 0x8302ce48
	ctx.lr = 0x82FF699C;
	sub_8302CE48(ctx, base);
	// 82FF699C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF69A0: 4BFE18A9  bl 0x82fd8248
	ctx.lr = 0x82FF69A4;
	sub_82FD8248(ctx, base);
	// 82FF69A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF69A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF69AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF69B0: 3BCB7E70  addi r30, r11, 0x7e70
	ctx.r[30].s64 = ctx.r[11].s64 + 32368;
	// 82FF69B4: 41820018  beq 0x82ff69cc
	if ctx.cr[0].eq {
	pc = 0x82FF69CC; continue 'dispatch;
	}
	// 82FF69B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF69BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF69C0: 4BFFF259  bl 0x82ff5c18
	ctx.lr = 0x82FF69C4;
	sub_82FF5C18(ctx, base);
	// 82FF69C4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF69C8: 48000008  b 0x82ff69d0
	pc = 0x82FF69D0; continue 'dispatch;
	// 82FF69CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF69D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF69D4: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF69D8: 48036471  bl 0x8302ce48
	ctx.lr = 0x82FF69DC;
	sub_8302CE48(ctx, base);
	// 82FF69DC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF69E0: 4BFE1869  bl 0x82fd8248
	ctx.lr = 0x82FF69E4;
	sub_82FD8248(ctx, base);
	// 82FF69E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF69E8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF69EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF69F0: 3BCB7D5C  addi r30, r11, 0x7d5c
	ctx.r[30].s64 = ctx.r[11].s64 + 32092;
	// 82FF69F4: 41820018  beq 0x82ff6a0c
	if ctx.cr[0].eq {
	pc = 0x82FF6A0C; continue 'dispatch;
	}
	// 82FF69F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF69FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6A00: 4BFFF2F9  bl 0x82ff5cf8
	ctx.lr = 0x82FF6A04;
	sub_82FF5CF8(ctx, base);
	// 82FF6A04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF6A08: 48000008  b 0x82ff6a10
	pc = 0x82FF6A10; continue 'dispatch;
	// 82FF6A0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6A10: 817AB8FC  lwz r11, -0x4704(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6A14: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FF6A18: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF6A1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6A20: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6A28: 4E800421  bctrl
	ctx.lr = 0x82FF6A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6A2C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6A30: 4BFE1819  bl 0x82fd8248
	ctx.lr = 0x82FF6A34;
	sub_82FD8248(ctx, base);
	// 82FF6A34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6A38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6A3C: 41820018  beq 0x82ff6a54
	if ctx.cr[0].eq {
	pc = 0x82FF6A54; continue 'dispatch;
	}
	// 82FF6A40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF6A44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6A48: 4BFFF2B1  bl 0x82ff5cf8
	ctx.lr = 0x82FF6A4C;
	sub_82FF5CF8(ctx, base);
	// 82FF6A4C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6A50: 48000008  b 0x82ff6a58
	pc = 0x82FF6A58; continue 'dispatch;
	// 82FF6A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6A5C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6A60: 480363E9  bl 0x8302ce48
	ctx.lr = 0x82FF6A64;
	sub_8302CE48(ctx, base);
	// 82FF6A64: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6A68: 4BFE17E1  bl 0x82fd8248
	ctx.lr = 0x82FF6A6C;
	sub_82FD8248(ctx, base);
	// 82FF6A6C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6A70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6A74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6A78: 3BCB7D74  addi r30, r11, 0x7d74
	ctx.r[30].s64 = ctx.r[11].s64 + 32116;
	// 82FF6A7C: 41820018  beq 0x82ff6a94
	if ctx.cr[0].eq {
	pc = 0x82FF6A94; continue 'dispatch;
	}
	// 82FF6A80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF6A84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6A88: 4BFFF271  bl 0x82ff5cf8
	ctx.lr = 0x82FF6A8C;
	sub_82FF5CF8(ctx, base);
	// 82FF6A8C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6A90: 48000008  b 0x82ff6a98
	pc = 0x82FF6A98; continue 'dispatch;
	// 82FF6A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6A98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6A9C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6AA0: 480363A9  bl 0x8302ce48
	ctx.lr = 0x82FF6AA4;
	sub_8302CE48(ctx, base);
	// 82FF6AA4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6AA8: 4BFE17A1  bl 0x82fd8248
	ctx.lr = 0x82FF6AAC;
	sub_82FD8248(ctx, base);
	// 82FF6AAC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6AB0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6AB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6AB8: 3BCB7E2C  addi r30, r11, 0x7e2c
	ctx.r[30].s64 = ctx.r[11].s64 + 32300;
	// 82FF6ABC: 41820018  beq 0x82ff6ad4
	if ctx.cr[0].eq {
	pc = 0x82FF6AD4; continue 'dispatch;
	}
	// 82FF6AC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6AC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6AC8: 4BFFF151  bl 0x82ff5c18
	ctx.lr = 0x82FF6ACC;
	sub_82FF5C18(ctx, base);
	// 82FF6ACC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF6AD0: 48000008  b 0x82ff6ad8
	pc = 0x82FF6AD8; continue 'dispatch;
	// 82FF6AD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6AD8: 817AB8FC  lwz r11, -0x4704(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6ADC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FF6AE0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF6AE4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6AE8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6AF0: 4E800421  bctrl
	ctx.lr = 0x82FF6AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6AF4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6AF8: 4BFE1751  bl 0x82fd8248
	ctx.lr = 0x82FF6AFC;
	sub_82FD8248(ctx, base);
	// 82FF6AFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6B00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6B04: 41820018  beq 0x82ff6b1c
	if ctx.cr[0].eq {
	pc = 0x82FF6B1C; continue 'dispatch;
	}
	// 82FF6B08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6B0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6B10: 4BFFF109  bl 0x82ff5c18
	ctx.lr = 0x82FF6B14;
	sub_82FF5C18(ctx, base);
	// 82FF6B14: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6B18: 48000008  b 0x82ff6b20
	pc = 0x82FF6B20; continue 'dispatch;
	// 82FF6B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6B24: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6B28: 48036321  bl 0x8302ce48
	ctx.lr = 0x82FF6B2C;
	sub_8302CE48(ctx, base);
	// 82FF6B2C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6B30: 4BFE1719  bl 0x82fd8248
	ctx.lr = 0x82FF6B34;
	sub_82FD8248(ctx, base);
	// 82FF6B34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6B38: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6B3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6B40: 3BCB7E44  addi r30, r11, 0x7e44
	ctx.r[30].s64 = ctx.r[11].s64 + 32324;
	// 82FF6B44: 41820018  beq 0x82ff6b5c
	if ctx.cr[0].eq {
	pc = 0x82FF6B5C; continue 'dispatch;
	}
	// 82FF6B48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6B4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6B50: 4BFFF0C9  bl 0x82ff5c18
	ctx.lr = 0x82FF6B54;
	sub_82FF5C18(ctx, base);
	// 82FF6B54: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6B58: 48000008  b 0x82ff6b60
	pc = 0x82FF6B60; continue 'dispatch;
	// 82FF6B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6B64: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6B68: 480362E1  bl 0x8302ce48
	ctx.lr = 0x82FF6B6C;
	sub_8302CE48(ctx, base);
	// 82FF6B6C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6B70: 4BFE16D9  bl 0x82fd8248
	ctx.lr = 0x82FF6B74;
	sub_82FD8248(ctx, base);
	// 82FF6B74: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6B78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6B7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6B80: 3BCB7D34  addi r30, r11, 0x7d34
	ctx.r[30].s64 = ctx.r[11].s64 + 32052;
	// 82FF6B84: 41820018  beq 0x82ff6b9c
	if ctx.cr[0].eq {
	pc = 0x82FF6B9C; continue 'dispatch;
	}
	// 82FF6B88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6B8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6B90: 4BFFF169  bl 0x82ff5cf8
	ctx.lr = 0x82FF6B94;
	sub_82FF5CF8(ctx, base);
	// 82FF6B94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF6B98: 48000008  b 0x82ff6ba0
	pc = 0x82FF6BA0; continue 'dispatch;
	// 82FF6B9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6BA0: 817AB8FC  lwz r11, -0x4704(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6BA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF6BA8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF6BAC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6BB0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6BB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6BB8: 4E800421  bctrl
	ctx.lr = 0x82FF6BBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6BBC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6BC0: 4BFE1689  bl 0x82fd8248
	ctx.lr = 0x82FF6BC4;
	sub_82FD8248(ctx, base);
	// 82FF6BC4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6BC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6BCC: 41820018  beq 0x82ff6be4
	if ctx.cr[0].eq {
	pc = 0x82FF6BE4; continue 'dispatch;
	}
	// 82FF6BD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6BD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6BD8: 4BFFF121  bl 0x82ff5cf8
	ctx.lr = 0x82FF6BDC;
	sub_82FF5CF8(ctx, base);
	// 82FF6BDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6BE0: 48000008  b 0x82ff6be8
	pc = 0x82FF6BE8; continue 'dispatch;
	// 82FF6BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6BE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6BEC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6BF0: 48036259  bl 0x8302ce48
	ctx.lr = 0x82FF6BF4;
	sub_8302CE48(ctx, base);
	// 82FF6BF4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6BF8: 4BFE1651  bl 0x82fd8248
	ctx.lr = 0x82FF6BFC;
	sub_82FD8248(ctx, base);
	// 82FF6BFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6C00: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6C04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6C08: 3BCB7D4C  addi r30, r11, 0x7d4c
	ctx.r[30].s64 = ctx.r[11].s64 + 32076;
	// 82FF6C0C: 41820018  beq 0x82ff6c24
	if ctx.cr[0].eq {
	pc = 0x82FF6C24; continue 'dispatch;
	}
	// 82FF6C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6C14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6C18: 4BFFF0E1  bl 0x82ff5cf8
	ctx.lr = 0x82FF6C1C;
	sub_82FF5CF8(ctx, base);
	// 82FF6C1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6C20: 48000008  b 0x82ff6c28
	pc = 0x82FF6C28; continue 'dispatch;
	// 82FF6C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6C28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6C2C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6C30: 48036219  bl 0x8302ce48
	ctx.lr = 0x82FF6C34;
	sub_8302CE48(ctx, base);
	// 82FF6C34: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6C38: 4BFE1611  bl 0x82fd8248
	ctx.lr = 0x82FF6C3C;
	sub_82FD8248(ctx, base);
	// 82FF6C3C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6C40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6C44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6C48: 3BCB7DE0  addi r30, r11, 0x7de0
	ctx.r[30].s64 = ctx.r[11].s64 + 32224;
	// 82FF6C4C: 41820018  beq 0x82ff6c64
	if ctx.cr[0].eq {
	pc = 0x82FF6C64; continue 'dispatch;
	}
	// 82FF6C50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6C54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6C58: 4BFFEFC1  bl 0x82ff5c18
	ctx.lr = 0x82FF6C5C;
	sub_82FF5C18(ctx, base);
	// 82FF6C5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6C60: 48000008  b 0x82ff6c68
	pc = 0x82FF6C68; continue 'dispatch;
	// 82FF6C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6C68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6C6C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6C70: 480361D9  bl 0x8302ce48
	ctx.lr = 0x82FF6C74;
	sub_8302CE48(ctx, base);
	// 82FF6C74: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6C78: 4BFE15D1  bl 0x82fd8248
	ctx.lr = 0x82FF6C7C;
	sub_82FD8248(ctx, base);
	// 82FF6C7C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6C80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6C84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6C88: 3BCB7DF0  addi r30, r11, 0x7df0
	ctx.r[30].s64 = ctx.r[11].s64 + 32240;
	// 82FF6C8C: 41820018  beq 0x82ff6ca4
	if ctx.cr[0].eq {
	pc = 0x82FF6CA4; continue 'dispatch;
	}
	// 82FF6C90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6C94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6C98: 4BFFEF81  bl 0x82ff5c18
	ctx.lr = 0x82FF6C9C;
	sub_82FF5C18(ctx, base);
	// 82FF6C9C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6CA0: 48000008  b 0x82ff6ca8
	pc = 0x82FF6CA8; continue 'dispatch;
	// 82FF6CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6CA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6CAC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6CB0: 48036199  bl 0x8302ce48
	ctx.lr = 0x82FF6CB4;
	sub_8302CE48(ctx, base);
	// 82FF6CB4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6CB8: 4BFE1591  bl 0x82fd8248
	ctx.lr = 0x82FF6CBC;
	sub_82FD8248(ctx, base);
	// 82FF6CBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6CC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6CC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6CC8: 3BCB7DFC  addi r30, r11, 0x7dfc
	ctx.r[30].s64 = ctx.r[11].s64 + 32252;
	// 82FF6CCC: 41820018  beq 0x82ff6ce4
	if ctx.cr[0].eq {
	pc = 0x82FF6CE4; continue 'dispatch;
	}
	// 82FF6CD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6CD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6CD8: 4BFFEF41  bl 0x82ff5c18
	ctx.lr = 0x82FF6CDC;
	sub_82FF5C18(ctx, base);
	// 82FF6CDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6CE0: 48000008  b 0x82ff6ce8
	pc = 0x82FF6CE8; continue 'dispatch;
	// 82FF6CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6CE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6CEC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6CF0: 48036159  bl 0x8302ce48
	ctx.lr = 0x82FF6CF4;
	sub_8302CE48(ctx, base);
	// 82FF6CF4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6CF8: 4BFE1551  bl 0x82fd8248
	ctx.lr = 0x82FF6CFC;
	sub_82FD8248(ctx, base);
	// 82FF6CFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6D00: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6D04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6D08: 3BCB7E0C  addi r30, r11, 0x7e0c
	ctx.r[30].s64 = ctx.r[11].s64 + 32268;
	// 82FF6D0C: 41820018  beq 0x82ff6d24
	if ctx.cr[0].eq {
	pc = 0x82FF6D24; continue 'dispatch;
	}
	// 82FF6D10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6D14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6D18: 4BFFEF01  bl 0x82ff5c18
	ctx.lr = 0x82FF6D1C;
	sub_82FF5C18(ctx, base);
	// 82FF6D1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6D20: 48000008  b 0x82ff6d28
	pc = 0x82FF6D28; continue 'dispatch;
	// 82FF6D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6D28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6D2C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6D30: 48036119  bl 0x8302ce48
	ctx.lr = 0x82FF6D34;
	sub_8302CE48(ctx, base);
	// 82FF6D34: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6D38: 4BFE1511  bl 0x82fd8248
	ctx.lr = 0x82FF6D3C;
	sub_82FD8248(ctx, base);
	// 82FF6D3C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6D40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6D44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6D48: 3BCB7E20  addi r30, r11, 0x7e20
	ctx.r[30].s64 = ctx.r[11].s64 + 32288;
	// 82FF6D4C: 41820018  beq 0x82ff6d64
	if ctx.cr[0].eq {
	pc = 0x82FF6D64; continue 'dispatch;
	}
	// 82FF6D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6D58: 4BFFEEC1  bl 0x82ff5c18
	ctx.lr = 0x82FF6D5C;
	sub_82FF5C18(ctx, base);
	// 82FF6D5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6D60: 48000008  b 0x82ff6d68
	pc = 0x82FF6D68; continue 'dispatch;
	// 82FF6D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6D68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6D6C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6D70: 480360D9  bl 0x8302ce48
	ctx.lr = 0x82FF6D74;
	sub_8302CE48(ctx, base);
	// 82FF6D74: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6D78: 4BFE14D1  bl 0x82fd8248
	ctx.lr = 0x82FF6D7C;
	sub_82FD8248(ctx, base);
	// 82FF6D7C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6D80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6D84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6D88: 3BCB7D10  addi r30, r11, 0x7d10
	ctx.r[30].s64 = ctx.r[11].s64 + 32016;
	// 82FF6D8C: 41820018  beq 0x82ff6da4
	if ctx.cr[0].eq {
	pc = 0x82FF6DA4; continue 'dispatch;
	}
	// 82FF6D90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6D94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6D98: 4BFFEF61  bl 0x82ff5cf8
	ctx.lr = 0x82FF6D9C;
	sub_82FF5CF8(ctx, base);
	// 82FF6D9C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6DA0: 48000008  b 0x82ff6da8
	pc = 0x82FF6DA8; continue 'dispatch;
	// 82FF6DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6DA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6DAC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6DB0: 48036099  bl 0x8302ce48
	ctx.lr = 0x82FF6DB4;
	sub_8302CE48(ctx, base);
	// 82FF6DB4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6DB8: 4BFE1491  bl 0x82fd8248
	ctx.lr = 0x82FF6DBC;
	sub_82FD8248(ctx, base);
	// 82FF6DBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6DC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6DC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6DC8: 3BCB7D1C  addi r30, r11, 0x7d1c
	ctx.r[30].s64 = ctx.r[11].s64 + 32028;
	// 82FF6DCC: 41820018  beq 0x82ff6de4
	if ctx.cr[0].eq {
	pc = 0x82FF6DE4; continue 'dispatch;
	}
	// 82FF6DD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6DD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6DD8: 4BFFEF21  bl 0x82ff5cf8
	ctx.lr = 0x82FF6DDC;
	sub_82FF5CF8(ctx, base);
	// 82FF6DDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6DE0: 48000008  b 0x82ff6de8
	pc = 0x82FF6DE8; continue 'dispatch;
	// 82FF6DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6DE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6DEC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6DF0: 48036059  bl 0x8302ce48
	ctx.lr = 0x82FF6DF4;
	sub_8302CE48(ctx, base);
	// 82FF6DF4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FF6DF8: 4BFE1451  bl 0x82fd8248
	ctx.lr = 0x82FF6DFC;
	sub_82FD8248(ctx, base);
	// 82FF6DFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF6E00: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6E04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF6E08: 3BCB7D28  addi r30, r11, 0x7d28
	ctx.r[30].s64 = ctx.r[11].s64 + 32040;
	// 82FF6E0C: 41820018  beq 0x82ff6e24
	if ctx.cr[0].eq {
	pc = 0x82FF6E24; continue 'dispatch;
	}
	// 82FF6E10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6E18: 4BFFEEE1  bl 0x82ff5cf8
	ctx.lr = 0x82FF6E1C;
	sub_82FF5CF8(ctx, base);
	// 82FF6E1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF6E20: 48000008  b 0x82ff6e28
	pc = 0x82FF6E28; continue 'dispatch;
	// 82FF6E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6E28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6E2C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6E30: 48036019  bl 0x8302ce48
	ctx.lr = 0x82FF6E34;
	sub_8302CE48(ctx, base);
	// 82FF6E34: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6E38: 4BFE1411  bl 0x82fd8248
	ctx.lr = 0x82FF6E3C;
	sub_82FD8248(ctx, base);
	// 82FF6E3C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6E40: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6E44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6E48: 3B6BEF40  addi r27, r11, -0x10c0
	ctx.r[27].s64 = ctx.r[11].s64 + -4288;
	// 82FF6E4C: 41820020  beq 0x82ff6e6c
	if ctx.cr[0].eq {
	pc = 0x82FF6E6C; continue 'dispatch;
	}
	// 82FF6E50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6E58: 388B7930  addi r4, r11, 0x7930
	ctx.r[4].s64 = ctx.r[11].s64 + 31024;
	// 82FF6E5C: 4BFFE9C5  bl 0x82ff5820
	ctx.lr = 0x82FF6E60;
	sub_82FF5820(ctx, base);
	// 82FF6E60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF6E64: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6E68: 48000008  b 0x82ff6e70
	pc = 0x82FF6E70; continue 'dispatch;
	// 82FF6E6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF6E70: 807AB8FC  lwz r3, -0x4704(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18180 as u32) ) } as u64;
	// 82FF6E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF6E7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF6E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF6E84: 4E800421  bctrl
	ctx.lr = 0x82FF6E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF6E88: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6E8C: 4BFE13BD  bl 0x82fd8248
	ctx.lr = 0x82FF6E90;
	sub_82FD8248(ctx, base);
	// 82FF6E90: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6E94: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6E98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6E9C: 3BAB7A20  addi r29, r11, 0x7a20
	ctx.r[29].s64 = ctx.r[11].s64 + 31264;
	// 82FF6EA0: 4182001C  beq 0x82ff6ebc
	if ctx.cr[0].eq {
	pc = 0x82FF6EBC; continue 'dispatch;
	}
	// 82FF6EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6EA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6EAC: 4BFFE975  bl 0x82ff5820
	ctx.lr = 0x82FF6EB0;
	sub_82FF5820(ctx, base);
	// 82FF6EB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6EB4: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6EB8: 48000008  b 0x82ff6ec0
	pc = 0x82FF6EC0; continue 'dispatch;
	// 82FF6EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6EC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6EC4: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6EC8: 48035F81  bl 0x8302ce48
	ctx.lr = 0x82FF6ECC;
	sub_8302CE48(ctx, base);
	// 82FF6ECC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6ED0: 4BFE1379  bl 0x82fd8248
	ctx.lr = 0x82FF6ED4;
	sub_82FD8248(ctx, base);
	// 82FF6ED4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6ED8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6EDC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6EE0: 3BAB7A3C  addi r29, r11, 0x7a3c
	ctx.r[29].s64 = ctx.r[11].s64 + 31292;
	// 82FF6EE4: 4182001C  beq 0x82ff6f00
	if ctx.cr[0].eq {
	pc = 0x82FF6F00; continue 'dispatch;
	}
	// 82FF6EE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6EEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6EF0: 4BFFE931  bl 0x82ff5820
	ctx.lr = 0x82FF6EF4;
	sub_82FF5820(ctx, base);
	// 82FF6EF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6EF8: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6EFC: 48000008  b 0x82ff6f04
	pc = 0x82FF6F04; continue 'dispatch;
	// 82FF6F00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6F04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6F08: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6F0C: 48035F3D  bl 0x8302ce48
	ctx.lr = 0x82FF6F10;
	sub_8302CE48(ctx, base);
	// 82FF6F10: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6F14: 4BFE1335  bl 0x82fd8248
	ctx.lr = 0x82FF6F18;
	sub_82FD8248(ctx, base);
	// 82FF6F18: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6F1C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6F20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6F24: 3B6BEF80  addi r27, r11, -0x1080
	ctx.r[27].s64 = ctx.r[11].s64 + -4224;
	// 82FF6F28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6F2C: 3BAB7A4C  addi r29, r11, 0x7a4c
	ctx.r[29].s64 = ctx.r[11].s64 + 31308;
	// 82FF6F30: 4182001C  beq 0x82ff6f4c
	if ctx.cr[0].eq {
	pc = 0x82FF6F4C; continue 'dispatch;
	}
	// 82FF6F34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6F38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6F3C: 4BFFE8E5  bl 0x82ff5820
	ctx.lr = 0x82FF6F40;
	sub_82FF5820(ctx, base);
	// 82FF6F40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6F44: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6F48: 48000008  b 0x82ff6f50
	pc = 0x82FF6F50; continue 'dispatch;
	// 82FF6F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6F50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6F54: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6F58: 48035EF1  bl 0x8302ce48
	ctx.lr = 0x82FF6F5C;
	sub_8302CE48(ctx, base);
	// 82FF6F5C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6F60: 4BFE12E9  bl 0x82fd8248
	ctx.lr = 0x82FF6F64;
	sub_82FD8248(ctx, base);
	// 82FF6F64: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6F68: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6F6C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6F70: 3BAB7A5C  addi r29, r11, 0x7a5c
	ctx.r[29].s64 = ctx.r[11].s64 + 31324;
	// 82FF6F74: 4182001C  beq 0x82ff6f90
	if ctx.cr[0].eq {
	pc = 0x82FF6F90; continue 'dispatch;
	}
	// 82FF6F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6F7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6F80: 4BFFE8A1  bl 0x82ff5820
	ctx.lr = 0x82FF6F84;
	sub_82FF5820(ctx, base);
	// 82FF6F84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6F88: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6F8C: 48000008  b 0x82ff6f94
	pc = 0x82FF6F94; continue 'dispatch;
	// 82FF6F90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6F94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6F98: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6F9C: 48035EAD  bl 0x8302ce48
	ctx.lr = 0x82FF6FA0;
	sub_8302CE48(ctx, base);
	// 82FF6FA0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6FA4: 4BFE12A5  bl 0x82fd8248
	ctx.lr = 0x82FF6FA8;
	sub_82FD8248(ctx, base);
	// 82FF6FA8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6FAC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6FB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF6FB4: 3B6BEFC0  addi r27, r11, -0x1040
	ctx.r[27].s64 = ctx.r[11].s64 + -4160;
	// 82FF6FB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF6FBC: 3BAB7A70  addi r29, r11, 0x7a70
	ctx.r[29].s64 = ctx.r[11].s64 + 31344;
	// 82FF6FC0: 4182001C  beq 0x82ff6fdc
	if ctx.cr[0].eq {
	pc = 0x82FF6FDC; continue 'dispatch;
	}
	// 82FF6FC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF6FC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6FCC: 4BFFE855  bl 0x82ff5820
	ctx.lr = 0x82FF6FD0;
	sub_82FF5820(ctx, base);
	// 82FF6FD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF6FD4: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF6FD8: 48000008  b 0x82ff6fe0
	pc = 0x82FF6FE0; continue 'dispatch;
	// 82FF6FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF6FE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF6FE4: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF6FE8: 48035E61  bl 0x8302ce48
	ctx.lr = 0x82FF6FEC;
	sub_8302CE48(ctx, base);
	// 82FF6FEC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF6FF0: 4BFE1259  bl 0x82fd8248
	ctx.lr = 0x82FF6FF4;
	sub_82FD8248(ctx, base);
	// 82FF6FF4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF6FF8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF6FFC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF7000: 3BAB7A80  addi r29, r11, 0x7a80
	ctx.r[29].s64 = ctx.r[11].s64 + 31360;
	// 82FF7004: 4182001C  beq 0x82ff7020
	if ctx.cr[0].eq {
	pc = 0x82FF7020; continue 'dispatch;
	}
	// 82FF7008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF700C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7010: 4BFFE811  bl 0x82ff5820
	ctx.lr = 0x82FF7014;
	sub_82FF5820(ctx, base);
	// 82FF7014: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF7018: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF701C: 48000008  b 0x82ff7024
	pc = 0x82FF7024; continue 'dispatch;
	// 82FF7020: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF7024: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7028: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF702C: 48035E1D  bl 0x8302ce48
	ctx.lr = 0x82FF7030;
	sub_8302CE48(ctx, base);
	// 82FF7030: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF7034: 4BFE1215  bl 0x82fd8248
	ctx.lr = 0x82FF7038;
	sub_82FD8248(ctx, base);
	// 82FF7038: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF703C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF7040: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF7044: 3BAB7A94  addi r29, r11, 0x7a94
	ctx.r[29].s64 = ctx.r[11].s64 + 31380;
	// 82FF7048: 4182001C  beq 0x82ff7064
	if ctx.cr[0].eq {
	pc = 0x82FF7064; continue 'dispatch;
	}
	// 82FF704C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7050: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7054: 4BFFE7CD  bl 0x82ff5820
	ctx.lr = 0x82FF7058;
	sub_82FF5820(ctx, base);
	// 82FF7058: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF705C: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF7060: 48000008  b 0x82ff7068
	pc = 0x82FF7068; continue 'dispatch;
	// 82FF7064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF7068: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF706C: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF7070: 48035DD9  bl 0x8302ce48
	ctx.lr = 0x82FF7074;
	sub_8302CE48(ctx, base);
	// 82FF7074: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF7078: 4BFE11D1  bl 0x82fd8248
	ctx.lr = 0x82FF707C;
	sub_82FD8248(ctx, base);
	// 82FF707C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF7080: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF7084: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF7088: 3BAB7AAC  addi r29, r11, 0x7aac
	ctx.r[29].s64 = ctx.r[11].s64 + 31404;
	// 82FF708C: 4182001C  beq 0x82ff70a8
	if ctx.cr[0].eq {
	pc = 0x82FF70A8; continue 'dispatch;
	}
	// 82FF7090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7094: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7098: 4BFFE789  bl 0x82ff5820
	ctx.lr = 0x82FF709C;
	sub_82FF5820(ctx, base);
	// 82FF709C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF70A0: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF70A4: 48000008  b 0x82ff70ac
	pc = 0x82FF70AC; continue 'dispatch;
	// 82FF70A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF70AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF70B0: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF70B4: 48035D95  bl 0x8302ce48
	ctx.lr = 0x82FF70B8;
	sub_8302CE48(ctx, base);
	// 82FF70B8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FF70BC: 4BFE118D  bl 0x82fd8248
	ctx.lr = 0x82FF70C0;
	sub_82FD8248(ctx, base);
	// 82FF70C0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF70C4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF70C8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF70CC: 3BAB7E94  addi r29, r11, 0x7e94
	ctx.r[29].s64 = ctx.r[11].s64 + 32404;
	// 82FF70D0: 41820024  beq 0x82ff70f4
	if ctx.cr[0].eq {
	pc = 0x82FF70F4; continue 'dispatch;
	}
	// 82FF70D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF70D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF70DC: 4BFFE745  bl 0x82ff5820
	ctx.lr = 0x82FF70E0;
	sub_82FF5820(ctx, base);
	// 82FF70E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF70E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF70E8: 396BF000  addi r11, r11, -0x1000
	ctx.r[11].s64 = ctx.r[11].s64 + -4096;
	// 82FF70EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF70F0: 48000008  b 0x82ff70f8
	pc = 0x82FF70F8; continue 'dispatch;
	// 82FF70F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF70F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF70FC: 807CB8F8  lwz r3, -0x4708(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF7100: 48035D49  bl 0x8302ce48
	ctx.lr = 0x82FF7104;
	sub_8302CE48(ctx, base);
	// 82FF7104: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF7108: 481B10A8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF710C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF710C size=40
    let mut pc: u32 = 0x82FF710C;
    'dispatch: loop {
        match pc {
            0x82FF710C => {
    //   block [0x82FF710C..0x82FF7134)
	// 82FF710C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF711C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7120: 4BFE11C1  bl 0x82fd82e0
	ctx.lr = 0x82FF7124;
	sub_82FD82E0(ctx, base);
	// 82FF7124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF712C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7134 size=40
    let mut pc: u32 = 0x82FF7134;
    'dispatch: loop {
        match pc {
            0x82FF7134 => {
    //   block [0x82FF7134..0x82FF715C)
	// 82FF7134: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7144: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7148: 4BFE1199  bl 0x82fd82e0
	ctx.lr = 0x82FF714C;
	sub_82FD82E0(ctx, base);
	// 82FF714C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF715C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF715C size=40
    let mut pc: u32 = 0x82FF715C;
    'dispatch: loop {
        match pc {
            0x82FF715C => {
    //   block [0x82FF715C..0x82FF7184)
	// 82FF715C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF716C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7170: 4BFE1171  bl 0x82fd82e0
	ctx.lr = 0x82FF7174;
	sub_82FD82E0(ctx, base);
	// 82FF7174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF717C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7184 size=40
    let mut pc: u32 = 0x82FF7184;
    'dispatch: loop {
        match pc {
            0x82FF7184 => {
    //   block [0x82FF7184..0x82FF71AC)
	// 82FF7184: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7194: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7198: 4BFE1149  bl 0x82fd82e0
	ctx.lr = 0x82FF719C;
	sub_82FD82E0(ctx, base);
	// 82FF719C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF71A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF71A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF71A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF71AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF71AC size=40
    let mut pc: u32 = 0x82FF71AC;
    'dispatch: loop {
        match pc {
            0x82FF71AC => {
    //   block [0x82FF71AC..0x82FF71D4)
	// 82FF71AC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF71B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF71B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF71B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF71BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF71C0: 4BFE1121  bl 0x82fd82e0
	ctx.lr = 0x82FF71C4;
	sub_82FD82E0(ctx, base);
	// 82FF71C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF71C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF71CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF71D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF71D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF71D4 size=40
    let mut pc: u32 = 0x82FF71D4;
    'dispatch: loop {
        match pc {
            0x82FF71D4 => {
    //   block [0x82FF71D4..0x82FF71FC)
	// 82FF71D4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF71D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF71DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF71E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF71E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF71E8: 4BFE10F9  bl 0x82fd82e0
	ctx.lr = 0x82FF71EC;
	sub_82FD82E0(ctx, base);
	// 82FF71EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF71F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF71F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF71F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF71FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF71FC size=40
    let mut pc: u32 = 0x82FF71FC;
    'dispatch: loop {
        match pc {
            0x82FF71FC => {
    //   block [0x82FF71FC..0x82FF7224)
	// 82FF71FC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF720C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7210: 4BFE10D1  bl 0x82fd82e0
	ctx.lr = 0x82FF7214;
	sub_82FD82E0(ctx, base);
	// 82FF7214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF721C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7224(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7224 size=40
    let mut pc: u32 = 0x82FF7224;
    'dispatch: loop {
        match pc {
            0x82FF7224 => {
    //   block [0x82FF7224..0x82FF724C)
	// 82FF7224: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7234: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7238: 4BFE10A9  bl 0x82fd82e0
	ctx.lr = 0x82FF723C;
	sub_82FD82E0(ctx, base);
	// 82FF723C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF724C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF724C size=40
    let mut pc: u32 = 0x82FF724C;
    'dispatch: loop {
        match pc {
            0x82FF724C => {
    //   block [0x82FF724C..0x82FF7274)
	// 82FF724C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF725C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7260: 4BFE1081  bl 0x82fd82e0
	ctx.lr = 0x82FF7264;
	sub_82FD82E0(ctx, base);
	// 82FF7264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF726C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7274 size=40
    let mut pc: u32 = 0x82FF7274;
    'dispatch: loop {
        match pc {
            0x82FF7274 => {
    //   block [0x82FF7274..0x82FF729C)
	// 82FF7274: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7284: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7288: 4BFE1059  bl 0x82fd82e0
	ctx.lr = 0x82FF728C;
	sub_82FD82E0(ctx, base);
	// 82FF728C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF729C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF729C size=40
    let mut pc: u32 = 0x82FF729C;
    'dispatch: loop {
        match pc {
            0x82FF729C => {
    //   block [0x82FF729C..0x82FF72C4)
	// 82FF729C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF72A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF72A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF72A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF72AC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF72B0: 4BFE1031  bl 0x82fd82e0
	ctx.lr = 0x82FF72B4;
	sub_82FD82E0(ctx, base);
	// 82FF72B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF72B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF72BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF72C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF72C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF72C4 size=40
    let mut pc: u32 = 0x82FF72C4;
    'dispatch: loop {
        match pc {
            0x82FF72C4 => {
    //   block [0x82FF72C4..0x82FF72EC)
	// 82FF72C4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF72C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF72CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF72D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF72D4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF72D8: 4BFE1009  bl 0x82fd82e0
	ctx.lr = 0x82FF72DC;
	sub_82FD82E0(ctx, base);
	// 82FF72DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF72E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF72E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF72E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF72EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF72EC size=40
    let mut pc: u32 = 0x82FF72EC;
    'dispatch: loop {
        match pc {
            0x82FF72EC => {
    //   block [0x82FF72EC..0x82FF7314)
	// 82FF72EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF72F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF72F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF72F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF72FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7300: 4BFE0FE1  bl 0x82fd82e0
	ctx.lr = 0x82FF7304;
	sub_82FD82E0(ctx, base);
	// 82FF7304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF730C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7314 size=40
    let mut pc: u32 = 0x82FF7314;
    'dispatch: loop {
        match pc {
            0x82FF7314 => {
    //   block [0x82FF7314..0x82FF733C)
	// 82FF7314: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7324: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7328: 4BFE0FB9  bl 0x82fd82e0
	ctx.lr = 0x82FF732C;
	sub_82FD82E0(ctx, base);
	// 82FF732C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF733C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF733C size=40
    let mut pc: u32 = 0x82FF733C;
    'dispatch: loop {
        match pc {
            0x82FF733C => {
    //   block [0x82FF733C..0x82FF7364)
	// 82FF733C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF734C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7350: 4BFE0F91  bl 0x82fd82e0
	ctx.lr = 0x82FF7354;
	sub_82FD82E0(ctx, base);
	// 82FF7354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF735C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7364 size=40
    let mut pc: u32 = 0x82FF7364;
    'dispatch: loop {
        match pc {
            0x82FF7364 => {
    //   block [0x82FF7364..0x82FF738C)
	// 82FF7364: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF736C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7374: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7378: 4BFE0F69  bl 0x82fd82e0
	ctx.lr = 0x82FF737C;
	sub_82FD82E0(ctx, base);
	// 82FF737C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF738C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF738C size=40
    let mut pc: u32 = 0x82FF738C;
    'dispatch: loop {
        match pc {
            0x82FF738C => {
    //   block [0x82FF738C..0x82FF73B4)
	// 82FF738C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF739C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF73A0: 4BFE0F41  bl 0x82fd82e0
	ctx.lr = 0x82FF73A4;
	sub_82FD82E0(ctx, base);
	// 82FF73A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF73A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF73AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF73B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF73B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF73B4 size=40
    let mut pc: u32 = 0x82FF73B4;
    'dispatch: loop {
        match pc {
            0x82FF73B4 => {
    //   block [0x82FF73B4..0x82FF73DC)
	// 82FF73B4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF73B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF73BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF73C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF73C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF73C8: 4BFE0F19  bl 0x82fd82e0
	ctx.lr = 0x82FF73CC;
	sub_82FD82E0(ctx, base);
	// 82FF73CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF73D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF73D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF73D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF73DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF73DC size=40
    let mut pc: u32 = 0x82FF73DC;
    'dispatch: loop {
        match pc {
            0x82FF73DC => {
    //   block [0x82FF73DC..0x82FF7404)
	// 82FF73DC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF73E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF73E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF73E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF73EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF73F0: 4BFE0EF1  bl 0x82fd82e0
	ctx.lr = 0x82FF73F4;
	sub_82FD82E0(ctx, base);
	// 82FF73F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF73F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF73FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7404 size=40
    let mut pc: u32 = 0x82FF7404;
    'dispatch: loop {
        match pc {
            0x82FF7404 => {
    //   block [0x82FF7404..0x82FF742C)
	// 82FF7404: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF740C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7414: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7418: 4BFE0EC9  bl 0x82fd82e0
	ctx.lr = 0x82FF741C;
	sub_82FD82E0(ctx, base);
	// 82FF741C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF742C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF742C size=40
    let mut pc: u32 = 0x82FF742C;
    'dispatch: loop {
        match pc {
            0x82FF742C => {
    //   block [0x82FF742C..0x82FF7454)
	// 82FF742C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF743C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7440: 4BFE0EA1  bl 0x82fd82e0
	ctx.lr = 0x82FF7444;
	sub_82FD82E0(ctx, base);
	// 82FF7444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF744C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7454 size=40
    let mut pc: u32 = 0x82FF7454;
    'dispatch: loop {
        match pc {
            0x82FF7454 => {
    //   block [0x82FF7454..0x82FF747C)
	// 82FF7454: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7464: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7468: 4BFE0E79  bl 0x82fd82e0
	ctx.lr = 0x82FF746C;
	sub_82FD82E0(ctx, base);
	// 82FF746C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF747C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF747C size=40
    let mut pc: u32 = 0x82FF747C;
    'dispatch: loop {
        match pc {
            0x82FF747C => {
    //   block [0x82FF747C..0x82FF74A4)
	// 82FF747C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF748C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7490: 4BFE0E51  bl 0x82fd82e0
	ctx.lr = 0x82FF7494;
	sub_82FD82E0(ctx, base);
	// 82FF7494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF749C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF74A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF74A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF74A4 size=40
    let mut pc: u32 = 0x82FF74A4;
    'dispatch: loop {
        match pc {
            0x82FF74A4 => {
    //   block [0x82FF74A4..0x82FF74CC)
	// 82FF74A4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF74A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF74AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF74B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF74B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF74B8: 4BFE0E29  bl 0x82fd82e0
	ctx.lr = 0x82FF74BC;
	sub_82FD82E0(ctx, base);
	// 82FF74BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF74C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF74C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF74C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF74CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF74CC size=40
    let mut pc: u32 = 0x82FF74CC;
    'dispatch: loop {
        match pc {
            0x82FF74CC => {
    //   block [0x82FF74CC..0x82FF74F4)
	// 82FF74CC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF74D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF74D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF74D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF74DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF74E0: 4BFE0E01  bl 0x82fd82e0
	ctx.lr = 0x82FF74E4;
	sub_82FD82E0(ctx, base);
	// 82FF74E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF74E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF74EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF74F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF74F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF74F4 size=40
    let mut pc: u32 = 0x82FF74F4;
    'dispatch: loop {
        match pc {
            0x82FF74F4 => {
    //   block [0x82FF74F4..0x82FF751C)
	// 82FF74F4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF74F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF74FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7504: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7508: 4BFE0DD9  bl 0x82fd82e0
	ctx.lr = 0x82FF750C;
	sub_82FD82E0(ctx, base);
	// 82FF750C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF751C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF751C size=40
    let mut pc: u32 = 0x82FF751C;
    'dispatch: loop {
        match pc {
            0x82FF751C => {
    //   block [0x82FF751C..0x82FF7544)
	// 82FF751C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF752C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7530: 4BFE0DB1  bl 0x82fd82e0
	ctx.lr = 0x82FF7534;
	sub_82FD82E0(ctx, base);
	// 82FF7534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7544 size=40
    let mut pc: u32 = 0x82FF7544;
    'dispatch: loop {
        match pc {
            0x82FF7544 => {
    //   block [0x82FF7544..0x82FF756C)
	// 82FF7544: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7554: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7558: 4BFE0D89  bl 0x82fd82e0
	ctx.lr = 0x82FF755C;
	sub_82FD82E0(ctx, base);
	// 82FF755C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF756C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF756C size=40
    let mut pc: u32 = 0x82FF756C;
    'dispatch: loop {
        match pc {
            0x82FF756C => {
    //   block [0x82FF756C..0x82FF7594)
	// 82FF756C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF757C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7580: 4BFE0D61  bl 0x82fd82e0
	ctx.lr = 0x82FF7584;
	sub_82FD82E0(ctx, base);
	// 82FF7584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF758C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7594(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7594 size=40
    let mut pc: u32 = 0x82FF7594;
    'dispatch: loop {
        match pc {
            0x82FF7594 => {
    //   block [0x82FF7594..0x82FF75BC)
	// 82FF7594: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF75A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF75A4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF75A8: 4BFE0D39  bl 0x82fd82e0
	ctx.lr = 0x82FF75AC;
	sub_82FD82E0(ctx, base);
	// 82FF75AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF75B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF75B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF75B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF75BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF75BC size=40
    let mut pc: u32 = 0x82FF75BC;
    'dispatch: loop {
        match pc {
            0x82FF75BC => {
    //   block [0x82FF75BC..0x82FF75E4)
	// 82FF75BC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF75C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF75C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF75C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF75CC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF75D0: 4BFE0D11  bl 0x82fd82e0
	ctx.lr = 0x82FF75D4;
	sub_82FD82E0(ctx, base);
	// 82FF75D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF75D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF75DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF75E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF75E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF75E4 size=40
    let mut pc: u32 = 0x82FF75E4;
    'dispatch: loop {
        match pc {
            0x82FF75E4 => {
    //   block [0x82FF75E4..0x82FF760C)
	// 82FF75E4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF75E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF75EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF75F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF75F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF75F8: 4BFE0CE9  bl 0x82fd82e0
	ctx.lr = 0x82FF75FC;
	sub_82FD82E0(ctx, base);
	// 82FF75FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF760C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF760C size=40
    let mut pc: u32 = 0x82FF760C;
    'dispatch: loop {
        match pc {
            0x82FF760C => {
    //   block [0x82FF760C..0x82FF7634)
	// 82FF760C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF761C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7620: 4BFE0CC1  bl 0x82fd82e0
	ctx.lr = 0x82FF7624;
	sub_82FD82E0(ctx, base);
	// 82FF7624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF762C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7634 size=40
    let mut pc: u32 = 0x82FF7634;
    'dispatch: loop {
        match pc {
            0x82FF7634 => {
    //   block [0x82FF7634..0x82FF765C)
	// 82FF7634: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF763C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7644: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7648: 4BFE0C99  bl 0x82fd82e0
	ctx.lr = 0x82FF764C;
	sub_82FD82E0(ctx, base);
	// 82FF764C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF765C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF765C size=40
    let mut pc: u32 = 0x82FF765C;
    'dispatch: loop {
        match pc {
            0x82FF765C => {
    //   block [0x82FF765C..0x82FF7684)
	// 82FF765C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF766C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7670: 4BFE0C71  bl 0x82fd82e0
	ctx.lr = 0x82FF7674;
	sub_82FD82E0(ctx, base);
	// 82FF7674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF767C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7684 size=40
    let mut pc: u32 = 0x82FF7684;
    'dispatch: loop {
        match pc {
            0x82FF7684 => {
    //   block [0x82FF7684..0x82FF76AC)
	// 82FF7684: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF768C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7694: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7698: 4BFE0C49  bl 0x82fd82e0
	ctx.lr = 0x82FF769C;
	sub_82FD82E0(ctx, base);
	// 82FF769C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF76A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF76A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF76A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF76AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF76AC size=40
    let mut pc: u32 = 0x82FF76AC;
    'dispatch: loop {
        match pc {
            0x82FF76AC => {
    //   block [0x82FF76AC..0x82FF76D4)
	// 82FF76AC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF76B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF76B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF76B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF76BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF76C0: 4BFE0C21  bl 0x82fd82e0
	ctx.lr = 0x82FF76C4;
	sub_82FD82E0(ctx, base);
	// 82FF76C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF76C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF76CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF76D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF76D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF76D4 size=40
    let mut pc: u32 = 0x82FF76D4;
    'dispatch: loop {
        match pc {
            0x82FF76D4 => {
    //   block [0x82FF76D4..0x82FF76FC)
	// 82FF76D4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF76E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF76E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF76E8: 4BFE0BF9  bl 0x82fd82e0
	ctx.lr = 0x82FF76EC;
	sub_82FD82E0(ctx, base);
	// 82FF76EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF76F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF76F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF76F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF76FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF76FC size=40
    let mut pc: u32 = 0x82FF76FC;
    'dispatch: loop {
        match pc {
            0x82FF76FC => {
    //   block [0x82FF76FC..0x82FF7724)
	// 82FF76FC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF770C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7710: 4BFE0BD1  bl 0x82fd82e0
	ctx.lr = 0x82FF7714;
	sub_82FD82E0(ctx, base);
	// 82FF7714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7724 size=40
    let mut pc: u32 = 0x82FF7724;
    'dispatch: loop {
        match pc {
            0x82FF7724 => {
    //   block [0x82FF7724..0x82FF774C)
	// 82FF7724: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7734: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7738: 4BFE0BA9  bl 0x82fd82e0
	ctx.lr = 0x82FF773C;
	sub_82FD82E0(ctx, base);
	// 82FF773C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF774C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF774C size=40
    let mut pc: u32 = 0x82FF774C;
    'dispatch: loop {
        match pc {
            0x82FF774C => {
    //   block [0x82FF774C..0x82FF7774)
	// 82FF774C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF775C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7760: 4BFE0B81  bl 0x82fd82e0
	ctx.lr = 0x82FF7764;
	sub_82FD82E0(ctx, base);
	// 82FF7764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7774 size=40
    let mut pc: u32 = 0x82FF7774;
    'dispatch: loop {
        match pc {
            0x82FF7774 => {
    //   block [0x82FF7774..0x82FF779C)
	// 82FF7774: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7784: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7788: 4BFE0B59  bl 0x82fd82e0
	ctx.lr = 0x82FF778C;
	sub_82FD82E0(ctx, base);
	// 82FF778C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF779C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF779C size=40
    let mut pc: u32 = 0x82FF779C;
    'dispatch: loop {
        match pc {
            0x82FF779C => {
    //   block [0x82FF779C..0x82FF77C4)
	// 82FF779C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF77A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF77A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF77A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF77AC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF77B0: 4BFE0B31  bl 0x82fd82e0
	ctx.lr = 0x82FF77B4;
	sub_82FD82E0(ctx, base);
	// 82FF77B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF77B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF77BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF77C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF77C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF77C4 size=40
    let mut pc: u32 = 0x82FF77C4;
    'dispatch: loop {
        match pc {
            0x82FF77C4 => {
    //   block [0x82FF77C4..0x82FF77EC)
	// 82FF77C4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF77C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF77CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF77D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF77D4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF77D8: 4BFE0B09  bl 0x82fd82e0
	ctx.lr = 0x82FF77DC;
	sub_82FD82E0(ctx, base);
	// 82FF77DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF77E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF77E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF77E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF77EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF77EC size=40
    let mut pc: u32 = 0x82FF77EC;
    'dispatch: loop {
        match pc {
            0x82FF77EC => {
    //   block [0x82FF77EC..0x82FF7814)
	// 82FF77EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF77F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF77F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF77F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF77FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7800: 4BFE0AE1  bl 0x82fd82e0
	ctx.lr = 0x82FF7804;
	sub_82FD82E0(ctx, base);
	// 82FF7804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF780C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7814 size=40
    let mut pc: u32 = 0x82FF7814;
    'dispatch: loop {
        match pc {
            0x82FF7814 => {
    //   block [0x82FF7814..0x82FF783C)
	// 82FF7814: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF781C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7824: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7828: 4BFE0AB9  bl 0x82fd82e0
	ctx.lr = 0x82FF782C;
	sub_82FD82E0(ctx, base);
	// 82FF782C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF783C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF783C size=40
    let mut pc: u32 = 0x82FF783C;
    'dispatch: loop {
        match pc {
            0x82FF783C => {
    //   block [0x82FF783C..0x82FF7864)
	// 82FF783C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF784C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7850: 4BFE0A91  bl 0x82fd82e0
	ctx.lr = 0x82FF7854;
	sub_82FD82E0(ctx, base);
	// 82FF7854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF785C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7864 size=40
    let mut pc: u32 = 0x82FF7864;
    'dispatch: loop {
        match pc {
            0x82FF7864 => {
    //   block [0x82FF7864..0x82FF788C)
	// 82FF7864: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7874: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7878: 4BFE0A69  bl 0x82fd82e0
	ctx.lr = 0x82FF787C;
	sub_82FD82E0(ctx, base);
	// 82FF787C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF788C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF788C size=40
    let mut pc: u32 = 0x82FF788C;
    'dispatch: loop {
        match pc {
            0x82FF788C => {
    //   block [0x82FF788C..0x82FF78B4)
	// 82FF788C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF789C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF78A0: 4BFE0A41  bl 0x82fd82e0
	ctx.lr = 0x82FF78A4;
	sub_82FD82E0(ctx, base);
	// 82FF78A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF78A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF78AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF78B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF78B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF78B4 size=40
    let mut pc: u32 = 0x82FF78B4;
    'dispatch: loop {
        match pc {
            0x82FF78B4 => {
    //   block [0x82FF78B4..0x82FF78DC)
	// 82FF78B4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF78B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF78BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF78C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF78C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF78C8: 4BFE0A19  bl 0x82fd82e0
	ctx.lr = 0x82FF78CC;
	sub_82FD82E0(ctx, base);
	// 82FF78CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF78D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF78D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF78D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF78DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF78DC size=40
    let mut pc: u32 = 0x82FF78DC;
    'dispatch: loop {
        match pc {
            0x82FF78DC => {
    //   block [0x82FF78DC..0x82FF7904)
	// 82FF78DC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF78E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF78E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF78E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF78EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF78F0: 4BFE09F1  bl 0x82fd82e0
	ctx.lr = 0x82FF78F4;
	sub_82FD82E0(ctx, base);
	// 82FF78F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7904 size=40
    let mut pc: u32 = 0x82FF7904;
    'dispatch: loop {
        match pc {
            0x82FF7904 => {
    //   block [0x82FF7904..0x82FF792C)
	// 82FF7904: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7914: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7918: 4BFE09C9  bl 0x82fd82e0
	ctx.lr = 0x82FF791C;
	sub_82FD82E0(ctx, base);
	// 82FF791C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7930 size=84
    let mut pc: u32 = 0x82FF7930;
    'dispatch: loop {
        match pc {
            0x82FF7930 => {
    //   block [0x82FF7930..0x82FF7984)
	// 82FF7930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF793C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7944: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF7948: 83FEB8F8  lwz r31, -0x4708(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18184 as u32) ) } as u64;
	// 82FF794C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FF7950: 419A0014  beq cr6, 0x82ff7964
	if ctx.cr[6].eq {
	pc = 0x82FF7964; continue 'dispatch;
	}
	// 82FF7954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7958: 4BFEF311  bl 0x82fe6c68
	ctx.lr = 0x82FF795C;
	sub_82FE6C68(ctx, base);
	// 82FF795C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7960: 4BFE0981  bl 0x82fd82e0
	ctx.lr = 0x82FF7964;
	sub_82FD82E0(ctx, base);
	// 82FF7964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF7968: 917EB8F8  stw r11, -0x4708(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18184 as u32), ctx.r[11].u32 ) };
	// 82FF796C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF7970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7978: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF797C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7988 size=8
    let mut pc: u32 = 0x82FF7988;
    'dispatch: loop {
        match pc {
            0x82FF7988 => {
    //   block [0x82FF7988..0x82FF7990)
	// 82FF7988: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF798C: 8213F560  lwz r16, -0xaa0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7990 size=344
    let mut pc: u32 = 0x82FF7990;
    'dispatch: loop {
        match pc {
            0x82FF7990 => {
    //   block [0x82FF7990..0x82FF7AE8)
	// 82FF7990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7994: 481B07D1  bl 0x831a8164
	ctx.lr = 0x82FF7998;
	sub_831A8130(ctx, base);
	// 82FF7998: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF799C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF79A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF79A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FF79A8: 396BED60  addi r11, r11, -0x12a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4768;
	// 82FF79AC: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 82FF79B0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF79B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF79B8: 3BABB8F8  addi r29, r11, -0x4708
	ctx.r[29].s64 = ctx.r[11].s64 + -18184;
	// 82FF79BC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF79C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF79C4: 409A0078  bne cr6, 0x82ff7a3c
	if !ctx.cr[6].eq {
	pc = 0x82FF7A3C; continue 'dispatch;
	}
	// 82FF79C8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF79CC: 4BFE087D  bl 0x82fd8248
	ctx.lr = 0x82FF79D0;
	sub_82FD8248(ctx, base);
	// 82FF79D0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FF79D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF79D8: 41820018  beq 0x82ff79f0
	if ctx.cr[0].eq {
	pc = 0x82FF79F0; continue 'dispatch;
	}
	// 82FF79DC: 38800067  li r4, 0x67
	ctx.r[4].s64 = 103;
	// 82FF79E0: 80BCB7E8  lwz r5, -0x4818(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF79E4: 48072C25  bl 0x8306a608
	ctx.lr = 0x82FF79E8;
	sub_8306A608(ctx, base);
	// 82FF79E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF79EC: 48000008  b 0x82ff79f4
	pc = 0x82FF79F4; continue 'dispatch;
	// 82FF79F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF79F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF79F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF79FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF7A00: 4BFFD541  bl 0x82ff4f40
	ctx.lr = 0x82FF7A04;
	sub_82FF4F40(ctx, base);
	// 82FF7A04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7A08: 41820020  beq 0x82ff7a28
	if ctx.cr[0].eq {
	pc = 0x82FF7A28; continue 'dispatch;
	}
	// 82FF7A0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF7A10: 419A002C  beq cr6, 0x82ff7a3c
	if ctx.cr[6].eq {
	pc = 0x82FF7A3C; continue 'dispatch;
	}
	// 82FF7A14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7A18: 4BFEF251  bl 0x82fe6c68
	ctx.lr = 0x82FF7A1C;
	sub_82FE6C68(ctx, base);
	// 82FF7A1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7A20: 4BFE08C1  bl 0x82fd82e0
	ctx.lr = 0x82FF7A24;
	sub_82FD82E0(ctx, base);
	// 82FF7A24: 48000018  b 0x82ff7a3c
	pc = 0x82FF7A3C; continue 'dispatch;
	// 82FF7A28: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FF7A2C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FF7A30: 388B7930  addi r4, r11, 0x7930
	ctx.r[4].s64 = ctx.r[11].s64 + 31024;
	// 82FF7A34: 386AB90C  addi r3, r10, -0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18164;
	// 82FF7A38: 48000101  bl 0x82ff7b38
	ctx.lr = 0x82FF7A3C;
	sub_82FF7B38(ctx, base);
	// 82FF7A3C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF7A40: 3BABB8FC  addi r29, r11, -0x4704
	ctx.r[29].s64 = ctx.r[11].s64 + -18180;
	// 82FF7A44: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF7A48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF7A4C: 409A0090  bne cr6, 0x82ff7adc
	if !ctx.cr[6].eq {
	pc = 0x82FF7ADC; continue 'dispatch;
	}
	// 82FF7A50: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FF7A54: 4BFE07F5  bl 0x82fd8248
	ctx.lr = 0x82FF7A58;
	sub_82FD8248(ctx, base);
	// 82FF7A58: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF7A5C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF7A60: 41820028  beq 0x82ff7a88
	if ctx.cr[0].eq {
	pc = 0x82FF7A88; continue 'dispatch;
	}
	// 82FF7A64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF7A68: 80DCB7E8  lwz r6, -0x4818(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF7A6C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FF7A70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7A74: 48054D7D  bl 0x8304c7f0
	ctx.lr = 0x82FF7A78;
	sub_8304C7F0(ctx, base);
	// 82FF7A78: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FF7A7C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FF7A80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7A84: 48000008  b 0x82ff7a8c
	pc = 0x82FF7A8C; continue 'dispatch;
	// 82FF7A88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF7A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF7A90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF7A94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF7A98: 4BFFD4A9  bl 0x82ff4f40
	ctx.lr = 0x82FF7A9C;
	sub_82FF4F40(ctx, base);
	// 82FF7A9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7AA0: 41820028  beq 0x82ff7ac8
	if ctx.cr[0].eq {
	pc = 0x82FF7AC8; continue 'dispatch;
	}
	// 82FF7AA4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF7AA8: 419A0034  beq cr6, 0x82ff7adc
	if ctx.cr[6].eq {
	pc = 0x82FF7ADC; continue 'dispatch;
	}
	// 82FF7AAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF7AB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF7AB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7AB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF7ABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF7AC0: 4E800421  bctrl
	ctx.lr = 0x82FF7AC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF7AC4: 48000018  b 0x82ff7adc
	pc = 0x82FF7ADC; continue 'dispatch;
	// 82FF7AC8: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FF7ACC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FF7AD0: 388B5870  addi r4, r11, 0x5870
	ctx.r[4].s64 = ctx.r[11].s64 + 22640;
	// 82FF7AD4: 386AB900  addi r3, r10, -0x4700
	ctx.r[3].s64 = ctx.r[10].s64 + -18176;
	// 82FF7AD8: 48000061  bl 0x82ff7b38
	ctx.lr = 0x82FF7ADC;
	sub_82FF7B38(ctx, base);
	// 82FF7ADC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FF7AE0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF7AE4: 481B06D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7AE8 size=40
    let mut pc: u32 = 0x82FF7AE8;
    'dispatch: loop {
        match pc {
            0x82FF7AE8 => {
    //   block [0x82FF7AE8..0x82FF7B10)
	// 82FF7AE8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7AEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7AF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7AF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7AF8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7AFC: 4BFE07E5  bl 0x82fd82e0
	ctx.lr = 0x82FF7B00;
	sub_82FD82E0(ctx, base);
	// 82FF7B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7B10 size=40
    let mut pc: u32 = 0x82FF7B10;
    'dispatch: loop {
        match pc {
            0x82FF7B10 => {
    //   block [0x82FF7B10..0x82FF7B38)
	// 82FF7B10: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7B14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7B18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7B20: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF7B24: 4BFE07BD  bl 0x82fd82e0
	ctx.lr = 0x82FF7B28;
	sub_82FD82E0(ctx, base);
	// 82FF7B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7B38 size=128
    let mut pc: u32 = 0x82FF7B38;
    'dispatch: loop {
        match pc {
            0x82FF7B38 => {
    //   block [0x82FF7B38..0x82FF7BB8)
	// 82FF7B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF7B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7B4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF7B50: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF7B54: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FF7B58: 807EB7D4  lwz r3, -0x482c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18476 as u32) ) } as u64;
	// 82FF7B5C: 4BFFDC6D  bl 0x82ff57c8
	ctx.lr = 0x82FF7B60;
	sub_82FF57C8(ctx, base);
	// 82FF7B60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF7B68: 409A0030  bne cr6, 0x82ff7b98
	if !ctx.cr[6].eq {
	pc = 0x82FF7B98; continue 'dispatch;
	}
	// 82FF7B6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7B70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FF7B74: 409A0024  bne cr6, 0x82ff7b98
	if !ctx.cr[6].eq {
	pc = 0x82FF7B98; continue 'dispatch;
	}
	// 82FF7B78: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF7B7C: 814BB7D0  lwz r10, -0x4830(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18480 as u32) ) } as u64;
	// 82FF7B80: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FF7B84: 93EBB7D0  stw r31, -0x4830(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18480 as u32), ctx.r[31].u32 ) };
	// 82FF7B88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7B8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7B90: 41820008  beq 0x82ff7b98
	if ctx.cr[0].eq {
	pc = 0x82FF7B98; continue 'dispatch;
	}
	// 82FF7B94: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82FF7B98: 807EB7D4  lwz r3, -0x482c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18476 as u32) ) } as u64;
	// 82FF7B9C: 4BFFDC35  bl 0x82ff57d0
	ctx.lr = 0x82FF7BA0;
	sub_82FF57D0(ctx, base);
	// 82FF7BA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF7BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7BAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF7BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7BB8 size=140
    let mut pc: u32 = 0x82FF7BB8;
    'dispatch: loop {
        match pc {
            0x82FF7BB8 => {
    //   block [0x82FF7BB8..0x82FF7C44)
	// 82FF7BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7BC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF7BC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7BCC: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF7BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF7BD4: 807EB7D4  lwz r3, -0x482c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18476 as u32) ) } as u64;
	// 82FF7BD8: 4BFFDBF1  bl 0x82ff57c8
	ctx.lr = 0x82FF7BDC;
	sub_82FF57C8(ctx, base);
	// 82FF7BDC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7BE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7BE4: 4182000C  beq 0x82ff7bf0
	if ctx.cr[0].eq {
	pc = 0x82FF7BF0; continue 'dispatch;
	}
	// 82FF7BE8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7BEC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FF7BF0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7BF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7BF8: 40820014  bne 0x82ff7c0c
	if !ctx.cr[0].eq {
	pc = 0x82FF7C0C; continue 'dispatch;
	}
	// 82FF7BFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7C00: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FF7C04: 916AB7D0  stw r11, -0x4830(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18480 as u32), ctx.r[11].u32 ) };
	// 82FF7C08: 4800000C  b 0x82ff7c14
	pc = 0x82FF7C14; continue 'dispatch;
	// 82FF7C0C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7C10: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FF7C14: 807EB7D4  lwz r3, -0x482c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18476 as u32) ) } as u64;
	// 82FF7C18: 4BFFDBB9  bl 0x82ff57d0
	ctx.lr = 0x82FF7C1C;
	sub_82FF57D0(ctx, base);
	// 82FF7C1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF7C20: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FF7C24: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF7C28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7C2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF7C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7C38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF7C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7C48 size=20
    let mut pc: u32 = 0x82FF7C48;
    'dispatch: loop {
        match pc {
            0x82FF7C48 => {
    //   block [0x82FF7C48..0x82FF7C5C)
	// 82FF7C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF7C4C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FF7C50: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF7C54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7C60 size=68
    let mut pc: u32 = 0x82FF7C60;
    'dispatch: loop {
        match pc {
            0x82FF7C60 => {
    //   block [0x82FF7C60..0x82FF7CA4)
	// 82FF7C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF7C74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF7C78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7C7C: 4182000C  beq 0x82ff7c88
	if ctx.cr[0].eq {
	pc = 0x82FF7C88; continue 'dispatch;
	}
	// 82FF7C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF7C84: 4E800421  bctrl
	ctx.lr = 0x82FF7C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF7C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7C8C: 4BFFFF2D  bl 0x82ff7bb8
	ctx.lr = 0x82FF7C90;
	sub_82FF7BB8(ctx, base);
	// 82FF7C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7CA8 size=8
    let mut pc: u32 = 0x82FF7CA8;
    'dispatch: loop {
        match pc {
            0x82FF7CA8 => {
    //   block [0x82FF7CA8..0x82FF7CB0)
	// 82FF7CA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF7CAC: 8213F5C8  lwz r16, -0xa38(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7CB0 size=88
    let mut pc: u32 = 0x82FF7CB0;
    'dispatch: loop {
        match pc {
            0x82FF7CB0 => {
    //   block [0x82FF7CB0..0x82FF7D08)
	// 82FF7CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7CB4: 481B04B5  bl 0x831a8168
	ctx.lr = 0x82FF7CB8;
	sub_831A8130(ctx, base);
	// 82FF7CB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF7CBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7CC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF7CC4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF7CC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF7CCC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FF7CD0: 480020E9  bl 0x82ff9db8
	ctx.lr = 0x82FF7CD4;
	sub_82FF9DB8(ctx, base);
	// 82FF7CD4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF7CDC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FF7CE0: 396BF5B0  addi r11, r11, -0xa50
	ctx.r[11].s64 = ctx.r[11].s64 + -2640;
	// 82FF7CE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7CE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF7CEC: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FF7CF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7CF4: 4BFFCE5D  bl 0x82ff4b50
	ctx.lr = 0x82FF7CF8;
	sub_82FF4B50(ctx, base);
	// 82FF7CF8: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FF7CFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7D00: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF7D04: 481B04B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7D08 size=40
    let mut pc: u32 = 0x82FF7D08;
    'dispatch: loop {
        match pc {
            0x82FF7D08 => {
    //   block [0x82FF7D08..0x82FF7D30)
	// 82FF7D08: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF7D0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7D10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7D18: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FF7D1C: 4800209D  bl 0x82ff9db8
	ctx.lr = 0x82FF7D20;
	sub_82FF9DB8(ctx, base);
	// 82FF7D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7D30 size=8
    let mut pc: u32 = 0x82FF7D30;
    'dispatch: loop {
        match pc {
            0x82FF7D30 => {
    //   block [0x82FF7D30..0x82FF7D38)
	// 82FF7D30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF7D34: 8213F600  lwz r16, -0xa00(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7D38 size=96
    let mut pc: u32 = 0x82FF7D38;
    'dispatch: loop {
        match pc {
            0x82FF7D38 => {
    //   block [0x82FF7D38..0x82FF7D98)
	// 82FF7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF7D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7D48: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF7D4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7D50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7D54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF7D58: 396BF5B0  addi r11, r11, -0xa50
	ctx.r[11].s64 = ctx.r[11].s64 + -2640;
	// 82FF7D5C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF7D60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7D64: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7D68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF7D6C: 4182000C  beq 0x82ff7d78
	if ctx.cr[0].eq {
	pc = 0x82FF7D78; continue 'dispatch;
	}
	// 82FF7D70: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7D74: 4BFFCD7D  bl 0x82ff4af0
	ctx.lr = 0x82FF7D78;
	sub_82FF4AF0(ctx, base);
	// 82FF7D78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7D7C: 4800203D  bl 0x82ff9db8
	ctx.lr = 0x82FF7D80;
	sub_82FF9DB8(ctx, base);
	// 82FF7D80: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF7D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF7D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7D98 size=40
    let mut pc: u32 = 0x82FF7D98;
    'dispatch: loop {
        match pc {
            0x82FF7D98 => {
    //   block [0x82FF7D98..0x82FF7DC0)
	// 82FF7D98: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF7D9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7DA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7DA8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF7DAC: 4800200D  bl 0x82ff9db8
	ctx.lr = 0x82FF7DB0;
	sub_82FF9DB8(ctx, base);
	// 82FF7DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7DC0 size=12
    let mut pc: u32 = 0x82FF7DC0;
    'dispatch: loop {
        match pc {
            0x82FF7DC0 => {
    //   block [0x82FF7DC0..0x82FF7DCC)
	// 82FF7DC0: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7DC4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7DC8: 4BFFCCC8  b 0x82ff4a90
	sub_82FF4A90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7DD0 size=24
    let mut pc: u32 = 0x82FF7DD0;
    'dispatch: loop {
        match pc {
            0x82FF7DD0 => {
    //   block [0x82FF7DD0..0x82FF7DE8)
	// 82FF7DD0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF7DD4: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF7DD8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FF7DDC: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF7DE0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82FF7DE4: 4BFFCFF4  b 0x82ff4dd8
	sub_82FF4DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7DE8 size=76
    let mut pc: u32 = 0x82FF7DE8;
    'dispatch: loop {
        match pc {
            0x82FF7DE8 => {
    //   block [0x82FF7DE8..0x82FF7E34)
	// 82FF7DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF7DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7DFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF7E00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF7E04: 4BFFFF35  bl 0x82ff7d38
	ctx.lr = 0x82FF7E08;
	sub_82FF7D38(ctx, base);
	// 82FF7E08: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF7E0C: 4182000C  beq 0x82ff7e18
	if ctx.cr[0].eq {
	pc = 0x82FF7E18; continue 'dispatch;
	}
	// 82FF7E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7E14: 4BFE04CD  bl 0x82fd82e0
	ctx.lr = 0x82FF7E18;
	sub_82FD82E0(ctx, base);
	// 82FF7E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7E1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF7E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7E28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF7E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7E38 size=8
    let mut pc: u32 = 0x82FF7E38;
    'dispatch: loop {
        match pc {
            0x82FF7E38 => {
    //   block [0x82FF7E38..0x82FF7E40)
	// 82FF7E38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF7E3C: 8213F650  lwz r16, -0x9b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7E40 size=72
    let mut pc: u32 = 0x82FF7E40;
    'dispatch: loop {
        match pc {
            0x82FF7E40 => {
    //   block [0x82FF7E40..0x82FF7E88)
	// 82FF7E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7E44: 481B0329  bl 0x831a816c
	ctx.lr = 0x82FF7E48;
	sub_831A8130(ctx, base);
	// 82FF7E48: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FF7E4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7E50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF7E54: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FF7E58: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FF7E5C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FF7E60: 4BFE10D1  bl 0x82fd8f30
	ctx.lr = 0x82FF7E64;
	sub_82FD8F30(ctx, base);
	// 82FF7E64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7E68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7E6C: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF7E70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7E74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7E78: 4BFE1441  bl 0x82fd92b8
	ctx.lr = 0x82FF7E7C;
	sub_82FD92B8(ctx, base);
	// 82FF7E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7E80: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FF7E84: 481B0338  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7E88 size=40
    let mut pc: u32 = 0x82FF7E88;
    'dispatch: loop {
        match pc {
            0x82FF7E88 => {
    //   block [0x82FF7E88..0x82FF7EB0)
	// 82FF7E88: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FF7E8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7E90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7E98: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FF7E9C: 4BFE0FDD  bl 0x82fd8e78
	ctx.lr = 0x82FF7EA0;
	sub_82FD8E78(ctx, base);
	// 82FF7EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7EB0 size=60
    let mut pc: u32 = 0x82FF7EB0;
    'dispatch: loop {
        match pc {
            0x82FF7EB0 => {
    //   block [0x82FF7EB0..0x82FF7EEC)
	// 82FF7EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF7EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF7EC4: 4BFE10E5  bl 0x82fd8fa8
	ctx.lr = 0x82FF7EC8;
	sub_82FD8FA8(ctx, base);
	// 82FF7EC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF7ED0: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF7ED4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7EE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF7EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7EF0 size=8
    let mut pc: u32 = 0x82FF7EF0;
    'dispatch: loop {
        match pc {
            0x82FF7EF0 => {
    //   block [0x82FF7EF0..0x82FF7EF8)
	// 82FF7EF0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF7EF4: 8213F688  lwz r16, -0x978(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7EF8 size=104
    let mut pc: u32 = 0x82FF7EF8;
    'dispatch: loop {
        match pc {
            0x82FF7EF8 => {
    //   block [0x82FF7EF8..0x82FF7F60)
	// 82FF7EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7EFC: 481B0261  bl 0x831a815c
	ctx.lr = 0x82FF7F00;
	sub_831A8130(ctx, base);
	// 82FF7F00: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF7F04: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7F08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF7F0C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FF7F10: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FF7F14: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FF7F18: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FF7F1C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82FF7F20: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FF7F24: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82FF7F28: 4BFE1009  bl 0x82fd8f30
	ctx.lr = 0x82FF7F2C;
	sub_82FD8F30(ctx, base);
	// 82FF7F2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7F30: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FF7F34: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF7F38: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FF7F3C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FF7F40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FF7F44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7F48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7F4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7F50: 4BFE13E9  bl 0x82fd9338
	ctx.lr = 0x82FF7F54;
	sub_82FD9338(ctx, base);
	// 82FF7F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7F58: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF7F5C: 481B0250  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7F60 size=40
    let mut pc: u32 = 0x82FF7F60;
    'dispatch: loop {
        match pc {
            0x82FF7F60 => {
    //   block [0x82FF7F60..0x82FF7F88)
	// 82FF7F60: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF7F64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7F68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF7F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7F70: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FF7F74: 4BFE0F05  bl 0x82fd8e78
	ctx.lr = 0x82FF7F78;
	sub_82FD8E78(ctx, base);
	// 82FF7F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF7F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF7F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF7F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7F88 size=16
    let mut pc: u32 = 0x82FF7F88;
    'dispatch: loop {
        match pc {
            0x82FF7F88 => {
    //   block [0x82FF7F88..0x82FF7F98)
	// 82FF7F88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7F8C: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF7F90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7F94: 4BFE0EE4  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF7F98 size=8
    let mut pc: u32 = 0x82FF7F98;
    'dispatch: loop {
        match pc {
            0x82FF7F98 => {
    //   block [0x82FF7F98..0x82FF7FA0)
	// 82FF7F98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF7F9C: 8213F6C0  lwz r16, -0x940(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7FA0 size=92
    let mut pc: u32 = 0x82FF7FA0;
    'dispatch: loop {
        match pc {
            0x82FF7FA0 => {
    //   block [0x82FF7FA0..0x82FF7FFC)
	// 82FF7FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF7FA4: 481B01C9  bl 0x831a816c
	ctx.lr = 0x82FF7FA8;
	sub_831A8130(ctx, base);
	// 82FF7FA8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF7FAC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF7FB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF7FB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FF7FB8: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF7FBC: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FF7FC0: 4BFE02D9  bl 0x82fd8298
	ctx.lr = 0x82FF7FC4;
	sub_82FD8298(ctx, base);
	// 82FF7FC4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF7FC8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FF7FCC: 41820024  beq 0x82ff7ff0
	if ctx.cr[0].eq {
	pc = 0x82FF7FF0; continue 'dispatch;
	}
	// 82FF7FD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FF7FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7FD8: 4BFE0FD1  bl 0x82fd8fa8
	ctx.lr = 0x82FF7FDC;
	sub_82FD8FA8(ctx, base);
	// 82FF7FDC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF7FE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF7FE4: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF7FE8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF7FEC: 48000008  b 0x82ff7ff4
	pc = 0x82FF7FF4; continue 'dispatch;
	// 82FF7FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF7FF4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF7FF8: 481B01C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF7FFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF7FFC size=48
    let mut pc: u32 = 0x82FF7FFC;
    'dispatch: loop {
        match pc {
            0x82FF7FFC => {
    //   block [0x82FF7FFC..0x82FF802C)
	// 82FF7FFC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF8000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF800C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FF8010: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF8014: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF8018: 4BFE02C9  bl 0x82fd82e0
	ctx.lr = 0x82FF801C;
	sub_82FD82E0(ctx, base);
	// 82FF801C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF8020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF8030 size=12
    let mut pc: u32 = 0x82FF8030;
    'dispatch: loop {
        match pc {
            0x82FF8030 => {
    //   block [0x82FF8030..0x82FF803C)
	// 82FF8030: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8034: 386B8600  addi r3, r11, -0x7a00
	ctx.r[3].s64 = ctx.r[11].s64 + -31232;
	// 82FF8038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8040 size=88
    let mut pc: u32 = 0x82FF8040;
    'dispatch: loop {
        match pc {
            0x82FF8040 => {
    //   block [0x82FF8040..0x82FF8098)
	// 82FF8040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF804C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8054: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF805C: 396BF63C  addi r11, r11, -0x9c4
	ctx.r[11].s64 = ctx.r[11].s64 + -2500;
	// 82FF8060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF8064: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF8068: 4BFE0E11  bl 0x82fd8e78
	ctx.lr = 0x82FF806C;
	sub_82FD8E78(ctx, base);
	// 82FF806C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF8070: 4182000C  beq 0x82ff807c
	if ctx.cr[0].eq {
	pc = 0x82FF807C; continue 'dispatch;
	}
	// 82FF8074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8078: 4BFE0269  bl 0x82fd82e0
	ctx.lr = 0x82FF807C;
	sub_82FD82E0(ctx, base);
	// 82FF807C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF8084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF808C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF8098 size=20
    let mut pc: u32 = 0x82FF8098;
    'dispatch: loop {
        match pc {
            0x82FF8098 => {
    //   block [0x82FF8098..0x82FF80AC)
	// 82FF8098: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF809C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF80A0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FF80A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF80A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF80B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF80B0 size=20
    let mut pc: u32 = 0x82FF80B0;
    'dispatch: loop {
        match pc {
            0x82FF80B0 => {
    //   block [0x82FF80B0..0x82FF80C4)
	// 82FF80B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF80B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF80B8: 4182000C  beq 0x82ff80c4
	if ctx.cr[0].eq {
		sub_82FF80C4(ctx, base);
		return;
	}
	// 82FF80BC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF80C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF80C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF80C4 size=12
    let mut pc: u32 = 0x82FF80C4;
    'dispatch: loop {
        match pc {
            0x82FF80C4 => {
    //   block [0x82FF80C4..0x82FF80D0)
	// 82FF80C4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF80C8: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF80CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF80D0 size=96
    let mut pc: u32 = 0x82FF80D0;
    'dispatch: loop {
        match pc {
            0x82FF80D0 => {
    //   block [0x82FF80D0..0x82FF8130)
	// 82FF80D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF80D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF80D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF80DC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF80E0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF80E4: 41980030  blt cr6, 0x82ff8114
	if ctx.cr[6].lt {
	pc = 0x82FF8114; continue 'dispatch;
	}
	// 82FF80E8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FF80EC: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF80F0: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FF80F4: 388B7360  addi r4, r11, 0x7360
	ctx.r[4].s64 = ctx.r[11].s64 + 29536;
	// 82FF80F8: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82FF80FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF8100: 4BFD8859  bl 0x82fd0958
	ctx.lr = 0x82FF8104;
	sub_82FD0958(ctx, base);
	// 82FF8104: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8108: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF810C: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FF8110: 481B8B19  bl 0x831b0c28
	ctx.lr = 0x82FF8114;
	sub_831B0C28(ctx, base);
	// 82FF8114: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF8118: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF811C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FF8120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF8124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8130 size=104
    let mut pc: u32 = 0x82FF8130;
    'dispatch: loop {
        match pc {
            0x82FF8130 => {
    //   block [0x82FF8130..0x82FF8198)
	// 82FF8130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF813C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FF8140: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF8144: 40820044  bne 0x82ff8188
	if !ctx.cr[0].eq {
	pc = 0x82FF8188; continue 'dispatch;
	}
	// 82FF8148: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF814C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8150: 4182000C  beq 0x82ff815c
	if ctx.cr[0].eq {
	pc = 0x82FF815C; continue 'dispatch;
	}
	// 82FF8154: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8158: 4800000C  b 0x82ff8164
	pc = 0x82FF8164; continue 'dispatch;
	// 82FF815C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF8160: 80EBB7E8  lwz r7, -0x4818(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8164: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8168: 38A0030C  li r5, 0x30c
	ctx.r[5].s64 = 780;
	// 82FF816C: 388BF6F0  addi r4, r11, -0x910
	ctx.r[4].s64 = ctx.r[11].s64 + -2320;
	// 82FF8170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF8174: 4BFFFCCD  bl 0x82ff7e40
	ctx.lr = 0x82FF8178;
	sub_82FF7E40(ctx, base);
	// 82FF8178: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF817C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF8180: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8184: 481B8AA5  bl 0x831b0c28
	ctx.lr = 0x82FF8188;
	sub_831B0C28(ctx, base);
	// 82FF8188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF818C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8198 size=268
    let mut pc: u32 = 0x82FF8198;
    'dispatch: loop {
        match pc {
            0x82FF8198 => {
    //   block [0x82FF8198..0x82FF82A4)
	// 82FF8198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF819C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF81A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF81A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF81A8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF81AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF81B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF81B4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF81B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF81BC: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF81C0: 409900B4  ble cr6, 0x82ff8274
	if !ctx.cr[6].gt {
	pc = 0x82FF8274; continue 'dispatch;
	}
	// 82FF81C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF81C8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF81CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF81D0: 4182000C  beq 0x82ff81dc
	if ctx.cr[0].eq {
	pc = 0x82FF81DC; continue 'dispatch;
	}
	// 82FF81D4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF81D8: 48000008  b 0x82ff81e0
	pc = 0x82FF81E0; continue 'dispatch;
	// 82FF81DC: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF81E0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF81E4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF81E8: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF81EC: 4BFD967D  bl 0x82fd1868
	ctx.lr = 0x82FF81F0;
	sub_82FD1868(ctx, base);
	// 82FF81F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF81F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF81F8: 4182000C  beq 0x82ff8204
	if ctx.cr[0].eq {
	pc = 0x82FF8204; continue 'dispatch;
	}
	// 82FF81FC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8200: 48000008  b 0x82ff8208
	pc = 0x82FF8208; continue 'dispatch;
	// 82FF8204: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8208: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF820C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8210: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8214: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8218: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF821C: 4BFD964D  bl 0x82fd1868
	ctx.lr = 0x82FF8220;
	sub_82FD1868(ctx, base);
	// 82FF8220: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8224: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8228: 4182000C  beq 0x82ff8234
	if ctx.cr[0].eq {
	pc = 0x82FF8234; continue 'dispatch;
	}
	// 82FF822C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8230: 48000008  b 0x82ff8238
	pc = 0x82FF8238; continue 'dispatch;
	// 82FF8234: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8238: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF823C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8244: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8248: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF824C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8250: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF8254: 38C00180  li r6, 0x180
	ctx.r[6].s64 = 384;
	// 82FF8258: 38A00375  li r5, 0x375
	ctx.r[5].s64 = 885;
	// 82FF825C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8260: 4BFFFC99  bl 0x82ff7ef8
	ctx.lr = 0x82FF8264;
	sub_82FF7EF8(ctx, base);
	// 82FF8264: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8268: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF826C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8270: 481B89B9  bl 0x831b0c28
	ctx.lr = 0x82FF8274;
	sub_831B0C28(ctx, base);
	// 82FF8274: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FF8278: 419A0014  beq cr6, 0x82ff828c
	if ctx.cr[6].eq {
	pc = 0x82FF828C; continue 'dispatch;
	}
	// 82FF827C: 3883FFFF  addi r4, r3, -1
	ctx.r[4].s64 = ctx.r[3].s64 + -1;
	// 82FF8280: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FF8284: 4BFFFE4D  bl 0x82ff80d0
	ctx.lr = 0x82FF8288;
	sub_82FF80D0(ctx, base);
	// 82FF8288: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF828C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82FF8290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF829C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF82A8 size=224
    let mut pc: u32 = 0x82FF82A8;
    'dispatch: loop {
        match pc {
            0x82FF82A8 => {
    //   block [0x82FF82A8..0x82FF8388)
	// 82FF82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF82AC: 481AFEC1  bl 0x831a816c
	ctx.lr = 0x82FF82B0;
	sub_831A8130(ctx, base);
	// 82FF82B0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF82B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF82B8: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82FF82BC: 617DFFFD  ori r29, r11, 0xfffd
	ctx.r[29].u64 = ctx.r[11].u64 | 65533;
	// 82FF82C0: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF82C4: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FF82C8: 419800B0  blt cr6, 0x82ff8378
	if ctx.cr[6].lt {
	pc = 0x82FF8378; continue 'dispatch;
	}
	// 82FF82CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF82D0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF82D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF82D8: 4182000C  beq 0x82ff82e4
	if ctx.cr[0].eq {
	pc = 0x82FF82E4; continue 'dispatch;
	}
	// 82FF82DC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF82E0: 48000008  b 0x82ff82e8
	pc = 0x82FF82E8; continue 'dispatch;
	// 82FF82E4: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF82E8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF82EC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF82F0: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF82F4: 4BFD9575  bl 0x82fd1868
	ctx.lr = 0x82FF82F8;
	sub_82FD1868(ctx, base);
	// 82FF82F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF82FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8300: 4182000C  beq 0x82ff830c
	if ctx.cr[0].eq {
	pc = 0x82FF830C; continue 'dispatch;
	}
	// 82FF8304: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8308: 48000008  b 0x82ff8310
	pc = 0x82FF8310; continue 'dispatch;
	// 82FF830C: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8310: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8314: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8318: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF831C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF8320: 4BFD9549  bl 0x82fd1868
	ctx.lr = 0x82FF8324;
	sub_82FD1868(ctx, base);
	// 82FF8324: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8328: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF832C: 4182000C  beq 0x82ff8338
	if ctx.cr[0].eq {
	pc = 0x82FF8338; continue 'dispatch;
	}
	// 82FF8330: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8334: 48000008  b 0x82ff833c
	pc = 0x82FF833C; continue 'dispatch;
	// 82FF8338: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF833C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8340: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8348: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF834C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8350: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8354: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF8358: 38C0018B  li r6, 0x18b
	ctx.r[6].s64 = 395;
	// 82FF835C: 38A00395  li r5, 0x395
	ctx.r[5].s64 = 917;
	// 82FF8360: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8364: 4BFFFB95  bl 0x82ff7ef8
	ctx.lr = 0x82FF8368;
	sub_82FF7EF8(ctx, base);
	// 82FF8368: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF836C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8370: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8374: 481B88B5  bl 0x831b0c28
	ctx.lr = 0x82FF8378;
	sub_831B0C28(ctx, base);
	// 82FF8378: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82FF837C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FF8380: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82FF8384: 481AFE38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8388 size=260
    let mut pc: u32 = 0x82FF8388;
    'dispatch: loop {
        match pc {
            0x82FF8388 => {
    //   block [0x82FF8388..0x82FF848C)
	// 82FF8388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF838C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8398: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF839C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF83A0: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF83A4: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF83A8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF83AC: 41990010  bgt cr6, 0x82ff83bc
	if ctx.cr[6].gt {
	pc = 0x82FF83BC; continue 'dispatch;
	}
	// 82FF83B0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF83B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF83B8: 409900BC  ble cr6, 0x82ff8474
	if !ctx.cr[6].gt {
	pc = 0x82FF8474; continue 'dispatch;
	}
	// 82FF83BC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF83C0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF83C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF83C8: 4182000C  beq 0x82ff83d4
	if ctx.cr[0].eq {
	pc = 0x82FF83D4; continue 'dispatch;
	}
	// 82FF83CC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF83D0: 48000008  b 0x82ff83d8
	pc = 0x82FF83D8; continue 'dispatch;
	// 82FF83D4: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF83D8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF83DC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF83E0: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF83E4: 7C695050  subf r3, r9, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82FF83E8: 4BFD9489  bl 0x82fd1870
	ctx.lr = 0x82FF83EC;
	sub_82FD1870(ctx, base);
	// 82FF83EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF83F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF83F4: 4182000C  beq 0x82ff8400
	if ctx.cr[0].eq {
	pc = 0x82FF8400; continue 'dispatch;
	}
	// 82FF83F8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF83FC: 48000008  b 0x82ff8404
	pc = 0x82FF8404; continue 'dispatch;
	// 82FF8400: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8404: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF8408: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF840C: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF8410: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8414: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8418: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FF841C: 4BFD9455  bl 0x82fd1870
	ctx.lr = 0x82FF8420;
	sub_82FD1870(ctx, base);
	// 82FF8420: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8424: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8428: 4182000C  beq 0x82ff8434
	if ctx.cr[0].eq {
	pc = 0x82FF8434; continue 'dispatch;
	}
	// 82FF842C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8430: 48000008  b 0x82ff8438
	pc = 0x82FF8438; continue 'dispatch;
	// 82FF8434: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8438: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF843C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8444: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8448: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF844C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8450: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF8454: 38C0017F  li r6, 0x17f
	ctx.r[6].s64 = 383;
	// 82FF8458: 38A003F9  li r5, 0x3f9
	ctx.r[5].s64 = 1017;
	// 82FF845C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8460: 4BFFFA99  bl 0x82ff7ef8
	ctx.lr = 0x82FF8464;
	sub_82FF7EF8(ctx, base);
	// 82FF8464: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8468: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF846C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8470: 481B87B9  bl 0x831b0c28
	ctx.lr = 0x82FF8474;
	sub_831B0C28(ctx, base);
	// 82FF8474: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82FF8478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF847C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8490 size=260
    let mut pc: u32 = 0x82FF8490;
    'dispatch: loop {
        match pc {
            0x82FF8490 => {
    //   block [0x82FF8490..0x82FF8594)
	// 82FF8490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF849C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF84A0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF84A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF84A8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF84AC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF84B0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF84B4: 41990010  bgt cr6, 0x82ff84c4
	if ctx.cr[6].gt {
	pc = 0x82FF84C4; continue 'dispatch;
	}
	// 82FF84B8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF84BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF84C0: 409900BC  ble cr6, 0x82ff857c
	if !ctx.cr[6].gt {
	pc = 0x82FF857C; continue 'dispatch;
	}
	// 82FF84C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF84C8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF84CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF84D0: 4182000C  beq 0x82ff84dc
	if ctx.cr[0].eq {
	pc = 0x82FF84DC; continue 'dispatch;
	}
	// 82FF84D4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF84D8: 48000008  b 0x82ff84e0
	pc = 0x82FF84E0; continue 'dispatch;
	// 82FF84DC: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF84E0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF84E4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF84E8: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF84EC: 7C695050  subf r3, r9, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82FF84F0: 4BFD9381  bl 0x82fd1870
	ctx.lr = 0x82FF84F4;
	sub_82FD1870(ctx, base);
	// 82FF84F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF84F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF84FC: 4182000C  beq 0x82ff8508
	if ctx.cr[0].eq {
	pc = 0x82FF8508; continue 'dispatch;
	}
	// 82FF8500: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8504: 48000008  b 0x82ff850c
	pc = 0x82FF850C; continue 'dispatch;
	// 82FF8508: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF850C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF8510: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8514: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF8518: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF851C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8520: 7C6A5850  subf r3, r10, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FF8524: 4BFD934D  bl 0x82fd1870
	ctx.lr = 0x82FF8528;
	sub_82FD1870(ctx, base);
	// 82FF8528: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF852C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8530: 4182000C  beq 0x82ff853c
	if ctx.cr[0].eq {
	pc = 0x82FF853C; continue 'dispatch;
	}
	// 82FF8534: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8538: 48000008  b 0x82ff8540
	pc = 0x82FF8540; continue 'dispatch;
	// 82FF853C: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8540: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8544: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF854C: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8550: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8554: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8558: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF855C: 38C00183  li r6, 0x183
	ctx.r[6].s64 = 387;
	// 82FF8560: 38A00404  li r5, 0x404
	ctx.r[5].s64 = 1028;
	// 82FF8564: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8568: 4BFFF991  bl 0x82ff7ef8
	ctx.lr = 0x82FF856C;
	sub_82FF7EF8(ctx, base);
	// 82FF856C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8570: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8574: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8578: 481B86B1  bl 0x831b0c28
	ctx.lr = 0x82FF857C;
	sub_831B0C28(ctx, base);
	// 82FF857C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82FF8580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF858C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8598 size=188
    let mut pc: u32 = 0x82FF8598;
    'dispatch: loop {
        match pc {
            0x82FF8598 => {
    //   block [0x82FF8598..0x82FF8654)
	// 82FF8598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF85A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF85A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF85A8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF85AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF85B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FF85B4: 409A0088  bne cr6, 0x82ff863c
	if !ctx.cr[6].eq {
	pc = 0x82FF863C; continue 'dispatch;
	}
	// 82FF85B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF85BC: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF85C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF85C4: 4182000C  beq 0x82ff85d0
	if ctx.cr[0].eq {
	pc = 0x82FF85D0; continue 'dispatch;
	}
	// 82FF85C8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF85CC: 48000008  b 0x82ff85d4
	pc = 0x82FF85D4; continue 'dispatch;
	// 82FF85D0: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF85D4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF85D8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF85DC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF85E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF85E4: 4BFD928D  bl 0x82fd1870
	ctx.lr = 0x82FF85E8;
	sub_82FD1870(ctx, base);
	// 82FF85E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF85EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF85F0: 4182000C  beq 0x82ff85fc
	if ctx.cr[0].eq {
	pc = 0x82FF85FC; continue 'dispatch;
	}
	// 82FF85F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF85F8: 48000008  b 0x82ff8600
	pc = 0x82FF8600; continue 'dispatch;
	// 82FF85FC: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8600: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8604: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF860C: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8610: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF8618: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FF861C: 38C00188  li r6, 0x188
	ctx.r[6].s64 = 392;
	// 82FF8620: 38A0040E  li r5, 0x40e
	ctx.r[5].s64 = 1038;
	// 82FF8624: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8628: 4BFFF8D1  bl 0x82ff7ef8
	ctx.lr = 0x82FF862C;
	sub_82FF7EF8(ctx, base);
	// 82FF862C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8630: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8634: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8638: 481B85F1  bl 0x831b0c28
	ctx.lr = 0x82FF863C;
	sub_831B0C28(ctx, base);
	// 82FF863C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FF8640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF864C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8658 size=188
    let mut pc: u32 = 0x82FF8658;
    'dispatch: loop {
        match pc {
            0x82FF8658 => {
    //   block [0x82FF8658..0x82FF8714)
	// 82FF8658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF865C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8668: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF866C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8670: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF8674: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FF8678: 40980084  bge cr6, 0x82ff86fc
	if !ctx.cr[6].lt {
	pc = 0x82FF86FC; continue 'dispatch;
	}
	// 82FF867C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8680: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8684: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8688: 4182000C  beq 0x82ff8694
	if ctx.cr[0].eq {
	pc = 0x82FF8694; continue 'dispatch;
	}
	// 82FF868C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8690: 48000008  b 0x82ff8698
	pc = 0x82FF8698; continue 'dispatch;
	// 82FF8694: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8698: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF869C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF86A0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF86A4: 4BFD91CD  bl 0x82fd1870
	ctx.lr = 0x82FF86A8;
	sub_82FD1870(ctx, base);
	// 82FF86A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF86AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF86B0: 4182000C  beq 0x82ff86bc
	if ctx.cr[0].eq {
	pc = 0x82FF86BC; continue 'dispatch;
	}
	// 82FF86B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF86B8: 48000008  b 0x82ff86c0
	pc = 0x82FF86C0; continue 'dispatch;
	// 82FF86BC: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF86C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF86C4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF86C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF86CC: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF86D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF86D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF86D8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FF86DC: 38C00189  li r6, 0x189
	ctx.r[6].s64 = 393;
	// 82FF86E0: 38A00418  li r5, 0x418
	ctx.r[5].s64 = 1048;
	// 82FF86E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF86E8: 4BFFF811  bl 0x82ff7ef8
	ctx.lr = 0x82FF86EC;
	sub_82FF7EF8(ctx, base);
	// 82FF86EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF86F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF86F4: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF86F8: 481B8531  bl 0x831b0c28
	ctx.lr = 0x82FF86FC;
	sub_831B0C28(ctx, base);
	// 82FF86FC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FF8700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF870C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF8718 size=8
    let mut pc: u32 = 0x82FF8718;
    'dispatch: loop {
        match pc {
            0x82FF8718 => {
    //   block [0x82FF8718..0x82FF8720)
	// 82FF8718: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF871C: 8213F778  lwz r16, -0x888(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8720 size=280
    let mut pc: u32 = 0x82FF8720;
    'dispatch: loop {
        match pc {
            0x82FF8720 => {
    //   block [0x82FF8720..0x82FF8838)
	// 82FF8720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8724: 481AFA41  bl 0x831a8164
	ctx.lr = 0x82FF8728;
	sub_831A8130(ctx, base);
	// 82FF8728: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FF872C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8730: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF8734: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF8738: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF873C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FF8740: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FF8744: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF8748: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FF874C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FF8750: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82FF8754: B37E0000  sth r27, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 82FF8758: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FF875C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8760: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF8764: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF876C: 4E800421  bctrl
	ctx.lr = 0x82FF8770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF8770: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF8774: 937E0020  stw r27, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82FF8778: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FF877C: 7D4BE214  add r10, r11, r28
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FF8780: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FF8784: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF8788: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FF878C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8790: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FF8794: 4BFDFB05  bl 0x82fd8298
	ctx.lr = 0x82FF8798;
	sub_82FD8298(ctx, base);
	// 82FF8798: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FF879C: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82FF87A0: 4182004C  beq 0x82ff87ec
	if ctx.cr[0].eq {
	pc = 0x82FF87EC; continue 'dispatch;
	}
	// 82FF87A4: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF87A8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FF87AC: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82FF87B0: 4BFDFAE9  bl 0x82fd8298
	ctx.lr = 0x82FF87B4;
	sub_82FD8298(ctx, base);
	// 82FF87B4: 907F005C  stw r3, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82FF87B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF87BC: 41820010  beq 0x82ff87cc
	if ctx.cr[0].eq {
	pc = 0x82FF87CC; continue 'dispatch;
	}
	// 82FF87C0: 48006DD1  bl 0x82fff590
	ctx.lr = 0x82FF87C4;
	sub_82FFF590(ctx, base);
	// 82FF87C4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82FF87C8: 48000008  b 0x82ff87d0
	pc = 0x82FF87D0; continue 'dispatch;
	// 82FF87CC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FF87D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FF87D4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 82FF87D8: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF87DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF87E0: 48042911  bl 0x8303b0f0
	ctx.lr = 0x82FF87E4;
	sub_8303B0F0(ctx, base);
	// 82FF87E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF87E8: 48000008  b 0x82ff87f0
	pc = 0x82FF87F0; continue 'dispatch;
	// 82FF87EC: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FF87F0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FF87F4: 937E0028  stw r27, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 82FF87F8: 937E002C  stw r27, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 82FF87FC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FF8800: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8804: 4BFDFA95  bl 0x82fd8298
	ctx.lr = 0x82FF8808;
	sub_82FD8298(ctx, base);
	// 82FF8808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF880C: 41820010  beq 0x82ff881c
	if ctx.cr[0].eq {
	pc = 0x82FF881C; continue 'dispatch;
	}
	// 82FF8810: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF8814: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FF8818: 48000008  b 0x82ff8820
	pc = 0x82FF8820; continue 'dispatch;
	// 82FF881C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FF8820: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF8824: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF8828: 480B7AE1  bl 0x830b0308
	ctx.lr = 0x82FF882C;
	sub_830B0308(ctx, base);
	// 82FF882C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF8830: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FF8834: 481AF980  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8838 size=44
    let mut pc: u32 = 0x82FF8838;
    'dispatch: loop {
        match pc {
            0x82FF8838 => {
    //   block [0x82FF8838..0x82FF8864)
	// 82FF8838: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF883C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8840: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8848: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF884C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF8850: 4BFDFA91  bl 0x82fd82e0
	ctx.lr = 0x82FF8854;
	sub_82FD82E0(ctx, base);
	// 82FF8854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF8858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF885C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8864 size=44
    let mut pc: u32 = 0x82FF8864;
    'dispatch: loop {
        match pc {
            0x82FF8864 => {
    //   block [0x82FF8864..0x82FF8890)
	// 82FF8864: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FF8868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF886C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8874: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FF8878: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FF887C: 4BFDFA65  bl 0x82fd82e0
	ctx.lr = 0x82FF8880;
	sub_82FD82E0(ctx, base);
	// 82FF8880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF8884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF888C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8890 size=72
    let mut pc: u32 = 0x82FF8890;
    'dispatch: loop {
        match pc {
            0x82FF8890 => {
    //   block [0x82FF8890..0x82FF88D8)
	// 82FF8890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF889C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FF88A0: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF88A4: 48001BBD  bl 0x82ffa460
	ctx.lr = 0x82FF88A8;
	sub_82FFA460(ctx, base);
	// 82FF88A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF88AC: 41820018  beq 0x82ff88c4
	if ctx.cr[0].eq {
	pc = 0x82FF88C4; continue 'dispatch;
	}
	// 82FF88B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF88B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF88B8: 4182000C  beq 0x82ff88c4
	if ctx.cr[0].eq {
	pc = 0x82FF88C4; continue 'dispatch;
	}
	// 82FF88BC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF88C0: 48000008  b 0x82ff88c8
	pc = 0x82FF88C8; continue 'dispatch;
	// 82FF88C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF88C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF88CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF88D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF88D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF88D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF88D8 size=112
    let mut pc: u32 = 0x82FF88D8;
    'dispatch: loop {
        match pc {
            0x82FF88D8 => {
    //   block [0x82FF88D8..0x82FF8948)
	// 82FF88D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF88DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF88E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF88E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF88E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF88EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF88F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF88F4: 4BFFF9B5  bl 0x82ff82a8
	ctx.lr = 0x82FF88F8;
	sub_82FF82A8(ctx, base);
	// 82FF88F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF88FC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FF8900: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8904: 4BFDF995  bl 0x82fd8298
	ctx.lr = 0x82FF8908;
	sub_82FD8298(ctx, base);
	// 82FF8908: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF890C: 41820014  beq 0x82ff8920
	if ctx.cr[0].eq {
	pc = 0x82FF8920; continue 'dispatch;
	}
	// 82FF8910: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF8914: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FF8918: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF891C: 48000008  b 0x82ff8924
	pc = 0x82FF8924; continue 'dispatch;
	// 82FF8920: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FF8924: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF8928: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF892C: 480B79DD  bl 0x830b0308
	ctx.lr = 0x82FF8930;
	sub_830B0308(ctx, base);
	// 82FF8930: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF8934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF893C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8948 size=268
    let mut pc: u32 = 0x82FF8948;
    'dispatch: loop {
        match pc {
            0x82FF8948 => {
    //   block [0x82FF8948..0x82FF8A54)
	// 82FF8948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8958: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF895C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8960: 9081010C  stw r4, 0x10c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), ctx.r[4].u32 ) };
	// 82FF8964: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF8968: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FF896C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8970: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FF8974: 419A00B4  beq cr6, 0x82ff8a28
	if ctx.cr[6].eq {
	pc = 0x82FF8A28; continue 'dispatch;
	}
	// 82FF8978: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF897C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8980: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8984: 4182000C  beq 0x82ff8990
	if ctx.cr[0].eq {
	pc = 0x82FF8990; continue 'dispatch;
	}
	// 82FF8988: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF898C: 48000008  b 0x82ff8994
	pc = 0x82FF8994; continue 'dispatch;
	// 82FF8990: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8994: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8998: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF899C: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF89A0: 4BFD8EC9  bl 0x82fd1868
	ctx.lr = 0x82FF89A4;
	sub_82FD1868(ctx, base);
	// 82FF89A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF89A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF89AC: 4182000C  beq 0x82ff89b8
	if ctx.cr[0].eq {
	pc = 0x82FF89B8; continue 'dispatch;
	}
	// 82FF89B0: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF89B4: 48000008  b 0x82ff89bc
	pc = 0x82FF89BC; continue 'dispatch;
	// 82FF89B8: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF89BC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF89C0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF89C4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF89C8: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF89CC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF89D0: 4BFD8E99  bl 0x82fd1868
	ctx.lr = 0x82FF89D4;
	sub_82FD1868(ctx, base);
	// 82FF89D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF89D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF89DC: 4182000C  beq 0x82ff89e8
	if ctx.cr[0].eq {
	pc = 0x82FF89E8; continue 'dispatch;
	}
	// 82FF89E0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF89E4: 48000008  b 0x82ff89ec
	pc = 0x82FF89EC; continue 'dispatch;
	// 82FF89E8: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF89EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF89F0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF89F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF89F8: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF89FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8A00: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8A04: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF8A08: 38C00181  li r6, 0x181
	ctx.r[6].s64 = 385;
	// 82FF8A0C: 38A00387  li r5, 0x387
	ctx.r[5].s64 = 903;
	// 82FF8A10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8A14: 4BFFF4E5  bl 0x82ff7ef8
	ctx.lr = 0x82FF8A18;
	sub_82FF7EF8(ctx, base);
	// 82FF8A18: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8A1C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8A20: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8A24: 481B8205  bl 0x831b0c28
	ctx.lr = 0x82FF8A28;
	sub_831B0C28(ctx, base);
	// 82FF8A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8A2C: 4BFFF87D  bl 0x82ff82a8
	ctx.lr = 0x82FF8A30;
	sub_82FF82A8(ctx, base);
	// 82FF8A30: 3881010C  addi r4, r1, 0x10c
	ctx.r[4].s64 = ctx.r[1].s64 + 268;
	// 82FF8A34: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF8A38: 4BFEE1D1  bl 0x82fe6c08
	ctx.lr = 0x82FF8A3C;
	sub_82FE6C08(ctx, base);
	// 82FF8A3C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82FF8A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8A48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8A58 size=496
    let mut pc: u32 = 0x82FF8A58;
    'dispatch: loop {
        match pc {
            0x82FF8A58 => {
    //   block [0x82FF8A58..0x82FF8C48)
	// 82FF8A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8A68: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8A70: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF8A74: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF8A78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF8A7C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF8A80: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF8A84: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF8A88: 4BFFF6A9  bl 0x82ff8130
	ctx.lr = 0x82FF8A8C;
	sub_82FF8130(ctx, base);
	// 82FF8A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8A90: 4BFFFA01  bl 0x82ff8490
	ctx.lr = 0x82FF8A94;
	sub_82FF8490(ctx, base);
	// 82FF8A94: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF8A98: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF8A9C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF8AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF8AA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF8AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF8AAC: 4E800421  bctrl
	ctx.lr = 0x82FF8AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF8AB0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF8AB4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FF8AB8: 409800B0  bge cr6, 0x82ff8b68
	if !ctx.cr[6].lt {
	pc = 0x82FF8B68; continue 'dispatch;
	}
	// 82FF8ABC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8AC0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8AC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8AC8: 4182000C  beq 0x82ff8ad4
	if ctx.cr[0].eq {
	pc = 0x82FF8AD4; continue 'dispatch;
	}
	// 82FF8ACC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8AD0: 48000008  b 0x82ff8ad8
	pc = 0x82FF8AD8; continue 'dispatch;
	// 82FF8AD4: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8AD8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8ADC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8AE0: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF8AE4: 4BFD8D8D  bl 0x82fd1870
	ctx.lr = 0x82FF8AE8;
	sub_82FD1870(ctx, base);
	// 82FF8AE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8AEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8AF0: 4182000C  beq 0x82ff8afc
	if ctx.cr[0].eq {
	pc = 0x82FF8AFC; continue 'dispatch;
	}
	// 82FF8AF4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8AF8: 48000008  b 0x82ff8b00
	pc = 0x82FF8B00; continue 'dispatch;
	// 82FF8AFC: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8B00: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8B04: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF8B08: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8B0C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8B10: 4BFD8B99  bl 0x82fd16a8
	ctx.lr = 0x82FF8B14;
	sub_82FD16A8(ctx, base);
	// 82FF8B14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8B18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8B1C: 4182000C  beq 0x82ff8b28
	if ctx.cr[0].eq {
	pc = 0x82FF8B28; continue 'dispatch;
	}
	// 82FF8B20: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8B24: 48000008  b 0x82ff8b2c
	pc = 0x82FF8B2C; continue 'dispatch;
	// 82FF8B28: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8B2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8B30: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8B34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8B38: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8B40: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FF8B44: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82FF8B48: 38C0017C  li r6, 0x17c
	ctx.r[6].s64 = 380;
	// 82FF8B4C: 38A003B6  li r5, 0x3b6
	ctx.r[5].s64 = 950;
	// 82FF8B50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8B54: 4BFFF3A5  bl 0x82ff7ef8
	ctx.lr = 0x82FF8B58;
	sub_82FF7EF8(ctx, base);
	// 82FF8B58: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8B5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8B60: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8B64: 481B80C5  bl 0x831b0c28
	ctx.lr = 0x82FF8B68;
	sub_831B0C28(ctx, base);
	// 82FF8B68: 409900B0  ble cr6, 0x82ff8c18
	if !ctx.cr[6].gt {
	pc = 0x82FF8C18; continue 'dispatch;
	}
	// 82FF8B6C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8B70: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8B74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8B78: 4182000C  beq 0x82ff8b84
	if ctx.cr[0].eq {
	pc = 0x82FF8B84; continue 'dispatch;
	}
	// 82FF8B7C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8B80: 48000008  b 0x82ff8b88
	pc = 0x82FF8B88; continue 'dispatch;
	// 82FF8B84: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8B88: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8B8C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8B90: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8B94: 4BFD8CDD  bl 0x82fd1870
	ctx.lr = 0x82FF8B98;
	sub_82FD1870(ctx, base);
	// 82FF8B98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8B9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8BA0: 4182000C  beq 0x82ff8bac
	if ctx.cr[0].eq {
	pc = 0x82FF8BAC; continue 'dispatch;
	}
	// 82FF8BA4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8BA8: 48000008  b 0x82ff8bb0
	pc = 0x82FF8BB0; continue 'dispatch;
	// 82FF8BAC: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8BB0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8BB4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF8BB8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8BBC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82FF8BC0: 4BFD8AE9  bl 0x82fd16a8
	ctx.lr = 0x82FF8BC4;
	sub_82FD16A8(ctx, base);
	// 82FF8BC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8BC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8BCC: 4182000C  beq 0x82ff8bd8
	if ctx.cr[0].eq {
	pc = 0x82FF8BD8; continue 'dispatch;
	}
	// 82FF8BD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8BD4: 48000008  b 0x82ff8bdc
	pc = 0x82FF8BDC; continue 'dispatch;
	// 82FF8BD8: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8BDC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8BE0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8BE8: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8BF0: 390100B0  addi r8, r1, 0xb0
	ctx.r[8].s64 = ctx.r[1].s64 + 176;
	// 82FF8BF4: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FF8BF8: 38C0017D  li r6, 0x17d
	ctx.r[6].s64 = 381;
	// 82FF8BFC: 38A003BC  li r5, 0x3bc
	ctx.r[5].s64 = 956;
	// 82FF8C00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8C04: 4BFFF2F5  bl 0x82ff7ef8
	ctx.lr = 0x82FF8C08;
	sub_82FF7EF8(ctx, base);
	// 82FF8C08: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8C0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8C10: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8C14: 481B8015  bl 0x831b0c28
	ctx.lr = 0x82FF8C18;
	sub_831B0C28(ctx, base);
	// 82FF8C18: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF8C1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8C20: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF8C24: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82FF8C28: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FF8C2C: 4BFFF865  bl 0x82ff8490
	ctx.lr = 0x82FF8C30;
	sub_82FF8490(ctx, base);
	// 82FF8C30: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82FF8C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8C3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8C40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8C48 size=116
    let mut pc: u32 = 0x82FF8C48;
    'dispatch: loop {
        match pc {
            0x82FF8C48 => {
    //   block [0x82FF8C48..0x82FF8CBC)
	// 82FF8C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8C50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8C58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8C5C: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FF8C60: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF8C64: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF8C68: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF8C6C: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF8C70: 4BFFF4C1  bl 0x82ff8130
	ctx.lr = 0x82FF8C74;
	sub_82FF8130(ctx, base);
	// 82FF8C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8C78: 4BFFF711  bl 0x82ff8388
	ctx.lr = 0x82FF8C7C;
	sub_82FF8388(ctx, base);
	// 82FF8C7C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF8C80: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF8C84: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF8C88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF8C8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF8C90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF8C94: 4E800421  bctrl
	ctx.lr = 0x82FF8C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF8C98: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF8C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8CA0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF8CA4: 4BFFF6E5  bl 0x82ff8388
	ctx.lr = 0x82FF8CA8;
	sub_82FF8388(ctx, base);
	// 82FF8CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF8CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8CC0 size=216
    let mut pc: u32 = 0x82FF8CC0;
    'dispatch: loop {
        match pc {
            0x82FF8CC0 => {
    //   block [0x82FF8CC0..0x82FF8D98)
	// 82FF8CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8CD0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8CD8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF8CDC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FF8CE0: 41990084  bgt cr6, 0x82ff8d64
	if ctx.cr[6].gt {
	pc = 0x82FF8D64; continue 'dispatch;
	}
	// 82FF8CE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8CE8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8CEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8CF0: 4182000C  beq 0x82ff8cfc
	if ctx.cr[0].eq {
	pc = 0x82FF8CFC; continue 'dispatch;
	}
	// 82FF8CF4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8CF8: 48000008  b 0x82ff8d00
	pc = 0x82FF8D00; continue 'dispatch;
	// 82FF8CFC: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8D00: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8D04: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8D08: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8D0C: 4BFD8B65  bl 0x82fd1870
	ctx.lr = 0x82FF8D10;
	sub_82FD1870(ctx, base);
	// 82FF8D10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8D14: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8D18: 4182000C  beq 0x82ff8d24
	if ctx.cr[0].eq {
	pc = 0x82FF8D24; continue 'dispatch;
	}
	// 82FF8D1C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8D20: 48000008  b 0x82ff8d28
	pc = 0x82FF8D28; continue 'dispatch;
	// 82FF8D24: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8D28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8D2C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8D34: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8D38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF8D40: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FF8D44: 38C00187  li r6, 0x187
	ctx.r[6].s64 = 391;
	// 82FF8D48: 38A003DB  li r5, 0x3db
	ctx.r[5].s64 = 987;
	// 82FF8D4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8D50: 4BFFF1A9  bl 0x82ff7ef8
	ctx.lr = 0x82FF8D54;
	sub_82FF7EF8(ctx, base);
	// 82FF8D54: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8D58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8D5C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8D60: 481B7EC9  bl 0x831b0c28
	ctx.lr = 0x82FF8D64;
	sub_831B0C28(ctx, base);
	// 82FF8D64: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF8D68: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF8D6C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FF8D70: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF8D74: 4198000C  blt cr6, 0x82ff8d80
	if ctx.cr[6].lt {
	pc = 0x82FF8D80; continue 'dispatch;
	}
	// 82FF8D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8D7C: 4BFFFECD  bl 0x82ff8c48
	ctx.lr = 0x82FF8D80;
	sub_82FF8C48(ctx, base);
	// 82FF8D80: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FF8D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8D98 size=216
    let mut pc: u32 = 0x82FF8D98;
    'dispatch: loop {
        match pc {
            0x82FF8D98 => {
    //   block [0x82FF8D98..0x82FF8E70)
	// 82FF8D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8DA8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8DAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8DB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FF8DB4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FF8DB8: 41990084  bgt cr6, 0x82ff8e3c
	if ctx.cr[6].gt {
	pc = 0x82FF8E3C; continue 'dispatch;
	}
	// 82FF8DBC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8DC0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF8DC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8DC8: 4182000C  beq 0x82ff8dd4
	if ctx.cr[0].eq {
	pc = 0x82FF8DD4; continue 'dispatch;
	}
	// 82FF8DCC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8DD0: 48000008  b 0x82ff8dd8
	pc = 0x82FF8DD8; continue 'dispatch;
	// 82FF8DD4: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8DD8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF8DDC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF8DE0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82FF8DE4: 4BFD8A8D  bl 0x82fd1870
	ctx.lr = 0x82FF8DE8;
	sub_82FD1870(ctx, base);
	// 82FF8DE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8DEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8DF0: 4182000C  beq 0x82ff8dfc
	if ctx.cr[0].eq {
	pc = 0x82FF8DFC; continue 'dispatch;
	}
	// 82FF8DF4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8DF8: 48000008  b 0x82ff8e00
	pc = 0x82FF8E00; continue 'dispatch;
	// 82FF8DFC: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF8E00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF8E04: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF8E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF8E0C: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF8E10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF8E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FF8E18: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FF8E1C: 38C00186  li r6, 0x186
	ctx.r[6].s64 = 390;
	// 82FF8E20: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 82FF8E24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8E28: 4BFFF0D1  bl 0x82ff7ef8
	ctx.lr = 0x82FF8E2C;
	sub_82FF7EF8(ctx, base);
	// 82FF8E2C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF8E30: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FF8E34: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF8E38: 481B7DF1  bl 0x831b0c28
	ctx.lr = 0x82FF8E3C;
	sub_831B0C28(ctx, base);
	// 82FF8E3C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF8E40: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF8E44: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FF8E48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF8E4C: 4099000C  ble cr6, 0x82ff8e58
	if !ctx.cr[6].gt {
	pc = 0x82FF8E58; continue 'dispatch;
	}
	// 82FF8E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8E54: 4BFFFC05  bl 0x82ff8a58
	ctx.lr = 0x82FF8E58;
	sub_82FF8A58(ctx, base);
	// 82FF8E58: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FF8E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8E64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8E68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8E70 size=92
    let mut pc: u32 = 0x82FF8E70;
    'dispatch: loop {
        match pc {
            0x82FF8E70 => {
    //   block [0x82FF8E70..0x82FF8ECC)
	// 82FF8E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF8E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF8E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8E84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8E88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF8E8C: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF8E90: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF8E94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF8E98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF8E9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF8EA0: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF8EA4: 4BFFF28D  bl 0x82ff8130
	ctx.lr = 0x82FF8EA8;
	sub_82FF8130(ctx, base);
	// 82FF8EA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF8EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8EB0: 4BFFFA99  bl 0x82ff8948
	ctx.lr = 0x82FF8EB4;
	sub_82FF8948(ctx, base);
	// 82FF8EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF8EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8EC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF8EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF8EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


