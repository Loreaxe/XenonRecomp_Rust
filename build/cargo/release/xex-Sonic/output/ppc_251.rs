pub fn sub_8307F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F698 size=116
    let mut pc: u32 = 0x8307F698;
    'dispatch: loop {
        match pc {
            0x8307F698 => {
    //   block [0x8307F698..0x8307F70C)
	// 8307F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F6A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307F6A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F6A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F6AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8307F6B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8307F6B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8307F6B8: 57E4103A  slwi r4, r31, 2
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8307F6BC: 4BF62335  bl 0x82fe19f0
	ctx.lr = 0x8307F6C0;
	sub_82FE19F0(ctx, base);
	// 8307F6C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8307F6C4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8307F6C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8307F6CC: 419A0020  beq cr6, 0x8307f6ec
	if ctx.cr[6].eq {
	pc = 0x8307F6EC; continue 'dispatch;
	}
	// 8307F6D0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8307F6D4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8307F6D8: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F6DC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307F6E0: 7D2A412E  stwx r9, r10, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 8307F6E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8307F6E8: 4082FFF0  bne 0x8307f6d8
	if !ctx.cr[0].eq {
	pc = 0x8307F6D8; continue 'dispatch;
	}
	// 8307F6EC: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8307F6F0: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8307F6F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307F6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307F704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F710 size=176
    let mut pc: u32 = 0x8307F710;
    'dispatch: loop {
        match pc {
            0x8307F710 => {
    //   block [0x8307F710..0x8307F7C0)
	// 8307F710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307F71C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F728: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F72C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8307F730: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8307F734: 409A0074  bne cr6, 0x8307f7a8
	if !ctx.cr[6].eq {
	pc = 0x8307F7A8; continue 'dispatch;
	}
	// 8307F738: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8307F73C: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 8307F740: 40980008  bge cr6, 0x8307f748
	if !ctx.cr[6].lt {
	pc = 0x8307F748; continue 'dispatch;
	}
	// 8307F744: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8307F748: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F74C: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8307F750: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F754: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F758: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8307F75C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8307F760: 4E800421  bctrl
	ctx.lr = 0x8307F764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8307F764: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8307F768: 4BF62289  bl 0x82fe19f0
	ctx.lr = 0x8307F76C;
	sub_82FE19F0(ctx, base);
	// 8307F76C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8307F770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8307F774: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8307F778: 40990028  ble cr6, 0x8307f7a0
	if !ctx.cr[6].gt {
	pc = 0x8307F7A0; continue 'dispatch;
	}
	// 8307F77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8307F780: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F784: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8307F788: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8307F78C: 7D2A192E  stwx r9, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 8307F790: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8307F794: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8307F798: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8307F79C: 4198FFE4  blt cr6, 0x8307f780
	if ctx.cr[6].lt {
	pc = 0x8307F780; continue 'dispatch;
	}
	// 8307F7A0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8307F7A4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8307F7A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307F7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F7B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307F7B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307F7C0 size=20
    let mut pc: u32 = 0x8307F7C0;
    'dispatch: loop {
        match pc {
            0x8307F7C0 => {
    //   block [0x8307F7C0..0x8307F7D4)
	// 8307F7C0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F7C4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8307F7C8: 4198000C  blt cr6, 0x8307f7d4
	if ctx.cr[6].lt {
		sub_8307F7D4(ctx, base);
		return;
	}
	// 8307F7CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8307F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F7D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307F7D4 size=16
    let mut pc: u32 = 0x8307F7D4;
    'dispatch: loop {
        match pc {
            0x8307F7D4 => {
    //   block [0x8307F7D4..0x8307F7E4)
	// 8307F7D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F7D8: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F7DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8307F7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F7E8 size=108
    let mut pc: u32 = 0x8307F7E8;
    'dispatch: loop {
        match pc {
            0x8307F7E8 => {
    //   block [0x8307F7E8..0x8307F854)
	// 8307F7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F7EC: 48128981  bl 0x831a816c
	ctx.lr = 0x8307F7F0;
	sub_831A8130(ctx, base);
	// 8307F7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F7F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F7F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8307F7FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8307F800: 4BFFFF11  bl 0x8307f710
	ctx.lr = 0x8307F804;
	sub_8307F710(ctx, base);
	// 8307F804: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F808: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8307F80C: 40990028  ble cr6, 0x8307f834
	if !ctx.cr[6].gt {
	pc = 0x8307F834; continue 'dispatch;
	}
	// 8307F810: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F814: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8307F818: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F81C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307F820: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8307F824: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8307F828: 8109FFFC  lwz r8, -4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8307F82C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8307F830: 4082FFE8  bne 0x8307f818
	if !ctx.cr[0].eq {
	pc = 0x8307F818; continue 'dispatch;
	}
	// 8307F834: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F838: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F83C: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8307F840: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F844: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8307F848: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8307F84C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307F850: 4812896C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307F858 size=80
    let mut pc: u32 = 0x8307F858;
    'dispatch: loop {
        match pc {
            0x8307F858 => {
    //   block [0x8307F858..0x8307F8A8)
	// 8307F858: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F85C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8307F860: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8307F864: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8307F868: 40980030  bge cr6, 0x8307f898
	if !ctx.cr[6].lt {
	pc = 0x8307F898; continue 'dispatch;
	}
	// 8307F86C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F870: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F874: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8307F878: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8307F87C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8307F880: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8307F884: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8307F888: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F88C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8307F890: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8307F894: 4198FFDC  blt cr6, 0x8307f870
	if ctx.cr[6].lt {
	pc = 0x8307F870; continue 'dispatch;
	}
	// 8307F898: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F89C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8307F8A0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8307F8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307F8A8 size=16
    let mut pc: u32 = 0x8307F8A8;
    'dispatch: loop {
        match pc {
            0x8307F8A8 => {
    //   block [0x8307F8A8..0x8307F8B8)
	// 8307F8A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F8AC: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F8B0: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 8307F8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F8B8 size=52
    let mut pc: u32 = 0x8307F8B8;
    'dispatch: loop {
        match pc {
            0x8307F8B8 => {
    //   block [0x8307F8B8..0x8307F8EC)
	// 8307F8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F8C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F8C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F8C8: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8307F8CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F8D0: 4BFFFDC9  bl 0x8307f698
	ctx.lr = 0x8307F8D4;
	sub_8307F698(ctx, base);
	// 8307F8D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307F8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307F8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F8E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F8F0 size=48
    let mut pc: u32 = 0x8307F8F0;
    'dispatch: loop {
        match pc {
            0x8307F8F0 => {
    //   block [0x8307F8F0..0x8307F920)
	// 8307F8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F8F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F8FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F904: 4BFFFD95  bl 0x8307f698
	ctx.lr = 0x8307F908;
	sub_8307F698(ctx, base);
	// 8307F908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307F90C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307F910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F920 size=84
    let mut pc: u32 = 0x8307F920;
    'dispatch: loop {
        match pc {
            0x8307F920 => {
    //   block [0x8307F920..0x8307F974)
	// 8307F920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307F92C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F938: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307F93C: 4BFFFDD5  bl 0x8307f710
	ctx.lr = 0x8307F940;
	sub_8307F710(ctx, base);
	// 8307F940: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F944: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307F948: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8307F94C: 7FCA592E  stwx r30, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 8307F950: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307F954: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8307F958: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8307F95C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307F96C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307F978 size=16
    let mut pc: u32 = 0x8307F978;
    'dispatch: loop {
        match pc {
            0x8307F978 => {
    //   block [0x8307F978..0x8307F988)
	// 8307F978: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307F97C: 396B9ADC  addi r11, r11, -0x6524
	ctx.r[11].s64 = ctx.r[11].s64 + -25892;
	// 8307F980: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307F984: 4BF75F4C  b 0x82ff58d0
	sub_82FF58D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F988 size=88
    let mut pc: u32 = 0x8307F988;
    'dispatch: loop {
        match pc {
            0x8307F988 => {
    //   block [0x8307F988..0x8307F9E0)
	// 8307F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307F994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F99C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307F9A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307F9A4: 396B9ADC  addi r11, r11, -0x6524
	ctx.r[11].s64 = ctx.r[11].s64 + -25892;
	// 8307F9A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307F9AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307F9B0: 4BF75F21  bl 0x82ff58d0
	ctx.lr = 0x8307F9B4;
	sub_82FF58D0(ctx, base);
	// 8307F9B4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307F9B8: 4182000C  beq 0x8307f9c4
	if ctx.cr[0].eq {
	pc = 0x8307F9C4; continue 'dispatch;
	}
	// 8307F9BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307F9C0: 4BF58921  bl 0x82fd82e0
	ctx.lr = 0x8307F9C4;
	sub_82FD82E0(ctx, base);
	// 8307F9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307F9C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307F9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307F9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307F9D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307F9D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307F9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307F9E0 size=136
    let mut pc: u32 = 0x8307F9E0;
    'dispatch: loop {
        match pc {
            0x8307F9E0 => {
    //   block [0x8307F9E0..0x8307FA68)
	// 8307F9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307F9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307F9E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307F9EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307F9F0: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8307F9F4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8307F9F8: 41980008  blt cr6, 0x8307fa00
	if ctx.cr[6].lt {
	pc = 0x8307FA00; continue 'dispatch;
	}
	// 8307F9FC: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8307FA00: 7D5F2214  add r10, r31, r4
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 8307FA04: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8307FA08: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8307FA0C: 40980030  bge cr6, 0x8307fa3c
	if !ctx.cr[6].lt {
	pc = 0x8307FA3C; continue 'dispatch;
	}
	// 8307FA10: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FA14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8307FA18: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8307FA1C: 54E7083E  rotlwi r7, r7, 1
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 8307FA20: 7CE72A2E  lhzx r7, r7, r5
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8307FA24: 2B07FFFF  cmplwi cr6, r7, 0xffff
	ctx.cr[6].compare_u32(ctx.r[7].u32, 65535 as u32, &mut ctx.xer);
	// 8307FA28: 419A000C  beq cr6, 0x8307fa34
	if ctx.cr[6].eq {
	pc = 0x8307FA34; continue 'dispatch;
	}
	// 8307FA2C: B0E60000  sth r7, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8307FA30: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 8307FA34: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8307FA38: 4198FFD8  blt cr6, 0x8307fa10
	if ctx.cr[6].lt {
	pc = 0x8307FA10; continue 'dispatch;
	}
	// 8307FA3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8307FA40: 93E80000  stw r31, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8307FA44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8307FA48: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8307FA4C: 48128795  bl 0x831a81e0
	ctx.lr = 0x8307FA50;
	sub_831A81E0(ctx, base);
	// 8307FA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FA54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307FA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307FA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FA68 size=72
    let mut pc: u32 = 0x8307FA68;
    'dispatch: loop {
        match pc {
            0x8307FA68 => {
    //   block [0x8307FA68..0x8307FAB0)
	// 8307FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FA6C: 481286FD  bl 0x831a8168
	ctx.lr = 0x8307FA70;
	sub_831A8130(ctx, base);
	// 8307FA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FA74: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8307FA78: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 8307FA7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FA80: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8307FA84: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 8307FA88: 4BF75EE1  bl 0x82ff5968
	ctx.lr = 0x8307FA8C;
	sub_82FF5968(ctx, base);
	// 8307FA8C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FA90: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8307FA94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FA98: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8307FA9C: 396B9ADC  addi r11, r11, -0x6524
	ctx.r[11].s64 = ctx.r[11].s64 + -25892;
	// 8307FAA0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8307FAA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307FAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8307FAAC: 4812870C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FAB0 size=56
    let mut pc: u32 = 0x8307FAB0;
    'dispatch: loop {
        match pc {
            0x8307FAB0 => {
    //   block [0x8307FAB0..0x8307FAE8)
	// 8307FAB0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8307FAB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8307FAB8: 80C30018  lwz r6, 0x18(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8307FABC: 5487043E  clrlwi r7, r4, 0x10
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8307FAC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8307FAC4: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8307FAC8: 5529F87E  srwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8307FACC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8307FAD0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8307FAD4: 7D08322E  lhzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 8307FAD8: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8307FADC: 4099000C  ble cr6, 0x8307fae8
	if !ctx.cr[6].gt {
		sub_8307FAE8(ctx, base);
		return;
	}
	// 8307FAE0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8307FAE4: 4800000C  b 0x8307faf0
	sub_8307FAE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FAE8 size=48
    let mut pc: u32 = 0x8307FAE8;
    'dispatch: loop {
        match pc {
            0x8307FAE8 => {
    //   block [0x8307FAE8..0x8307FB18)
	// 8307FAE8: 40980030  bge cr6, 0x8307fb18
	if !ctx.cr[6].lt {
		sub_8307FB18(ctx, base);
		return;
	}
	// 8307FAEC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8307FAF0: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8307FAF4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8307FAF8: 4198FFCC  blt cr6, 0x8307fac4
	if ctx.cr[6].lt {
		sub_8307FAB0(ctx, base);
		return;
	}
	// 8307FAFC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8307FB00: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8307FB04: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FB08: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8307FB0C: 419A0014  beq cr6, 0x8307fb20
	if ctx.cr[6].eq {
		sub_8307FB18(ctx, base);
		return;
	}
	// 8307FB10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8307FB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FB18 size=16
    let mut pc: u32 = 0x8307FB18;
    'dispatch: loop {
        match pc {
            0x8307FB18 => {
    //   block [0x8307FB18..0x8307FB28)
	// 8307FB18: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8307FB1C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8307FB20: 886B0002  lbz r3, 2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8307FB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FB28 size=228
    let mut pc: u32 = 0x8307FB28;
    'dispatch: loop {
        match pc {
            0x8307FB28 => {
    //   block [0x8307FB28..0x8307FC0C)
	// 8307FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FB2C: 48128635  bl 0x831a8160
	ctx.lr = 0x8307FB30;
	sub_831A8130(ctx, base);
	// 8307FB30: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FB34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8307FB38: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8307FB3C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8307FB40: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8307FB44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8307FB48: 41980008  blt cr6, 0x8307fb50
	if ctx.cr[6].lt {
	pc = 0x8307FB50; continue 'dispatch;
	}
	// 8307FB4C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8307FB50: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8307FB54: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8307FB58: 7FAB2214  add r29, r11, r4
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8307FB5C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8307FB60: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8307FB64: 40980040  bge cr6, 0x8307fba4
	if !ctx.cr[6].lt {
	pc = 0x8307FBA4; continue 'dispatch;
	}
	// 8307FB68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8307FB6C: A0850000  lhz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FB70: 4BFFFF41  bl 0x8307fab0
	ctx.lr = 0x8307FB74;
	sub_8307FAB0(ctx, base);
	// 8307FB74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307FB78: 4182000C  beq 0x8307fb84
	if ctx.cr[0].eq {
	pc = 0x8307FB84; continue 'dispatch;
	}
	// 8307FB7C: 987F0000  stb r3, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 8307FB80: 48000014  b 0x8307fb94
	pc = 0x8307FB94; continue 'dispatch;
	// 8307FB84: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8307FB88: 419A002C  beq cr6, 0x8307fbb4
	if ctx.cr[6].eq {
	pc = 0x8307FBB4; continue 'dispatch;
	}
	// 8307FB8C: 3960003F  li r11, 0x3f
	ctx.r[11].s64 = 63;
	// 8307FB90: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8307FB94: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 8307FB98: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8307FB9C: 7F05E840  cmplw cr6, r5, r29
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8307FBA0: 4198FFC8  blt cr6, 0x8307fb68
	if ctx.cr[6].lt {
	pc = 0x8307FB68; continue 'dispatch;
	}
	// 8307FBA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8307FBA8: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8307FBAC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8307FBB0: 48128600  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8307FBB4: A0650000  lhz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FBB8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8307FBBC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8307FBC0: 80FE000C  lwz r7, 0xc(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8307FBC4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8307FBC8: 4BF51CA1  bl 0x82fd1868
	ctx.lr = 0x8307FBCC;
	sub_82FD1868(ctx, base);
	// 8307FBCC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8307FBD0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FBD4: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307FBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8307FBDC: 388B9AEC  addi r4, r11, -0x6514
	ctx.r[4].s64 = ctx.r[11].s64 + -25876;
	// 8307FBE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8307FBE4: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8307FBE8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8307FBEC: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 8307FBF0: 38A00091  li r5, 0x91
	ctx.r[5].s64 = 145;
	// 8307FBF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8307FBF8: 4BF6D0B1  bl 0x82fecca8
	ctx.lr = 0x8307FBFC;
	sub_82FECCA8(ctx, base);
	// 8307FBFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8307FC00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8307FC04: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 8307FC08: 48131021  bl 0x831b0c28
	ctx.lr = 0x8307FC0C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FC10 size=56
    let mut pc: u32 = 0x8307FC10;
    'dispatch: loop {
        match pc {
            0x8307FC10 => {
    //   block [0x8307FC10..0x8307FC48)
	// 8307FC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FC18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FC1C: 5484043E  clrlwi r4, r4, 0x10
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8307FC20: 4BFFFE91  bl 0x8307fab0
	ctx.lr = 0x8307FC24;
	sub_8307FAB0(ctx, base);
	// 8307FC24: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8307FC28: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8307FC2C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8307FC30: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8307FC34: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8307FC38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307FC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FC48 size=96
    let mut pc: u32 = 0x8307FC48;
    'dispatch: loop {
        match pc {
            0x8307FC48 => {
    //   block [0x8307FC48..0x8307FCA8)
	// 8307FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FC50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307FC54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307FC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FC5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307FC60: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8307FC64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FC68: 4BFAF849  bl 0x8302f4b0
	ctx.lr = 0x8307FC6C;
	sub_8302F4B0(ctx, base);
	// 8307FC6C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FC74: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8307FC78: 394B9B28  addi r10, r11, -0x64d8
	ctx.r[10].s64 = ctx.r[11].s64 + -25816;
	// 8307FC7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8307FC80: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8307FC84: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8307FC88: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8307FC8C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8307FC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307FC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FC9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307FCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307FCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FCA8 size=16
    let mut pc: u32 = 0x8307FCA8;
    'dispatch: loop {
        match pc {
            0x8307FCA8 => {
    //   block [0x8307FCA8..0x8307FCB8)
	// 8307FCA8: 98830014  stb r4, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u8 ) };
	// 8307FCAC: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 8307FCB0: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 8307FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FCB8 size=88
    let mut pc: u32 = 0x8307FCB8;
    'dispatch: loop {
        match pc {
            0x8307FCB8 => {
    //   block [0x8307FCB8..0x8307FD10)
	// 8307FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FCC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307FCC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307FCC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FCCC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FCD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FCD4: 396B9B28  addi r11, r11, -0x64d8
	ctx.r[11].s64 = ctx.r[11].s64 + -25816;
	// 8307FCD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307FCDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307FCE0: 4BFAF839  bl 0x8302f518
	ctx.lr = 0x8307FCE4;
	sub_8302F518(ctx, base);
	// 8307FCE4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307FCE8: 4182000C  beq 0x8307fcf4
	if ctx.cr[0].eq {
	pc = 0x8307FCF4; continue 'dispatch;
	}
	// 8307FCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FCF0: 4BF585F1  bl 0x82fd82e0
	ctx.lr = 0x8307FCF4;
	sub_82FD82E0(ctx, base);
	// 8307FCF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307FCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FD04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307FD08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307FD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FD10 size=8
    let mut pc: u32 = 0x8307FD10;
    'dispatch: loop {
        match pc {
            0x8307FD10 => {
    //   block [0x8307FD10..0x8307FD18)
	// 8307FD10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8307FD14: 82169B58  lwz r16, -0x64a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25768 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FD18 size=140
    let mut pc: u32 = 0x8307FD18;
    'dispatch: loop {
        match pc {
            0x8307FD18 => {
    //   block [0x8307FD18..0x8307FDA4)
	// 8307FD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FD20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307FD24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307FD28: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8307FD2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FD30: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FD34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8307FD38: 396B9B3C  addi r11, r11, -0x64c4
	ctx.r[11].s64 = ctx.r[11].s64 + -25796;
	// 8307FD3C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8307FD40: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307FD44: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8307FD48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8307FD4C: 41820018  beq 0x8307fd64
	if ctx.cr[0].eq {
	pc = 0x8307FD64; continue 'dispatch;
	}
	// 8307FD50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FD54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8307FD58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FD5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8307FD60: 4E800421  bctrl
	ctx.lr = 0x8307FD64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8307FD64: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8307FD68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8307FD6C: 41820018  beq 0x8307fd84
	if ctx.cr[0].eq {
	pc = 0x8307FD84; continue 'dispatch;
	}
	// 8307FD70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FD74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8307FD78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FD7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8307FD80: 4E800421  bctrl
	ctx.lr = 0x8307FD84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8307FD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8307FD88: 4BFAF791  bl 0x8302f518
	ctx.lr = 0x8307FD8C;
	sub_8302F518(ctx, base);
	// 8307FD8C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8307FD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FD98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307FD9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307FDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FDA4 size=40
    let mut pc: u32 = 0x8307FDA4;
    'dispatch: loop {
        match pc {
            0x8307FDA4 => {
    //   block [0x8307FDA4..0x8307FDCC)
	// 8307FDA4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8307FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FDB4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8307FDB8: 4BFAF761  bl 0x8302f518
	ctx.lr = 0x8307FDBC;
	sub_8302F518(ctx, base);
	// 8307FDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307FDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FDD0 size=12
    let mut pc: u32 = 0x8307FDD0;
    'dispatch: loop {
        match pc {
            0x8307FDD0 => {
    //   block [0x8307FDD0..0x8307FDDC)
	// 8307FDD0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8307FDD4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8307FDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FDE0 size=76
    let mut pc: u32 = 0x8307FDE0;
    'dispatch: loop {
        match pc {
            0x8307FDE0 => {
    //   block [0x8307FDE0..0x8307FE2C)
	// 8307FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FDE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307FDEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307FDF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FDF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FDF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307FDFC: 4BFFFF1D  bl 0x8307fd18
	ctx.lr = 0x8307FE00;
	sub_8307FD18(ctx, base);
	// 8307FE00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8307FE04: 4182000C  beq 0x8307fe10
	if ctx.cr[0].eq {
	pc = 0x8307FE10; continue 'dispatch;
	}
	// 8307FE08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FE0C: 4BF584D5  bl 0x82fd82e0
	ctx.lr = 0x8307FE10;
	sub_82FD82E0(ctx, base);
	// 8307FE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307FE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FE20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8307FE24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8307FE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FE30 size=8
    let mut pc: u32 = 0x8307FE30;
    'dispatch: loop {
        match pc {
            0x8307FE30 => {
    //   block [0x8307FE30..0x8307FE38)
	// 8307FE30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8307FE34: 82169BA0  lwz r16, -0x6460(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FE38 size=216
    let mut pc: u32 = 0x8307FE38;
    'dispatch: loop {
        match pc {
            0x8307FE38 => {
    //   block [0x8307FE38..0x8307FF10)
	// 8307FE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FE3C: 48128321  bl 0x831a815c
	ctx.lr = 0x8307FE40;
	sub_831A8130(ctx, base);
	// 8307FE40: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8307FE44: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FE48: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 8307FE4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8307FE50: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8307FE54: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8307FE58: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8307FE5C: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8307FE60: 93BF00E4  stw r29, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[29].u32 ) };
	// 8307FE64: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8307FE68: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8307FE6C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8307FE70: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8307FE74: 4BFAF63D  bl 0x8302f4b0
	ctx.lr = 0x8307FE78;
	sub_8302F4B0(ctx, base);
	// 8307FE78: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FE7C: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8307FE80: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8307FE84: 396B9B3C  addi r11, r11, -0x64c4
	ctx.r[11].s64 = ctx.r[11].s64 + -25796;
	// 8307FE88: 937E0018  stw r27, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 8307FE8C: 935E001C  stw r26, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 8307FE90: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307FE94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8307FE98: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8307FE9C: 419A0068  beq cr6, 0x8307ff04
	if ctx.cr[6].eq {
	pc = 0x8307FF04; continue 'dispatch;
	}
	// 8307FEA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8307FEA4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8307FEA8: 4BF583F1  bl 0x82fd8298
	ctx.lr = 0x8307FEAC;
	sub_82FD8298(ctx, base);
	// 8307FEAC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8307FEB0: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8307FEB4: 4182002C  beq 0x8307fee0
	if ctx.cr[0].eq {
	pc = 0x8307FEE0; continue 'dispatch;
	}
	// 8307FEB8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8307FEBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8307FEC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8307FEC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8307FEC8: 4BFCC929  bl 0x8304c7f0
	ctx.lr = 0x8307FECC;
	sub_8304C7F0(ctx, base);
	// 8307FECC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8307FED0: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8307FED4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8307FED8: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8307FEDC: 48000008  b 0x8307fee4
	pc = 0x8307FEE4; continue 'dispatch;
	// 8307FEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8307FEE4: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8307FEE8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8307FEEC: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8307FEF0: 4BFBB261  bl 0x8303b150
	ctx.lr = 0x8307FEF4;
	sub_8303B150(ctx, base);
	// 8307FEF4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8307FEF8: 4BDF8651  bl 0x82e78548
	ctx.lr = 0x8307FEFC;
	sub_82E78548(ctx, base);
	// 8307FEFC: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8307FF00: 4082FFE8  bne 0x8307fee8
	if !ctx.cr[0].eq {
	pc = 0x8307FEE8; continue 'dispatch;
	}
	// 8307FF04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8307FF08: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8307FF0C: 481282A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FF10 size=40
    let mut pc: u32 = 0x8307FF10;
    'dispatch: loop {
        match pc {
            0x8307FF10 => {
    //   block [0x8307FF10..0x8307FF38)
	// 8307FF10: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8307FF14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FF18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FF1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FF20: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8307FF24: 4BFAF5F5  bl 0x8302f518
	ctx.lr = 0x8307FF28;
	sub_8302F518(ctx, base);
	// 8307FF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307FF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FF38 size=44
    let mut pc: u32 = 0x8307FF38;
    'dispatch: loop {
        match pc {
            0x8307FF38 => {
    //   block [0x8307FF38..0x8307FF64)
	// 8307FF38: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8307FF3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FF40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FF44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FF48: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8307FF4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8307FF50: 4BF58391  bl 0x82fd82e0
	ctx.lr = 0x8307FF54;
	sub_82FD82E0(ctx, base);
	// 8307FF54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8307FF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8307FF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8307FF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FF68 size=72
    let mut pc: u32 = 0x8307FF68;
    'dispatch: loop {
        match pc {
            0x8307FF68 => {
    //   block [0x8307FF68..0x8307FFB0)
	// 8307FF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FF6C: 48128201  bl 0x831a816c
	ctx.lr = 0x8307FF70;
	sub_831A8130(ctx, base);
	// 8307FF70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FF74: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8307FF78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8307FF7C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8307FF80: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8307FF84: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8307FF88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FF8C: 4BFAF525  bl 0x8302f4b0
	ctx.lr = 0x8307FF90;
	sub_8302F4B0(ctx, base);
	// 8307FF90: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FF94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8307FF98: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8307FF9C: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 8307FFA0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8307FFA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8307FFA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8307FFAC: 48128210  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FFB0 size=12
    let mut pc: u32 = 0x8307FFB0;
    'dispatch: loop {
        match pc {
            0x8307FFB0 => {
    //   block [0x8307FFB0..0x8307FFBC)
	// 8307FFB0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8307FFB4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307FFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8307FFC0 size=32
    let mut pc: u32 = 0x8307FFC0;
    'dispatch: loop {
        match pc {
            0x8307FFC0 => {
    //   block [0x8307FFC0..0x8307FFE0)
	// 8307FFC0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8307FFC4: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8307FFC8: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 8307FFCC: 808A0018  lwz r4, 0x18(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8307FFD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8307FFD4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8307FFD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8307FFDC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8307FFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8307FFE0 size=88
    let mut pc: u32 = 0x8307FFE0;
    'dispatch: loop {
        match pc {
            0x8307FFE0 => {
    //   block [0x8307FFE0..0x83080038)
	// 8307FFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8307FFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8307FFE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8307FFEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8307FFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8307FFF4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8307FFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8307FFFC: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 83080000: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080004: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080008: 4BFAF511  bl 0x8302f518
	ctx.lr = 0x8308000C;
	sub_8302F518(ctx, base);
	// 8308000C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83080010: 4182000C  beq 0x8308001c
	if ctx.cr[0].eq {
	pc = 0x8308001C; continue 'dispatch;
	}
	// 83080014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080018: 4BF582C9  bl 0x82fd82e0
	ctx.lr = 0x8308001C;
	sub_82FD82E0(ctx, base);
	// 8308001C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308002C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080038 size=8
    let mut pc: u32 = 0x83080038;
    'dispatch: loop {
        match pc {
            0x83080038 => {
    //   block [0x83080038..0x83080040)
	// 83080038: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308003C: 82169C10  lwz r16, -0x63f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080040 size=108
    let mut pc: u32 = 0x83080040;
    'dispatch: loop {
        match pc {
            0x83080040 => {
    //   block [0x83080040..0x830800AC)
	// 83080040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308004C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080050: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83080054: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080058: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308005C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080060: 396B9BF4  addi r11, r11, -0x640c
	ctx.r[11].s64 = ctx.r[11].s64 + -25612;
	// 83080064: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83080068: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308006C: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83080070: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080074: 41820018  beq 0x8308008c
	if ctx.cr[0].eq {
	pc = 0x8308008C; continue 'dispatch;
	}
	// 83080078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308007C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080080: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080088: 4E800421  bctrl
	ctx.lr = 0x8308008C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308008C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080090: 4BFAF489  bl 0x8302f518
	ctx.lr = 0x83080094;
	sub_8302F518(ctx, base);
	// 83080094: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83080098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308009C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830800A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830800A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830800A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830800AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830800AC size=40
    let mut pc: u32 = 0x830800AC;
    'dispatch: loop {
        match pc {
            0x830800AC => {
    //   block [0x830800AC..0x830800D4)
	// 830800AC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830800B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830800B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830800B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830800BC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830800C0: 4BFAF459  bl 0x8302f518
	ctx.lr = 0x830800C4;
	sub_8302F518(ctx, base);
	// 830800C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830800C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830800CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830800D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830800D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830800D8 size=76
    let mut pc: u32 = 0x830800D8;
    'dispatch: loop {
        match pc {
            0x830800D8 => {
    //   block [0x830800D8..0x83080124)
	// 830800D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830800DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830800E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830800E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830800E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830800EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830800F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830800F4: 4BFFFF4D  bl 0x83080040
	ctx.lr = 0x830800F8;
	sub_83080040(ctx, base);
	// 830800F8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830800FC: 4182000C  beq 0x83080108
	if ctx.cr[0].eq {
	pc = 0x83080108; continue 'dispatch;
	}
	// 83080100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080104: 4BF581DD  bl 0x82fd82e0
	ctx.lr = 0x83080108;
	sub_82FD82E0(ctx, base);
	// 83080108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308010C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080118: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308011C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080128 size=8
    let mut pc: u32 = 0x83080128;
    'dispatch: loop {
        match pc {
            0x83080128 => {
    //   block [0x83080128..0x83080130)
	// 83080128: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308012C: 82169C60  lwz r16, -0x63a0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25504 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080130 size=488
    let mut pc: u32 = 0x83080130;
    'dispatch: loop {
        match pc {
            0x83080130 => {
    //   block [0x83080130..0x83080318)
	// 83080130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080134: 48128019  bl 0x831a814c
	ctx.lr = 0x83080138;
	sub_831A8130(ctx, base);
	// 83080138: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 8308013C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080140: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83080144: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080148: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8308014C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 83080150: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83080154: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83080158: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 8308015C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 83080160: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 83080164: 4BFAF34D  bl 0x8302f4b0
	ctx.lr = 0x83080168;
	sub_8302F4B0(ctx, base);
	// 83080168: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308016C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83080170: 939E0020  stw r28, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 83080174: 396B9BF4  addi r11, r11, -0x640c
	ctx.r[11].s64 = ctx.r[11].s64 + -25612;
	// 83080178: 3AC00002  li r22, 2
	ctx.r[22].s64 = 2;
	// 8308017C: 3AA00003  li r21, 3
	ctx.r[21].s64 = 3;
	// 83080180: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83080184: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308018C: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83080190: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83080194: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080198: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8308019C: 409A0088  bne cr6, 0x83080224
	if !ctx.cr[6].eq {
	pc = 0x83080224; continue 'dispatch;
	}
	// 830801A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830801A4: 92DE0014  stw r22, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[22].u32 ) };
	// 830801A8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830801AC: 4BF580ED  bl 0x82fd8298
	ctx.lr = 0x830801B0;
	sub_82FD8298(ctx, base);
	// 830801B0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830801B4: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830801B8: 4182002C  beq 0x830801e4
	if ctx.cr[0].eq {
	pc = 0x830801E4; continue 'dispatch;
	}
	// 830801BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830801C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830801C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830801C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830801CC: 4BF52875  bl 0x82fd2a40
	ctx.lr = 0x830801D0;
	sub_82FD2A40(ctx, base);
	// 830801D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830801D4: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 830801D8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830801DC: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830801E0: 48000008  b 0x830801e8
	pc = 0x830801E8; continue 'dispatch;
	// 830801E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830801E8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830801EC: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830801F0: 81770028  lwz r11, 0x28(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(40 as u32) ) } as u64;
	// 830801F4: 806A007C  lwz r3, 0x7c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) } as u64;
	// 830801F8: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830801FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080200: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83080204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080208: 4E800421  bctrl
	ctx.lr = 0x8308020C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308020C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83080210: 4BF50971  bl 0x82fd0b80
	ctx.lr = 0x83080214;
	sub_82FD0B80(ctx, base);
	// 83080214: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83080218: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8308021C: 4BFBAF35  bl 0x8303b150
	ctx.lr = 0x83080220;
	sub_8303B150(ctx, base);
	// 83080220: 480000CC  b 0x830802ec
	pc = 0x830802EC; continue 'dispatch;
	// 83080224: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 83080228: 409A00C4  bne cr6, 0x830802ec
	if !ctx.cr[6].eq {
	pc = 0x830802EC; continue 'dispatch;
	}
	// 8308022C: 92BE0014  stw r21, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[21].u32 ) };
	// 83080230: 83170038  lwz r24, 0x38(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(56 as u32) ) } as u64;
	// 83080234: 28180000  cmplwi r24, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080238: 418200B4  beq 0x830802ec
	if ctx.cr[0].eq {
	pc = 0x830802EC; continue 'dispatch;
	}
	// 8308023C: 83580004  lwz r26, 4(r24)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 83080240: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080244: 418200A8  beq 0x830802ec
	if ctx.cr[0].eq {
	pc = 0x830802EC; continue 'dispatch;
	}
	// 83080248: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8308024C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83080250: 4BF58049  bl 0x82fd8298
	ctx.lr = 0x83080254;
	sub_82FD8298(ctx, base);
	// 83080254: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83080258: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8308025C: 4182002C  beq 0x83080288
	if ctx.cr[0].eq {
	pc = 0x83080288; continue 'dispatch;
	}
	// 83080260: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83080264: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83080268: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8308026C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83080270: 4BF527D1  bl 0x82fd2a40
	ctx.lr = 0x83080274;
	sub_82FD2A40(ctx, base);
	// 83080274: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83080278: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 8308027C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83080280: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83080284: 48000008  b 0x8308028c
	pc = 0x8308028C; continue 'dispatch;
	// 83080288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308028C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83080290: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83080294: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83080298: 419A0054  beq cr6, 0x830802ec
	if ctx.cr[6].eq {
	pc = 0x830802EC; continue 'dispatch;
	}
	// 8308029C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830802A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830802A4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830802A8: 836B007C  lwz r27, 0x7c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830802AC: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830802B0: 4BF77E21  bl 0x82ff80d0
	ctx.lr = 0x830802B4;
	sub_82FF80D0(ctx, base);
	// 830802B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830802B8: 81590024  lwz r10, 0x24(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 830802BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830802C0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830802C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830802C8: 4E800421  bctrl
	ctx.lr = 0x830802CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830802CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830802D0: 4BF508B1  bl 0x82fd0b80
	ctx.lr = 0x830802D4;
	sub_82FD0B80(ctx, base);
	// 830802D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830802D8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830802DC: 4BFBAE75  bl 0x8303b150
	ctx.lr = 0x830802E0;
	sub_8303B150(ctx, base);
	// 830802E0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830802E4: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830802E8: 4198FFB4  blt cr6, 0x8308029c
	if ctx.cr[6].lt {
	pc = 0x8308029C; continue 'dispatch;
	}
	// 830802EC: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830802F0: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830802F4: 409A000C  bne cr6, 0x83080300
	if !ctx.cr[6].eq {
	pc = 0x83080300; continue 'dispatch;
	}
	// 830802F8: 92DE0018  stw r22, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[22].u32 ) };
	// 830802FC: 48000010  b 0x8308030c
	pc = 0x8308030C; continue 'dispatch;
	// 83080300: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83080304: 409A0008  bne cr6, 0x8308030c
	if !ctx.cr[6].eq {
	pc = 0x8308030C; continue 'dispatch;
	}
	// 83080308: 92BE0018  stw r21, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[21].u32 ) };
	// 8308030C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080310: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83080314: 48127E88  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080318 size=40
    let mut pc: u32 = 0x83080318;
    'dispatch: loop {
        match pc {
            0x83080318 => {
    //   block [0x83080318..0x83080340)
	// 83080318: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8308031C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080320: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080328: 807F00D4  lwz r3, 0xd4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8308032C: 4BFAF1ED  bl 0x8302f518
	ctx.lr = 0x83080330;
	sub_8302F518(ctx, base);
	// 83080330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080340 size=44
    let mut pc: u32 = 0x83080340;
    'dispatch: loop {
        match pc {
            0x83080340 => {
    //   block [0x83080340..0x8308036C)
	// 83080340: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83080344: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080348: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308034C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080350: 809F00F4  lwz r4, 0xf4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 83080354: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83080358: 4BF57F89  bl 0x82fd82e0
	ctx.lr = 0x8308035C;
	sub_82FD82E0(ctx, base);
	// 8308035C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308036C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308036C size=44
    let mut pc: u32 = 0x8308036C;
    'dispatch: loop {
        match pc {
            0x8308036C => {
    //   block [0x8308036C..0x83080398)
	// 8308036C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83080370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308037C: 809F00F4  lwz r4, 0xf4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 83080380: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83080384: 4BF57F5D  bl 0x82fd82e0
	ctx.lr = 0x83080388;
	sub_82FD82E0(ctx, base);
	// 83080388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308038C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080398 size=140
    let mut pc: u32 = 0x83080398;
    'dispatch: loop {
        match pc {
            0x83080398 => {
    //   block [0x83080398..0x83080424)
	// 83080398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830803A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830803A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830803A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830803AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830803B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830803B4: 48000014  b 0x830803c8
	pc = 0x830803C8; continue 'dispatch;
	// 830803B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830803BC: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830803C0: 4BFFFFD9  bl 0x83080398
	ctx.lr = 0x830803C4;
	sub_83080398(ctx, base);
	// 830803C4: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830803C8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830803CC: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 830803D0: 419AFFE8  beq cr6, 0x830803b8
	if ctx.cr[6].eq {
	pc = 0x830803B8; continue 'dispatch;
	}
	// 830803D4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830803D8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830803DC: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830803E0: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830803E4: 808A0020  lwz r4, 0x20(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 830803E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830803EC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830803F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830803F4: 4E800421  bctrl
	ctx.lr = 0x830803F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830803F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830803FC: 4BF50785  bl 0x82fd0b80
	ctx.lr = 0x83080400;
	sub_82FD0B80(ctx, base);
	// 83080400: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83080404: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83080408: 4BFBAD49  bl 0x8303b150
	ctx.lr = 0x8308040C;
	sub_8303B150(ctx, base);
	// 8308040C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080418: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308041C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080428 size=8
    let mut pc: u32 = 0x83080428;
    'dispatch: loop {
        match pc {
            0x83080428 => {
    //   block [0x83080428..0x83080430)
	// 83080428: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308042C: 82169CC8  lwz r16, -0x6338(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25400 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080430 size=528
    let mut pc: u32 = 0x83080430;
    'dispatch: loop {
        match pc {
            0x83080430 => {
    //   block [0x83080430..0x83080640)
	// 83080430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080434: 48127D2D  bl 0x831a8160
	ctx.lr = 0x83080438;
	sub_831A8130(ctx, base);
	// 83080438: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8308043C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080440: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83080444: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080448: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8308044C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83080450: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83080454: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83080458: 939F00C4  stw r28, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[28].u32 ) };
	// 8308045C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 83080460: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83080464: 4BFAF04D  bl 0x8302f4b0
	ctx.lr = 0x83080468;
	sub_8302F4B0(ctx, base);
	// 83080468: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308046C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83080470: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83080474: 396B9BF4  addi r11, r11, -0x640c
	ctx.r[11].s64 = ctx.r[11].s64 + -25612;
	// 83080478: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 8308047C: 3B496C68  addi r26, r9, 0x6c68
	ctx.r[26].s64 = ctx.r[9].s64 + 27752;
	// 83080480: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83080484: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308048C: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83080490: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83080494: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080498: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8308049C: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830804A0: 409A002C  bne cr6, 0x830804cc
	if !ctx.cr[6].eq {
	pc = 0x830804CC; continue 'dispatch;
	}
	// 830804A4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830804A8: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 830804AC: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830804B0: 409A000C  bne cr6, 0x830804bc
	if !ctx.cr[6].eq {
	pc = 0x830804BC; continue 'dispatch;
	}
	// 830804B4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 830804B8: 48000038  b 0x830804f0
	pc = 0x830804F0; continue 'dispatch;
	// 830804BC: 2F0B0027  cmpwi cr6, r11, 0x27
	ctx.cr[6].compare_i32(ctx.r[11].s32, 39, &mut ctx.xer);
	// 830804C0: 409A00E0  bne cr6, 0x830805a0
	if !ctx.cr[6].eq {
	pc = 0x830805A0; continue 'dispatch;
	}
	// 830804C4: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 830804C8: 480000D8  b 0x830805a0
	pc = 0x830805A0; continue 'dispatch;
	// 830804CC: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 830804D0: 409A0028  bne cr6, 0x830804f8
	if !ctx.cr[6].eq {
	pc = 0x830804F8; continue 'dispatch;
	}
	// 830804D4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830804D8: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 830804DC: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830804E0: 419AFFE4  beq cr6, 0x830804c4
	if ctx.cr[6].eq {
	pc = 0x830804C4; continue 'dispatch;
	}
	// 830804E4: 2F0B0028  cmpwi cr6, r11, 0x28
	ctx.cr[6].compare_i32(ctx.r[11].s32, 40, &mut ctx.xer);
	// 830804E8: 409A00B8  bne cr6, 0x830805a0
	if !ctx.cr[6].eq {
	pc = 0x830805A0; continue 'dispatch;
	}
	// 830804EC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830804F0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830804F4: 480000AC  b 0x830805a0
	pc = 0x830805A0; continue 'dispatch;
	// 830804F8: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 830804FC: 409A0084  bne cr6, 0x83080580
	if !ctx.cr[6].eq {
	pc = 0x83080580; continue 'dispatch;
	}
	// 83080500: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83080504: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83080508: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308050C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080510: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 83080514: 409A000C  bne cr6, 0x83080520
	if !ctx.cr[6].eq {
	pc = 0x83080520; continue 'dispatch;
	}
	// 83080518: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8308051C: 48000014  b 0x83080530
	pc = 0x83080530; continue 'dispatch;
	// 83080520: 2F0B0028  cmpwi cr6, r11, 0x28
	ctx.cr[6].compare_i32(ctx.r[11].s32, 40, &mut ctx.xer);
	// 83080524: 409A000C  bne cr6, 0x83080530
	if !ctx.cr[6].eq {
	pc = 0x83080530; continue 'dispatch;
	}
	// 83080528: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8308052C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83080530: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83080534: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83080538: 4BF57D61  bl 0x82fd8298
	ctx.lr = 0x8308053C;
	sub_82FD8298(ctx, base);
	// 8308053C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83080540: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83080544: 41820024  beq 0x83080568
	if ctx.cr[0].eq {
	pc = 0x83080568; continue 'dispatch;
	}
	// 83080548: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8308054C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83080550: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83080554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83080558: 4BF524E9  bl 0x82fd2a40
	ctx.lr = 0x8308055C;
	sub_82FD2A40(ctx, base);
	// 8308055C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83080560: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 83080564: 48000008  b 0x8308056c
	pc = 0x8308056C; continue 'dispatch;
	// 83080568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308056C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83080570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080574: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83080578: 4BFFFE21  bl 0x83080398
	ctx.lr = 0x8308057C;
	sub_83080398(ctx, base);
	// 8308057C: 48000024  b 0x830805a0
	pc = 0x830805A0; continue 'dispatch;
	// 83080580: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 83080584: 409A000C  bne cr6, 0x83080590
	if !ctx.cr[6].eq {
	pc = 0x83080590; continue 'dispatch;
	}
	// 83080588: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8308058C: 48000010  b 0x8308059c
	pc = 0x8308059C; continue 'dispatch;
	// 83080590: 2F0B0026  cmpwi cr6, r11, 0x26
	ctx.cr[6].compare_i32(ctx.r[11].s32, 38, &mut ctx.xer);
	// 83080594: 409A000C  bne cr6, 0x830805a0
	if !ctx.cr[6].eq {
	pc = 0x830805A0; continue 'dispatch;
	}
	// 83080598: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8308059C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830805A0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830805A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830805A8: 419A0018  beq cr6, 0x830805c0
	if ctx.cr[6].eq {
	pc = 0x830805C0; continue 'dispatch;
	}
	// 830805AC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830805B0: 409A0084  bne cr6, 0x83080634
	if !ctx.cr[6].eq {
	pc = 0x83080634; continue 'dispatch;
	}
	// 830805B4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830805B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830805BC: 409A0078  bne cr6, 0x83080634
	if !ctx.cr[6].eq {
	pc = 0x83080634; continue 'dispatch;
	}
	// 830805C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830805C4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830805C8: 4BF57CD1  bl 0x82fd8298
	ctx.lr = 0x830805CC;
	sub_82FD8298(ctx, base);
	// 830805CC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830805D0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830805D4: 41820024  beq 0x830805f8
	if ctx.cr[0].eq {
	pc = 0x830805F8; continue 'dispatch;
	}
	// 830805D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830805DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830805E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830805E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830805E8: 4BF52459  bl 0x82fd2a40
	ctx.lr = 0x830805EC;
	sub_82FD2A40(ctx, base);
	// 830805EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830805F0: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 830805F4: 48000008  b 0x830805fc
	pc = 0x830805FC; continue 'dispatch;
	// 830805F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830805FC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080600: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83080604: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080608: 806A007C  lwz r3, 0x7c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) } as u64;
	// 8308060C: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83080610: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080614: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83080618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308061C: 4E800421  bctrl
	ctx.lr = 0x83080620;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080620: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83080624: 4BF5055D  bl 0x82fd0b80
	ctx.lr = 0x83080628;
	sub_82FD0B80(ctx, base);
	// 83080628: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8308062C: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83080630: 4BFBAB21  bl 0x8303b150
	ctx.lr = 0x83080634;
	sub_8303B150(ctx, base);
	// 83080634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080638: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8308063C: 48127B74  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080640 size=40
    let mut pc: u32 = 0x83080640;
    'dispatch: loop {
        match pc {
            0x83080640 => {
    //   block [0x83080640..0x83080668)
	// 83080640: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83080644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308064C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080650: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83080654: 4BFAEEC5  bl 0x8302f518
	ctx.lr = 0x83080658;
	sub_8302F518(ctx, base);
	// 83080658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308065C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080668 size=44
    let mut pc: u32 = 0x83080668;
    'dispatch: loop {
        match pc {
            0x83080668 => {
    //   block [0x83080668..0x83080694)
	// 83080668: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8308066C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080670: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080678: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8308067C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83080680: 4BF57C61  bl 0x82fd82e0
	ctx.lr = 0x83080684;
	sub_82FD82E0(ctx, base);
	// 83080684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308068C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080694 size=44
    let mut pc: u32 = 0x83080694;
    'dispatch: loop {
        match pc {
            0x83080694 => {
    //   block [0x83080694..0x830806C0)
	// 83080694: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83080698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830806A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830806A4: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830806A8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830806AC: 4BF57C35  bl 0x82fd82e0
	ctx.lr = 0x830806B0;
	sub_82FD82E0(ctx, base);
	// 830806B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830806B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830806B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830806BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830806C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830806C0 size=84
    let mut pc: u32 = 0x830806C0;
    'dispatch: loop {
        match pc {
            0x830806C0 => {
    //   block [0x830806C0..0x83080714)
	// 830806C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830806C4: 48127AA1  bl 0x831a8164
	ctx.lr = 0x830806C8;
	sub_831A8130(ctx, base);
	// 830806C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830806CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830806D0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830806D4: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 830806D8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830806DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830806E0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830806E4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830806E8: 4BFAEDC9  bl 0x8302f4b0
	ctx.lr = 0x830806EC;
	sub_8302F4B0(ctx, base);
	// 830806EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830806F0: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830806F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830806F8: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 830806FC: 396B9D18  addi r11, r11, -0x62e8
	ctx.r[11].s64 = ctx.r[11].s64 + -25320;
	// 83080700: 937F001C  stw r27, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 83080704: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83080708: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83080710: 48127AA4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080718 size=8
    let mut pc: u32 = 0x83080718;
    'dispatch: loop {
        match pc {
            0x83080718 => {
    //   block [0x83080718..0x83080720)
	// 83080718: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308071C: 82169D38  lwz r16, -0x62c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080720 size=120
    let mut pc: u32 = 0x83080720;
    'dispatch: loop {
        match pc {
            0x83080720 => {
    //   block [0x83080720..0x83080798)
	// 83080720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308072C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080730: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83080734: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080738: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308073C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080740: 396B9D18  addi r11, r11, -0x62e8
	ctx.r[11].s64 = ctx.r[11].s64 + -25320;
	// 83080744: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83080748: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308074C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83080750: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080754: 41820024  beq 0x83080778
	if ctx.cr[0].eq {
	pc = 0x83080778; continue 'dispatch;
	}
	// 83080758: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308075C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83080760: 409A0018  bne cr6, 0x83080778
	if !ctx.cr[6].eq {
	pc = 0x83080778; continue 'dispatch;
	}
	// 83080764: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080768: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8308076C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080774: 4E800421  bctrl
	ctx.lr = 0x83080778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308077C: 4BFAED9D  bl 0x8302f518
	ctx.lr = 0x83080780;
	sub_8302F518(ctx, base);
	// 83080780: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83080784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308078C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080798 size=40
    let mut pc: u32 = 0x83080798;
    'dispatch: loop {
        match pc {
            0x83080798 => {
    //   block [0x83080798..0x830807C0)
	// 83080798: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308079C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830807A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830807A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830807A8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830807AC: 4BFAED6D  bl 0x8302f518
	ctx.lr = 0x830807B0;
	sub_8302F518(ctx, base);
	// 830807B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830807B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830807B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830807BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830807C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830807C0 size=76
    let mut pc: u32 = 0x830807C0;
    'dispatch: loop {
        match pc {
            0x830807C0 => {
    //   block [0x830807C0..0x8308080C)
	// 830807C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830807C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830807C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830807CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830807D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830807D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830807D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830807DC: 4BFFFF45  bl 0x83080720
	ctx.lr = 0x830807E0;
	sub_83080720(ctx, base);
	// 830807E0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830807E4: 4182000C  beq 0x830807f0
	if ctx.cr[0].eq {
	pc = 0x830807F0; continue 'dispatch;
	}
	// 830807E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830807EC: 4BF57AF5  bl 0x82fd82e0
	ctx.lr = 0x830807F0;
	sub_82FD82E0(ctx, base);
	// 830807F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830807F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830807F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830807FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080800: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080810 size=8
    let mut pc: u32 = 0x83080810;
    'dispatch: loop {
        match pc {
            0x83080810 => {
    //   block [0x83080810..0x83080818)
	// 83080810: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83080814: 82169D98  lwz r16, -0x6268(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080818 size=236
    let mut pc: u32 = 0x83080818;
    'dispatch: loop {
        match pc {
            0x83080818 => {
    //   block [0x83080818..0x83080904)
	// 83080818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080828: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8308082C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080830: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080834: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080838: 396B9D70  addi r11, r11, -0x6290
	ctx.r[11].s64 = ctx.r[11].s64 + -25232;
	// 8308083C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83080840: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080844: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83080848: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308084C: 41820018  beq 0x83080864
	if ctx.cr[0].eq {
	pc = 0x83080864; continue 'dispatch;
	}
	// 83080850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080854: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080858: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308085C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080860: 4E800421  bctrl
	ctx.lr = 0x83080864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080864: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83080868: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308086C: 41820018  beq 0x83080884
	if ctx.cr[0].eq {
	pc = 0x83080884; continue 'dispatch;
	}
	// 83080870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080874: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080878: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308087C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080880: 4E800421  bctrl
	ctx.lr = 0x83080884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080884: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83080888: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308088C: 41820018  beq 0x830808a4
	if ctx.cr[0].eq {
	pc = 0x830808A4; continue 'dispatch;
	}
	// 83080890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080894: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080898: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308089C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830808A0: 4E800421  bctrl
	ctx.lr = 0x830808A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830808A4: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 830808A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830808AC: 41820018  beq 0x830808c4
	if ctx.cr[0].eq {
	pc = 0x830808C4; continue 'dispatch;
	}
	// 830808B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830808B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830808B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830808BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830808C0: 4E800421  bctrl
	ctx.lr = 0x830808C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830808C4: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830808C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830808CC: 41820018  beq 0x830808e4
	if ctx.cr[0].eq {
	pc = 0x830808E4; continue 'dispatch;
	}
	// 830808D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830808D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830808D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830808DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830808E0: 4E800421  bctrl
	ctx.lr = 0x830808E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830808E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830808E8: 48021499  bl 0x830a1d80
	ctx.lr = 0x830808EC;
	sub_830A1D80(ctx, base);
	// 830808EC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830808F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830808F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830808F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830808FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080904 size=40
    let mut pc: u32 = 0x83080904;
    'dispatch: loop {
        match pc {
            0x83080904 => {
    //   block [0x83080904..0x8308092C)
	// 83080904: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83080908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080914: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83080918: 48021469  bl 0x830a1d80
	ctx.lr = 0x8308091C;
	sub_830A1D80(ctx, base);
	// 8308091C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080930 size=12
    let mut pc: u32 = 0x83080930;
    'dispatch: loop {
        match pc {
            0x83080930 => {
    //   block [0x83080930..0x8308093C)
	// 83080930: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83080934: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83080938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080940 size=12
    let mut pc: u32 = 0x83080940;
    'dispatch: loop {
        match pc {
            0x83080940 => {
    //   block [0x83080940..0x8308094C)
	// 83080940: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83080944: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83080948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080950 size=12
    let mut pc: u32 = 0x83080950;
    'dispatch: loop {
        match pc {
            0x83080950 => {
    //   block [0x83080950..0x8308095C)
	// 83080950: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83080954: 886B0008  lbz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080960 size=124
    let mut pc: u32 = 0x83080960;
    'dispatch: loop {
        match pc {
            0x83080960 => {
    //   block [0x83080960..0x830809DC)
	// 83080960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308096C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080970: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83080974: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83080978: 409A000C  bne cr6, 0x83080984
	if !ctx.cr[6].eq {
	pc = 0x83080984; continue 'dispatch;
	}
	// 8308097C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83080980: 48000048  b 0x830809c8
	pc = 0x830809C8; continue 'dispatch;
	// 83080984: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83080988: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 8308098C: 419AFFF0  beq cr6, 0x8308097c
	if ctx.cr[6].eq {
	pc = 0x8308097C; continue 'dispatch;
	}
	// 83080990: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83080994: 419A0024  beq cr6, 0x830809b8
	if ctx.cr[6].eq {
	pc = 0x830809B8; continue 'dispatch;
	}
	// 83080998: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8308099C: 419A001C  beq cr6, 0x830809b8
	if ctx.cr[6].eq {
	pc = 0x830809B8; continue 'dispatch;
	}
	// 830809A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830809A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830809A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830809AC: 4E800421  bctrl
	ctx.lr = 0x830809B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830809B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830809B4: 4082FFE4  bne 0x83080998
	if !ctx.cr[0].eq {
	pc = 0x83080998; continue 'dispatch;
	}
	// 830809B8: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 830809BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830809C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830809C4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830809C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830809CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830809D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830809D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830809D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830809E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830809E0 size=24
    let mut pc: u32 = 0x830809E0;
    'dispatch: loop {
        match pc {
            0x830809E0 => {
    //   block [0x830809E0..0x830809F8)
	// 830809E0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 830809E4: 90A30024  stw r5, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 830809E8: 90C30030  stw r6, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 830809EC: 90E30034  stw r7, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 830809F0: 91030038  stw r8, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[8].u32 ) };
	// 830809F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830809F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830809F8 size=76
    let mut pc: u32 = 0x830809F8;
    'dispatch: loop {
        match pc {
            0x830809F8 => {
    //   block [0x830809F8..0x83080A44)
	// 830809F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830809FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080A0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080A10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080A14: 4BFFFE05  bl 0x83080818
	ctx.lr = 0x83080A18;
	sub_83080818(ctx, base);
	// 83080A18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83080A1C: 4182000C  beq 0x83080a28
	if ctx.cr[0].eq {
	pc = 0x83080A28; continue 'dispatch;
	}
	// 83080A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080A24: 4BF578BD  bl 0x82fd82e0
	ctx.lr = 0x83080A28;
	sub_82FD82E0(ctx, base);
	// 83080A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080A2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080A38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080A48 size=8
    let mut pc: u32 = 0x83080A48;
    'dispatch: loop {
        match pc {
            0x83080A48 => {
    //   block [0x83080A48..0x83080A50)
	// 83080A48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83080A4C: 82169DE0  lwz r16, -0x6220(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25120 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080A50 size=328
    let mut pc: u32 = 0x83080A50;
    'dispatch: loop {
        match pc {
            0x83080A50 => {
    //   block [0x83080A50..0x83080B98)
	// 83080A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080A54: 48127701  bl 0x831a8154
	ctx.lr = 0x83080A58;
	sub_831A8130(ctx, base);
	// 83080A58: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83080A5C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080A60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080A64: 831F0104  lwz r24, 0x104(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83080A68: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83080A6C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83080A70: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83080A74: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83080A78: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 83080A7C: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83080A80: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 83080A84: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83080A88: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 83080A8C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 83080A90: 480212A1  bl 0x830a1d30
	ctx.lr = 0x83080A94;
	sub_830A1D30(ctx, base);
	// 83080A94: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080A98: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83080A9C: 937E0028  stw r27, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 83080AA0: 396B9D70  addi r11, r11, -0x6290
	ctx.r[11].s64 = ctx.r[11].s64 + -25232;
	// 83080AA4: 939E002C  stw r28, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 83080AA8: 935E003C  stw r26, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 83080AAC: 933E0040  stw r25, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[25].u32 ) };
	// 83080AB0: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83080AB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080AB8: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 83080ABC: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 83080AC0: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 83080AC4: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83080AC8: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83080ACC: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83080AD0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83080AD4: 41820054  beq 0x83080b28
	if ctx.cr[0].eq {
	pc = 0x83080B28; continue 'dispatch;
	}
	// 83080AD8: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83080ADC: 41820010  beq 0x83080aec
	if ctx.cr[0].eq {
	pc = 0x83080AEC; continue 'dispatch;
	}
	// 83080AE0: A15E0018  lhz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080AE4: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 83080AE8: B15E0018  sth r10, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83080AEC: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83080AF0: 41820010  beq 0x83080b00
	if ctx.cr[0].eq {
	pc = 0x83080B00; continue 'dispatch;
	}
	// 83080AF4: A15E0018  lhz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080AF8: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 83080AFC: B15E0018  sth r10, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83080B00: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83080B04: 41820010  beq 0x83080b14
	if ctx.cr[0].eq {
	pc = 0x83080B14; continue 'dispatch;
	}
	// 83080B08: A15E0018  lhz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080B0C: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 83080B10: B15E0018  sth r10, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83080B14: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83080B18: 41820010  beq 0x83080b28
	if ctx.cr[0].eq {
	pc = 0x83080B28; continue 'dispatch;
	}
	// 83080B1C: A17E0018  lhz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83080B20: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 83080B24: B17E0018  sth r11, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 83080B28: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83080B2C: 419A0060  beq cr6, 0x83080b8c
	if ctx.cr[6].eq {
	pc = 0x83080B8C; continue 'dispatch;
	}
	// 83080B30: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83080B34: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83080B38: 4BF57761  bl 0x82fd8298
	ctx.lr = 0x83080B3C;
	sub_82FD8298(ctx, base);
	// 83080B3C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83080B40: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83080B44: 41820028  beq 0x83080b6c
	if ctx.cr[0].eq {
	pc = 0x83080B6C; continue 'dispatch;
	}
	// 83080B48: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83080B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83080B50: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83080B54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83080B58: 4BFCBC99  bl 0x8304c7f0
	ctx.lr = 0x83080B5C;
	sub_8304C7F0(ctx, base);
	// 83080B5C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83080B60: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83080B64: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 83080B68: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080B6C: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83080B70: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83080B74: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83080B78: 4BFBA5D9  bl 0x8303b150
	ctx.lr = 0x83080B7C;
	sub_8303B150(ctx, base);
	// 83080B7C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83080B80: 4BDF79C9  bl 0x82e78548
	ctx.lr = 0x83080B84;
	sub_82E78548(ctx, base);
	// 83080B84: 7C771B79  or. r23, r3, r3
	ctx.r[23].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83080B88: 4082FFE8  bne 0x83080b70
	if !ctx.cr[0].eq {
	pc = 0x83080B70; continue 'dispatch;
	}
	// 83080B8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080B90: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83080B94: 48127610  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080B98 size=40
    let mut pc: u32 = 0x83080B98;
    'dispatch: loop {
        match pc {
            0x83080B98 => {
    //   block [0x83080B98..0x83080BC0)
	// 83080B98: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83080B9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080BA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080BA8: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83080BAC: 480211D5  bl 0x830a1d80
	ctx.lr = 0x83080BB0;
	sub_830A1D80(ctx, base);
	// 83080BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080BC0 size=44
    let mut pc: u32 = 0x83080BC0;
    'dispatch: loop {
        match pc {
            0x83080BC0 => {
    //   block [0x83080BC0..0x83080BEC)
	// 83080BC0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83080BC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080BC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080BD0: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83080BD4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83080BD8: 4BF57709  bl 0x82fd82e0
	ctx.lr = 0x83080BDC;
	sub_82FD82E0(ctx, base);
	// 83080BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080BF0 size=88
    let mut pc: u32 = 0x83080BF0;
    'dispatch: loop {
        match pc {
            0x83080BF0 => {
    //   block [0x83080BF0..0x83080C48)
	// 83080BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080BF4: 48127571  bl 0x831a8164
	ctx.lr = 0x83080BF8;
	sub_831A8130(ctx, base);
	// 83080BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080BFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080C00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83080C04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83080C08: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 83080C0C: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 83080C10: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 83080C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080C18: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83080C1C: 4BFAE895  bl 0x8302f4b0
	ctx.lr = 0x83080C20;
	sub_8302F4B0(ctx, base);
	// 83080C20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080C24: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83080C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080C2C: 9B9F0018  stb r28, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u8 ) };
	// 83080C30: 396B9B28  addi r11, r11, -0x64d8
	ctx.r[11].s64 = ctx.r[11].s64 + -25816;
	// 83080C34: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 83080C38: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 83080C3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83080C44: 48127570  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080C48 size=8
    let mut pc: u32 = 0x83080C48;
    'dispatch: loop {
        match pc {
            0x83080C48 => {
    //   block [0x83080C48..0x83080C50)
	// 83080C48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83080C4C: 82169E40  lwz r16, -0x61c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-25024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080C50 size=108
    let mut pc: u32 = 0x83080C50;
    'dispatch: loop {
        match pc {
            0x83080C50 => {
    //   block [0x83080C50..0x83080CBC)
	// 83080C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080C60: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83080C64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080C68: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080C6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080C70: 396B9E20  addi r11, r11, -0x61e0
	ctx.r[11].s64 = ctx.r[11].s64 + -25056;
	// 83080C74: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83080C78: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080C7C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83080C80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080C84: 41820018  beq 0x83080c9c
	if ctx.cr[0].eq {
	pc = 0x83080C9C; continue 'dispatch;
	}
	// 83080C88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080C8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080C90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080C98: 4E800421  bctrl
	ctx.lr = 0x83080C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080C9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080CA0: 4BFAE879  bl 0x8302f518
	ctx.lr = 0x83080CA4;
	sub_8302F518(ctx, base);
	// 83080CA4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83080CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080CB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080CBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080CBC size=40
    let mut pc: u32 = 0x83080CBC;
    'dispatch: loop {
        match pc {
            0x83080CBC => {
    //   block [0x83080CBC..0x83080CE4)
	// 83080CBC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83080CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080CCC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83080CD0: 4BFAE849  bl 0x8302f518
	ctx.lr = 0x83080CD4;
	sub_8302F518(ctx, base);
	// 83080CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080CE8 size=76
    let mut pc: u32 = 0x83080CE8;
    'dispatch: loop {
        match pc {
            0x83080CE8 => {
    //   block [0x83080CE8..0x83080D34)
	// 83080CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080D04: 4BFFFF4D  bl 0x83080c50
	ctx.lr = 0x83080D08;
	sub_83080C50(ctx, base);
	// 83080D08: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83080D0C: 4182000C  beq 0x83080d18
	if ctx.cr[0].eq {
	pc = 0x83080D18; continue 'dispatch;
	}
	// 83080D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080D14: 4BF575CD  bl 0x82fd82e0
	ctx.lr = 0x83080D18;
	sub_82FD82E0(ctx, base);
	// 83080D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080D28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080D38 size=8
    let mut pc: u32 = 0x83080D38;
    'dispatch: loop {
        match pc {
            0x83080D38 => {
    //   block [0x83080D38..0x83080D40)
	// 83080D38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83080D3C: 82169E88  lwz r16, -0x6178(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080D40 size=216
    let mut pc: u32 = 0x83080D40;
    'dispatch: loop {
        match pc {
            0x83080D40 => {
    //   block [0x83080D40..0x83080E18)
	// 83080D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080D44: 48127419  bl 0x831a815c
	ctx.lr = 0x83080D48;
	sub_831A8130(ctx, base);
	// 83080D48: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83080D4C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080D50: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 83080D54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080D58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83080D5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83080D60: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83080D64: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 83080D68: 93BF00E4  stw r29, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[29].u32 ) };
	// 83080D6C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83080D70: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83080D74: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 83080D78: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 83080D7C: 4BFAE735  bl 0x8302f4b0
	ctx.lr = 0x83080D80;
	sub_8302F4B0(ctx, base);
	// 83080D80: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080D84: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 83080D88: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83080D8C: 396B9E20  addi r11, r11, -0x61e0
	ctx.r[11].s64 = ctx.r[11].s64 + -25056;
	// 83080D90: 9B5E0018  stb r26, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[26].u8 ) };
	// 83080D94: 937E001C  stw r27, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 83080D98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080D9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83080DA0: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83080DA4: 419A0068  beq cr6, 0x83080e0c
	if ctx.cr[6].eq {
	pc = 0x83080E0C; continue 'dispatch;
	}
	// 83080DA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83080DAC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83080DB0: 4BF574E9  bl 0x82fd8298
	ctx.lr = 0x83080DB4;
	sub_82FD8298(ctx, base);
	// 83080DB4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83080DB8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83080DBC: 4182002C  beq 0x83080de8
	if ctx.cr[0].eq {
	pc = 0x83080DE8; continue 'dispatch;
	}
	// 83080DC0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83080DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83080DC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080DCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83080DD0: 4BFCBA21  bl 0x8304c7f0
	ctx.lr = 0x83080DD4;
	sub_8304C7F0(ctx, base);
	// 83080DD4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83080DD8: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 83080DDC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83080DE0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83080DE4: 48000008  b 0x83080dec
	pc = 0x83080DEC; continue 'dispatch;
	// 83080DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83080DEC: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83080DF0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83080DF4: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83080DF8: 4BFBA359  bl 0x8303b150
	ctx.lr = 0x83080DFC;
	sub_8303B150(ctx, base);
	// 83080DFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83080E00: 4BDF7749  bl 0x82e78548
	ctx.lr = 0x83080E04;
	sub_82E78548(ctx, base);
	// 83080E04: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83080E08: 4082FFE8  bne 0x83080df0
	if !ctx.cr[0].eq {
	pc = 0x83080DF0; continue 'dispatch;
	}
	// 83080E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83080E10: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83080E14: 48127398  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080E18 size=40
    let mut pc: u32 = 0x83080E18;
    'dispatch: loop {
        match pc {
            0x83080E18 => {
    //   block [0x83080E18..0x83080E40)
	// 83080E18: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83080E1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080E20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080E24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080E28: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83080E2C: 4BFAE6ED  bl 0x8302f518
	ctx.lr = 0x83080E30;
	sub_8302F518(ctx, base);
	// 83080E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080E40 size=44
    let mut pc: u32 = 0x83080E40;
    'dispatch: loop {
        match pc {
            0x83080E40 => {
    //   block [0x83080E40..0x83080E6C)
	// 83080E40: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83080E44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080E48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080E50: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83080E54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83080E58: 4BF57489  bl 0x82fd82e0
	ctx.lr = 0x83080E5C;
	sub_82FD82E0(ctx, base);
	// 83080E5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080E70 size=96
    let mut pc: u32 = 0x83080E70;
    'dispatch: loop {
        match pc {
            0x83080E70 => {
    //   block [0x83080E70..0x83080ED0)
	// 83080E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080E74: 481272ED  bl 0x831a8160
	ctx.lr = 0x83080E78;
	sub_831A8130(ctx, base);
	// 83080E78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080E7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080E80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83080E84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83080E88: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 83080E8C: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 83080E90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080E98: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83080E9C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83080EA0: 4BFAE611  bl 0x8302f4b0
	ctx.lr = 0x83080EA4;
	sub_8302F4B0(ctx, base);
	// 83080EA4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080EA8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83080EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080EB0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83080EB4: 396B9EC8  addi r11, r11, -0x6138
	ctx.r[11].s64 = ctx.r[11].s64 + -24888;
	// 83080EB8: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 83080EBC: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 83080EC0: 935F0024  stw r26, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 83080EC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080EC8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83080ECC: 481272E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080ED0 size=16
    let mut pc: u32 = 0x83080ED0;
    'dispatch: loop {
        match pc {
            0x83080ED0 => {
    //   block [0x83080ED0..0x83080EE0)
	// 83080ED0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83080ED4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83080ED8: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83080EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080EE0 size=36
    let mut pc: u32 = 0x83080EE0;
    'dispatch: loop {
        match pc {
            0x83080EE0 => {
    //   block [0x83080EE0..0x83080F04)
	// 83080EE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080EE4: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83080EE8: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 83080EEC: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 83080EF0: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83080EF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080EF8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83080EFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080F00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080F08 size=68
    let mut pc: u32 = 0x83080F08;
    'dispatch: loop {
        match pc {
            0x83080F08 => {
    //   block [0x83080F08..0x83080F4C)
	// 83080F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080F1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080F20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080F28: 4E800421  bctrl
	ctx.lr = 0x83080F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080F2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83080F30: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83080F34: 4BF8CC15  bl 0x8300db48
	ctx.lr = 0x83080F38;
	sub_8300DB48(ctx, base);
	// 83080F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83080F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080F44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080F50 size=88
    let mut pc: u32 = 0x83080F50;
    'dispatch: loop {
        match pc {
            0x83080F50 => {
    //   block [0x83080F50..0x83080FA8)
	// 83080F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080F64: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080F68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83080F6C: 396B9EC8  addi r11, r11, -0x6138
	ctx.r[11].s64 = ctx.r[11].s64 + -24888;
	// 83080F70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83080F74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080F78: 4BFAE5A1  bl 0x8302f518
	ctx.lr = 0x83080F7C;
	sub_8302F518(ctx, base);
	// 83080F7C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83080F80: 4182000C  beq 0x83080f8c
	if ctx.cr[0].eq {
	pc = 0x83080F8C; continue 'dispatch;
	}
	// 83080F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080F88: 4BF57359  bl 0x82fd82e0
	ctx.lr = 0x83080F8C;
	sub_82FD82E0(ctx, base);
	// 83080F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83080F90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83080F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83080F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83080F9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83080FA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83080FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83080FA8 size=8
    let mut pc: u32 = 0x83080FA8;
    'dispatch: loop {
        match pc {
            0x83080FA8 => {
    //   block [0x83080FA8..0x83080FB0)
	// 83080FA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83080FAC: 82169F08  lwz r16, -0x60f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24824 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83080FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83080FB0 size=172
    let mut pc: u32 = 0x83080FB0;
    'dispatch: loop {
        match pc {
            0x83080FB0 => {
    //   block [0x83080FB0..0x8308105C)
	// 83080FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83080FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83080FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83080FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83080FC0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83080FC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83080FC8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83080FCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83080FD0: 396B9EDC  addi r11, r11, -0x6124
	ctx.r[11].s64 = ctx.r[11].s64 + -24868;
	// 83080FD4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83080FD8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83080FDC: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83080FE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83080FE4: 41820018  beq 0x83080ffc
	if ctx.cr[0].eq {
	pc = 0x83080FFC; continue 'dispatch;
	}
	// 83080FE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080FEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83080FF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83080FF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83080FF8: 4E800421  bctrl
	ctx.lr = 0x83080FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83080FFC: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081000: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081004: 41820018  beq 0x8308101c
	if ctx.cr[0].eq {
	pc = 0x8308101C; continue 'dispatch;
	}
	// 83081008: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308100C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83081010: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081018: 4E800421  bctrl
	ctx.lr = 0x8308101C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308101C: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081020: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081024: 41820018  beq 0x8308103c
	if ctx.cr[0].eq {
	pc = 0x8308103C; continue 'dispatch;
	}
	// 83081028: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308102C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83081030: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081038: 4E800421  bctrl
	ctx.lr = 0x8308103C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308103C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081040: 48020D41  bl 0x830a1d80
	ctx.lr = 0x83081044;
	sub_830A1D80(ctx, base);
	// 83081044: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83081048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308104C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081050: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83081054: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308105C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308105C size=40
    let mut pc: u32 = 0x8308105C;
    'dispatch: loop {
        match pc {
            0x8308105C => {
    //   block [0x8308105C..0x83081084)
	// 8308105C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83081060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308106C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83081070: 48020D11  bl 0x830a1d80
	ctx.lr = 0x83081074;
	sub_830A1D80(ctx, base);
	// 83081074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308107C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081088 size=40
    let mut pc: u32 = 0x83081088;
    'dispatch: loop {
        match pc {
            0x83081088 => {
    //   block [0x83081088..0x830810B0)
	// 83081088: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308108C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081090: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83081094: 4182002C  beq 0x830810c0
	if ctx.cr[0].eq {
		sub_830810C0(ctx, base);
		return;
	}
	// 83081098: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8308109C: 419A001C  beq cr6, 0x830810b8
	if ctx.cr[6].eq {
		sub_830810B8(ctx, base);
		return;
	}
	// 830810A0: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830810A4: 419A000C  beq cr6, 0x830810b0
	if ctx.cr[6].eq {
		sub_830810B0(ctx, base);
		return;
	}
	// 830810A8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 830810AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810B0 size=8
    let mut pc: u32 = 0x830810B0;
    'dispatch: loop {
        match pc {
            0x830810B0 => {
    //   block [0x830810B0..0x830810B8)
	// 830810B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830810B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810B8 size=8
    let mut pc: u32 = 0x830810B8;
    'dispatch: loop {
        match pc {
            0x830810B8 => {
    //   block [0x830810B8..0x830810C0)
	// 830810B8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830810BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810C0 size=8
    let mut pc: u32 = 0x830810C0;
    'dispatch: loop {
        match pc {
            0x830810C0 => {
    //   block [0x830810C0..0x830810C8)
	// 830810C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830810C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810C8 size=12
    let mut pc: u32 = 0x830810C8;
    'dispatch: loop {
        match pc {
            0x830810C8 => {
    //   block [0x830810C8..0x830810D4)
	// 830810C8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830810CC: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830810D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810D8 size=12
    let mut pc: u32 = 0x830810D8;
    'dispatch: loop {
        match pc {
            0x830810D8 => {
    //   block [0x830810D8..0x830810E4)
	// 830810D8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830810DC: 806B002C  lwz r3, 0x2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830810E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830810E8 size=12
    let mut pc: u32 = 0x830810E8;
    'dispatch: loop {
        match pc {
            0x830810E8 => {
    //   block [0x830810E8..0x830810F4)
	// 830810E8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830810EC: 886B0004  lbz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830810F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830810F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830810F8 size=112
    let mut pc: u32 = 0x830810F8;
    'dispatch: loop {
        match pc {
            0x830810F8 => {
    //   block [0x830810F8..0x83081168)
	// 830810F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830810FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081108: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8308110C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83081110: 409A000C  bne cr6, 0x8308111c
	if !ctx.cr[6].eq {
	pc = 0x8308111C; continue 'dispatch;
	}
	// 83081114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83081118: 4800003C  b 0x83081154
	pc = 0x83081154; continue 'dispatch;
	// 8308111C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83081120: 419A0024  beq cr6, 0x83081144
	if ctx.cr[6].eq {
	pc = 0x83081144; continue 'dispatch;
	}
	// 83081124: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83081128: 419A001C  beq cr6, 0x83081144
	if ctx.cr[6].eq {
	pc = 0x83081144; continue 'dispatch;
	}
	// 8308112C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081130: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83081134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081138: 4E800421  bctrl
	ctx.lr = 0x8308113C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308113C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081140: 4082FFE4  bne 0x83081124
	if !ctx.cr[0].eq {
	pc = 0x83081124; continue 'dispatch;
	}
	// 83081144: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 83081148: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8308114C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83081150: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83081154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308115C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081168 size=76
    let mut pc: u32 = 0x83081168;
    'dispatch: loop {
        match pc {
            0x83081168 => {
    //   block [0x83081168..0x830811B4)
	// 83081168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308116C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308117C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83081180: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83081184: 4BFFFE2D  bl 0x83080fb0
	ctx.lr = 0x83081188;
	sub_83080FB0(ctx, base);
	// 83081188: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308118C: 4182000C  beq 0x83081198
	if ctx.cr[0].eq {
	pc = 0x83081198; continue 'dispatch;
	}
	// 83081190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081194: 4BF5714D  bl 0x82fd82e0
	ctx.lr = 0x83081198;
	sub_82FD82E0(ctx, base);
	// 83081198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308119C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830811A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830811A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830811A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830811AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830811B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830811B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830811B8 size=8
    let mut pc: u32 = 0x830811B8;
    'dispatch: loop {
        match pc {
            0x830811B8 => {
    //   block [0x830811B8..0x830811C0)
	// 830811B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830811BC: 82169F50  lwz r16, -0x60b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830811C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830811C0 size=332
    let mut pc: u32 = 0x830811C0;
    'dispatch: loop {
        match pc {
            0x830811C0 => {
    //   block [0x830811C0..0x8308130C)
	// 830811C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830811C4: 48126F8D  bl 0x831a8150
	ctx.lr = 0x830811C8;
	sub_831A8130(ctx, base);
	// 830811C8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830811CC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830811D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830811D4: 82FF010C  lwz r23, 0x10c(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 830811D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830811DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830811E0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 830811E4: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 830811E8: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 830811EC: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830811F0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830811F4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830811F8: 80DF0104  lwz r6, 0x104(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 830811FC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 83081200: 7D565378  mr r22, r10
	ctx.r[22].u64 = ctx.r[10].u64;
	// 83081204: 48020B2D  bl 0x830a1d30
	ctx.lr = 0x83081208;
	sub_830A1D30(ctx, base);
	// 83081208: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308120C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83081210: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83081214: 396B9EDC  addi r11, r11, -0x6124
	ctx.r[11].s64 = ctx.r[11].s64 + -24868;
	// 83081218: 939E0024  stw r28, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 8308121C: 935E0028  stw r26, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[26].u32 ) };
	// 83081220: 937E002C  stw r27, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 83081224: 933E0034  stw r25, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[25].u32 ) };
	// 83081228: 931E0030  stw r24, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 8308122C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83081230: B31E0038  sth r24, 0x38(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[24].u16 ) };
	// 83081234: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83081238: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308123C: 41820028  beq 0x83081264
	if ctx.cr[0].eq {
	pc = 0x83081264; continue 'dispatch;
	}
	// 83081240: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83081244: 4182000C  beq 0x83081250
	if ctx.cr[0].eq {
	pc = 0x83081250; continue 'dispatch;
	}
	// 83081248: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8308124C: B15E0038  sth r10, 0x38(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[10].u16 ) };
	// 83081250: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83081254: 41820010  beq 0x83081264
	if ctx.cr[0].eq {
	pc = 0x83081264; continue 'dispatch;
	}
	// 83081258: A17E0038  lhz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308125C: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 83081260: B17E0038  sth r11, 0x38(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 83081264: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83081268: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308126C: 4182002C  beq 0x83081298
	if ctx.cr[0].eq {
	pc = 0x83081298; continue 'dispatch;
	}
	// 83081270: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83081274: 41820010  beq 0x83081284
	if ctx.cr[0].eq {
	pc = 0x83081284; continue 'dispatch;
	}
	// 83081278: A15E0018  lhz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308127C: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 83081280: B15E0018  sth r10, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 83081284: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83081288: 41820010  beq 0x83081298
	if ctx.cr[0].eq {
	pc = 0x83081298; continue 'dispatch;
	}
	// 8308128C: A17E0018  lhz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83081290: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 83081294: B17E0018  sth r11, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 83081298: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 8308129C: 419A0064  beq cr6, 0x83081300
	if ctx.cr[6].eq {
	pc = 0x83081300; continue 'dispatch;
	}
	// 830812A0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830812A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830812A8: 4BF56FF1  bl 0x82fd8298
	ctx.lr = 0x830812AC;
	sub_82FD8298(ctx, base);
	// 830812AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830812B0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830812B4: 41820028  beq 0x830812dc
	if ctx.cr[0].eq {
	pc = 0x830812DC; continue 'dispatch;
	}
	// 830812B8: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 830812BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830812C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830812C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830812C8: 4BFCB529  bl 0x8304c7f0
	ctx.lr = 0x830812CC;
	sub_8304C7F0(ctx, base);
	// 830812CC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830812D0: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 830812D4: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 830812D8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830812DC: 7EDDB378  mr r29, r22
	ctx.r[29].u64 = ctx.r[22].u64;
	// 830812E0: 931E0030  stw r24, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 830812E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830812E8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830812EC: 4BFB9E65  bl 0x8303b150
	ctx.lr = 0x830812F0;
	sub_8303B150(ctx, base);
	// 830812F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830812F4: 4BDF7255  bl 0x82e78548
	ctx.lr = 0x830812F8;
	sub_82E78548(ctx, base);
	// 830812F8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830812FC: 4082FFE8  bne 0x830812e4
	if !ctx.cr[0].eq {
	pc = 0x830812E4; continue 'dispatch;
	}
	// 83081300: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081304: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83081308: 48126E98  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308130C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308130C size=40
    let mut pc: u32 = 0x8308130C;
    'dispatch: loop {
        match pc {
            0x8308130C => {
    //   block [0x8308130C..0x83081334)
	// 8308130C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83081310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308131C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83081320: 48020A61  bl 0x830a1d80
	ctx.lr = 0x83081324;
	sub_830A1D80(ctx, base);
	// 83081324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308132C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081334 size=44
    let mut pc: u32 = 0x83081334;
    'dispatch: loop {
        match pc {
            0x83081334 => {
    //   block [0x83081334..0x83081360)
	// 83081334: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83081338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308133C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081344: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 83081348: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308134C: 4BF56F95  bl 0x82fd82e0
	ctx.lr = 0x83081350;
	sub_82FD82E0(ctx, base);
	// 83081350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081360 size=88
    let mut pc: u32 = 0x83081360;
    'dispatch: loop {
        match pc {
            0x83081360 => {
    //   block [0x83081360..0x830813B8)
	// 83081360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081364: 48126E01  bl 0x831a8164
	ctx.lr = 0x83081368;
	sub_831A8130(ctx, base);
	// 83081368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308136C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83081370: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83081374: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83081378: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8308137C: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 83081380: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83081384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83081388: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8308138C: 4BFAE125  bl 0x8302f4b0
	ctx.lr = 0x83081390;
	sub_8302F4B0(ctx, base);
	// 83081390: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83081394: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83081398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308139C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830813A0: 396B9F90  addi r11, r11, -0x6070
	ctx.r[11].s64 = ctx.r[11].s64 + -24688;
	// 830813A4: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 830813A8: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 830813AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830813B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830813B4: 48126E00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830813B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830813B8 size=8
    let mut pc: u32 = 0x830813B8;
    'dispatch: loop {
        match pc {
            0x830813B8 => {
    //   block [0x830813B8..0x830813C0)
	// 830813B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830813BC: 82169FB0  lwz r16, -0x6050(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830813C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830813C0 size=108
    let mut pc: u32 = 0x830813C0;
    'dispatch: loop {
        match pc {
            0x830813C0 => {
    //   block [0x830813C0..0x8308142C)
	// 830813C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830813C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830813C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830813CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830813D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830813D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830813D8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830813DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830813E0: 396B9F90  addi r11, r11, -0x6070
	ctx.r[11].s64 = ctx.r[11].s64 + -24688;
	// 830813E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830813E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830813EC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830813F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830813F4: 41820018  beq 0x8308140c
	if ctx.cr[0].eq {
	pc = 0x8308140C; continue 'dispatch;
	}
	// 830813F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830813FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83081400: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081408: 4E800421  bctrl
	ctx.lr = 0x8308140C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308140C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081410: 4BFAE109  bl 0x8302f518
	ctx.lr = 0x83081414;
	sub_8302F518(ctx, base);
	// 83081414: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83081418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308141C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081420: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83081424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308142C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308142C size=40
    let mut pc: u32 = 0x8308142C;
    'dispatch: loop {
        match pc {
            0x8308142C => {
    //   block [0x8308142C..0x83081454)
	// 8308142C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83081430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308143C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83081440: 4BFAE0D9  bl 0x8302f518
	ctx.lr = 0x83081444;
	sub_8302F518(ctx, base);
	// 83081444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308144C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081458 size=32
    let mut pc: u32 = 0x83081458;
    'dispatch: loop {
        match pc {
            0x83081458 => {
    //   block [0x83081458..0x83081478)
	// 83081458: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308145C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83081460: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 83081464: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081468: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308146C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83081470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081474: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081478 size=76
    let mut pc: u32 = 0x83081478;
    'dispatch: loop {
        match pc {
            0x83081478 => {
    //   block [0x83081478..0x830814C4)
	// 83081478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308148C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83081490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83081494: 4BFFFF2D  bl 0x830813c0
	ctx.lr = 0x83081498;
	sub_830813C0(ctx, base);
	// 83081498: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308149C: 4182000C  beq 0x830814a8
	if ctx.cr[0].eq {
	pc = 0x830814A8; continue 'dispatch;
	}
	// 830814A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830814A4: 4BF56E3D  bl 0x82fd82e0
	ctx.lr = 0x830814A8;
	sub_82FD82E0(ctx, base);
	// 830814A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830814AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830814B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830814B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830814B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830814BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830814C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830814C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830814C8 size=80
    let mut pc: u32 = 0x830814C8;
    'dispatch: loop {
        match pc {
            0x830814C8 => {
    //   block [0x830814C8..0x83081518)
	// 830814C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830814CC: 48126C9D  bl 0x831a8168
	ctx.lr = 0x830814D0;
	sub_831A8130(ctx, base);
	// 830814D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830814D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830814D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830814DC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830814E0: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 830814E4: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 830814E8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830814EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830814F0: 4BFADFC1  bl 0x8302f4b0
	ctx.lr = 0x830814F4;
	sub_8302F4B0(ctx, base);
	// 830814F4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830814F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830814FC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83081500: 396B9FE8  addi r11, r11, -0x6018
	ctx.r[11].s64 = ctx.r[11].s64 + -24600;
	// 83081504: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83081508: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 8308150C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83081510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83081514: 48126CA4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081518 size=8
    let mut pc: u32 = 0x83081518;
    'dispatch: loop {
        match pc {
            0x83081518 => {
    //   block [0x83081518..0x83081520)
	// 83081518: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308151C: 8216A008  lwz r16, -0x5ff8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081520 size=108
    let mut pc: u32 = 0x83081520;
    'dispatch: loop {
        match pc {
            0x83081520 => {
    //   block [0x83081520..0x8308158C)
	// 83081520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308152C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081530: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83081534: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081538: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308153C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081540: 396B9FE8  addi r11, r11, -0x6018
	ctx.r[11].s64 = ctx.r[11].s64 + -24600;
	// 83081544: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83081548: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308154C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83081550: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081554: 41820018  beq 0x8308156c
	if ctx.cr[0].eq {
	pc = 0x8308156C; continue 'dispatch;
	}
	// 83081558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308155C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83081560: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081568: 4E800421  bctrl
	ctx.lr = 0x8308156C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308156C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081570: 4BFADFA9  bl 0x8302f518
	ctx.lr = 0x83081574;
	sub_8302F518(ctx, base);
	// 83081574: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83081578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308157C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83081584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308158C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308158C size=40
    let mut pc: u32 = 0x8308158C;
    'dispatch: loop {
        match pc {
            0x8308158C => {
    //   block [0x8308158C..0x830815B4)
	// 8308158C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83081590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308159C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830815A0: 4BFADF79  bl 0x8302f518
	ctx.lr = 0x830815A4;
	sub_8302F518(ctx, base);
	// 830815A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830815A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830815AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830815B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830815B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830815B8 size=76
    let mut pc: u32 = 0x830815B8;
    'dispatch: loop {
        match pc {
            0x830815B8 => {
    //   block [0x830815B8..0x83081604)
	// 830815B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830815BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830815C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830815C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830815C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830815CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830815D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830815D4: 4BFFFF4D  bl 0x83081520
	ctx.lr = 0x830815D8;
	sub_83081520(ctx, base);
	// 830815D8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830815DC: 4182000C  beq 0x830815e8
	if ctx.cr[0].eq {
	pc = 0x830815E8; continue 'dispatch;
	}
	// 830815E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830815E4: 4BF56CFD  bl 0x82fd82e0
	ctx.lr = 0x830815E8;
	sub_82FD82E0(ctx, base);
	// 830815E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830815EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830815F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830815F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830815F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830815FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081608 size=80
    let mut pc: u32 = 0x83081608;
    'dispatch: loop {
        match pc {
            0x83081608 => {
    //   block [0x83081608..0x83081658)
	// 83081608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308160C: 48126B5D  bl 0x831a8168
	ctx.lr = 0x83081610;
	sub_831A8130(ctx, base);
	// 83081610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081614: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83081618: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8308161C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83081620: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 83081624: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 83081628: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8308162C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83081630: 4BFADE81  bl 0x8302f4b0
	ctx.lr = 0x83081634;
	sub_8302F4B0(ctx, base);
	// 83081634: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83081638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308163C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83081640: 396BA040  addi r11, r11, -0x5fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -24512;
	// 83081644: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83081648: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 8308164C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83081650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83081654: 48126B64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081658 size=8
    let mut pc: u32 = 0x83081658;
    'dispatch: loop {
        match pc {
            0x83081658 => {
    //   block [0x83081658..0x83081660)
	// 83081658: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308165C: 8216A060  lwz r16, -0x5fa0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081660 size=108
    let mut pc: u32 = 0x83081660;
    'dispatch: loop {
        match pc {
            0x83081660 => {
    //   block [0x83081660..0x830816CC)
	// 83081660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308166C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081670: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83081674: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081678: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308167C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081680: 396BA040  addi r11, r11, -0x5fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -24512;
	// 83081684: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83081688: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308168C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83081690: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081694: 41820018  beq 0x830816ac
	if ctx.cr[0].eq {
	pc = 0x830816AC; continue 'dispatch;
	}
	// 83081698: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308169C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830816A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830816A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830816A8: 4E800421  bctrl
	ctx.lr = 0x830816AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830816AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830816B0: 4BFADE69  bl 0x8302f518
	ctx.lr = 0x830816B4;
	sub_8302F518(ctx, base);
	// 830816B4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830816B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830816BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830816C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830816C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830816C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830816CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830816CC size=40
    let mut pc: u32 = 0x830816CC;
    'dispatch: loop {
        match pc {
            0x830816CC => {
    //   block [0x830816CC..0x830816F4)
	// 830816CC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830816D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830816D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830816D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830816DC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830816E0: 4BFADE39  bl 0x8302f518
	ctx.lr = 0x830816E4;
	sub_8302F518(ctx, base);
	// 830816E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830816E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830816EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830816F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830816F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830816F8 size=32
    let mut pc: u32 = 0x830816F8;
    'dispatch: loop {
        match pc {
            0x830816F8 => {
    //   block [0x830816F8..0x83081718)
	// 830816F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830816FC: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83081700: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 83081704: 808A000C  lwz r4, 0xc(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83081708: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308170C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83081710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081714: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081718 size=32
    let mut pc: u32 = 0x83081718;
    'dispatch: loop {
        match pc {
            0x83081718 => {
    //   block [0x83081718..0x83081738)
	// 83081718: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308171C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83081720: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 83081724: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83081728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308172C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83081730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83081734: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081738 size=76
    let mut pc: u32 = 0x83081738;
    'dispatch: loop {
        match pc {
            0x83081738 => {
    //   block [0x83081738..0x83081784)
	// 83081738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308173C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308174C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83081750: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83081754: 4BFFFF0D  bl 0x83081660
	ctx.lr = 0x83081758;
	sub_83081660(ctx, base);
	// 83081758: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308175C: 4182000C  beq 0x83081768
	if ctx.cr[0].eq {
	pc = 0x83081768; continue 'dispatch;
	}
	// 83081760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081764: 4BF56B7D  bl 0x82fd82e0
	ctx.lr = 0x83081768;
	sub_82FD82E0(ctx, base);
	// 83081768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308176C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83081770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081778: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308177C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081788 size=8
    let mut pc: u32 = 0x83081788;
    'dispatch: loop {
        match pc {
            0x83081788 => {
    //   block [0x83081788..0x83081790)
	// 83081788: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8308178C: 4BF5CBDC  b 0x82fde368
	sub_82FDE368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081790 size=128
    let mut pc: u32 = 0x83081790;
    'dispatch: loop {
        match pc {
            0x83081790 => {
    //   block [0x83081790..0x83081810)
	// 83081790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308179C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830817A0: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830817A4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830817A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830817AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830817B0: 48018999  bl 0x8309a148
	ctx.lr = 0x830817B4;
	sub_8309A148(ctx, base);
	// 830817B4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830817B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830817BC: 394BA0A8  addi r10, r11, -0x5f58
	ctx.r[10].s64 = ctx.r[11].s64 + -24408;
	// 830817C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830817C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830817C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830817CC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830817D0: 814A9770  lwz r10, -0x6890(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 830817D4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830817D8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830817DC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830817E0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830817E4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 830817E8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830817EC: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 830817F0: 913F0040  stw r9, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 830817F4: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 830817F8: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 830817FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308180C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081810 size=80
    let mut pc: u32 = 0x83081810;
    'dispatch: loop {
        match pc {
            0x83081810 => {
    //   block [0x83081810..0x83081860)
	// 83081810: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83081818: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308181C: 41820010  beq 0x8308182c
	if ctx.cr[0].eq {
	pc = 0x8308182C; continue 'dispatch;
	}
	// 83081820: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83081824: 2F090019  cmpwi cr6, r9, 0x19
	ctx.cr[6].compare_i32(ctx.r[9].s32, 25, &mut ctx.xer);
	// 83081828: 419A001C  beq cr6, 0x83081844
	if ctx.cr[6].eq {
	pc = 0x83081844; continue 'dispatch;
	}
	// 8308182C: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081830: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081834: 41820014  beq 0x83081848
	if ctx.cr[0].eq {
	pc = 0x83081848; continue 'dispatch;
	}
	// 83081838: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308183C: 2F090019  cmpwi cr6, r9, 0x19
	ctx.cr[6].compare_i32(ctx.r[9].s32, 25, &mut ctx.xer);
	// 83081840: 409A0008  bne cr6, 0x83081848
	if !ctx.cr[6].eq {
	pc = 0x83081848; continue 'dispatch;
	}
	// 83081844: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83081848: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308184C: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83081850: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83081854: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83081858: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8308185C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081860 size=8
    let mut pc: u32 = 0x83081860;
    'dispatch: loop {
        match pc {
            0x83081860 => {
    //   block [0x83081860..0x83081868)
	// 83081860: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83081864: 8216A0D8  lwz r16, -0x5f28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081868 size=192
    let mut pc: u32 = 0x83081868;
    'dispatch: loop {
        match pc {
            0x83081868 => {
    //   block [0x83081868..0x83081928)
	// 83081868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308186C: 481268F1  bl 0x831a815c
	ctx.lr = 0x83081870;
	sub_831A8130(ctx, base);
	// 83081870: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83081874: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081878: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 8308187C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081880: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83081884: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83081888: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8308188C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83081890: 939F00E4  stw r28, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[28].u32 ) };
	// 83081894: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 83081898: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8308189C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830818A0: 480188A9  bl 0x8309a148
	ctx.lr = 0x830818A4;
	sub_8309A148(ctx, base);
	// 830818A4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830818A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830818AC: 396BA0A8  addi r11, r11, -0x5f58
	ctx.r[11].s64 = ctx.r[11].s64 + -24408;
	// 830818B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830818B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830818B8: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830818BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830818C0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830818C4: 816B9770  lwz r11, -0x6890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 830818C8: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830818CC: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 830818D0: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 830818D4: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 830818D8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830818DC: 915E003C  stw r10, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 830818E0: 915E0040  stw r10, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 830818E4: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 830818E8: 93BE0048  stw r29, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 830818EC: 4BF569AD  bl 0x82fd8298
	ctx.lr = 0x830818F0;
	sub_82FD8298(ctx, base);
	// 830818F0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830818F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830818F8: 4182001C  beq 0x83081914
	if ctx.cr[0].eq {
	pc = 0x83081914; continue 'dispatch;
	}
	// 830818FC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83081900: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83081904: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83081908: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8308190C: 4BF5D31D  bl 0x82fdec28
	ctx.lr = 0x83081910;
	sub_82FDEC28(ctx, base);
	// 83081910: 48000008  b 0x83081918
	pc = 0x83081918; continue 'dispatch;
	// 83081914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83081918: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8308191C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081920: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83081924: 48126888  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081928 size=40
    let mut pc: u32 = 0x83081928;
    'dispatch: loop {
        match pc {
            0x83081928 => {
    //   block [0x83081928..0x83081950)
	// 83081928: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8308192C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081930: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081938: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8308193C: 48018A15  bl 0x8309a350
	ctx.lr = 0x83081940;
	sub_8309A350(ctx, base);
	// 83081940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081950 size=44
    let mut pc: u32 = 0x83081950;
    'dispatch: loop {
        match pc {
            0x83081950 => {
    //   block [0x83081950..0x8308197C)
	// 83081950: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83081954: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081958: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308195C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081960: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83081964: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081968: 4BF56979  bl 0x82fd82e0
	ctx.lr = 0x8308196C;
	sub_82FD82E0(ctx, base);
	// 8308196C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081980 size=8
    let mut pc: u32 = 0x83081980;
    'dispatch: loop {
        match pc {
            0x83081980 => {
    //   block [0x83081980..0x83081988)
	// 83081980: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83081984: 8216A128  lwz r16, -0x5ed8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081988 size=196
    let mut pc: u32 = 0x83081988;
    'dispatch: loop {
        match pc {
            0x83081988 => {
    //   block [0x83081988..0x83081A4C)
	// 83081988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308198C: 481267D1  bl 0x831a815c
	ctx.lr = 0x83081990;
	sub_831A8130(ctx, base);
	// 83081990: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83081994: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081998: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308199C: 839F00F4  lwz r28, 0xf4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 830819A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830819A4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830819A8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830819AC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 830819B0: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 830819B4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 830819B8: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 830819BC: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 830819C0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 830819C4: 48018A65  bl 0x8309a428
	ctx.lr = 0x830819C8;
	sub_8309A428(ctx, base);
	// 830819C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830819CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830819D0: 396BA0A8  addi r11, r11, -0x5f58
	ctx.r[11].s64 = ctx.r[11].s64 + -24408;
	// 830819D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830819D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830819DC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830819E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830819E4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830819E8: 816B9770  lwz r11, -0x6890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 830819EC: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830819F0: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 830819F4: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 830819F8: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 830819FC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83081A00: 915E003C  stw r10, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 83081A04: 915E0040  stw r10, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83081A08: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83081A0C: 93BE0048  stw r29, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 83081A10: 4BF56889  bl 0x82fd8298
	ctx.lr = 0x83081A14;
	sub_82FD8298(ctx, base);
	// 83081A14: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83081A18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081A1C: 4182001C  beq 0x83081a38
	if ctx.cr[0].eq {
	pc = 0x83081A38; continue 'dispatch;
	}
	// 83081A20: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83081A24: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83081A28: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83081A2C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83081A30: 4BF5D1F9  bl 0x82fdec28
	ctx.lr = 0x83081A34;
	sub_82FDEC28(ctx, base);
	// 83081A34: 48000008  b 0x83081a3c
	pc = 0x83081A3C; continue 'dispatch;
	// 83081A38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83081A3C: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83081A40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081A44: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83081A48: 48126764  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081A4C size=40
    let mut pc: u32 = 0x83081A4C;
    'dispatch: loop {
        match pc {
            0x83081A4C => {
    //   block [0x83081A4C..0x83081A74)
	// 83081A4C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83081A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081A5C: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83081A60: 480188F1  bl 0x8309a350
	ctx.lr = 0x83081A64;
	sub_8309A350(ctx, base);
	// 83081A64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081A74 size=44
    let mut pc: u32 = 0x83081A74;
    'dispatch: loop {
        match pc {
            0x83081A74 => {
    //   block [0x83081A74..0x83081AA0)
	// 83081A74: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83081A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081A84: 809F00F4  lwz r4, 0xf4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 83081A88: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081A8C: 4BF56855  bl 0x82fd82e0
	ctx.lr = 0x83081A90;
	sub_82FD82E0(ctx, base);
	// 83081A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081AA0 size=8
    let mut pc: u32 = 0x83081AA0;
    'dispatch: loop {
        match pc {
            0x83081AA0 => {
    //   block [0x83081AA0..0x83081AA8)
	// 83081AA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83081AA4: 8216A170  lwz r16, -0x5e90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24208 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081AA8 size=96
    let mut pc: u32 = 0x83081AA8;
    'dispatch: loop {
        match pc {
            0x83081AA8 => {
    //   block [0x83081AA8..0x83081B08)
	// 83081AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081AB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83081ABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081AC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081AC4: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 83081AC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83081ACC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83081AD0: 4BF567C9  bl 0x82fd8298
	ctx.lr = 0x83081AD4;
	sub_82FD8298(ctx, base);
	// 83081AD4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83081AD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081ADC: 41820010  beq 0x83081aec
	if ctx.cr[0].eq {
	pc = 0x83081AEC; continue 'dispatch;
	}
	// 83081AE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83081AE4: 4BFFFCAD  bl 0x83081790
	ctx.lr = 0x83081AE8;
	sub_83081790(ctx, base);
	// 83081AE8: 48000008  b 0x83081af0
	pc = 0x83081AF0; continue 'dispatch;
	// 83081AEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83081AF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83081AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081AFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83081B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081B08 size=44
    let mut pc: u32 = 0x83081B08;
    'dispatch: loop {
        match pc {
            0x83081B08 => {
    //   block [0x83081B08..0x83081B34)
	// 83081B08: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83081B0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081B10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081B14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081B18: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83081B1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081B20: 4BF567C1  bl 0x82fd82e0
	ctx.lr = 0x83081B24;
	sub_82FD82E0(ctx, base);
	// 83081B24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081B38 size=12
    let mut pc: u32 = 0x83081B38;
    'dispatch: loop {
        match pc {
            0x83081B38 => {
    //   block [0x83081B38..0x83081B44)
	// 83081B38: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83081B3C: 386B34C0  addi r3, r11, 0x34c0
	ctx.r[3].s64 = ctx.r[11].s64 + 13504;
	// 83081B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081B48 size=344
    let mut pc: u32 = 0x83081B48;
    'dispatch: loop {
        match pc {
            0x83081B48 => {
    //   block [0x83081B48..0x83081CA0)
	// 83081B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081B5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081B60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83081B64: 4801869D  bl 0x8309a200
	ctx.lr = 0x83081B68;
	sub_8309A200(ctx, base);
	// 83081B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081B6C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83081B70: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83081B74: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83081B78: 41820070  beq 0x83081be8
	if ctx.cr[0].eq {
	pc = 0x83081BE8; continue 'dispatch;
	}
	// 83081B7C: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83081B80: 4BF77779  bl 0x82ff92f8
	ctx.lr = 0x83081B84;
	sub_82FF92F8(ctx, base);
	// 83081B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081B88: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83081B8C: 4BF78075  bl 0x82ff9c00
	ctx.lr = 0x83081B90;
	sub_82FF9C00(ctx, base);
	// 83081B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081B94: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081B98: 4BFBF6D1  bl 0x83041268
	ctx.lr = 0x83081B9C;
	sub_83041268(ctx, base);
	// 83081B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BA0: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081BA4: 4BFBF6C5  bl 0x83041268
	ctx.lr = 0x83081BA8;
	sub_83041268(ctx, base);
	// 83081BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BAC: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081BB0: 4BFBF6B9  bl 0x83041268
	ctx.lr = 0x83081BB4;
	sub_83041268(ctx, base);
	// 83081BB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83081BB8: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83081BBC: 4BFCAE8D  bl 0x8304ca48
	ctx.lr = 0x83081BC0;
	sub_8304CA48(ctx, base);
	// 83081BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BC4: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83081BC8: 4BF77731  bl 0x82ff92f8
	ctx.lr = 0x83081BCC;
	sub_82FF92F8(ctx, base);
	// 83081BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BD0: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83081BD4: 4BF77725  bl 0x82ff92f8
	ctx.lr = 0x83081BD8;
	sub_82FF92F8(ctx, base);
	// 83081BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BDC: 809E0048  lwz r4, 0x48(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83081BE0: 4BF78021  bl 0x82ff9c00
	ctx.lr = 0x83081BE4;
	sub_82FF9C00(ctx, base);
	// 83081BE4: 480000A4  b 0x83081c88
	pc = 0x83081C88; continue 'dispatch;
	// 83081BE8: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83081BEC: 4BF7798D  bl 0x82ff9578
	ctx.lr = 0x83081BF0;
	sub_82FF9578(ctx, base);
	// 83081BF0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83081BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081BF8: 388B2E10  addi r4, r11, 0x2e10
	ctx.r[4].s64 = ctx.r[11].s64 + 11792;
	// 83081BFC: 4BF780C5  bl 0x82ff9cc0
	ctx.lr = 0x83081C00;
	sub_82FF9CC0(ctx, base);
	// 83081C00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83081C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C08: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83081C0C: 4BFBF725  bl 0x83041330
	ctx.lr = 0x83081C10;
	sub_83041330(ctx, base);
	// 83081C10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83081C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C18: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83081C1C: 4BFBF715  bl 0x83041330
	ctx.lr = 0x83081C20;
	sub_83041330(ctx, base);
	// 83081C20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83081C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C28: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83081C2C: 4BFBF705  bl 0x83041330
	ctx.lr = 0x83081C30;
	sub_83041330(ctx, base);
	// 83081C30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83081C34: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83081C38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83081C3C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83081C40: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 83081C44: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83081C48: 4BFCB801  bl 0x8304d448
	ctx.lr = 0x83081C4C;
	sub_8304D448(ctx, base);
	// 83081C4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83081C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C54: 4BF77925  bl 0x82ff9578
	ctx.lr = 0x83081C58;
	sub_82FF9578(ctx, base);
	// 83081C58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081C5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83081C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C64: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83081C68: 4BF77911  bl 0x82ff9578
	ctx.lr = 0x83081C6C;
	sub_82FF9578(ctx, base);
	// 83081C6C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081C70: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83081C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83081C78: 388B34C0  addi r4, r11, 0x34c0
	ctx.r[4].s64 = ctx.r[11].s64 + 13504;
	// 83081C7C: 915E0040  stw r10, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83081C80: 4BF78041  bl 0x82ff9cc0
	ctx.lr = 0x83081C84;
	sub_82FF9CC0(ctx, base);
	// 83081C84: 907E0048  stw r3, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 83081C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83081C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081C94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83081C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83081C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081CA0 size=24
    let mut pc: u32 = 0x83081CA0;
    'dispatch: loop {
        match pc {
            0x83081CA0 => {
    //   block [0x83081CA0..0x83081CB8)
	// 83081CA0: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83081CA4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83081CA8: 419A0010  beq cr6, 0x83081cb8
	if ctx.cr[6].eq {
		sub_83081CB8(ctx, base);
		return;
	}
	// 83081CAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83081CB0: 386BD8FC  addi r3, r11, -0x2704
	ctx.r[3].s64 = ctx.r[11].s64 + -9988;
	// 83081CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081CB8 size=20
    let mut pc: u32 = 0x83081CB8;
    'dispatch: loop {
        match pc {
            0x83081CB8 => {
    //   block [0x83081CB8..0x83081CCC)
	// 83081CB8: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081CBC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081CC0: 4182000C  beq 0x83081ccc
	if ctx.cr[0].eq {
		sub_83081CCC(ctx, base);
		return;
	}
	// 83081CC4: 89690008  lbz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081CC8: 4800001C  b 0x83081ce4
	sub_83081CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081CCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081CCC size=20
    let mut pc: u32 = 0x83081CCC;
    'dispatch: loop {
        match pc {
            0x83081CCC => {
    //   block [0x83081CCC..0x83081CE0)
	// 83081CCC: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081CD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081CD4: 4182000C  beq 0x83081ce0
	if ctx.cr[0].eq {
		sub_83081CE0(ctx, base);
		return;
	}
	// 83081CD8: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081CDC: 48000008  b 0x83081ce4
	sub_83081CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081CE0 size=32
    let mut pc: u32 = 0x83081CE0;
    'dispatch: loop {
        match pc {
            0x83081CE0 => {
    //   block [0x83081CE0..0x83081D00)
	// 83081CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83081CE4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83081CE8: 4082005C  bne 0x83081d44
	if !ctx.cr[0].eq {
		sub_83081D2C(ctx, base);
		return;
	}
	// 83081CEC: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081CF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081CF4: 4182000C  beq 0x83081d00
	if ctx.cr[0].eq {
		sub_83081D00(ctx, base);
		return;
	}
	// 83081CF8: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081CFC: 48000008  b 0x83081d04
	sub_83081D00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D00 size=28
    let mut pc: u32 = 0x83081D00;
    'dispatch: loop {
        match pc {
            0x83081D00 => {
    //   block [0x83081D00..0x83081D1C)
	// 83081D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83081D04: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83081D08: 4082003C  bne 0x83081d44
	if !ctx.cr[0].eq {
		sub_83081D2C(ctx, base);
		return;
	}
	// 83081D0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83081D10: 419A000C  beq cr6, 0x83081d1c
	if ctx.cr[6].eq {
		sub_83081D1C(ctx, base);
		return;
	}
	// 83081D14: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D1C size=16
    let mut pc: u32 = 0x83081D1C;
    'dispatch: loop {
        match pc {
            0x83081D1C => {
    //   block [0x83081D1C..0x83081D2C)
	// 83081D1C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83081D20: 419A000C  beq cr6, 0x83081d2c
	if ctx.cr[6].eq {
		sub_83081D2C(ctx, base);
		return;
	}
	// 83081D24: 80690030  lwz r3, 0x30(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D2C size=32
    let mut pc: u32 = 0x83081D2C;
    'dispatch: loop {
        match pc {
            0x83081D2C => {
    //   block [0x83081D2C..0x83081D4C)
	// 83081D2C: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081D30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081D34: 4082FFE0  bne 0x83081d14
	if !ctx.cr[0].eq {
		sub_83081D00(ctx, base);
		return;
	}
	// 83081D38: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83081D3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83081D40: 419AFF6C  beq cr6, 0x83081cac
	if ctx.cr[6].eq {
		sub_83081CA0(ctx, base);
		return;
	}
	// 83081D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83081D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D50 size=24
    let mut pc: u32 = 0x83081D50;
    'dispatch: loop {
        match pc {
            0x83081D50 => {
    //   block [0x83081D50..0x83081D68)
	// 83081D50: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83081D54: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83081D58: 419A0010  beq cr6, 0x83081d68
	if ctx.cr[6].eq {
		sub_83081D68(ctx, base);
		return;
	}
	// 83081D5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83081D60: 386BCC98  addi r3, r11, -0x3368
	ctx.r[3].s64 = ctx.r[11].s64 + -13160;
	// 83081D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D68 size=20
    let mut pc: u32 = 0x83081D68;
    'dispatch: loop {
        match pc {
            0x83081D68 => {
    //   block [0x83081D68..0x83081D7C)
	// 83081D68: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081D6C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081D70: 4182000C  beq 0x83081d7c
	if ctx.cr[0].eq {
		sub_83081D7C(ctx, base);
		return;
	}
	// 83081D74: 89690008  lbz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081D78: 4800001C  b 0x83081d94
	sub_83081D90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D7C size=20
    let mut pc: u32 = 0x83081D7C;
    'dispatch: loop {
        match pc {
            0x83081D7C => {
    //   block [0x83081D7C..0x83081D90)
	// 83081D7C: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081D80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081D84: 4182000C  beq 0x83081d90
	if ctx.cr[0].eq {
		sub_83081D90(ctx, base);
		return;
	}
	// 83081D88: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081D8C: 48000008  b 0x83081d94
	sub_83081D90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081D90 size=32
    let mut pc: u32 = 0x83081D90;
    'dispatch: loop {
        match pc {
            0x83081D90 => {
    //   block [0x83081D90..0x83081DB0)
	// 83081D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83081D94: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83081D98: 4082005C  bne 0x83081df4
	if !ctx.cr[0].eq {
		sub_83081DDC(ctx, base);
		return;
	}
	// 83081D9C: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081DA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081DA4: 4182000C  beq 0x83081db0
	if ctx.cr[0].eq {
		sub_83081DB0(ctx, base);
		return;
	}
	// 83081DA8: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081DAC: 48000008  b 0x83081db4
	sub_83081DB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081DB0 size=28
    let mut pc: u32 = 0x83081DB0;
    'dispatch: loop {
        match pc {
            0x83081DB0 => {
    //   block [0x83081DB0..0x83081DCC)
	// 83081DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83081DB4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83081DB8: 4082003C  bne 0x83081df4
	if !ctx.cr[0].eq {
		sub_83081DDC(ctx, base);
		return;
	}
	// 83081DBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83081DC0: 419A000C  beq cr6, 0x83081dcc
	if ctx.cr[6].eq {
		sub_83081DCC(ctx, base);
		return;
	}
	// 83081DC4: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081DCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081DCC size=16
    let mut pc: u32 = 0x83081DCC;
    'dispatch: loop {
        match pc {
            0x83081DCC => {
    //   block [0x83081DCC..0x83081DDC)
	// 83081DCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83081DD0: 419A000C  beq cr6, 0x83081ddc
	if ctx.cr[6].eq {
		sub_83081DDC(ctx, base);
		return;
	}
	// 83081DD4: 80690034  lwz r3, 0x34(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081DDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081DDC size=32
    let mut pc: u32 = 0x83081DDC;
    'dispatch: loop {
        match pc {
            0x83081DDC => {
    //   block [0x83081DDC..0x83081DFC)
	// 83081DDC: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081DE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081DE4: 4082FFE0  bne 0x83081dc4
	if !ctx.cr[0].eq {
		sub_83081DB0(ctx, base);
		return;
	}
	// 83081DE8: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83081DEC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83081DF0: 419AFF6C  beq cr6, 0x83081d5c
	if ctx.cr[6].eq {
		sub_83081D50(ctx, base);
		return;
	}
	// 83081DF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83081DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081E00 size=8
    let mut pc: u32 = 0x83081E00;
    'dispatch: loop {
        match pc {
            0x83081E00 => {
    //   block [0x83081E00..0x83081E08)
	// 83081E00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83081E04: 8216A1B8  lwz r16, -0x5e48(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081E08 size=292
    let mut pc: u32 = 0x83081E08;
    'dispatch: loop {
        match pc {
            0x83081E08 => {
    //   block [0x83081E08..0x83081F2C)
	// 83081E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081E0C: 48126359  bl 0x831a8164
	ctx.lr = 0x83081E10;
	sub_831A8130(ctx, base);
	// 83081E10: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83081E14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081E18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83081E1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081E20: 811D0020  lwz r8, 0x20(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081E24: 80FD001C  lwz r7, 0x1c(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83081E28: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83081E2C: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081E30: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83081E34: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83081E38: 480185F1  bl 0x8309a428
	ctx.lr = 0x83081E3C;
	sub_8309A428(ctx, base);
	// 83081E3C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83081E40: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83081E44: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081E48: 396BA0A8  addi r11, r11, -0x5f58
	ctx.r[11].s64 = ctx.r[11].s64 + -24408;
	// 83081E4C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83081E50: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83081E54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83081E58: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83081E5C: 816B9770  lwz r11, -0x6890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83081E60: 937E0028  stw r27, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 83081E64: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83081E68: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83081E6C: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83081E70: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83081E74: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83081E78: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 83081E7C: 937E0038  stw r27, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[27].u32 ) };
	// 83081E80: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83081E84: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83081E88: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83081E8C: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 83081E90: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83081E94: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 83081E98: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83081E9C: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83081EA0: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83081EA4: 839D0028  lwz r28, 0x28(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83081EA8: 4BF563F1  bl 0x82fd8298
	ctx.lr = 0x83081EAC;
	sub_82FD8298(ctx, base);
	// 83081EAC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83081EB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081EB4: 4182001C  beq 0x83081ed0
	if ctx.cr[0].eq {
	pc = 0x83081ED0; continue 'dispatch;
	}
	// 83081EB8: 80FE0020  lwz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081EBC: 80DC0020  lwz r6, 0x20(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081EC0: 80BC0010  lwz r5, 0x10(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 83081EC4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83081EC8: 4BF5CD61  bl 0x82fdec28
	ctx.lr = 0x83081ECC;
	sub_82FDEC28(ctx, base);
	// 83081ECC: 48000008  b 0x83081ed4
	pc = 0x83081ED4; continue 'dispatch;
	// 83081ED0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83081ED4: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83081ED8: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83081EDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081EE0: 41820040  beq 0x83081f20
	if ctx.cr[0].eq {
	pc = 0x83081F20; continue 'dispatch;
	}
	// 83081EE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83081EE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081EEC: 41820034  beq 0x83081f20
	if ctx.cr[0].eq {
	pc = 0x83081F20; continue 'dispatch;
	}
	// 83081EF0: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83081EF4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83081EF8: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83081EFC: 4BF5639D  bl 0x82fd8298
	ctx.lr = 0x83081F00;
	sub_82FD8298(ctx, base);
	// 83081F00: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83081F04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081F08: 41820010  beq 0x83081f18
	if ctx.cr[0].eq {
	pc = 0x83081F18; continue 'dispatch;
	}
	// 83081F0C: 809D0038  lwz r4, 0x38(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83081F10: 4802D339  bl 0x830af248
	ctx.lr = 0x83081F14;
	sub_830AF248(ctx, base);
	// 83081F14: 48000008  b 0x83081f1c
	pc = 0x83081F1C; continue 'dispatch;
	// 83081F18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83081F1C: 907E0038  stw r3, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 83081F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83081F24: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83081F28: 4812628C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081F2C size=40
    let mut pc: u32 = 0x83081F2C;
    'dispatch: loop {
        match pc {
            0x83081F2C => {
    //   block [0x83081F2C..0x83081F54)
	// 83081F2C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83081F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081F3C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83081F40: 48018411  bl 0x8309a350
	ctx.lr = 0x83081F44;
	sub_8309A350(ctx, base);
	// 83081F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081F54 size=44
    let mut pc: u32 = 0x83081F54;
    'dispatch: loop {
        match pc {
            0x83081F54 => {
    //   block [0x83081F54..0x83081F80)
	// 83081F54: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83081F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081F64: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081F68: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83081F6C: 4BF56375  bl 0x82fd82e0
	ctx.lr = 0x83081F70;
	sub_82FD82E0(ctx, base);
	// 83081F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081F80 size=44
    let mut pc: u32 = 0x83081F80;
    'dispatch: loop {
        match pc {
            0x83081F80 => {
    //   block [0x83081F80..0x83081FAC)
	// 83081F80: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83081F84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081F88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081F90: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83081F94: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83081F98: 4BF56349  bl 0x82fd82e0
	ctx.lr = 0x83081F9C;
	sub_82FD82E0(ctx, base);
	// 83081F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83081FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83081FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83081FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83081FB0 size=8
    let mut pc: u32 = 0x83081FB0;
    'dispatch: loop {
        match pc {
            0x83081FB0 => {
    //   block [0x83081FB0..0x83081FB8)
	// 83081FB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83081FB4: 8216A210  lwz r16, -0x5df0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-24048 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83081FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83081FB8 size=128
    let mut pc: u32 = 0x83081FB8;
    'dispatch: loop {
        match pc {
            0x83081FB8 => {
    //   block [0x83081FB8..0x83082038)
	// 83081FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83081FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83081FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83081FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83081FC8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83081FCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83081FD0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83081FD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83081FD8: 396BA0A8  addi r11, r11, -0x5f58
	ctx.r[11].s64 = ctx.r[11].s64 + -24408;
	// 83081FDC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83081FE0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83081FE4: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83081FE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83081FEC: 41820018  beq 0x83082004
	if ctx.cr[0].eq {
	pc = 0x83082004; continue 'dispatch;
	}
	// 83081FF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081FF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83081FF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83081FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082000: 4E800421  bctrl
	ctx.lr = 0x83082004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83082004: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83082008: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308200C: 4182000C  beq 0x83082018
	if ctx.cr[0].eq {
	pc = 0x83082018; continue 'dispatch;
	}
	// 83082010: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83082014: 48018705  bl 0x8309a718
	ctx.lr = 0x83082018;
	sub_8309A718(ctx, base);
	// 83082018: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308201C: 48018335  bl 0x8309a350
	ctx.lr = 0x83082020;
	sub_8309A350(ctx, base);
	// 83082020: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83082024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308202C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83082030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83082034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082038 size=40
    let mut pc: u32 = 0x83082038;
    'dispatch: loop {
        match pc {
            0x83082038 => {
    //   block [0x83082038..0x83082060)
	// 83082038: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308203C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082040: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082044: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082048: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8308204C: 48018305  bl 0x8309a350
	ctx.lr = 0x83082050;
	sub_8309A350(ctx, base);
	// 83082050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83082054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082060 size=76
    let mut pc: u32 = 0x83082060;
    'dispatch: loop {
        match pc {
            0x83082060 => {
    //   block [0x83082060..0x830820AC)
	// 83082060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308206C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83082070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082078: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308207C: 4BFFFF3D  bl 0x83081fb8
	ctx.lr = 0x83082080;
	sub_83081FB8(ctx, base);
	// 83082080: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082084: 4182000C  beq 0x83082090
	if ctx.cr[0].eq {
	pc = 0x83082090; continue 'dispatch;
	}
	// 83082088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308208C: 4BF56255  bl 0x82fd82e0
	ctx.lr = 0x83082090;
	sub_82FD82E0(ctx, base);
	// 83082090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83082098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308209C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830820A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830820A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830820A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830820B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830820B0 size=8
    let mut pc: u32 = 0x830820B0;
    'dispatch: loop {
        match pc {
            0x830820B0 => {
    //   block [0x830820B0..0x830820B8)
	// 830820B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830820B4: 8216A2B0  lwz r16, -0x5d50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830820B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830820B8 size=136
    let mut pc: u32 = 0x830820B8;
    'dispatch: loop {
        match pc {
            0x830820B8 => {
    //   block [0x830820B8..0x83082140)
	// 830820B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830820BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830820C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830820C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830820C8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830820CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830820D0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830820D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830820D8: 396BA260  addi r11, r11, -0x5da0
	ctx.r[11].s64 = ctx.r[11].s64 + -23968;
	// 830820DC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830820E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830820E4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830820E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830820EC: 41820018  beq 0x83082104
	if ctx.cr[0].eq {
	pc = 0x83082104; continue 'dispatch;
	}
	// 830820F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830820F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830820F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830820FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082100: 4E800421  bctrl
	ctx.lr = 0x83082104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83082104: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082108: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308210C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082110: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082118: 4E800421  bctrl
	ctx.lr = 0x8308211C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308211C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83082120: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83082124: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83082128: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8308212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83082138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308213C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082140 size=40
    let mut pc: u32 = 0x83082140;
    'dispatch: loop {
        match pc {
            0x83082140 => {
    //   block [0x83082140..0x83082168)
	// 83082140: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83082144: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082148: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308214C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082150: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83082154: 4BFCA60D  bl 0x8304c760
	ctx.lr = 0x83082158;
	sub_8304C760(ctx, base);
	// 83082158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082168 size=56
    let mut pc: u32 = 0x83082168;
    'dispatch: loop {
        match pc {
            0x83082168 => {
    //   block [0x83082168..0x830821A0)
	// 83082168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082174: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83082178: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308217C: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 83082180: 388BA2E0  addi r4, r11, -0x5d20
	ctx.r[4].s64 = ctx.r[11].s64 + -23840;
	// 83082184: 38A0008F  li r5, 0x8f
	ctx.r[5].s64 = 143;
	// 83082188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308218C: 4BF4EECD  bl 0x82fd1058
	ctx.lr = 0x83082190;
	sub_82FD1058(ctx, base);
	// 83082190: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83082194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082198: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8308219C: 4812EA8D  bl 0x831b0c28
	ctx.lr = 0x830821A0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830821A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830821A0 size=56
    let mut pc: u32 = 0x830821A0;
    'dispatch: loop {
        match pc {
            0x830821A0 => {
    //   block [0x830821A0..0x830821D8)
	// 830821A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830821A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830821A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830821AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830821B0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830821B4: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 830821B8: 388BA2E0  addi r4, r11, -0x5d20
	ctx.r[4].s64 = ctx.r[11].s64 + -23840;
	// 830821BC: 38A00099  li r5, 0x99
	ctx.r[5].s64 = 153;
	// 830821C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830821C4: 4BF4EE95  bl 0x82fd1058
	ctx.lr = 0x830821C8;
	sub_82FD1058(ctx, base);
	// 830821C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830821CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830821D0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830821D4: 4812EA55  bl 0x831b0c28
	ctx.lr = 0x830821D8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830821D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830821D8 size=96
    let mut pc: u32 = 0x830821D8;
    'dispatch: loop {
        match pc {
            0x830821D8 => {
    //   block [0x830821D8..0x83082238)
	// 830821D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830821DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830821E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830821E4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830821E8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830821EC: 41980030  blt cr6, 0x8308221c
	if ctx.cr[6].lt {
	pc = 0x8308221C; continue 'dispatch;
	}
	// 830821F0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830821F4: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830821F8: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 830821FC: 388BA2E0  addi r4, r11, -0x5d20
	ctx.r[4].s64 = ctx.r[11].s64 + -23840;
	// 83082200: 38A000B7  li r5, 0xb7
	ctx.r[5].s64 = 183;
	// 83082204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082208: 4BF4E751  bl 0x82fd0958
	ctx.lr = 0x8308220C;
	sub_82FD0958(ctx, base);
	// 8308220C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83082210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082214: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83082218: 4812EA11  bl 0x831b0c28
	ctx.lr = 0x8308221C;
	sub_831B0C28(ctx, base);
	// 8308221C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082220: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83082224: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83082228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308222C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082238 size=96
    let mut pc: u32 = 0x83082238;
    'dispatch: loop {
        match pc {
            0x83082238 => {
    //   block [0x83082238..0x83082298)
	// 83082238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308223C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082244: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082248: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8308224C: 41980030  blt cr6, 0x8308227c
	if ctx.cr[6].lt {
	pc = 0x8308227C; continue 'dispatch;
	}
	// 83082250: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83082254: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082258: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8308225C: 388BA2E0  addi r4, r11, -0x5d20
	ctx.r[4].s64 = ctx.r[11].s64 + -23840;
	// 83082260: 38A000C1  li r5, 0xc1
	ctx.r[5].s64 = 193;
	// 83082264: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082268: 4BF4E6F1  bl 0x82fd0958
	ctx.lr = 0x8308226C;
	sub_82FD0958(ctx, base);
	// 8308226C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83082270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082274: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83082278: 4812E9B1  bl 0x831b0c28
	ctx.lr = 0x8308227C;
	sub_831B0C28(ctx, base);
	// 8308227C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082280: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83082284: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83082288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308228C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83082298 size=12
    let mut pc: u32 = 0x83082298;
    'dispatch: loop {
        match pc {
            0x83082298 => {
    //   block [0x83082298..0x830822A4)
	// 83082298: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8308229C: 386B34C8  addi r3, r11, 0x34c8
	ctx.r[3].s64 = ctx.r[11].s64 + 13512;
	// 830822A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830822A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830822A8 size=76
    let mut pc: u32 = 0x830822A8;
    'dispatch: loop {
        match pc {
            0x830822A8 => {
    //   block [0x830822A8..0x830822F4)
	// 830822A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830822AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830822B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830822B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830822B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830822BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830822C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830822C4: 4BFFFDF5  bl 0x830820b8
	ctx.lr = 0x830822C8;
	sub_830820B8(ctx, base);
	// 830822C8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830822CC: 4182000C  beq 0x830822d8
	if ctx.cr[0].eq {
	pc = 0x830822D8; continue 'dispatch;
	}
	// 830822D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830822D4: 4BF5600D  bl 0x82fd82e0
	ctx.lr = 0x830822D8;
	sub_82FD82E0(ctx, base);
	// 830822D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830822DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830822E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830822E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830822E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830822EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830822F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830822F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830822F8 size=60
    let mut pc: u32 = 0x830822F8;
    'dispatch: loop {
        match pc {
            0x830822F8 => {
    //   block [0x830822F8..0x83082334)
	// 830822F8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830822FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83082300: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83082304: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082308: 41820024  beq 0x8308232c
	if ctx.cr[0].eq {
	pc = 0x8308232C; continue 'dispatch;
	}
	// 8308230C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082310: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082314: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83082318: 409A001C  bne cr6, 0x83082334
	if !ctx.cr[6].eq {
		sub_83082334(ctx, base);
		return;
	}
	// 8308231C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83082320: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83082324: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83082328: 4198FFE8  blt cr6, 0x83082310
	if ctx.cr[6].lt {
	pc = 0x83082310; continue 'dispatch;
	}
	// 8308232C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83082330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83082334 size=8
    let mut pc: u32 = 0x83082334;
    'dispatch: loop {
        match pc {
            0x83082334 => {
    //   block [0x83082334..0x8308233C)
	// 83082334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83082338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082340 size=108
    let mut pc: u32 = 0x83082340;
    'dispatch: loop {
        match pc {
            0x83082340 => {
    //   block [0x83082340..0x830823AC)
	// 83082340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308234C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082354: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83082358: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8308235C: 4BF55F3D  bl 0x82fd8298
	ctx.lr = 0x83082360;
	sub_82FD8298(ctx, base);
	// 83082360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082364: 41820030  beq 0x83082394
	if ctx.cr[0].eq {
	pc = 0x83082394; continue 'dispatch;
	}
	// 83082368: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308236C: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83082370: 394BA260  addi r10, r11, -0x5da0
	ctx.r[10].s64 = ctx.r[11].s64 + -23968;
	// 83082374: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83082378: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8308237C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83082380: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83082384: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83082388: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8308238C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83082390: 48000008  b 0x83082398
	pc = 0x83082398; continue 'dispatch;
	// 83082394: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83082398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830823A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830823A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830823A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830823B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830823B0 size=64
    let mut pc: u32 = 0x830823B0;
    'dispatch: loop {
        match pc {
            0x830823B0 => {
    //   block [0x830823B0..0x830823F0)
	// 830823B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830823B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830823B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830823BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830823C0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830823C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830823C8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830823CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 830823D0: 4BFE6831  bl 0x83068c00
	ctx.lr = 0x830823D4;
	sub_83068C00(ctx, base);
	// 830823D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830823D8: 41820008  beq 0x830823e0
	if ctx.cr[0].eq {
	pc = 0x830823E0; continue 'dispatch;
	}
	// 830823DC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830823E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830823E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830823E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830823EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830823F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830823F0 size=8
    let mut pc: u32 = 0x830823F0;
    'dispatch: loop {
        match pc {
            0x830823F0 => {
    //   block [0x830823F0..0x830823F8)
	// 830823F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830823F4: 8216A338  lwz r16, -0x5cc8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830823F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830823F8 size=168
    let mut pc: u32 = 0x830823F8;
    'dispatch: loop {
        match pc {
            0x830823F8 => {
    //   block [0x830823F8..0x830824A0)
	// 830823F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830823FC: 48125D6D  bl 0x831a8168
	ctx.lr = 0x83082400;
	sub_831A8130(ctx, base);
	// 83082400: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83082404: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082408: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308240C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83082410: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83082414: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83082418: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308241C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83082420: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 83082424: 396BA260  addi r11, r11, -0x5da0
	ctx.r[11].s64 = ctx.r[11].s64 + -23968;
	// 83082428: 90BF0050  stw r5, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8308242C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83082430: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83082434: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83082438: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308243C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83082440: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83082444: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83082448: 4BF55E51  bl 0x82fd8298
	ctx.lr = 0x8308244C;
	sub_82FD8298(ctx, base);
	// 8308244C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83082450: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082454: 41820018  beq 0x8308246c
	if ctx.cr[0].eq {
	pc = 0x8308246C; continue 'dispatch;
	}
	// 83082458: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8308245C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082460: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83082464: 4BFBD965  bl 0x8303fdc8
	ctx.lr = 0x83082468;
	sub_8303FDC8(ctx, base);
	// 83082468: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8308246C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082470: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83082474: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83082478: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308247C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082484: 4E800421  bctrl
	ctx.lr = 0x83082488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83082488: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8308248C: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83082490: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83082494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83082498: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8308249C: 48125D1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830824A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830824A0 size=40
    let mut pc: u32 = 0x830824A0;
    'dispatch: loop {
        match pc {
            0x830824A0 => {
    //   block [0x830824A0..0x830824C8)
	// 830824A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830824A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830824A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830824AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830824B0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830824B4: 4BFCA2AD  bl 0x8304c760
	ctx.lr = 0x830824B8;
	sub_8304C760(ctx, base);
	// 830824B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830824BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830824C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830824C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830824C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830824C8 size=44
    let mut pc: u32 = 0x830824C8;
    'dispatch: loop {
        match pc {
            0x830824C8 => {
    //   block [0x830824C8..0x830824F4)
	// 830824C8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830824CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830824D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830824D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830824D8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830824DC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830824E0: 4BF55E01  bl 0x82fd82e0
	ctx.lr = 0x830824E4;
	sub_82FD82E0(ctx, base);
	// 830824E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830824E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830824EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830824F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830824F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830824F8 size=100
    let mut pc: u32 = 0x830824F8;
    'dispatch: loop {
        match pc {
            0x830824F8 => {
    //   block [0x830824F8..0x8308255C)
	// 830824F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830824FC: 48125C71  bl 0x831a816c
	ctx.lr = 0x83082500;
	sub_831A8130(ctx, base);
	// 83082500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082504: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83082508: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308250C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83082510: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 83082514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082518: 4BF4F899  bl 0x82fd1db0
	ctx.lr = 0x8308251C;
	sub_82FD1DB0(ctx, base);
	// 8308251C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83082520: 41800014  blt 0x83082534
	if ctx.cr[0].lt {
	pc = 0x83082534; continue 'dispatch;
	}
	// 83082524: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 83082528: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308252C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83082530: 48000008  b 0x83082538
	pc = 0x83082538; continue 'dispatch;
	// 83082534: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83082538: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8308253C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83082540: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83082544: 4BFE66BD  bl 0x83068c00
	ctx.lr = 0x83082548;
	sub_83068C00(ctx, base);
	// 83082548: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308254C: 41820008  beq 0x83082554
	if ctx.cr[0].eq {
	pc = 0x83082554; continue 'dispatch;
	}
	// 83082550: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83082558: 48125C64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83082560 size=8
    let mut pc: u32 = 0x83082560;
    'dispatch: loop {
        match pc {
            0x83082560 => {
    //   block [0x83082560..0x83082568)
	// 83082560: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83082564: 8216A380  lwz r16, -0x5c80(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082568 size=372
    let mut pc: u32 = 0x83082568;
    'dispatch: loop {
        match pc {
            0x83082568 => {
    //   block [0x83082568..0x830826DC)
	// 83082568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308256C: 48125BF9  bl 0x831a8164
	ctx.lr = 0x83082570;
	sub_831A8130(ctx, base);
	// 83082570: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83082574: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082578: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308257C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83082580: 48045561  bl 0x830c7ae0
	ctx.lr = 0x83082584;
	sub_830C7AE0(ctx, base);
	// 83082584: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83082588: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8308258C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082590: 41820020  beq 0x830825b0
	if ctx.cr[0].eq {
	pc = 0x830825B0; continue 'dispatch;
	}
	// 83082594: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83082598: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308259C: 4BFCDF0D  bl 0x830504a8
	ctx.lr = 0x830825A0;
	sub_830504A8(ctx, base);
	// 830825A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830825A4: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830825A8: 4BF76D51  bl 0x82ff92f8
	ctx.lr = 0x830825AC;
	sub_82FF92F8(ctx, base);
	// 830825AC: 48000128  b 0x830826d4
	pc = 0x830826D4; continue 'dispatch;
	// 830825B0: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 830825B4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830825B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830825BC: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 830825C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830825C4: 4BFCC855  bl 0x8304ee18
	ctx.lr = 0x830825C8;
	sub_8304EE18(ctx, base);
	// 830825C8: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 830825CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830825D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830825D4: 4BF76FA5  bl 0x82ff9578
	ctx.lr = 0x830825D8;
	sub_82FF9578(ctx, base);
	// 830825D8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830825DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830825E0: 409A0048  bne cr6, 0x83082628
	if !ctx.cr[6].eq {
	pc = 0x83082628; continue 'dispatch;
	}
	// 830825E4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830825E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830825EC: 419A003C  beq cr6, 0x83082628
	if ctx.cr[6].eq {
	pc = 0x83082628; continue 'dispatch;
	}
	// 830825F0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830825F4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830825F8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830825FC: 4BF55C9D  bl 0x82fd8298
	ctx.lr = 0x83082600;
	sub_82FD8298(ctx, base);
	// 83082600: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83082604: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082608: 41820018  beq 0x83082620
	if ctx.cr[0].eq {
	pc = 0x83082620; continue 'dispatch;
	}
	// 8308260C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83082610: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082614: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082618: 4BFBD7B1  bl 0x8303fdc8
	ctx.lr = 0x8308261C;
	sub_8303FDC8(ctx, base);
	// 8308261C: 48000008  b 0x83082624
	pc = 0x83082624; continue 'dispatch;
	// 83082620: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83082624: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83082628: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308262C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83082630: 419A00A4  beq cr6, 0x830826d4
	if ctx.cr[6].eq {
	pc = 0x830826D4; continue 'dispatch;
	}
	// 83082634: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082638: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308263C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082640: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082648: 4E800421  bctrl
	ctx.lr = 0x8308264C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308264C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082650: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082654: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83082658: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308265C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082664: 4E800421  bctrl
	ctx.lr = 0x83082668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83082668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308266C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082670: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83082674: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83082678: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8308267C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082680: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082684: 48000040  b 0x830826c4
	pc = 0x830826C4; continue 'dispatch;
	// 83082688: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308268C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082690: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83082698: 4E800421  bctrl
	ctx.lr = 0x8308269C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308269C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830826A0: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830826A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830826A8: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 830826AC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830826B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830826B4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830826B8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830826BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830826C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830826C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830826C8: 4E800421  bctrl
	ctx.lr = 0x830826CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830826CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830826D0: 4082FFB8  bne 0x83082688
	if !ctx.cr[0].eq {
	pc = 0x83082688; continue 'dispatch;
	}
	// 830826D4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830826D8: 48125ADC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830826DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830826DC size=44
    let mut pc: u32 = 0x830826DC;
    'dispatch: loop {
        match pc {
            0x830826DC => {
    //   block [0x830826DC..0x83082708)
	// 830826DC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830826E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830826E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830826E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830826EC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830826F0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830826F4: 4BF55BED  bl 0x82fd82e0
	ctx.lr = 0x830826F8;
	sub_82FD82E0(ctx, base);
	// 830826F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830826FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83082708 size=8
    let mut pc: u32 = 0x83082708;
    'dispatch: loop {
        match pc {
            0x83082708 => {
    //   block [0x83082708..0x83082710)
	// 83082708: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308270C: 8216A400  lwz r16, -0x5c00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082710 size=288
    let mut pc: u32 = 0x83082710;
    'dispatch: loop {
        match pc {
            0x83082710 => {
    //   block [0x83082710..0x83082830)
	// 83082710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082714: 48125A51  bl 0x831a8164
	ctx.lr = 0x83082718;
	sub_831A8130(ctx, base);
	// 83082718: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8308271C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082720: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83082724: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83082728: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8308272C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83082730: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83082734: 396BD9C8  addi r11, r11, -0x2638
	ctx.r[11].s64 = ctx.r[11].s64 + -9784;
	// 83082738: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8308273C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83082740: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082744: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83082748: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8308274C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83082750: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83082754: 937E0014  stw r27, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 83082758: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8308275C: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082760: 995E001C  stb r10, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 83082764: 995E001D  stb r10, 0x1d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 83082768: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8308276C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082770: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83082774: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83082778: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8308277C: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082780: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082784: 41820030  beq 0x830827b4
	if ctx.cr[0].eq {
	pc = 0x830827B4; continue 'dispatch;
	}
	// 83082788: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8308278C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082790: 4BF55B09  bl 0x82fd8298
	ctx.lr = 0x83082794;
	sub_82FD8298(ctx, base);
	// 83082794: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83082798: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308279C: 41820010  beq 0x830827ac
	if ctx.cr[0].eq {
	pc = 0x830827AC; continue 'dispatch;
	}
	// 830827A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830827A4: 4BF5BA4D  bl 0x82fde1f0
	ctx.lr = 0x830827A8;
	sub_82FDE1F0(ctx, base);
	// 830827A8: 48000008  b 0x830827b0
	pc = 0x830827B0; continue 'dispatch;
	// 830827AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830827B0: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830827B4: 839D0010  lwz r28, 0x10(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830827B8: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830827BC: 41820030  beq 0x830827ec
	if ctx.cr[0].eq {
	pc = 0x830827EC; continue 'dispatch;
	}
	// 830827C0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830827C4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830827C8: 4BF55AD1  bl 0x82fd8298
	ctx.lr = 0x830827CC;
	sub_82FD8298(ctx, base);
	// 830827CC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830827D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830827D4: 41820010  beq 0x830827e4
	if ctx.cr[0].eq {
	pc = 0x830827E4; continue 'dispatch;
	}
	// 830827D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830827DC: 4BFFFF35  bl 0x83082710
	ctx.lr = 0x830827E0;
	sub_83082710(ctx, base);
	// 830827E0: 48000008  b 0x830827e8
	pc = 0x830827E8; continue 'dispatch;
	// 830827E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830827E8: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830827EC: 83BD0014  lwz r29, 0x14(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830827F0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830827F4: 41820030  beq 0x83082824
	if ctx.cr[0].eq {
	pc = 0x83082824; continue 'dispatch;
	}
	// 830827F8: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830827FC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082800: 4BF55A99  bl 0x82fd8298
	ctx.lr = 0x83082804;
	sub_82FD8298(ctx, base);
	// 83082804: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83082808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308280C: 41820010  beq 0x8308281c
	if ctx.cr[0].eq {
	pc = 0x8308281C; continue 'dispatch;
	}
	// 83082810: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83082814: 4BFFFEFD  bl 0x83082710
	ctx.lr = 0x83082818;
	sub_83082710(ctx, base);
	// 83082818: 48000008  b 0x83082820
	pc = 0x83082820; continue 'dispatch;
	// 8308281C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83082820: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83082824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83082828: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8308282C: 48125988  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082830 size=40
    let mut pc: u32 = 0x83082830;
    'dispatch: loop {
        match pc {
            0x83082830 => {
    //   block [0x83082830..0x83082858)
	// 83082830: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83082834: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082838: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308283C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082840: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83082844: 4BFC9F1D  bl 0x8304c760
	ctx.lr = 0x83082848;
	sub_8304C760(ctx, base);
	// 83082848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308284C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082858 size=48
    let mut pc: u32 = 0x83082858;
    'dispatch: loop {
        match pc {
            0x83082858 => {
    //   block [0x83082858..0x83082888)
	// 83082858: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8308285C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082860: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082868: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8308286C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082870: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83082874: 4BF55A6D  bl 0x82fd82e0
	ctx.lr = 0x83082878;
	sub_82FD82E0(ctx, base);
	// 83082878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308287C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082888 size=48
    let mut pc: u32 = 0x83082888;
    'dispatch: loop {
        match pc {
            0x83082888 => {
    //   block [0x83082888..0x830828B8)
	// 83082888: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8308288C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082890: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082898: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8308289C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830828A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830828A4: 4BF55A3D  bl 0x82fd82e0
	ctx.lr = 0x830828A8;
	sub_82FD82E0(ctx, base);
	// 830828A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830828AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830828B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830828B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830828B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830828B8 size=48
    let mut pc: u32 = 0x830828B8;
    'dispatch: loop {
        match pc {
            0x830828B8 => {
    //   block [0x830828B8..0x830828E8)
	// 830828B8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830828BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830828C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830828C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830828C8: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830828CC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830828D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830828D4: 4BF55A0D  bl 0x82fd82e0
	ctx.lr = 0x830828D8;
	sub_82FD82E0(ctx, base);
	// 830828D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830828DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830828E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830828E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830828E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830828E8 size=576
    let mut pc: u32 = 0x830828E8;
    'dispatch: loop {
        match pc {
            0x830828E8 => {
    //   block [0x830828E8..0x83082B28)
	// 830828E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830828EC: 48125879  bl 0x831a8164
	ctx.lr = 0x830828F0;
	sub_831A8130(ctx, base);
	// 830828F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830828F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830828F8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830828FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83082900: 419A0220  beq cr6, 0x83082b20
	if ctx.cr[6].eq {
	pc = 0x83082B20; continue 'dispatch;
	}
	// 83082904: 83830010  lwz r28, 0x10(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082908: 83630014  lwz r27, 0x14(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308290C: 83C30018  lwz r30, 0x18(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082910: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082914: 4182000C  beq 0x83082920
	if ctx.cr[0].eq {
	pc = 0x83082920; continue 'dispatch;
	}
	// 83082918: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308291C: 48000008  b 0x83082924
	pc = 0x83082924; continue 'dispatch;
	// 83082920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83082924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83082928: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308292C: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83082930: 4182000C  beq 0x8308293c
	if ctx.cr[0].eq {
	pc = 0x8308293C; continue 'dispatch;
	}
	// 83082934: 409A000C  bne cr6, 0x83082940
	if !ctx.cr[6].eq {
	pc = 0x83082940; continue 'dispatch;
	}
	// 83082938: 4800000C  b 0x83082944
	pc = 0x83082944; continue 'dispatch;
	// 8308293C: 409A0008  bne cr6, 0x83082944
	if !ctx.cr[6].eq {
	pc = 0x83082944; continue 'dispatch;
	}
	// 83082940: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83082944: 57CB073E  clrlwi r11, r30, 0x1c
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	// 83082948: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8308294C: 4198019C  blt cr6, 0x83082ae8
	if ctx.cr[6].lt {
	pc = 0x83082AE8; continue 'dispatch;
	}
	// 83082950: 419A0150  beq cr6, 0x83082aa0
	if ctx.cr[6].eq {
	pc = 0x83082AA0; continue 'dispatch;
	}
	// 83082954: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83082958: 41980108  blt cr6, 0x83082a60
	if ctx.cr[6].lt {
	pc = 0x83082A60; continue 'dispatch;
	}
	// 8308295C: 419A00C4  beq cr6, 0x83082a20
	if ctx.cr[6].eq {
	pc = 0x83082A20; continue 'dispatch;
	}
	// 83082960: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 83082964: 4198006C  blt cr6, 0x830829d0
	if ctx.cr[6].lt {
	pc = 0x830829D0; continue 'dispatch;
	}
	// 83082968: 419A003C  beq cr6, 0x830829a4
	if ctx.cr[6].eq {
	pc = 0x830829A4; continue 'dispatch;
	}
	// 8308296C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 83082970: 409A01B0  bne cr6, 0x83082b20
	if !ctx.cr[6].eq {
	pc = 0x83082B20; continue 'dispatch;
	}
	// 83082974: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83082978: 419A0040  beq cr6, 0x830829b8
	if ctx.cr[6].eq {
	pc = 0x830829B8; continue 'dispatch;
	}
	// 8308297C: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 83082980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082984: 4BF4E195  bl 0x82fd0b18
	ctx.lr = 0x83082988;
	sub_82FD0B18(ctx, base);
	// 83082988: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 8308298C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082990: 4BF4E189  bl 0x82fd0b18
	ctx.lr = 0x83082994;
	sub_82FD0B18(ctx, base);
	// 83082994: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 83082998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308299C: 4BF4E17D  bl 0x82fd0b18
	ctx.lr = 0x830829A0;
	sub_82FD0B18(ctx, base);
	// 830829A0: 4800000C  b 0x830829ac
	pc = 0x830829AC; continue 'dispatch;
	// 830829A4: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830829A8: 419A0010  beq cr6, 0x830829b8
	if ctx.cr[6].eq {
	pc = 0x830829B8; continue 'dispatch;
	}
	// 830829AC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 830829B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830829B4: 4BF4E165  bl 0x82fd0b18
	ctx.lr = 0x830829B8;
	sub_82FD0B18(ctx, base);
	// 830829B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830829BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830829C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830829C4: 4BFFFF25  bl 0x830828e8
	ctx.lr = 0x830829C8;
	sub_830828E8(ctx, base);
	// 830829C8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 830829CC: 4800002C  b 0x830829f8
	pc = 0x830829F8; continue 'dispatch;
	// 830829D0: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830829D4: 419A0010  beq cr6, 0x830829e4
	if ctx.cr[6].eq {
	pc = 0x830829E4; continue 'dispatch;
	}
	// 830829D8: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 830829DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830829E0: 4BF4E139  bl 0x82fd0b18
	ctx.lr = 0x830829E4;
	sub_82FD0B18(ctx, base);
	// 830829E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830829E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830829EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830829F0: 4BFFFEF9  bl 0x830828e8
	ctx.lr = 0x830829F4;
	sub_830828E8(ctx, base);
	// 830829F4: 3880007C  li r4, 0x7c
	ctx.r[4].s64 = 124;
	// 830829F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830829FC: 4BF4E11D  bl 0x82fd0b18
	ctx.lr = 0x83082A00;
	sub_82FD0B18(ctx, base);
	// 83082A00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83082A04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83082A08: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83082A0C: 4BFFFEDD  bl 0x830828e8
	ctx.lr = 0x83082A10;
	sub_830828E8(ctx, base);
	// 83082A10: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83082A14: 419A010C  beq cr6, 0x83082b20
	if ctx.cr[6].eq {
	pc = 0x83082B20; continue 'dispatch;
	}
	// 83082A18: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 83082A1C: 480000C0  b 0x83082adc
	pc = 0x83082ADC; continue 'dispatch;
	// 83082A20: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83082A24: 41820010  beq 0x83082a34
	if ctx.cr[0].eq {
	pc = 0x83082A34; continue 'dispatch;
	}
	// 83082A28: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83082A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082A30: 4BF4E0E9  bl 0x82fd0b18
	ctx.lr = 0x83082A34;
	sub_82FD0B18(ctx, base);
	// 83082A34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83082A38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83082A3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83082A40: 4BFFFEA9  bl 0x830828e8
	ctx.lr = 0x83082A44;
	sub_830828E8(ctx, base);
	// 83082A44: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83082A48: 419A0010  beq cr6, 0x83082a58
	if ctx.cr[6].eq {
	pc = 0x83082A58; continue 'dispatch;
	}
	// 83082A4C: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 83082A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082A54: 4BF4E0C5  bl 0x82fd0b18
	ctx.lr = 0x83082A58;
	sub_82FD0B18(ctx, base);
	// 83082A58: 3880002B  li r4, 0x2b
	ctx.r[4].s64 = 43;
	// 83082A5C: 48000080  b 0x83082adc
	pc = 0x83082ADC; continue 'dispatch;
	// 83082A60: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83082A64: 41820010  beq 0x83082a74
	if ctx.cr[0].eq {
	pc = 0x83082A74; continue 'dispatch;
	}
	// 83082A68: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83082A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082A70: 4BF4E0A9  bl 0x82fd0b18
	ctx.lr = 0x83082A74;
	sub_82FD0B18(ctx, base);
	// 83082A74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83082A78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83082A7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83082A80: 4BFFFE69  bl 0x830828e8
	ctx.lr = 0x83082A84;
	sub_830828E8(ctx, base);
	// 83082A84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83082A88: 419A0010  beq cr6, 0x83082a98
	if ctx.cr[6].eq {
	pc = 0x83082A98; continue 'dispatch;
	}
	// 83082A8C: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 83082A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082A94: 4BF4E085  bl 0x82fd0b18
	ctx.lr = 0x83082A98;
	sub_82FD0B18(ctx, base);
	// 83082A98: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 83082A9C: 48000040  b 0x83082adc
	pc = 0x83082ADC; continue 'dispatch;
	// 83082AA0: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83082AA4: 41820010  beq 0x83082ab4
	if ctx.cr[0].eq {
	pc = 0x83082AB4; continue 'dispatch;
	}
	// 83082AA8: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83082AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082AB0: 4BF4E069  bl 0x82fd0b18
	ctx.lr = 0x83082AB4;
	sub_82FD0B18(ctx, base);
	// 83082AB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83082AB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83082ABC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83082AC0: 4BFFFE29  bl 0x830828e8
	ctx.lr = 0x83082AC4;
	sub_830828E8(ctx, base);
	// 83082AC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83082AC8: 419A0010  beq cr6, 0x83082ad8
	if ctx.cr[6].eq {
	pc = 0x83082AD8; continue 'dispatch;
	}
	// 83082ACC: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 83082AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082AD4: 4BF4E045  bl 0x82fd0b18
	ctx.lr = 0x83082AD8;
	sub_82FD0B18(ctx, base);
	// 83082AD8: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 83082ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082AE0: 4BF4E039  bl 0x82fd0b18
	ctx.lr = 0x83082AE4;
	sub_82FD0B18(ctx, base);
	// 83082AE4: 4800003C  b 0x83082b20
	pc = 0x83082B20; continue 'dispatch;
	// 83082AE8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082AEC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83082AF0: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082AF4: 816B9774  lwz r11, -0x688c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 83082AF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83082AFC: 409A0010  bne cr6, 0x83082b0c
	if !ctx.cr[6].eq {
	pc = 0x83082B0C; continue 'dispatch;
	}
	// 83082B00: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83082B04: 388B9778  addi r4, r11, -0x6888
	ctx.r[4].s64 = ctx.r[11].s64 + -26760;
	// 83082B08: 4800000C  b 0x83082b14
	pc = 0x83082B14; continue 'dispatch;
	// 83082B0C: 4BF5B85D  bl 0x82fde368
	ctx.lr = 0x83082B10;
	sub_82FDE368(ctx, base);
	// 83082B10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83082B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83082B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082B1C: 4BF56A55  bl 0x82fd9570
	ctx.lr = 0x83082B20;
	sub_82FD9570(ctx, base);
	// 83082B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83082B24: 48125690  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082B28 size=132
    let mut pc: u32 = 0x83082B28;
    'dispatch: loop {
        match pc {
            0x83082B28 => {
    //   block [0x83082B28..0x83082BAC)
	// 83082B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082B30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83082B34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83082B38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082B3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83082B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83082B44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83082B48: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082B4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83082B50: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83082B54: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082B58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082B5C: 409A0010  bne cr6, 0x83082b6c
	if !ctx.cr[6].eq {
	pc = 0x83082B6C; continue 'dispatch;
	}
	// 83082B60: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83082B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082B68: 4BF4DFB1  bl 0x82fd0b18
	ctx.lr = 0x83082B6C;
	sub_82FD0B18(ctx, base);
	// 83082B6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83082B70: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83082B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83082B78: 4BFFFD71  bl 0x830828e8
	ctx.lr = 0x83082B7C;
	sub_830828E8(ctx, base);
	// 83082B7C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082B80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082B84: 409A0010  bne cr6, 0x83082b94
	if !ctx.cr[6].eq {
	pc = 0x83082B94; continue 'dispatch;
	}
	// 83082B88: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 83082B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082B90: 4BF4DF89  bl 0x82fd0b18
	ctx.lr = 0x83082B94;
	sub_82FD0B18(ctx, base);
	// 83082B94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83082B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082BA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83082BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83082BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082BB0 size=148
    let mut pc: u32 = 0x83082BB0;
    'dispatch: loop {
        match pc {
            0x83082BB0 => {
    //   block [0x83082BB0..0x83082C44)
	// 83082BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082BB4: 481255B9  bl 0x831a816c
	ctx.lr = 0x83082BB8;
	sub_831A8130(ctx, base);
	// 83082BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082BC0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082BC4: 83BF0020  lwz r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082BC8: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83082BCC: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 83082BD0: 419A0014  beq cr6, 0x83082be4
	if ctx.cr[6].eq {
	pc = 0x83082BE4; continue 'dispatch;
	}
	// 83082BD4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83082BD8: 419A000C  beq cr6, 0x83082be4
	if ctx.cr[6].eq {
	pc = 0x83082BE4; continue 'dispatch;
	}
	// 83082BDC: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83082BE0: 409A0058  bne cr6, 0x83082c38
	if !ctx.cr[6].eq {
	pc = 0x83082C38; continue 'dispatch;
	}
	// 83082BE4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082BE8: 4BFFFFC9  bl 0x83082bb0
	ctx.lr = 0x83082BEC;
	sub_83082BB0(ctx, base);
	// 83082BEC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83082BF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83082BF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082BF8: 4182003C  beq 0x83082c34
	if ctx.cr[0].eq {
	pc = 0x83082C34; continue 'dispatch;
	}
	// 83082BFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83082C00: 4BFFFFB1  bl 0x83082bb0
	ctx.lr = 0x83082C04;
	sub_83082BB0(ctx, base);
	// 83082C04: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082C08: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83082C0C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83082C10: 409A0018  bne cr6, 0x83082c28
	if !ctx.cr[6].eq {
	pc = 0x83082C28; continue 'dispatch;
	}
	// 83082C14: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83082C18: 40980008  bge cr6, 0x83082c20
	if !ctx.cr[6].lt {
	pc = 0x83082C20; continue 'dispatch;
	}
	// 83082C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83082C20: 7FA3E9D6  mullw r29, r3, r29
	ctx.r[29].s64 = (ctx.r[3].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83082C24: 48000014  b 0x83082c38
	pc = 0x83082C38; continue 'dispatch;
	// 83082C28: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83082C2C: 7FABE9D6  mullw r29, r11, r29
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83082C30: 48000008  b 0x83082c38
	pc = 0x83082C38; continue 'dispatch;
	// 83082C34: 7FBEE9D6  mullw r29, r30, r29
	ctx.r[29].s64 = (ctx.r[30].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83082C38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83082C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83082C40: 4812557C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082C48 size=196
    let mut pc: u32 = 0x83082C48;
    'dispatch: loop {
        match pc {
            0x83082C48 => {
    //   block [0x83082C48..0x83082D0C)
	// 83082C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082C4C: 48125521  bl 0x831a816c
	ctx.lr = 0x83082C50;
	sub_831A8130(ctx, base);
	// 83082C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082C58: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83082C5C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 83082C60: 409A000C  bne cr6, 0x83082c6c
	if !ctx.cr[6].eq {
	pc = 0x83082C6C; continue 'dispatch;
	}
	// 83082C64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83082C68: 4800009C  b 0x83082d04
	pc = 0x83082D04; continue 'dispatch;
	// 83082C6C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082C70: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83082C74: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 83082C78: 419A0014  beq cr6, 0x83082c8c
	if ctx.cr[6].eq {
	pc = 0x83082C8C; continue 'dispatch;
	}
	// 83082C7C: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83082C80: 419A000C  beq cr6, 0x83082c8c
	if ctx.cr[6].eq {
	pc = 0x83082C8C; continue 'dispatch;
	}
	// 83082C84: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83082C88: 409A0078  bne cr6, 0x83082d00
	if !ctx.cr[6].eq {
	pc = 0x83082D00; continue 'dispatch;
	}
	// 83082C8C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082C90: 4BFFFFB9  bl 0x83082c48
	ctx.lr = 0x83082C94;
	sub_83082C48(ctx, base);
	// 83082C94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83082C98: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83082C9C: 419AFFC8  beq cr6, 0x83082c64
	if ctx.cr[6].eq {
	pc = 0x83082C64; continue 'dispatch;
	}
	// 83082CA0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83082CA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082CA8: 41820054  beq 0x83082cfc
	if ctx.cr[0].eq {
	pc = 0x83082CFC; continue 'dispatch;
	}
	// 83082CAC: 4BFFFF9D  bl 0x83082c48
	ctx.lr = 0x83082CB0;
	sub_83082C48(ctx, base);
	// 83082CB0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83082CB4: 419AFFB0  beq cr6, 0x83082c64
	if ctx.cr[6].eq {
	pc = 0x83082C64; continue 'dispatch;
	}
	// 83082CB8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082CBC: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83082CC0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83082CC4: 409A002C  bne cr6, 0x83082cf0
	if !ctx.cr[6].eq {
	pc = 0x83082CF0; continue 'dispatch;
	}
	// 83082CC8: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83082CCC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83082CD0: 41990008  bgt cr6, 0x83082cd8
	if ctx.cr[6].gt {
	pc = 0x83082CD8; continue 'dispatch;
	}
	// 83082CD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83082CD8: 7D6BF1D7  mullw. r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082CDC: 4182000C  beq 0x83082ce8
	if ctx.cr[0].eq {
	pc = 0x83082CE8; continue 'dispatch;
	}
	// 83082CE0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83082CE4: 4800001C  b 0x83082d00
	pc = 0x83082D00; continue 'dispatch;
	// 83082CE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83082CEC: 48000014  b 0x83082d00
	pc = 0x83082D00; continue 'dispatch;
	// 83082CF0: 7D63EA14  add r11, r3, r29
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83082CF4: 7FCBF1D6  mullw r30, r11, r30
	ctx.r[30].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 83082CF8: 48000008  b 0x83082d00
	pc = 0x83082D00; continue 'dispatch;
	// 83082CFC: 7FDDF1D6  mullw r30, r29, r30
	ctx.r[30].s64 = (ctx.r[29].s32 as i64) * (ctx.r[30].s32 as i64);
	// 83082D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83082D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83082D08: 481254B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082D10 size=128
    let mut pc: u32 = 0x83082D10;
    'dispatch: loop {
        match pc {
            0x83082D10 => {
    //   block [0x83082D10..0x83082D90)
	// 83082D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83082D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83082D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082D20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082D24: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83082D28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83082D2C: 4BF5556D  bl 0x82fd8298
	ctx.lr = 0x83082D30;
	sub_82FD8298(ctx, base);
	// 83082D30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082D34: 41820044  beq 0x83082d78
	if ctx.cr[0].eq {
	pc = 0x83082D78; continue 'dispatch;
	}
	// 83082D38: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83082D3C: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83082D40: 394BD9C8  addi r10, r11, -0x2638
	ctx.r[10].s64 = ctx.r[11].s64 + -9784;
	// 83082D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83082D48: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83082D4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83082D50: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83082D54: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83082D58: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83082D5C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83082D60: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83082D64: 9943001C  stb r10, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 83082D68: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 83082D6C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83082D70: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 83082D74: 48000008  b 0x83082d7c
	pc = 0x83082D7C; continue 'dispatch;
	// 83082D78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83082D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83082D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83082D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83082D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83082D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83082D90 size=12
    let mut pc: u32 = 0x83082D90;
    'dispatch: loop {
        match pc {
            0x83082D90 => {
    //   block [0x83082D90..0x83082D9C)
	// 83082D90: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83082D94: 386B34D0  addi r3, r11, 0x34d0
	ctx.r[3].s64 = ctx.r[11].s64 + 13520;
	// 83082D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082DA0 size=308
    let mut pc: u32 = 0x83082DA0;
    'dispatch: loop {
        match pc {
            0x83082DA0 => {
    //   block [0x83082DA0..0x83082ED4)
	// 83082DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082DA4: 481253C9  bl 0x831a816c
	ctx.lr = 0x83082DA8;
	sub_831A8130(ctx, base);
	// 83082DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082DAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83082DB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83082DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082DB8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83082DBC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83082DC0: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082DC4: 41820070  beq 0x83082e34
	if ctx.cr[0].eq {
	pc = 0x83082E34; continue 'dispatch;
	}
	// 83082DC8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082DCC: 4BF76E35  bl 0x82ff9c00
	ctx.lr = 0x83082DD0;
	sub_82FF9C00(ctx, base);
	// 83082DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082DD4: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83082DD8: 4BFFB8E1  bl 0x8307e6b8
	ctx.lr = 0x83082DDC;
	sub_8307E6B8(ctx, base);
	// 83082DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082DE0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082DE4: 4BF76E1D  bl 0x82ff9c00
	ctx.lr = 0x83082DE8;
	sub_82FF9C00(ctx, base);
	// 83082DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082DEC: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83082DF0: 4BF76E11  bl 0x82ff9c00
	ctx.lr = 0x83082DF4;
	sub_82FF9C00(ctx, base);
	// 83082DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082DF8: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83082DFC: 4BF764FD  bl 0x82ff92f8
	ctx.lr = 0x83082E00;
	sub_82FF92F8(ctx, base);
	// 83082E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E04: 889E001C  lbz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83082E08: 4BF763F9  bl 0x82ff9200
	ctx.lr = 0x83082E0C;
	sub_82FF9200(ctx, base);
	// 83082E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E10: 889E001D  lbz r4, 0x1d(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 83082E14: 4BF763ED  bl 0x82ff9200
	ctx.lr = 0x83082E18;
	sub_82FF9200(ctx, base);
	// 83082E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E1C: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082E20: 4BF764D9  bl 0x82ff92f8
	ctx.lr = 0x83082E24;
	sub_82FF92F8(ctx, base);
	// 83082E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E28: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83082E2C: 4BF764CD  bl 0x82ff92f8
	ctx.lr = 0x83082E30;
	sub_82FF92F8(ctx, base);
	// 83082E30: 4800009C  b 0x83082ecc
	pc = 0x83082ECC; continue 'dispatch;
	// 83082E34: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83082E38: 388B2E10  addi r4, r11, 0x2e10
	ctx.r[4].s64 = ctx.r[11].s64 + 11792;
	// 83082E3C: 4BF76E85  bl 0x82ff9cc0
	ctx.lr = 0x83082E40;
	sub_82FF9CC0(ctx, base);
	// 83082E40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83082E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E48: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83082E4C: 4BFFB8E5  bl 0x8307e730
	ctx.lr = 0x83082E50;
	sub_8307E730(ctx, base);
	// 83082E50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83082E54: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83082E58: 3BAB34D0  addi r29, r11, 0x34d0
	ctx.r[29].s64 = ctx.r[11].s64 + 13520;
	// 83082E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83082E64: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83082E68: 4BF76E59  bl 0x82ff9cc0
	ctx.lr = 0x83082E6C;
	sub_82FF9CC0(ctx, base);
	// 83082E6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83082E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83082E78: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83082E7C: 4BF76E45  bl 0x82ff9cc0
	ctx.lr = 0x83082E80;
	sub_82FF9CC0(ctx, base);
	// 83082E80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83082E84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83082E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082E8C: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83082E90: 4BF766E9  bl 0x82ff9578
	ctx.lr = 0x83082E94;
	sub_82FF9578(ctx, base);
	// 83082E94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83082E98: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 83082E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082EA0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83082EA4: 4BF765DD  bl 0x82ff9480
	ctx.lr = 0x83082EA8;
	sub_82FF9480(ctx, base);
	// 83082EA8: 389E001D  addi r4, r30, 0x1d
	ctx.r[4].s64 = ctx.r[30].s64 + 29;
	// 83082EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082EB0: 4BF765D1  bl 0x82ff9480
	ctx.lr = 0x83082EB4;
	sub_82FF9480(ctx, base);
	// 83082EB4: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 83082EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082EBC: 4BF766BD  bl 0x82ff9578
	ctx.lr = 0x83082EC0;
	sub_82FF9578(ctx, base);
	// 83082EC0: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83082EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83082EC8: 4BF766B1  bl 0x82ff9578
	ctx.lr = 0x83082ECC;
	sub_82FF9578(ctx, base);
	// 83082ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83082ED0: 481252EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83082ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83082ED8 size=1016
    let mut pc: u32 = 0x83082ED8;
    'dispatch: loop {
        match pc {
            0x83082ED8 => {
    //   block [0x83082ED8..0x830832D0)
	// 83082ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83082EDC: 48125289  bl 0x831a8164
	ctx.lr = 0x83082EE0;
	sub_831A8130(ctx, base);
	// 83082EE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83082EE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082EE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83082EEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83082EF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83082EF4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83082EF8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83082EFC: 41980394  blt cr6, 0x83083290
	if ctx.cr[6].lt {
	pc = 0x83083290; continue 'dispatch;
	}
	// 83082F00: 419A0330  beq cr6, 0x83083230
	if ctx.cr[6].eq {
	pc = 0x83083230; continue 'dispatch;
	}
	// 83082F04: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83082F08: 41980278  blt cr6, 0x83083180
	if ctx.cr[6].lt {
	pc = 0x83083180; continue 'dispatch;
	}
	// 83082F0C: 419A01D4  beq cr6, 0x830830e0
	if ctx.cr[6].eq {
	pc = 0x830830E0; continue 'dispatch;
	}
	// 83082F10: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 83082F14: 41980104  blt cr6, 0x83083018
	if ctx.cr[6].lt {
	pc = 0x83083018; continue 'dispatch;
	}
	// 83082F18: 419A0030  beq cr6, 0x83082f48
	if ctx.cr[6].eq {
	pc = 0x83082F48; continue 'dispatch;
	}
	// 83082F1C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83082F20: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83082F24: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83082F28: 388BA460  addi r4, r11, -0x5ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -23456;
	// 83082F2C: 38A00132  li r5, 0x132
	ctx.r[5].s64 = 306;
	// 83082F30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082F34: 4BF4E125  bl 0x82fd1058
	ctx.lr = 0x83082F38;
	sub_82FD1058(ctx, base);
	// 83082F38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83082F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83082F40: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83082F44: 4812DCE5  bl 0x831b0c28
	ctx.lr = 0x83082F48;
	sub_831B0C28(ctx, base);
	// 83082F48: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83082F4C: 419A0128  beq cr6, 0x83083074
	if ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83082F50: 2B1B0002  cmplwi cr6, r27, 2
	ctx.cr[6].compare_u32(ctx.r[27].u32, 2 as u32, &mut ctx.xer);
	// 83082F54: 409A00B0  bne cr6, 0x83083004
	if !ctx.cr[6].eq {
	pc = 0x83083004; continue 'dispatch;
	}
	// 83082F58: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082F5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83082F60: 41820054  beq 0x83082fb4
	if ctx.cr[0].eq {
	pc = 0x83082FB4; continue 'dispatch;
	}
	// 83082F64: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082F68: 4BF5B401  bl 0x82fde368
	ctx.lr = 0x83082F6C;
	sub_82FDE368(ctx, base);
	// 83082F6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83082F70: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082F74: 4BF5B3F5  bl 0x82fde368
	ctx.lr = 0x83082F78;
	sub_82FDE368(ctx, base);
	// 83082F78: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83082F7C: 4BF50CC5  bl 0x82fd3c40
	ctx.lr = 0x83082F80;
	sub_82FD3C40(ctx, base);
	// 83082F80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082F84: 418200F0  beq 0x83083074
	if ctx.cr[0].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83082F88: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082F8C: 4BF5B3DD  bl 0x82fde368
	ctx.lr = 0x83082F90;
	sub_82FDE368(ctx, base);
	// 83082F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83082F94: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082F98: 4BF5B3D1  bl 0x82fde368
	ctx.lr = 0x83082F9C;
	sub_82FDE368(ctx, base);
	// 83082F9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83082FA0: 4BF50CA1  bl 0x82fd3c40
	ctx.lr = 0x83082FA4;
	sub_82FD3C40(ctx, base);
	// 83082FA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082FA8: 41820130  beq 0x830830d8
	if ctx.cr[0].eq {
	pc = 0x830830D8; continue 'dispatch;
	}
	// 83082FAC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83082FB0: 480000C8  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 83082FB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082FB8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83082FBC: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082FC0: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082FC4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83082FC8: 409A00AC  bne cr6, 0x83083074
	if !ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83082FCC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082FD0: 806A0010  lwz r3, 0x10(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082FD4: 4BF50C6D  bl 0x82fd3c40
	ctx.lr = 0x83082FD8;
	sub_82FD3C40(ctx, base);
	// 83082FD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83082FDC: 41820098  beq 0x83083074
	if ctx.cr[0].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83082FE0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83082FE4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83082FE8: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082FEC: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83082FF0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83082FF4: 409A00E4  bne cr6, 0x830830d8
	if !ctx.cr[6].eq {
	pc = 0x830830D8; continue 'dispatch;
	}
	// 83082FF8: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83082FFC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083000: 4BFFFFA0  b 0x83082fa0
	pc = 0x83082FA0; continue 'dispatch;
	// 83083004: 4099000C  ble cr6, 0x83083010
	if !ctx.cr[6].gt {
	pc = 0x83083010; continue 'dispatch;
	}
	// 83083008: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8308300C: 4800006C  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 83083010: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83083014: 48000064  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 83083018: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8308301C: 419A0058  beq cr6, 0x83083074
	if ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83083020: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083024: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083028: 41820058  beq 0x83083080
	if ctx.cr[0].eq {
	pc = 0x83083080; continue 'dispatch;
	}
	// 8308302C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083030: 4BF5B339  bl 0x82fde368
	ctx.lr = 0x83083034;
	sub_82FDE368(ctx, base);
	// 83083034: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83083038: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308303C: 4BF5B32D  bl 0x82fde368
	ctx.lr = 0x83083040;
	sub_82FDE368(ctx, base);
	// 83083040: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83083044: 4BF50BFD  bl 0x82fd3c40
	ctx.lr = 0x83083048;
	sub_82FD3C40(ctx, base);
	// 83083048: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308304C: 40820084  bne 0x830830d0
	if !ctx.cr[0].eq {
	pc = 0x830830D0; continue 'dispatch;
	}
	// 83083050: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083054: 4BF5B315  bl 0x82fde368
	ctx.lr = 0x83083058;
	sub_82FDE368(ctx, base);
	// 83083058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308305C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083060: 4BF5B309  bl 0x82fde368
	ctx.lr = 0x83083064;
	sub_82FDE368(ctx, base);
	// 83083064: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83083068: 4BF50BD9  bl 0x82fd3c40
	ctx.lr = 0x8308306C;
	sub_82FD3C40(ctx, base);
	// 8308306C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083070: 40820060  bne 0x830830d0
	if !ctx.cr[0].eq {
	pc = 0x830830D0; continue 'dispatch;
	}
	// 83083074: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83083078: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8308307C: 48125138  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83083080: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083084: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083088: 83BE0020  lwz r29, 0x20(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308308C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083090: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083094: 409A0018  bne cr6, 0x830830ac
	if !ctx.cr[6].eq {
	pc = 0x830830AC; continue 'dispatch;
	}
	// 83083098: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308309C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830830A0: 4BF50BA1  bl 0x82fd3c40
	ctx.lr = 0x830830A4;
	sub_82FD3C40(ctx, base);
	// 830830A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830830A8: 40820028  bne 0x830830d0
	if !ctx.cr[0].eq {
	pc = 0x830830D0; continue 'dispatch;
	}
	// 830830AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830830B0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830830B4: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830830B8: 409AFFBC  bne cr6, 0x83083074
	if !ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 830830BC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830830C0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830830C4: 4BF50B7D  bl 0x82fd3c40
	ctx.lr = 0x830830C8;
	sub_82FD3C40(ctx, base);
	// 830830C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830830CC: 4182FFA8  beq 0x83083074
	if ctx.cr[0].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 830830D0: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 830830D4: 4099FED8  ble cr6, 0x83082fac
	if !ctx.cr[6].gt {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 830830D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830830DC: 4BFFFF9C  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 830830E0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830830E4: 419AFF90  beq cr6, 0x83083074
	if ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 830830E8: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830830EC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830830F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830830F4: 41820044  beq 0x83083138
	if ctx.cr[0].eq {
	pc = 0x83083138; continue 'dispatch;
	}
	// 830830F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830830FC: 419AFEB0  beq cr6, 0x83082fac
	if ctx.cr[6].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 83083100: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083104: 4BF5B265  bl 0x82fde368
	ctx.lr = 0x83083108;
	sub_82FDE368(ctx, base);
	// 83083108: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8308310C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083110: 4BF5B259  bl 0x82fde368
	ctx.lr = 0x83083114;
	sub_82FDE368(ctx, base);
	// 83083114: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83083118: 4BF50B29  bl 0x82fd3c40
	ctx.lr = 0x8308311C;
	sub_82FD3C40(ctx, base);
	// 8308311C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083120: 418200B8  beq 0x830831d8
	if ctx.cr[0].eq {
	pc = 0x830831D8; continue 'dispatch;
	}
	// 83083124: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83083128: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8308312C: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83083130: 4198FFD0  blt cr6, 0x83083100
	if ctx.cr[6].lt {
	pc = 0x83083100; continue 'dispatch;
	}
	// 83083134: 4BFFFE78  b 0x83082fac
	pc = 0x83082FAC; continue 'dispatch;
	// 83083138: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8308313C: 419AFE70  beq cr6, 0x83082fac
	if ctx.cr[6].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 83083140: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083144: 83BF0020  lwz r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083148: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308314C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083150: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83083154: 409A00D4  bne cr6, 0x83083228
	if !ctx.cr[6].eq {
	pc = 0x83083228; continue 'dispatch;
	}
	// 83083158: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308315C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083160: 4BF50AE1  bl 0x82fd3c40
	ctx.lr = 0x83083164;
	sub_82FD3C40(ctx, base);
	// 83083164: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083168: 418200C0  beq 0x83083228
	if ctx.cr[0].eq {
	pc = 0x83083228; continue 'dispatch;
	}
	// 8308316C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83083170: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83083174: 7F1CD840  cmplw cr6, r28, r27
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83083178: 4198FFD0  blt cr6, 0x83083148
	if ctx.cr[6].lt {
	pc = 0x83083148; continue 'dispatch;
	}
	// 8308317C: 4BFFFE30  b 0x83082fac
	pc = 0x83082FAC; continue 'dispatch;
	// 83083180: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83083184: 419AFE28  beq cr6, 0x83082fac
	if ctx.cr[6].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 83083188: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308318C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83083190: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083194: 4182004C  beq 0x830831e0
	if ctx.cr[0].eq {
	pc = 0x830831E0; continue 'dispatch;
	}
	// 83083198: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8308319C: 419AFE10  beq cr6, 0x83082fac
	if ctx.cr[6].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 830831A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830831A4: 4BF5B1C5  bl 0x82fde368
	ctx.lr = 0x830831A8;
	sub_82FDE368(ctx, base);
	// 830831A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830831AC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830831B0: 4BF5B1B9  bl 0x82fde368
	ctx.lr = 0x830831B4;
	sub_82FDE368(ctx, base);
	// 830831B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830831B8: 4BF50A89  bl 0x82fd3c40
	ctx.lr = 0x830831BC;
	sub_82FD3C40(ctx, base);
	// 830831BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830831C0: 41820018  beq 0x830831d8
	if ctx.cr[0].eq {
	pc = 0x830831D8; continue 'dispatch;
	}
	// 830831C4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830831C8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830831CC: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830831D0: 4198FFD0  blt cr6, 0x830831a0
	if ctx.cr[6].lt {
	pc = 0x830831A0; continue 'dispatch;
	}
	// 830831D4: 4BFFFDD8  b 0x83082fac
	pc = 0x83082FAC; continue 'dispatch;
	// 830831D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830831DC: 4BFFFE9C  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 830831E0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830831E4: 419AFDC8  beq cr6, 0x83082fac
	if ctx.cr[6].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 830831E8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830831EC: 83BF0020  lwz r29, 0x20(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830831F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830831F4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830831F8: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830831FC: 409A002C  bne cr6, 0x83083228
	if !ctx.cr[6].eq {
	pc = 0x83083228; continue 'dispatch;
	}
	// 83083200: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083204: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083208: 4BF50A39  bl 0x82fd3c40
	ctx.lr = 0x8308320C;
	sub_82FD3C40(ctx, base);
	// 8308320C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083210: 41820018  beq 0x83083228
	if ctx.cr[0].eq {
	pc = 0x83083228; continue 'dispatch;
	}
	// 83083214: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83083218: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8308321C: 7F1CD840  cmplw cr6, r28, r27
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83083220: 4198FFD0  blt cr6, 0x830831f0
	if ctx.cr[6].lt {
	pc = 0x830831F0; continue 'dispatch;
	}
	// 83083224: 4BFFFD88  b 0x83082fac
	pc = 0x83082FAC; continue 'dispatch;
	// 83083228: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8308322C: 4BFFFE4C  b 0x83083078
	pc = 0x83083078; continue 'dispatch;
	// 83083230: 2B1B0001  cmplwi cr6, r27, 1
	ctx.cr[6].compare_u32(ctx.r[27].u32, 1 as u32, &mut ctx.xer);
	// 83083234: 409AFEA0  bne cr6, 0x830830d4
	if !ctx.cr[6].eq {
	pc = 0x830830D4; continue 'dispatch;
	}
	// 83083238: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308323C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083240: 4182002C  beq 0x8308326c
	if ctx.cr[0].eq {
	pc = 0x8308326C; continue 'dispatch;
	}
	// 83083244: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083248: 4BF5B121  bl 0x82fde368
	ctx.lr = 0x8308324C;
	sub_82FDE368(ctx, base);
	// 8308324C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83083250: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083254: 4BF5B115  bl 0x82fde368
	ctx.lr = 0x83083258;
	sub_82FDE368(ctx, base);
	// 83083258: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8308325C: 4BF509E5  bl 0x82fd3c40
	ctx.lr = 0x83083260;
	sub_82FD3C40(ctx, base);
	// 83083260: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083264: 4082FD48  bne 0x83082fac
	if !ctx.cr[0].eq {
	pc = 0x83082FAC; continue 'dispatch;
	}
	// 83083268: 4BFFFE0C  b 0x83083074
	pc = 0x83083074; continue 'dispatch;
	// 8308326C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083270: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083274: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083278: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308327C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83083280: 409AFDF4  bne cr6, 0x83083074
	if !ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83083284: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083288: 806A0010  lwz r3, 0x10(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308328C: 4BFFFFD0  b 0x8308325c
	pc = 0x8308325C; continue 'dispatch;
	// 83083290: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83083294: 419AFDE0  beq cr6, 0x83083074
	if ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 83083298: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308329C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830832A0: 4182000C  beq 0x830832ac
	if ctx.cr[0].eq {
	pc = 0x830832AC; continue 'dispatch;
	}
	// 830832A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830832A8: 4BFFFDAC  b 0x83083054
	pc = 0x83083054; continue 'dispatch;
	// 830832AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830832B0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830832B4: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830832B8: 810A0020  lwz r8, 0x20(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 830832BC: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830832C0: 409AFDB4  bne cr6, 0x83083074
	if !ctx.cr[6].eq {
	pc = 0x83083074; continue 'dispatch;
	}
	// 830832C4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830832C8: 806A0010  lwz r3, 0x10(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830832CC: 4BFFFD9C  b 0x83083068
	pc = 0x83083068; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830832D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830832D0 size=852
    let mut pc: u32 = 0x830832D0;
    'dispatch: loop {
        match pc {
            0x830832D0 => {
    //   block [0x830832D0..0x83083624)
	// 830832D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830832D4: 48124E89  bl 0x831a815c
	ctx.lr = 0x830832D8;
	sub_831A8130(ctx, base);
	// 830832D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830832DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830832E0: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 830832E4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830832E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 830832EC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 830832F0: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830832F4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830832F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830832FC: 419802BC  blt cr6, 0x830835b8
	if ctx.cr[6].lt {
	pc = 0x830835B8; continue 'dispatch;
	}
	// 83083300: 419A0268  beq cr6, 0x83083568
	if ctx.cr[6].eq {
	pc = 0x83083568; continue 'dispatch;
	}
	// 83083304: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83083308: 419801F0  blt cr6, 0x830834f8
	if ctx.cr[6].lt {
	pc = 0x830834F8; continue 'dispatch;
	}
	// 8308330C: 419A0184  beq cr6, 0x83083490
	if ctx.cr[6].eq {
	pc = 0x83083490; continue 'dispatch;
	}
	// 83083310: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 83083314: 419800E4  blt cr6, 0x830833f8
	if ctx.cr[6].lt {
	pc = 0x830833F8; continue 'dispatch;
	}
	// 83083318: 419A0030  beq cr6, 0x83083348
	if ctx.cr[6].eq {
	pc = 0x83083348; continue 'dispatch;
	}
	// 8308331C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83083320: 80FC0014  lwz r7, 0x14(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 83083324: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83083328: 388BA460  addi r4, r11, -0x5ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -23456;
	// 8308332C: 38A001D2  li r5, 0x1d2
	ctx.r[5].s64 = 466;
	// 83083330: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83083334: 4BF4DD25  bl 0x82fd1058
	ctx.lr = 0x83083338;
	sub_82FD1058(ctx, base);
	// 83083338: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308333C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83083340: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83083344: 4812D8E5  bl 0x831b0c28
	ctx.lr = 0x83083348;
	sub_831B0C28(ctx, base);
	// 83083348: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8308334C: 419A0274  beq cr6, 0x830835c0
	if ctx.cr[6].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 83083350: 2B190002  cmplwi cr6, r25, 2
	ctx.cr[6].compare_u32(ctx.r[25].u32, 2 as u32, &mut ctx.xer);
	// 83083354: 409A0090  bne cr6, 0x830833e4
	if !ctx.cr[6].eq {
	pc = 0x830833E4; continue 'dispatch;
	}
	// 83083358: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308335C: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083360: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083364: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083368: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8308336C: 409A0018  bne cr6, 0x83083384
	if !ctx.cr[6].eq {
	pc = 0x83083384; continue 'dispatch;
	}
	// 83083370: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083374: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083378: 4BF508C9  bl 0x82fd3c40
	ctx.lr = 0x8308337C;
	sub_82FD3C40(ctx, base);
	// 8308337C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083380: 4082001C  bne 0x8308339c
	if !ctx.cr[0].eq {
	pc = 0x8308339C; continue 'dispatch;
	}
	// 83083384: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83083388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308338C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083390: 4BFFA939  bl 0x8307dcc8
	ctx.lr = 0x83083394;
	sub_8307DCC8(ctx, base);
	// 83083394: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083398: 41820228  beq 0x830835c0
	if ctx.cr[0].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 8308339C: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830833A0: 83DC0008  lwz r30, 8(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830833A4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830833A8: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830833AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830833B0: 409A0018  bne cr6, 0x830833c8
	if !ctx.cr[6].eq {
	pc = 0x830833C8; continue 'dispatch;
	}
	// 830833B4: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830833B8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830833BC: 4BF50885  bl 0x82fd3c40
	ctx.lr = 0x830833C0;
	sub_82FD3C40(ctx, base);
	// 830833C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830833C4: 40820254  bne 0x83083618
	if !ctx.cr[0].eq {
	pc = 0x83083618; continue 'dispatch;
	}
	// 830833C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830833CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830833D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830833D4: 4BFFA8F5  bl 0x8307dcc8
	ctx.lr = 0x830833D8;
	sub_8307DCC8(ctx, base);
	// 830833D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830833DC: 4082023C  bne 0x83083618
	if !ctx.cr[0].eq {
	pc = 0x83083618; continue 'dispatch;
	}
	// 830833E0: 480000A8  b 0x83083488
	pc = 0x83083488; continue 'dispatch;
	// 830833E4: 4099000C  ble cr6, 0x830833f0
	if !ctx.cr[6].gt {
	pc = 0x830833F0; continue 'dispatch;
	}
	// 830833E8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830833EC: 48000230  b 0x8308361c
	pc = 0x8308361C; continue 'dispatch;
	// 830833F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830833F4: 48000228  b 0x8308361c
	pc = 0x8308361C; continue 'dispatch;
	// 830833F8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830833FC: 419A01C4  beq cr6, 0x830835c0
	if ctx.cr[6].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 83083400: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083404: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083408: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308340C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083410: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083414: 409A0018  bne cr6, 0x8308342c
	if !ctx.cr[6].eq {
	pc = 0x8308342C; continue 'dispatch;
	}
	// 83083418: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308341C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083420: 4BF50821  bl 0x82fd3c40
	ctx.lr = 0x83083424;
	sub_82FD3C40(ctx, base);
	// 83083424: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083428: 40820058  bne 0x83083480
	if !ctx.cr[0].eq {
	pc = 0x83083480; continue 'dispatch;
	}
	// 8308342C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083430: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083434: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083438: 409A0018  bne cr6, 0x83083450
	if !ctx.cr[6].eq {
	pc = 0x83083450; continue 'dispatch;
	}
	// 8308343C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083440: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083444: 4BF507FD  bl 0x82fd3c40
	ctx.lr = 0x83083448;
	sub_82FD3C40(ctx, base);
	// 83083448: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308344C: 40820034  bne 0x83083480
	if !ctx.cr[0].eq {
	pc = 0x83083480; continue 'dispatch;
	}
	// 83083450: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83083454: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83083458: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308345C: 4BFFA86D  bl 0x8307dcc8
	ctx.lr = 0x83083460;
	sub_8307DCC8(ctx, base);
	// 83083460: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083464: 4082001C  bne 0x83083480
	if !ctx.cr[0].eq {
	pc = 0x83083480; continue 'dispatch;
	}
	// 83083468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308346C: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083470: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083474: 4BFFA855  bl 0x8307dcc8
	ctx.lr = 0x83083478;
	sub_8307DCC8(ctx, base);
	// 83083478: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308347C: 41820144  beq 0x830835c0
	if ctx.cr[0].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 83083480: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 83083484: 40990194  ble cr6, 0x83083618
	if !ctx.cr[6].gt {
	pc = 0x83083618; continue 'dispatch;
	}
	// 83083488: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8308348C: 48000190  b 0x8308361c
	pc = 0x8308361C; continue 'dispatch;
	// 83083490: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83083494: 419A012C  beq cr6, 0x830835c0
	if ctx.cr[6].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 83083498: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8308349C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 830834A0: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830834A4: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830834A8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830834AC: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830834B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830834B4: 409A0018  bne cr6, 0x830834cc
	if !ctx.cr[6].eq {
	pc = 0x830834CC; continue 'dispatch;
	}
	// 830834B8: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830834BC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830834C0: 4BF50781  bl 0x82fd3c40
	ctx.lr = 0x830834C4;
	sub_82FD3C40(ctx, base);
	// 830834C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830834C8: 4082001C  bne 0x830834e4
	if !ctx.cr[0].eq {
	pc = 0x830834E4; continue 'dispatch;
	}
	// 830834CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830834D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830834D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830834D8: 4BFFA7F1  bl 0x8307dcc8
	ctx.lr = 0x830834DC;
	sub_8307DCC8(ctx, base);
	// 830834DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830834E0: 41820080  beq 0x83083560
	if ctx.cr[0].eq {
	pc = 0x83083560; continue 'dispatch;
	}
	// 830834E4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 830834E8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830834EC: 7F1AC840  cmplw cr6, r26, r25
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[25].u32, &mut ctx.xer);
	// 830834F0: 4198FFB0  blt cr6, 0x830834a0
	if ctx.cr[6].lt {
	pc = 0x830834A0; continue 'dispatch;
	}
	// 830834F4: 48000124  b 0x83083618
	pc = 0x83083618; continue 'dispatch;
	// 830834F8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830834FC: 419A011C  beq cr6, 0x83083618
	if ctx.cr[6].eq {
	pc = 0x83083618; continue 'dispatch;
	}
	// 83083500: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83083504: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 83083508: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308350C: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083510: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083514: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083518: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8308351C: 409A0018  bne cr6, 0x83083534
	if !ctx.cr[6].eq {
	pc = 0x83083534; continue 'dispatch;
	}
	// 83083520: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083524: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083528: 4BF50719  bl 0x82fd3c40
	ctx.lr = 0x8308352C;
	sub_82FD3C40(ctx, base);
	// 8308352C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083530: 4082001C  bne 0x8308354c
	if !ctx.cr[0].eq {
	pc = 0x8308354C; continue 'dispatch;
	}
	// 83083534: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83083538: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8308353C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083540: 4BFFA789  bl 0x8307dcc8
	ctx.lr = 0x83083544;
	sub_8307DCC8(ctx, base);
	// 83083544: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083548: 41820018  beq 0x83083560
	if ctx.cr[0].eq {
	pc = 0x83083560; continue 'dispatch;
	}
	// 8308354C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83083550: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83083554: 7F1AC840  cmplw cr6, r26, r25
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83083558: 4198FFB0  blt cr6, 0x83083508
	if ctx.cr[6].lt {
	pc = 0x83083508; continue 'dispatch;
	}
	// 8308355C: 480000BC  b 0x83083618
	pc = 0x83083618; continue 'dispatch;
	// 83083560: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83083564: 480000B8  b 0x8308361c
	pc = 0x8308361C; continue 'dispatch;
	// 83083568: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 8308356C: 409AFF18  bne cr6, 0x83083484
	if !ctx.cr[6].eq {
	pc = 0x83083484; continue 'dispatch;
	}
	// 83083570: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083574: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083578: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308357C: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083580: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083584: 409A0018  bne cr6, 0x8308359c
	if !ctx.cr[6].eq {
	pc = 0x8308359C; continue 'dispatch;
	}
	// 83083588: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308358C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083590: 4BF506B1  bl 0x82fd3c40
	ctx.lr = 0x83083594;
	sub_82FD3C40(ctx, base);
	// 83083594: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083598: 40820080  bne 0x83083618
	if !ctx.cr[0].eq {
	pc = 0x83083618; continue 'dispatch;
	}
	// 8308359C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830835A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830835A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830835A8: 4BFFA721  bl 0x8307dcc8
	ctx.lr = 0x830835AC;
	sub_8307DCC8(ctx, base);
	// 830835AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830835B0: 40820068  bne 0x83083618
	if !ctx.cr[0].eq {
	pc = 0x83083618; continue 'dispatch;
	}
	// 830835B4: 4800000C  b 0x830835c0
	pc = 0x830835C0; continue 'dispatch;
	// 830835B8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830835BC: 409A000C  bne cr6, 0x830835c8
	if !ctx.cr[6].eq {
	pc = 0x830835C8; continue 'dispatch;
	}
	// 830835C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830835C4: 48000058  b 0x8308361c
	pc = 0x8308361C; continue 'dispatch;
	// 830835C8: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830835CC: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830835D0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830835D4: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830835D8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830835DC: 409A0018  bne cr6, 0x830835f4
	if !ctx.cr[6].eq {
	pc = 0x830835F4; continue 'dispatch;
	}
	// 830835E0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830835E4: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830835E8: 4BF50659  bl 0x82fd3c40
	ctx.lr = 0x830835EC;
	sub_82FD3C40(ctx, base);
	// 830835EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830835F0: 4082001C  bne 0x8308360c
	if !ctx.cr[0].eq {
	pc = 0x8308360C; continue 'dispatch;
	}
	// 830835F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830835F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830835FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083600: 4BFFA6C9  bl 0x8307dcc8
	ctx.lr = 0x83083604;
	sub_8307DCC8(ctx, base);
	// 83083604: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083608: 4182FFB8  beq 0x830835c0
	if ctx.cr[0].eq {
	pc = 0x830835C0; continue 'dispatch;
	}
	// 8308360C: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 83083610: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83083614: 41990008  bgt cr6, 0x8308361c
	if ctx.cr[6].gt {
	pc = 0x8308361C; continue 'dispatch;
	}
	// 83083618: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8308361C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83083620: 48124B8C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083628 size=272
    let mut pc: u32 = 0x83083628;
    'dispatch: loop {
        match pc {
            0x83083628 => {
    //   block [0x83083628..0x83083738)
	// 83083628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308362C: 48124B39  bl 0x831a8164
	ctx.lr = 0x83083630;
	sub_831A8130(ctx, base);
	// 83083630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83083638: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8308363C: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83083640: 3CE08217  lis r7, -0x7de9
	ctx.r[7].s64 = -2112421888;
	// 83083644: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83083648: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308364C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83083650: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83083654: 3CA08217  lis r5, -0x7de9
	ctx.r[5].s64 = -2112421888;
	// 83083658: 81479758  lwz r10, -0x68a8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-26792 as u32) ) } as u64;
	// 8308365C: 3CC08217  lis r6, -0x7de9
	ctx.r[6].s64 = -2112421888;
	// 83083660: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083664: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083668: 419A002C  beq cr6, 0x83083694
	if ctx.cr[6].eq {
	pc = 0x83083694; continue 'dispatch;
	}
	// 8308366C: 80869770  lwz r4, -0x6890(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83083670: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83083674: 419A0020  beq cr6, 0x83083694
	if ctx.cr[6].eq {
	pc = 0x83083694; continue 'dispatch;
	}
	// 83083678: 80859774  lwz r4, -0x688c(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 8308367C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83083680: 419A0014  beq cr6, 0x83083694
	if ctx.cr[6].eq {
	pc = 0x83083694; continue 'dispatch;
	}
	// 83083684: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83083688: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8308368C: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83083690: 81479758  lwz r10, -0x68a8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-26792 as u32) ) } as u64;
	// 83083694: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083698: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308369C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830836A0: 419A0028  beq cr6, 0x830836c8
	if ctx.cr[6].eq {
	pc = 0x830836C8; continue 'dispatch;
	}
	// 830836A4: 81469770  lwz r10, -0x6890(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 830836A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830836AC: 419A001C  beq cr6, 0x830836c8
	if ctx.cr[6].eq {
	pc = 0x830836C8; continue 'dispatch;
	}
	// 830836B0: 81459774  lwz r10, -0x688c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 830836B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830836B8: 419A0010  beq cr6, 0x830836c8
	if ctx.cr[6].eq {
	pc = 0x830836C8; continue 'dispatch;
	}
	// 830836BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830836C0: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830836C4: 91690020  stw r11, 0x20(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830836C8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830836CC: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830836D0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830836D4: 409A005C  bne cr6, 0x83083730
	if !ctx.cr[6].eq {
	pc = 0x83083730; continue 'dispatch;
	}
	// 830836D8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 830836DC: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830836E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830836E4: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830836E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830836EC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830836F0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830836F4: 4801E7CD  bl 0x830a1ec0
	ctx.lr = 0x830836F8;
	sub_830A1EC0(ctx, base);
	// 830836F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830836FC: 41820034  beq 0x83083730
	if ctx.cr[0].eq {
	pc = 0x83083730; continue 'dispatch;
	}
	// 83083700: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083704: 4BF5AC65  bl 0x82fde368
	ctx.lr = 0x83083708;
	sub_82FDE368(ctx, base);
	// 83083708: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308370C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083710: 4BF5AC59  bl 0x82fde368
	ctx.lr = 0x83083714;
	sub_82FDE368(ctx, base);
	// 83083714: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83083718: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8308371C: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 83083720: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83083724: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83083728: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308372C: 4BF93E2D  bl 0x83017558
	ctx.lr = 0x83083730;
	sub_83017558(ctx, base);
	// 83083730: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83083734: 48124A80  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083738 size=176
    let mut pc: u32 = 0x83083738;
    'dispatch: loop {
        match pc {
            0x83083738 => {
    //   block [0x83083738..0x830837E8)
	// 83083738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308373C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83083740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83083744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83083748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308374C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83083750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83083754: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83083758: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308375C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83083760: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83083764: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83083768: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308376C: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 83083770: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83083774: 40990048  ble cr6, 0x830837bc
	if !ctx.cr[6].gt {
	pc = 0x830837BC; continue 'dispatch;
	}
	// 83083778: 556AE8FE  srwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8308377C: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083780: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83083784: 4182000C  beq 0x83083790
	if ctx.cr[0].eq {
	pc = 0x83083790; continue 'dispatch;
	}
	// 83083788: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8308378C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83083790: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83083794: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308379C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830837A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830837A4: 4E800421  bctrl
	ctx.lr = 0x830837A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830837A8: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830837AC: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830837B0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830837B4: 48124D5D  bl 0x831a8510
	ctx.lr = 0x830837B8;
	sub_831A8510(ctx, base);
	// 830837B8: 48000014  b 0x830837cc
	pc = 0x830837CC; continue 'dispatch;
	// 830837BC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830837C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830837C4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830837C8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830837CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830837D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830837D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830837D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830837DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830837E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830837E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830837E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830837E8 size=48
    let mut pc: u32 = 0x830837E8;
    'dispatch: loop {
        match pc {
            0x830837E8 => {
    //   block [0x830837E8..0x83083818)
	// 830837E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830837EC: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830837F0: 40980028  bge cr6, 0x83083818
	if !ctx.cr[6].lt {
		sub_83083818(ctx, base);
		return;
	}
	// 830837F4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830837F8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830837FC: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083800: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83083804: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83083808: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308380C: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 83083810: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83083814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083818 size=16
    let mut pc: u32 = 0x83083818;
    'dispatch: loop {
        match pc {
            0x83083818 => {
    //   block [0x83083818..0x83083828)
	// 83083818: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308381C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083820: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83083824: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083828 size=48
    let mut pc: u32 = 0x83083828;
    'dispatch: loop {
        match pc {
            0x83083828 => {
    //   block [0x83083828..0x83083858)
	// 83083828: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308382C: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083830: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83083834: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083838: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308383C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083840: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 83083844: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83083848: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308384C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083850: 4198FFD8  blt cr6, 0x83083828
	if ctx.cr[6].lt {
	pc = 0x83083828; continue 'dispatch;
	}
	// 83083854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083858 size=172
    let mut pc: u32 = 0x83083858;
    'dispatch: loop {
        match pc {
            0x83083858 => {
    //   block [0x83083858..0x83083904)
	// 83083858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83083860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083868: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8308386C: 41980030  blt cr6, 0x8308389c
	if ctx.cr[6].lt {
	pc = 0x8308389C; continue 'dispatch;
	}
	// 83083870: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83083874: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83083878: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8308387C: 388BA4A4  addi r4, r11, -0x5b5c
	ctx.r[4].s64 = ctx.r[11].s64 + -23388;
	// 83083880: 38A000F5  li r5, 0xf5
	ctx.r[5].s64 = 245;
	// 83083884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083888: 4BF4D0D1  bl 0x82fd0958
	ctx.lr = 0x8308388C;
	sub_82FD0958(ctx, base);
	// 8308388C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83083890: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083894: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83083898: 4812D391  bl 0x831b0c28
	ctx.lr = 0x8308389C;
	sub_831B0C28(ctx, base);
	// 8308389C: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830838A0: 40980028  bge cr6, 0x830838c8
	if !ctx.cr[6].lt {
	pc = 0x830838C8; continue 'dispatch;
	}
	// 830838A4: 548B06FE  clrlwi r11, r4, 0x1b
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 830838A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830838AC: 2B040020  cmplwi cr6, r4, 0x20
	ctx.cr[6].compare_u32(ctx.r[4].u32, 32 as u32, &mut ctx.xer);
	// 830838B0: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 830838B4: 4098000C  bge cr6, 0x830838c0
	if !ctx.cr[6].lt {
	pc = 0x830838C0; continue 'dispatch;
	}
	// 830838B8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830838BC: 48000024  b 0x830838e0
	pc = 0x830838E0; continue 'dispatch;
	// 830838C0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830838C4: 4800001C  b 0x830838e0
	pc = 0x830838E0; continue 'dispatch;
	// 830838C8: 548B077E  clrlwi r11, r4, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 830838CC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830838D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830838D4: 5488E8FE  srwi r8, r4, 3
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shr(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830838D8: 7D4850AE  lbzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830838DC: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 830838E0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830838E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830838E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830838EC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830838F0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830838F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830838F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830838FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83083900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083908 size=52
    let mut pc: u32 = 0x83083908;
    'dispatch: loop {
        match pc {
            0x83083908 => {
    //   block [0x83083908..0x8308393C)
	// 83083908: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308390C: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83083910: 4098002C  bge cr6, 0x8308393c
	if !ctx.cr[6].lt {
		sub_8308393C(ctx, base);
		return;
	}
	// 83083914: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8308391C: 409A0014  bne cr6, 0x83083930
	if !ctx.cr[6].eq {
	pc = 0x83083930; continue 'dispatch;
	}
	// 83083920: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83083928: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308392C: 419A0008  beq cr6, 0x83083934
	if ctx.cr[6].eq {
	pc = 0x83083934; continue 'dispatch;
	}
	// 83083930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083934: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83083938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308393C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308393C size=52
    let mut pc: u32 = 0x8308393C;
    'dispatch: loop {
        match pc {
            0x8308393C => {
    //   block [0x8308393C..0x83083970)
	// 8308393C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083944: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083948: 41820020  beq 0x83083968
	if ctx.cr[0].eq {
	pc = 0x83083968; continue 'dispatch;
	}
	// 8308394C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083950: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083954: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083958: 40820018  bne 0x83083970
	if !ctx.cr[0].eq {
		sub_83083970(ctx, base);
		return;
	}
	// 8308395C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83083960: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083964: 4198FFEC  blt cr6, 0x83083950
	if ctx.cr[6].lt {
	pc = 0x83083950; continue 'dispatch;
	}
	// 83083968: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8308396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083970 size=8
    let mut pc: u32 = 0x83083970;
    'dispatch: loop {
        match pc {
            0x83083970 => {
    //   block [0x83083970..0x83083978)
	// 83083970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83083974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083978 size=104
    let mut pc: u32 = 0x83083978;
    'dispatch: loop {
        match pc {
            0x83083978 => {
    //   block [0x83083978..0x830839E0)
	// 83083978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83083980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83083984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83083988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308398C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83083990: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83083994: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083998: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308399C: 41820018  beq 0x830839b4
	if ctx.cr[0].eq {
	pc = 0x830839B4; continue 'dispatch;
	}
	// 830839A0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830839A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830839A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830839AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830839B0: 4E800421  bctrl
	ctx.lr = 0x830839B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830839B4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830839B8: 4182000C  beq 0x830839c4
	if ctx.cr[0].eq {
	pc = 0x830839C4; continue 'dispatch;
	}
	// 830839BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830839C0: 4BF54921  bl 0x82fd82e0
	ctx.lr = 0x830839C4;
	sub_82FD82E0(ctx, base);
	// 830839C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830839C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830839CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830839D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830839D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830839D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830839DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830839E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830839E0 size=24
    let mut pc: u32 = 0x830839E0;
    'dispatch: loop {
        match pc {
            0x830839E0 => {
    //   block [0x830839E0..0x830839F8)
	// 830839E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830839E4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830839E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830839EC: 419A000C  beq cr6, 0x830839f8
	if ctx.cr[6].eq {
		sub_830839F8(ctx, base);
		return;
	}
	// 830839F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830839F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830839F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830839F8 size=56
    let mut pc: u32 = 0x830839F8;
    'dispatch: loop {
        match pc {
            0x830839F8 => {
    //   block [0x830839F8..0x83083A30)
	// 830839F8: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830839FC: 40980034  bge cr6, 0x83083a30
	if !ctx.cr[6].lt {
		sub_83083A30(ctx, base);
		return;
	}
	// 83083A00: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083A04: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083A08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083A0C: 409A0018  bne cr6, 0x83083a24
	if !ctx.cr[6].eq {
	pc = 0x83083A24; continue 'dispatch;
	}
	// 83083A10: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083A14: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083A18: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083A1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83083A20: 419A0008  beq cr6, 0x83083a28
	if ctx.cr[6].eq {
	pc = 0x83083A28; continue 'dispatch;
	}
	// 83083A24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083A28: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83083A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083A30 size=60
    let mut pc: u32 = 0x83083A30;
    'dispatch: loop {
        match pc {
            0x83083A30 => {
    //   block [0x83083A30..0x83083A6C)
	// 83083A30: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083A38: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083A3C: 41820028  beq 0x83083a64
	if ctx.cr[0].eq {
	pc = 0x83083A64; continue 'dispatch;
	}
	// 83083A40: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083A44: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083A48: 7CE958AE  lbzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083A4C: 7CCA58AE  lbzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083A50: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 83083A54: 409AFF9C  bne cr6, 0x830839f0
	if !ctx.cr[6].eq {
		sub_830839E0(ctx, base);
		return;
	}
	// 83083A58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83083A5C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83083A60: 4198FFE8  blt cr6, 0x83083a48
	if ctx.cr[6].lt {
	pc = 0x83083A48; continue 'dispatch;
	}
	// 83083A64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83083A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083A70 size=36
    let mut pc: u32 = 0x83083A70;
    'dispatch: loop {
        match pc {
            0x83083A70 => {
    //   block [0x83083A70..0x83083A94)
	// 83083A70: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83083A74: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083A78: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83083A7C: 40980018  bge cr6, 0x83083a94
	if !ctx.cr[6].lt {
		sub_83083A94(ctx, base);
		return;
	}
	// 83083A80: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083A84: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083A88: 1D6B001F  mulli r11, r11, 0x1f
	ctx.r[11].s64 = ctx.r[11].s64 * 31;
	// 83083A8C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83083A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083A94 size=16
    let mut pc: u32 = 0x83083A94;
    'dispatch: loop {
        match pc {
            0x83083A94 => {
    //   block [0x83083A94..0x83083AA4)
	// 83083A94: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083A98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83083A9C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083AA0: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083AA4 size=28
    let mut pc: u32 = 0x83083AA4;
    'dispatch: loop {
        match pc {
            0x83083AA4 => {
    //   block [0x83083AA4..0x83083AC0)
	// 83083AA4: 810A0010  lwz r8, 0x10(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083AA8: 7D4858AE  lbzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083AAC: 1D23001F  mulli r9, r3, 0x1f
	ctx.r[9].s64 = ctx.r[3].s64 * 31;
	// 83083AB0: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83083AB4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083AB8: 4080FFF0  bge 0x83083aa8
	if !ctx.cr[0].lt {
	pc = 0x83083AA8; continue 'dispatch;
	}
	// 83083ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083AC0 size=64
    let mut pc: u32 = 0x83083AC0;
    'dispatch: loop {
        match pc {
            0x83083AC0 => {
    //   block [0x83083AC0..0x83083B00)
	// 83083AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83083AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83083AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83083ACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083AD0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83083AD4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83083AD8: 4BFFFF99  bl 0x83083a70
	ctx.lr = 0x83083ADC;
	sub_83083A70(ctx, base);
	// 83083ADC: 7D63FB96  divwu r11, r3, r31
	ctx.r[11].u32 = ctx.r[3].u32 / ctx.r[31].u32;
	// 83083AE0: 0CDF0000  twi 6, r31, 0
	// 83083AE4: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[31].s32 as i64);
	// 83083AE8: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83083AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83083AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83083AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83083AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83083AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083B00 size=12
    let mut pc: u32 = 0x83083B00;
    'dispatch: loop {
        match pc {
            0x83083B00 => {
    //   block [0x83083B00..0x83083B0C)
	// 83083B00: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83083B04: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83083B08: 4BFFFED8  b 0x830839e0
	sub_830839E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83083B10 size=452
    let mut pc: u32 = 0x83083B10;
    'dispatch: loop {
        match pc {
            0x83083B10 => {
    //   block [0x83083B10..0x83083CD4)
	// 83083B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83083B14: 48124631  bl 0x831a8144
	ctx.lr = 0x83083B18;
	sub_831A8130(ctx, base);
	// 83083B18: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083B1C: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 83083B20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83083B24: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 83083B28: 409A0014  bne cr6, 0x83083b3c
	if !ctx.cr[6].eq {
	pc = 0x83083B3C; continue 'dispatch;
	}
	// 83083B2C: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083B30: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83083B34: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83083B38: 4800018C  b 0x83083cc4
	pc = 0x83083CC4; continue 'dispatch;
	// 83083B3C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83083B40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83083B44: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 83083B48: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 83083B4C: 419A0160  beq cr6, 0x83083cac
	if ctx.cr[6].eq {
	pc = 0x83083CAC; continue 'dispatch;
	}
	// 83083B50: 3EE08217  lis r23, -0x7de9
	ctx.r[23].s64 = -2112421888;
	// 83083B54: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 83083B58: 3E608217  lis r19, -0x7de9
	ctx.r[19].s64 = -2112421888;
	// 83083B5C: 83979754  lwz r28, -0x68ac(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083B60: 897D0038  lbz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83083B64: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83083B68: 83560000  lwz r26, 0(r22)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083B6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083B70: 41820014  beq 0x83083b84
	if ctx.cr[0].eq {
	pc = 0x83083B84; continue 'dispatch;
	}
	// 83083B74: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83083B78: 4BF5A7F1  bl 0x82fde368
	ctx.lr = 0x83083B7C;
	sub_82FDE368(ctx, base);
	// 83083B7C: 83979754  lwz r28, -0x68ac(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083B80: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83083B84: 897D0039  lbz r11, 0x39(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(57 as u32) ) } as u64;
	// 83083B88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083B8C: 41820014  beq 0x83083ba0
	if ctx.cr[0].eq {
	pc = 0x83083BA0; continue 'dispatch;
	}
	// 83083B90: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083B94: 81539774  lwz r10, -0x688c(r19)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 83083B98: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083B9C: 419A0100  beq cr6, 0x83083c9c
	if ctx.cr[6].eq {
	pc = 0x83083C9C; continue 'dispatch;
	}
	// 83083BA0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083BA4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83083BA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83083BAC: 409900D4  ble cr6, 0x83083c80
	if !ctx.cr[6].gt {
	pc = 0x83083C80; continue 'dispatch;
	}
	// 83083BB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83083BB4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083BB8: 895D0038  lbz r10, 0x38(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83083BBC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083BC0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83083BC4: 41820018  beq 0x83083bdc
	if ctx.cr[0].eq {
	pc = 0x83083BDC; continue 'dispatch;
	}
	// 83083BC8: 4BF5A7A1  bl 0x82fde368
	ctx.lr = 0x83083BCC;
	sub_82FDE368(ctx, base);
	// 83083BCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83083BD0: 4BF50071  bl 0x82fd3c40
	ctx.lr = 0x83083BD4;
	sub_82FD3C40(ctx, base);
	// 83083BD4: 83979754  lwz r28, -0x68ac(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083BD8: 48000030  b 0x83083c08
	pc = 0x83083C08; continue 'dispatch;
	// 83083BDC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083BE0: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83083BE4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83083BE8: 4082002C  bne 0x83083c14
	if !ctx.cr[0].eq {
	pc = 0x83083C14; continue 'dispatch;
	}
	// 83083BEC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083BF0: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083BF4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083BF8: 409A0074  bne cr6, 0x83083c6c
	if !ctx.cr[6].eq {
	pc = 0x83083C6C; continue 'dispatch;
	}
	// 83083BFC: 809A0010  lwz r4, 0x10(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083C00: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083C04: 4BF5003D  bl 0x82fd3c40
	ctx.lr = 0x83083C08;
	sub_82FD3C40(ctx, base);
	// 83083C08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083C0C: 41820060  beq 0x83083c6c
	if ctx.cr[0].eq {
	pc = 0x83083C6C; continue 'dispatch;
	}
	// 83083C10: 48000044  b 0x83083c54
	pc = 0x83083C54; continue 'dispatch;
	// 83083C14: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83083C18: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83083C1C: 419A0038  beq cr6, 0x83083c54
	if ctx.cr[6].eq {
	pc = 0x83083C54; continue 'dispatch;
	}
	// 83083C20: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83083C24: 409A0018  bne cr6, 0x83083c3c
	if !ctx.cr[6].eq {
	pc = 0x83083C3C; continue 'dispatch;
	}
	// 83083C28: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083C2C: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083C30: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083C34: 409A0038  bne cr6, 0x83083c6c
	if !ctx.cr[6].eq {
	pc = 0x83083C6C; continue 'dispatch;
	}
	// 83083C38: 4800001C  b 0x83083c54
	pc = 0x83083C54; continue 'dispatch;
	// 83083C3C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83083C40: 409A002C  bne cr6, 0x83083c6c
	if !ctx.cr[6].eq {
	pc = 0x83083C6C; continue 'dispatch;
	}
	// 83083C44: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083C48: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083C4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083C50: 419A001C  beq cr6, 0x83083c6c
	if ctx.cr[6].eq {
	pc = 0x83083C6C; continue 'dispatch;
	}
	// 83083C54: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83083C58: 576A103A  slwi r10, r27, 2
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83083C5C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083C60: 7FCBF82E  lwzx r30, r11, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83083C64: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83083C68: 409A0020  bne cr6, 0x83083c88
	if !ctx.cr[6].eq {
	pc = 0x83083C88; continue 'dispatch;
	}
	// 83083C6C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083C70: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 83083C74: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83083C78: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083C7C: 4198FF38  blt cr6, 0x83083bb4
	if ctx.cr[6].lt {
	pc = 0x83083BB4; continue 'dispatch;
	}
	// 83083C80: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83083C84: 419A0048  beq cr6, 0x83083ccc
	if ctx.cr[6].eq {
	pc = 0x83083CCC; continue 'dispatch;
	}
	// 83083C88: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083C8C: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083C90: 419A003C  beq cr6, 0x83083ccc
	if ctx.cr[6].eq {
	pc = 0x83083CCC; continue 'dispatch;
	}
	// 83083C94: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 83083C98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83083C9C: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 83083CA0: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 83083CA4: 7F15A040  cmplw cr6, r21, r20
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[20].u32, &mut ctx.xer);
	// 83083CA8: 4198FEB8  blt cr6, 0x83083b60
	if ctx.cr[6].lt {
	pc = 0x83083B60; continue 'dispatch;
	}
	// 83083CAC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83083CB0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83083CB4: 7D6BD8AE  lbzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83083CB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083CBC: 41820008  beq 0x83083cc4
	if ctx.cr[0].eq {
	pc = 0x83083CC4; continue 'dispatch;
	}
	// 83083CC0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83083CC4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83083CC8: 481244CC  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 83083CCC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83083CD0: 4BFFFFF4  b 0x83083cc4
	pc = 0x83083CC4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83083CD8 size=404
    let mut pc: u32 = 0x83083CD8;
    'dispatch: loop {
        match pc {
            0x83083CD8 => {
    //   block [0x83083CD8..0x83083E6C)
	// 83083CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83083CDC: 48124471  bl 0x831a814c
	ctx.lr = 0x83083CE0;
	sub_831A8130(ctx, base);
	// 83083CE0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083CE4: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83083CE8: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 83083CEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83083CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83083CF4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83083CF8: 409A0014  bne cr6, 0x83083d0c
	if !ctx.cr[6].eq {
	pc = 0x83083D0C; continue 'dispatch;
	}
	// 83083CFC: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83083D00: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83083D04: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83083D08: 48000154  b 0x83083e5c
	pc = 0x83083E5C; continue 'dispatch;
	// 83083D0C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83083D10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83083D14: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 83083D18: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83083D1C: 419A0128  beq cr6, 0x83083e44
	if ctx.cr[6].eq {
	pc = 0x83083E44; continue 'dispatch;
	}
	// 83083D20: 3F208217  lis r25, -0x7de9
	ctx.r[25].s64 = -2112421888;
	// 83083D24: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 83083D28: 3EA08217  lis r21, -0x7de9
	ctx.r[21].s64 = -2112421888;
	// 83083D2C: 81599754  lwz r10, -0x68ac(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083D30: 897D0039  lbz r11, 0x39(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(57 as u32) ) } as u64;
	// 83083D34: 83570000  lwz r26, 0(r23)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083D38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083D3C: 41820014  beq 0x83083d50
	if ctx.cr[0].eq {
	pc = 0x83083D50; continue 'dispatch;
	}
	// 83083D40: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083D44: 81359774  lwz r9, -0x688c(r21)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 83083D48: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83083D4C: 419A00E8  beq cr6, 0x83083e34
	if ctx.cr[6].eq {
	pc = 0x83083E34; continue 'dispatch;
	}
	// 83083D50: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083D54: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83083D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83083D5C: 409900BC  ble cr6, 0x83083e18
	if !ctx.cr[6].gt {
	pc = 0x83083E18; continue 'dispatch;
	}
	// 83083D60: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83083D64: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83083D68: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083D6C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83083D70: 7CBF482E  lwzx r5, r31, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83083D74: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83083D78: 4082002C  bne 0x83083da4
	if !ctx.cr[0].eq {
	pc = 0x83083DA4; continue 'dispatch;
	}
	// 83083D7C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83083D80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83083D84: 4BFF9F45  bl 0x8307dcc8
	ctx.lr = 0x83083D88;
	sub_8307DCC8(ctx, base);
	// 83083D88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83083D8C: 41820074  beq 0x83083e00
	if ctx.cr[0].eq {
	pc = 0x83083E00; continue 'dispatch;
	}
	// 83083D90: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83083D94: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83083D98: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083D9C: 81599754  lwz r10, -0x68ac(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083DA0: 48000034  b 0x83083dd4
	pc = 0x83083DD4; continue 'dispatch;
	// 83083DA4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83083DA8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83083DAC: 419A001C  beq cr6, 0x83083dc8
	if ctx.cr[6].eq {
	pc = 0x83083DC8; continue 'dispatch;
	}
	// 83083DB0: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83083DB4: 409A0030  bne cr6, 0x83083de4
	if !ctx.cr[6].eq {
	pc = 0x83083DE4; continue 'dispatch;
	}
	// 83083DB8: 81650020  lwz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083DBC: 813A0020  lwz r9, 0x20(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083DC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83083DC4: 409A0040  bne cr6, 0x83083e04
	if !ctx.cr[6].eq {
	pc = 0x83083E04; continue 'dispatch;
	}
	// 83083DC8: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83083DCC: 5789103A  slwi r9, r28, 2
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83083DD0: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083DD4: 7FCBF82E  lwzx r30, r11, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83083DD8: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083DDC: 409A0044  bne cr6, 0x83083e20
	if !ctx.cr[6].eq {
	pc = 0x83083E20; continue 'dispatch;
	}
	// 83083DE0: 48000024  b 0x83083e04
	pc = 0x83083E04; continue 'dispatch;
	// 83083DE4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83083DE8: 409A001C  bne cr6, 0x83083e04
	if !ctx.cr[6].eq {
	pc = 0x83083E04; continue 'dispatch;
	}
	// 83083DEC: 81650020  lwz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083DF0: 813A0020  lwz r9, 0x20(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083DF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83083DF8: 419A000C  beq cr6, 0x83083e04
	if ctx.cr[6].eq {
	pc = 0x83083E04; continue 'dispatch;
	}
	// 83083DFC: 4BFFFFCC  b 0x83083dc8
	pc = 0x83083DC8; continue 'dispatch;
	// 83083E00: 81599754  lwz r10, -0x68ac(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083E04: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083E08: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83083E0C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83083E10: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083E14: 4198FF50  blt cr6, 0x83083d64
	if ctx.cr[6].lt {
	pc = 0x83083D64; continue 'dispatch;
	}
	// 83083E18: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83083E1C: 419A0048  beq cr6, 0x83083e64
	if ctx.cr[6].eq {
	pc = 0x83083E64; continue 'dispatch;
	}
	// 83083E20: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083E24: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083E28: 419A003C  beq cr6, 0x83083e64
	if ctx.cr[6].eq {
	pc = 0x83083E64; continue 'dispatch;
	}
	// 83083E2C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 83083E30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83083E34: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 83083E38: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 83083E3C: 7F16C040  cmplw cr6, r22, r24
	ctx.cr[6].compare_u32(ctx.r[22].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83083E40: 4198FEF0  blt cr6, 0x83083d30
	if ctx.cr[6].lt {
	pc = 0x83083D30; continue 'dispatch;
	}
	// 83083E44: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83083E48: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83083E4C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83083E50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083E54: 41820008  beq 0x83083e5c
	if ctx.cr[0].eq {
	pc = 0x83083E5C; continue 'dispatch;
	}
	// 83083E58: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83083E5C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83083E60: 4812433C  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 83083E64: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83083E68: 4BFFFFF4  b 0x83083e5c
	pc = 0x83083E5C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083E70 size=120
    let mut pc: u32 = 0x83083E70;
    'dispatch: loop {
        match pc {
            0x83083E70 => {
    //   block [0x83083E70..0x83083EE8)
	// 83083E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83083E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83083E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83083E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083E80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83083E84: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83083E88: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083E8C: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83083E90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083E94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083E98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83083E9C: 4E800421  bctrl
	ctx.lr = 0x83083EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83083EA0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83083EA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83083EAC: 40990028  ble cr6, 0x83083ed4
	if !ctx.cr[6].gt {
	pc = 0x83083ED4; continue 'dispatch;
	}
	// 83083EB0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83083EB4: 3D008217  lis r8, -0x7de9
	ctx.r[8].s64 = -2112421888;
	// 83083EB8: 81289754  lwz r9, -0x68ac(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83083EBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83083EC0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83083EC4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83083EC8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083ECC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83083ED0: 4198FFE8  blt cr6, 0x83083eb8
	if ctx.cr[6].lt {
	pc = 0x83083EB8; continue 'dispatch;
	}
	// 83083ED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83083ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83083EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83083EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83083EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83083EE8 size=8
    let mut pc: u32 = 0x83083EE8;
    'dispatch: loop {
        match pc {
            0x83083EE8 => {
    //   block [0x83083EE8..0x83083EF0)
	// 83083EE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83083EEC: 8216A510  lwz r16, -0x5af0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83083EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83083EF0 size=1072
    let mut pc: u32 = 0x83083EF0;
    'dispatch: loop {
        match pc {
            0x83083EF0 => {
    //   block [0x83083EF0..0x83084320)
	// 83083EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83083EF4: 48124241  bl 0x831a8134
	ctx.lr = 0x83083EF8;
	sub_831A8130(ctx, base);
	// 83083EF8: 3BE1FED0  addi r31, r1, -0x130
	ctx.r[31].s64 = ctx.r[1].s64 + -304;
	// 83083EFC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83083F00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83083F04: 90BF0050  stw r5, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83083F08: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 83083F0C: 90DF0054  stw r6, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83083F10: 7CF13B78  mr r17, r7
	ctx.r[17].u64 = ctx.r[7].u64;
	// 83083F14: 7C8F2378  mr r15, r4
	ctx.r[15].u64 = ctx.r[4].u64;
	// 83083F18: 7D304B78  mr r16, r9
	ctx.r[16].u64 = ctx.r[9].u64;
	// 83083F1C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083F20: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 83083F24: 3E408217  lis r18, -0x7de9
	ctx.r[18].s64 = -2112421888;
	// 83083F28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83083F2C: 40990070  ble cr6, 0x83083f9c
	if !ctx.cr[6].gt {
	pc = 0x83083F9C; continue 'dispatch;
	}
	// 83083F30: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 83083F34: 3CA08217  lis r5, -0x7de9
	ctx.r[5].s64 = -2112421888;
	// 83083F38: 3C808217  lis r4, -0x7de9
	ctx.r[4].s64 = -2112421888;
	// 83083F3C: 3CC08217  lis r6, -0x7de9
	ctx.r[6].s64 = -2112421888;
	// 83083F40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083F44: 80669758  lwz r3, -0x68a8(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26792 as u32) ) } as u64;
	// 83083F48: 7D49582E  lwzx r10, r9, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83083F4C: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83083F50: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83083F54: 419A0034  beq cr6, 0x83083f88
	if ctx.cr[6].eq {
	pc = 0x83083F88; continue 'dispatch;
	}
	// 83083F58: 8064975C  lwz r3, -0x68a4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-26788 as u32) ) } as u64;
	// 83083F5C: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83083F60: 419A0028  beq cr6, 0x83083f88
	if ctx.cr[6].eq {
	pc = 0x83083F88; continue 'dispatch;
	}
	// 83083F64: 80659770  lwz r3, -0x6890(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83083F68: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83083F6C: 419A001C  beq cr6, 0x83083f88
	if ctx.cr[6].eq {
	pc = 0x83083F88; continue 'dispatch;
	}
	// 83083F70: 80729774  lwz r3, -0x688c(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 83083F74: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83083F78: 419A0010  beq cr6, 0x83083f88
	if ctx.cr[6].eq {
	pc = 0x83083F88; continue 'dispatch;
	}
	// 83083F7C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83083F80: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83083F84: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83083F88: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083F8C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 83083F90: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83083F94: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83083F98: 4198FFA8  blt cr6, 0x83083f40
	if ctx.cr[6].lt {
	pc = 0x83083F40; continue 'dispatch;
	}
	// 83083F9C: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83083FA0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083FA4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83083FA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083FAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083FB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83083FB4: 4E800421  bctrl
	ctx.lr = 0x83083FB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83083FB8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083FBC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83083FC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83083FC4: 3F008217  lis r24, -0x7de9
	ctx.r[24].s64 = -2112421888;
	// 83083FC8: 41820078  beq 0x83084040
	if ctx.cr[0].eq {
	pc = 0x83084040; continue 'dispatch;
	}
	// 83083FCC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83083FD0: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 83083FD4: 23770004  subfic r27, r23, 4
	ctx.xer.ca = ctx.r[23].u32 <= 4 as u32;
	ctx.r[27].s64 = (4 as i64) - ctx.r[23].s64;
	// 83083FD8: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83083FDC: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83083FE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83083FE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83083FE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83083FEC: 4E800421  bctrl
	ctx.lr = 0x83083FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83083FF0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83083FF4: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83083FF8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83083FFC: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83084000: 40980028  bge cr6, 0x83084028
	if !ctx.cr[6].lt {
	pc = 0x83084028; continue 'dispatch;
	}
	// 83084004: 7D5BEA14  add r10, r27, r29
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 83084008: 81389754  lwz r9, -0x68ac(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 8308400C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83084010: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084014: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 83084018: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8308401C: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084020: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83084024: 4198FFE4  blt cr6, 0x83084008
	if ctx.cr[6].lt {
	pc = 0x83084008; continue 'dispatch;
	}
	// 83084028: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8308402C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084030: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83084034: 395CFFFF  addi r10, r28, -1
	ctx.r[10].s64 = ctx.r[28].s64 + -1;
	// 83084038: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8308403C: 4198FF9C  blt cr6, 0x83083fd8
	if ctx.cr[6].lt {
	pc = 0x83083FD8; continue 'dispatch;
	}
	// 83084040: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83084044: 7ED3B378  mr r19, r22
	ctx.r[19].u64 = ctx.r[22].u64;
	// 83084048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8308404C: 40990274  ble cr6, 0x830842c0
	if !ctx.cr[6].gt {
	pc = 0x830842C0; continue 'dispatch;
	}
	// 83084050: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83084054: 81389754  lwz r9, -0x68ac(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83084058: 3AABD35C  addi r21, r11, -0x2ca4
	ctx.r[21].s64 = ctx.r[11].s64 + -11428;
	// 8308405C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83084060: 3A8BD340  addi r20, r11, -0x2cc0
	ctx.r[20].s64 = ctx.r[11].s64 + -11456;
	// 83084064: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084068: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308406C: 41820244  beq 0x830842b0
	if ctx.cr[0].eq {
	pc = 0x830842B0; continue 'dispatch;
	}
	// 83084070: 7EDDB378  mr r29, r22
	ctx.r[29].u64 = ctx.r[22].u64;
	// 83084074: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83084078: 7F79DB78  mr r25, r27
	ctx.r[25].u64 = ctx.r[27].u64;
	// 8308407C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83084080: 40980218  bge cr6, 0x83084298
	if !ctx.cr[6].lt {
	pc = 0x83084298; continue 'dispatch;
	}
	// 83084084: 567A103A  slwi r26, r19, 2
	ctx.r[26].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83084088: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 8308408C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83084090: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83084094: 7D5D582E  lwzx r10, r29, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084098: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8308409C: 419A01E8  beq cr6, 0x83084284
	if ctx.cr[6].eq {
	pc = 0x83084284; continue 'dispatch;
	}
	// 830840A0: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830840A4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830840A8: 419A01DC  beq cr6, 0x83084284
	if ctx.cr[6].eq {
	pc = 0x83084284; continue 'dispatch;
	}
	// 830840AC: 7D7DB82E  lwzx r11, r29, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 830840B0: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830840B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830840B8: 409A01CC  bne cr6, 0x83084284
	if !ctx.cr[6].eq {
	pc = 0x83084284; continue 'dispatch;
	}
	// 830840BC: 897E0039  lbz r11, 0x39(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(57 as u32) ) } as u64;
	// 830840C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830840C4: 41820030  beq 0x830840f4
	if ctx.cr[0].eq {
	pc = 0x830840F4; continue 'dispatch;
	}
	// 830840C8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830840CC: 81129774  lwz r8, -0x688c(r18)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 830840D0: 7D5D582E  lwzx r10, r29, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830840D4: 814A0020  lwz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 830840D8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830840DC: 419A01A8  beq cr6, 0x83084284
	if ctx.cr[6].eq {
	pc = 0x83084284; continue 'dispatch;
	}
	// 830840E0: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830840E4: 550A003E  slwi r10, r8, 0
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830840E8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830840EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830840F0: 419A0194  beq cr6, 0x83084284
	if ctx.cr[6].eq {
	pc = 0x83084284; continue 'dispatch;
	}
	// 830840F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830840F8: 391F0050  addi r8, r31, 0x50
	ctx.r[8].s64 = ctx.r[31].s64 + 80;
	// 830840FC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084100: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 83084104: 7CFC582E  lwzx r7, r28, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084108: 7CDC502E  lwzx r6, r28, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8308410C: 7CBD582E  lwzx r5, r29, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084110: 7C9D502E  lwzx r4, r29, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83084114: 4801DDAD  bl 0x830a1ec0
	ctx.lr = 0x83084118;
	sub_830A1EC0(ctx, base);
	// 83084118: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308411C: 7D7DB82E  lwzx r11, r29, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83084120: 4182015C  beq 0x8308427c
	if ctx.cr[0].eq {
	pc = 0x8308427C; continue 'dispatch;
	}
	// 83084124: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83084128: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 8308412C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83084130: 7D5C592E  stwx r10, r28, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 83084134: 80BE0040  lwz r5, 0x40(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084138: 4BF5AD21  bl 0x82fdee58
	ctx.lr = 0x8308413C;
	sub_82FDEE58(ctx, base);
	// 8308413C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084140: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084144: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83084148: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8308414C: 419A003C  beq cr6, 0x83084188
	if ctx.cr[6].eq {
	pc = 0x83084188; continue 'dispatch;
	}
	// 83084150: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83084154: 419A0034  beq cr6, 0x83084188
	if ctx.cr[6].eq {
	pc = 0x83084188; continue 'dispatch;
	}
	// 83084158: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 8308415C: 409A0018  bne cr6, 0x83084174
	if !ctx.cr[6].eq {
	pc = 0x83084174; continue 'dispatch;
	}
	// 83084160: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83084164: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 83084168: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8308416C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83084170: 48000028  b 0x83084198
	pc = 0x83084198; continue 'dispatch;
	// 83084174: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084178: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8308417C: 4BF5A1ED  bl 0x82fde368
	ctx.lr = 0x83084180;
	sub_82FDE368(ctx, base);
	// 83084180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83084184: 48000008  b 0x8308418c
	pc = 0x8308418C; continue 'dispatch;
	// 83084188: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8308418C: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 83084190: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83084194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83084198: 4BF553D9  bl 0x82fd9570
	ctx.lr = 0x8308419C;
	sub_82FD9570(ctx, base);
	// 8308419C: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 830841A0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830841A4: 80BE0040  lwz r5, 0x40(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 830841A8: 4BF5ACB1  bl 0x82fdee58
	ctx.lr = 0x830841AC;
	sub_82FDEE58(ctx, base);
	// 830841AC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830841B0: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830841B4: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830841B8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830841BC: 419A003C  beq cr6, 0x830841f8
	if ctx.cr[6].eq {
	pc = 0x830841F8; continue 'dispatch;
	}
	// 830841C0: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830841C4: 419A0034  beq cr6, 0x830841f8
	if ctx.cr[6].eq {
	pc = 0x830841F8; continue 'dispatch;
	}
	// 830841C8: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830841CC: 409A0018  bne cr6, 0x830841e4
	if !ctx.cr[6].eq {
	pc = 0x830841E4; continue 'dispatch;
	}
	// 830841D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830841D4: 92DF0064  stw r22, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 830841D8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 830841DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830841E0: 48000028  b 0x83084208
	pc = 0x83084208; continue 'dispatch;
	// 830841E4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830841E8: 7C7C582E  lwzx r3, r28, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830841EC: 4BF5A17D  bl 0x82fde368
	ctx.lr = 0x830841F0;
	sub_82FDE368(ctx, base);
	// 830841F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830841F4: 48000008  b 0x830841fc
	pc = 0x830841FC; continue 'dispatch;
	// 830841F8: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 830841FC: 92DF0064  stw r22, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 83084200: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83084204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83084208: 4BF55369  bl 0x82fd9570
	ctx.lr = 0x8308420C;
	sub_82FD9570(ctx, base);
	// 8308420C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83084210: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83084214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83084218: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308421C: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 83084220: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 83084224: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 83084228: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 8308422C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83084230: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83084234: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83084238: 80FF0078  lwz r7, 0x78(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8308423C: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 83084240: 80DF0098  lwz r6, 0x98(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83084244: 4BF93315  bl 0x83017558
	ctx.lr = 0x83084248;
	sub_83017558(ctx, base);
	// 83084248: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8308424C: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83084250: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084254: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308425C: 4E800421  bctrl
	ctx.lr = 0x83084260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084260: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83084264: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83084268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308426C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084274: 4E800421  bctrl
	ctx.lr = 0x83084278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084278: 48000008  b 0x83084280
	pc = 0x83084280; continue 'dispatch;
	// 8308427C: 7EDC592E  stwx r22, r28, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32), ctx.r[22].u32) };
	// 83084280: 81389754  lwz r9, -0x68ac(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83084284: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084288: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8308428C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83084290: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83084294: 4198FDF8  blt cr6, 0x8308408c
	if ctx.cr[6].lt {
	pc = 0x8308408C; continue 'dispatch;
	}
	// 83084298: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8308429C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830842A0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830842A4: 395BFFFF  addi r10, r27, -1
	ctx.r[10].s64 = ctx.r[27].s64 + -1;
	// 830842A8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830842AC: 4198FDCC  blt cr6, 0x83084078
	if ctx.cr[6].lt {
	pc = 0x83084078; continue 'dispatch;
	}
	// 830842B0: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830842B4: 3A730001  addi r19, r19, 1
	ctx.r[19].s64 = ctx.r[19].s64 + 1;
	// 830842B8: 7F135840  cmplw cr6, r19, r11
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830842BC: 4198FDA8  blt cr6, 0x83084064
	if ctx.cr[6].lt {
	pc = 0x83084064; continue 'dispatch;
	}
	// 830842C0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830842C4: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 830842C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830842CC: 40990034  ble cr6, 0x83084300
	if !ctx.cr[6].gt {
	pc = 0x83084300; continue 'dispatch;
	}
	// 830842D0: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 830842D4: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 830842D8: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830842DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830842E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830842E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830842E8: 4E800421  bctrl
	ctx.lr = 0x830842EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830842EC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830842F0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830842F4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830842F8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830842FC: 4198FFD8  blt cr6, 0x830842d4
	if ctx.cr[6].lt {
	pc = 0x830842D4; continue 'dispatch;
	}
	// 83084300: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084304: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83084308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308430C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084314: 4E800421  bctrl
	ctx.lr = 0x83084318;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084318: 383F0130  addi r1, r31, 0x130
	ctx.r[1].s64 = ctx.r[31].s64 + 304;
	// 8308431C: 48123E68  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084320 size=40
    let mut pc: u32 = 0x83084320;
    'dispatch: loop {
        match pc {
            0x83084320 => {
    //   block [0x83084320..0x83084348)
	// 83084320: 3BECFED0  addi r31, r12, -0x130
	ctx.r[31].s64 = ctx.r[12].s64 + -304;
	// 83084324: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084328: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308432C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084330: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83084334: 4BF5ABA5  bl 0x82fdeed8
	ctx.lr = 0x83084338;
	sub_82FDEED8(ctx, base);
	// 83084338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308433C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084348 size=40
    let mut pc: u32 = 0x83084348;
    'dispatch: loop {
        match pc {
            0x83084348 => {
    //   block [0x83084348..0x83084370)
	// 83084348: 3BECFED0  addi r31, r12, -0x130
	ctx.r[31].s64 = ctx.r[12].s64 + -304;
	// 8308434C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084350: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084354: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084358: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8308435C: 4BF5AB7D  bl 0x82fdeed8
	ctx.lr = 0x83084360;
	sub_82FDEED8(ctx, base);
	// 83084360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308436C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084370 size=148
    let mut pc: u32 = 0x83084370;
    'dispatch: loop {
        match pc {
            0x83084370 => {
    //   block [0x83084370..0x83084404)
	// 83084370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084374: 48123DED  bl 0x831a8160
	ctx.lr = 0x83084378;
	sub_831A8130(ctx, base);
	// 83084378: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308437C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83084380: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83084384: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 83084388: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308438C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084390: 40990068  ble cr6, 0x830843f8
	if !ctx.cr[6].gt {
	pc = 0x830843F8; continue 'dispatch;
	}
	// 83084394: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 83084398: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308439C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830843A0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830843A4: 41820038  beq 0x830843dc
	if ctx.cr[0].eq {
	pc = 0x830843DC; continue 'dispatch;
	}
	// 830843A8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830843AC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830843B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830843B4: 41820014  beq 0x830843c8
	if ctx.cr[0].eq {
	pc = 0x830843C8; continue 'dispatch;
	}
	// 830843B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830843BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830843C0: 41820008  beq 0x830843c8
	if ctx.cr[0].eq {
	pc = 0x830843C8; continue 'dispatch;
	}
	// 830843C4: 4BF53F1D  bl 0x82fd82e0
	ctx.lr = 0x830843C8;
	sub_82FD82E0(ctx, base);
	// 830843C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830843CC: 4BF53F15  bl 0x82fd82e0
	ctx.lr = 0x830843D0;
	sub_82FD82E0(ctx, base);
	// 830843D0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830843D4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830843D8: 409AFFD0  bne cr6, 0x830843a8
	if !ctx.cr[6].eq {
	pc = 0x830843A8; continue 'dispatch;
	}
	// 830843DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830843E0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 830843E4: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 830843E8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830843EC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830843F0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830843F4: 4198FFA4  blt cr6, 0x83084398
	if ctx.cr[6].lt {
	pc = 0x83084398; continue 'dispatch;
	}
	// 830843F8: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 830843FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83084400: 48123DB0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084408 size=192
    let mut pc: u32 = 0x83084408;
    'dispatch: loop {
        match pc {
            0x83084408 => {
    //   block [0x83084408..0x830844C8)
	// 83084408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83084414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83084418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308441C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83084420: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83084424: 2B040040  cmplwi cr6, r4, 0x40
	ctx.cr[6].compare_u32(ctx.r[4].u32, 64 as u32, &mut ctx.xer);
	// 83084428: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8308442C: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 83084430: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83084434: 40990038  ble cr6, 0x8308446c
	if !ctx.cr[6].gt {
	pc = 0x8308446C; continue 'dispatch;
	}
	// 83084438: 548BE8FE  srwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308443C: 548A077F  clrlwi. r10, r4, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83084440: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83084444: 4182000C  beq 0x83084450
	if ctx.cr[0].eq {
	pc = 0x83084450; continue 'dispatch;
	}
	// 83084448: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308444C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83084450: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084454: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83084458: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308445C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084464: 4E800421  bctrl
	ctx.lr = 0x83084468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084468: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8308446C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084470: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83084474: 40980010  bge cr6, 0x83084484
	if !ctx.cr[6].lt {
	pc = 0x83084484; continue 'dispatch;
	}
	// 83084478: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8308447C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83084480: 4800002C  b 0x830844ac
	pc = 0x830844AC; continue 'dispatch;
	// 83084484: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084488: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8308448C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83084490: 4099001C  ble cr6, 0x830844ac
	if !ctx.cr[6].gt {
	pc = 0x830844AC; continue 'dispatch;
	}
	// 83084494: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084498: 7FCA59AE  stbx r30, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u8) };
	// 8308449C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830844A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830844A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830844A8: 4198FFEC  blt cr6, 0x83084494
	if ctx.cr[6].lt {
	pc = 0x83084494; continue 'dispatch;
	}
	// 830844AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830844B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830844B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830844B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830844BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830844C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830844C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830844C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830844C8 size=92
    let mut pc: u32 = 0x830844C8;
    'dispatch: loop {
        match pc {
            0x830844C8 => {
    //   block [0x830844C8..0x83084524)
	// 830844C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830844CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830844D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830844D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830844D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830844DC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830844E0: 396BA4E0  addi r11, r11, -0x5b20
	ctx.r[11].s64 = ctx.r[11].s64 + -23328;
	// 830844E4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830844E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830844EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830844F0: 4182000C  beq 0x830844fc
	if ctx.cr[0].eq {
	pc = 0x830844FC; continue 'dispatch;
	}
	// 830844F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830844F8: 4BFFF481  bl 0x83083978
	ctx.lr = 0x830844FC;
	sub_83083978(ctx, base);
	// 830844FC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084500: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084504: 4182000C  beq 0x83084510
	if ctx.cr[0].eq {
	pc = 0x83084510; continue 'dispatch;
	}
	// 83084508: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8308450C: 4BFFF46D  bl 0x83083978
	ctx.lr = 0x83084510;
	sub_83083978(ctx, base);
	// 83084510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308451C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83084520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83084528 size=8
    let mut pc: u32 = 0x83084528;
    'dispatch: loop {
        match pc {
            0x83084528 => {
    //   block [0x83084528..0x83084530)
	// 83084528: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308452C: 8216A568  lwz r16, -0x5a98(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084530 size=144
    let mut pc: u32 = 0x83084530;
    'dispatch: loop {
        match pc {
            0x83084530 => {
    //   block [0x83084530..0x830845C0)
	// 83084530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308453C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83084540: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83084544: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084548: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308454C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084550: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83084554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084558: 409A004C  bne cr6, 0x830845a4
	if !ctx.cr[6].eq {
	pc = 0x830845A4; continue 'dispatch;
	}
	// 8308455C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83084560: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084564: 4BF53D35  bl 0x82fd8298
	ctx.lr = 0x83084568;
	sub_82FD8298(ctx, base);
	// 83084568: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8308456C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084570: 41820018  beq 0x83084588
	if ctx.cr[0].eq {
	pc = 0x83084588; continue 'dispatch;
	}
	// 83084574: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084578: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308457C: 4BFFFE8D  bl 0x83084408
	ctx.lr = 0x83084580;
	sub_83084408(ctx, base);
	// 83084580: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83084584: 48000008  b 0x8308458c
	pc = 0x8308458C; continue 'dispatch;
	// 83084588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8308458C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83084594: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 83084598: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308459C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830845A0: 4E800421  bctrl
	ctx.lr = 0x830845A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830845A4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830845A8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830845AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830845B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830845B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830845B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830845BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830845C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830845C0 size=48
    let mut pc: u32 = 0x830845C0;
    'dispatch: loop {
        match pc {
            0x830845C0 => {
    //   block [0x830845C0..0x830845F0)
	// 830845C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830845C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830845C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830845CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830845D0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830845D4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830845D8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830845DC: 4BF53D05  bl 0x82fd82e0
	ctx.lr = 0x830845E0;
	sub_82FD82E0(ctx, base);
	// 830845E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830845E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830845E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830845EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830845F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830845F0 size=8
    let mut pc: u32 = 0x830845F0;
    'dispatch: loop {
        match pc {
            0x830845F0 => {
    //   block [0x830845F0..0x830845F8)
	// 830845F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830845F4: 8216A5A8  lwz r16, -0x5a58(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23128 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830845F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830845F8 size=144
    let mut pc: u32 = 0x830845F8;
    'dispatch: loop {
        match pc {
            0x830845F8 => {
    //   block [0x830845F8..0x83084688)
	// 830845F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830845FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83084604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83084608: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8308460C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084610: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83084614: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084618: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8308461C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084620: 409A004C  bne cr6, 0x8308466c
	if !ctx.cr[6].eq {
	pc = 0x8308466C; continue 'dispatch;
	}
	// 83084624: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83084628: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308462C: 4BF53C6D  bl 0x82fd8298
	ctx.lr = 0x83084630;
	sub_82FD8298(ctx, base);
	// 83084630: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084634: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084638: 41820018  beq 0x83084650
	if ctx.cr[0].eq {
	pc = 0x83084650; continue 'dispatch;
	}
	// 8308463C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084640: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83084644: 4BFFFDC5  bl 0x83084408
	ctx.lr = 0x83084648;
	sub_83084408(ctx, base);
	// 83084648: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8308464C: 48000008  b 0x83084654
	pc = 0x83084654; continue 'dispatch;
	// 83084650: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83084654: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308465C: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 83084660: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084668: 4E800421  bctrl
	ctx.lr = 0x8308466C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308466C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084670: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83084674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308467C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83084680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83084684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084688 size=48
    let mut pc: u32 = 0x83084688;
    'dispatch: loop {
        match pc {
            0x83084688 => {
    //   block [0x83084688..0x830846B8)
	// 83084688: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308468C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084690: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084698: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8308469C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830846A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830846A4: 4BF53C3D  bl 0x82fd82e0
	ctx.lr = 0x830846A8;
	sub_82FD82E0(ctx, base);
	// 830846A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830846AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830846B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830846B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830846B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830846B8 size=8
    let mut pc: u32 = 0x830846B8;
    'dispatch: loop {
        match pc {
            0x830846B8 => {
    //   block [0x830846B8..0x830846C0)
	// 830846B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830846BC: 8216A600  lwz r16, -0x5a00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-23040 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830846C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830846C0 size=180
    let mut pc: u32 = 0x830846C0;
    'dispatch: loop {
        match pc {
            0x830846C0 => {
    //   block [0x830846C0..0x83084774)
	// 830846C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830846C4: 48123AA9  bl 0x831a816c
	ctx.lr = 0x830846C8;
	sub_831A8130(ctx, base);
	// 830846C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830846CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830846D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830846D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830846D8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830846DC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830846E0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830846E4: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830846E8: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830846EC: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830846F0: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830846F4: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830846F8: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830846FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084700: 90BE001C  stw r5, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83084704: 394AA5E0  addi r10, r10, -0x5a20
	ctx.r[10].s64 = ctx.r[10].s64 + -23072;
	// 83084708: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8308470C: 9BBE0020  stb r29, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 83084710: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83084714: 409A0050  bne cr6, 0x83084764
	if !ctx.cr[6].eq {
	pc = 0x83084764; continue 'dispatch;
	}
	// 83084718: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8308471C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83084720: 4BF53B79  bl 0x82fd8298
	ctx.lr = 0x83084724;
	sub_82FD8298(ctx, base);
	// 83084724: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084728: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308472C: 41820024  beq 0x83084750
	if ctx.cr[0].eq {
	pc = 0x83084750; continue 'dispatch;
	}
	// 83084730: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83084734: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084738: 38AB8158  addi r5, r11, -0x7ea8
	ctx.r[5].s64 = ctx.r[11].s64 + -32424;
	// 8308473C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83084740: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83084744: 80CB9770  lwz r6, -0x6890(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83084748: 4BF5A4E1  bl 0x82fdec28
	ctx.lr = 0x8308474C;
	sub_82FDEC28(ctx, base);
	// 8308474C: 48000008  b 0x83084754
	pc = 0x83084754; continue 'dispatch;
	// 83084750: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83084754: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83084758: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8308475C: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 83084760: 48000008  b 0x83084768
	pc = 0x83084768; continue 'dispatch;
	// 83084764: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83084768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308476C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83084770: 48123A4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084774 size=40
    let mut pc: u32 = 0x83084774;
    'dispatch: loop {
        match pc {
            0x83084774 => {
    //   block [0x83084774..0x8308479C)
	// 83084774: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83084778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308477C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084784: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83084788: 4BFFFD41  bl 0x830844c8
	ctx.lr = 0x8308478C;
	sub_830844C8(ctx, base);
	// 8308478C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308479C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308479C size=48
    let mut pc: u32 = 0x8308479C;
    'dispatch: loop {
        match pc {
            0x8308479C => {
    //   block [0x8308479C..0x830847CC)
	// 8308479C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830847A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830847A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830847A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830847AC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830847B0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830847B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830847B8: 4BF53B29  bl 0x82fd82e0
	ctx.lr = 0x830847BC;
	sub_82FD82E0(ctx, base);
	// 830847BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830847C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830847C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830847C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830847D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830847D0 size=204
    let mut pc: u32 = 0x830847D0;
    'dispatch: loop {
        match pc {
            0x830847D0 => {
    //   block [0x830847D0..0x8308489C)
	// 830847D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830847D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830847D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830847DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830847E0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830847E4: 41980030  blt cr6, 0x83084814
	if ctx.cr[6].lt {
	pc = 0x83084814; continue 'dispatch;
	}
	// 830847E8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830847EC: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830847F0: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 830847F4: 388BA4A4  addi r4, r11, -0x5b5c
	ctx.r[4].s64 = ctx.r[11].s64 + -23388;
	// 830847F8: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 830847FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83084800: 4BF4C159  bl 0x82fd0958
	ctx.lr = 0x83084804;
	sub_82FD0958(ctx, base);
	// 83084804: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83084808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308480C: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83084810: 4812C419  bl 0x831b0c28
	ctx.lr = 0x83084814;
	sub_831B0C28(ctx, base);
	// 83084814: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83084818: 40980040  bge cr6, 0x83084858
	if !ctx.cr[6].lt {
	pc = 0x83084858; continue 'dispatch;
	}
	// 8308481C: 548B06FE  clrlwi r11, r4, 0x1b
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 83084820: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83084824: 2B040020  cmplwi cr6, r4, 0x20
	ctx.cr[6].compare_u32(ctx.r[4].u32, 32 as u32, &mut ctx.xer);
	// 83084828: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8308482C: 40980018  bge cr6, 0x83084844
	if !ctx.cr[6].lt {
	pc = 0x83084844; continue 'dispatch;
	}
	// 83084830: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084834: 7D4A5878  andc r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[11].u64;
	// 83084838: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8308483C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83084840: 4800004C  b 0x8308488c
	pc = 0x8308488C; continue 'dispatch;
	// 83084844: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084848: 7D4A5878  andc r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[11].u64;
	// 8308484C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83084850: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83084854: 48000038  b 0x8308488c
	pc = 0x8308488C; continue 'dispatch;
	// 83084858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8308485C: 548A077E  clrlwi r10, r4, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000007u64;
	// 83084860: 548BE8FE  srwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83084864: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 83084868: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308486C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83084870: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084874: 7D085078  andc r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 & !ctx.r[10].u64;
	// 83084878: 7D0959AE  stbx r8, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 8308487C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084880: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084884: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 83084888: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 8308488C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83084890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830848A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830848A0 size=44
    let mut pc: u32 = 0x830848A0;
    'dispatch: loop {
        match pc {
            0x830848A0 => {
    //   block [0x830848A0..0x830848CC)
	// 830848A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830848A4: 8083001C  lwz r4, 0x1c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830848A8: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 830848AC: 409A004C  bne cr6, 0x830848f8
	if !ctx.cr[6].eq {
		sub_830848F8(ctx, base);
		return;
	}
	// 830848B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830848B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830848B8: 2B0A0041  cmplwi cr6, r10, 0x41
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65 as u32, &mut ctx.xer);
	// 830848BC: 40980010  bge cr6, 0x830848cc
	if !ctx.cr[6].lt {
		sub_830848CC(ctx, base);
		return;
	}
	// 830848C0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830848C4: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830848C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830848CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830848CC size=16
    let mut pc: u32 = 0x830848CC;
    'dispatch: loop {
        match pc {
            0x830848CC => {
    //   block [0x830848CC..0x830848DC)
	// 830848CC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830848D0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 830848D4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830848D8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830848DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830848DC size=28
    let mut pc: u32 = 0x830848DC;
    'dispatch: loop {
        match pc {
            0x830848DC => {
    //   block [0x830848DC..0x830848F8)
	// 830848DC: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830848E0: 7D2851AE  stbx r9, r8, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 830848E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830848E8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830848EC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830848F0: 4198FFEC  blt cr6, 0x830848dc
	if ctx.cr[6].lt {
	pc = 0x830848DC; continue 'dispatch;
	}
	// 830848F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830848F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830848F8 size=8
    let mut pc: u32 = 0x830848F8;
    'dispatch: loop {
        match pc {
            0x830848F8 => {
    //   block [0x830848F8..0x83084900)
	// 830848F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830848FC: 4BFFFED4  b 0x830847d0
	sub_830847D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83084900 size=8
    let mut pc: u32 = 0x83084900;
    'dispatch: loop {
        match pc {
            0x83084900 => {
    //   block [0x83084900..0x83084908)
	// 83084900: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83084904: 8216A650  lwz r16, -0x59b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-22960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084908 size=180
    let mut pc: u32 = 0x83084908;
    'dispatch: loop {
        match pc {
            0x83084908 => {
    //   block [0x83084908..0x830849BC)
	// 83084908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308490C: 48123861  bl 0x831a816c
	ctx.lr = 0x83084910;
	sub_831A8130(ctx, base);
	// 83084910: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83084914: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084918: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308491C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83084920: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83084924: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83084928: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8308492C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83084930: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83084934: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83084938: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8308493C: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83084940: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83084944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084948: 90BE001C  stw r5, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8308494C: 394AA5E0  addi r10, r10, -0x5a20
	ctx.r[10].s64 = ctx.r[10].s64 + -23072;
	// 83084950: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83084954: 98DE0020  stb r6, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[6].u8 ) };
	// 83084958: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8308495C: 409A0050  bne cr6, 0x830849ac
	if !ctx.cr[6].eq {
	pc = 0x830849AC; continue 'dispatch;
	}
	// 83084960: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83084964: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83084968: 4BF53931  bl 0x82fd8298
	ctx.lr = 0x8308496C;
	sub_82FD8298(ctx, base);
	// 8308496C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084970: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084974: 41820024  beq 0x83084998
	if ctx.cr[0].eq {
	pc = 0x83084998; continue 'dispatch;
	}
	// 83084978: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8308497C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084980: 38AB8158  addi r5, r11, -0x7ea8
	ctx.r[5].s64 = ctx.r[11].s64 + -32424;
	// 83084984: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83084988: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8308498C: 80CB9770  lwz r6, -0x6890(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83084990: 4BF5A299  bl 0x82fdec28
	ctx.lr = 0x83084994;
	sub_82FDEC28(ctx, base);
	// 83084994: 48000008  b 0x8308499c
	pc = 0x8308499C; continue 'dispatch;
	// 83084998: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8308499C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830849A0: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830849A4: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 830849A8: 48000008  b 0x830849b0
	pc = 0x830849B0; continue 'dispatch;
	// 830849AC: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830849B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830849B4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830849B8: 48123804  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830849BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830849BC size=40
    let mut pc: u32 = 0x830849BC;
    'dispatch: loop {
        match pc {
            0x830849BC => {
    //   block [0x830849BC..0x830849E4)
	// 830849BC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830849C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830849C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830849C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830849CC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830849D0: 4BFFFAF9  bl 0x830844c8
	ctx.lr = 0x830849D4;
	sub_830844C8(ctx, base);
	// 830849D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830849D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830849DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830849E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830849E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830849E4 size=48
    let mut pc: u32 = 0x830849E4;
    'dispatch: loop {
        match pc {
            0x830849E4 => {
    //   block [0x830849E4..0x83084A14)
	// 830849E4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830849E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830849EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830849F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830849F4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830849F8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830849FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83084A00: 4BF538E1  bl 0x82fd82e0
	ctx.lr = 0x83084A04;
	sub_82FD82E0(ctx, base);
	// 83084A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83084A18 size=8
    let mut pc: u32 = 0x83084A18;
    'dispatch: loop {
        match pc {
            0x83084A18 => {
    //   block [0x83084A18..0x83084A20)
	// 83084A18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83084A1C: 8216A698  lwz r16, -0x5968(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-22888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084A20 size=120
    let mut pc: u32 = 0x83084A20;
    'dispatch: loop {
        match pc {
            0x83084A20 => {
    //   block [0x83084A20..0x83084A98)
	// 83084A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084A28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83084A2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83084A30: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83084A34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084A38: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83084A3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83084A40: 396BA5E0  addi r11, r11, -0x5a20
	ctx.r[11].s64 = ctx.r[11].s64 + -23072;
	// 83084A44: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83084A48: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83084A4C: 897E0020  lbz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83084A50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084A54: 41820024  beq 0x83084a78
	if ctx.cr[0].eq {
	pc = 0x83084A78; continue 'dispatch;
	}
	// 83084A58: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83084A5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084A60: 41820018  beq 0x83084a78
	if ctx.cr[0].eq {
	pc = 0x83084A78; continue 'dispatch;
	}
	// 83084A64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084A68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83084A6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084A70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084A74: 4E800421  bctrl
	ctx.lr = 0x83084A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83084A7C: 4BFFFA4D  bl 0x830844c8
	ctx.lr = 0x83084A80;
	sub_830844C8(ctx, base);
	// 83084A80: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83084A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084A8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83084A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83084A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084A98 size=40
    let mut pc: u32 = 0x83084A98;
    'dispatch: loop {
        match pc {
            0x83084A98 => {
    //   block [0x83084A98..0x83084AC0)
	// 83084A98: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83084A9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084AA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084AA8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83084AAC: 4BFFFA1D  bl 0x830844c8
	ctx.lr = 0x83084AB0;
	sub_830844C8(ctx, base);
	// 83084AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83084AC0 size=8
    let mut pc: u32 = 0x83084AC0;
    'dispatch: loop {
        match pc {
            0x83084AC0 => {
    //   block [0x83084AC0..0x83084AC8)
	// 83084AC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83084AC4: 8216A6F0  lwz r16, -0x5910(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-22800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084AC8 size=348
    let mut pc: u32 = 0x83084AC8;
    'dispatch: loop {
        match pc {
            0x83084AC8 => {
    //   block [0x83084AC8..0x83084C24)
	// 83084AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084ACC: 4812369D  bl 0x831a8168
	ctx.lr = 0x83084AD0;
	sub_831A8130(ctx, base);
	// 83084AD0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83084AD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084AD8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83084ADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83084AE0: 396BA6D0  addi r11, r11, -0x5930
	ctx.r[11].s64 = ctx.r[11].s64 + -22832;
	// 83084AE4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83084AE8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83084AEC: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084AF0: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83084AF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084AF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084B00: 4E800421  bctrl
	ctx.lr = 0x83084B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084B04: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83084B08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83084B0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084B10: 40990038  ble cr6, 0x83084b48
	if !ctx.cr[6].gt {
	pc = 0x83084B48; continue 'dispatch;
	}
	// 83084B14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83084B18: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83084B1C: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084B20: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83084B24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084B28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084B2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084B30: 4E800421  bctrl
	ctx.lr = 0x83084B34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084B34: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83084B38: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83084B3C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83084B40: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83084B44: 4198FFD4  blt cr6, 0x83084b18
	if ctx.cr[6].lt {
	pc = 0x83084B18; continue 'dispatch;
	}
	// 83084B48: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084B4C: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83084B50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084B54: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084B5C: 4E800421  bctrl
	ctx.lr = 0x83084B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084B60: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83084B64: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83084B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83084B6C: 40990040  ble cr6, 0x83084bac
	if !ctx.cr[6].gt {
	pc = 0x83084BAC; continue 'dispatch;
	}
	// 83084B70: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83084B74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084B78: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83084B7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084B80: 41820018  beq 0x83084b98
	if ctx.cr[0].eq {
	pc = 0x83084B98; continue 'dispatch;
	}
	// 83084B84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084B88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83084B8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084B90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084B94: 4E800421  bctrl
	ctx.lr = 0x83084B98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084B98: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83084B9C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83084BA0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83084BA4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83084BA8: 4198FFCC  blt cr6, 0x83084b74
	if ctx.cr[6].lt {
	pc = 0x83084B74; continue 'dispatch;
	}
	// 83084BAC: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084BB0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83084BB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084BB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084BC0: 4E800421  bctrl
	ctx.lr = 0x83084BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084BC4: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084BC8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084BCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084BD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084BD8: 4E800421  bctrl
	ctx.lr = 0x83084BDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084BDC: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084BE0: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83084BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83084BE8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83084BF0: 4E800421  bctrl
	ctx.lr = 0x83084BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83084BF4: 83BE003C  lwz r29, 0x3c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83084BF8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084BFC: 41820014  beq 0x83084c10
	if ctx.cr[0].eq {
	pc = 0x83084C10; continue 'dispatch;
	}
	// 83084C00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83084C04: 4BFF9515  bl 0x8307e118
	ctx.lr = 0x83084C08;
	sub_8307E118(ctx, base);
	// 83084C08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83084C0C: 4BF536D5  bl 0x82fd82e0
	ctx.lr = 0x83084C10;
	sub_82FD82E0(ctx, base);
	// 83084C10: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83084C14: 396BD9B0  addi r11, r11, -0x2650
	ctx.r[11].s64 = ctx.r[11].s64 + -9808;
	// 83084C18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83084C1C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83084C20: 48123598  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084C24 size=40
    let mut pc: u32 = 0x83084C24;
    'dispatch: loop {
        match pc {
            0x83084C24 => {
    //   block [0x83084C24..0x83084C4C)
	// 83084C24: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83084C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084C34: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83084C38: 4BFB9031  bl 0x8303dc68
	ctx.lr = 0x83084C3C;
	sub_8303DC68(ctx, base);
	// 83084C3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084C50 size=136
    let mut pc: u32 = 0x83084C50;
    'dispatch: loop {
        match pc {
            0x83084C50 => {
    //   block [0x83084C50..0x83084CD8)
	// 83084C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084C5C: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83084C60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83084C64: 806A9754  lwz r3, -0x68ac(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26796 as u32) ) } as u64;
	// 83084C68: 7F041840  cmplw cr6, r4, r3
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83084C6C: 419A0030  beq cr6, 0x83084c9c
	if ctx.cr[6].eq {
	pc = 0x83084C9C; continue 'dispatch;
	}
	// 83084C70: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83084C74: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83084C78: 40980034  bge cr6, 0x83084cac
	if !ctx.cr[6].lt {
	pc = 0x83084CAC; continue 'dispatch;
	}
	// 83084C7C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83084C80: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83084C84: 40980028  bge cr6, 0x83084cac
	if !ctx.cr[6].lt {
	pc = 0x83084CAC; continue 'dispatch;
	}
	// 83084C88: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83084C8C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83084C90: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83084C94: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83084C98: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83084C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83084CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084CA8: 4E800020  blr
	return;
	// 83084CAC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83084CB0: 80EB0040  lwz r7, 0x40(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084CB4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 83084CB8: 388AA720  addi r4, r10, -0x58e0
	ctx.r[4].s64 = ctx.r[10].s64 + -22752;
	// 83084CBC: 38A0013B  li r5, 0x13b
	ctx.r[5].s64 = 315;
	// 83084CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83084CC4: 4BF4BC95  bl 0x82fd0958
	ctx.lr = 0x83084CC8;
	sub_82FD0958(ctx, base);
	// 83084CC8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83084CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83084CD0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83084CD4: 4812BF55  bl 0x831b0c28
	ctx.lr = 0x83084CD8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83084CD8 size=8
    let mut pc: u32 = 0x83084CD8;
    'dispatch: loop {
        match pc {
            0x83084CD8 => {
    //   block [0x83084CD8..0x83084CE0)
	// 83084CD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83084CDC: 8216A7D0  lwz r16, -0x5830(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-22576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084CE0 size=452
    let mut pc: u32 = 0x83084CE0;
    'dispatch: loop {
        match pc {
            0x83084CE0 => {
    //   block [0x83084CE0..0x83084EA4)
	// 83084CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084CE4: 4812347D  bl 0x831a8160
	ctx.lr = 0x83084CE8;
	sub_831A8130(ctx, base);
	// 83084CE8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83084CEC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084CF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83084CF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83084CF8: 839D0018  lwz r28, 0x18(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83084CFC: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83084D00: 578B073E  clrlwi r11, r28, 0x1c
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000000Fu64;
	// 83084D04: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83084D08: 419A0154  beq cr6, 0x83084e5c
	if ctx.cr[6].eq {
	pc = 0x83084E5C; continue 'dispatch;
	}
	// 83084D0C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83084D10: 419A014C  beq cr6, 0x83084e5c
	if ctx.cr[6].eq {
	pc = 0x83084E5C; continue 'dispatch;
	}
	// 83084D14: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83084D18: 419A0144  beq cr6, 0x83084e5c
	if ctx.cr[6].eq {
	pc = 0x83084E5C; continue 'dispatch;
	}
	// 83084D1C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83084D20: 409A0040  bne cr6, 0x83084d60
	if !ctx.cr[6].eq {
	pc = 0x83084D60; continue 'dispatch;
	}
	// 83084D24: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83084D28: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084D2C: 4BF5356D  bl 0x82fd8298
	ctx.lr = 0x83084D30;
	sub_82FD8298(ctx, base);
	// 83084D30: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084D34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084D38: 41820020  beq 0x83084d58
	if ctx.cr[0].eq {
	pc = 0x83084D58; continue 'dispatch;
	}
	// 83084D3C: 80BE0024  lwz r5, 0x24(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83084D40: 80DE0040  lwz r6, 0x40(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084D44: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 83084D48: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83084D4C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084D50: 4BFFF971  bl 0x830846c0
	ctx.lr = 0x83084D54;
	sub_830846C0(ctx, base);
	// 83084D54: 48000008  b 0x83084d5c
	pc = 0x83084D5C; continue 'dispatch;
	// 83084D58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83084D5C: 48000140  b 0x83084e9c
	pc = 0x83084E9C; continue 'dispatch;
	// 83084D60: 837D0010  lwz r27, 0x10(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83084D64: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83084D68: 835D0014  lwz r26, 0x14(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83084D6C: 419A0098  beq cr6, 0x83084e04
	if ctx.cr[6].eq {
	pc = 0x83084E04; continue 'dispatch;
	}
	// 83084D70: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83084D74: 419A0090  beq cr6, 0x83084e04
	if ctx.cr[6].eq {
	pc = 0x83084E04; continue 'dispatch;
	}
	// 83084D78: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 83084D7C: 419A0040  beq cr6, 0x83084dbc
	if ctx.cr[6].eq {
	pc = 0x83084DBC; continue 'dispatch;
	}
	// 83084D80: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 83084D84: 419A0038  beq cr6, 0x83084dbc
	if ctx.cr[6].eq {
	pc = 0x83084DBC; continue 'dispatch;
	}
	// 83084D88: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 83084D8C: 419A0030  beq cr6, 0x83084dbc
	if ctx.cr[6].eq {
	pc = 0x83084DBC; continue 'dispatch;
	}
	// 83084D90: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83084D94: 80FE0040  lwz r7, 0x40(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084D98: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83084D9C: 388BA768  addi r4, r11, -0x5898
	ctx.r[4].s64 = ctx.r[11].s64 + -22680;
	// 83084DA0: 38A00450  li r5, 0x450
	ctx.r[5].s64 = 1104;
	// 83084DA4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83084DA8: 4BF4C2B1  bl 0x82fd1058
	ctx.lr = 0x83084DAC;
	sub_82FD1058(ctx, base);
	// 83084DAC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83084DB0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83084DB4: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83084DB8: 4812BE71  bl 0x831b0c28
	ctx.lr = 0x83084DBC;
	sub_831B0C28(ctx, base);
	// 83084DBC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83084DC0: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084DC4: 4BF534D5  bl 0x82fd8298
	ctx.lr = 0x83084DC8;
	sub_82FD8298(ctx, base);
	// 83084DC8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83084DCC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83084DD0: 4182002C  beq 0x83084dfc
	if ctx.cr[0].eq {
	pc = 0x83084DFC; continue 'dispatch;
	}
	// 83084DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83084DD8: 83DE0040  lwz r30, 0x40(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084DDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83084DE0: 4BFFFF01  bl 0x83084ce0
	ctx.lr = 0x83084DE4;
	sub_83084CE0(ctx, base);
	// 83084DE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83084DE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83084DEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83084DF0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83084DF4: 4801D83D  bl 0x830a2630
	ctx.lr = 0x83084DF8;
	sub_830A2630(ctx, base);
	// 83084DF8: 48000008  b 0x83084e00
	pc = 0x83084E00; continue 'dispatch;
	// 83084DFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83084E00: 4800009C  b 0x83084e9c
	pc = 0x83084E9C; continue 'dispatch;
	// 83084E04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83084E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83084E0C: 4BFFFED5  bl 0x83084ce0
	ctx.lr = 0x83084E10;
	sub_83084CE0(ctx, base);
	// 83084E10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83084E14: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83084E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83084E1C: 4BFFFEC5  bl 0x83084ce0
	ctx.lr = 0x83084E20;
	sub_83084CE0(ctx, base);
	// 83084E20: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83084E24: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83084E28: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084E2C: 4BF5346D  bl 0x82fd8298
	ctx.lr = 0x83084E30;
	sub_82FD8298(ctx, base);
	// 83084E30: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084E34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084E38: 4182001C  beq 0x83084e54
	if ctx.cr[0].eq {
	pc = 0x83084E54; continue 'dispatch;
	}
	// 83084E3C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83084E40: 80FE0040  lwz r7, 0x40(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084E44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83084E48: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83084E4C: 4801D48D  bl 0x830a22d8
	ctx.lr = 0x83084E50;
	sub_830A22D8(ctx, base);
	// 83084E50: 48000008  b 0x83084e58
	pc = 0x83084E58; continue 'dispatch;
	// 83084E54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83084E58: 48000044  b 0x83084e9c
	pc = 0x83084E9C; continue 'dispatch;
	// 83084E5C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83084E60: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084E64: 4BF53435  bl 0x82fd8298
	ctx.lr = 0x83084E68;
	sub_82FD8298(ctx, base);
	// 83084E68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83084E6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83084E70: 41820028  beq 0x83084e98
	if ctx.cr[0].eq {
	pc = 0x83084E98; continue 'dispatch;
	}
	// 83084E74: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83084E78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83084E7C: 80FE0040  lwz r7, 0x40(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084E80: 39660001  addi r11, r6, 1
	ctx.r[11].s64 = ctx.r[6].s64 + 1;
	// 83084E84: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83084E88: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83084E8C: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83084E90: 4801D1A1  bl 0x830a2030
	ctx.lr = 0x83084E94;
	sub_830A2030(ctx, base);
	// 83084E94: 48000008  b 0x83084e9c
	pc = 0x83084E9C; continue 'dispatch;
	// 83084E98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83084E9C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83084EA0: 48123310  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084EA4 size=48
    let mut pc: u32 = 0x83084EA4;
    'dispatch: loop {
        match pc {
            0x83084EA4 => {
    //   block [0x83084EA4..0x83084ED4)
	// 83084EA4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83084EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084EB4: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83084EB8: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084EBC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83084EC0: 4BF53421  bl 0x82fd82e0
	ctx.lr = 0x83084EC4;
	sub_82FD82E0(ctx, base);
	// 83084EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084ED4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084ED4 size=48
    let mut pc: u32 = 0x83084ED4;
    'dispatch: loop {
        match pc {
            0x83084ED4 => {
    //   block [0x83084ED4..0x83084F04)
	// 83084ED4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83084ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084EE4: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83084EE8: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084EEC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83084EF0: 4BF533F1  bl 0x82fd82e0
	ctx.lr = 0x83084EF4;
	sub_82FD82E0(ctx, base);
	// 83084EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83084F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83084F04 size=48
    let mut pc: u32 = 0x83084F04;
    'dispatch: loop {
        match pc {
            0x83084F04 => {
    //   block [0x83084F04..0x83084F34)
	// 83084F04: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83084F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83084F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83084F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83084F14: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83084F18: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83084F1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83084F20: 4BF533C1  bl 0x82fd82e0
	ctx.lr = 0x83084F24;
	sub_82FD82E0(ctx, base);
	// 83084F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83084F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83084F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83084F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


