pub fn sub_825165C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825165C0 size=8
    let mut pc: u32 = 0x825165C0;
    'dispatch: loop {
        match pc {
            0x825165C0 => {
    //   block [0x825165C0..0x825165C8)
	// 825165C0: 2B0401F4  cmplwi cr6, r4, 0x1f4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 500 as u32, &mut ctx.xer);
	// 825165C4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825165C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825165C8 size=36
    let mut pc: u32 = 0x825165C8;
    'dispatch: loop {
        match pc {
            0x825165C8 => {
    //   block [0x825165C8..0x825165EC)
	// 825165C8: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825165CC: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 825165D0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825165D4: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825165D8: 396B838C  addi r11, r11, -0x7c74
	ctx.r[11].s64 = ctx.r[11].s64 + -31860;
	// 825165DC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825165E0: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 825165E4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825165E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825165F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825165F0 size=20
    let mut pc: u32 = 0x825165F0;
    'dispatch: loop {
        match pc {
            0x825165F0 => {
    //   block [0x825165F0..0x82516604)
	// 825165F0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825165F4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825165F8: 614A85A0  ori r10, r10, 0x85a0
	ctx.r[10].u64 = ctx.r[10].u64 | 34208;
	// 825165FC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516608 size=60
    let mut pc: u32 = 0x82516608;
    'dispatch: loop {
        match pc {
            0x82516608 => {
    //   block [0x82516608..0x82516644)
	// 82516608: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251660C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516610: 616B85A0  ori r11, r11, 0x85a0
	ctx.r[11].u64 = ctx.r[11].u64 | 34208;
	// 82516614: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516618: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251661C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82516620: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82516624: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516628: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251662C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516630: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516634: 40980010  bge cr6, 0x82516644
	if !ctx.cr[6].lt {
		sub_82516644(ctx, base);
		return;
	}
	// 82516638: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251663C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82516640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516644 size=16
    let mut pc: u32 = 0x82516644;
    'dispatch: loop {
        match pc {
            0x82516644 => {
    //   block [0x82516644..0x82516654)
	// 82516644: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516648: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251664C: 2F0A00C8  cmpwi cr6, r10, 0xc8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 200, &mut ctx.xer);
	// 82516650: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516654 size=12
    let mut pc: u32 = 0x82516654;
    'dispatch: loop {
        match pc {
            0x82516654 => {
    //   block [0x82516654..0x82516660)
	// 82516654: 394000C8  li r10, 0xc8
	ctx.r[10].s64 = 200;
	// 82516658: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251665C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516660 size=12
    let mut pc: u32 = 0x82516660;
    'dispatch: loop {
        match pc {
            0x82516660 => {
    //   block [0x82516660..0x8251666C)
	// 82516660: 548BD97E  srwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516664: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82516668: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251666C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251666C size=40
    let mut pc: u32 = 0x8251666C;
    'dispatch: loop {
        match pc {
            0x8251666C => {
    //   block [0x8251666C..0x82516694)
	// 8251666C: 396B2160  addi r11, r11, 0x2160
	ctx.r[11].s64 = ctx.r[11].s64 + 8544;
	// 82516670: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516674: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82516678: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251667C: 548906FE  clrlwi r9, r4, 0x1b
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82516680: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82516684: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516688: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 8251668C: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82516690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516698 size=64
    let mut pc: u32 = 0x82516698;
    'dispatch: loop {
        match pc {
            0x82516698 => {
    //   block [0x82516698..0x825166D8)
	// 82516698: 548BD97E  srwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251669C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 825166A0: 40980038  bge cr6, 0x825166d8
	if !ctx.cr[6].lt {
		sub_825166D8(ctx, base);
		return;
	}
	// 825166A4: 396B2160  addi r11, r11, 0x2160
	ctx.r[11].s64 = ctx.r[11].s64 + 8544;
	// 825166A8: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825166AC: 548906FE  clrlwi r9, r4, 0x1b
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 825166B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825166B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825166B8: 7D094830  slw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 825166BC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825166C0: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 825166C4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825166C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825166CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825166D0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 825166D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825166D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825166D8 size=8
    let mut pc: u32 = 0x825166D8;
    'dispatch: loop {
        match pc {
            0x825166D8 => {
    //   block [0x825166D8..0x825166E0)
	// 825166D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825166DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825166E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825166E0 size=20
    let mut pc: u32 = 0x825166E0;
    'dispatch: loop {
        match pc {
            0x825166E0 => {
    //   block [0x825166E0..0x825166F4)
	// 825166E0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825166E4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825166E8: 614A85A8  ori r10, r10, 0x85a8
	ctx.r[10].u64 = ctx.r[10].u64 | 34216;
	// 825166EC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825166F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825166F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825166F8 size=60
    let mut pc: u32 = 0x825166F8;
    'dispatch: loop {
        match pc {
            0x825166F8 => {
    //   block [0x825166F8..0x82516734)
	// 825166F8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825166FC: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516700: 616B85A8  ori r11, r11, 0x85a8
	ctx.r[11].u64 = ctx.r[11].u64 | 34216;
	// 82516704: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516708: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251670C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82516710: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82516714: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516718: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251671C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516720: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516724: 40980010  bge cr6, 0x82516734
	if !ctx.cr[6].lt {
		sub_82516734(ctx, base);
		return;
	}
	// 82516728: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251672C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82516730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516734 size=24
    let mut pc: u32 = 0x82516734;
    'dispatch: loop {
        match pc {
            0x82516734 => {
    //   block [0x82516734..0x8251674C)
	// 82516734: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516738: 3D20000F  lis r9, 0xf
	ctx.r[9].s64 = 983040;
	// 8251673C: 612B423F  ori r11, r9, 0x423f
	ctx.r[11].u64 = ctx.r[9].u64 | 16959;
	// 82516740: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516744: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82516748: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251674C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251674C size=8
    let mut pc: u32 = 0x8251674C;
    'dispatch: loop {
        match pc {
            0x8251674C => {
    //   block [0x8251674C..0x82516754)
	// 8251674C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516758 size=20
    let mut pc: u32 = 0x82516758;
    'dispatch: loop {
        match pc {
            0x82516758 => {
    //   block [0x82516758..0x8251676C)
	// 82516758: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8251675C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516760: 614A85AC  ori r10, r10, 0x85ac
	ctx.r[10].u64 = ctx.r[10].u64 | 34220;
	// 82516764: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516770 size=44
    let mut pc: u32 = 0x82516770;
    'dispatch: loop {
        match pc {
            0x82516770 => {
    //   block [0x82516770..0x8251679C)
	// 82516770: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82516774: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516778: 616B85AC  ori r11, r11, 0x85ac
	ctx.r[11].u64 = ctx.r[11].u64 | 34220;
	// 8251677C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516780: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516784: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82516788: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251678C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516790: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82516794: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516798: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251679C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251679C size=16
    let mut pc: u32 = 0x8251679C;
    'dispatch: loop {
        match pc {
            0x8251679C => {
    //   block [0x8251679C..0x825167AC)
	// 8251679C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825167A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825167A4: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 825167A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825167B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825167B0 size=32
    let mut pc: u32 = 0x825167B0;
    'dispatch: loop {
        match pc {
            0x825167B0 => {
    //   block [0x825167B0..0x825167D0)
	// 825167B0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825167B4: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825167B8: 616B85AC  ori r11, r11, 0x85ac
	ctx.r[11].u64 = ctx.r[11].u64 | 34220;
	// 825167BC: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 825167C0: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825167C4: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825167C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825167CC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825167D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825167D0 size=16
    let mut pc: u32 = 0x825167D0;
    'dispatch: loop {
        match pc {
            0x825167D0 => {
    //   block [0x825167D0..0x825167E0)
	// 825167D0: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825167D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825167D8: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 825167DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825167E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825167E0 size=20
    let mut pc: u32 = 0x825167E0;
    'dispatch: loop {
        match pc {
            0x825167E0 => {
    //   block [0x825167E0..0x825167F4)
	// 825167E0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825167E4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825167E8: 614A85B0  ori r10, r10, 0x85b0
	ctx.r[10].u64 = ctx.r[10].u64 | 34224;
	// 825167EC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825167F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825167F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825167F8 size=60
    let mut pc: u32 = 0x825167F8;
    'dispatch: loop {
        match pc {
            0x825167F8 => {
    //   block [0x825167F8..0x82516834)
	// 825167F8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825167FC: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516800: 616B85B0  ori r11, r11, 0x85b0
	ctx.r[11].u64 = ctx.r[11].u64 | 34224;
	// 82516804: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516808: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251680C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82516810: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82516814: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516818: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251681C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516820: 40980014  bge cr6, 0x82516834
	if !ctx.cr[6].lt {
		sub_82516834(ctx, base);
		return;
	}
	// 82516824: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516828: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251682C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82516830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516834(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516834 size=28
    let mut pc: u32 = 0x82516834;
    'dispatch: loop {
        match pc {
            0x82516834 => {
    //   block [0x82516834..0x82516850)
	// 82516834: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516838: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251683C: 614A85AC  ori r10, r10, 0x85ac
	ctx.r[10].u64 = ctx.r[10].u64 | 34220;
	// 82516840: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82516844: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516848: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8251684C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516850 size=16
    let mut pc: u32 = 0x82516850;
    'dispatch: loop {
        match pc {
            0x82516850 => {
    //   block [0x82516850..0x82516860)
	// 82516850: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516854: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516858: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8251685C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516860 size=48
    let mut pc: u32 = 0x82516860;
    'dispatch: loop {
        match pc {
            0x82516860 => {
    //   block [0x82516860..0x82516890)
	// 82516860: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82516864: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516868: 616B85B0  ori r11, r11, 0x85b0
	ctx.r[11].u64 = ctx.r[11].u64 | 34224;
	// 8251686C: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 82516870: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516874: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82516878: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251687C: 40980014  bge cr6, 0x82516890
	if !ctx.cr[6].lt {
		sub_82516890(ctx, base);
		return;
	}
	// 82516880: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82516888: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8251688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516890 size=28
    let mut pc: u32 = 0x82516890;
    'dispatch: loop {
        match pc {
            0x82516890 => {
    //   block [0x82516890..0x825168AC)
	// 82516890: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516894: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516898: 614A85AC  ori r10, r10, 0x85ac
	ctx.r[10].u64 = ctx.r[10].u64 | 34220;
	// 8251689C: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825168A0: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825168A4: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825168A8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825168AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825168AC size=16
    let mut pc: u32 = 0x825168AC;
    'dispatch: loop {
        match pc {
            0x825168AC => {
    //   block [0x825168AC..0x825168BC)
	// 825168AC: 81230094  lwz r9, 0x94(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825168B0: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825168B4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 825168B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825168C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825168C0 size=20
    let mut pc: u32 = 0x825168C0;
    'dispatch: loop {
        match pc {
            0x825168C0 => {
    //   block [0x825168C0..0x825168D4)
	// 825168C0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825168C4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825168C8: 614A85F0  ori r10, r10, 0x85f0
	ctx.r[10].u64 = ctx.r[10].u64 | 34288;
	// 825168CC: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 825168D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825168D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825168D8 size=20
    let mut pc: u32 = 0x825168D8;
    'dispatch: loop {
        match pc {
            0x825168D8 => {
    //   block [0x825168D8..0x825168EC)
	// 825168D8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825168DC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825168E0: 614A85F0  ori r10, r10, 0x85f0
	ctx.r[10].u64 = ctx.r[10].u64 | 34288;
	// 825168E4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825168E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825168F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825168F0 size=20
    let mut pc: u32 = 0x825168F0;
    'dispatch: loop {
        match pc {
            0x825168F0 => {
    //   block [0x825168F0..0x82516904)
	// 825168F0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825168F4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825168F8: 614A85F4  ori r10, r10, 0x85f4
	ctx.r[10].u64 = ctx.r[10].u64 | 34292;
	// 825168FC: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516908 size=20
    let mut pc: u32 = 0x82516908;
    'dispatch: loop {
        match pc {
            0x82516908 => {
    //   block [0x82516908..0x8251691C)
	// 82516908: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8251690C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516910: 614A85F4  ori r10, r10, 0x85f4
	ctx.r[10].u64 = ctx.r[10].u64 | 34292;
	// 82516914: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516920 size=20
    let mut pc: u32 = 0x82516920;
    'dispatch: loop {
        match pc {
            0x82516920 => {
    //   block [0x82516920..0x82516934)
	// 82516920: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516924: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516928: 614A85F8  ori r10, r10, 0x85f8
	ctx.r[10].u64 = ctx.r[10].u64 | 34296;
	// 8251692C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516938 size=20
    let mut pc: u32 = 0x82516938;
    'dispatch: loop {
        match pc {
            0x82516938 => {
    //   block [0x82516938..0x8251694C)
	// 82516938: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8251693C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516940: 614A85F8  ori r10, r10, 0x85f8
	ctx.r[10].u64 = ctx.r[10].u64 | 34296;
	// 82516944: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516950 size=20
    let mut pc: u32 = 0x82516950;
    'dispatch: loop {
        match pc {
            0x82516950 => {
    //   block [0x82516950..0x82516964)
	// 82516950: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516954: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516958: 614A85FC  ori r10, r10, 0x85fc
	ctx.r[10].u64 = ctx.r[10].u64 | 34300;
	// 8251695C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516968 size=20
    let mut pc: u32 = 0x82516968;
    'dispatch: loop {
        match pc {
            0x82516968 => {
    //   block [0x82516968..0x8251697C)
	// 82516968: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8251696C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516970: 614A85FC  ori r10, r10, 0x85fc
	ctx.r[10].u64 = ctx.r[10].u64 | 34300;
	// 82516974: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516980 size=20
    let mut pc: u32 = 0x82516980;
    'dispatch: loop {
        match pc {
            0x82516980 => {
    //   block [0x82516980..0x82516994)
	// 82516980: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516984: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516988: 614A85B4  ori r10, r10, 0x85b4
	ctx.r[10].u64 = ctx.r[10].u64 | 34228;
	// 8251698C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516998 size=28
    let mut pc: u32 = 0x82516998;
    'dispatch: loop {
        match pc {
            0x82516998 => {
    //   block [0x82516998..0x825169B4)
	// 82516998: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251699C: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 825169A0: 394A85B4  addi r10, r10, -0x7a4c
	ctx.r[10].s64 = ctx.r[10].s64 + -31308;
	// 825169A4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825169A8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825169AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825169B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825169B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825169B8 size=20
    let mut pc: u32 = 0x825169B8;
    'dispatch: loop {
        match pc {
            0x825169B8 => {
    //   block [0x825169B8..0x825169CC)
	// 825169B8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825169BC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825169C0: 614A85B4  ori r10, r10, 0x85b4
	ctx.r[10].u64 = ctx.r[10].u64 | 34228;
	// 825169C4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825169C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825169D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825169D0 size=20
    let mut pc: u32 = 0x825169D0;
    'dispatch: loop {
        match pc {
            0x825169D0 => {
    //   block [0x825169D0..0x825169E4)
	// 825169D0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825169D4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825169D8: 614A85B8  ori r10, r10, 0x85b8
	ctx.r[10].u64 = ctx.r[10].u64 | 34232;
	// 825169DC: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 825169E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825169E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825169E8 size=20
    let mut pc: u32 = 0x825169E8;
    'dispatch: loop {
        match pc {
            0x825169E8 => {
    //   block [0x825169E8..0x825169FC)
	// 825169E8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825169EC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825169F0: 614A85B8  ori r10, r10, 0x85b8
	ctx.r[10].u64 = ctx.r[10].u64 | 34232;
	// 825169F4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825169F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A00 size=20
    let mut pc: u32 = 0x82516A00;
    'dispatch: loop {
        match pc {
            0x82516A00 => {
    //   block [0x82516A00..0x82516A14)
	// 82516A00: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516A04: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A08: 614A85BC  ori r10, r10, 0x85bc
	ctx.r[10].u64 = ctx.r[10].u64 | 34236;
	// 82516A0C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A18 size=28
    let mut pc: u32 = 0x82516A18;
    'dispatch: loop {
        match pc {
            0x82516A18 => {
    //   block [0x82516A18..0x82516A34)
	// 82516A18: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A1C: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82516A20: 394A85BC  addi r10, r10, -0x7a44
	ctx.r[10].s64 = ctx.r[10].s64 + -31300;
	// 82516A24: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516A28: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516A2C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A38 size=20
    let mut pc: u32 = 0x82516A38;
    'dispatch: loop {
        match pc {
            0x82516A38 => {
    //   block [0x82516A38..0x82516A4C)
	// 82516A38: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516A3C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A40: 614A85BC  ori r10, r10, 0x85bc
	ctx.r[10].u64 = ctx.r[10].u64 | 34236;
	// 82516A44: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A50 size=20
    let mut pc: u32 = 0x82516A50;
    'dispatch: loop {
        match pc {
            0x82516A50 => {
    //   block [0x82516A50..0x82516A64)
	// 82516A50: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516A54: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A58: 614A85C0  ori r10, r10, 0x85c0
	ctx.r[10].u64 = ctx.r[10].u64 | 34240;
	// 82516A5C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A68 size=20
    let mut pc: u32 = 0x82516A68;
    'dispatch: loop {
        match pc {
            0x82516A68 => {
    //   block [0x82516A68..0x82516A7C)
	// 82516A68: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516A6C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A70: 614A85C0  ori r10, r10, 0x85c0
	ctx.r[10].u64 = ctx.r[10].u64 | 34240;
	// 82516A74: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A80 size=20
    let mut pc: u32 = 0x82516A80;
    'dispatch: loop {
        match pc {
            0x82516A80 => {
    //   block [0x82516A80..0x82516A94)
	// 82516A80: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516A84: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A88: 614A85C4  ori r10, r10, 0x85c4
	ctx.r[10].u64 = ctx.r[10].u64 | 34244;
	// 82516A8C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516A98 size=28
    let mut pc: u32 = 0x82516A98;
    'dispatch: loop {
        match pc {
            0x82516A98 => {
    //   block [0x82516A98..0x82516AB4)
	// 82516A98: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516A9C: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82516AA0: 394A85C4  addi r10, r10, -0x7a3c
	ctx.r[10].s64 = ctx.r[10].s64 + -31292;
	// 82516AA4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516AA8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516AAC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516AB8 size=20
    let mut pc: u32 = 0x82516AB8;
    'dispatch: loop {
        match pc {
            0x82516AB8 => {
    //   block [0x82516AB8..0x82516ACC)
	// 82516AB8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516ABC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516AC0: 614A85C4  ori r10, r10, 0x85c4
	ctx.r[10].u64 = ctx.r[10].u64 | 34244;
	// 82516AC4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516AD0 size=20
    let mut pc: u32 = 0x82516AD0;
    'dispatch: loop {
        match pc {
            0x82516AD0 => {
    //   block [0x82516AD0..0x82516AE4)
	// 82516AD0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516AD4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516AD8: 614A85C8  ori r10, r10, 0x85c8
	ctx.r[10].u64 = ctx.r[10].u64 | 34248;
	// 82516ADC: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516AE8 size=20
    let mut pc: u32 = 0x82516AE8;
    'dispatch: loop {
        match pc {
            0x82516AE8 => {
    //   block [0x82516AE8..0x82516AFC)
	// 82516AE8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516AEC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516AF0: 614A85C8  ori r10, r10, 0x85c8
	ctx.r[10].u64 = ctx.r[10].u64 | 34248;
	// 82516AF4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B00 size=20
    let mut pc: u32 = 0x82516B00;
    'dispatch: loop {
        match pc {
            0x82516B00 => {
    //   block [0x82516B00..0x82516B14)
	// 82516B00: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516B04: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B08: 614A85CC  ori r10, r10, 0x85cc
	ctx.r[10].u64 = ctx.r[10].u64 | 34252;
	// 82516B0C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B18 size=28
    let mut pc: u32 = 0x82516B18;
    'dispatch: loop {
        match pc {
            0x82516B18 => {
    //   block [0x82516B18..0x82516B34)
	// 82516B18: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B1C: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82516B20: 394A85CC  addi r10, r10, -0x7a34
	ctx.r[10].s64 = ctx.r[10].s64 + -31284;
	// 82516B24: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516B28: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516B2C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B38 size=20
    let mut pc: u32 = 0x82516B38;
    'dispatch: loop {
        match pc {
            0x82516B38 => {
    //   block [0x82516B38..0x82516B4C)
	// 82516B38: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516B3C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B40: 614A85CC  ori r10, r10, 0x85cc
	ctx.r[10].u64 = ctx.r[10].u64 | 34252;
	// 82516B44: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B50 size=20
    let mut pc: u32 = 0x82516B50;
    'dispatch: loop {
        match pc {
            0x82516B50 => {
    //   block [0x82516B50..0x82516B64)
	// 82516B50: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516B54: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B58: 614A85D0  ori r10, r10, 0x85d0
	ctx.r[10].u64 = ctx.r[10].u64 | 34256;
	// 82516B5C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B68 size=20
    let mut pc: u32 = 0x82516B68;
    'dispatch: loop {
        match pc {
            0x82516B68 => {
    //   block [0x82516B68..0x82516B7C)
	// 82516B68: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516B6C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B70: 614A85D0  ori r10, r10, 0x85d0
	ctx.r[10].u64 = ctx.r[10].u64 | 34256;
	// 82516B74: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B80 size=20
    let mut pc: u32 = 0x82516B80;
    'dispatch: loop {
        match pc {
            0x82516B80 => {
    //   block [0x82516B80..0x82516B94)
	// 82516B80: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516B84: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B88: 614A85D4  ori r10, r10, 0x85d4
	ctx.r[10].u64 = ctx.r[10].u64 | 34260;
	// 82516B8C: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516B98 size=28
    let mut pc: u32 = 0x82516B98;
    'dispatch: loop {
        match pc {
            0x82516B98 => {
    //   block [0x82516B98..0x82516BB4)
	// 82516B98: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516B9C: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82516BA0: 394A85D4  addi r10, r10, -0x7a2c
	ctx.r[10].s64 = ctx.r[10].s64 + -31276;
	// 82516BA4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516BA8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516BAC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516BB8 size=20
    let mut pc: u32 = 0x82516BB8;
    'dispatch: loop {
        match pc {
            0x82516BB8 => {
    //   block [0x82516BB8..0x82516BCC)
	// 82516BB8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516BBC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516BC0: 614A85D4  ori r10, r10, 0x85d4
	ctx.r[10].u64 = ctx.r[10].u64 | 34260;
	// 82516BC4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516BD0 size=20
    let mut pc: u32 = 0x82516BD0;
    'dispatch: loop {
        match pc {
            0x82516BD0 => {
    //   block [0x82516BD0..0x82516BE4)
	// 82516BD0: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516BD4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516BD8: 614A85D8  ori r10, r10, 0x85d8
	ctx.r[10].u64 = ctx.r[10].u64 | 34264;
	// 82516BDC: 7C8B512E  stwx r4, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u32) };
	// 82516BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516BE8 size=20
    let mut pc: u32 = 0x82516BE8;
    'dispatch: loop {
        match pc {
            0x82516BE8 => {
    //   block [0x82516BE8..0x82516BFC)
	// 82516BE8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516BEC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516BF0: 614A85D8  ori r10, r10, 0x85d8
	ctx.r[10].u64 = ctx.r[10].u64 | 34264;
	// 82516BF4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C00 size=32
    let mut pc: u32 = 0x82516C00;
    'dispatch: loop {
        match pc {
            0x82516C00 => {
    //   block [0x82516C00..0x82516C20)
	// 82516C00: 2F04270F  cmpwi cr6, r4, 0x270f
	ctx.cr[6].compare_i32(ctx.r[4].s32, 9999, &mut ctx.xer);
	// 82516C04: 40990008  ble cr6, 0x82516c0c
	if !ctx.cr[6].gt {
	pc = 0x82516C0C; continue 'dispatch;
	}
	// 82516C08: 3880270F  li r4, 0x270f
	ctx.r[4].s64 = 9999;
	// 82516C0C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82516C10: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516C14: 616B85DC  ori r11, r11, 0x85dc
	ctx.r[11].u64 = ctx.r[11].u64 | 34268;
	// 82516C18: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 82516C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C20 size=60
    let mut pc: u32 = 0x82516C20;
    'dispatch: loop {
        match pc {
            0x82516C20 => {
    //   block [0x82516C20..0x82516C5C)
	// 82516C20: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82516C24: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516C28: 616B85DC  ori r11, r11, 0x85dc
	ctx.r[11].u64 = ctx.r[11].u64 | 34268;
	// 82516C2C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516C30: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516C34: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82516C38: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82516C3C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516C40: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82516C44: 2F0A270F  cmpwi cr6, r10, 0x270f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9999, &mut ctx.xer);
	// 82516C48: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516C4C: 40990010  ble cr6, 0x82516c5c
	if !ctx.cr[6].gt {
		sub_82516C5C(ctx, base);
		return;
	}
	// 82516C50: 3920270F  li r9, 0x270f
	ctx.r[9].s64 = 9999;
	// 82516C54: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82516C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C5C size=16
    let mut pc: u32 = 0x82516C5C;
    'dispatch: loop {
        match pc {
            0x82516C5C => {
    //   block [0x82516C5C..0x82516C6C)
	// 82516C5C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82516C60: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516C64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516C68: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C6C size=12
    let mut pc: u32 = 0x82516C6C;
    'dispatch: loop {
        match pc {
            0x82516C6C => {
    //   block [0x82516C6C..0x82516C78)
	// 82516C6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82516C70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82516C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C78 size=20
    let mut pc: u32 = 0x82516C78;
    'dispatch: loop {
        match pc {
            0x82516C78 => {
    //   block [0x82516C78..0x82516C8C)
	// 82516C78: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516C7C: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516C80: 614A85DC  ori r10, r10, 0x85dc
	ctx.r[10].u64 = ctx.r[10].u64 | 34268;
	// 82516C84: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516C90 size=28
    let mut pc: u32 = 0x82516C90;
    'dispatch: loop {
        match pc {
            0x82516C90 => {
    //   block [0x82516C90..0x82516CAC)
	// 82516C90: 1D64000A  mulli r11, r4, 0xa
	ctx.r[11].s64 = ctx.r[4].s64 * 10;
	// 82516C94: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82516C98: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 82516C9C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82516CA0: 1D6B000A  mulli r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 * 10;
	// 82516CA4: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82516CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516CB0 size=12
    let mut pc: u32 = 0x82516CB0;
    'dispatch: loop {
        match pc {
            0x82516CB0 => {
    //   block [0x82516CB0..0x82516CBC)
	// 82516CB0: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516CB4: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82516CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516CC0 size=12
    let mut pc: u32 = 0x82516CC0;
    'dispatch: loop {
        match pc {
            0x82516CC0 => {
    //   block [0x82516CC0..0x82516CCC)
	// 82516CC0: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516CC4: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516CD0 size=40
    let mut pc: u32 = 0x82516CD0;
    'dispatch: loop {
        match pc {
            0x82516CD0 => {
    //   block [0x82516CD0..0x82516CF8)
	// 82516CD0: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516CD4: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82516CD8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516CDC: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516CE0: 1D4A000A  mulli r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 * 10;
	// 82516CE4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516CE8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516CEC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516CF0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82516CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516CF8 size=16
    let mut pc: u32 = 0x82516CF8;
    'dispatch: loop {
        match pc {
            0x82516CF8 => {
    //   block [0x82516CF8..0x82516D08)
	// 82516CF8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82516CFC: 409A000C  bne cr6, 0x82516d08
	if !ctx.cr[6].eq {
		sub_82516D08(ctx, base);
		return;
	}
	// 82516D00: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82516D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82516D08 size=60
    let mut pc: u32 = 0x82516D08;
    'dispatch: loop {
        match pc {
            0x82516D08 => {
    //   block [0x82516D08..0x82516D44)
	// 82516D08: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82516D0C: 7D445B96  divwu r10, r4, r11
	ctx.r[10].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82516D10: 1D4A000A  mulli r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 * 10;
	// 82516D14: 7D4A2051  subf. r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516D18: 4082FFE8  bne 0x82516d00
	if !ctx.cr[0].eq {
		sub_82516CF8(ctx, base);
		return;
	}
	// 82516D1C: 7D645B96  divwu r11, r4, r11
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82516D20: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 82516D24: 39200046  li r9, 0x46
	ctx.r[9].s64 = 70;
	// 82516D28: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516D2C: 1D4A0064  mulli r10, r10, 0x64
	ctx.r[10].s64 = ctx.r[10].s64 * 100;
	// 82516D30: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516D34: 7D695810  subfc r11, r9, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[9].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82516D38: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82516D3C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82516D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516D48 size=72
    let mut pc: u32 = 0x82516D48;
    'dispatch: loop {
        match pc {
            0x82516D48 => {
    //   block [0x82516D48..0x82516D90)
	// 82516D48: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516D4C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82516D50: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516D54: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516D58: 1D4A000A  mulli r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 * 10;
	// 82516D5C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516D60: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 82516D64: 409A002C  bne cr6, 0x82516d90
	if !ctx.cr[6].eq {
		sub_82516D90(ctx, base);
		return;
	}
	// 82516D68: 39402710  li r10, 0x2710
	ctx.r[10].s64 = 10000;
	// 82516D6C: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 82516D70: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516D74: 1D4A2710  mulli r10, r10, 0x2710
	ctx.r[10].s64 = ctx.r[10].s64 * 10000;
	// 82516D78: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516D7C: 7D6B4B96  divwu r11, r11, r9
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82516D80: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82516D84: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516D88: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516D90 size=8
    let mut pc: u32 = 0x82516D90;
    'dispatch: loop {
        match pc {
            0x82516D90 => {
    //   block [0x82516D90..0x82516D98)
	// 82516D90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516D98 size=28
    let mut pc: u32 = 0x82516D98;
    'dispatch: loop {
        match pc {
            0x82516D98 => {
    //   block [0x82516D98..0x82516DB4)
	// 82516D98: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 82516D9C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82516DA0: 7D645B96  divwu r11, r4, r11
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82516DA4: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 82516DA8: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82516DAC: 7C6B5396  divwu r3, r11, r10
	ctx.r[3].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516DB8 size=12
    let mut pc: u32 = 0x82516DB8;
    'dispatch: loop {
        match pc {
            0x82516DB8 => {
    //   block [0x82516DB8..0x82516DC4)
	// 82516DB8: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516DBC: 908B0028  stw r4, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82516DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516DC8 size=12
    let mut pc: u32 = 0x82516DC8;
    'dispatch: loop {
        match pc {
            0x82516DC8 => {
    //   block [0x82516DC8..0x82516DD4)
	// 82516DC8: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516DCC: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82516DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516DD8 size=44
    let mut pc: u32 = 0x82516DD8;
    'dispatch: loop {
        match pc {
            0x82516DD8 => {
    //   block [0x82516DD8..0x82516E04)
	// 82516DD8: 81430098  lwz r10, 0x98(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516DDC: 896A0020  lbz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516DE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82516DE4: 41820020  beq 0x82516e04
	if ctx.cr[0].eq {
		sub_82516E04(ctx, base);
		return;
	}
	// 82516DE8: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82516DEC: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82516DF0: 2F0B0063  cmpwi cr6, r11, 0x63
	ctx.cr[6].compare_i32(ctx.r[11].s32, 99, &mut ctx.xer);
	// 82516DF4: 40990008  ble cr6, 0x82516dfc
	if !ctx.cr[6].gt {
	pc = 0x82516DFC; continue 'dispatch;
	}
	// 82516DF8: 39600063  li r11, 0x63
	ctx.r[11].s64 = 99;
	// 82516DFC: 916A001C  stw r11, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82516E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516E04 size=40
    let mut pc: u32 = 0x82516E04;
    'dispatch: loop {
        match pc {
            0x82516E04 => {
    //   block [0x82516E04..0x82516E2C)
	// 82516E04: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516E08: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 82516E0C: 394A8000  addi r10, r10, -0x8000
	ctx.r[10].s64 = ctx.r[10].s64 + -32768;
	// 82516E10: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516E14: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82516E18: 2F0B0063  cmpwi cr6, r11, 0x63
	ctx.cr[6].compare_i32(ctx.r[11].s32, 99, &mut ctx.xer);
	// 82516E1C: 40990008  ble cr6, 0x82516e24
	if !ctx.cr[6].gt {
	pc = 0x82516E24; continue 'dispatch;
	}
	// 82516E20: 39600063  li r11, 0x63
	ctx.r[11].s64 = 99;
	// 82516E24: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516E30 size=60
    let mut pc: u32 = 0x82516E30;
    'dispatch: loop {
        match pc {
            0x82516E30 => {
    //   block [0x82516E30..0x82516E6C)
	// 82516E30: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516E34: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82516E38: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82516E3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516E40: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516E44: 1D4A03E8  mulli r10, r10, 0x3e8
	ctx.r[10].s64 = ctx.r[10].s64 * 1000;
	// 82516E48: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516E4C: 7D6B4B96  divwu r11, r11, r9
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82516E50: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82516E54: 419A0018  beq cr6, 0x82516e6c
	if ctx.cr[6].eq {
		sub_82516E6C(ctx, base);
		return;
	}
	// 82516E58: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82516E5C: 419A0010  beq cr6, 0x82516e6c
	if ctx.cr[6].eq {
		sub_82516E6C(ctx, base);
		return;
	}
	// 82516E60: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82516E64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82516E68: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516E6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516E6C size=8
    let mut pc: u32 = 0x82516E6C;
    'dispatch: loop {
        match pc {
            0x82516E6C => {
    //   block [0x82516E6C..0x82516E74)
	// 82516E6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516E78 size=52
    let mut pc: u32 = 0x82516E78;
    'dispatch: loop {
        match pc {
            0x82516E78 => {
    //   block [0x82516E78..0x82516EAC)
	// 82516E78: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516E7C: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82516E80: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82516E84: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516E88: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516E8C: 1D4A03E8  mulli r10, r10, 0x3e8
	ctx.r[10].s64 = ctx.r[10].s64 * 1000;
	// 82516E90: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516E94: 7D6B4B96  divwu r11, r11, r9
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82516E98: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82516E9C: 419A0010  beq cr6, 0x82516eac
	if ctx.cr[6].eq {
		sub_82516EAC(ctx, base);
		return;
	}
	// 82516EA0: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 82516EA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516EA8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516EAC size=8
    let mut pc: u32 = 0x82516EAC;
    'dispatch: loop {
        match pc {
            0x82516EAC => {
    //   block [0x82516EAC..0x82516EB4)
	// 82516EAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82516EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516EB8 size=84
    let mut pc: u32 = 0x82516EB8;
    'dispatch: loop {
        match pc {
            0x82516EB8 => {
    //   block [0x82516EB8..0x82516F0C)
	// 82516EB8: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516EBC: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82516EC0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82516EC4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516EC8: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516ECC: 1D4A03E8  mulli r10, r10, 0x3e8
	ctx.r[10].s64 = ctx.r[10].s64 * 1000;
	// 82516ED0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516ED4: 7D6B4B96  divwu r11, r11, r9
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82516ED8: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82516EDC: 419A0030  beq cr6, 0x82516f0c
	if ctx.cr[6].eq {
		sub_82516F0C(ctx, base);
		return;
	}
	// 82516EE0: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82516EE4: 419A0028  beq cr6, 0x82516f0c
	if ctx.cr[6].eq {
		sub_82516F0C(ctx, base);
		return;
	}
	// 82516EE8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82516EEC: 419A0020  beq cr6, 0x82516f0c
	if ctx.cr[6].eq {
		sub_82516F0C(ctx, base);
		return;
	}
	// 82516EF0: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82516EF4: 419A0018  beq cr6, 0x82516f0c
	if ctx.cr[6].eq {
		sub_82516F0C(ctx, base);
		return;
	}
	// 82516EF8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82516EFC: 419A0010  beq cr6, 0x82516f0c
	if ctx.cr[6].eq {
		sub_82516F0C(ctx, base);
		return;
	}
	// 82516F00: 2B0B000D  cmplwi cr6, r11, 0xd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 13 as u32, &mut ctx.xer);
	// 82516F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516F08: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516F0C size=8
    let mut pc: u32 = 0x82516F0C;
    'dispatch: loop {
        match pc {
            0x82516F0C => {
    //   block [0x82516F0C..0x82516F14)
	// 82516F0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82516F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516F18 size=12
    let mut pc: u32 = 0x82516F18;
    'dispatch: loop {
        match pc {
            0x82516F18 => {
    //   block [0x82516F18..0x82516F24)
	// 82516F18: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516F1C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516F20: 4BFFFDD8  b 0x82516cf8
	sub_82516CF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516F28 size=48
    let mut pc: u32 = 0x82516F28;
    'dispatch: loop {
        match pc {
            0x82516F28 => {
    //   block [0x82516F28..0x82516F58)
	// 82516F28: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516F2C: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82516F30: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82516F34: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82516F38: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516F3C: 1D4A03E8  mulli r10, r10, 0x3e8
	ctx.r[10].s64 = ctx.r[10].s64 * 1000;
	// 82516F40: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82516F44: 7D6B4B96  divwu r11, r11, r9
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[9].u32;
	// 82516F48: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82516F4C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516F50: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516F58 size=40
    let mut pc: u32 = 0x82516F58;
    'dispatch: loop {
        match pc {
            0x82516F58 => {
    //   block [0x82516F58..0x82516F80)
	// 82516F58: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 82516F5C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82516F60: 7D645B96  divwu r11, r4, r11
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82516F64: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 82516F68: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82516F6C: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82516F70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82516F74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516F78: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82516F80 size=644
    let mut pc: u32 = 0x82516F80;
    'dispatch: loop {
        match pc {
            0x82516F80 => {
    //   block [0x82516F80..0x82517204)
	// 82516F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82516F84: 48C911B5  bl 0x831a8138
	ctx.lr = 0x82516F88;
	sub_831A8130(ctx, base);
	// 82516F88: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516F8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82516F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82516F94: 38E001FF  li r7, 0x1ff
	ctx.r[7].s64 = 511;
	// 82516F98: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82516F9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82516FA0: C00AE37C  lfs f0, -0x1c84(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82516FA4: 93CBFFF4  stw r30, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[30].u32 ) };
	// 82516FA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82516FAC: 93CBFFF8  stw r30, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[30].u32 ) };
	// 82516FB0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82516FB4: 93CBFFFC  stw r30, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[30].u32 ) };
	// 82516FB8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82516FBC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82516FC0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82516FC4: 394B0014  addi r10, r11, 0x14
	ctx.r[10].s64 = ctx.r[11].s64 + 20;
	// 82516FC8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82516FCC: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82516FD0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82516FD4: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82516FD8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82516FDC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82516FE0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82516FE4: 4200FFF8  bdnz 0x82516fdc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82516FDC; continue 'dispatch;
	}
	// 82516FE8: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82516FEC: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82516FF0: 4080FFB4  bge 0x82516fa4
	if !ctx.cr[0].lt {
	pc = 0x82516FA4; continue 'dispatch;
	}
	// 82516FF4: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82516FF8: 3C7F0001  addis r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 65536;
	// 82516FFC: 616B85EC  ori r11, r11, 0x85ec
	ctx.r[11].u64 = ctx.r[11].u64 | 34284;
	// 82517000: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82517004: 38A0017C  li r5, 0x17c
	ctx.r[5].s64 = 380;
	// 82517008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251700C: 38638600  addi r3, r3, -0x7a00
	ctx.r[3].s64 = ctx.r[3].s64 + -31232;
	// 82517010: 7FBF592E  stwx r29, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82517014: 48C911CD  bl 0x831a81e0
	ctx.lr = 0x82517018;
	sub_831A81E0(ctx, base);
	// 82517018: 3D208328  lis r9, -0x7cd8
	ctx.r[9].s64 = -2094530560;
	// 8251701C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82517020: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 82517024: 61488000  ori r8, r10, 0x8000
	ctx.r[8].u64 = ctx.r[10].u64 | 32768;
	// 82517028: 3D5F0001  addis r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 65536;
	// 8251702C: 396B8004  addi r11, r11, -0x7ffc
	ctx.r[11].s64 = ctx.r[11].s64 + -32764;
	// 82517030: 394A8008  addi r10, r10, -0x7ff8
	ctx.r[10].s64 = ctx.r[10].s64 + -32760;
	// 82517034: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82517038: 8129CEA8  lwz r9, -0x3158(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 8251703C: 7D3F412E  stwx r9, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82517040: 419A0020  beq cr6, 0x82517060
	if ctx.cr[6].eq {
	pc = 0x82517060; continue 'dispatch;
	}
	// 82517044: 7D2B5051  subf. r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82517048: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8251704C: 41820014  beq 0x82517060
	if ctx.cr[0].eq {
	pc = 0x82517060; continue 'dispatch;
	}
	// 82517050: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82517054: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82517058: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8251705C: 4200FFF8  bdnz 0x82517054
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82517054; continue 'dispatch;
	}
	// 82517060: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 82517064: 396B8328  addi r11, r11, -0x7cd8
	ctx.r[11].s64 = ctx.r[11].s64 + -31960;
	// 82517068: 48000010  b 0x82517078
	pc = 0x82517078; continue 'dispatch;
	// 8251706C: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82517070: 9BCA0001  stb r30, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 82517074: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82517078: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251707C: 4198FFF0  blt cr6, 0x8251706c
	if ctx.cr[6].lt {
	pc = 0x8251706C; continue 'dispatch;
	}
	// 82517080: 3D5F0001  addis r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 65536;
	// 82517084: 394A838C  addi r10, r10, -0x7c74
	ctx.r[10].s64 = ctx.r[10].s64 + -31860;
	// 82517088: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251708C: 40980020  bge cr6, 0x825170ac
	if !ctx.cr[6].lt {
	pc = 0x825170AC; continue 'dispatch;
	}
	// 82517090: 7D2B5051  subf. r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82517094: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82517098: 41820014  beq 0x825170ac
	if ctx.cr[0].eq {
	pc = 0x825170AC; continue 'dispatch;
	}
	// 8251709C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825170A0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 825170A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825170A8: 4200FFF8  bdnz 0x825170a0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825170A0; continue 'dispatch;
	}
	// 825170AC: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 825170B0: 396B8580  addi r11, r11, -0x7a80
	ctx.r[11].s64 = ctx.r[11].s64 + -31360;
	// 825170B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825170B8: 40980020  bge cr6, 0x825170d8
	if !ctx.cr[6].lt {
	pc = 0x825170D8; continue 'dispatch;
	}
	// 825170BC: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825170C0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 825170C4: 41820014  beq 0x825170d8
	if ctx.cr[0].eq {
	pc = 0x825170D8; continue 'dispatch;
	}
	// 825170C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825170CC: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 825170D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825170D4: 4200FFF8  bdnz 0x825170cc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825170CC; continue 'dispatch;
	}
	// 825170D8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 825170DC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 825170E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 825170E4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825170E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825170EC: 4200FFF8  bdnz 0x825170e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825170E4; continue 'dispatch;
	}
	// 825170F0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825170F4: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825170F8: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825170FC: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 82517100: 616B85A0  ori r11, r11, 0x85a0
	ctx.r[11].u64 = ctx.r[11].u64 | 34208;
	// 82517104: 614A85A8  ori r10, r10, 0x85a8
	ctx.r[10].u64 = ctx.r[10].u64 | 34216;
	// 82517108: 612985AC  ori r9, r9, 0x85ac
	ctx.r[9].u64 = ctx.r[9].u64 | 34220;
	// 8251710C: 610885B0  ori r8, r8, 0x85b0
	ctx.r[8].u64 = ctx.r[8].u64 | 34224;
	// 82517110: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 82517114: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 82517118: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 8251711C: 3CC00000  lis r6, 0
	ctx.r[6].s64 = 0;
	// 82517120: 7FDF512E  stwx r30, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82517124: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 82517128: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 8251712C: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 82517130: 7FDF412E  stwx r30, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u32) };
	// 82517134: 3F800000  lis r28, 0
	ctx.r[28].s64 = 0;
	// 82517138: 60E785E0  ori r7, r7, 0x85e0
	ctx.r[7].u64 = ctx.r[7].u64 | 34272;
	// 8251713C: 60C685E4  ori r6, r6, 0x85e4
	ctx.r[6].u64 = ctx.r[6].u64 | 34276;
	// 82517140: 606985F4  ori r9, r3, 0x85f4
	ctx.r[9].u64 = ctx.r[3].u64 | 34292;
	// 82517144: 60AB85E8  ori r11, r5, 0x85e8
	ctx.r[11].u64 = ctx.r[5].u64 | 34280;
	// 82517148: 608A85F0  ori r10, r4, 0x85f0
	ctx.r[10].u64 = ctx.r[4].u64 | 34288;
	// 8251714C: 638885F8  ori r8, r28, 0x85f8
	ctx.r[8].u64 = ctx.r[28].u64 | 34296;
	// 82517150: 7FDF392E  stwx r30, r31, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 82517154: 3F600000  lis r27, 0
	ctx.r[27].s64 = 0;
	// 82517158: 7FDF312E  stwx r30, r31, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32), ctx.r[30].u32) };
	// 8251715C: 3F400000  lis r26, 0
	ctx.r[26].s64 = 0;
	// 82517160: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 82517164: 3F200000  lis r25, 0
	ctx.r[25].s64 = 0;
	// 82517168: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 8251716C: 3F000000  lis r24, 0
	ctx.r[24].s64 = 0;
	// 82517170: 7FBF512E  stwx r29, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82517174: 3EE00000  lis r23, 0
	ctx.r[23].s64 = 0;
	// 82517178: 7FBF412E  stwx r29, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u32) };
	// 8251717C: 3EC00000  lis r22, 0
	ctx.r[22].s64 = 0;
	// 82517180: 636785FC  ori r7, r27, 0x85fc
	ctx.r[7].u64 = ctx.r[27].u64 | 34300;
	// 82517184: 634685B4  ori r6, r26, 0x85b4
	ctx.r[6].u64 = ctx.r[26].u64 | 34228;
	// 82517188: 632B85B8  ori r11, r25, 0x85b8
	ctx.r[11].u64 = ctx.r[25].u64 | 34232;
	// 8251718C: 630A85BC  ori r10, r24, 0x85bc
	ctx.r[10].u64 = ctx.r[24].u64 | 34236;
	// 82517190: 62E985C0  ori r9, r23, 0x85c0
	ctx.r[9].u64 = ctx.r[23].u64 | 34240;
	// 82517194: 62C885C4  ori r8, r22, 0x85c4
	ctx.r[8].u64 = ctx.r[22].u64 | 34244;
	// 82517198: 7FDF392E  stwx r30, r31, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8251719C: 3EA00000  lis r21, 0
	ctx.r[21].s64 = 0;
	// 825171A0: 7FBF312E  stwx r29, r31, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32), ctx.r[29].u32) };
	// 825171A4: 3E800000  lis r20, 0
	ctx.r[20].s64 = 0;
	// 825171A8: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 825171AC: 3E600000  lis r19, 0
	ctx.r[19].s64 = 0;
	// 825171B0: 7FBF512E  stwx r29, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 825171B4: 3E400000  lis r18, 0
	ctx.r[18].s64 = 0;
	// 825171B8: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 825171BC: 3E200000  lis r17, 0
	ctx.r[17].s64 = 0;
	// 825171C0: 7FBF412E  stwx r29, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u32) };
	// 825171C4: 3E000000  lis r16, 0
	ctx.r[16].s64 = 0;
	// 825171C8: 62A785C8  ori r7, r21, 0x85c8
	ctx.r[7].u64 = ctx.r[21].u64 | 34248;
	// 825171CC: 628685CC  ori r6, r20, 0x85cc
	ctx.r[6].u64 = ctx.r[20].u64 | 34252;
	// 825171D0: 626B85D0  ori r11, r19, 0x85d0
	ctx.r[11].u64 = ctx.r[19].u64 | 34256;
	// 825171D4: 624A85D4  ori r10, r18, 0x85d4
	ctx.r[10].u64 = ctx.r[18].u64 | 34260;
	// 825171D8: 622985D8  ori r9, r17, 0x85d8
	ctx.r[9].u64 = ctx.r[17].u64 | 34264;
	// 825171DC: 620885DC  ori r8, r16, 0x85dc
	ctx.r[8].u64 = ctx.r[16].u64 | 34268;
	// 825171E0: 7FDF392E  stwx r30, r31, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 825171E4: 7FBF312E  stwx r29, r31, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[6].u32), ctx.r[29].u32) };
	// 825171E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825171EC: 7FDF592E  stwx r30, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 825171F0: 7FBF512E  stwx r29, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 825171F4: 7FDF492E  stwx r30, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 825171F8: 7FDF412E  stwx r30, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u32) };
	// 825171FC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82517200: 48C90F88  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517208 size=196
    let mut pc: u32 = 0x82517208;
    'dispatch: loop {
        match pc {
            0x82517208 => {
    //   block [0x82517208..0x825172CC)
	// 82517208: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8251720C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82517210: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82517214: 81660098  lwz r11, 0x98(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(152 as u32) ) } as u64;
	// 82517218: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8251721C: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82517220: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82517224: 41820014  beq 0x82517238
	if ctx.cr[0].eq {
	pc = 0x82517238; continue 'dispatch;
	}
	// 82517228: 39440003  addi r10, r4, 3
	ctx.r[10].s64 = ctx.r[4].s64 + 3;
	// 8251722C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82517230: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82517234: 4800008C  b 0x825172c0
	pc = 0x825172C0; continue 'dispatch;
	// 82517238: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 8251723C: 81660094  lwz r11, 0x94(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(148 as u32) ) } as u64;
	// 82517240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82517244: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82517248: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8251724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82517250: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82517254: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82517258: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251725C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82517260: 419A0044  beq cr6, 0x825172a4
	if ctx.cr[6].eq {
	pc = 0x825172A4; continue 'dispatch;
	}
	// 82517264: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8251726C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82517270: 7FFE5030  slw r30, r31, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[30].u64 = 0;
	} else {
		ctx.r[30].u64 = ((ctx.r[31].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82517274: 7FDE2839  and. r30, r30, r5
	ctx.r[30].u64 = ctx.r[30].u64 & ctx.r[5].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82517278: 41820008  beq 0x82517280
	if ctx.cr[0].eq {
	pc = 0x82517280; continue 'dispatch;
	}
	// 8251727C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82517280: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82517284: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 82517288: 4198FFE8  blt cr6, 0x82517270
	if ctx.cr[6].lt {
	pc = 0x82517270; continue 'dispatch;
	}
	// 8251728C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82517290: 7C671A14  add r3, r7, r3
	ctx.r[3].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82517294: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82517298: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 8251729C: 2B090200  cmplwi cr6, r9, 0x200
	ctx.cr[6].compare_u32(ctx.r[9].u32, 512 as u32, &mut ctx.xer);
	// 825172A0: 4198FFB8  blt cr6, 0x82517258
	if ctx.cr[6].lt {
	pc = 0x82517258; continue 'dispatch;
	}
	// 825172A4: 81660098  lwz r11, 0x98(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(152 as u32) ) } as u64;
	// 825172A8: 39440003  addi r10, r4, 3
	ctx.r[10].s64 = ctx.r[4].s64 + 3;
	// 825172AC: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 825172B0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825172B4: 9BEB0014  stb r31, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u8 ) };
	// 825172B8: 81660098  lwz r11, 0x98(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(152 as u32) ) } as u64;
	// 825172BC: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 825172C0: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825172C4: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 825172C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825172D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825172D0 size=40
    let mut pc: u32 = 0x825172D0;
    'dispatch: loop {
        match pc {
            0x825172D0 => {
    //   block [0x825172D0..0x825172F8)
	// 825172D0: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 825172D4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825172D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825172DC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825172E0: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 825172E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825172E8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825172EC: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 825172F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825172F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825172F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825172F8 size=88
    let mut pc: u32 = 0x825172F8;
    'dispatch: loop {
        match pc {
            0x825172F8 => {
    //   block [0x825172F8..0x82517350)
	// 825172F8: 39202710  li r9, 0x2710
	ctx.r[9].s64 = 10000;
	// 825172FC: 7D4A4B96  divwu r10, r10, r9
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[9].u32;
	// 82517300: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82517304: 409A0034  bne cr6, 0x82517338
	if !ctx.cr[6].eq {
	pc = 0x82517338; continue 'dispatch;
	}
	// 82517308: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251730C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82517310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82517314: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82517318: 7C845030  slw r4, r4, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[4].u64 = 0;
	} else {
		ctx.r[4].u64 = ((ctx.r[4].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8251731C: 7C844039  and. r4, r4, r8
	ctx.r[4].u64 = ctx.r[4].u64 & ctx.r[8].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82517320: 41820008  beq 0x82517328
	if ctx.cr[0].eq {
	pc = 0x82517328; continue 'dispatch;
	}
	// 82517324: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82517328: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251732C: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 82517330: 4198FFE4  blt cr6, 0x82517314
	if ctx.cr[6].lt {
	pc = 0x82517314; continue 'dispatch;
	}
	// 82517334: 7C691A14  add r3, r9, r3
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 82517338: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8251733C: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 82517340: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82517344: 2B060200  cmplwi cr6, r6, 0x200
	ctx.cr[6].compare_u32(ctx.r[6].u32, 512 as u32, &mut ctx.xer);
	// 82517348: 4198FFA4  blt cr6, 0x825172ec
	if ctx.cr[6].lt {
		sub_825172D0(ctx, base);
		return;
	}
	// 8251734C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517350 size=128
    let mut pc: u32 = 0x82517350;
    'dispatch: loop {
        match pc {
            0x82517350 => {
    //   block [0x82517350..0x825173D0)
	// 82517350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251735C: 4BFFFEAD  bl 0x82517208
	ctx.lr = 0x82517360;
	sub_82517208(ctx, base);
	// 82517360: 2B030078  cmplwi cr6, r3, 0x78
	ctx.cr[6].compare_u32(ctx.r[3].u32, 120 as u32, &mut ctx.xer);
	// 82517364: 4198000C  blt cr6, 0x82517370
	if ctx.cr[6].lt {
	pc = 0x82517370; continue 'dispatch;
	}
	// 82517368: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 8251736C: 48000054  b 0x825173c0
	pc = 0x825173C0; continue 'dispatch;
	// 82517370: 2B030050  cmplwi cr6, r3, 0x50
	ctx.cr[6].compare_u32(ctx.r[3].u32, 80 as u32, &mut ctx.xer);
	// 82517374: 4198000C  blt cr6, 0x82517380
	if ctx.cr[6].lt {
	pc = 0x82517380; continue 'dispatch;
	}
	// 82517378: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8251737C: 48000044  b 0x825173c0
	pc = 0x825173C0; continue 'dispatch;
	// 82517380: 2B03003C  cmplwi cr6, r3, 0x3c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 60 as u32, &mut ctx.xer);
	// 82517384: 4198000C  blt cr6, 0x82517390
	if ctx.cr[6].lt {
	pc = 0x82517390; continue 'dispatch;
	}
	// 82517388: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8251738C: 48000034  b 0x825173c0
	pc = 0x825173C0; continue 'dispatch;
	// 82517390: 2B03002D  cmplwi cr6, r3, 0x2d
	ctx.cr[6].compare_u32(ctx.r[3].u32, 45 as u32, &mut ctx.xer);
	// 82517394: 4198000C  blt cr6, 0x825173a0
	if ctx.cr[6].lt {
	pc = 0x825173A0; continue 'dispatch;
	}
	// 82517398: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8251739C: 48000024  b 0x825173c0
	pc = 0x825173C0; continue 'dispatch;
	// 825173A0: 2B03001E  cmplwi cr6, r3, 0x1e
	ctx.cr[6].compare_u32(ctx.r[3].u32, 30 as u32, &mut ctx.xer);
	// 825173A4: 4198000C  blt cr6, 0x825173b0
	if ctx.cr[6].lt {
	pc = 0x825173B0; continue 'dispatch;
	}
	// 825173A8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 825173AC: 48000014  b 0x825173c0
	pc = 0x825173C0; continue 'dispatch;
	// 825173B0: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 825173B4: 7D6B1810  subfc r11, r11, r3
	ctx.xer.ca = ctx.r[3].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 825173B8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 825173BC: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 825173C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825173C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825173C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825173CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825173D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825173D0 size=28
    let mut pc: u32 = 0x825173D0;
    'dispatch: loop {
        match pc {
            0x825173D0 => {
    //   block [0x825173D0..0x825173EC)
	// 825173D0: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825173D4: 3D4B0001  addis r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 65536;
	// 825173D8: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825173DC: 394A8008  addi r10, r10, -0x7ff8
	ctx.r[10].s64 = ctx.r[10].s64 + -32760;
	// 825173E0: 396B8004  addi r11, r11, -0x7ffc
	ctx.r[11].s64 = ctx.r[11].s64 + -32764;
	// 825173E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825173E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825173EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825173EC size=12
    let mut pc: u32 = 0x825173EC;
    'dispatch: loop {
        match pc {
            0x825173EC => {
    //   block [0x825173EC..0x825173F8)
	// 825173EC: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825173F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825173F4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825173F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825173F8 size=20
    let mut pc: u32 = 0x825173F8;
    'dispatch: loop {
        match pc {
            0x825173F8 => {
    //   block [0x825173F8..0x8251740C)
	// 825173F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 825173FC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82517400: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82517404: 4200FFF8  bdnz 0x825173fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825173FC; continue 'dispatch;
	}
	// 82517408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517410 size=1484
    let mut pc: u32 = 0x82517410;
    'dispatch: loop {
        match pc {
            0x82517410 => {
    //   block [0x82517410..0x825179DC)
	// 82517410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517414: 48C90D55  bl 0x831a8168
	ctx.lr = 0x82517418;
	sub_831A8130(ctx, base);
	// 82517418: 9421FC70  stwu r1, -0x390(r1)
	ea = ctx.r[1].u32.wrapping_add(-912 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251741C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82517420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82517424: 3BAB7AF0  addi r29, r11, 0x7af0
	ctx.r[29].s64 = ctx.r[11].s64 + 31472;
	// 82517428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251742C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517430: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82517434: 488DC5D5  bl 0x82df3a08
	ctx.lr = 0x82517438;
	sub_82DF3A08(ctx, base);
	// 82517438: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251743C: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 82517440: 388B201C  addi r4, r11, 0x201c
	ctx.r[4].s64 = ctx.r[11].s64 + 8220;
	// 82517444: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517448: 3B9C8000  addi r28, r28, -0x8000
	ctx.r[28].s64 = ctx.r[28].s64 + -32768;
	// 8251744C: 488DC5BD  bl 0x82df3a08
	ctx.lr = 0x82517450;
	sub_82DF3A08(ctx, base);
	// 82517450: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517454: 38610260  addi r3, r1, 0x260
	ctx.r[3].s64 = ctx.r[1].s64 + 608;
	// 82517458: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251745C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82517460: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 82517464: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82517468: 4808BFB1  bl 0x825a3418
	ctx.lr = 0x8251746C;
	sub_825A3418(ctx, base);
	// 8251746C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517474: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517478: 4808B899  bl 0x825a2d10
	ctx.lr = 0x8251747C;
	sub_825A2D10(ctx, base);
	// 8251747C: 38610298  addi r3, r1, 0x298
	ctx.r[3].s64 = ctx.r[1].s64 + 664;
	// 82517480: 488DBFA9  bl 0x82df3428
	ctx.lr = 0x82517484;
	sub_82DF3428(ctx, base);
	// 82517484: 38610278  addi r3, r1, 0x278
	ctx.r[3].s64 = ctx.r[1].s64 + 632;
	// 82517488: 4BDB1831  bl 0x822c8cb8
	ctx.lr = 0x8251748C;
	sub_822C8CB8(ctx, base);
	// 8251748C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517490: 488DBF99  bl 0x82df3428
	ctx.lr = 0x82517494;
	sub_82DF3428(ctx, base);
	// 82517494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517498: 488DBF91  bl 0x82df3428
	ctx.lr = 0x8251749C;
	sub_82DF3428(ctx, base);
	// 8251749C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825174A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825174A4: 388B2010  addi r4, r11, 0x2010
	ctx.r[4].s64 = ctx.r[11].s64 + 8208;
	// 825174A8: 488DC561  bl 0x82df3a08
	ctx.lr = 0x825174AC;
	sub_82DF3A08(ctx, base);
	// 825174AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825174B0: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 825174B4: 388B1FFC  addi r4, r11, 0x1ffc
	ctx.r[4].s64 = ctx.r[11].s64 + 8188;
	// 825174B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825174BC: 3B9C8004  addi r28, r28, -0x7ffc
	ctx.r[28].s64 = ctx.r[28].s64 + -32764;
	// 825174C0: 488DC549  bl 0x82df3a08
	ctx.lr = 0x825174C4;
	sub_82DF3A08(ctx, base);
	// 825174C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825174C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825174CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825174D0: 4808BB81  bl 0x825a3050
	ctx.lr = 0x825174D4;
	sub_825A3050(ctx, base);
	// 825174D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825174D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825174DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825174E0: 4808B9F9  bl 0x825a2ed8
	ctx.lr = 0x825174E4;
	sub_825A2ED8(ctx, base);
	// 825174E4: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825174E8: 488DBF41  bl 0x82df3428
	ctx.lr = 0x825174EC;
	sub_82DF3428(ctx, base);
	// 825174EC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825174F0: 4BDB17C9  bl 0x822c8cb8
	ctx.lr = 0x825174F4;
	sub_822C8CB8(ctx, base);
	// 825174F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825174F8: 488DBF31  bl 0x82df3428
	ctx.lr = 0x825174FC;
	sub_82DF3428(ctx, base);
	// 825174FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517500: 488DBF29  bl 0x82df3428
	ctx.lr = 0x82517504;
	sub_82DF3428(ctx, base);
	// 82517504: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251750C: 388B1FF0  addi r4, r11, 0x1ff0
	ctx.r[4].s64 = ctx.r[11].s64 + 8176;
	// 82517510: 488DC4F9  bl 0x82df3a08
	ctx.lr = 0x82517514;
	sub_82DF3A08(ctx, base);
	// 82517514: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517518: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 8251751C: 388B1FE0  addi r4, r11, 0x1fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 8160;
	// 82517520: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517524: 3B9C8005  addi r28, r28, -0x7ffb
	ctx.r[28].s64 = ctx.r[28].s64 + -32763;
	// 82517528: 488DC4E1  bl 0x82df3a08
	ctx.lr = 0x8251752C;
	sub_82DF3A08(ctx, base);
	// 8251752C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517530: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82517534: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82517538: 4808BB19  bl 0x825a3050
	ctx.lr = 0x8251753C;
	sub_825A3050(ctx, base);
	// 8251753C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517540: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517548: 4808B991  bl 0x825a2ed8
	ctx.lr = 0x8251754C;
	sub_825A2ED8(ctx, base);
	// 8251754C: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82517550: 488DBED9  bl 0x82df3428
	ctx.lr = 0x82517554;
	sub_82DF3428(ctx, base);
	// 82517554: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82517558: 4BDB1761  bl 0x822c8cb8
	ctx.lr = 0x8251755C;
	sub_822C8CB8(ctx, base);
	// 8251755C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517560: 488DBEC9  bl 0x82df3428
	ctx.lr = 0x82517564;
	sub_82DF3428(ctx, base);
	// 82517564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517568: 488DBEC1  bl 0x82df3428
	ctx.lr = 0x8251756C;
	sub_82DF3428(ctx, base);
	// 8251756C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517574: 388B1FD4  addi r4, r11, 0x1fd4
	ctx.r[4].s64 = ctx.r[11].s64 + 8148;
	// 82517578: 488DC491  bl 0x82df3a08
	ctx.lr = 0x8251757C;
	sub_82DF3A08(ctx, base);
	// 8251757C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517580: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 82517584: 388B1FC0  addi r4, r11, 0x1fc0
	ctx.r[4].s64 = ctx.r[11].s64 + 8128;
	// 82517588: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251758C: 3B9C8006  addi r28, r28, -0x7ffa
	ctx.r[28].s64 = ctx.r[28].s64 + -32762;
	// 82517590: 488DC479  bl 0x82df3a08
	ctx.lr = 0x82517594;
	sub_82DF3A08(ctx, base);
	// 82517594: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517598: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251759C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825175A0: 4808BAB1  bl 0x825a3050
	ctx.lr = 0x825175A4;
	sub_825A3050(ctx, base);
	// 825175A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825175A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825175AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825175B0: 4808B929  bl 0x825a2ed8
	ctx.lr = 0x825175B4;
	sub_825A2ED8(ctx, base);
	// 825175B4: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 825175B8: 488DBE71  bl 0x82df3428
	ctx.lr = 0x825175BC;
	sub_82DF3428(ctx, base);
	// 825175BC: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 825175C0: 4BDB16F9  bl 0x822c8cb8
	ctx.lr = 0x825175C4;
	sub_822C8CB8(ctx, base);
	// 825175C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825175C8: 488DBE61  bl 0x82df3428
	ctx.lr = 0x825175CC;
	sub_82DF3428(ctx, base);
	// 825175CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825175D0: 488DBE59  bl 0x82df3428
	ctx.lr = 0x825175D4;
	sub_82DF3428(ctx, base);
	// 825175D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825175D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825175DC: 388B1FAC  addi r4, r11, 0x1fac
	ctx.r[4].s64 = ctx.r[11].s64 + 8108;
	// 825175E0: 488DC429  bl 0x82df3a08
	ctx.lr = 0x825175E4;
	sub_82DF3A08(ctx, base);
	// 825175E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825175E8: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 825175EC: 388B1F8C  addi r4, r11, 0x1f8c
	ctx.r[4].s64 = ctx.r[11].s64 + 8076;
	// 825175F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825175F4: 3B9C8007  addi r28, r28, -0x7ff9
	ctx.r[28].s64 = ctx.r[28].s64 + -32761;
	// 825175F8: 488DC411  bl 0x82df3a08
	ctx.lr = 0x825175FC;
	sub_82DF3A08(ctx, base);
	// 825175FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517600: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82517604: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82517608: 4808BA49  bl 0x825a3050
	ctx.lr = 0x8251760C;
	sub_825A3050(ctx, base);
	// 8251760C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517614: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517618: 4808B8C1  bl 0x825a2ed8
	ctx.lr = 0x8251761C;
	sub_825A2ED8(ctx, base);
	// 8251761C: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 82517620: 488DBE09  bl 0x82df3428
	ctx.lr = 0x82517624;
	sub_82DF3428(ctx, base);
	// 82517624: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 82517628: 4BDB1691  bl 0x822c8cb8
	ctx.lr = 0x8251762C;
	sub_822C8CB8(ctx, base);
	// 8251762C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517630: 488DBDF9  bl 0x82df3428
	ctx.lr = 0x82517634;
	sub_82DF3428(ctx, base);
	// 82517634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517638: 488DBDF1  bl 0x82df3428
	ctx.lr = 0x8251763C;
	sub_82DF3428(ctx, base);
	// 8251763C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82517640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517644: 488DC3C5  bl 0x82df3a08
	ctx.lr = 0x82517648;
	sub_82DF3A08(ctx, base);
	// 82517648: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251764C: 3F9F0001  addis r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 65536;
	// 82517650: 388B1F78  addi r4, r11, 0x1f78
	ctx.r[4].s64 = ctx.r[11].s64 + 8056;
	// 82517654: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517658: 3B9C85AC  addi r28, r28, -0x7a54
	ctx.r[28].s64 = ctx.r[28].s64 + -31316;
	// 8251765C: 488DC3AD  bl 0x82df3a08
	ctx.lr = 0x82517660;
	sub_82DF3A08(ctx, base);
	// 82517660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517664: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 82517668: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251766C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82517670: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 82517674: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82517678: 4808BDA1  bl 0x825a3418
	ctx.lr = 0x8251767C;
	sub_825A3418(ctx, base);
	// 8251767C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517680: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517684: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517688: 4808B689  bl 0x825a2d10
	ctx.lr = 0x8251768C;
	sub_825A2D10(ctx, base);
	// 8251768C: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 82517690: 488DBD99  bl 0x82df3428
	ctx.lr = 0x82517694;
	sub_82DF3428(ctx, base);
	// 82517694: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 82517698: 4BDB1621  bl 0x822c8cb8
	ctx.lr = 0x8251769C;
	sub_822C8CB8(ctx, base);
	// 8251769C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825176A0: 488DBD89  bl 0x82df3428
	ctx.lr = 0x825176A4;
	sub_82DF3428(ctx, base);
	// 825176A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825176A8: 488DBD81  bl 0x82df3428
	ctx.lr = 0x825176AC;
	sub_82DF3428(ctx, base);
	// 825176AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825176B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825176B4: 488DC355  bl 0x82df3a08
	ctx.lr = 0x825176B8;
	sub_82DF3A08(ctx, base);
	// 825176B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825176BC: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 825176C0: 388B1F64  addi r4, r11, 0x1f64
	ctx.r[4].s64 = ctx.r[11].s64 + 8036;
	// 825176C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825176C8: 3BBD85B0  addi r29, r29, -0x7a50
	ctx.r[29].s64 = ctx.r[29].s64 + -31312;
	// 825176CC: 488DC33D  bl 0x82df3a08
	ctx.lr = 0x825176D0;
	sub_82DF3A08(ctx, base);
	// 825176D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825176D4: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 825176D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825176DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825176E0: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 825176E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825176E8: 4808BD31  bl 0x825a3418
	ctx.lr = 0x825176EC;
	sub_825A3418(ctx, base);
	// 825176EC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825176F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825176F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825176F8: 4808B619  bl 0x825a2d10
	ctx.lr = 0x825176FC;
	sub_825A2D10(ctx, base);
	// 825176FC: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 82517700: 488DBD29  bl 0x82df3428
	ctx.lr = 0x82517704;
	sub_82DF3428(ctx, base);
	// 82517704: 386101F8  addi r3, r1, 0x1f8
	ctx.r[3].s64 = ctx.r[1].s64 + 504;
	// 82517708: 4BDB15B1  bl 0x822c8cb8
	ctx.lr = 0x8251770C;
	sub_822C8CB8(ctx, base);
	// 8251770C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517710: 488DBD19  bl 0x82df3428
	ctx.lr = 0x82517714;
	sub_82DF3428(ctx, base);
	// 82517714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517718: 488DBD11  bl 0x82df3428
	ctx.lr = 0x8251771C;
	sub_82DF3428(ctx, base);
	// 8251771C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517720: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517724: 388B1F5C  addi r4, r11, 0x1f5c
	ctx.r[4].s64 = ctx.r[11].s64 + 8028;
	// 82517728: 488DC2E1  bl 0x82df3a08
	ctx.lr = 0x8251772C;
	sub_82DF3A08(ctx, base);
	// 8251772C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517730: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 82517734: 388B1F50  addi r4, r11, 0x1f50
	ctx.r[4].s64 = ctx.r[11].s64 + 8016;
	// 82517738: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251773C: 3BBD85DC  addi r29, r29, -0x7a24
	ctx.r[29].s64 = ctx.r[29].s64 + -31268;
	// 82517740: 488DC2C9  bl 0x82df3a08
	ctx.lr = 0x82517744;
	sub_82DF3A08(ctx, base);
	// 82517744: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517748: 386102E0  addi r3, r1, 0x2e0
	ctx.r[3].s64 = ctx.r[1].s64 + 736;
	// 8251774C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82517750: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82517754: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 82517758: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8251775C: 4808BCBD  bl 0x825a3418
	ctx.lr = 0x82517760;
	sub_825A3418(ctx, base);
	// 82517760: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517768: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251776C: 4808B5A5  bl 0x825a2d10
	ctx.lr = 0x82517770;
	sub_825A2D10(ctx, base);
	// 82517770: 38610318  addi r3, r1, 0x318
	ctx.r[3].s64 = ctx.r[1].s64 + 792;
	// 82517774: 488DBCB5  bl 0x82df3428
	ctx.lr = 0x82517778;
	sub_82DF3428(ctx, base);
	// 82517778: 386102F8  addi r3, r1, 0x2f8
	ctx.r[3].s64 = ctx.r[1].s64 + 760;
	// 8251777C: 4BDB153D  bl 0x822c8cb8
	ctx.lr = 0x82517780;
	sub_822C8CB8(ctx, base);
	// 82517780: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517784: 488DBCA5  bl 0x82df3428
	ctx.lr = 0x82517788;
	sub_82DF3428(ctx, base);
	// 82517788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251778C: 488DBC9D  bl 0x82df3428
	ctx.lr = 0x82517790;
	sub_82DF3428(ctx, base);
	// 82517790: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517798: 388B1F40  addi r4, r11, 0x1f40
	ctx.r[4].s64 = ctx.r[11].s64 + 8000;
	// 8251779C: 488DC26D  bl 0x82df3a08
	ctx.lr = 0x825177A0;
	sub_82DF3A08(ctx, base);
	// 825177A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825177A4: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 825177A8: 388B1F34  addi r4, r11, 0x1f34
	ctx.r[4].s64 = ctx.r[11].s64 + 7988;
	// 825177AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825177B0: 3BBD85B4  addi r29, r29, -0x7a4c
	ctx.r[29].s64 = ctx.r[29].s64 + -31308;
	// 825177B4: 488DC255  bl 0x82df3a08
	ctx.lr = 0x825177B8;
	sub_82DF3A08(ctx, base);
	// 825177B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825177BC: 38610160  addi r3, r1, 0x160
	ctx.r[3].s64 = ctx.r[1].s64 + 352;
	// 825177C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825177C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825177C8: 38E00063  li r7, 0x63
	ctx.r[7].s64 = 99;
	// 825177CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825177D0: 4808BC49  bl 0x825a3418
	ctx.lr = 0x825177D4;
	sub_825A3418(ctx, base);
	// 825177D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825177D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825177DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825177E0: 4808B531  bl 0x825a2d10
	ctx.lr = 0x825177E4;
	sub_825A2D10(ctx, base);
	// 825177E4: 38610198  addi r3, r1, 0x198
	ctx.r[3].s64 = ctx.r[1].s64 + 408;
	// 825177E8: 488DBC41  bl 0x82df3428
	ctx.lr = 0x825177EC;
	sub_82DF3428(ctx, base);
	// 825177EC: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 825177F0: 4BDB14C9  bl 0x822c8cb8
	ctx.lr = 0x825177F4;
	sub_822C8CB8(ctx, base);
	// 825177F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825177F8: 488DBC31  bl 0x82df3428
	ctx.lr = 0x825177FC;
	sub_82DF3428(ctx, base);
	// 825177FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517800: 488DBC29  bl 0x82df3428
	ctx.lr = 0x82517804;
	sub_82DF3428(ctx, base);
	// 82517804: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251780C: 388B1F24  addi r4, r11, 0x1f24
	ctx.r[4].s64 = ctx.r[11].s64 + 7972;
	// 82517810: 488DC1F9  bl 0x82df3a08
	ctx.lr = 0x82517814;
	sub_82DF3A08(ctx, base);
	// 82517814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517818: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 8251781C: 388B1F18  addi r4, r11, 0x1f18
	ctx.r[4].s64 = ctx.r[11].s64 + 7960;
	// 82517820: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517824: 3BBD85BC  addi r29, r29, -0x7a44
	ctx.r[29].s64 = ctx.r[29].s64 + -31300;
	// 82517828: 488DC1E1  bl 0x82df3a08
	ctx.lr = 0x8251782C;
	sub_82DF3A08(ctx, base);
	// 8251782C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517830: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 82517834: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82517838: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251783C: 38E00063  li r7, 0x63
	ctx.r[7].s64 = 99;
	// 82517840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82517844: 4808BBD5  bl 0x825a3418
	ctx.lr = 0x82517848;
	sub_825A3418(ctx, base);
	// 82517848: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8251784C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517850: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517854: 4808B4BD  bl 0x825a2d10
	ctx.lr = 0x82517858;
	sub_825A2D10(ctx, base);
	// 82517858: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 8251785C: 488DBBCD  bl 0x82df3428
	ctx.lr = 0x82517860;
	sub_82DF3428(ctx, base);
	// 82517860: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 82517864: 4BDB1455  bl 0x822c8cb8
	ctx.lr = 0x82517868;
	sub_822C8CB8(ctx, base);
	// 82517868: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251786C: 488DBBBD  bl 0x82df3428
	ctx.lr = 0x82517870;
	sub_82DF3428(ctx, base);
	// 82517870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517874: 488DBBB5  bl 0x82df3428
	ctx.lr = 0x82517878;
	sub_82DF3428(ctx, base);
	// 82517878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251787C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517880: 388B1F08  addi r4, r11, 0x1f08
	ctx.r[4].s64 = ctx.r[11].s64 + 7944;
	// 82517884: 488DC185  bl 0x82df3a08
	ctx.lr = 0x82517888;
	sub_82DF3A08(ctx, base);
	// 82517888: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251788C: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 82517890: 388B1EFC  addi r4, r11, 0x1efc
	ctx.r[4].s64 = ctx.r[11].s64 + 7932;
	// 82517894: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517898: 3BBD85C4  addi r29, r29, -0x7a3c
	ctx.r[29].s64 = ctx.r[29].s64 + -31292;
	// 8251789C: 488DC16D  bl 0x82df3a08
	ctx.lr = 0x825178A0;
	sub_82DF3A08(ctx, base);
	// 825178A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825178A4: 38610220  addi r3, r1, 0x220
	ctx.r[3].s64 = ctx.r[1].s64 + 544;
	// 825178A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825178AC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825178B0: 38E00063  li r7, 0x63
	ctx.r[7].s64 = 99;
	// 825178B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825178B8: 4808BB61  bl 0x825a3418
	ctx.lr = 0x825178BC;
	sub_825A3418(ctx, base);
	// 825178BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825178C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825178C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825178C8: 4808B449  bl 0x825a2d10
	ctx.lr = 0x825178CC;
	sub_825A2D10(ctx, base);
	// 825178CC: 38610258  addi r3, r1, 0x258
	ctx.r[3].s64 = ctx.r[1].s64 + 600;
	// 825178D0: 488DBB59  bl 0x82df3428
	ctx.lr = 0x825178D4;
	sub_82DF3428(ctx, base);
	// 825178D4: 38610238  addi r3, r1, 0x238
	ctx.r[3].s64 = ctx.r[1].s64 + 568;
	// 825178D8: 4BDB13E1  bl 0x822c8cb8
	ctx.lr = 0x825178DC;
	sub_822C8CB8(ctx, base);
	// 825178DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825178E0: 488DBB49  bl 0x82df3428
	ctx.lr = 0x825178E4;
	sub_82DF3428(ctx, base);
	// 825178E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825178E8: 488DBB41  bl 0x82df3428
	ctx.lr = 0x825178EC;
	sub_82DF3428(ctx, base);
	// 825178EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825178F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825178F4: 388B1EE8  addi r4, r11, 0x1ee8
	ctx.r[4].s64 = ctx.r[11].s64 + 7912;
	// 825178F8: 488DC111  bl 0x82df3a08
	ctx.lr = 0x825178FC;
	sub_82DF3A08(ctx, base);
	// 825178FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517900: 3FBF0001  addis r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 65536;
	// 82517904: 388B1ED8  addi r4, r11, 0x1ed8
	ctx.r[4].s64 = ctx.r[11].s64 + 7896;
	// 82517908: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251790C: 3BBD85CC  addi r29, r29, -0x7a34
	ctx.r[29].s64 = ctx.r[29].s64 + -31284;
	// 82517910: 488DC0F9  bl 0x82df3a08
	ctx.lr = 0x82517914;
	sub_82DF3A08(ctx, base);
	// 82517914: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82517918: 386102A0  addi r3, r1, 0x2a0
	ctx.r[3].s64 = ctx.r[1].s64 + 672;
	// 8251791C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82517920: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82517924: 38E00063  li r7, 0x63
	ctx.r[7].s64 = 99;
	// 82517928: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251792C: 4808BAED  bl 0x825a3418
	ctx.lr = 0x82517930;
	sub_825A3418(ctx, base);
	// 82517930: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517938: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251793C: 4808B3D5  bl 0x825a2d10
	ctx.lr = 0x82517940;
	sub_825A2D10(ctx, base);
	// 82517940: 386102D8  addi r3, r1, 0x2d8
	ctx.r[3].s64 = ctx.r[1].s64 + 728;
	// 82517944: 488DBAE5  bl 0x82df3428
	ctx.lr = 0x82517948;
	sub_82DF3428(ctx, base);
	// 82517948: 386102B8  addi r3, r1, 0x2b8
	ctx.r[3].s64 = ctx.r[1].s64 + 696;
	// 8251794C: 4BDB136D  bl 0x822c8cb8
	ctx.lr = 0x82517950;
	sub_822C8CB8(ctx, base);
	// 82517950: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517954: 488DBAD5  bl 0x82df3428
	ctx.lr = 0x82517958;
	sub_82DF3428(ctx, base);
	// 82517958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251795C: 488DBACD  bl 0x82df3428
	ctx.lr = 0x82517960;
	sub_82DF3428(ctx, base);
	// 82517960: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517968: 388B1EC8  addi r4, r11, 0x1ec8
	ctx.r[4].s64 = ctx.r[11].s64 + 7880;
	// 8251796C: 488DC09D  bl 0x82df3a08
	ctx.lr = 0x82517970;
	sub_82DF3A08(ctx, base);
	// 82517970: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517974: 3FFF0001  addis r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 65536;
	// 82517978: 388B1EBC  addi r4, r11, 0x1ebc
	ctx.r[4].s64 = ctx.r[11].s64 + 7868;
	// 8251797C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517980: 3BFF85D4  addi r31, r31, -0x7a2c
	ctx.r[31].s64 = ctx.r[31].s64 + -31276;
	// 82517984: 488DC085  bl 0x82df3a08
	ctx.lr = 0x82517988;
	sub_82DF3A08(ctx, base);
	// 82517988: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251798C: 38610320  addi r3, r1, 0x320
	ctx.r[3].s64 = ctx.r[1].s64 + 800;
	// 82517990: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82517994: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82517998: 38E00063  li r7, 0x63
	ctx.r[7].s64 = 99;
	// 8251799C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825179A0: 4808BA79  bl 0x825a3418
	ctx.lr = 0x825179A4;
	sub_825A3418(ctx, base);
	// 825179A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825179A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825179AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825179B0: 4808B361  bl 0x825a2d10
	ctx.lr = 0x825179B4;
	sub_825A2D10(ctx, base);
	// 825179B4: 38610358  addi r3, r1, 0x358
	ctx.r[3].s64 = ctx.r[1].s64 + 856;
	// 825179B8: 488DBA71  bl 0x82df3428
	ctx.lr = 0x825179BC;
	sub_82DF3428(ctx, base);
	// 825179BC: 38610338  addi r3, r1, 0x338
	ctx.r[3].s64 = ctx.r[1].s64 + 824;
	// 825179C0: 4BDB12F9  bl 0x822c8cb8
	ctx.lr = 0x825179C4;
	sub_822C8CB8(ctx, base);
	// 825179C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825179C8: 488DBA61  bl 0x82df3428
	ctx.lr = 0x825179CC;
	sub_82DF3428(ctx, base);
	// 825179CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825179D0: 488DBA59  bl 0x82df3428
	ctx.lr = 0x825179D4;
	sub_82DF3428(ctx, base);
	// 825179D4: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 825179D8: 48C907E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825179E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825179E0 size=808
    let mut pc: u32 = 0x825179E0;
    'dispatch: loop {
        match pc {
            0x825179E0 => {
    //   block [0x825179E0..0x82517D08)
	// 825179E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825179E4: 48C90785  bl 0x831a8168
	ctx.lr = 0x825179E8;
	sub_831A8130(ctx, base);
	// 825179E8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825179EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825179F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825179F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825179F8: 388B20F0  addi r4, r11, 0x20f0
	ctx.r[4].s64 = ctx.r[11].s64 + 8432;
	// 825179FC: 488DC00D  bl 0x82df3a08
	ctx.lr = 0x82517A00;
	sub_82DF3A08(ctx, base);
	// 82517A00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82517A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517A08: 388BD6B0  addi r4, r11, -0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + -10576;
	// 82517A0C: 488DBFFD  bl 0x82df3a08
	ctx.lr = 0x82517A10;
	sub_82DF3A08(ctx, base);
	// 82517A10: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82517A14: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82517A18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82517A1C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82517A20: 480B1179  bl 0x825c8b98
	ctx.lr = 0x82517A24;
	sub_825C8B98(ctx, base);
	// 82517A24: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82517A28: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517A30: 419A000C  beq cr6, 0x82517a3c
	if ctx.cr[6].eq {
	pc = 0x82517A3C; continue 'dispatch;
	}
	// 82517A34: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82517A38: 4BDA8E59  bl 0x822c0890
	ctx.lr = 0x82517A3C;
	sub_822C0890(ctx, base);
	// 82517A3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517A40: 488DB9E9  bl 0x82df3428
	ctx.lr = 0x82517A44;
	sub_82DF3428(ctx, base);
	// 82517A44: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517A48: 488DB9E1  bl 0x82df3428
	ctx.lr = 0x82517A4C;
	sub_82DF3428(ctx, base);
	// 82517A4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82517A50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82517A54: 388B6DE0  addi r4, r11, 0x6de0
	ctx.r[4].s64 = ctx.r[11].s64 + 28128;
	// 82517A58: 488DBFB1  bl 0x82df3a08
	ctx.lr = 0x82517A5C;
	sub_82DF3A08(ctx, base);
	// 82517A5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82517A60: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82517A64: 388B6DD8  addi r4, r11, 0x6dd8
	ctx.r[4].s64 = ctx.r[11].s64 + 28120;
	// 82517A68: 488DBFA1  bl 0x82df3a08
	ctx.lr = 0x82517A6C;
	sub_82DF3A08(ctx, base);
	// 82517A6C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82517A70: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82517A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517A78: 480B2A89  bl 0x825ca500
	ctx.lr = 0x82517A7C;
	sub_825CA500(ctx, base);
	// 82517A7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82517A80: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82517A84: 488DB9A5  bl 0x82df3428
	ctx.lr = 0x82517A88;
	sub_82DF3428(ctx, base);
	// 82517A88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82517A8C: 488DB99D  bl 0x82df3428
	ctx.lr = 0x82517A90;
	sub_82DF3428(ctx, base);
	// 82517A90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517A94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82517A98: 388B20E0  addi r4, r11, 0x20e0
	ctx.r[4].s64 = ctx.r[11].s64 + 8416;
	// 82517A9C: 488DBF6D  bl 0x82df3a08
	ctx.lr = 0x82517AA0;
	sub_82DF3A08(ctx, base);
	// 82517AA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517AA4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82517AA8: 388B20D4  addi r4, r11, 0x20d4
	ctx.r[4].s64 = ctx.r[11].s64 + 8404;
	// 82517AAC: 488DBF5D  bl 0x82df3a08
	ctx.lr = 0x82517AB0;
	sub_82DF3A08(ctx, base);
	// 82517AB0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82517AB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82517AB8: 3BEBCEA8  addi r31, r11, -0x3158
	ctx.r[31].s64 = ctx.r[11].s64 + -12632;
	// 82517ABC: 38E00064  li r7, 0x64
	ctx.r[7].s64 = 100;
	// 82517AC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82517AC4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82517AC8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82517ACC: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82517AD0: 480960C1  bl 0x825adb90
	ctx.lr = 0x82517AD4;
	sub_825ADB90(ctx, base);
	// 82517AD4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517AD8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82517ADC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82517AE0: 4808A061  bl 0x825a1b40
	ctx.lr = 0x82517AE4;
	sub_825A1B40(ctx, base);
	// 82517AE4: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 82517AE8: 488DB941  bl 0x82df3428
	ctx.lr = 0x82517AEC;
	sub_82DF3428(ctx, base);
	// 82517AEC: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 82517AF0: 4BDB11C9  bl 0x822c8cb8
	ctx.lr = 0x82517AF4;
	sub_822C8CB8(ctx, base);
	// 82517AF4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82517AF8: 488DB931  bl 0x82df3428
	ctx.lr = 0x82517AFC;
	sub_82DF3428(ctx, base);
	// 82517AFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82517B00: 488DB929  bl 0x82df3428
	ctx.lr = 0x82517B04;
	sub_82DF3428(ctx, base);
	// 82517B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517B08: 480B21C9  bl 0x825c9cd0
	ctx.lr = 0x82517B0C;
	sub_825C9CD0(ctx, base);
	// 82517B0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517B10: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82517B14: 388B20CC  addi r4, r11, 0x20cc
	ctx.r[4].s64 = ctx.r[11].s64 + 8396;
	// 82517B18: 488DBEF1  bl 0x82df3a08
	ctx.lr = 0x82517B1C;
	sub_82DF3A08(ctx, base);
	// 82517B1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517B20: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82517B24: 388B20C0  addi r4, r11, 0x20c0
	ctx.r[4].s64 = ctx.r[11].s64 + 8384;
	// 82517B28: 488DBEE1  bl 0x82df3a08
	ctx.lr = 0x82517B2C;
	sub_82DF3A08(ctx, base);
	// 82517B2C: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82517B30: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82517B34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82517B38: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82517B3C: 480B105D  bl 0x825c8b98
	ctx.lr = 0x82517B40;
	sub_825C8B98(ctx, base);
	// 82517B40: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82517B44: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517B4C: 419A000C  beq cr6, 0x82517b58
	if ctx.cr[6].eq {
	pc = 0x82517B58; continue 'dispatch;
	}
	// 82517B50: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82517B54: 4BDA8D3D  bl 0x822c0890
	ctx.lr = 0x82517B58;
	sub_822C0890(ctx, base);
	// 82517B58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82517B5C: 488DB8CD  bl 0x82df3428
	ctx.lr = 0x82517B60;
	sub_82DF3428(ctx, base);
	// 82517B60: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82517B64: 488DB8C5  bl 0x82df3428
	ctx.lr = 0x82517B68;
	sub_82DF3428(ctx, base);
	// 82517B68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517B6C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82517B70: 388B20B8  addi r4, r11, 0x20b8
	ctx.r[4].s64 = ctx.r[11].s64 + 8376;
	// 82517B74: 488DBE95  bl 0x82df3a08
	ctx.lr = 0x82517B78;
	sub_82DF3A08(ctx, base);
	// 82517B78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517B7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82517B80: 388B20AC  addi r4, r11, 0x20ac
	ctx.r[4].s64 = ctx.r[11].s64 + 8364;
	// 82517B84: 488DBE85  bl 0x82df3a08
	ctx.lr = 0x82517B88;
	sub_82DF3A08(ctx, base);
	// 82517B88: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82517B8C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82517B90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517B94: 480B296D  bl 0x825ca500
	ctx.lr = 0x82517B98;
	sub_825CA500(ctx, base);
	// 82517B98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82517B9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82517BA0: 488DB889  bl 0x82df3428
	ctx.lr = 0x82517BA4;
	sub_82DF3428(ctx, base);
	// 82517BA4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82517BA8: 488DB881  bl 0x82df3428
	ctx.lr = 0x82517BAC;
	sub_82DF3428(ctx, base);
	// 82517BAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517BB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517BB4: 388B209C  addi r4, r11, 0x209c
	ctx.r[4].s64 = ctx.r[11].s64 + 8348;
	// 82517BB8: 488DBE51  bl 0x82df3a08
	ctx.lr = 0x82517BBC;
	sub_82DF3A08(ctx, base);
	// 82517BBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517BC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517BC4: 388B2084  addi r4, r11, 0x2084
	ctx.r[4].s64 = ctx.r[11].s64 + 8324;
	// 82517BC8: 488DBE41  bl 0x82df3a08
	ctx.lr = 0x82517BCC;
	sub_82DF3A08(ctx, base);
	// 82517BCC: 3CE0000F  lis r7, 0xf
	ctx.r[7].s64 = 983040;
	// 82517BD0: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 82517BD4: 60E74240  ori r7, r7, 0x4240
	ctx.r[7].u64 = ctx.r[7].u64 | 16960;
	// 82517BD8: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 82517BDC: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 82517BE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517BE4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82517BE8: 48095FA9  bl 0x825adb90
	ctx.lr = 0x82517BEC;
	sub_825ADB90(ctx, base);
	// 82517BEC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517BF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82517BF4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82517BF8: 48089F49  bl 0x825a1b40
	ctx.lr = 0x82517BFC;
	sub_825A1B40(ctx, base);
	// 82517BFC: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82517C00: 488DB829  bl 0x82df3428
	ctx.lr = 0x82517C04;
	sub_82DF3428(ctx, base);
	// 82517C04: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82517C08: 4BDB10B1  bl 0x822c8cb8
	ctx.lr = 0x82517C0C;
	sub_822C8CB8(ctx, base);
	// 82517C0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517C10: 488DB819  bl 0x82df3428
	ctx.lr = 0x82517C14;
	sub_82DF3428(ctx, base);
	// 82517C14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517C18: 488DB811  bl 0x82df3428
	ctx.lr = 0x82517C1C;
	sub_82DF3428(ctx, base);
	// 82517C1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517C24: 388B2074  addi r4, r11, 0x2074
	ctx.r[4].s64 = ctx.r[11].s64 + 8308;
	// 82517C28: 488DBDE1  bl 0x82df3a08
	ctx.lr = 0x82517C2C;
	sub_82DF3A08(ctx, base);
	// 82517C2C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517C30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517C34: 388B205C  addi r4, r11, 0x205c
	ctx.r[4].s64 = ctx.r[11].s64 + 8284;
	// 82517C38: 488DBDD1  bl 0x82df3a08
	ctx.lr = 0x82517C3C;
	sub_82DF3A08(ctx, base);
	// 82517C3C: 3CE0000F  lis r7, 0xf
	ctx.r[7].s64 = 983040;
	// 82517C40: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 82517C44: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 82517C48: 60E74240  ori r7, r7, 0x4240
	ctx.r[7].u64 = ctx.r[7].u64 | 16960;
	// 82517C4C: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 82517C50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517C54: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82517C58: 48095F39  bl 0x825adb90
	ctx.lr = 0x82517C5C;
	sub_825ADB90(ctx, base);
	// 82517C5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517C60: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82517C64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82517C68: 48089ED9  bl 0x825a1b40
	ctx.lr = 0x82517C6C;
	sub_825A1B40(ctx, base);
	// 82517C6C: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 82517C70: 488DB7B9  bl 0x82df3428
	ctx.lr = 0x82517C74;
	sub_82DF3428(ctx, base);
	// 82517C74: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82517C78: 4BDB1041  bl 0x822c8cb8
	ctx.lr = 0x82517C7C;
	sub_822C8CB8(ctx, base);
	// 82517C7C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517C80: 488DB7A9  bl 0x82df3428
	ctx.lr = 0x82517C84;
	sub_82DF3428(ctx, base);
	// 82517C84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517C88: 488DB7A1  bl 0x82df3428
	ctx.lr = 0x82517C8C;
	sub_82DF3428(ctx, base);
	// 82517C8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517C90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517C94: 388B2044  addi r4, r11, 0x2044
	ctx.r[4].s64 = ctx.r[11].s64 + 8260;
	// 82517C98: 488DBD71  bl 0x82df3a08
	ctx.lr = 0x82517C9C;
	sub_82DF3A08(ctx, base);
	// 82517C9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517CA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517CA4: 388B2024  addi r4, r11, 0x2024
	ctx.r[4].s64 = ctx.r[11].s64 + 8228;
	// 82517CA8: 488DBD61  bl 0x82df3a08
	ctx.lr = 0x82517CAC;
	sub_82DF3A08(ctx, base);
	// 82517CAC: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82517CB0: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 82517CB4: 38E01388  li r7, 0x1388
	ctx.r[7].s64 = 5000;
	// 82517CB8: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 82517CBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517CC0: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82517CC4: 48095ECD  bl 0x825adb90
	ctx.lr = 0x82517CC8;
	sub_825ADB90(ctx, base);
	// 82517CC8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82517CCC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82517CD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82517CD4: 48089E6D  bl 0x825a1b40
	ctx.lr = 0x82517CD8;
	sub_825A1B40(ctx, base);
	// 82517CD8: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 82517CDC: 488DB74D  bl 0x82df3428
	ctx.lr = 0x82517CE0;
	sub_82DF3428(ctx, base);
	// 82517CE0: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 82517CE4: 4BDB0FD5  bl 0x822c8cb8
	ctx.lr = 0x82517CE8;
	sub_822C8CB8(ctx, base);
	// 82517CE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82517CEC: 488DB73D  bl 0x82df3428
	ctx.lr = 0x82517CF0;
	sub_82DF3428(ctx, base);
	// 82517CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517CF4: 488DB735  bl 0x82df3428
	ctx.lr = 0x82517CF8;
	sub_82DF3428(ctx, base);
	// 82517CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82517CFC: 480B1FD5  bl 0x825c9cd0
	ctx.lr = 0x82517D00;
	sub_825C9CD0(ctx, base);
	// 82517D00: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 82517D04: 48C904B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517D08 size=116
    let mut pc: u32 = 0x82517D08;
    'dispatch: loop {
        match pc {
            0x82517D08 => {
    //   block [0x82517D08..0x82517D7C)
	// 82517D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517D0C: 48C90461  bl 0x831a816c
	ctx.lr = 0x82517D10;
	sub_831A8130(ctx, base);
	// 82517D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517D14: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82517D18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82517D1C: 83A30098  lwz r29, 0x98(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82517D20: 38A100A4  addi r5, r1, 0xa4
	ctx.r[5].s64 = ctx.r[1].s64 + 164;
	// 82517D24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517D28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82517D2C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82517D30: 48004479  bl 0x8251c1a8
	ctx.lr = 0x82517D34;
	sub_8251C1A8(ctx, base);
	// 82517D34: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517D38: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82517D3C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82517D40: 419A0030  beq cr6, 0x82517d70
	if ctx.cr[6].eq {
	pc = 0x82517D70; continue 'dispatch;
	}
	// 82517D44: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82517D48: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82517D4C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82517D50: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82517D54: 7D29F030  slw r9, r9, r30
	if (ctx.r[30].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[30].u8 & 0x1F) as u32)) as u64;
	}
	// 82517D58: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82517D5C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82517D60: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82517D64: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82517D68: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82517D6C: 48000008  b 0x82517d74
	pc = 0x82517D74; continue 'dispatch;
	// 82517D70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82517D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82517D78: 48C90444  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517D80 size=92
    let mut pc: u32 = 0x82517D80;
    'dispatch: loop {
        match pc {
            0x82517D80 => {
    //   block [0x82517D80..0x82517DDC)
	// 82517D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517D8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517D90: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82517D94: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 82517D98: 83E30098  lwz r31, 0x98(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82517D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517DA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82517DA4: 48004405  bl 0x8251c1a8
	ctx.lr = 0x82517DA8;
	sub_8251C1A8(ctx, base);
	// 82517DA8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517DAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517DB0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82517DB4: 419A0010  beq cr6, 0x82517dc4
	if ctx.cr[6].eq {
	pc = 0x82517DC4; continue 'dispatch;
	}
	// 82517DB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82517DBC: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82517DC0: 48000008  b 0x82517dc8
	pc = 0x82517DC8; continue 'dispatch;
	// 82517DC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82517DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517DD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82517DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517DE0 size=96
    let mut pc: u32 = 0x82517DE0;
    'dispatch: loop {
        match pc {
            0x82517DE0 => {
    //   block [0x82517DE0..0x82517E40)
	// 82517DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517DEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517DF0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82517DF4: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 82517DF8: 83E30098  lwz r31, 0x98(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82517DFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517E00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82517E04: 480043A5  bl 0x8251c1a8
	ctx.lr = 0x82517E08;
	sub_8251C1A8(ctx, base);
	// 82517E08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517E0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517E10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82517E14: 419A0010  beq cr6, 0x82517e24
	if ctx.cr[6].eq {
	pc = 0x82517E24; continue 'dispatch;
	}
	// 82517E18: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82517E1C: C02B0010  lfs f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82517E20: 4800000C  b 0x82517e2c
	pc = 0x82517E2C; continue 'dispatch;
	// 82517E24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517E28: C02B20FC  lfs f1, 0x20fc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8444 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82517E2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82517E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517E40 size=92
    let mut pc: u32 = 0x82517E40;
    'dispatch: loop {
        match pc {
            0x82517E40 => {
    //   block [0x82517E40..0x82517E9C)
	// 82517E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517E4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517E50: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82517E54: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 82517E58: 83E30098  lwz r31, 0x98(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82517E5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517E60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82517E64: 48004345  bl 0x8251c1a8
	ctx.lr = 0x82517E68;
	sub_8251C1A8(ctx, base);
	// 82517E68: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517E6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517E70: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82517E74: 419A0010  beq cr6, 0x82517e84
	if ctx.cr[6].eq {
	pc = 0x82517E84; continue 'dispatch;
	}
	// 82517E78: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82517E7C: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82517E80: 48000008  b 0x82517e88
	pc = 0x82517E88; continue 'dispatch;
	// 82517E84: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82517E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82517E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517EA0 size=8
    let mut pc: u32 = 0x82517EA0;
    'dispatch: loop {
        match pc {
            0x82517EA0 => {
    //   block [0x82517EA0..0x82517EA8)
	// 82517EA0: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82517EA4: 4BFFF56C  b 0x82517410
	sub_82517410(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517EA8 size=572
    let mut pc: u32 = 0x82517EA8;
    'dispatch: loop {
        match pc {
            0x82517EA8 => {
    //   block [0x82517EA8..0x825180E4)
	// 82517EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517EAC: 48C902A9  bl 0x831a8154
	ctx.lr = 0x82517EB0;
	sub_831A8130(ctx, base);
	// 82517EB0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517EB4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82517EB8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82517EBC: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 82517EC0: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82517EC4: 40980218  bge cr6, 0x825180dc
	if !ctx.cr[6].lt {
	pc = 0x825180DC; continue 'dispatch;
	}
	// 82517EC8: 39444004  addi r10, r4, 0x4004
	ctx.r[10].s64 = ctx.r[4].s64 + 16388;
	// 82517ECC: 81770094  lwz r11, 0x94(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(148 as u32) ) } as u64;
	// 82517ED0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82517ED4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82517ED8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517EDC: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82517EE0: 7D4A0775  extsb. r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82517EE4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82517EE8: 4080000C  bge 0x82517ef4
	if !ctx.cr[0].lt {
	pc = 0x82517EF4; continue 'dispatch;
	}
	// 82517EEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82517EF0: 48000014  b 0x82517f04
	pc = 0x82517F04; continue 'dispatch;
	// 82517EF4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82517EF8: 2F0A0063  cmpwi cr6, r10, 0x63
	ctx.cr[6].compare_i32(ctx.r[10].s32, 99, &mut ctx.xer);
	// 82517EFC: 4099000C  ble cr6, 0x82517f08
	if !ctx.cr[6].gt {
	pc = 0x82517F08; continue 'dispatch;
	}
	// 82517F00: 39400063  li r10, 0x63
	ctx.r[10].s64 = 99;
	// 82517F04: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82517F08: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82517F0C: 554907FF  clrlwi. r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82517F10: 408201CC  bne 0x825180dc
	if !ctx.cr[0].eq {
	pc = 0x825180DC; continue 'dispatch;
	}
	// 82517F14: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82517F18: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82517F1C: 512A07AE  rlwimi r10, r9, 0, 0x1e, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFFFFF03) | (ctx.r[10].u64 & 0x00000000000000FC);
	// 82517F20: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82517F24: 4BFD2B65  bl 0x824eaa88
	ctx.lr = 0x82517F28;
	sub_824EAA88(ctx, base);
	// 82517F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82517F2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82517F30: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517F34: 4BFD33CD  bl 0x824eb300
	ctx.lr = 0x82517F38;
	sub_824EB300(ctx, base);
	// 82517F38: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82517F3C: 83230000  lwz r25, 0(r3)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517F40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F44: 419A000C  beq cr6, 0x82517f50
	if ctx.cr[6].eq {
	pc = 0x82517F50; continue 'dispatch;
	}
	// 82517F48: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82517F4C: 4BDA8945  bl 0x822c0890
	ctx.lr = 0x82517F50;
	sub_822C0890(ctx, base);
	// 82517F50: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82517F54: 488D9D3D  bl 0x82df1c90
	ctx.lr = 0x82517F58;
	sub_82DF1C90(ctx, base);
	// 82517F58: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F60: 409A000C  bne cr6, 0x82517f6c
	if !ctx.cr[6].eq {
	pc = 0x82517F6C; continue 'dispatch;
	}
	// 82517F64: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82517F68: 48000010  b 0x82517f78
	pc = 0x82517F78; continue 'dispatch;
	// 82517F6C: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82517F70: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82517F74: 7D7B1E70  srawi r27, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82517F78: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82517F7C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82517F80: 409900A8  ble cr6, 0x82518028
	if !ctx.cr[6].gt {
	pc = 0x82518028; continue 'dispatch;
	}
	// 82517F84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82517F88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82517F8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82517F90: 3B4B213C  addi r26, r11, 0x213c
	ctx.r[26].s64 = ctx.r[11].s64 + 8508;
	// 82517F94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82517F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82517F9C: 488DBA6D  bl 0x82df3a08
	ctx.lr = 0x82517FA0;
	sub_82DF3A08(ctx, base);
	// 82517FA0: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517FA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82517FA8: 63180001  ori r24, r24, 1
	ctx.r[24].u64 = ctx.r[24].u64 | 1;
	// 82517FAC: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82517FB0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82517FB4: 488DB355  bl 0x82df3308
	ctx.lr = 0x82517FB8;
	sub_82DF3308(ctx, base);
	// 82517FB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82517FBC: 41820034  beq 0x82517ff0
	if ctx.cr[0].eq {
	pc = 0x82517FF0; continue 'dispatch;
	}
	// 82517FC0: 2B1C0190  cmplwi cr6, r28, 0x190
	ctx.cr[6].compare_u32(ctx.r[28].u32, 400 as u32, &mut ctx.xer);
	// 82517FC4: 4198000C  blt cr6, 0x82517fd0
	if ctx.cr[6].lt {
	pc = 0x82517FD0; continue 'dispatch;
	}
	// 82517FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82517FCC: 48000018  b 0x82517fe4
	pc = 0x82517FE4; continue 'dispatch;
	// 82517FD0: 81770094  lwz r11, 0x94(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(148 as u32) ) } as u64;
	// 82517FD4: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82517FD8: 614A8009  ori r10, r10, 0x8009
	ctx.r[10].u64 = ctx.r[10].u64 | 32777;
	// 82517FDC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82517FE0: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82517FE4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82517FE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82517FEC: 41820008  beq 0x82517ff4
	if ctx.cr[0].eq {
	pc = 0x82517FF4; continue 'dispatch;
	}
	// 82517FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82517FF4: 570A07FF  clrlwi. r10, r24, 0x1f
	ctx.r[10].u64 = ctx.r[24].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82517FF8: 557F063E  clrlwi r31, r11, 0x18
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82517FFC: 41820010  beq 0x8251800c
	if ctx.cr[0].eq {
	pc = 0x8251800C; continue 'dispatch;
	}
	// 82518000: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518004: 5718003C  rlwinm r24, r24, 0, 0, 0x1e
	ctx.r[24].u64 = ctx.r[24].u32 as u64 & 0xFFFFFFFFu64;
	// 82518008: 488DB421  bl 0x82df3428
	ctx.lr = 0x8251800C;
	sub_82DF3428(ctx, base);
	// 8251800C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518010: 40820018  bne 0x82518028
	if !ctx.cr[0].eq {
	pc = 0x82518028; continue 'dispatch;
	}
	// 82518014: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82518018: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8251801C: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82518020: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82518024: 4198FF70  blt cr6, 0x82517f94
	if ctx.cr[6].lt {
	pc = 0x82517F94; continue 'dispatch;
	}
	// 82518028: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8251802C: 409A00B0  bne cr6, 0x825180dc
	if !ctx.cr[6].eq {
	pc = 0x825180DC; continue 'dispatch;
	}
	// 82518030: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 82518034: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518038: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251803C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82518040: 4BDF1791  bl 0x823097d0
	ctx.lr = 0x82518044;
	sub_823097D0(ctx, base);
	// 82518044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518048: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251804C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518054: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518058: 419A0024  beq cr6, 0x8251807c
	if ctx.cr[6].eq {
	pc = 0x8251807C; continue 'dispatch;
	}
	// 8251805C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518060: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518064: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518068: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251806C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518070: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82518074: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518078: 4082FFE8  bne 0x82518060
	if !ctx.cr[0].eq {
	pc = 0x82518060; continue 'dispatch;
	}
	// 8251807C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82518080: 4BFD2A09  bl 0x824eaa88
	ctx.lr = 0x82518084;
	sub_824EAA88(ctx, base);
	// 82518084: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518088: 3BE10058  addi r31, r1, 0x58
	ctx.r[31].s64 = ctx.r[1].s64 + 88;
	// 8251808C: 4BFD1815  bl 0x824e98a0
	ctx.lr = 0x82518090;
	sub_824E98A0(ctx, base);
	// 82518090: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82518094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82518098: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8251809C: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825180A0: 38A0038A  li r5, 0x38a
	ctx.r[5].s64 = 906;
	// 825180A4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 825180A8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825180AC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 825180B0: 48940991  bl 0x82e58a40
	ctx.lr = 0x825180B4;
	sub_82E58A40(ctx, base);
	// 825180B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825180B8: 488D9BD9  bl 0x82df1c90
	ctx.lr = 0x825180BC;
	sub_82DF1C90(ctx, base);
	// 825180BC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825180C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825180C4: 419A0008  beq cr6, 0x825180cc
	if ctx.cr[6].eq {
	pc = 0x825180CC; continue 'dispatch;
	}
	// 825180C8: 4BDA87C9  bl 0x822c0890
	ctx.lr = 0x825180CC;
	sub_822C0890(ctx, base);
	// 825180CC: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 825180D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825180D4: 419A0008  beq cr6, 0x825180dc
	if ctx.cr[6].eq {
	pc = 0x825180DC; continue 'dispatch;
	}
	// 825180D8: 4BDA87B9  bl 0x822c0890
	ctx.lr = 0x825180DC;
	sub_822C0890(ctx, base);
	// 825180DC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825180E0: 48C900C4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825180E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825180E8 size=276
    let mut pc: u32 = 0x825180E8;
    'dispatch: loop {
        match pc {
            0x825180E8 => {
    //   block [0x825180E8..0x825181FC)
	// 825180E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825180EC: 48C90081  bl 0x831a816c
	ctx.lr = 0x825180F0;
	sub_831A8130(ctx, base);
	// 825180F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825180F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825180F8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 825180FC: 3D208328  lis r9, -0x7cd8
	ctx.r[9].s64 = -2094530560;
	// 82518100: 616B85E4  ori r11, r11, 0x85e4
	ctx.r[11].u64 = ctx.r[11].u64 | 34276;
	// 82518104: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518108: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8251810C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82518110: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518114: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82518118: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251811C: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518120: 8169CEB0  lwz r11, -0x3150(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12624 as u32) ) } as u64;
	// 82518124: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82518128: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8251812C: 419800C8  blt cr6, 0x825181f4
	if ctx.cr[6].lt {
	pc = 0x825181F4; continue 'dispatch;
	}
	// 82518130: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82518134: 897E7045  lbz r11, 0x7045(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28741 as u32) ) } as u64;
	// 82518138: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251813C: 408200B8  bne 0x825181f4
	if !ctx.cr[0].eq {
	pc = 0x825181F4; continue 'dispatch;
	}
	// 82518140: 3960002F  li r11, 0x2f
	ctx.r[11].s64 = 47;
	// 82518144: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518148: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251814C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82518150: 4BDF1681  bl 0x823097d0
	ctx.lr = 0x82518154;
	sub_823097D0(ctx, base);
	// 82518154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518158: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251815C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518164: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518168: 419A0024  beq cr6, 0x8251818c
	if ctx.cr[6].eq {
	pc = 0x8251818C; continue 'dispatch;
	}
	// 8251816C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518170: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518174: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518178: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251817C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518180: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82518184: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518188: 4082FFE8  bne 0x82518170
	if !ctx.cr[0].eq {
	pc = 0x82518170; continue 'dispatch;
	}
	// 8251818C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82518190: 4BFD28F9  bl 0x824eaa88
	ctx.lr = 0x82518194;
	sub_824EAA88(ctx, base);
	// 82518194: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518198: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 8251819C: 4BFD1705  bl 0x824e98a0
	ctx.lr = 0x825181A0;
	sub_824E98A0(ctx, base);
	// 825181A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825181A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825181A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825181AC: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825181B0: 38A0048C  li r5, 0x48c
	ctx.r[5].s64 = 1164;
	// 825181B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825181B8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825181BC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825181C0: 48940881  bl 0x82e58a40
	ctx.lr = 0x825181C4;
	sub_82E58A40(ctx, base);
	// 825181C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825181C8: 488D9AC9  bl 0x82df1c90
	ctx.lr = 0x825181CC;
	sub_82DF1C90(ctx, base);
	// 825181CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825181D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825181D4: 419A0008  beq cr6, 0x825181dc
	if ctx.cr[6].eq {
	pc = 0x825181DC; continue 'dispatch;
	}
	// 825181D8: 4BDA86B9  bl 0x822c0890
	ctx.lr = 0x825181DC;
	sub_822C0890(ctx, base);
	// 825181DC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825181E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825181E4: 419A0008  beq cr6, 0x825181ec
	if ctx.cr[6].eq {
	pc = 0x825181EC; continue 'dispatch;
	}
	// 825181E8: 4BDA86A9  bl 0x822c0890
	ctx.lr = 0x825181EC;
	sub_822C0890(ctx, base);
	// 825181EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825181F0: 997E7045  stb r11, 0x7045(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28741 as u32), ctx.r[11].u8 ) };
	// 825181F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825181F8: 48C8FFC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518200 size=276
    let mut pc: u32 = 0x82518200;
    'dispatch: loop {
        match pc {
            0x82518200 => {
    //   block [0x82518200..0x82518314)
	// 82518200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518204: 48C8FF69  bl 0x831a816c
	ctx.lr = 0x82518208;
	sub_831A8130(ctx, base);
	// 82518208: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251820C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518210: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82518214: 3D208328  lis r9, -0x7cd8
	ctx.r[9].s64 = -2094530560;
	// 82518218: 616B85E8  ori r11, r11, 0x85e8
	ctx.r[11].u64 = ctx.r[11].u64 | 34280;
	// 8251821C: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518220: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82518224: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82518228: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251822C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82518230: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518234: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518238: 8169CEB4  lwz r11, -0x314c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12620 as u32) ) } as u64;
	// 8251823C: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82518240: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82518244: 419800C8  blt cr6, 0x8251830c
	if ctx.cr[6].lt {
	pc = 0x8251830C; continue 'dispatch;
	}
	// 82518248: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 8251824C: 897E7046  lbz r11, 0x7046(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28742 as u32) ) } as u64;
	// 82518250: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518254: 408200B8  bne 0x8251830c
	if !ctx.cr[0].eq {
	pc = 0x8251830C; continue 'dispatch;
	}
	// 82518258: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8251825C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518260: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82518264: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82518268: 4BDF1569  bl 0x823097d0
	ctx.lr = 0x8251826C;
	sub_823097D0(ctx, base);
	// 8251826C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518270: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82518274: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251827C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518280: 419A0024  beq cr6, 0x825182a4
	if ctx.cr[6].eq {
	pc = 0x825182A4; continue 'dispatch;
	}
	// 82518284: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518288: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251828C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518290: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82518294: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518298: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251829C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825182A0: 4082FFE8  bne 0x82518288
	if !ctx.cr[0].eq {
	pc = 0x82518288; continue 'dispatch;
	}
	// 825182A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825182A8: 4BFD27E1  bl 0x824eaa88
	ctx.lr = 0x825182AC;
	sub_824EAA88(ctx, base);
	// 825182AC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825182B0: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 825182B4: 4BFD15ED  bl 0x824e98a0
	ctx.lr = 0x825182B8;
	sub_824E98A0(ctx, base);
	// 825182B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825182BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825182C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825182C4: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825182C8: 38A0049F  li r5, 0x49f
	ctx.r[5].s64 = 1183;
	// 825182CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825182D0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825182D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825182D8: 48940769  bl 0x82e58a40
	ctx.lr = 0x825182DC;
	sub_82E58A40(ctx, base);
	// 825182DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825182E0: 488D99B1  bl 0x82df1c90
	ctx.lr = 0x825182E4;
	sub_82DF1C90(ctx, base);
	// 825182E4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825182E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825182EC: 419A0008  beq cr6, 0x825182f4
	if ctx.cr[6].eq {
	pc = 0x825182F4; continue 'dispatch;
	}
	// 825182F0: 4BDA85A1  bl 0x822c0890
	ctx.lr = 0x825182F4;
	sub_822C0890(ctx, base);
	// 825182F4: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825182F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825182FC: 419A0008  beq cr6, 0x82518304
	if ctx.cr[6].eq {
	pc = 0x82518304; continue 'dispatch;
	}
	// 82518300: 4BDA8591  bl 0x822c0890
	ctx.lr = 0x82518304;
	sub_822C0890(ctx, base);
	// 82518304: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82518308: 997E7046  stb r11, 0x7046(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28742 as u32), ctx.r[11].u8 ) };
	// 8251830C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82518310: 48C8FEAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518318 size=256
    let mut pc: u32 = 0x82518318;
    'dispatch: loop {
        match pc {
            0x82518318 => {
    //   block [0x82518318..0x82518418)
	// 82518318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251831C: 48C8FE51  bl 0x831a816c
	ctx.lr = 0x82518320;
	sub_831A8130(ctx, base);
	// 82518320: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518328: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251832C: 3D208328  lis r9, -0x7cd8
	ctx.r[9].s64 = -2094530560;
	// 82518330: 616885E0  ori r8, r11, 0x85e0
	ctx.r[8].u64 = ctx.r[11].u64 | 34272;
	// 82518334: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518338: 8169CEAC  lwz r11, -0x3154(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12628 as u32) ) } as u64;
	// 8251833C: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82518340: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82518344: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82518348: 419800C8  blt cr6, 0x82518410
	if ctx.cr[6].lt {
	pc = 0x82518410; continue 'dispatch;
	}
	// 8251834C: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82518350: 897E7044  lbz r11, 0x7044(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28740 as u32) ) } as u64;
	// 82518354: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518358: 408200B8  bne 0x82518410
	if !ctx.cr[0].eq {
	pc = 0x82518410; continue 'dispatch;
	}
	// 8251835C: 3960002E  li r11, 0x2e
	ctx.r[11].s64 = 46;
	// 82518360: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518364: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82518368: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251836C: 4BDF1465  bl 0x823097d0
	ctx.lr = 0x82518370;
	sub_823097D0(ctx, base);
	// 82518370: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518374: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82518378: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251837C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518384: 419A0024  beq cr6, 0x825183a8
	if ctx.cr[6].eq {
	pc = 0x825183A8; continue 'dispatch;
	}
	// 82518388: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251838C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518390: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518394: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82518398: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251839C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825183A0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825183A4: 4082FFE8  bne 0x8251838c
	if !ctx.cr[0].eq {
	pc = 0x8251838C; continue 'dispatch;
	}
	// 825183A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825183AC: 4BFD26DD  bl 0x824eaa88
	ctx.lr = 0x825183B0;
	sub_824EAA88(ctx, base);
	// 825183B0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825183B4: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 825183B8: 4BFD14E9  bl 0x824e98a0
	ctx.lr = 0x825183BC;
	sub_824E98A0(ctx, base);
	// 825183BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825183C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825183C4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825183C8: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825183CC: 38A004B3  li r5, 0x4b3
	ctx.r[5].s64 = 1203;
	// 825183D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825183D4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825183D8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825183DC: 48940665  bl 0x82e58a40
	ctx.lr = 0x825183E0;
	sub_82E58A40(ctx, base);
	// 825183E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825183E4: 488D98AD  bl 0x82df1c90
	ctx.lr = 0x825183E8;
	sub_82DF1C90(ctx, base);
	// 825183E8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825183EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825183F0: 419A0008  beq cr6, 0x825183f8
	if ctx.cr[6].eq {
	pc = 0x825183F8; continue 'dispatch;
	}
	// 825183F4: 4BDA849D  bl 0x822c0890
	ctx.lr = 0x825183F8;
	sub_822C0890(ctx, base);
	// 825183F8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825183FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518400: 419A0008  beq cr6, 0x82518408
	if ctx.cr[6].eq {
	pc = 0x82518408; continue 'dispatch;
	}
	// 82518404: 4BDA848D  bl 0x822c0890
	ctx.lr = 0x82518408;
	sub_822C0890(ctx, base);
	// 82518408: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251840C: 997E7044  stb r11, 0x7044(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28740 as u32), ctx.r[11].u8 ) };
	// 82518410: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82518414: 48C8FDA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518418 size=276
    let mut pc: u32 = 0x82518418;
    'dispatch: loop {
        match pc {
            0x82518418 => {
    //   block [0x82518418..0x8251852C)
	// 82518418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251841C: 48C8FD51  bl 0x831a816c
	ctx.lr = 0x82518420;
	sub_831A8130(ctx, base);
	// 82518420: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518428: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251842C: 3D208328  lis r9, -0x7cd8
	ctx.r[9].s64 = -2094530560;
	// 82518430: 616B85E0  ori r11, r11, 0x85e0
	ctx.r[11].u64 = ctx.r[11].u64 | 34272;
	// 82518434: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518438: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8251843C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82518440: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518444: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82518448: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251844C: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518450: 8169CEAC  lwz r11, -0x3154(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12628 as u32) ) } as u64;
	// 82518454: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82518458: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8251845C: 419800C8  blt cr6, 0x82518524
	if ctx.cr[6].lt {
	pc = 0x82518524; continue 'dispatch;
	}
	// 82518460: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82518464: 897E7044  lbz r11, 0x7044(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28740 as u32) ) } as u64;
	// 82518468: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251846C: 408200B8  bne 0x82518524
	if !ctx.cr[0].eq {
	pc = 0x82518524; continue 'dispatch;
	}
	// 82518470: 3960002E  li r11, 0x2e
	ctx.r[11].s64 = 46;
	// 82518474: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518478: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251847C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82518480: 4BDF1351  bl 0x823097d0
	ctx.lr = 0x82518484;
	sub_823097D0(ctx, base);
	// 82518484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518488: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251848C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518494: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518498: 419A0024  beq cr6, 0x825184bc
	if ctx.cr[6].eq {
	pc = 0x825184BC; continue 'dispatch;
	}
	// 8251849C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825184A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825184A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825184A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825184AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825184B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825184B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825184B8: 4082FFE8  bne 0x825184a0
	if !ctx.cr[0].eq {
	pc = 0x825184A0; continue 'dispatch;
	}
	// 825184BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825184C0: 4BFD25C9  bl 0x824eaa88
	ctx.lr = 0x825184C4;
	sub_824EAA88(ctx, base);
	// 825184C4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825184C8: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 825184CC: 4BFD13D5  bl 0x824e98a0
	ctx.lr = 0x825184D0;
	sub_824E98A0(ctx, base);
	// 825184D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825184D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825184D8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825184DC: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825184E0: 38A004C8  li r5, 0x4c8
	ctx.r[5].s64 = 1224;
	// 825184E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825184E8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825184EC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825184F0: 48940551  bl 0x82e58a40
	ctx.lr = 0x825184F4;
	sub_82E58A40(ctx, base);
	// 825184F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825184F8: 488D9799  bl 0x82df1c90
	ctx.lr = 0x825184FC;
	sub_82DF1C90(ctx, base);
	// 825184FC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82518500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518504: 419A0008  beq cr6, 0x8251850c
	if ctx.cr[6].eq {
	pc = 0x8251850C; continue 'dispatch;
	}
	// 82518508: 4BDA8389  bl 0x822c0890
	ctx.lr = 0x8251850C;
	sub_822C0890(ctx, base);
	// 8251850C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82518510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518514: 419A0008  beq cr6, 0x8251851c
	if ctx.cr[6].eq {
	pc = 0x8251851C; continue 'dispatch;
	}
	// 82518518: 4BDA8379  bl 0x822c0890
	ctx.lr = 0x8251851C;
	sub_822C0890(ctx, base);
	// 8251851C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82518520: 997E7044  stb r11, 0x7044(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28740 as u32), ctx.r[11].u8 ) };
	// 82518524: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82518528: 48C8FC94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518530 size=100
    let mut pc: u32 = 0x82518530;
    'dispatch: loop {
        match pc {
            0x82518530 => {
    //   block [0x82518530..0x82518594)
	// 82518530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251853C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518540: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82518544: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 82518548: 83E30098  lwz r31, 0x98(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 8251854C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518550: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82518554: 48003C55  bl 0x8251c1a8
	ctx.lr = 0x82518558;
	sub_8251C1A8(ctx, base);
	// 82518558: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251855C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82518560: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82518564: 419A0010  beq cr6, 0x82518574
	if ctx.cr[6].eq {
	pc = 0x82518574; continue 'dispatch;
	}
	// 82518568: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251856C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518570: 409A000C  bne cr6, 0x8251857c
	if !ctx.cr[6].eq {
	pc = 0x8251857C; continue 'dispatch;
	}
	// 82518574: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82518578: 48000008  b 0x82518580
	pc = 0x82518580; continue 'dispatch;
	// 8251857C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82518580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251858C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518598 size=140
    let mut pc: u32 = 0x82518598;
    'dispatch: loop {
        match pc {
            0x82518598 => {
    //   block [0x82518598..0x82518624)
	// 82518598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825185A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825185A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825185A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825185AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825185B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825185B4: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 825185B8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825185BC: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 825185C0: 9BCB0015  stb r30, 0x15(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(21 as u32), ctx.r[30].u8 ) };
	// 825185C4: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 825185C8: 48003B89  bl 0x8251c150
	ctx.lr = 0x825185CC;
	sub_8251C150(ctx, base);
	// 825185CC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825185D0: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825185D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825185D8: 419A0034  beq cr6, 0x8251860c
	if ctx.cr[6].eq {
	pc = 0x8251860C; continue 'dispatch;
	}
	// 825185DC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825185E0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825185E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825185E8: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 825185EC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825185F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825185F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825185F8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825185FC: 4800458D  bl 0x8251cb88
	ctx.lr = 0x82518600;
	sub_8251CB88(ctx, base);
	// 82518600: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 82518604: 2B1E8000  cmplwi cr6, r30, 0x8000
	ctx.cr[6].compare_u32(ctx.r[30].u32, 32768 as u32, &mut ctx.xer);
	// 82518608: 4198FFC4  blt cr6, 0x825185cc
	if ctx.cr[6].lt {
	pc = 0x825185CC; continue 'dispatch;
	}
	// 8251860C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518618: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251861C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518628 size=180
    let mut pc: u32 = 0x82518628;
    'dispatch: loop {
        match pc {
            0x82518628 => {
    //   block [0x82518628..0x825186DC)
	// 82518628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251862C: 48C8FB41  bl 0x831a816c
	ctx.lr = 0x82518630;
	sub_831A8130(ctx, base);
	// 82518630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518634: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82518638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251863C: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 82518640: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82518644: 419A0054  beq cr6, 0x82518698
	if ctx.cr[6].eq {
	pc = 0x82518698; continue 'dispatch;
	}
	// 82518648: 83DF0098  lwz r30, 0x98(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8251864C: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82518650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518654: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518658: 48003B51  bl 0x8251c1a8
	ctx.lr = 0x8251865C;
	sub_8251C1A8(ctx, base);
	// 8251865C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518660: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82518664: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82518668: 419A000C  beq cr6, 0x82518674
	if ctx.cr[6].eq {
	pc = 0x82518674; continue 'dispatch;
	}
	// 8251866C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82518670: 4800002C  b 0x8251869c
	pc = 0x8251869C; continue 'dispatch;
	// 82518674: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251867C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518680: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82518684: 419A0020  beq cr6, 0x825186a4
	if ctx.cr[6].eq {
	pc = 0x825186A4; continue 'dispatch;
	}
	// 82518688: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8251868C: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82518690: 2B0B0200  cmplwi cr6, r11, 0x200
	ctx.cr[6].compare_u32(ctx.r[11].u32, 512 as u32, &mut ctx.xer);
	// 82518694: 4198FFE8  blt cr6, 0x8251867c
	if ctx.cr[6].lt {
	pc = 0x8251867C; continue 'dispatch;
	}
	// 82518698: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251869C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825186A0: 48C8FB1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 825186A4: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825186A8: 557E3032  slwi r30, r11, 6
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825186AC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825186B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825186B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825186B8: 7FBE512E  stwx r29, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 825186BC: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 825186C0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825186C4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825186C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825186CC: 480044BD  bl 0x8251cb88
	ctx.lr = 0x825186D0;
	sub_8251CB88(ctx, base);
	// 825186D0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 825186D4: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825186D8: 4BFFFFC4  b 0x8251869c
	pc = 0x8251869C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825186E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825186E0 size=108
    let mut pc: u32 = 0x825186E0;
    'dispatch: loop {
        match pc {
            0x825186E0 => {
    //   block [0x825186E0..0x8251874C)
	// 825186E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825186E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825186E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825186EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825186F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825186F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825186F8: 4BFFFF31  bl 0x82518628
	ctx.lr = 0x825186FC;
	sub_82518628(ctx, base);
	// 825186FC: 7C6A1B79  or. r10, r3, r3
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82518700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518704: 41820034  beq 0x82518738
	if ctx.cr[0].eq {
	pc = 0x82518738; continue 'dispatch;
	}
	// 82518708: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8251870C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82518710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82518714: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82518718: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8251871C: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82518720: 7D295039  and. r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82518724: 41820008  beq 0x8251872c
	if ctx.cr[0].eq {
	pc = 0x8251872C; continue 'dispatch;
	}
	// 82518728: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8251872C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82518730: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 82518734: 4198FFE4  blt cr6, 0x82518718
	if ctx.cr[6].lt {
	pc = 0x82518718; continue 'dispatch;
	}
	// 82518738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251873C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518750 size=1116
    let mut pc: u32 = 0x82518750;
    'dispatch: loop {
        match pc {
            0x82518750 => {
    //   block [0x82518750..0x82518BAC)
	// 82518750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518754: 48C8FA11  bl 0x831a8164
	ctx.lr = 0x82518758;
	sub_831A8130(ctx, base);
	// 82518758: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251875C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82518760: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82518764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518768: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251876C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82518770: 4BFFFEB9  bl 0x82518628
	ctx.lr = 0x82518774;
	sub_82518628(ctx, base);
	// 82518774: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518778: 4182042C  beq 0x82518ba4
	if ctx.cr[0].eq {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 8251877C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82518780: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82518784: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82518788: 7D4BE830  slw r11, r10, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 8251878C: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82518790: 7D485839  and. r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82518794: 408203CC  bne 0x82518b60
	if !ctx.cr[0].eq {
	pc = 0x82518B60; continue 'dispatch;
	}
	// 82518798: 5788063F  clrlwi. r8, r28, 0x18
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8251879C: 418203C4  beq 0x82518b60
	if ctx.cr[0].eq {
	pc = 0x82518B60; continue 'dispatch;
	}
	// 825187A0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 825187A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825187A8: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 825187AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825187B0: 4BFFEA59  bl 0x82517208
	ctx.lr = 0x825187B4;
	sub_82517208(ctx, base);
	// 825187B4: 395F0003  addi r10, r31, 3
	ctx.r[10].s64 = ctx.r[31].s64 + 3;
	// 825187B8: 817E0098  lwz r11, 0x98(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 825187BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825187C0: 555C103A  slwi r28, r10, 2
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 825187C4: 7D4BE02E  lwzx r10, r11, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825187C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825187CC: 7D4BE12E  stwx r10, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 825187D0: 409A002C  bne cr6, 0x825187fc
	if !ctx.cr[6].eq {
	pc = 0x825187FC; continue 'dispatch;
	}
	// 825187D4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825187D8: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 825187DC: 4BFD22AD  bl 0x824eaa88
	ctx.lr = 0x825187E0;
	sub_824EAA88(ctx, base);
	// 825187E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825187E4: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 825187E8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825187EC: 4BFD2BD5  bl 0x824eb3c0
	ctx.lr = 0x825187F0;
	sub_824EB3C0(ctx, base);
	// 825187F0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825187F4: 48003535  bl 0x8251bd28
	ctx.lr = 0x825187F8;
	sub_8251BD28(ctx, base);
	// 825187F8: 48000028  b 0x82518820
	pc = 0x82518820; continue 'dispatch;
	// 825187FC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82518800: 3BA0000C  li r29, 0xc
	ctx.r[29].s64 = 12;
	// 82518804: 4BFD2285  bl 0x824eaa88
	ctx.lr = 0x82518808;
	sub_824EAA88(ctx, base);
	// 82518808: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251880C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82518810: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518814: 4BFD2BAD  bl 0x824eb3c0
	ctx.lr = 0x82518818;
	sub_824EB3C0(ctx, base);
	// 82518818: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251881C: 48003515  bl 0x8251bd30
	ctx.lr = 0x82518820;
	sub_8251BD30(ctx, base);
	// 82518820: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82518824: 57AB0739  rlwinm. r11, r29, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518828: 41820018  beq 0x82518840
	if ctx.cr[0].eq {
	pc = 0x82518840; continue 'dispatch;
	}
	// 8251882C: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82518830: 57BD0776  rlwinm r29, r29, 0, 0x1d, 0x1b
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 82518834: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518838: 419A0008  beq cr6, 0x82518840
	if ctx.cr[6].eq {
	pc = 0x82518840; continue 'dispatch;
	}
	// 8251883C: 4BDA8055  bl 0x822c0890
	ctx.lr = 0x82518840;
	sub_822C0890(ctx, base);
	// 82518840: 57AB077B  rlwinm. r11, r29, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518844: 41820010  beq 0x82518854
	if ctx.cr[0].eq {
	pc = 0x82518854; continue 'dispatch;
	}
	// 82518848: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251884C: 57BD07B8  rlwinm r29, r29, 0, 0x1e, 0x1c
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 82518850: 488D9441  bl 0x82df1c90
	ctx.lr = 0x82518854;
	sub_82DF1C90(ctx, base);
	// 82518854: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518858: 41820018  beq 0x82518870
	if ctx.cr[0].eq {
	pc = 0x82518870; continue 'dispatch;
	}
	// 8251885C: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 82518860: 57BD07FA  rlwinm r29, r29, 0, 0x1f, 0x1d
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 82518864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518868: 419A0008  beq cr6, 0x82518870
	if ctx.cr[6].eq {
	pc = 0x82518870; continue 'dispatch;
	}
	// 8251886C: 4BDA8025  bl 0x822c0890
	ctx.lr = 0x82518870;
	sub_822C0890(ctx, base);
	// 82518870: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518874: 4182000C  beq 0x82518880
	if ctx.cr[0].eq {
	pc = 0x82518880; continue 'dispatch;
	}
	// 82518878: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251887C: 488D9415  bl 0x82df1c90
	ctx.lr = 0x82518880;
	sub_82DF1C90(ctx, base);
	// 82518880: 817E0098  lwz r11, 0x98(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518884: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82518888: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8251888C: 41980158  blt cr6, 0x825189e4
	if ctx.cr[6].lt {
	pc = 0x825189E4; continue 'dispatch;
	}
	// 82518890: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82518894: 419800AC  blt cr6, 0x82518940
	if ctx.cr[6].lt {
	pc = 0x82518940; continue 'dispatch;
	}
	// 82518898: 409A030C  bne cr6, 0x82518ba4
	if !ctx.cr[6].eq {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 8251889C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 825188A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825188A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825188A8: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 825188AC: 4BDF0F25  bl 0x823097d0
	ctx.lr = 0x825188B0;
	sub_823097D0(ctx, base);
	// 825188B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825188B4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825188B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825188BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825188C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825188C4: 419A0024  beq cr6, 0x825188e8
	if ctx.cr[6].eq {
	pc = 0x825188E8; continue 'dispatch;
	}
	// 825188C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825188CC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825188D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825188D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825188D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825188DC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825188E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825188E4: 4082FFE8  bne 0x825188cc
	if !ctx.cr[0].eq {
	pc = 0x825188CC; continue 'dispatch;
	}
	// 825188E8: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 825188EC: 4BFD219D  bl 0x824eaa88
	ctx.lr = 0x825188F0;
	sub_824EAA88(ctx, base);
	// 825188F0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825188F4: 3BE10068  addi r31, r1, 0x68
	ctx.r[31].s64 = ctx.r[1].s64 + 104;
	// 825188F8: 4BFD0FA9  bl 0x824e98a0
	ctx.lr = 0x825188FC;
	sub_824E98A0(ctx, base);
	// 825188FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82518900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82518904: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82518908: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 8251890C: 38A00206  li r5, 0x206
	ctx.r[5].s64 = 518;
	// 82518910: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518914: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82518918: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8251891C: 48940125  bl 0x82e58a40
	ctx.lr = 0x82518920;
	sub_82E58A40(ctx, base);
	// 82518920: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82518924: 488D936D  bl 0x82df1c90
	ctx.lr = 0x82518928;
	sub_82DF1C90(ctx, base);
	// 82518928: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8251892C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518930: 419A0008  beq cr6, 0x82518938
	if ctx.cr[6].eq {
	pc = 0x82518938; continue 'dispatch;
	}
	// 82518934: 4BDA7F5D  bl 0x822c0890
	ctx.lr = 0x82518938;
	sub_822C0890(ctx, base);
	// 82518938: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 8251893C: 48000214  b 0x82518b50
	pc = 0x82518B50; continue 'dispatch;
	// 82518940: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82518944: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518948: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251894C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82518950: 4BDF0E81  bl 0x823097d0
	ctx.lr = 0x82518954;
	sub_823097D0(ctx, base);
	// 82518954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518958: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251895C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518964: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82518968: 419A0024  beq cr6, 0x8251898c
	if ctx.cr[6].eq {
	pc = 0x8251898C; continue 'dispatch;
	}
	// 8251896C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518970: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518974: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518978: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251897C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518980: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82518984: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518988: 4082FFE8  bne 0x82518970
	if !ctx.cr[0].eq {
	pc = 0x82518970; continue 'dispatch;
	}
	// 8251898C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82518990: 4BFD20F9  bl 0x824eaa88
	ctx.lr = 0x82518994;
	sub_824EAA88(ctx, base);
	// 82518994: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518998: 3BE10058  addi r31, r1, 0x58
	ctx.r[31].s64 = ctx.r[1].s64 + 88;
	// 8251899C: 4BFD0F05  bl 0x824e98a0
	ctx.lr = 0x825189A0;
	sub_824E98A0(ctx, base);
	// 825189A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825189A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825189A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825189AC: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 825189B0: 38A00203  li r5, 0x203
	ctx.r[5].s64 = 515;
	// 825189B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825189B8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825189BC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 825189C0: 48940081  bl 0x82e58a40
	ctx.lr = 0x825189C4;
	sub_82E58A40(ctx, base);
	// 825189C4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825189C8: 488D92C9  bl 0x82df1c90
	ctx.lr = 0x825189CC;
	sub_82DF1C90(ctx, base);
	// 825189CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825189D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825189D4: 419A0008  beq cr6, 0x825189dc
	if ctx.cr[6].eq {
	pc = 0x825189DC; continue 'dispatch;
	}
	// 825189D8: 4BDA7EB9  bl 0x822c0890
	ctx.lr = 0x825189DC;
	sub_822C0890(ctx, base);
	// 825189DC: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 825189E0: 48000170  b 0x82518b50
	pc = 0x82518B50; continue 'dispatch;
	// 825189E4: 576907FE  clrlwi r9, r27, 0x1f
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	// 825189E8: 576AF87E  srwi r10, r27, 1
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825189EC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 825189F0: 409A0008  bne cr6, 0x825189f8
	if !ctx.cr[6].eq {
	pc = 0x825189F8; continue 'dispatch;
	}
	// 825189F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825189F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825189FC: 419801A8  blt cr6, 0x82518ba4
	if ctx.cr[6].lt {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 82518A00: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82518A04: 419800AC  blt cr6, 0x82518ab0
	if ctx.cr[6].lt {
	pc = 0x82518AB0; continue 'dispatch;
	}
	// 82518A08: 409A019C  bne cr6, 0x82518ba4
	if !ctx.cr[6].eq {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 82518A0C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82518A10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518A14: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82518A18: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82518A1C: 4BDF0DB5  bl 0x823097d0
	ctx.lr = 0x82518A20;
	sub_823097D0(ctx, base);
	// 82518A20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518A24: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82518A28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82518A34: 419A0024  beq cr6, 0x82518a58
	if ctx.cr[6].eq {
	pc = 0x82518A58; continue 'dispatch;
	}
	// 82518A38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518A3C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518A40: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518A44: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82518A48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518A4C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82518A50: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518A54: 4082FFE8  bne 0x82518a3c
	if !ctx.cr[0].eq {
	pc = 0x82518A3C; continue 'dispatch;
	}
	// 82518A58: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82518A5C: 4BFD202D  bl 0x824eaa88
	ctx.lr = 0x82518A60;
	sub_824EAA88(ctx, base);
	// 82518A60: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518A64: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 82518A68: 4BFD0E39  bl 0x824e98a0
	ctx.lr = 0x82518A6C;
	sub_824E98A0(ctx, base);
	// 82518A6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82518A70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82518A74: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82518A78: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 82518A7C: 38A00216  li r5, 0x216
	ctx.r[5].s64 = 534;
	// 82518A80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518A84: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82518A88: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82518A8C: 4893FFB5  bl 0x82e58a40
	ctx.lr = 0x82518A90;
	sub_82E58A40(ctx, base);
	// 82518A90: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82518A94: 488D91FD  bl 0x82df1c90
	ctx.lr = 0x82518A98;
	sub_82DF1C90(ctx, base);
	// 82518A98: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82518A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518AA0: 419A0008  beq cr6, 0x82518aa8
	if ctx.cr[6].eq {
	pc = 0x82518AA8; continue 'dispatch;
	}
	// 82518AA4: 4BDA7DED  bl 0x822c0890
	ctx.lr = 0x82518AA8;
	sub_822C0890(ctx, base);
	// 82518AA8: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82518AAC: 480000A4  b 0x82518b50
	pc = 0x82518B50; continue 'dispatch;
	// 82518AB0: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82518AB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518AB8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82518ABC: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82518AC0: 4BDF0D11  bl 0x823097d0
	ctx.lr = 0x82518AC4;
	sub_823097D0(ctx, base);
	// 82518AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518AC8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82518ACC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518AD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518AD4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82518AD8: 419A0024  beq cr6, 0x82518afc
	if ctx.cr[6].eq {
	pc = 0x82518AFC; continue 'dispatch;
	}
	// 82518ADC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82518AE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82518AE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518AE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82518AEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82518AF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82518AF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82518AF8: 4082FFE8  bne 0x82518ae0
	if !ctx.cr[0].eq {
	pc = 0x82518AE0; continue 'dispatch;
	}
	// 82518AFC: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82518B00: 4BFD1F89  bl 0x824eaa88
	ctx.lr = 0x82518B04;
	sub_824EAA88(ctx, base);
	// 82518B04: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518B08: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 82518B0C: 4BFD0D95  bl 0x824e98a0
	ctx.lr = 0x82518B10;
	sub_824E98A0(ctx, base);
	// 82518B10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82518B14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82518B18: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82518B1C: 388A2100  addi r4, r10, 0x2100
	ctx.r[4].s64 = ctx.r[10].s64 + 8448;
	// 82518B20: 38A00213  li r5, 0x213
	ctx.r[5].s64 = 531;
	// 82518B24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518B28: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82518B2C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82518B30: 4893FF11  bl 0x82e58a40
	ctx.lr = 0x82518B34;
	sub_82E58A40(ctx, base);
	// 82518B34: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82518B38: 488D9159  bl 0x82df1c90
	ctx.lr = 0x82518B3C;
	sub_82DF1C90(ctx, base);
	// 82518B3C: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82518B40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518B44: 419A0008  beq cr6, 0x82518b4c
	if ctx.cr[6].eq {
	pc = 0x82518B4C; continue 'dispatch;
	}
	// 82518B48: 4BDA7D49  bl 0x822c0890
	ctx.lr = 0x82518B4C;
	sub_822C0890(ctx, base);
	// 82518B4C: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82518B50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82518B54: 419A0050  beq cr6, 0x82518ba4
	if ctx.cr[6].eq {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 82518B58: 4BDA7D39  bl 0x822c0890
	ctx.lr = 0x82518B5C;
	sub_822C0890(ctx, base);
	// 82518B5C: 48000048  b 0x82518ba4
	pc = 0x82518BA4; continue 'dispatch;
	// 82518B60: 5788063F  clrlwi. r8, r28, 0x18
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82518B64: 40820040  bne 0x82518ba4
	if !ctx.cr[0].eq {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 82518B68: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82518B6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82518B70: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 82518B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518B78: 4BFFE691  bl 0x82517208
	ctx.lr = 0x82518B7C;
	sub_82517208(ctx, base);
	// 82518B7C: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 82518B80: 815E0098  lwz r10, 0x98(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518B84: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82518B88: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518B8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82518B90: 40990014  ble cr6, 0x82518ba4
	if !ctx.cr[6].gt {
	pc = 0x82518BA4; continue 'dispatch;
	}
	// 82518B94: 815E0098  lwz r10, 0x98(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518B98: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518B9C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82518BA0: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82518BA4: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82518BA8: 48C8F60C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518BB0 size=84
    let mut pc: u32 = 0x82518BB0;
    'dispatch: loop {
        match pc {
            0x82518BB0 => {
    //   block [0x82518BB0..0x82518C04)
	// 82518BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518BB4: 48C8F5B9  bl 0x831a816c
	ctx.lr = 0x82518BB8;
	sub_831A8130(ctx, base);
	// 82518BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518BC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82518BC4: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518BC8: 83AB0018  lwz r29, 0x18(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82518BCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518BD0: 4BFFF1B1  bl 0x82517d80
	ctx.lr = 0x82518BD4;
	sub_82517D80(ctx, base);
	// 82518BD4: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82518BD8: 40990008  ble cr6, 0x82518be0
	if !ctx.cr[6].gt {
	pc = 0x82518BE0; continue 'dispatch;
	}
	// 82518BDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518BE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518BE8: 4BFFFA41  bl 0x82518628
	ctx.lr = 0x82518BEC;
	sub_82518628(ctx, base);
	// 82518BEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518BF0: 41820008  beq 0x82518bf8
	if ctx.cr[0].eq {
	pc = 0x82518BF8; continue 'dispatch;
	}
	// 82518BF4: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82518BF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518BFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518C00: 48C8F5BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518C08 size=116
    let mut pc: u32 = 0x82518C08;
    'dispatch: loop {
        match pc {
            0x82518C08 => {
    //   block [0x82518C08..0x82518C7C)
	// 82518C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82518C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518C18: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82518C1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518C20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518C24: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82518C28: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518C2C: 83CB0018  lwz r30, 0x18(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82518C30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518C34: 4BFFF1AD  bl 0x82517de0
	ctx.lr = 0x82518C38;
	sub_82517DE0(ctx, base);
	// 82518C38: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82518C3C: 40980008  bge cr6, 0x82518c44
	if !ctx.cr[6].lt {
	pc = 0x82518C44; continue 'dispatch;
	}
	// 82518C40: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82518C44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518C4C: 4BFFF9DD  bl 0x82518628
	ctx.lr = 0x82518C50;
	sub_82518628(ctx, base);
	// 82518C50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518C54: 41820008  beq 0x82518c5c
	if ctx.cr[0].eq {
	pc = 0x82518C5C; continue 'dispatch;
	}
	// 82518C58: D3E30010  stfs f31, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82518C5C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82518C60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518C6C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82518C70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82518C74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518C80 size=84
    let mut pc: u32 = 0x82518C80;
    'dispatch: loop {
        match pc {
            0x82518C80 => {
    //   block [0x82518C80..0x82518CD4)
	// 82518C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518C84: 48C8F4E9  bl 0x831a816c
	ctx.lr = 0x82518C88;
	sub_831A8130(ctx, base);
	// 82518C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518C90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82518C94: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518C98: 83AB0018  lwz r29, 0x18(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82518C9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518CA0: 4BFFF1A1  bl 0x82517e40
	ctx.lr = 0x82518CA4;
	sub_82517E40(ctx, base);
	// 82518CA4: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82518CA8: 40980008  bge cr6, 0x82518cb0
	if !ctx.cr[6].lt {
	pc = 0x82518CB0; continue 'dispatch;
	}
	// 82518CAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518CB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518CB8: 4BFFF971  bl 0x82518628
	ctx.lr = 0x82518CBC;
	sub_82518628(ctx, base);
	// 82518CBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518CC0: 41820008  beq 0x82518cc8
	if ctx.cr[0].eq {
	pc = 0x82518CC8; continue 'dispatch;
	}
	// 82518CC4: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82518CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518CD0: 48C8F4EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518CD8 size=268
    let mut pc: u32 = 0x82518CD8;
    'dispatch: loop {
        match pc {
            0x82518CD8 => {
    //   block [0x82518CD8..0x82518DE4)
	// 82518CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518CDC: 48C8F48D  bl 0x831a8168
	ctx.lr = 0x82518CE0;
	sub_831A8130(ctx, base);
	// 82518CE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518CE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518CE8: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	// 82518CEC: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518CF0: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82518CF4: 7D64EB96  divwu r11, r4, r29
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[29].u32;
	// 82518CF8: 1D6B000A  mulli r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 * 10;
	// 82518CFC: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82518D00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82518D04: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82518D08: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82518D0C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518D10: 418200CC  beq 0x82518ddc
	if ctx.cr[0].eq {
	pc = 0x82518DDC; continue 'dispatch;
	}
	// 82518D14: 4BFFF915  bl 0x82518628
	ctx.lr = 0x82518D18;
	sub_82518628(ctx, base);
	// 82518D18: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82518D1C: 418200C0  beq 0x82518ddc
	if ctx.cr[0].eq {
	pc = 0x82518DDC; continue 'dispatch;
	}
	// 82518D20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82518D24: 4BFD1D65  bl 0x824eaa88
	ctx.lr = 0x82518D28;
	sub_824EAA88(ctx, base);
	// 82518D28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82518D2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82518D30: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518D34: 4BFD2425  bl 0x824eb158
	ctx.lr = 0x82518D38;
	sub_824EB158(ctx, base);
	// 82518D38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82518D3C: 488D8F55  bl 0x82df1c90
	ctx.lr = 0x82518D40;
	sub_82DF1C90(ctx, base);
	// 82518D40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82518D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518D48: 388BE43C  addi r4, r11, -0x1bc4
	ctx.r[4].s64 = ctx.r[11].s64 + -7108;
	// 82518D4C: 488DACBD  bl 0x82df3a08
	ctx.lr = 0x82518D50;
	sub_82DF3A08(ctx, base);
	// 82518D50: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82518D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518D58: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82518D5C: 409A0008  bne cr6, 0x82518d64
	if !ctx.cr[6].eq {
	pc = 0x82518D64; continue 'dispatch;
	}
	// 82518D60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82518D68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82518D6C: 4BFE8675  bl 0x825013e0
	ctx.lr = 0x82518D70;
	sub_825013E0(ctx, base);
	// 82518D70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518D74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518D78: 488DA6B1  bl 0x82df3428
	ctx.lr = 0x82518D7C;
	sub_82DF3428(ctx, base);
	// 82518D7C: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518D80: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82518D84: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82518D88: 7D4BEB96  divwu r10, r11, r29
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[29].u32;
	// 82518D8C: 1D4A000A  mulli r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 * 10;
	// 82518D90: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82518D94: 419A0038  beq cr6, 0x82518dcc
	if ctx.cr[6].eq {
	pc = 0x82518DCC; continue 'dispatch;
	}
	// 82518D98: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82518D9C: 41980030  blt cr6, 0x82518dcc
	if ctx.cr[6].lt {
	pc = 0x82518DCC; continue 'dispatch;
	}
	// 82518DA0: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82518DA4: 41990028  bgt cr6, 0x82518dcc
	if ctx.cr[6].gt {
	pc = 0x82518DCC; continue 'dispatch;
	}
	// 82518DA8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82518DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518DB0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82518DB4: 409A0008  bne cr6, 0x82518dbc
	if !ctx.cr[6].eq {
	pc = 0x82518DBC; continue 'dispatch;
	}
	// 82518DB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518DBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518DC0: 4BFE6FD1  bl 0x824ffd90
	ctx.lr = 0x82518DC4;
	sub_824FFD90(ctx, base);
	// 82518DC4: 907C001C  stw r3, 0x1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82518DC8: 4800000C  b 0x82518dd4
	pc = 0x82518DD4; continue 'dispatch;
	// 82518DCC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82518DD0: 917C001C  stw r11, 0x1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82518DD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82518DD8: 488D8EB9  bl 0x82df1c90
	ctx.lr = 0x82518DDC;
	sub_82DF1C90(ctx, base);
	// 82518DDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82518DE0: 48C8F3D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518DE8 size=100
    let mut pc: u32 = 0x82518DE8;
    'dispatch: loop {
        match pc {
            0x82518DE8 => {
    //   block [0x82518DE8..0x82518E4C)
	// 82518DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518DFC: 4BDF062D  bl 0x82309428
	ctx.lr = 0x82518E00;
	sub_82309428(ctx, base);
	// 82518E00: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82518E04: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82518E08: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82518E0C: 993F0020  stb r9, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u8 ) };
	// 82518E10: 391F0014  addi r8, r31, 0x14
	ctx.r[8].s64 = ctx.r[31].s64 + 20;
	// 82518E14: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518E18: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82518E1C: 7D2859AE  stbx r9, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82518E20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82518E24: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82518E28: 4198FFEC  blt cr6, 0x82518e14
	if ctx.cr[6].lt {
	pc = 0x82518E14; continue 'dispatch;
	}
	// 82518E2C: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82518E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518E34: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82518E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82518E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518E50 size=168
    let mut pc: u32 = 0x82518E50;
    'dispatch: loop {
        match pc {
            0x82518E50 => {
    //   block [0x82518E50..0x82518EF8)
	// 82518E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82518E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518E68: 48940089  bl 0x82e58ef0
	ctx.lr = 0x82518E6C;
	sub_82E58EF0(ctx, base);
	// 82518E6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82518E70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82518E74: 394A214C  addi r10, r10, 0x214c
	ctx.r[10].s64 = ctx.r[10].s64 + 8524;
	// 82518E78: 3BCB2100  addi r30, r11, 0x2100
	ctx.r[30].s64 = ctx.r[11].s64 + 8448;
	// 82518E7C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82518E80: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 82518E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82518E88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518E8C: 38A00168  li r5, 0x168
	ctx.r[5].s64 = 360;
	// 82518E90: 6063877C  ori r3, r3, 0x877c
	ctx.r[3].u64 = ctx.r[3].u64 | 34684;
	// 82518E94: 4BDA7545  bl 0x822c03d8
	ctx.lr = 0x82518E98;
	sub_822C03D8(ctx, base);
	// 82518E98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518E9C: 4182000C  beq 0x82518ea8
	if ctx.cr[0].eq {
	pc = 0x82518EA8; continue 'dispatch;
	}
	// 82518EA0: 4BFFE0E1  bl 0x82516f80
	ctx.lr = 0x82518EA4;
	sub_82516F80(ctx, base);
	// 82518EA4: 48000008  b 0x82518eac
	pc = 0x82518EAC; continue 'dispatch;
	// 82518EA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518EAC: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 82518EB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82518EB8: 38A00169  li r5, 0x169
	ctx.r[5].s64 = 361;
	// 82518EBC: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82518EC0: 4BDA7519  bl 0x822c03d8
	ctx.lr = 0x82518EC4;
	sub_822C03D8(ctx, base);
	// 82518EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82518EC8: 4182000C  beq 0x82518ed4
	if ctx.cr[0].eq {
	pc = 0x82518ED4; continue 'dispatch;
	}
	// 82518ECC: 4BFFFF1D  bl 0x82518de8
	ctx.lr = 0x82518ED0;
	sub_82518DE8(ctx, base);
	// 82518ED0: 48000008  b 0x82518ed8
	pc = 0x82518ED8; continue 'dispatch;
	// 82518ED4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518ED8: 907F0098  stw r3, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 82518EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518EE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518EEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82518EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518EF8 size=104
    let mut pc: u32 = 0x82518EF8;
    'dispatch: loop {
        match pc {
            0x82518EF8 => {
    //   block [0x82518EF8..0x82518F60)
	// 82518EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82518F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518F0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518F10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82518F14: 396B214C  addi r11, r11, 0x214c
	ctx.r[11].s64 = ctx.r[11].s64 + 8524;
	// 82518F18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518F1C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82518F20: 4BDA7349  bl 0x822c0268
	ctx.lr = 0x82518F24;
	sub_822C0268(ctx, base);
	// 82518F24: 83DF0098  lwz r30, 0x98(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82518F28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82518F2C: 419A0014  beq cr6, 0x82518f40
	if ctx.cr[6].eq {
	pc = 0x82518F40; continue 'dispatch;
	}
	// 82518F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518F34: 482942B5  bl 0x827ad1e8
	ctx.lr = 0x82518F38;
	sub_827AD1E8(ctx, base);
	// 82518F38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518F3C: 4BDA732D  bl 0x822c0268
	ctx.lr = 0x82518F40;
	sub_822C0268(ctx, base);
	// 82518F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518F44: 4893FE25  bl 0x82e58d68
	ctx.lr = 0x82518F48;
	sub_82E58D68(ctx, base);
	// 82518F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82518F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518F60 size=76
    let mut pc: u32 = 0x82518F60;
    'dispatch: loop {
        match pc {
            0x82518F60 => {
    //   block [0x82518F60..0x82518FAC)
	// 82518F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82518F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518F78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82518F7C: 4BFFFF7D  bl 0x82518ef8
	ctx.lr = 0x82518F80;
	sub_82518EF8(ctx, base);
	// 82518F80: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82518F84: 4182000C  beq 0x82518f90
	if ctx.cr[0].eq {
	pc = 0x82518F90; continue 'dispatch;
	}
	// 82518F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518F8C: 488D944D  bl 0x82df23d8
	ctx.lr = 0x82518F90;
	sub_82DF23D8(ctx, base);
	// 82518F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518F94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518FA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82518FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82518FB0 size=16
    let mut pc: u32 = 0x82518FB0;
    'dispatch: loop {
        match pc {
            0x82518FB0 => {
    //   block [0x82518FB0..0x82518FC0)
	// 82518FB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82518FB4: 396B2168  addi r11, r11, 0x2168
	ctx.r[11].s64 = ctx.r[11].s64 + 8552;
	// 82518FB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518FC0 size=264
    let mut pc: u32 = 0x82518FC0;
    'dispatch: loop {
        match pc {
            0x82518FC0 => {
    //   block [0x82518FC0..0x825190C8)
	// 82518FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518FC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82518FCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82518FD0: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 82518FD4: 48C8FA9D  bl 0x831a8a70
	ctx.lr = 0x82518FD8;
	sub_831A8A40(ctx, base);
	// 82518FD8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518FDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82518FE0: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82518FE4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82518FE8: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82518FEC: C1640008  lfs f11, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82518FF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82518FF4: 3BE30400  addi r31, r3, 0x400
	ctx.r[31].s64 = ctx.r[3].s64 + 1024;
	// 82518FF8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82518FFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82519000: C36A94AC  lfs f27, -0x6b54(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82519004: EFC06824  fdivs f30, f0, f13
	ctx.f[30].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82519008: EFA06024  fdivs f29, f0, f12
	ctx.f[29].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8251900C: EF805824  fdivs f28, f0, f11
	ctx.f[28].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82519010: C3EB216C  lfs f31, 0x216c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8556 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82519014: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82519018: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8251901C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82519020: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82519024: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82519028: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8251902C: EF4006F2  fmuls f26, f0, f27
	ctx.f[26].f64 = (((ctx.f[0].f64 * ctx.f[27].f64) as f32) as f64);
	// 82519030: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 82519034: 48C92475  bl 0x831ab4a8
	ctx.lr = 0x82519038;
	sub_831AB4A8(ctx, base);
	// 82519038: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8251903C: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 82519040: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 82519044: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82519048: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8251904C: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82519050: A161005E  lhz r11, 0x5e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82519054: B17FFC00  sth r11, -0x400(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(-1024 as u32), ctx.r[11].u16 ) };
	// 82519058: 48C92451  bl 0x831ab4a8
	ctx.lr = 0x8251905C;
	sub_831AB4A8(ctx, base);
	// 8251905C: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82519060: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 82519064: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 82519068: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8251906C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82519070: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82519074: A161005E  lhz r11, 0x5e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82519078: B17FFE00  sth r11, -0x200(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(-512 as u32), ctx.r[11].u16 ) };
	// 8251907C: 48C9242D  bl 0x831ab4a8
	ctx.lr = 0x82519080;
	sub_831AB4A8(ctx, base);
	// 82519080: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82519084: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82519088: 2F1E0100  cmpwi cr6, r30, 0x100
	ctx.cr[6].compare_i32(ctx.r[30].s32, 256, &mut ctx.xer);
	// 8251908C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82519090: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82519094: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 82519098: A161005E  lhz r11, 0x5e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 8251909C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 825190A0: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 825190A4: 409AFF70  bne cr6, 0x82519014
	if !ctx.cr[6].eq {
	pc = 0x82519014; continue 'dispatch;
	}
	// 825190A8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825190AC: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 825190B0: 48C8FA0D  bl 0x831a8abc
	ctx.lr = 0x825190B4;
	sub_831A8A8C(ctx, base);
	// 825190B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825190B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825190BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825190C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825190C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825190C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825190C8 size=56
    let mut pc: u32 = 0x825190C8;
    'dispatch: loop {
        match pc {
            0x825190C8 => {
    //   block [0x825190C8..0x82519100)
	// 825190C8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825190CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825190D0: 396BCEF0  addi r11, r11, -0x3110
	ctx.r[11].s64 = ctx.r[11].s64 + -12560;
	// 825190D4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 825190D8: 394A2168  addi r10, r10, 0x2168
	ctx.r[10].s64 = ctx.r[10].s64 + 8552;
	// 825190DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825190E0: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 825190E4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825190E8: 99030010  stb r8, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u8 ) };
	// 825190EC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519100 size=68
    let mut pc: u32 = 0x82519100;
    'dispatch: loop {
        match pc {
            0x82519100 => {
    //   block [0x82519100..0x82519144)
	// 82519100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251910C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82519118: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251911C: 396B2168  addi r11, r11, 0x2168
	ctx.r[11].s64 = ctx.r[11].s64 + 8552;
	// 82519120: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519124: 41820008  beq 0x8251912c
	if ctx.cr[0].eq {
	pc = 0x8251912C; continue 'dispatch;
	}
	// 82519128: 488D92B1  bl 0x82df23d8
	ctx.lr = 0x8251912C;
	sub_82DF23D8(ctx, base);
	// 8251912C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251913C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82519148 size=32
    let mut pc: u32 = 0x82519148;
    'dispatch: loop {
        match pc {
            0x82519148 => {
    //   block [0x82519148..0x82519168)
	// 82519148: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251914C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82519154: C1AB2160  lfs f13, 0x2160(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8544 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82519158: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8251915C: 4099000C  ble cr6, 0x82519168
	if !ctx.cr[6].gt {
		sub_82519168(ctx, base);
		return;
	}
	// 82519160: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82519164: 48000010  b 0x82519174
	sub_82519168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82519168 size=44
    let mut pc: u32 = 0x82519168;
    'dispatch: loop {
        match pc {
            0x82519168 => {
    //   block [0x82519168..0x82519194)
	// 82519168: C1AA215C  lfs f13, 0x215c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8540 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8251916C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82519170: 41980008  blt cr6, 0x82519178
	if ctx.cr[6].lt {
	pc = 0x82519178; continue 'dispatch;
	}
	// 82519174: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82519178: D1A30020  stfs f13, 0x20(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8251917C: C1AB2160  lfs f13, 0x2160(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8544 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82519180: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519184: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82519188: 4099000C  ble cr6, 0x82519194
	if !ctx.cr[6].gt {
		sub_82519194(ctx, base);
		return;
	}
	// 8251918C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82519190: 48000010  b 0x825191a0
	sub_82519194(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82519194 size=60
    let mut pc: u32 = 0x82519194;
    'dispatch: loop {
        match pc {
            0x82519194 => {
    //   block [0x82519194..0x825191D0)
	// 82519194: C1AA215C  lfs f13, 0x215c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8540 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82519198: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8251919C: 41980008  blt cr6, 0x825191a4
	if ctx.cr[6].lt {
	pc = 0x825191A4; continue 'dispatch;
	}
	// 825191A0: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 825191A4: D1A30024  stfs f13, 0x24(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825191A8: C1AB2160  lfs f13, 0x2160(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8544 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825191AC: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825191B0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825191B4: 41990010  bgt cr6, 0x825191c4
	if ctx.cr[6].gt {
	pc = 0x825191C4; continue 'dispatch;
	}
	// 825191B8: C1AA215C  lfs f13, 0x215c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8540 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825191BC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825191C0: 40980008  bge cr6, 0x825191c8
	if !ctx.cr[6].lt {
	pc = 0x825191C8; continue 'dispatch;
	}
	// 825191C4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825191C8: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 825191CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825191D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825191D0 size=152
    let mut pc: u32 = 0x825191D0;
    'dispatch: loop {
        match pc {
            0x825191D0 => {
    //   block [0x825191D0..0x82519268)
	// 825191D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825191D4: 48C8EF95  bl 0x831a8168
	ctx.lr = 0x825191D8;
	sub_831A8130(ctx, base);
	// 825191D8: 9421F970  stwu r1, -0x690(r1)
	ea = ctx.r[1].u32.wrapping_add(-1680 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825191DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825191E0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825191E4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825191E8: 57DD063E  clrlwi r29, r30, 0x18
	ctx.r[29].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 825191EC: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825191F0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825191F4: 409A0018  bne cr6, 0x8251920c
	if !ctx.cr[6].eq {
	pc = 0x8251920C; continue 'dispatch;
	}
	// 825191F8: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 825191FC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82519200: 4BF0CF31  bl 0x82426130
	ctx.lr = 0x82519204;
	sub_82426130(ctx, base);
	// 82519204: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82519208: 40820058  bne 0x82519260
	if !ctx.cr[0].eq {
	pc = 0x82519260; continue 'dispatch;
	}
	// 8251920C: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82519210: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82519214: 419A000C  beq cr6, 0x82519220
	if ctx.cr[6].eq {
	pc = 0x82519220; continue 'dispatch;
	}
	// 82519218: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 8251921C: 4800000C  b 0x82519228
	pc = 0x82519228; continue 'dispatch;
	// 82519220: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82519224: 396BCEF0  addi r11, r11, -0x3110
	ctx.r[11].s64 = ctx.r[11].s64 + -12560;
	// 82519228: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8251922C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82519230: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519234: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519268 size=20
    let mut pc: u32 = 0x82519268;
    'dispatch: loop {
        match pc {
            0x82519268 => {
    //   block [0x82519268..0x8251927C)
	// 82519268: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251926C: 396B2174  addi r11, r11, 0x2174
	ctx.r[11].s64 = ctx.r[11].s64 + 8564;
	// 82519270: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519274: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82519278: 488D92D8  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519280 size=8
    let mut pc: u32 = 0x82519280;
    'dispatch: loop {
        match pc {
            0x82519280 => {
    //   block [0x82519280..0x82519288)
	// 82519280: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82519284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519288 size=8
    let mut pc: u32 = 0x82519288;
    'dispatch: loop {
        match pc {
            0x82519288 => {
    //   block [0x82519288..0x82519290)
	// 82519288: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8251928C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519290 size=8
    let mut pc: u32 = 0x82519290;
    'dispatch: loop {
        match pc {
            0x82519290 => {
    //   block [0x82519290..0x82519298)
	// 82519290: 80630040  lwz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82519294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519298 size=136
    let mut pc: u32 = 0x82519298;
    'dispatch: loop {
        match pc {
            0x82519298 => {
    //   block [0x82519298..0x82519320)
	// 82519298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251929C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825192A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825192A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825192A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825192AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825192B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825192B4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825192B8: 409A0020  bne cr6, 0x825192d8
	if !ctx.cr[6].eq {
	pc = 0x825192D8; continue 'dispatch;
	}
	// 825192BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825192C0: 419A0048  beq cr6, 0x82519308
	if ctx.cr[6].eq {
	pc = 0x82519308; continue 'dispatch;
	}
	// 825192C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825192C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825192CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825192D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825192D4: 48000034  b 0x82519308
	pc = 0x82519308; continue 'dispatch;
	// 825192D8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825192DC: 419A002C  beq cr6, 0x82519308
	if ctx.cr[6].eq {
	pc = 0x82519308; continue 'dispatch;
	}
	// 825192E0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825192E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825192E8: 388BCF98  addi r4, r11, -0x3068
	ctx.r[4].s64 = ctx.r[11].s64 + -12392;
	// 825192EC: 48C8EE0D  bl 0x831a80f8
	ctx.lr = 0x825192F0;
	sub_831A80F8(ctx, base);
	// 825192F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825192F4: 4182000C  beq 0x82519300
	if ctx.cr[0].eq {
	pc = 0x82519300; continue 'dispatch;
	}
	// 825192F8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825192FC: 4800000C  b 0x82519308
	pc = 0x82519308; continue 'dispatch;
	// 82519300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82519304: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251930C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519314: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82519318: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251931C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519320 size=64
    let mut pc: u32 = 0x82519320;
    'dispatch: loop {
        match pc {
            0x82519320 => {
    //   block [0x82519320..0x82519360)
	// 82519320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251932C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519334: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82519338: 488D91A1  bl 0x82df24d8
	ctx.lr = 0x8251933C;
	sub_82DF24D8(ctx, base);
	// 8251933C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82519340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519344: 396B219C  addi r11, r11, 0x219c
	ctx.r[11].s64 = ctx.r[11].s64 + 8604;
	// 82519348: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251934C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519360 size=92
    let mut pc: u32 = 0x82519360;
    'dispatch: loop {
        match pc {
            0x82519360 => {
    //   block [0x82519360..0x825193BC)
	// 82519360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251936C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519378: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251937C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82519380: 396B2174  addi r11, r11, 0x2174
	ctx.r[11].s64 = ctx.r[11].s64 + 8564;
	// 82519384: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82519388: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251938C: 488D91C5  bl 0x82df2550
	ctx.lr = 0x82519390;
	sub_82DF2550(ctx, base);
	// 82519390: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82519394: 4182000C  beq 0x825193a0
	if ctx.cr[0].eq {
	pc = 0x825193A0; continue 'dispatch;
	}
	// 82519398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251939C: 4BDA6ECD  bl 0x822c0268
	ctx.lr = 0x825193A0;
	sub_822C0268(ctx, base);
	// 825193A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825193A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825193A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825193AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825193B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825193B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825193B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825193C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825193C0 size=80
    let mut pc: u32 = 0x825193C0;
    'dispatch: loop {
        match pc {
            0x825193C0 => {
    //   block [0x825193C0..0x82519410)
	// 825193C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825193C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825193C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825193CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825193D0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825193D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825193D8: 816B1664  lwz r11, 0x1664(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5732 as u32) ) } as u64;
	// 825193DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825193E0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825193E4: 409A0008  bne cr6, 0x825193ec
	if !ctx.cr[6].eq {
	pc = 0x825193EC; continue 'dispatch;
	}
	// 825193E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825193EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825193F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825193F4: 488D8805  bl 0x82df1bf8
	ctx.lr = 0x825193F8;
	sub_82DF1BF8(ctx, base);
	// 825193F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825193FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519410 size=72
    let mut pc: u32 = 0x82519410;
    'dispatch: loop {
        match pc {
            0x82519410 => {
    //   block [0x82519410..0x82519458)
	// 82519410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251941C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82519420: 419A001C  beq cr6, 0x8251943c
	if ctx.cr[6].eq {
	pc = 0x8251943C; continue 'dispatch;
	}
	// 82519424: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82519428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251942C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82519430: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82519434: 4BFFFE65  bl 0x82519298
	ctx.lr = 0x82519438;
	sub_82519298(ctx, base);
	// 82519438: 48000010  b 0x82519448
	pc = 0x82519448; continue 'dispatch;
	// 8251943C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82519440: 396BCF98  addi r11, r11, -0x3068
	ctx.r[11].s64 = ctx.r[11].s64 + -12392;
	// 82519444: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251944C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519458 size=72
    let mut pc: u32 = 0x82519458;
    'dispatch: loop {
        match pc {
            0x82519458 => {
    //   block [0x82519458..0x825194A0)
	// 82519458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251945C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519468: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251946C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519470: 48B84FE1  bl 0x8309e450
	ctx.lr = 0x82519474;
	sub_8309E450(ctx, base);
	// 82519474: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82519478: 4BDAF841  bl 0x822c8cb8
	ctx.lr = 0x8251947C;
	sub_822C8CB8(ctx, base);
	// 8251947C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519484: 419A0008  beq cr6, 0x8251948c
	if ctx.cr[6].eq {
	pc = 0x8251948C; continue 'dispatch;
	}
	// 82519488: 4BDA7409  bl 0x822c0890
	ctx.lr = 0x8251948C;
	sub_822C0890(ctx, base);
	// 8251948C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251949C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825194A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825194A0 size=344
    let mut pc: u32 = 0x825194A0;
    'dispatch: loop {
        match pc {
            0x825194A0 => {
    //   block [0x825194A0..0x825195F8)
	// 825194A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825194A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825194A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825194AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825194B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825194B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825194B8: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 825194BC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825194C0: 419800DC  blt cr6, 0x8251959c
	if ctx.cr[6].lt {
	pc = 0x8251959C; continue 'dispatch;
	}
	// 825194C4: 409A0114  bne cr6, 0x825195d8
	if !ctx.cr[6].eq {
	pc = 0x825195D8; continue 'dispatch;
	}
	// 825194C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825194CC: 4BFFFEF5  bl 0x825193c0
	ctx.lr = 0x825194D0;
	sub_825193C0(ctx, base);
	// 825194D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825194D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825194D8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825194DC: 409A0008  bne cr6, 0x825194e4
	if !ctx.cr[6].eq {
	pc = 0x825194E4; continue 'dispatch;
	}
	// 825194E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825194E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825194E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825194EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825194F0: 4E800421  bctrl
	ctx.lr = 0x825194F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825194F4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 825194F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825194FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82519500: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82519504: 488D878D  bl 0x82df1c90
	ctx.lr = 0x82519508;
	sub_82DF1C90(ctx, base);
	// 82519508: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251950C: 418200CC  beq 0x825195d8
	if ctx.cr[0].eq {
	pc = 0x825195D8; continue 'dispatch;
	}
	// 82519510: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82519514: 4BFFFEAD  bl 0x825193c0
	ctx.lr = 0x82519518;
	sub_825193C0(ctx, base);
	// 82519518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251951C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519520: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519524: 409A0008  bne cr6, 0x8251952c
	if !ctx.cr[6].eq {
	pc = 0x8251952C; continue 'dispatch;
	}
	// 82519528: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251952C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82519534: 388A21D4  addi r4, r10, 0x21d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8660;
	// 82519538: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251953C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519540: 4E800421  bctrl
	ctx.lr = 0x82519544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519548: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251954C: 488D8745  bl 0x82df1c90
	ctx.lr = 0x82519550;
	sub_82DF1C90(ctx, base);
	// 82519550: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 82519554: 409A0040  bne cr6, 0x82519594
	if !ctx.cr[6].eq {
	pc = 0x82519594; continue 'dispatch;
	}
	// 82519558: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251955C: 4BFFFE65  bl 0x825193c0
	ctx.lr = 0x82519560;
	sub_825193C0(ctx, base);
	// 82519560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519564: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519568: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251956C: 409A0008  bne cr6, 0x82519574
	if !ctx.cr[6].eq {
	pc = 0x82519574; continue 'dispatch;
	}
	// 82519570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82519574: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251957C: 388A21C0  addi r4, r10, 0x21c0
	ctx.r[4].s64 = ctx.r[10].s64 + 8640;
	// 82519580: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519588: 4E800421  bctrl
	ctx.lr = 0x8251958C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251958C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82519590: 488D8701  bl 0x82df1c90
	ctx.lr = 0x82519594;
	sub_82DF1C90(ctx, base);
	// 82519594: 93FE0040  stw r31, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82519598: 48000040  b 0x825195d8
	pc = 0x825195D8; continue 'dispatch;
	// 8251959C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825195A0: 4BFFFE21  bl 0x825193c0
	ctx.lr = 0x825195A4;
	sub_825193C0(ctx, base);
	// 825195A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825195A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825195AC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825195B0: 409A0008  bne cr6, 0x825195b8
	if !ctx.cr[6].eq {
	pc = 0x825195B8; continue 'dispatch;
	}
	// 825195B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825195B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825195BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825195C0: 388A21C0  addi r4, r10, 0x21c0
	ctx.r[4].s64 = ctx.r[10].s64 + 8640;
	// 825195C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825195C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825195CC: 4E800421  bctrl
	ctx.lr = 0x825195D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825195D0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825195D4: 488D86BD  bl 0x82df1c90
	ctx.lr = 0x825195D8;
	sub_82DF1C90(ctx, base);
	// 825195D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825195DC: 997E003C  stb r11, 0x3c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 825195E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825195E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825195E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825195EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825195F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825195F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825195F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825195F8 size=92
    let mut pc: u32 = 0x825195F8;
    'dispatch: loop {
        match pc {
            0x825195F8 => {
    //   block [0x825195F8..0x82519654)
	// 825195F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825195FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519604: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82519608: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8251960C: 409A0038  bne cr6, 0x82519644
	if !ctx.cr[6].eq {
	pc = 0x82519644; continue 'dispatch;
	}
	// 82519610: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519614: 4BFFFDAD  bl 0x825193c0
	ctx.lr = 0x82519618;
	sub_825193C0(ctx, base);
	// 82519618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251961C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519620: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519624: 409A0008  bne cr6, 0x8251962c
	if !ctx.cr[6].eq {
	pc = 0x8251962C; continue 'dispatch;
	}
	// 82519628: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251962C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519630: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82519634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519638: 4E800421  bctrl
	ctx.lr = 0x8251963C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251963C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519640: 488D8651  bl 0x82df1c90
	ctx.lr = 0x82519644;
	sub_82DF1C90(ctx, base);
	// 82519644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519658 size=92
    let mut pc: u32 = 0x82519658;
    'dispatch: loop {
        match pc {
            0x82519658 => {
    //   block [0x82519658..0x825196B4)
	// 82519658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251965C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519664: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82519668: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8251966C: 409A0038  bne cr6, 0x825196a4
	if !ctx.cr[6].eq {
	pc = 0x825196A4; continue 'dispatch;
	}
	// 82519670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519674: 4BFFFD4D  bl 0x825193c0
	ctx.lr = 0x82519678;
	sub_825193C0(ctx, base);
	// 82519678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251967C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519680: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519684: 409A0008  bne cr6, 0x8251968c
	if !ctx.cr[6].eq {
	pc = 0x8251968C; continue 'dispatch;
	}
	// 82519688: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251968C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519690: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82519694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519698: 4E800421  bctrl
	ctx.lr = 0x8251969C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251969C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825196A0: 488D85F1  bl 0x82df1c90
	ctx.lr = 0x825196A4;
	sub_82DF1C90(ctx, base);
	// 825196A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825196A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825196AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825196B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825196B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825196B8 size=92
    let mut pc: u32 = 0x825196B8;
    'dispatch: loop {
        match pc {
            0x825196B8 => {
    //   block [0x825196B8..0x82519714)
	// 825196B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825196BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825196C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825196C4: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 825196C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 825196CC: 409A0038  bne cr6, 0x82519704
	if !ctx.cr[6].eq {
	pc = 0x82519704; continue 'dispatch;
	}
	// 825196D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825196D4: 4BFFFCED  bl 0x825193c0
	ctx.lr = 0x825196D8;
	sub_825193C0(ctx, base);
	// 825196D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825196DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825196E0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825196E4: 409A0008  bne cr6, 0x825196ec
	if !ctx.cr[6].eq {
	pc = 0x825196EC; continue 'dispatch;
	}
	// 825196E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825196EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825196F0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 825196F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825196F8: 4E800421  bctrl
	ctx.lr = 0x825196FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825196FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519700: 488D8591  bl 0x82df1c90
	ctx.lr = 0x82519704;
	sub_82DF1C90(ctx, base);
	// 82519704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519718 size=128
    let mut pc: u32 = 0x82519718;
    'dispatch: loop {
        match pc {
            0x82519718 => {
    //   block [0x82519718..0x82519798)
	// 82519718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251971C: 48C8EA51  bl 0x831a816c
	ctx.lr = 0x82519720;
	sub_831A8130(ctx, base);
	// 82519720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519724: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82519728: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251972C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82519730: 3BEB7048  addi r31, r11, 0x7048
	ctx.r[31].s64 = ctx.r[11].s64 + 28744;
	// 82519734: 816A7050  lwz r11, 0x7050(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28752 as u32) ) } as u64;
	// 82519738: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8251973C: 40820024  bne 0x82519760
	if !ctx.cr[0].eq {
	pc = 0x82519760; continue 'dispatch;
	}
	// 82519740: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 82519744: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82519748: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8251974C: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 82519750: 39089410  addi r8, r8, -0x6bf0
	ctx.r[8].s64 = ctx.r[8].s64 + -27632;
	// 82519754: 916A7050  stw r11, 0x7050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28752 as u32), ctx.r[11].u32 ) };
	// 82519758: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251975C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82519760: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82519764: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82519768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251976C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 82519770: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82519774: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82519778: 480B91A9  bl 0x825d2920
	ctx.lr = 0x8251977C;
	sub_825D2920(ctx, base);
	// 8251977C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82519780: 4182000C  beq 0x8251978c
	if ctx.cr[0].eq {
	pc = 0x8251978C; continue 'dispatch;
	}
	// 82519784: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82519788: 48000008  b 0x82519790
	pc = 0x82519790; continue 'dispatch;
	// 8251978C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82519790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519794: 48C8EA28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519798 size=160
    let mut pc: u32 = 0x82519798;
    'dispatch: loop {
        match pc {
            0x82519798 => {
    //   block [0x82519798..0x82519838)
	// 82519798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251979C: 48C8E9D1  bl 0x831a816c
	ctx.lr = 0x825197A0;
	sub_831A8130(ctx, base);
	// 825197A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825197A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825197A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825197AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825197B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825197B4: 388B21E8  addi r4, r11, 0x21e8
	ctx.r[4].s64 = ctx.r[11].s64 + 8680;
	// 825197B8: 488DA251  bl 0x82df3a08
	ctx.lr = 0x825197BC;
	sub_82DF3A08(ctx, base);
	// 825197BC: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 825197C0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825197C4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 825197C8: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 825197CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825197D0: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 825197D4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825197D8: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825197DC: 4BFFFF3D  bl 0x82519718
	ctx.lr = 0x825197E0;
	sub_82519718(ctx, base);
	// 825197E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825197E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825197E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825197EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825197F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825197F4: 488E0755  bl 0x82df9f48
	ctx.lr = 0x825197F8;
	sub_82DF9F48(ctx, base);
	// 825197F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825197FC: 488D9C2D  bl 0x82df3428
	ctx.lr = 0x82519800;
	sub_82DF3428(ctx, base);
	// 82519800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519804: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82519808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251980C: 9BFE003C  stb r31, 0x3c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[31].u8 ) };
	// 82519810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82519814: 93FE0040  stw r31, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82519818: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251981C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519820: 4E800421  bctrl
	ctx.lr = 0x82519824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519828: 48B84C11  bl 0x8309e438
	ctx.lr = 0x8251982C;
	sub_8309E438(ctx, base);
	// 8251982C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519830: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82519834: 48C8E988  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519838 size=92
    let mut pc: u32 = 0x82519838;
    'dispatch: loop {
        match pc {
            0x82519838 => {
    //   block [0x82519838..0x82519894)
	// 82519838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82519844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251984C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82519850: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82519854: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82519858: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 8251985C: 488DA375  bl 0x82df3bd0
	ctx.lr = 0x82519860;
	sub_82DF3BD0(ctx, base);
	// 82519860: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82519864: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82519868: 488DA369  bl 0x82df3bd0
	ctx.lr = 0x8251986C;
	sub_82DF3BD0(ctx, base);
	// 8251986C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82519870: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82519874: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82519878: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8251987C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251988C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519898 size=236
    let mut pc: u32 = 0x82519898;
    'dispatch: loop {
        match pc {
            0x82519898 => {
    //   block [0x82519898..0x82519984)
	// 82519898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825198A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825198A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825198A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825198AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825198B0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825198B4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825198B8: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 825198BC: 409A0008  bne cr6, 0x825198c4
	if !ctx.cr[6].eq {
	pc = 0x825198C4; continue 'dispatch;
	}
	// 825198C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825198C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825198C8: 4BFEEED9  bl 0x825087a0
	ctx.lr = 0x825198CC;
	sub_825087A0(ctx, base);
	// 825198CC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825198D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825198D4: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 825198D8: 488DA131  bl 0x82df3a08
	ctx.lr = 0x825198DC;
	sub_82DF3A08(ctx, base);
	// 825198DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825198E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825198E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825198E8: 4BFEEE99  bl 0x82508780
	ctx.lr = 0x825198EC;
	sub_82508780(ctx, base);
	// 825198EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825198F0: 488D9B39  bl 0x82df3428
	ctx.lr = 0x825198F4;
	sub_82DF3428(ctx, base);
	// 825198F4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825198F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825198FC: 808BE268  lwz r4, -0x1d98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7576 as u32) ) } as u64;
	// 82519900: 488DA109  bl 0x82df3a08
	ctx.lr = 0x82519904;
	sub_82DF3A08(ctx, base);
	// 82519904: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82519908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251990C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519910: 4BFEEE71  bl 0x82508780
	ctx.lr = 0x82519914;
	sub_82508780(ctx, base);
	// 82519914: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519918: 488D9B11  bl 0x82df3428
	ctx.lr = 0x8251991C;
	sub_82DF3428(ctx, base);
	// 8251991C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82519920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519924: 808BE25C  lwz r4, -0x1da4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7588 as u32) ) } as u64;
	// 82519928: 488DA0E1  bl 0x82df3a08
	ctx.lr = 0x8251992C;
	sub_82DF3A08(ctx, base);
	// 8251992C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82519930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519938: 4BFEEE49  bl 0x82508780
	ctx.lr = 0x8251993C;
	sub_82508780(ctx, base);
	// 8251993C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519940: 488D9AE9  bl 0x82df3428
	ctx.lr = 0x82519944;
	sub_82DF3428(ctx, base);
	// 82519944: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82519948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251994C: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 82519950: 488DA0B9  bl 0x82df3a08
	ctx.lr = 0x82519954;
	sub_82DF3A08(ctx, base);
	// 82519954: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82519958: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251995C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519960: 4BFEEE21  bl 0x82508780
	ctx.lr = 0x82519964;
	sub_82508780(ctx, base);
	// 82519964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82519968: 488D9AC1  bl 0x82df3428
	ctx.lr = 0x8251996C;
	sub_82DF3428(ctx, base);
	// 8251996C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519978: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251997C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519988 size=12
    let mut pc: u32 = 0x82519988;
    'dispatch: loop {
        match pc {
            0x82519988 => {
    //   block [0x82519988..0x82519994)
	// 82519988: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8251998C: 91630100  stw r11, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 82519990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519998 size=100
    let mut pc: u32 = 0x82519998;
    'dispatch: loop {
        match pc {
            0x82519998 => {
    //   block [0x82519998..0x825199FC)
	// 82519998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825199A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825199A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825199A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825199AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825199B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825199B4: 4893FCCD  bl 0x82e59680
	ctx.lr = 0x825199B8;
	sub_82E59680(ctx, base);
	// 825199B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825199BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825199C0: 396B2244  addi r11, r11, 0x2244
	ctx.r[11].s64 = ctx.r[11].s64 + 8772;
	// 825199C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825199C8: 488D9729  bl 0x82df30f0
	ctx.lr = 0x825199CC;
	sub_82DF30F0(ctx, base);
	// 825199CC: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 825199D0: 488D9721  bl 0x82df30f0
	ctx.lr = 0x825199D4;
	sub_82DF30F0(ctx, base);
	// 825199D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825199D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825199DC: 4BFFFE5D  bl 0x82519838
	ctx.lr = 0x825199E0;
	sub_82519838(ctx, base);
	// 825199E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825199E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825199E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825199EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825199F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825199F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825199F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519A00 size=92
    let mut pc: u32 = 0x82519A00;
    'dispatch: loop {
        match pc {
            0x82519A00 => {
    //   block [0x82519A00..0x82519A5C)
	// 82519A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519A08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82519A0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519A18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82519A1C: 48000020  b 0x82519a3c
	pc = 0x82519A3C; continue 'dispatch;
	// 82519A20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519A24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82519A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519A2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519A34: 4E800421  bctrl
	ctx.lr = 0x82519A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519A38: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82519A3C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82519A40: 409AFFE0  bne cr6, 0x82519a20
	if !ctx.cr[6].eq {
	pc = 0x82519A20; continue 'dispatch;
	}
	// 82519A44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519A50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82519A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519A60 size=352
    let mut pc: u32 = 0x82519A60;
    'dispatch: loop {
        match pc {
            0x82519A60 => {
    //   block [0x82519A60..0x82519BC0)
	// 82519A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519A64: 48C8E709  bl 0x831a816c
	ctx.lr = 0x82519A68;
	sub_831A8130(ctx, base);
	// 82519A68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519A70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82519A74: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 82519A78: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82519A7C: 488D9DFD  bl 0x82df3878
	ctx.lr = 0x82519A80;
	sub_82DF3878(ctx, base);
	// 82519A80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82519A84: 397F00F8  addi r11, r31, 0xf8
	ctx.r[11].s64 = ctx.r[31].s64 + 248;
	// 82519A88: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 82519A8C: 807F00FC  lwz r3, 0xfc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82519A90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519A94: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 82519A98: 419A0008  beq cr6, 0x82519aa0
	if ctx.cr[6].eq {
	pc = 0x82519AA0; continue 'dispatch;
	}
	// 82519A9C: 4BDA6DF5  bl 0x822c0890
	ctx.lr = 0x82519AA0;
	sub_822C0890(ctx, base);
	// 82519AA0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82519AA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82519AA8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82519AAC: 4BFD0FDD  bl 0x824eaa88
	ctx.lr = 0x82519AB0;
	sub_824EAA88(ctx, base);
	// 82519AB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519AB4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519AB8: 4BFD1BB1  bl 0x824eb668
	ctx.lr = 0x82519ABC;
	sub_824EB668(ctx, base);
	// 82519ABC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82519AC0: 488D81D1  bl 0x82df1c90
	ctx.lr = 0x82519AC4;
	sub_82DF1C90(ctx, base);
	// 82519AC4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82519AC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519ACC: 419A0008  beq cr6, 0x82519ad4
	if ctx.cr[6].eq {
	pc = 0x82519AD4; continue 'dispatch;
	}
	// 82519AD0: 4BDA6DC1  bl 0x822c0890
	ctx.lr = 0x82519AD4;
	sub_822C0890(ctx, base);
	// 82519AD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82519AD8: 4BEB9E31  bl 0x823d3908
	ctx.lr = 0x82519ADC;
	sub_823D3908(ctx, base);
	// 82519ADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519AE0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82519AE4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519AE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519AEC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82519AF0: 419A0024  beq cr6, 0x82519b14
	if ctx.cr[6].eq {
	pc = 0x82519B14; continue 'dispatch;
	}
	// 82519AF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82519AF8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82519AFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519B00: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82519B04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82519B08: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82519B0C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519B10: 4082FFE8  bne 0x82519af8
	if !ctx.cr[0].eq {
	pc = 0x82519AF8; continue 'dispatch;
	}
	// 82519B14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82519B18: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82519B1C: 4BFF59AD  bl 0x8250f4c8
	ctx.lr = 0x82519B20;
	sub_8250F4C8(ctx, base);
	// 82519B20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519B24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519B28: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519B2C: 409A0008  bne cr6, 0x82519b34
	if !ctx.cr[6].eq {
	pc = 0x82519B34; continue 'dispatch;
	}
	// 82519B30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519B34: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 82519B38: 4BFEE999  bl 0x825084d0
	ctx.lr = 0x82519B3C;
	sub_825084D0(ctx, base);
	// 82519B3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82519B40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82519B44: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82519B48: 388A2248  addi r4, r10, 0x2248
	ctx.r[4].s64 = ctx.r[10].s64 + 8776;
	// 82519B4C: 38A000A5  li r5, 0xa5
	ctx.r[5].s64 = 165;
	// 82519B50: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82519B54: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82519B58: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82519B5C: 4893EEE5  bl 0x82e58a40
	ctx.lr = 0x82519B60;
	sub_82E58A40(ctx, base);
	// 82519B60: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82519B64: 488D812D  bl 0x82df1c90
	ctx.lr = 0x82519B68;
	sub_82DF1C90(ctx, base);
	// 82519B68: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82519B6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519B70: 419A0008  beq cr6, 0x82519b78
	if ctx.cr[6].eq {
	pc = 0x82519B78; continue 'dispatch;
	}
	// 82519B74: 4BDA6D1D  bl 0x822c0890
	ctx.lr = 0x82519B78;
	sub_822C0890(ctx, base);
	// 82519B78: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82519B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519B80: 419A0008  beq cr6, 0x82519b88
	if ctx.cr[6].eq {
	pc = 0x82519B88; continue 'dispatch;
	}
	// 82519B84: 4BDA6D0D  bl 0x822c0890
	ctx.lr = 0x82519B88;
	sub_822C0890(ctx, base);
	// 82519B88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82519B8C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82519B90: 4BFF5939  bl 0x8250f4c8
	ctx.lr = 0x82519B94;
	sub_8250F4C8(ctx, base);
	// 82519B94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519B9C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519BA0: 409A0008  bne cr6, 0x82519ba8
	if !ctx.cr[6].eq {
	pc = 0x82519BA8; continue 'dispatch;
	}
	// 82519BA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519BA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82519BAC: 4BFEE915  bl 0x825084c0
	ctx.lr = 0x82519BB0;
	sub_825084C0(ctx, base);
	// 82519BB0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82519BB4: 488D80DD  bl 0x82df1c90
	ctx.lr = 0x82519BB8;
	sub_82DF1C90(ctx, base);
	// 82519BB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82519BBC: 48C8E600  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519BC0 size=432
    let mut pc: u32 = 0x82519BC0;
    'dispatch: loop {
        match pc {
            0x82519BC0 => {
    //   block [0x82519BC0..0x82519D70)
	// 82519BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519BC4: 48C8E5A9  bl 0x831a816c
	ctx.lr = 0x82519BC8;
	sub_831A8130(ctx, base);
	// 82519BC8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519BCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82519BD0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82519BD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82519BD8: 4BEB9D31  bl 0x823d3908
	ctx.lr = 0x82519BDC;
	sub_823D3908(ctx, base);
	// 82519BDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519BE0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82519BE4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519BE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519BEC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82519BF0: 419A0024  beq cr6, 0x82519c14
	if ctx.cr[6].eq {
	pc = 0x82519C14; continue 'dispatch;
	}
	// 82519BF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82519BF8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82519BFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519C00: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82519C04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82519C08: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82519C0C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519C10: 4082FFE8  bne 0x82519bf8
	if !ctx.cr[0].eq {
	pc = 0x82519BF8; continue 'dispatch;
	}
	// 82519C14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519C18: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82519C1C: 4BFF58AD  bl 0x8250f4c8
	ctx.lr = 0x82519C20;
	sub_8250F4C8(ctx, base);
	// 82519C20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519C28: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519C2C: 409A0008  bne cr6, 0x82519c34
	if !ctx.cr[6].eq {
	pc = 0x82519C34; continue 'dispatch;
	}
	// 82519C30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82519C34: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 82519C38: 4BFEE899  bl 0x825084d0
	ctx.lr = 0x82519C3C;
	sub_825084D0(ctx, base);
	// 82519C3C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82519C40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82519C44: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82519C48: 388A2248  addi r4, r10, 0x2248
	ctx.r[4].s64 = ctx.r[10].s64 + 8776;
	// 82519C4C: 38A000AC  li r5, 0xac
	ctx.r[5].s64 = 172;
	// 82519C50: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 82519C54: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82519C58: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82519C5C: 4893EDE5  bl 0x82e58a40
	ctx.lr = 0x82519C60;
	sub_82E58A40(ctx, base);
	// 82519C60: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82519C64: 488D802D  bl 0x82df1c90
	ctx.lr = 0x82519C68;
	sub_82DF1C90(ctx, base);
	// 82519C68: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82519C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519C70: 419A0008  beq cr6, 0x82519c78
	if ctx.cr[6].eq {
	pc = 0x82519C78; continue 'dispatch;
	}
	// 82519C74: 4BDA6C1D  bl 0x822c0890
	ctx.lr = 0x82519C78;
	sub_822C0890(ctx, base);
	// 82519C78: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82519C7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519C80: 419A0008  beq cr6, 0x82519c88
	if ctx.cr[6].eq {
	pc = 0x82519C88; continue 'dispatch;
	}
	// 82519C84: 4BDA6C0D  bl 0x822c0890
	ctx.lr = 0x82519C88;
	sub_822C0890(ctx, base);
	// 82519C88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519C8C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82519C90: 4BFF56A1  bl 0x8250f330
	ctx.lr = 0x82519C94;
	sub_8250F330(ctx, base);
	// 82519C94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82519C98: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519C9C: 4BFD19CD  bl 0x824eb668
	ctx.lr = 0x82519CA0;
	sub_824EB668(ctx, base);
	// 82519CA0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82519CA4: 488D7FED  bl 0x82df1c90
	ctx.lr = 0x82519CA8;
	sub_82DF1C90(ctx, base);
	// 82519CA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519CAC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519CB4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82519CB8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82519CBC: 419A0024  beq cr6, 0x82519ce0
	if ctx.cr[6].eq {
	pc = 0x82519CE0; continue 'dispatch;
	}
	// 82519CC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82519CC4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82519CC8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519CCC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82519CD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82519CD4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82519CD8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82519CDC: 4082FFE8  bne 0x82519cc4
	if !ctx.cr[0].eq {
	pc = 0x82519CC4; continue 'dispatch;
	}
	// 82519CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519CE4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82519CE8: 4BFF57E1  bl 0x8250f4c8
	ctx.lr = 0x82519CEC;
	sub_8250F4C8(ctx, base);
	// 82519CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519CF4: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 82519CF8: 409A0008  bne cr6, 0x82519d00
	if !ctx.cr[6].eq {
	pc = 0x82519D00; continue 'dispatch;
	}
	// 82519CFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82519D00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519D04: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82519D08: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82519D0C: 4BFF580D  bl 0x8250f518
	ctx.lr = 0x82519D10;
	sub_8250F518(ctx, base);
	// 82519D10: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82519D14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519D1C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82519D20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82519D24: 4BFF386D  bl 0x8250d590
	ctx.lr = 0x82519D28;
	sub_8250D590(ctx, base);
	// 82519D28: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82519D2C: 488D7F65  bl 0x82df1c90
	ctx.lr = 0x82519D30;
	sub_82DF1C90(ctx, base);
	// 82519D30: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82519D34: 488D7F5D  bl 0x82df1c90
	ctx.lr = 0x82519D38;
	sub_82DF1C90(ctx, base);
	// 82519D38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519D3C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82519D40: 4BFF5789  bl 0x8250f4c8
	ctx.lr = 0x82519D44;
	sub_8250F4C8(ctx, base);
	// 82519D44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519D48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519D4C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519D50: 409A0008  bne cr6, 0x82519d58
	if !ctx.cr[6].eq {
	pc = 0x82519D58; continue 'dispatch;
	}
	// 82519D54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82519D58: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82519D5C: 4BFEE765  bl 0x825084c0
	ctx.lr = 0x82519D60;
	sub_825084C0(ctx, base);
	// 82519D60: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82519D64: 488D7F2D  bl 0x82df1c90
	ctx.lr = 0x82519D68;
	sub_82DF1C90(ctx, base);
	// 82519D68: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82519D6C: 48C8E450  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519D70 size=120
    let mut pc: u32 = 0x82519D70;
    'dispatch: loop {
        match pc {
            0x82519D70 => {
    //   block [0x82519D70..0x82519DE8)
	// 82519D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519D74: 48C8E3ED  bl 0x831a8160
	ctx.lr = 0x82519D78;
	sub_831A8130(ctx, base);
	// 82519D78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519D7C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82519D80: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82519D84: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82519D88: 7F1B3040  cmplw cr6, r27, r6
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82519D8C: 419A004C  beq cr6, 0x82519dd8
	if ctx.cr[6].eq {
	pc = 0x82519DD8; continue 'dispatch;
	}
	// 82519D90: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519D94: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82519D98: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82519D9C: 7F06E040  cmplw cr6, r6, r28
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82519DA0: 419A0020  beq cr6, 0x82519dc0
	if ctx.cr[6].eq {
	pc = 0x82519DC0; continue 'dispatch;
	}
	// 82519DA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519DAC: 4BFFFA8D  bl 0x82519838
	ctx.lr = 0x82519DB0;
	sub_82519838(ctx, base);
	// 82519DB0: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82519DB4: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 82519DB8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82519DBC: 409AFFE8  bne cr6, 0x82519da4
	if !ctx.cr[6].eq {
	pc = 0x82519DA4; continue 'dispatch;
	}
	// 82519DC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82519DC4: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82519DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519DCC: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519DD0: 4BFFFC31  bl 0x82519a00
	ctx.lr = 0x82519DD4;
	sub_82519A00(ctx, base);
	// 82519DD4: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82519DD8: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82519DDC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82519DE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82519DE4: 48C8E3CC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519DE8 size=100
    let mut pc: u32 = 0x82519DE8;
    'dispatch: loop {
        match pc {
            0x82519DE8 => {
    //   block [0x82519DE8..0x82519E4C)
	// 82519DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519DF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519DFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519E00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519E04: 419A0024  beq cr6, 0x82519e28
	if ctx.cr[6].eq {
	pc = 0x82519E28; continue 'dispatch;
	}
	// 82519E08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82519E0C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82519E10: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519E14: 4BFFFBED  bl 0x82519a00
	ctx.lr = 0x82519E18;
	sub_82519A00(ctx, base);
	// 82519E18: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82519E1C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519E20: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82519E24: 488D8365  bl 0x82df2188
	ctx.lr = 0x82519E28;
	sub_82DF2188(ctx, base);
	// 82519E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82519E2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82519E30: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82519E34: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82519E38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519E50 size=88
    let mut pc: u32 = 0x82519E50;
    'dispatch: loop {
        match pc {
            0x82519E50 => {
    //   block [0x82519E50..0x82519EA8)
	// 82519E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519E54: 48C8E311  bl 0x831a8164
	ctx.lr = 0x82519E58;
	sub_831A8130(ctx, base);
	// 82519E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519E5C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82519E60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82519E64: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82519E68: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82519E6C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82519E70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82519E74: 419A0024  beq cr6, 0x82519e98
	if ctx.cr[6].eq {
	pc = 0x82519E98; continue 'dispatch;
	}
	// 82519E78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82519E7C: 419A0010  beq cr6, 0x82519e8c
	if ctx.cr[6].eq {
	pc = 0x82519E8C; continue 'dispatch;
	}
	// 82519E80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82519E84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519E88: 4BFFFB11  bl 0x82519998
	ctx.lr = 0x82519E8C;
	sub_82519998(ctx, base);
	// 82519E8C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82519E90: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 82519E94: 4082FFE4  bne 0x82519e78
	if !ctx.cr[0].eq {
	pc = 0x82519E78; continue 'dispatch;
	}
	// 82519E98: 1D7F0028  mulli r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 * 40;
	// 82519E9C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82519EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519EA4: 48C8E310  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519EA8 size=132
    let mut pc: u32 = 0x82519EA8;
    'dispatch: loop {
        match pc {
            0x82519EA8 => {
    //   block [0x82519EA8..0x82519F2C)
	// 82519EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519EB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82519EB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519EBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519EC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82519EC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82519EC8: 396B22A8  addi r11, r11, 0x22a8
	ctx.r[11].s64 = ctx.r[11].s64 + 8872;
	// 82519ECC: 394A2294  addi r10, r10, 0x2294
	ctx.r[10].s64 = ctx.r[10].s64 + 8852;
	// 82519ED0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519ED4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82519ED8: 807F00FC  lwz r3, 0xfc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82519EDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519EE0: 419A0008  beq cr6, 0x82519ee8
	if ctx.cr[6].eq {
	pc = 0x82519EE8; continue 'dispatch;
	}
	// 82519EE4: 4BDA69AD  bl 0x822c0890
	ctx.lr = 0x82519EE8;
	sub_822C0890(ctx, base);
	// 82519EE8: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 82519EEC: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82519EF0: 488D9539  bl 0x82df3428
	ctx.lr = 0x82519EF4;
	sub_82DF3428(ctx, base);
	// 82519EF4: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82519EF8: 488D9531  bl 0x82df3428
	ctx.lr = 0x82519EFC;
	sub_82DF3428(ctx, base);
	// 82519EFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82519F00: 4893F6D9  bl 0x82e595d8
	ctx.lr = 0x82519F04;
	sub_82E595D8(ctx, base);
	// 82519F04: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82519F08: 4BFFFEE1  bl 0x82519de8
	ctx.lr = 0x82519F0C;
	sub_82519DE8(ctx, base);
	// 82519F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519F10: 4BFF7289  bl 0x82511198
	ctx.lr = 0x82519F14;
	sub_82511198(ctx, base);
	// 82519F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519F20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82519F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519F30 size=8
    let mut pc: u32 = 0x82519F30;
    'dispatch: loop {
        match pc {
            0x82519F30 => {
    //   block [0x82519F30..0x82519F38)
	// 82519F30: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82519F34: 480003DC  b 0x8251a310
	sub_8251A310(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519F38 size=980
    let mut pc: u32 = 0x82519F38;
    'dispatch: loop {
        match pc {
            0x82519F38 => {
    //   block [0x82519F38..0x8251A30C)
	// 82519F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519F3C: 48C8E219  bl 0x831a8154
	ctx.lr = 0x82519F40;
	sub_831A8130(ctx, base);
	// 82519F40: 9421FDD0  stwu r1, -0x230(r1)
	ea = ctx.r[1].u32.wrapping_add(-560 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519F44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82519F48: 817D0100  lwz r11, 0x100(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(256 as u32) ) } as u64;
	// 82519F4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82519F50: 419A0118  beq cr6, 0x8251a068
	if ctx.cr[6].eq {
	pc = 0x8251A068; continue 'dispatch;
	}
	// 82519F54: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82519F58: 419A0100  beq cr6, 0x8251a058
	if ctx.cr[6].eq {
	pc = 0x8251A058; continue 'dispatch;
	}
	// 82519F5C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82519F60: 419A00DC  beq cr6, 0x8251a03c
	if ctx.cr[6].eq {
	pc = 0x8251A03C; continue 'dispatch;
	}
	// 82519F64: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82519F68: 419A00B4  beq cr6, 0x8251a01c
	if ctx.cr[6].eq {
	pc = 0x8251A01C; continue 'dispatch;
	}
	// 82519F6C: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82519F70: 419A005C  beq cr6, 0x82519fcc
	if ctx.cr[6].eq {
	pc = 0x82519FCC; continue 'dispatch;
	}
	// 82519F74: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82519F78: 409A038C  bne cr6, 0x8251a304
	if !ctx.cr[6].eq {
	pc = 0x8251A304; continue 'dispatch;
	}
	// 82519F7C: 391D00EC  addi r8, r29, 0xec
	ctx.r[8].s64 = ctx.r[29].s64 + 236;
	// 82519F80: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82519F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82519F88: 38BD00F8  addi r5, r29, 0xf8
	ctx.r[5].s64 = ctx.r[29].s64 + 248;
	// 82519F8C: 389D00E8  addi r4, r29, 0xe8
	ctx.r[4].s64 = ctx.r[29].s64 + 232;
	// 82519F90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82519F94: 4800B85D  bl 0x825257f0
	ctx.lr = 0x82519F98;
	sub_825257F0(ctx, base);
	// 82519F98: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82519F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519FA0: 419A0010  beq cr6, 0x82519fb0
	if ctx.cr[6].eq {
	pc = 0x82519FB0; continue 'dispatch;
	}
	// 82519FA4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82519FA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82519FAC: 4BFFFC15  bl 0x82519bc0
	ctx.lr = 0x82519FB0;
	sub_82519BC0(ctx, base);
	// 82519FB0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82519FB4: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82519FB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82519FBC: 917D0100  stw r11, 0x100(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 82519FC0: 419A0344  beq cr6, 0x8251a304
	if ctx.cr[6].eq {
	pc = 0x8251A304; continue 'dispatch;
	}
	// 82519FC4: 4BDA68CD  bl 0x822c0890
	ctx.lr = 0x82519FC8;
	sub_822C0890(ctx, base);
	// 82519FC8: 4800033C  b 0x8251a304
	pc = 0x8251A304; continue 'dispatch;
	// 82519FCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82519FD0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82519FD4: 4BFF54F5  bl 0x8250f4c8
	ctx.lr = 0x82519FD8;
	sub_8250F4C8(ctx, base);
	// 82519FD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519FDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519FE0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82519FE4: 409A0008  bne cr6, 0x82519fec
	if !ctx.cr[6].eq {
	pc = 0x82519FEC; continue 'dispatch;
	}
	// 82519FE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82519FEC: 4BFEE925  bl 0x82508910
	ctx.lr = 0x82519FF0;
	sub_82508910(ctx, base);
	// 82519FF0: 488E87A9  bl 0x82e02798
	ctx.lr = 0x82519FF4;
	sub_82E02798(ctx, base);
	// 82519FF4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82519FF8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82519FFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8251A000: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8251A004: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8251A008: 488D7C89  bl 0x82df1c90
	ctx.lr = 0x8251A00C;
	sub_82DF1C90(ctx, base);
	// 8251A00C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251A010: 418202F4  beq 0x8251a304
	if ctx.cr[0].eq {
	pc = 0x8251A304; continue 'dispatch;
	}
	// 8251A014: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8251A018: 48000078  b 0x8251a090
	pc = 0x8251A090; continue 'dispatch;
	// 8251A01C: 817D0104  lwz r11, 0x104(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(260 as u32) ) } as u64;
	// 8251A020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A024: 40990010  ble cr6, 0x8251a034
	if !ctx.cr[6].gt {
	pc = 0x8251A034; continue 'dispatch;
	}
	// 8251A028: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8251A02C: 917D0104  stw r11, 0x104(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 8251A030: 480002D4  b 0x8251a304
	pc = 0x8251A304; continue 'dispatch;
	// 8251A034: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251A038: 48000058  b 0x8251a090
	pc = 0x8251A090; continue 'dispatch;
	// 8251A03C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251A040: 4BFFFA21  bl 0x82519a60
	ctx.lr = 0x8251A044;
	sub_82519A60(ctx, base);
	// 8251A044: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8251A048: 3940001E  li r10, 0x1e
	ctx.r[10].s64 = 30;
	// 8251A04C: 917D0100  stw r11, 0x100(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 8251A050: 915D0104  stw r10, 0x104(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 8251A054: 480002B0  b 0x8251a304
	pc = 0x8251A304; continue 'dispatch;
	// 8251A058: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251A05C: 4BFFFA05  bl 0x82519a60
	ctx.lr = 0x8251A060;
	sub_82519A60(ctx, base);
	// 8251A060: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8251A064: 4800002C  b 0x8251a090
	pc = 0x8251A090; continue 'dispatch;
	// 8251A068: 817D00C4  lwz r11, 0xc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251A06C: 3BDD00C0  addi r30, r29, 0xc0
	ctx.r[30].s64 = ctx.r[29].s64 + 192;
	// 8251A070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A074: 419A0018  beq cr6, 0x8251a08c
	if ctx.cr[6].eq {
	pc = 0x8251A08C; continue 'dispatch;
	}
	// 8251A078: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A07C: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 8251A080: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8251A084: 7D6B53D7  divw. r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A088: 40820010  bne 0x8251a098
	if !ctx.cr[0].eq {
	pc = 0x8251A098; continue 'dispatch;
	}
	// 8251A08C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8251A090: 917D0100  stw r11, 0x100(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 8251A094: 48000270  b 0x8251a304
	pc = 0x8251A304; continue 'dispatch;
	// 8251A098: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A09C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251A0A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A0A4: 409A000C  bne cr6, 0x8251a0b0
	if !ctx.cr[6].eq {
	pc = 0x8251A0B0; continue 'dispatch;
	}
	// 8251A0A8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8251A0AC: 48000010  b 0x8251a0bc
	pc = 0x8251A0BC; continue 'dispatch;
	// 8251A0B0: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A0B4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8251A0B8: 7D4B53D6  divw r10, r11, r10
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8251A0BC: 817D00C4  lwz r11, 0xc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251A0C0: 1D4A0028  mulli r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 * 40;
	// 8251A0C4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8251A0C8: 387D00D0  addi r3, r29, 0xd0
	ctx.r[3].s64 = ctx.r[29].s64 + 208;
	// 8251A0CC: 388BFFD8  addi r4, r11, -0x28
	ctx.r[4].s64 = ctx.r[11].s64 + -40;
	// 8251A0D0: 4BFFF769  bl 0x82519838
	ctx.lr = 0x8251A0D4;
	sub_82519838(ctx, base);
	// 8251A0D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251A0D8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251A0DC: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A0E0: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A0E4: 4BFFFC8D  bl 0x82519d70
	ctx.lr = 0x8251A0E8;
	sub_82519D70(ctx, base);
	// 8251A0E8: 48006861  bl 0x82520948
	ctx.lr = 0x8251A0EC;
	sub_82520948(ctx, base);
	// 8251A0EC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8251A0F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251A0F4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251A0F8: 4BFF53D1  bl 0x8250f4c8
	ctx.lr = 0x8251A0FC;
	sub_8250F4C8(ctx, base);
	// 8251A0FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A104: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251A108: 409A0008  bne cr6, 0x8251a110
	if !ctx.cr[6].eq {
	pc = 0x8251A110; continue 'dispatch;
	}
	// 8251A10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A110: 4BFEE801  bl 0x82508910
	ctx.lr = 0x8251A114;
	sub_82508910(ctx, base);
	// 8251A114: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8251A118: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251A11C: 488D7B75  bl 0x82df1c90
	ctx.lr = 0x8251A120;
	sub_82DF1C90(ctx, base);
	// 8251A120: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251A124: 38BD00E8  addi r5, r29, 0xe8
	ctx.r[5].s64 = ctx.r[29].s64 + 232;
	// 8251A128: 388BA0D0  addi r4, r11, -0x5f30
	ctx.r[4].s64 = ctx.r[11].s64 + -24368;
	// 8251A12C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A130: 488D9B71  bl 0x82df3ca0
	ctx.lr = 0x8251A134;
	sub_82DF3CA0(ctx, base);
	// 8251A134: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8251A138: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 8251A13C: 93E100A8  stw r31, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[31].u32 ) };
	// 8251A140: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 8251A144: 93E100AC  stw r31, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 8251A148: 930100A0  stw r24, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[24].u32 ) };
	// 8251A14C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251A150: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8251A154: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A158: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 8251A15C: 38AA0878  addi r5, r10, 0x878
	ctx.r[5].s64 = ctx.r[10].s64 + 2168;
	// 8251A160: 93E100D0  stw r31, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[31].u32 ) };
	// 8251A164: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251A168: 93E100F0  stw r31, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[31].u32 ) };
	// 8251A16C: 3BC100A0  addi r30, r1, 0xa0
	ctx.r[30].s64 = ctx.r[1].s64 + 160;
	// 8251A170: 93E10110  stw r31, 0x110(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[31].u32 ) };
	// 8251A174: 93810130  stw r28, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[28].u32 ) };
	// 8251A178: 488D9B81  bl 0x82df3cf8
	ctx.lr = 0x8251A17C;
	sub_82DF3CF8(ctx, base);
	// 8251A17C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251A180: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8251A184: 3B2B0880  addi r25, r11, 0x880
	ctx.r[25].s64 = ctx.r[11].s64 + 2176;
	// 8251A188: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A18C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8251A190: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251A194: 488D9B65  bl 0x82df3cf8
	ctx.lr = 0x8251A198;
	sub_82DF3CF8(ctx, base);
	// 8251A198: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8251A19C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8251A1A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251A1A4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8251A1A8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8251A1AC: 488EC2F5  bl 0x82e064a0
	ctx.lr = 0x8251A1B0;
	sub_82E064A0(ctx, base);
	// 8251A1B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251A1B4: 488D9275  bl 0x82df3428
	ctx.lr = 0x8251A1B8;
	sub_82DF3428(ctx, base);
	// 8251A1B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251A1BC: 488D926D  bl 0x82df3428
	ctx.lr = 0x8251A1C0;
	sub_82DF3428(ctx, base);
	// 8251A1C0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251A1C4: 4BFB7505  bl 0x824d16c8
	ctx.lr = 0x8251A1C8;
	sub_824D16C8(ctx, base);
	// 8251A1C8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251A1CC: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8251A1D0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8251A1D4: 488EF4A5  bl 0x82e09678
	ctx.lr = 0x8251A1D8;
	sub_82E09678(ctx, base);
	// 8251A1D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251A1DC: 41820028  beq 0x8251a204
	if ctx.cr[0].eq {
	pc = 0x8251A204; continue 'dispatch;
	}
	// 8251A1E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251A1E4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251A1E8: 488EF4B9  bl 0x82e096a0
	ctx.lr = 0x8251A1EC;
	sub_82E096A0(ctx, base);
	// 8251A1EC: 7F63DA14  add r27, r3, r27
	ctx.r[27].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	// 8251A1F0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251A1F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8251A1F8: 488EF481  bl 0x82e09678
	ctx.lr = 0x8251A1FC;
	sub_82E09678(ctx, base);
	// 8251A1FC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8251A200: 4198FFE0  blt cr6, 0x8251a1e0
	if ctx.cr[6].lt {
	pc = 0x8251A1E0; continue 'dispatch;
	}
	// 8251A204: 3D7B0131  addis r11, r27, 0x131
	ctx.r[11].s64 = ctx.r[27].s64 + 19988480;
	// 8251A208: 396B2D00  addi r11, r11, 0x2d00
	ctx.r[11].s64 = ctx.r[11].s64 + 11520;
	// 8251A20C: 7F175840  cmplw cr6, r23, r11
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251A210: 419800DC  blt cr6, 0x8251a2ec
	if ctx.cr[6].lt {
	pc = 0x8251A2EC; continue 'dispatch;
	}
	// 8251A214: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251A218: 488E8169  bl 0x82e02380
	ctx.lr = 0x8251A21C;
	sub_82E02380(ctx, base);
	// 8251A21C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251A220: 3BDD00F8  addi r30, r29, 0xf8
	ctx.r[30].s64 = ctx.r[29].s64 + 248;
	// 8251A224: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251A228: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8251A22C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A230: 917D00F8  stw r11, 0xf8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 8251A234: 4BDAA22D  bl 0x822c4460
	ctx.lr = 0x8251A238;
	sub_822C4460(ctx, base);
	// 8251A238: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8251A23C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251A240: 419A0008  beq cr6, 0x8251a248
	if ctx.cr[6].eq {
	pc = 0x8251A248; continue 'dispatch;
	}
	// 8251A244: 4BDA664D  bl 0x822c0890
	ctx.lr = 0x8251A248;
	sub_822C0890(ctx, base);
	// 8251A248: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A24C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A250: 93010140  stw r24, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[24].u32 ) };
	// 8251A254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A258: 93E10144  stw r31, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[31].u32 ) };
	// 8251A25C: 93E10148  stw r31, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[31].u32 ) };
	// 8251A260: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251A264: 93E1014C  stw r31, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[31].u32 ) };
	// 8251A268: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251A26C: 93E10150  stw r31, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[31].u32 ) };
	// 8251A270: 93E10170  stw r31, 0x170(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[31].u32 ) };
	// 8251A274: 93E10190  stw r31, 0x190(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), ctx.r[31].u32 ) };
	// 8251A278: 93E101B0  stw r31, 0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[31].u32 ) };
	// 8251A27C: 938101D0  stw r28, 0x1d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(464 as u32), ctx.r[28].u32 ) };
	// 8251A280: 419A0024  beq cr6, 0x8251a2a4
	if ctx.cr[6].eq {
	pc = 0x8251A2A4; continue 'dispatch;
	}
	// 8251A284: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251A288: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251A28C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251A290: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251A294: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251A298: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251A29C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251A2A0: 4082FFE8  bne 0x8251a288
	if !ctx.cr[0].eq {
	pc = 0x8251A288; continue 'dispatch;
	}
	// 8251A2A4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8251A2A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A2AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251A2B0: 3BE10140  addi r31, r1, 0x140
	ctx.r[31].s64 = ctx.r[1].s64 + 320;
	// 8251A2B4: 488D9A45  bl 0x82df3cf8
	ctx.lr = 0x8251A2B8;
	sub_82DF3CF8(ctx, base);
	// 8251A2B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8251A2BC: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8251A2C0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8251A2C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8251A2C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8251A2CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251A2D0: 488EA4B1  bl 0x82e04780
	ctx.lr = 0x8251A2D4;
	sub_82E04780(ctx, base);
	// 8251A2D4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251A2D8: 488D9151  bl 0x82df3428
	ctx.lr = 0x8251A2DC;
	sub_82DF3428(ctx, base);
	// 8251A2DC: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8251A2E0: 4BFB73E9  bl 0x824d16c8
	ctx.lr = 0x8251A2E4;
	sub_824D16C8(ctx, base);
	// 8251A2E4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8251A2E8: 917D0100  stw r11, 0x100(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 8251A2EC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251A2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251A2F4: 419A0008  beq cr6, 0x8251a2fc
	if ctx.cr[6].eq {
	pc = 0x8251A2FC; continue 'dispatch;
	}
	// 8251A2F8: 4BDA6599  bl 0x822c0890
	ctx.lr = 0x8251A2FC;
	sub_822C0890(ctx, base);
	// 8251A2FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A300: 488D9129  bl 0x82df3428
	ctx.lr = 0x8251A304;
	sub_82DF3428(ctx, base);
	// 8251A304: 38210230  addi r1, r1, 0x230
	ctx.r[1].s64 = ctx.r[1].s64 + 560;
	// 8251A308: 48C8DE9C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A310 size=76
    let mut pc: u32 = 0x8251A310;
    'dispatch: loop {
        match pc {
            0x8251A310 => {
    //   block [0x8251A310..0x8251A35C)
	// 8251A310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A328: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251A32C: 4BFFFB7D  bl 0x82519ea8
	ctx.lr = 0x8251A330;
	sub_82519EA8(ctx, base);
	// 8251A330: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A334: 4182000C  beq 0x8251a340
	if ctx.cr[0].eq {
	pc = 0x8251A340; continue 'dispatch;
	}
	// 8251A338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A33C: 488D809D  bl 0x82df23d8
	ctx.lr = 0x8251A340;
	sub_82DF23D8(ctx, base);
	// 8251A340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A350: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A360 size=872
    let mut pc: u32 = 0x8251A360;
    'dispatch: loop {
        match pc {
            0x8251A360 => {
    //   block [0x8251A360..0x8251A6C8)
	// 8251A360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A364: 48C8DDED  bl 0x831a8150
	ctx.lr = 0x8251A368;
	sub_831A8130(ctx, base);
	// 8251A368: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A36C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A370: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8251A374: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8251A378: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251A37C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8251A380: 4BFFF619  bl 0x82519998
	ctx.lr = 0x8251A384;
	sub_82519998(ctx, base);
	// 8251A384: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A388: 3AC00028  li r22, 0x28
	ctx.r[22].s64 = 40;
	// 8251A38C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A390: 409A000C  bne cr6, 0x8251a39c
	if !ctx.cr[6].eq {
	pc = 0x8251A39C; continue 'dispatch;
	}
	// 8251A394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8251A398: 48000010  b 0x8251a3a8
	pc = 0x8251A3A8; continue 'dispatch;
	// 8251A39C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251A3A0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8251A3A4: 7D0BB3D6  divw r8, r11, r22
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A3A8: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8251A3AC: 419A02FC  beq cr6, 0x8251a6a8
	if ctx.cr[6].eq {
	pc = 0x8251A6A8; continue 'dispatch;
	}
	// 8251A3B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A3B4: 409A000C  bne cr6, 0x8251a3c0
	if !ctx.cr[6].eq {
	pc = 0x8251A3C0; continue 'dispatch;
	}
	// 8251A3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A3BC: 48000010  b 0x8251a3cc
	pc = 0x8251A3CC; continue 'dispatch;
	// 8251A3C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A3C4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8251A3C8: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A3CC: 3D200666  lis r9, 0x666
	ctx.r[9].s64 = 107347968;
	// 8251A3D0: 61296666  ori r9, r9, 0x6666
	ctx.r[9].u64 = ctx.r[9].u64 | 26214;
	// 8251A3D4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8251A3D8: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 8251A3DC: 4098000C  bge cr6, 0x8251a3e8
	if !ctx.cr[6].lt {
	pc = 0x8251A3E8; continue 'dispatch;
	}
	// 8251A3E0: 4869D941  bl 0x82bb7d20
	ctx.lr = 0x8251A3E4;
	sub_82BB7D20(ctx, base);
	// 8251A3E4: 480002C4  b 0x8251a6a8
	pc = 0x8251A6A8; continue 'dispatch;
	// 8251A3E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A3EC: 409A000C  bne cr6, 0x8251a3f8
	if !ctx.cr[6].eq {
	pc = 0x8251A3F8; continue 'dispatch;
	}
	// 8251A3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A3F4: 48000010  b 0x8251a404
	pc = 0x8251A404; continue 'dispatch;
	// 8251A3F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A3FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8251A400: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A404: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8251A408: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251A40C: 4098016C  bge cr6, 0x8251a578
	if !ctx.cr[6].lt {
	pc = 0x8251A578; continue 'dispatch;
	}
	// 8251A410: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251A414: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8251A418: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8251A41C: 4098000C  bge cr6, 0x8251a428
	if !ctx.cr[6].lt {
	pc = 0x8251A428; continue 'dispatch;
	}
	// 8251A420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A424: 48000008  b 0x8251a42c
	pc = 0x8251A42C; continue 'dispatch;
	// 8251A428: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8251A42C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A430: 409A000C  bne cr6, 0x8251a43c
	if !ctx.cr[6].eq {
	pc = 0x8251A43C; continue 'dispatch;
	}
	// 8251A434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251A438: 48000010  b 0x8251a448
	pc = 0x8251A448; continue 'dispatch;
	// 8251A43C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A440: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8251A444: 7D29B3D6  divw r9, r9, r22
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[22].s32;
	// 8251A448: 7D29BA14  add r9, r9, r23
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[23].u64;
	// 8251A44C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251A450: 40980024  bge cr6, 0x8251a474
	if !ctx.cr[6].lt {
	pc = 0x8251A474; continue 'dispatch;
	}
	// 8251A454: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A458: 409A000C  bne cr6, 0x8251a464
	if !ctx.cr[6].eq {
	pc = 0x8251A464; continue 'dispatch;
	}
	// 8251A45C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A460: 48000010  b 0x8251a470
	pc = 0x8251A470; continue 'dispatch;
	// 8251A464: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A468: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8251A46C: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A470: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8251A474: 1F0B0028  mulli r24, r11, 0x28
	ctx.r[24].s64 = ctx.r[11].s64 * 40;
	// 8251A478: 3F208335  lis r25, -0x7ccb
	ctx.r[25].s64 = -2093678592;
	// 8251A47C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8251A480: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 8251A484: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 8251A488: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8251A48C: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251A490: 488D7C39  bl 0x82df20c8
	ctx.lr = 0x8251A494;
	sub_82DF20C8(ctx, base);
	// 8251A494: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8251A498: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A49C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8251A4A0: 48000020  b 0x8251a4c0
	pc = 0x8251A4C0; continue 'dispatch;
	// 8251A4A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251A4A8: 419A0010  beq cr6, 0x8251a4b8
	if ctx.cr[6].eq {
	pc = 0x8251A4B8; continue 'dispatch;
	}
	// 8251A4AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251A4B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251A4B4: 4BFFF4E5  bl 0x82519998
	ctx.lr = 0x8251A4B8;
	sub_82519998(ctx, base);
	// 8251A4B8: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 8251A4BC: 3BBD0028  addi r29, r29, 0x28
	ctx.r[29].s64 = ctx.r[29].s64 + 40;
	// 8251A4C0: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251A4C4: 409AFFE0  bne cr6, 0x8251a4a4
	if !ctx.cr[6].eq {
	pc = 0x8251A4A4; continue 'dispatch;
	}
	// 8251A4C8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8251A4CC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8251A4D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251A4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A4D8: 4BFFF979  bl 0x82519e50
	ctx.lr = 0x8251A4DC;
	sub_82519E50(ctx, base);
	// 8251A4DC: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A4E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251A4E4: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8251A4E8: 419A002C  beq cr6, 0x8251a514
	if ctx.cr[6].eq {
	pc = 0x8251A514; continue 'dispatch;
	}
	// 8251A4EC: 7F9ED050  subf r28, r30, r26
	ctx.r[28].s64 = ctx.r[26].s64 - ctx.r[30].s64;
	// 8251A4F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251A4F4: 419A0010  beq cr6, 0x8251a504
	if ctx.cr[6].eq {
	pc = 0x8251A504; continue 'dispatch;
	}
	// 8251A4F8: 7C9CF214  add r4, r28, r30
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8251A4FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251A500: 4BFFF499  bl 0x82519998
	ctx.lr = 0x8251A504;
	sub_82519998(ctx, base);
	// 8251A504: 3BDE0028  addi r30, r30, 0x28
	ctx.r[30].s64 = ctx.r[30].s64 + 40;
	// 8251A508: 7D7CF214  add r11, r28, r30
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8251A50C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8251A510: 409AFFE0  bne cr6, 0x8251a4f0
	if !ctx.cr[6].eq {
	pc = 0x8251A4F0; continue 'dispatch;
	}
	// 8251A514: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A518: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251A51C: 409A000C  bne cr6, 0x8251a528
	if !ctx.cr[6].eq {
	pc = 0x8251A528; continue 'dispatch;
	}
	// 8251A520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A524: 48000010  b 0x8251a534
	pc = 0x8251A534; continue 'dispatch;
	// 8251A528: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A52C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8251A530: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A534: 7FCBBA14  add r30, r11, r23
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8251A538: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251A53C: 419A0020  beq cr6, 0x8251a55c
	if ctx.cr[6].eq {
	pc = 0x8251A55C; continue 'dispatch;
	}
	// 8251A540: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251A544: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251A548: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A54C: 4BFFF4B5  bl 0x82519a00
	ctx.lr = 0x8251A550;
	sub_82519A00(ctx, base);
	// 8251A550: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251A554: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A558: 488D7C31  bl 0x82df2188
	ctx.lr = 0x8251A55C;
	sub_82DF2188(ctx, base);
	// 8251A55C: 1D7E0028  mulli r11, r30, 0x28
	ctx.r[11].s64 = ctx.r[30].s64 * 40;
	// 8251A560: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8251A564: 7D58DA14  add r10, r24, r27
	ctx.r[10].u64 = ctx.r[24].u64 + ctx.r[27].u64;
	// 8251A568: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8251A56C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8251A570: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251A574: 48000134  b 0x8251a6a8
	pc = 0x8251A6A8; continue 'dispatch;
	// 8251A578: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A57C: 7D7AF050  subf r11, r26, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 8251A580: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A584: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 8251A588: 40980090  bge cr6, 0x8251a618
	if !ctx.cr[6].lt {
	pc = 0x8251A618; continue 'dispatch;
	}
	// 8251A58C: 1FB70028  mulli r29, r23, 0x28
	ctx.r[29].s64 = ctx.r[23].s64 * 40;
	// 8251A590: 7F9DD214  add r28, r29, r26
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 8251A594: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A598: 419A002C  beq cr6, 0x8251a5c4
	if ctx.cr[6].eq {
	pc = 0x8251A5C4; continue 'dispatch;
	}
	// 8251A59C: 7F7DE050  subf r27, r29, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 8251A5A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8251A5A4: 419A0010  beq cr6, 0x8251a5b4
	if ctx.cr[6].eq {
	pc = 0x8251A5B4; continue 'dispatch;
	}
	// 8251A5A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8251A5AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251A5B0: 4BFFF3E9  bl 0x82519998
	ctx.lr = 0x8251A5B4;
	sub_82519998(ctx, base);
	// 8251A5B4: 3B7B0028  addi r27, r27, 0x28
	ctx.r[27].s64 = ctx.r[27].s64 + 40;
	// 8251A5B8: 3B9C0028  addi r28, r28, 0x28
	ctx.r[28].s64 = ctx.r[28].s64 + 40;
	// 8251A5BC: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A5C0: 409AFFE0  bne cr6, 0x8251a5a0
	if !ctx.cr[6].eq {
	pc = 0x8251A5A0; continue 'dispatch;
	}
	// 8251A5C4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A5C8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8251A5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A5D0: 7D7A2050  subf r11, r26, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[26].s64;
	// 8251A5D4: 7D6BB3D6  divw r11, r11, r22
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[22].s32;
	// 8251A5D8: 7CABB850  subf r5, r11, r23
	ctx.r[5].s64 = ctx.r[23].s64 - ctx.r[11].s64;
	// 8251A5DC: 4BFFF875  bl 0x82519e50
	ctx.lr = 0x8251A5E0;
	sub_82519E50(ctx, base);
	// 8251A5E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A5E4: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 8251A5E8: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8251A5EC: 7FDD5850  subf r30, r29, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8251A5F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251A5F4: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A5F8: 419A00B0  beq cr6, 0x8251a6a8
	if ctx.cr[6].eq {
	pc = 0x8251A6A8; continue 'dispatch;
	}
	// 8251A5FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251A600: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251A604: 4BFFF235  bl 0x82519838
	ctx.lr = 0x8251A608;
	sub_82519838(ctx, base);
	// 8251A608: 3B9C0028  addi r28, r28, 0x28
	ctx.r[28].s64 = ctx.r[28].s64 + 40;
	// 8251A60C: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A610: 409AFFEC  bne cr6, 0x8251a5fc
	if !ctx.cr[6].eq {
	pc = 0x8251A5FC; continue 'dispatch;
	}
	// 8251A614: 48000094  b 0x8251a6a8
	pc = 0x8251A6A8; continue 'dispatch;
	// 8251A618: 1F370028  mulli r25, r23, 0x28
	ctx.r[25].s64 = ctx.r[23].s64 * 40;
	// 8251A61C: 7FB9F050  subf r29, r25, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 8251A620: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8251A624: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8251A628: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A62C: 419A0028  beq cr6, 0x8251a654
	if ctx.cr[6].eq {
	pc = 0x8251A654; continue 'dispatch;
	}
	// 8251A630: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8251A634: 419A0010  beq cr6, 0x8251a644
	if ctx.cr[6].eq {
	pc = 0x8251A644; continue 'dispatch;
	}
	// 8251A638: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8251A63C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251A640: 4BFFF359  bl 0x82519998
	ctx.lr = 0x8251A644;
	sub_82519998(ctx, base);
	// 8251A644: 3B7B0028  addi r27, r27, 0x28
	ctx.r[27].s64 = ctx.r[27].s64 + 40;
	// 8251A648: 3B9C0028  addi r28, r28, 0x28
	ctx.r[28].s64 = ctx.r[28].s64 + 40;
	// 8251A64C: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A650: 409AFFE0  bne cr6, 0x8251a630
	if !ctx.cr[6].eq {
	pc = 0x8251A630; continue 'dispatch;
	}
	// 8251A654: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8251A658: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8251A65C: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8251A660: 419A0020  beq cr6, 0x8251a680
	if ctx.cr[6].eq {
	pc = 0x8251A680; continue 'dispatch;
	}
	// 8251A664: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 8251A668: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 8251A66C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A670: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8251A674: 4BFFF1C5  bl 0x82519838
	ctx.lr = 0x8251A678;
	sub_82519838(ctx, base);
	// 8251A678: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251A67C: 409AFFEC  bne cr6, 0x8251a668
	if !ctx.cr[6].eq {
	pc = 0x8251A668; continue 'dispatch;
	}
	// 8251A680: 7FD9D214  add r30, r25, r26
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 8251A684: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 8251A688: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A68C: 419A001C  beq cr6, 0x8251a6a8
	if ctx.cr[6].eq {
	pc = 0x8251A6A8; continue 'dispatch;
	}
	// 8251A690: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251A694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A698: 4BFFF1A1  bl 0x82519838
	ctx.lr = 0x8251A69C;
	sub_82519838(ctx, base);
	// 8251A69C: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8251A6A0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251A6A4: 409AFFEC  bne cr6, 0x8251a690
	if !ctx.cr[6].eq {
	pc = 0x8251A690; continue 'dispatch;
	}
	// 8251A6A8: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 8251A6AC: 488D8D7D  bl 0x82df3428
	ctx.lr = 0x8251A6B0;
	sub_82DF3428(ctx, base);
	// 8251A6B0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251A6B4: 488D8D75  bl 0x82df3428
	ctx.lr = 0x8251A6B8;
	sub_82DF3428(ctx, base);
	// 8251A6B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251A6BC: 4893EF1D  bl 0x82e595d8
	ctx.lr = 0x8251A6C0;
	sub_82E595D8(ctx, base);
	// 8251A6C0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8251A6C4: 48C8DADC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A6C8 size=112
    let mut pc: u32 = 0x8251A6C8;
    'dispatch: loop {
        match pc {
            0x8251A6C8 => {
    //   block [0x8251A6C8..0x8251A738)
	// 8251A6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A6CC: 48C8DAA1  bl 0x831a816c
	ctx.lr = 0x8251A6D0;
	sub_831A8130(ctx, base);
	// 8251A6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A6D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251A6D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251A6DC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251A6E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A6E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A6E8: 419A0018  beq cr6, 0x8251a700
	if ctx.cr[6].eq {
	pc = 0x8251A700; continue 'dispatch;
	}
	// 8251A6EC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A6F0: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 8251A6F4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8251A6F8: 7D2953D7  divw. r9, r9, r10
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8251A6FC: 4082000C  bne 0x8251a708
	if !ctx.cr[0].eq {
	pc = 0x8251A708; continue 'dispatch;
	}
	// 8251A700: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251A704: 4800000C  b 0x8251a710
	pc = 0x8251A710; continue 'dispatch;
	// 8251A708: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8251A70C: 7FCB53D6  divw r30, r11, r10
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8251A710: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8251A714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A718: 4BFFFC49  bl 0x8251a360
	ctx.lr = 0x8251A71C;
	sub_8251A360(ctx, base);
	// 8251A71C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A720: 1D7E0028  mulli r11, r30, 0x28
	ctx.r[11].s64 = ctx.r[30].s64 * 40;
	// 8251A724: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251A728: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251A72C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A734: 48C8DA88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A738 size=164
    let mut pc: u32 = 0x8251A738;
    'dispatch: loop {
        match pc {
            0x8251A738 => {
    //   block [0x8251A738..0x8251A7DC)
	// 8251A738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A74C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A750: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 8251A754: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A75C: 409A000C  bne cr6, 0x8251a768
	if !ctx.cr[6].eq {
	pc = 0x8251A768; continue 'dispatch;
	}
	// 8251A760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A764: 48000010  b 0x8251a774
	pc = 0x8251A774; continue 'dispatch;
	// 8251A768: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A76C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8251A770: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 8251A774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A778: 419A0038  beq cr6, 0x8251a7b0
	if ctx.cr[6].eq {
	pc = 0x8251A7B0; continue 'dispatch;
	}
	// 8251A77C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251A780: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 8251A784: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8251A788: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251A78C: 40980024  bge cr6, 0x8251a7b0
	if !ctx.cr[6].lt {
	pc = 0x8251A7B0; continue 'dispatch;
	}
	// 8251A790: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A794: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251A798: 419A000C  beq cr6, 0x8251a7a4
	if ctx.cr[6].eq {
	pc = 0x8251A7A4; continue 'dispatch;
	}
	// 8251A79C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251A7A0: 4BFFF1F9  bl 0x82519998
	ctx.lr = 0x8251A7A4;
	sub_82519998(ctx, base);
	// 8251A7A4: 397E0028  addi r11, r30, 0x28
	ctx.r[11].s64 = ctx.r[30].s64 + 40;
	// 8251A7A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251A7AC: 48000018  b 0x8251a7c4
	pc = 0x8251A7C4; continue 'dispatch;
	// 8251A7B0: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8251A7B4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A7B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A7BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A7C0: 4BFFFF09  bl 0x8251a6c8
	ctx.lr = 0x8251A7C4;
	sub_8251A6C8(ctx, base);
	// 8251A7C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A7D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A7D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A7E0 size=180
    let mut pc: u32 = 0x8251A7E0;
    'dispatch: loop {
        match pc {
            0x8251A7E0 => {
    //   block [0x8251A7E0..0x8251A894)
	// 8251A7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A7F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A7F8: 4BFF68F9  bl 0x825110f0
	ctx.lr = 0x8251A7FC;
	sub_825110F0(ctx, base);
	// 8251A7FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251A800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251A804: 396B22A8  addi r11, r11, 0x22a8
	ctx.r[11].s64 = ctx.r[11].s64 + 8872;
	// 8251A808: 394A2294  addi r10, r10, 0x2294
	ctx.r[10].s64 = ctx.r[10].s64 + 8852;
	// 8251A80C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251A810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A814: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8251A818: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251A81C: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8251A820: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251A824: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 8251A828: 388B2238  addi r4, r11, 0x2238
	ctx.r[4].s64 = ctx.r[11].s64 + 8760;
	// 8251A82C: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 8251A830: 488D91D9  bl 0x82df3a08
	ctx.lr = 0x8251A834;
	sub_82DF3A08(ctx, base);
	// 8251A834: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251A838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A83C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 8251A840: 488D91C9  bl 0x82df3a08
	ctx.lr = 0x8251A844;
	sub_82DF3A08(ctx, base);
	// 8251A844: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8251A848: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8251A84C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8251A850: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8251A854: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A858: 486403B9  bl 0x82b5ac10
	ctx.lr = 0x8251A85C;
	sub_82B5AC10(ctx, base);
	// 8251A85C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A860: 488D8BC9  bl 0x82df3428
	ctx.lr = 0x8251A864;
	sub_82DF3428(ctx, base);
	// 8251A864: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251A868: 488D8BC1  bl 0x82df3428
	ctx.lr = 0x8251A86C;
	sub_82DF3428(ctx, base);
	// 8251A86C: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 8251A870: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 8251A874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A878: 93DF0100  stw r30, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[30].u32 ) };
	// 8251A87C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A88C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A898 size=148
    let mut pc: u32 = 0x8251A898;
    'dispatch: loop {
        match pc {
            0x8251A898 => {
    //   block [0x8251A898..0x8251A92C)
	// 8251A898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A8A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A8A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A8A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A8AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A8B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251A8B4: 48005A75  bl 0x82520328
	ctx.lr = 0x8251A8B8;
	sub_82520328(ctx, base);
	// 8251A8B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A8BC: 41820058  beq 0x8251a914
	if ctx.cr[0].eq {
	pc = 0x8251A914; continue 'dispatch;
	}
	// 8251A8C0: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 8251A8C4: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 8251A8C8: 488D8A41  bl 0x82df3308
	ctx.lr = 0x8251A8CC;
	sub_82DF3308(ctx, base);
	// 8251A8CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A8D0: 40820044  bne 0x8251a914
	if !ctx.cr[0].eq {
	pc = 0x8251A914; continue 'dispatch;
	}
	// 8251A8D4: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 8251A8D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251A8DC: 419A0024  beq cr6, 0x8251a900
	if ctx.cr[6].eq {
	pc = 0x8251A900; continue 'dispatch;
	}
	// 8251A8E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8251A8E4: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 8251A8E8: 917F0100  stw r11, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 8251A8EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251A8F0: 80DF00C8  lwz r6, 0xc8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 8251A8F4: 80BF00C4  lwz r5, 0xc4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251A8F8: 4BFFF479  bl 0x82519d70
	ctx.lr = 0x8251A8FC;
	sub_82519D70(ctx, base);
	// 8251A8FC: 4800000C  b 0x8251a908
	pc = 0x8251A908; continue 'dispatch;
	// 8251A900: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251A904: 917F0100  stw r11, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 8251A908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251A90C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8251A910: 4BFFFE29  bl 0x8251a738
	ctx.lr = 0x8251A914;
	sub_8251A738(ctx, base);
	// 8251A914: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A920: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A930 size=164
    let mut pc: u32 = 0x8251A930;
    'dispatch: loop {
        match pc {
            0x8251A930 => {
    //   block [0x8251A930..0x8251A9D4)
	// 8251A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A934: 48C8D835  bl 0x831a8168
	ctx.lr = 0x8251A938;
	sub_831A8130(ctx, base);
	// 8251A938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A93C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8251A940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251A944: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251A948: 57BC063F  clrlwi. r28, r29, 0x18
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8251A94C: 41820038  beq 0x8251a984
	if ctx.cr[0].eq {
	pc = 0x8251A984; continue 'dispatch;
	}
	// 8251A950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A954: 48C8F035  bl 0x831a9988
	ctx.lr = 0x8251A958;
	sub_831A9988(ctx, base);
	// 8251A958: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251A95C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251A960: 386BD07C  addi r3, r11, -0x2f84
	ctx.r[3].s64 = ctx.r[11].s64 + -12164;
	// 8251A964: 48C8D795  bl 0x831a80f8
	ctx.lr = 0x8251A968;
	sub_831A80F8(ctx, base);
	// 8251A968: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A96C: 41820018  beq 0x8251a984
	if ctx.cr[0].eq {
	pc = 0x8251A984; continue 'dispatch;
	}
	// 8251A970: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A974: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251A978: 4BFFFF21  bl 0x8251a898
	ctx.lr = 0x8251A97C;
	sub_8251A898(ctx, base);
	// 8251A97C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251A980: 4800004C  b 0x8251a9cc
	pc = 0x8251A9CC; continue 'dispatch;
	// 8251A984: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8251A988: 419A0034  beq cr6, 0x8251a9bc
	if ctx.cr[6].eq {
	pc = 0x8251A9BC; continue 'dispatch;
	}
	// 8251A98C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A990: 48C8EFF9  bl 0x831a9988
	ctx.lr = 0x8251A994;
	sub_831A9988(ctx, base);
	// 8251A994: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251A998: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251A99C: 386BD0E4  addi r3, r11, -0x2f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -12060;
	// 8251A9A0: 48C8D759  bl 0x831a80f8
	ctx.lr = 0x8251A9A4;
	sub_831A80F8(ctx, base);
	// 8251A9A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A9A8: 41820014  beq 0x8251a9bc
	if ctx.cr[0].eq {
	pc = 0x8251A9BC; continue 'dispatch;
	}
	// 8251A9AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A9B0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251A9B4: 4BFFEFD5  bl 0x82519988
	ctx.lr = 0x8251A9B8;
	sub_82519988(ctx, base);
	// 8251A9B8: 4BFFFFC4  b 0x8251a97c
	pc = 0x8251A97C; continue 'dispatch;
	// 8251A9BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251A9C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A9C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251A9C8: 4BFF6BD1  bl 0x82511598
	ctx.lr = 0x8251A9CC;
	sub_82511598(ctx, base);
	// 8251A9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251A9D0: 48C8D7E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251A9D8 size=20
    let mut pc: u32 = 0x8251A9D8;
    'dispatch: loop {
        match pc {
            0x8251A9D8 => {
    //   block [0x8251A9D8..0x8251A9EC)
	// 8251A9D8: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 8251A9DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251A9E0: 409A000C  bne cr6, 0x8251a9ec
	if !ctx.cr[6].eq {
		sub_8251A9EC(ctx, base);
		return;
	}
	// 8251A9E4: 806300D8  lwz r3, 0xd8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 8251A9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251A9EC size=8
    let mut pc: u32 = 0x8251A9EC;
    'dispatch: loop {
        match pc {
            0x8251A9EC => {
    //   block [0x8251A9EC..0x8251A9F4)
	// 8251A9EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251A9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251A9F8 size=216
    let mut pc: u32 = 0x8251A9F8;
    'dispatch: loop {
        match pc {
            0x8251A9F8 => {
    //   block [0x8251A9F8..0x8251AAD0)
	// 8251A9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A9FC: 48C8D771  bl 0x831a816c
	ctx.lr = 0x8251AA00;
	sub_831A8130(ctx, base);
	// 8251AA00: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8251AA04: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AA08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251AA0C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251AA10: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251AA14: 48962E2D  bl 0x82e7d840
	ctx.lr = 0x8251AA18;
	sub_82E7D840(ctx, base);
	// 8251AA18: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8251AA1C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251AA20: 419A00A4  beq cr6, 0x8251aac4
	if ctx.cr[6].eq {
	pc = 0x8251AAC4; continue 'dispatch;
	}
	// 8251AA24: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251AA28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251AA2C: 419A0090  beq cr6, 0x8251aabc
	if ctx.cr[6].eq {
	pc = 0x8251AABC; continue 'dispatch;
	}
	// 8251AA30: 488E3DB1  bl 0x82dfe7e0
	ctx.lr = 0x8251AA34;
	sub_82DFE7E0(ctx, base);
	// 8251AA34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251AA38: 41820084  beq 0x8251aabc
	if ctx.cr[0].eq {
	pc = 0x8251AABC; continue 'dispatch;
	}
	// 8251AA3C: 3BDF00D8  addi r30, r31, 0xd8
	ctx.r[30].s64 = ctx.r[31].s64 + 216;
	// 8251AA40: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251AA44: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8251AA48: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251AA4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251AA50: 48908589  bl 0x82e22fd8
	ctx.lr = 0x8251AA54;
	sub_82E22FD8(ctx, base);
	// 8251AA54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251AA58: 987F00D4  stb r3, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[3].u8 ) };
	// 8251AA5C: 40820068  bne 0x8251aac4
	if !ctx.cr[0].eq {
	pc = 0x8251AAC4; continue 'dispatch;
	}
	// 8251AA60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8251AA64: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251AA68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251AA6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251AA70: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251AA74: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251AA78: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8251AA7C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251AA80: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8251AA84: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8251AA88: 48962ED1  bl 0x82e7d958
	ctx.lr = 0x8251AA8C;
	sub_82E7D958(ctx, base);
	// 8251AA8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251AA90: D3E1006C  stfs f31, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8251AA94: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251AA98: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251AA9C: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251AAA0: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8251AAA4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8251AAA8: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8251AAAC: 48962EF5  bl 0x82e7d9a0
	ctx.lr = 0x8251AAB0;
	sub_82E7D9A0(ctx, base);
	// 8251AAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251AAB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AAB8: 4800000C  b 0x8251aac4
	pc = 0x8251AAC4; continue 'dispatch;
	// 8251AABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251AAC0: 997F00D4  stb r11, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 8251AAC4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251AAC8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8251AACC: 48C8D6F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8251AAD0 size=20
    let mut pc: u32 = 0x8251AAD0;
    'dispatch: loop {
        match pc {
            0x8251AAD0 => {
    //   block [0x8251AAD0..0x8251AAE4)
	// 8251AAD0: 396000F0  li r11, 0xf0
	ctx.r[11].s64 = 240;
	// 8251AAD4: 988300EC  stb r4, 0xec(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), ctx.r[4].u8 ) };
	// 8251AAD8: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251AAE8 size=172
    let mut pc: u32 = 0x8251AAE8;
    'dispatch: loop {
        match pc {
            0x8251AAE8 => {
    //   block [0x8251AAE8..0x8251AB94)
	// 8251AAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251AAF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251AAF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251AAF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AAFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251AB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251AB04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251AB08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251AB0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AB10: 4BDA5E29  bl 0x822c0938
	ctx.lr = 0x8251AB14;
	sub_822C0938(ctx, base);
	// 8251AB14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251AB18: 41820028  beq 0x8251ab40
	if ctx.cr[0].eq {
	pc = 0x8251AB40; continue 'dispatch;
	}
	// 8251AB1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AB20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251AB24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251AB28: 392B22D4  addi r9, r11, 0x22d4
	ctx.r[9].s64 = ctx.r[11].s64 + 8916;
	// 8251AB2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251AB30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251AB34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251AB38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251AB3C: 48000008  b 0x8251ab44
	pc = 0x8251AB44; continue 'dispatch;
	// 8251AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251AB44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AB48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AB4C: 409A002C  bne cr6, 0x8251ab78
	if !ctx.cr[6].eq {
	pc = 0x8251AB78; continue 'dispatch;
	}
	// 8251AB50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251AB54: 4BDA5715  bl 0x822c0268
	ctx.lr = 0x8251AB58;
	sub_822C0268(ctx, base);
	// 8251AB58: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251AB5C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251AB60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AB64: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251AB68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251AB6C: 816BD120  lwz r11, -0x2ee0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12000 as u32) ) } as u64;
	// 8251AB70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251AB74: 4BDA548D  bl 0x822c0000
	ctx.lr = 0x8251AB78;
	sub_822C0000(ctx, base);
	// 8251AB78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251AB7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251AB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251AB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251AB88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251AB8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251AB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251AB98 size=144
    let mut pc: u32 = 0x8251AB98;
    'dispatch: loop {
        match pc {
            0x8251AB98 => {
    //   block [0x8251AB98..0x8251AC28)
	// 8251AB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251ABA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251ABA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251ABA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251ABAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251ABB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251ABB4: 396B22FC  addi r11, r11, 0x22fc
	ctx.r[11].s64 = ctx.r[11].s64 + 8956;
	// 8251ABB8: 394A22E8  addi r10, r10, 0x22e8
	ctx.r[10].s64 = ctx.r[10].s64 + 8936;
	// 8251ABBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251ABC0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8251ABC4: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8251ABC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ABCC: 419A0008  beq cr6, 0x8251abd4
	if ctx.cr[6].eq {
	pc = 0x8251ABD4; continue 'dispatch;
	}
	// 8251ABD0: 4BDA5CC1  bl 0x822c0890
	ctx.lr = 0x8251ABD4;
	sub_822C0890(ctx, base);
	// 8251ABD4: 807F00E0  lwz r3, 0xe0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 8251ABD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ABDC: 419A0008  beq cr6, 0x8251abe4
	if ctx.cr[6].eq {
	pc = 0x8251ABE4; continue 'dispatch;
	}
	// 8251ABE0: 4BDA5CB1  bl 0x822c0890
	ctx.lr = 0x8251ABE4;
	sub_822C0890(ctx, base);
	// 8251ABE4: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8251ABE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ABEC: 419A0008  beq cr6, 0x8251abf4
	if ctx.cr[6].eq {
	pc = 0x8251ABF4; continue 'dispatch;
	}
	// 8251ABF0: 4BDA5CA1  bl 0x822c0890
	ctx.lr = 0x8251ABF4;
	sub_822C0890(ctx, base);
	// 8251ABF4: 807F00C8  lwz r3, 0xc8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 8251ABF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ABFC: 419A0008  beq cr6, 0x8251ac04
	if ctx.cr[6].eq {
	pc = 0x8251AC04; continue 'dispatch;
	}
	// 8251AC00: 4BDA5C91  bl 0x822c0890
	ctx.lr = 0x8251AC04;
	sub_822C0890(ctx, base);
	// 8251AC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251AC08: 4BFF6591  bl 0x82511198
	ctx.lr = 0x8251AC0C;
	sub_82511198(ctx, base);
	// 8251AC0C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8251AC10: 488D7941  bl 0x82df2550
	ctx.lr = 0x8251AC14;
	sub_82DF2550(ctx, base);
	// 8251AC14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251AC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251AC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251AC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251AC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251AC28 size=8
    let mut pc: u32 = 0x8251AC28;
    'dispatch: loop {
        match pc {
            0x8251AC28 => {
    //   block [0x8251AC28..0x8251AC30)
	// 8251AC28: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8251AC2C: 48000194  b 0x8251adc0
	sub_8251ADC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251AC30 size=176
    let mut pc: u32 = 0x8251AC30;
    'dispatch: loop {
        match pc {
            0x8251AC30 => {
    //   block [0x8251AC30..0x8251ACE0)
	// 8251AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251AC38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251AC3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AC40: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251AC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251AC48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AC4C: 388B059C  addi r4, r11, 0x59c
	ctx.r[4].s64 = ctx.r[11].s64 + 1436;
	// 8251AC50: 488D8DB9  bl 0x82df3a08
	ctx.lr = 0x8251AC54;
	sub_82DF3A08(ctx, base);
	// 8251AC54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251AC58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251AC5C: 4BFF486D  bl 0x8250f4c8
	ctx.lr = 0x8251AC60;
	sub_8250F4C8(ctx, base);
	// 8251AC60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AC64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AC68: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8251AC6C: 409A0008  bne cr6, 0x8251ac74
	if !ctx.cr[6].eq {
	pc = 0x8251AC74; continue 'dispatch;
	}
	// 8251AC70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251AC74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251AC78: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251AC7C: 4BFF09D5  bl 0x8250b650
	ctx.lr = 0x8251AC80;
	sub_8250B650(ctx, base);
	// 8251AC80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AC84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AC88: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8251AC8C: 409A0008  bne cr6, 0x8251ac94
	if !ctx.cr[6].eq {
	pc = 0x8251AC94; continue 'dispatch;
	}
	// 8251AC90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251AC94: 4800D8ED  bl 0x82528580
	ctx.lr = 0x8251AC98;
	sub_82528580(ctx, base);
	// 8251AC98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251AC9C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251ACA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251ACA4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251ACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251ACE0 size=220
    let mut pc: u32 = 0x8251ACE0;
    'dispatch: loop {
        match pc {
            0x8251ACE0 => {
    //   block [0x8251ACE0..0x8251ADBC)
	// 8251ACE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251ACE4: 48C8D485  bl 0x831a8168
	ctx.lr = 0x8251ACE8;
	sub_831A8130(ctx, base);
	// 8251ACE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251ACEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251ACF0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8251ACF4: 488D77E5  bl 0x82df24d8
	ctx.lr = 0x8251ACF8;
	sub_82DF24D8(ctx, base);
	// 8251ACF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251ACFC: 4BFF63F5  bl 0x825110f0
	ctx.lr = 0x8251AD00;
	sub_825110F0(ctx, base);
	// 8251AD00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AD04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251AD08: 396B22FC  addi r11, r11, 0x22fc
	ctx.r[11].s64 = ctx.r[11].s64 + 8956;
	// 8251AD0C: 394A22E8  addi r10, r10, 0x22e8
	ctx.r[10].s64 = ctx.r[10].s64 + 8936;
	// 8251AD10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251AD14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AD18: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8251AD1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AD20: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8251AD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251AD28: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 8251AD2C: 388B2324  addi r4, r11, 0x2324
	ctx.r[4].s64 = ctx.r[11].s64 + 8996;
	// 8251AD30: 38A000D9  li r5, 0xd9
	ctx.r[5].s64 = 217;
	// 8251AD34: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8251AD38: 4BDA56A1  bl 0x822c03d8
	ctx.lr = 0x8251AD3C;
	sub_822C03D8(ctx, base);
	// 8251AD3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251AD40: 41820010  beq 0x8251ad50
	if ctx.cr[0].eq {
	pc = 0x8251AD50; continue 'dispatch;
	}
	// 8251AD44: 48962A9D  bl 0x82e7d7e0
	ctx.lr = 0x8251AD48;
	sub_82E7D7E0(ctx, base);
	// 8251AD48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251AD4C: 48000008  b 0x8251ad54
	pc = 0x8251AD54; continue 'dispatch;
	// 8251AD50: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8251AD54: 93BF00CC  stw r29, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 8251AD58: 397F00CC  addi r11, r31, 0xcc
	ctx.r[11].s64 = ctx.r[31].s64 + 204;
	// 8251AD5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251AD60: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 8251AD64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251AD68: 4BFFFD81  bl 0x8251aae8
	ctx.lr = 0x8251AD6C;
	sub_8251AAE8(ctx, base);
	// 8251AD6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251AD70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251AD74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251AD78: 4BDA5289  bl 0x822c0000
	ctx.lr = 0x8251AD7C;
	sub_822C0000(ctx, base);
	// 8251AD7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8251AD80: 9BDF00D4  stb r30, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u8 ) };
	// 8251AD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251AD88: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 8251AD8C: 93DF00DC  stw r30, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 8251AD90: 93DF00E0  stw r30, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 8251AD94: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 8251AD98: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251AD9C: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 8251ADA0: 9BDF00EC  stb r30, 0xec(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u8 ) };
	// 8251ADA4: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8251ADA8: D01F00F4  stfs f0, 0xf4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8251ADAC: D01F00F8  stfs f0, 0xf8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8251ADB0: D01F00FC  stfs f0, 0xfc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8251ADB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251ADB8: 48C8D400  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251ADC0 size=76
    let mut pc: u32 = 0x8251ADC0;
    'dispatch: loop {
        match pc {
            0x8251ADC0 => {
    //   block [0x8251ADC0..0x8251AE0C)
	// 8251ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251ADC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251ADCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251ADD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251ADD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251ADD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251ADDC: 4BFFFDBD  bl 0x8251ab98
	ctx.lr = 0x8251ADE0;
	sub_8251AB98(ctx, base);
	// 8251ADE0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251ADE4: 4182000C  beq 0x8251adf0
	if ctx.cr[0].eq {
	pc = 0x8251ADF0; continue 'dispatch;
	}
	// 8251ADE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251ADEC: 488D75ED  bl 0x82df23d8
	ctx.lr = 0x8251ADF0;
	sub_82DF23D8(ctx, base);
	// 8251ADF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251ADF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251ADF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251ADFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251AE00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251AE04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251AE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251AE10 size=204
    let mut pc: u32 = 0x8251AE10;
    'dispatch: loop {
        match pc {
            0x8251AE10 => {
    //   block [0x8251AE10..0x8251AEDC)
	// 8251AE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251AE18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251AE1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AE20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251AE24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251AE28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251AE2C: 4BFF469D  bl 0x8250f4c8
	ctx.lr = 0x8251AE30;
	sub_8250F4C8(ctx, base);
	// 8251AE30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AE38: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251AE3C: 409A0008  bne cr6, 0x8251ae44
	if !ctx.cr[6].eq {
	pc = 0x8251AE44; continue 'dispatch;
	}
	// 8251AE40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251AE44: 4BFED6E5  bl 0x82508528
	ctx.lr = 0x8251AE48;
	sub_82508528(ctx, base);
	// 8251AE48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251AE4C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251AE50: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE54: 4890FC95  bl 0x82e2aae8
	ctx.lr = 0x8251AE58;
	sub_82E2AAE8(ctx, base);
	// 8251AE58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251AE5C: 488D6E35  bl 0x82df1c90
	ctx.lr = 0x8251AE60;
	sub_82DF1C90(ctx, base);
	// 8251AE60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AE64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AE68: 388B235C  addi r4, r11, 0x235c
	ctx.r[4].s64 = ctx.r[11].s64 + 9052;
	// 8251AE6C: 488D8B9D  bl 0x82df3a08
	ctx.lr = 0x8251AE70;
	sub_82DF3A08(ctx, base);
	// 8251AE70: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251AE74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251AE78: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251AE7C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251AE80: 489119E1  bl 0x82e2c860
	ctx.lr = 0x8251AE84;
	sub_82E2C860(ctx, base);
	// 8251AE84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251AE88: 3BFF00C4  addi r31, r31, 0xc4
	ctx.r[31].s64 = ctx.r[31].s64 + 196;
	// 8251AE8C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251AE90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251AE94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AE9C: 4BDA95C5  bl 0x822c4460
	ctx.lr = 0x8251AEA0;
	sub_822C4460(ctx, base);
	// 8251AEA0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8251AEA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251AEA8: 419A0008  beq cr6, 0x8251aeb0
	if ctx.cr[6].eq {
	pc = 0x8251AEB0; continue 'dispatch;
	}
	// 8251AEAC: 4BDA59E5  bl 0x822c0890
	ctx.lr = 0x8251AEB0;
	sub_822C0890(ctx, base);
	// 8251AEB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AEB4: 488D8575  bl 0x82df3428
	ctx.lr = 0x8251AEB8;
	sub_82DF3428(ctx, base);
	// 8251AEB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AEBC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251AEC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AEC4: 4890FC3D  bl 0x82e2ab00
	ctx.lr = 0x8251AEC8;
	sub_82E2AB00(ctx, base);
	// 8251AEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251AECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251AED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251AED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251AED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251AEE0 size=380
    let mut pc: u32 = 0x8251AEE0;
    'dispatch: loop {
        match pc {
            0x8251AEE0 => {
    //   block [0x8251AEE0..0x8251B05C)
	// 8251AEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AEE4: 48C8D285  bl 0x831a8168
	ctx.lr = 0x8251AEE8;
	sub_831A8130(ctx, base);
	// 8251AEE8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AEEC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251AEF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251AEF4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8251AEF8: 388B6DE0  addi r4, r11, 0x6de0
	ctx.r[4].s64 = ctx.r[11].s64 + 28128;
	// 8251AEFC: 488D8B0D  bl 0x82df3a08
	ctx.lr = 0x8251AF00;
	sub_82DF3A08(ctx, base);
	// 8251AF00: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251AF04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251AF08: 388B6DD8  addi r4, r11, 0x6dd8
	ctx.r[4].s64 = ctx.r[11].s64 + 28120;
	// 8251AF0C: 488D8AFD  bl 0x82df3a08
	ctx.lr = 0x8251AF10;
	sub_82DF3A08(ctx, base);
	// 8251AF10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AF14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251AF18: 3BCB0EC0  addi r30, r11, 0xec0
	ctx.r[30].s64 = ctx.r[11].s64 + 3776;
	// 8251AF1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251AF20: 488D8AE9  bl 0x82df3a08
	ctx.lr = 0x8251AF24;
	sub_82DF3A08(ctx, base);
	// 8251AF24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AF28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AF2C: 3BAB0EB8  addi r29, r11, 0xeb8
	ctx.r[29].s64 = ctx.r[11].s64 + 3768;
	// 8251AF30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251AF34: 488D8AD5  bl 0x82df3a08
	ctx.lr = 0x8251AF38;
	sub_82DF3A08(ctx, base);
	// 8251AF38: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8251AF3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251AF40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251AF44: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251AF48: 480ADC51  bl 0x825c8b98
	ctx.lr = 0x8251AF4C;
	sub_825C8B98(ctx, base);
	// 8251AF4C: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 8251AF50: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251AF54: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AF58: 480AF5A9  bl 0x825ca500
	ctx.lr = 0x8251AF5C;
	sub_825CA500(ctx, base);
	// 8251AF5C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8251AF60: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251AF64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AF68: 419A000C  beq cr6, 0x8251af74
	if ctx.cr[6].eq {
	pc = 0x8251AF74; continue 'dispatch;
	}
	// 8251AF6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251AF70: 4BDA5921  bl 0x822c0890
	ctx.lr = 0x8251AF74;
	sub_822C0890(ctx, base);
	// 8251AF74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251AF78: 488D84B1  bl 0x82df3428
	ctx.lr = 0x8251AF7C;
	sub_82DF3428(ctx, base);
	// 8251AF7C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251AF80: 488D84A9  bl 0x82df3428
	ctx.lr = 0x8251AF84;
	sub_82DF3428(ctx, base);
	// 8251AF84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251AF88: 488D84A1  bl 0x82df3428
	ctx.lr = 0x8251AF8C;
	sub_82DF3428(ctx, base);
	// 8251AF8C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8251AF90: 488D8499  bl 0x82df3428
	ctx.lr = 0x8251AF94;
	sub_82DF3428(ctx, base);
	// 8251AF94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251AF98: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8251AF9C: 488D8A6D  bl 0x82df3a08
	ctx.lr = 0x8251AFA0;
	sub_82DF3A08(ctx, base);
	// 8251AFA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251AFA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251AFA8: 488D8A61  bl 0x82df3a08
	ctx.lr = 0x8251AFAC;
	sub_82DF3A08(ctx, base);
	// 8251AFAC: 38C10064  addi r6, r1, 0x64
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	// 8251AFB0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8251AFB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251AFB8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251AFBC: 480ADBDD  bl 0x825c8b98
	ctx.lr = 0x8251AFC0;
	sub_825C8B98(ctx, base);
	// 8251AFC0: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8251AFC4: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AFC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AFCC: 419A000C  beq cr6, 0x8251afd8
	if ctx.cr[6].eq {
	pc = 0x8251AFD8; continue 'dispatch;
	}
	// 8251AFD0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251AFD4: 4BDA58BD  bl 0x822c0890
	ctx.lr = 0x8251AFD8;
	sub_822C0890(ctx, base);
	// 8251AFD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251AFDC: 488D844D  bl 0x82df3428
	ctx.lr = 0x8251AFE0;
	sub_82DF3428(ctx, base);
	// 8251AFE0: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8251AFE4: 488D8445  bl 0x82df3428
	ctx.lr = 0x8251AFE8;
	sub_82DF3428(ctx, base);
	// 8251AFE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AFEC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251AFF0: 388B2384  addi r4, r11, 0x2384
	ctx.r[4].s64 = ctx.r[11].s64 + 9092;
	// 8251AFF4: 488D8A15  bl 0x82df3a08
	ctx.lr = 0x8251AFF8;
	sub_82DF3A08(ctx, base);
	// 8251AFF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251AFFC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B000: 388B236C  addi r4, r11, 0x236c
	ctx.r[4].s64 = ctx.r[11].s64 + 9068;
	// 8251B004: 488D8A05  bl 0x82df3a08
	ctx.lr = 0x8251B008;
	sub_82DF3A08(ctx, base);
	// 8251B008: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251B00C: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 8251B010: 38AB7054  addi r5, r11, 0x7054
	ctx.r[5].s64 = ctx.r[11].s64 + 28756;
	// 8251B014: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251B018: 48088039  bl 0x825a3050
	ctx.lr = 0x8251B01C;
	sub_825A3050(ctx, base);
	// 8251B01C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8251B020: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8251B024: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251B028: 48087161  bl 0x825a2188
	ctx.lr = 0x8251B02C;
	sub_825A2188(ctx, base);
	// 8251B02C: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8251B030: 488D83F9  bl 0x82df3428
	ctx.lr = 0x8251B034;
	sub_82DF3428(ctx, base);
	// 8251B034: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251B038: 4BDADC81  bl 0x822c8cb8
	ctx.lr = 0x8251B03C;
	sub_822C8CB8(ctx, base);
	// 8251B03C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B040: 488D83E9  bl 0x82df3428
	ctx.lr = 0x8251B044;
	sub_82DF3428(ctx, base);
	// 8251B044: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251B048: 488D83E1  bl 0x82df3428
	ctx.lr = 0x8251B04C;
	sub_82DF3428(ctx, base);
	// 8251B04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B050: 480AEC81  bl 0x825c9cd0
	ctx.lr = 0x8251B054;
	sub_825C9CD0(ctx, base);
	// 8251B054: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8251B058: 48C8D160  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251B060 size=20
    let mut pc: u32 = 0x8251B060;
    'dispatch: loop {
        match pc {
            0x8251B060 => {
    //   block [0x8251B060..0x8251B074)
	// 8251B060: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B064: 396B2490  addi r11, r11, 0x2490
	ctx.r[11].s64 = ctx.r[11].s64 + 9360;
	// 8251B068: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B06C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8251B070: 488D74E0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B078 size=64
    let mut pc: u32 = 0x8251B078;
    'dispatch: loop {
        match pc {
            0x8251B078 => {
    //   block [0x8251B078..0x8251B0B8)
	// 8251B078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B08C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251B090: 488D7449  bl 0x82df24d8
	ctx.lr = 0x8251B094;
	sub_82DF24D8(ctx, base);
	// 8251B094: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B09C: 396B2490  addi r11, r11, 0x2490
	ctx.r[11].s64 = ctx.r[11].s64 + 9360;
	// 8251B0A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B0A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251B0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B0B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B0B8 size=92
    let mut pc: u32 = 0x8251B0B8;
    'dispatch: loop {
        match pc {
            0x8251B0B8 => {
    //   block [0x8251B0B8..0x8251B114)
	// 8251B0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B0C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B0CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B0D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B0D4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251B0D8: 396B2490  addi r11, r11, 0x2490
	ctx.r[11].s64 = ctx.r[11].s64 + 9360;
	// 8251B0DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251B0E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B0E4: 488D746D  bl 0x82df2550
	ctx.lr = 0x8251B0E8;
	sub_82DF2550(ctx, base);
	// 8251B0E8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251B0EC: 4182000C  beq 0x8251b0f8
	if ctx.cr[0].eq {
	pc = 0x8251B0F8; continue 'dispatch;
	}
	// 8251B0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B0F4: 4BDA5175  bl 0x822c0268
	ctx.lr = 0x8251B0F8;
	sub_822C0268(ctx, base);
	// 8251B0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B0FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B108: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B10C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B118 size=144
    let mut pc: u32 = 0x8251B118;
    'dispatch: loop {
        match pc {
            0x8251B118 => {
    //   block [0x8251B118..0x8251B1A8)
	// 8251B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B124: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B128: 4BFCF961  bl 0x824eaa88
	ctx.lr = 0x8251B12C;
	sub_824EAA88(ctx, base);
	// 8251B12C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B134: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B138: 4BFD0021  bl 0x824eb158
	ctx.lr = 0x8251B13C;
	sub_824EB158(ctx, base);
	// 8251B13C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B144: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B148: 409A0008  bne cr6, 0x8251b150
	if !ctx.cr[6].eq {
	pc = 0x8251B150; continue 'dispatch;
	}
	// 8251B14C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B150: 4BFE5059  bl 0x825001a8
	ctx.lr = 0x8251B154;
	sub_825001A8(ctx, base);
	// 8251B154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B158: 488D6B39  bl 0x82df1c90
	ctx.lr = 0x8251B15C;
	sub_82DF1C90(ctx, base);
	// 8251B15C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B160: 488D6B31  bl 0x82df1c90
	ctx.lr = 0x8251B164;
	sub_82DF1C90(ctx, base);
	// 8251B164: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B168: 4BFCF921  bl 0x824eaa88
	ctx.lr = 0x8251B16C;
	sub_824EAA88(ctx, base);
	// 8251B16C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B170: 4BFD1569  bl 0x824ec6d8
	ctx.lr = 0x8251B174;
	sub_824EC6D8(ctx, base);
	// 8251B174: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B178: 488D6B19  bl 0x82df1c90
	ctx.lr = 0x8251B17C;
	sub_82DF1C90(ctx, base);
	// 8251B17C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B180: 4BFCF909  bl 0x824eaa88
	ctx.lr = 0x8251B184;
	sub_824EAA88(ctx, base);
	// 8251B184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B188: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B18C: 4BFFAF55  bl 0x825160e0
	ctx.lr = 0x8251B190;
	sub_825160E0(ctx, base);
	// 8251B190: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B194: 488D6AFD  bl 0x82df1c90
	ctx.lr = 0x8251B198;
	sub_82DF1C90(ctx, base);
	// 8251B198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251B19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B1A8 size=460
    let mut pc: u32 = 0x8251B1A8;
    'dispatch: loop {
        match pc {
            0x8251B1A8 => {
    //   block [0x8251B1A8..0x8251B374)
	// 8251B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B1B8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B1BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B1C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251B1C4: 4BFCF8C5  bl 0x824eaa88
	ctx.lr = 0x8251B1C8;
	sub_824EAA88(ctx, base);
	// 8251B1C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B1CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B1D0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B1D4: 4BFCFF85  bl 0x824eb158
	ctx.lr = 0x8251B1D8;
	sub_824EB158(ctx, base);
	// 8251B1D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B1DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B1E0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B1E4: 409A0008  bne cr6, 0x8251b1ec
	if !ctx.cr[6].eq {
	pc = 0x8251B1EC; continue 'dispatch;
	}
	// 8251B1E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B1EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B1F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B1F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251B1F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B1FC: 4E800421  bctrl
	ctx.lr = 0x8251B200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B204: 488D6A8D  bl 0x82df1c90
	ctx.lr = 0x8251B208;
	sub_82DF1C90(ctx, base);
	// 8251B208: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B20C: 488D6A85  bl 0x82df1c90
	ctx.lr = 0x8251B210;
	sub_82DF1C90(ctx, base);
	// 8251B210: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B214: 4BFCF875  bl 0x824eaa88
	ctx.lr = 0x8251B218;
	sub_824EAA88(ctx, base);
	// 8251B218: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B21C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B220: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B224: 4BFCFF35  bl 0x824eb158
	ctx.lr = 0x8251B228;
	sub_824EB158(ctx, base);
	// 8251B228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B22C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B230: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B234: 409A0008  bne cr6, 0x8251b23c
	if !ctx.cr[6].eq {
	pc = 0x8251B23C; continue 'dispatch;
	}
	// 8251B238: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B23C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B240: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B248: 4E800421  bctrl
	ctx.lr = 0x8251B24C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B24C: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8251B250: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B254: 488D6A3D  bl 0x82df1c90
	ctx.lr = 0x8251B258;
	sub_82DF1C90(ctx, base);
	// 8251B258: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B25C: 488D6A35  bl 0x82df1c90
	ctx.lr = 0x8251B260;
	sub_82DF1C90(ctx, base);
	// 8251B260: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B264: 4BFCF825  bl 0x824eaa88
	ctx.lr = 0x8251B268;
	sub_824EAA88(ctx, base);
	// 8251B268: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B26C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B270: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B274: 4BFD004D  bl 0x824eb2c0
	ctx.lr = 0x8251B278;
	sub_824EB2C0(ctx, base);
	// 8251B278: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B27C: 4BFFAF15  bl 0x82516190
	ctx.lr = 0x8251B280;
	sub_82516190(ctx, base);
	// 8251B280: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8251B284: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251B288: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B28C: 419A000C  beq cr6, 0x8251b298
	if ctx.cr[6].eq {
	pc = 0x8251B298; continue 'dispatch;
	}
	// 8251B290: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251B294: 4BDA55FD  bl 0x822c0890
	ctx.lr = 0x8251B298;
	sub_822C0890(ctx, base);
	// 8251B298: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B29C: 488D69F5  bl 0x82df1c90
	ctx.lr = 0x8251B2A0;
	sub_82DF1C90(ctx, base);
	// 8251B2A0: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251B2A4: 4BFCF7E5  bl 0x824eaa88
	ctx.lr = 0x8251B2A8;
	sub_824EAA88(ctx, base);
	// 8251B2A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B2AC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251B2B0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B2B4: 4BFD000D  bl 0x824eb2c0
	ctx.lr = 0x8251B2B8;
	sub_824EB2C0(ctx, base);
	// 8251B2B8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B2BC: 4BFFAEE5  bl 0x825161a0
	ctx.lr = 0x8251B2C0;
	sub_825161A0(ctx, base);
	// 8251B2C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B2C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251B2C8: 48C8D249  bl 0x831a8510
	ctx.lr = 0x8251B2CC;
	sub_831A8510(ctx, base);
	// 8251B2CC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8251B2D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251B2D4: 419A0008  beq cr6, 0x8251b2dc
	if ctx.cr[6].eq {
	pc = 0x8251B2DC; continue 'dispatch;
	}
	// 8251B2D8: 4BDA55B9  bl 0x822c0890
	ctx.lr = 0x8251B2DC;
	sub_822C0890(ctx, base);
	// 8251B2DC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251B2E0: 488D69B1  bl 0x82df1c90
	ctx.lr = 0x8251B2E4;
	sub_82DF1C90(ctx, base);
	// 8251B2E4: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251B2E8: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8251B2EC: 4BFCF79D  bl 0x824eaa88
	ctx.lr = 0x8251B2F0;
	sub_824EAA88(ctx, base);
	// 8251B2F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B2F4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251B2F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B2FC: 4BFCFFC5  bl 0x824eb2c0
	ctx.lr = 0x8251B300;
	sub_824EB2C0(ctx, base);
	// 8251B300: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B304: 4BFFD295  bl 0x82518598
	ctx.lr = 0x8251B308;
	sub_82518598(ctx, base);
	// 8251B308: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251B30C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251B310: 419A0008  beq cr6, 0x8251b318
	if ctx.cr[6].eq {
	pc = 0x8251B318; continue 'dispatch;
	}
	// 8251B314: 4BDA557D  bl 0x822c0890
	ctx.lr = 0x8251B318;
	sub_822C0890(ctx, base);
	// 8251B318: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251B31C: 488D6975  bl 0x82df1c90
	ctx.lr = 0x8251B320;
	sub_82DF1C90(ctx, base);
	// 8251B320: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251B324: 4BFCF765  bl 0x824eaa88
	ctx.lr = 0x8251B328;
	sub_824EAA88(ctx, base);
	// 8251B328: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B330: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B334: 4BFFAC15  bl 0x82515f48
	ctx.lr = 0x8251B338;
	sub_82515F48(ctx, base);
	// 8251B338: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251B33C: 488D6955  bl 0x82df1c90
	ctx.lr = 0x8251B340;
	sub_82DF1C90(ctx, base);
	// 8251B340: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8251B344: 4BFCF745  bl 0x824eaa88
	ctx.lr = 0x8251B348;
	sub_824EAA88(ctx, base);
	// 8251B348: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B34C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B350: 4BFFAAF1  bl 0x82515e40
	ctx.lr = 0x8251B354;
	sub_82515E40(ctx, base);
	// 8251B354: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8251B358: 488D6939  bl 0x82df1c90
	ctx.lr = 0x8251B35C;
	sub_82DF1C90(ctx, base);
	// 8251B35C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8251B360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B368: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B378 size=460
    let mut pc: u32 = 0x8251B378;
    'dispatch: loop {
        match pc {
            0x8251B378 => {
    //   block [0x8251B378..0x8251B544)
	// 8251B378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B388: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B38C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B390: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251B394: 4BFCF6F5  bl 0x824eaa88
	ctx.lr = 0x8251B398;
	sub_824EAA88(ctx, base);
	// 8251B398: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B39C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B3A0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B3A4: 4BFCFDB5  bl 0x824eb158
	ctx.lr = 0x8251B3A8;
	sub_824EB158(ctx, base);
	// 8251B3A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B3AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B3B0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B3B4: 409A0008  bne cr6, 0x8251b3bc
	if !ctx.cr[6].eq {
	pc = 0x8251B3BC; continue 'dispatch;
	}
	// 8251B3B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B3BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B3C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B3C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251B3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B3CC: 4E800421  bctrl
	ctx.lr = 0x8251B3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B3D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B3D4: 488D68BD  bl 0x82df1c90
	ctx.lr = 0x8251B3D8;
	sub_82DF1C90(ctx, base);
	// 8251B3D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B3DC: 488D68B5  bl 0x82df1c90
	ctx.lr = 0x8251B3E0;
	sub_82DF1C90(ctx, base);
	// 8251B3E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B3E4: 4BFCF6A5  bl 0x824eaa88
	ctx.lr = 0x8251B3E8;
	sub_824EAA88(ctx, base);
	// 8251B3E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B3EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B3F0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B3F4: 4BFCFD65  bl 0x824eb158
	ctx.lr = 0x8251B3F8;
	sub_824EB158(ctx, base);
	// 8251B3F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B3FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B400: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B404: 409A0008  bne cr6, 0x8251b40c
	if !ctx.cr[6].eq {
	pc = 0x8251B40C; continue 'dispatch;
	}
	// 8251B408: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B40C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B410: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B418: 4E800421  bctrl
	ctx.lr = 0x8251B41C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B41C: 7FC3FA14  add r30, r3, r31
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8251B420: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B424: 488D686D  bl 0x82df1c90
	ctx.lr = 0x8251B428;
	sub_82DF1C90(ctx, base);
	// 8251B428: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B42C: 488D6865  bl 0x82df1c90
	ctx.lr = 0x8251B430;
	sub_82DF1C90(ctx, base);
	// 8251B430: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B434: 4BFCF655  bl 0x824eaa88
	ctx.lr = 0x8251B438;
	sub_824EAA88(ctx, base);
	// 8251B438: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B43C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B440: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B444: 4BFCFE7D  bl 0x824eb2c0
	ctx.lr = 0x8251B448;
	sub_824EB2C0(ctx, base);
	// 8251B448: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B44C: 4BFFAD5D  bl 0x825161a8
	ctx.lr = 0x8251B450;
	sub_825161A8(ctx, base);
	// 8251B450: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8251B454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251B458: 419A0008  beq cr6, 0x8251b460
	if ctx.cr[6].eq {
	pc = 0x8251B460; continue 'dispatch;
	}
	// 8251B45C: 4BDA5435  bl 0x822c0890
	ctx.lr = 0x8251B460;
	sub_822C0890(ctx, base);
	// 8251B460: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B464: 488D682D  bl 0x82df1c90
	ctx.lr = 0x8251B468;
	sub_82DF1C90(ctx, base);
	// 8251B468: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251B46C: 4BFCF61D  bl 0x824eaa88
	ctx.lr = 0x8251B470;
	sub_824EAA88(ctx, base);
	// 8251B470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B474: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251B478: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B47C: 4BFCFE45  bl 0x824eb2c0
	ctx.lr = 0x8251B480;
	sub_824EB2C0(ctx, base);
	// 8251B480: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B484: 4BFFAD0D  bl 0x82516190
	ctx.lr = 0x8251B488;
	sub_82516190(ctx, base);
	// 8251B488: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8251B48C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B494: 419A000C  beq cr6, 0x8251b4a0
	if ctx.cr[6].eq {
	pc = 0x8251B4A0; continue 'dispatch;
	}
	// 8251B498: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251B49C: 4BDA53F5  bl 0x822c0890
	ctx.lr = 0x8251B4A0;
	sub_822C0890(ctx, base);
	// 8251B4A0: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251B4A4: 488D67ED  bl 0x82df1c90
	ctx.lr = 0x8251B4A8;
	sub_82DF1C90(ctx, base);
	// 8251B4A8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251B4AC: 4BFCF5DD  bl 0x824eaa88
	ctx.lr = 0x8251B4B0;
	sub_824EAA88(ctx, base);
	// 8251B4B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B4B4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251B4B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B4BC: 4BFCFE05  bl 0x824eb2c0
	ctx.lr = 0x8251B4C0;
	sub_824EB2C0(ctx, base);
	// 8251B4C0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B4C4: 4BFFACDD  bl 0x825161a0
	ctx.lr = 0x8251B4C8;
	sub_825161A0(ctx, base);
	// 8251B4C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251B4CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251B4D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251B4D4: 48C8D03D  bl 0x831a8510
	ctx.lr = 0x8251B4D8;
	sub_831A8510(ctx, base);
	// 8251B4D8: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251B4DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251B4E0: 419A0008  beq cr6, 0x8251b4e8
	if ctx.cr[6].eq {
	pc = 0x8251B4E8; continue 'dispatch;
	}
	// 8251B4E4: 4BDA53AD  bl 0x822c0890
	ctx.lr = 0x8251B4E8;
	sub_822C0890(ctx, base);
	// 8251B4E8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251B4EC: 488D67A5  bl 0x82df1c90
	ctx.lr = 0x8251B4F0;
	sub_82DF1C90(ctx, base);
	// 8251B4F0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251B4F4: 4BFCF595  bl 0x824eaa88
	ctx.lr = 0x8251B4F8;
	sub_824EAA88(ctx, base);
	// 8251B4F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B4FC: 7C9FF214  add r4, r31, r30
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 8251B500: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B504: 4BFFA8E5  bl 0x82515de8
	ctx.lr = 0x8251B508;
	sub_82515DE8(ctx, base);
	// 8251B508: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251B50C: 488D6785  bl 0x82df1c90
	ctx.lr = 0x8251B510;
	sub_82DF1C90(ctx, base);
	// 8251B510: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8251B514: 4BFCF575  bl 0x824eaa88
	ctx.lr = 0x8251B518;
	sub_824EAA88(ctx, base);
	// 8251B518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B51C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B520: 4BFFA921  bl 0x82515e40
	ctx.lr = 0x8251B524;
	sub_82515E40(ctx, base);
	// 8251B524: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8251B528: 488D6769  bl 0x82df1c90
	ctx.lr = 0x8251B52C;
	sub_82DF1C90(ctx, base);
	// 8251B52C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8251B530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B538: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B53C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B548 size=216
    let mut pc: u32 = 0x8251B548;
    'dispatch: loop {
        match pc {
            0x8251B548 => {
    //   block [0x8251B548..0x8251B620)
	// 8251B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B554: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B558: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B55C: 4BFCF52D  bl 0x824eaa88
	ctx.lr = 0x8251B560;
	sub_824EAA88(ctx, base);
	// 8251B560: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B568: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B56C: 4BFCFBED  bl 0x824eb158
	ctx.lr = 0x8251B570;
	sub_824EB158(ctx, base);
	// 8251B570: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B578: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B57C: 409A0008  bne cr6, 0x8251b584
	if !ctx.cr[6].eq {
	pc = 0x8251B584; continue 'dispatch;
	}
	// 8251B580: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B584: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B588: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B58C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B590: 4E800421  bctrl
	ctx.lr = 0x8251B594;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B59C: 488D66F5  bl 0x82df1c90
	ctx.lr = 0x8251B5A0;
	sub_82DF1C90(ctx, base);
	// 8251B5A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B5A4: 488D66ED  bl 0x82df1c90
	ctx.lr = 0x8251B5A8;
	sub_82DF1C90(ctx, base);
	// 8251B5A8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B5AC: 4BFCF4DD  bl 0x824eaa88
	ctx.lr = 0x8251B5B0;
	sub_824EAA88(ctx, base);
	// 8251B5B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B5B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B5B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B5BC: 4BFCFD05  bl 0x824eb2c0
	ctx.lr = 0x8251B5C0;
	sub_824EB2C0(ctx, base);
	// 8251B5C0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B5C4: 4BFFABCD  bl 0x82516190
	ctx.lr = 0x8251B5C8;
	sub_82516190(ctx, base);
	// 8251B5C8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8251B5CC: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8251B5D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B5D4: 419A000C  beq cr6, 0x8251b5e0
	if ctx.cr[6].eq {
	pc = 0x8251B5E0; continue 'dispatch;
	}
	// 8251B5D8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251B5DC: 4BDA52B5  bl 0x822c0890
	ctx.lr = 0x8251B5E0;
	sub_822C0890(ctx, base);
	// 8251B5E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B5E4: 488D66AD  bl 0x82df1c90
	ctx.lr = 0x8251B5E8;
	sub_82DF1C90(ctx, base);
	// 8251B5E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B5EC: 4BFCF49D  bl 0x824eaa88
	ctx.lr = 0x8251B5F0;
	sub_824EAA88(ctx, base);
	// 8251B5F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B5F4: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8251B5F8: 4BFFA849  bl 0x82515e40
	ctx.lr = 0x8251B5FC;
	sub_82515E40(ctx, base);
	// 8251B5FC: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8251B600: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B604: 488D668D  bl 0x82df1c90
	ctx.lr = 0x8251B608;
	sub_82DF1C90(ctx, base);
	// 8251B608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B60C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251B610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251B620 size=20
    let mut pc: u32 = 0x8251B620;
    'dispatch: loop {
        match pc {
            0x8251B620 => {
    //   block [0x8251B620..0x8251B634)
	// 8251B620: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B624: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8251B628: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B62C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8251B630: 488D6F20  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251B638 size=8
    let mut pc: u32 = 0x8251B638;
    'dispatch: loop {
        match pc {
            0x8251B638 => {
    //   block [0x8251B638..0x8251B640)
	// 8251B638: 8863003C  lbz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8251B63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251B640 size=16
    let mut pc: u32 = 0x8251B640;
    'dispatch: loop {
        match pc {
            0x8251B640 => {
    //   block [0x8251B640..0x8251B650)
	// 8251B640: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8251B644: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8251B648: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8251B64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B650 size=136
    let mut pc: u32 = 0x8251B650;
    'dispatch: loop {
        match pc {
            0x8251B650 => {
    //   block [0x8251B650..0x8251B6D8)
	// 8251B650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B65C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B664: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251B668: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251B66C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8251B670: 409A0020  bne cr6, 0x8251b690
	if !ctx.cr[6].eq {
	pc = 0x8251B690; continue 'dispatch;
	}
	// 8251B674: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251B678: 419A0048  beq cr6, 0x8251b6c0
	if ctx.cr[6].eq {
	pc = 0x8251B6C0; continue 'dispatch;
	}
	// 8251B67C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B680: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B684: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251B688: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251B68C: 48000034  b 0x8251b6c0
	pc = 0x8251B6C0; continue 'dispatch;
	// 8251B690: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8251B694: 419A002C  beq cr6, 0x8251b6c0
	if ctx.cr[6].eq {
	pc = 0x8251B6C0; continue 'dispatch;
	}
	// 8251B698: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251B69C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B6A0: 388BD1A0  addi r4, r11, -0x2e60
	ctx.r[4].s64 = ctx.r[11].s64 + -11872;
	// 8251B6A4: 48C8CA55  bl 0x831a80f8
	ctx.lr = 0x8251B6A8;
	sub_831A80F8(ctx, base);
	// 8251B6A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251B6AC: 4182000C  beq 0x8251b6b8
	if ctx.cr[0].eq {
	pc = 0x8251B6B8; continue 'dispatch;
	}
	// 8251B6B0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8251B6B4: 4800000C  b 0x8251b6c0
	pc = 0x8251B6C0; continue 'dispatch;
	// 8251B6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251B6BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B6C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B6C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B6C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B6CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B6D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B6D8 size=64
    let mut pc: u32 = 0x8251B6D8;
    'dispatch: loop {
        match pc {
            0x8251B6D8 => {
    //   block [0x8251B6D8..0x8251B718)
	// 8251B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B6E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B6E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B6E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B6EC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251B6F0: 488D6DE9  bl 0x82df24d8
	ctx.lr = 0x8251B6F4;
	sub_82DF24D8(ctx, base);
	// 8251B6F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B6FC: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8251B700: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251B708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B718 size=92
    let mut pc: u32 = 0x8251B718;
    'dispatch: loop {
        match pc {
            0x8251B718 => {
    //   block [0x8251B718..0x8251B774)
	// 8251B718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B730: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B734: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251B738: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8251B73C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251B740: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B744: 488D6E0D  bl 0x82df2550
	ctx.lr = 0x8251B748;
	sub_82DF2550(ctx, base);
	// 8251B748: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251B74C: 4182000C  beq 0x8251b758
	if ctx.cr[0].eq {
	pc = 0x8251B758; continue 'dispatch;
	}
	// 8251B750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B754: 4BDA4B15  bl 0x822c0268
	ctx.lr = 0x8251B758;
	sub_822C0268(ctx, base);
	// 8251B758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B76C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B778 size=80
    let mut pc: u32 = 0x8251B778;
    'dispatch: loop {
        match pc {
            0x8251B778 => {
    //   block [0x8251B778..0x8251B7C8)
	// 8251B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B788: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251B78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B790: 816B1668  lwz r11, 0x1668(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5736 as u32) ) } as u64;
	// 8251B794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B798: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251B79C: 409A0008  bne cr6, 0x8251b7a4
	if !ctx.cr[6].eq {
	pc = 0x8251B7A4; continue 'dispatch;
	}
	// 8251B7A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251B7A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8251B7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B7AC: 488D644D  bl 0x82df1bf8
	ctx.lr = 0x8251B7B0;
	sub_82DF1BF8(ctx, base);
	// 8251B7B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B7B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251B7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B7C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B7C8 size=72
    let mut pc: u32 = 0x8251B7C8;
    'dispatch: loop {
        match pc {
            0x8251B7C8 => {
    //   block [0x8251B7C8..0x8251B810)
	// 8251B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B7D4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 8251B7D8: 419A001C  beq cr6, 0x8251b7f4
	if ctx.cr[6].eq {
	pc = 0x8251B7F4; continue 'dispatch;
	}
	// 8251B7DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251B7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251B7E4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8251B7E8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251B7EC: 4BFFFE65  bl 0x8251b650
	ctx.lr = 0x8251B7F0;
	sub_8251B650(ctx, base);
	// 8251B7F0: 48000010  b 0x8251b800
	pc = 0x8251B800; continue 'dispatch;
	// 8251B7F4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251B7F8: 396BD1A0  addi r11, r11, -0x2e60
	ctx.r[11].s64 = ctx.r[11].s64 + -11872;
	// 8251B7FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251B804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B810 size=744
    let mut pc: u32 = 0x8251B810;
    'dispatch: loop {
        match pc {
            0x8251B810 => {
    //   block [0x8251B810..0x8251BAF8)
	// 8251B810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B814: 48C8C955  bl 0x831a8168
	ctx.lr = 0x8251B818;
	sub_831A8130(ctx, base);
	// 8251B818: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B81C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B824: 4BDA47DD  bl 0x822c0000
	ctx.lr = 0x8251B828;
	sub_822C0000(ctx, base);
	// 8251B828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B82C: 4BFFFD1D  bl 0x8251b548
	ctx.lr = 0x8251B830;
	sub_8251B548(ctx, base);
	// 8251B830: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251B834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251B838: 388B24C0  addi r4, r11, 0x24c0
	ctx.r[4].s64 = ctx.r[11].s64 + 9408;
	// 8251B83C: 38A000B4  li r5, 0xb4
	ctx.r[5].s64 = 180;
	// 8251B840: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251B844: 4BDA4B95  bl 0x822c03d8
	ctx.lr = 0x8251B848;
	sub_822C03D8(ctx, base);
	// 8251B848: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8251B84C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8251B850: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251B854: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251B858: 4198021C  blt cr6, 0x8251ba74
	if ctx.cr[6].lt {
	pc = 0x8251BA74; continue 'dispatch;
	}
	// 8251B85C: 419A01BC  beq cr6, 0x8251ba18
	if ctx.cr[6].eq {
	pc = 0x8251BA18; continue 'dispatch;
	}
	// 8251B860: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8251B864: 4198017C  blt cr6, 0x8251b9e0
	if ctx.cr[6].lt {
	pc = 0x8251B9E0; continue 'dispatch;
	}
	// 8251B868: 419A0128  beq cr6, 0x8251b990
	if ctx.cr[6].eq {
	pc = 0x8251B990; continue 'dispatch;
	}
	// 8251B86C: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8251B870: 419800E0  blt cr6, 0x8251b950
	if ctx.cr[6].lt {
	pc = 0x8251B950; continue 'dispatch;
	}
	// 8251B874: 409A0250  bne cr6, 0x8251bac4
	if !ctx.cr[6].eq {
	pc = 0x8251BAC4; continue 'dispatch;
	}
	// 8251B878: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B87C: 4BFFFEFD  bl 0x8251b778
	ctx.lr = 0x8251B880;
	sub_8251B778(ctx, base);
	// 8251B880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B888: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B88C: 409A0008  bne cr6, 0x8251b894
	if !ctx.cr[6].eq {
	pc = 0x8251B894; continue 'dispatch;
	}
	// 8251B890: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B894: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B898: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251B89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B8A0: 4E800421  bctrl
	ctx.lr = 0x8251B8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B8A4: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 8251B8A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251B8AC: 488D63E5  bl 0x82df1c90
	ctx.lr = 0x8251B8B0;
	sub_82DF1C90(ctx, base);
	// 8251B8B0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8251B8B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251B8B8: 419A0044  beq cr6, 0x8251b8fc
	if ctx.cr[6].eq {
	pc = 0x8251B8FC; continue 'dispatch;
	}
	// 8251B8BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B8C0: 4BFFFEB9  bl 0x8251b778
	ctx.lr = 0x8251B8C4;
	sub_8251B778(ctx, base);
	// 8251B8C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B8C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B8CC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B8D0: 409A0008  bne cr6, 0x8251b8d8
	if !ctx.cr[6].eq {
	pc = 0x8251B8D8; continue 'dispatch;
	}
	// 8251B8D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B8D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B8DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251B8E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B8E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251B8E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B8EC: 4E800421  bctrl
	ctx.lr = 0x8251B8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B8F0: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251B8F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251B8F8: 4800015C  b 0x8251ba54
	pc = 0x8251BA54; continue 'dispatch;
	// 8251B8FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B900: 4BFFF819  bl 0x8251b118
	ctx.lr = 0x8251B904;
	sub_8251B118(ctx, base);
	// 8251B904: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B90C: 4BFFFA6D  bl 0x8251b378
	ctx.lr = 0x8251B910;
	sub_8251B378(ctx, base);
	// 8251B910: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B914: 4BFFFE65  bl 0x8251b778
	ctx.lr = 0x8251B918;
	sub_8251B778(ctx, base);
	// 8251B918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B91C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B920: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B924: 409A0008  bne cr6, 0x8251b92c
	if !ctx.cr[6].eq {
	pc = 0x8251B92C; continue 'dispatch;
	}
	// 8251B928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B92C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B930: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251B934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B938: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251B93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B940: 4E800421  bctrl
	ctx.lr = 0x8251B944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B944: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251B948: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251B94C: 48000174  b 0x8251bac0
	pc = 0x8251BAC0; continue 'dispatch;
	// 8251B950: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B954: 4BFFFE25  bl 0x8251b778
	ctx.lr = 0x8251B958;
	sub_8251B778(ctx, base);
	// 8251B958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B95C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B960: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B964: 409A0008  bne cr6, 0x8251b96c
	if !ctx.cr[6].eq {
	pc = 0x8251B96C; continue 'dispatch;
	}
	// 8251B968: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B96C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B970: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251B974: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B978: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251B97C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B980: 4E800421  bctrl
	ctx.lr = 0x8251B984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B984: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251B988: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251B98C: 480000C8  b 0x8251ba54
	pc = 0x8251BA54; continue 'dispatch;
	// 8251B990: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251B998: 4BFFF9E1  bl 0x8251b378
	ctx.lr = 0x8251B99C;
	sub_8251B378(ctx, base);
	// 8251B99C: 9B9F003E  stb r28, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[28].u8 ) };
	// 8251B9A0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B9A4: 4BFFFDD5  bl 0x8251b778
	ctx.lr = 0x8251B9A8;
	sub_8251B778(ctx, base);
	// 8251B9A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B9B0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B9B4: 409A0008  bne cr6, 0x8251b9bc
	if !ctx.cr[6].eq {
	pc = 0x8251B9BC; continue 'dispatch;
	}
	// 8251B9B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B9BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B9C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251B9C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251B9C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B9CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B9D0: 4E800421  bctrl
	ctx.lr = 0x8251B9D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251B9D4: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251B9D8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251B9DC: 480000E4  b 0x8251bac0
	pc = 0x8251BAC0; continue 'dispatch;
	// 8251B9E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251B9E4: 4BFFFD95  bl 0x8251b778
	ctx.lr = 0x8251B9E8;
	sub_8251B778(ctx, base);
	// 8251B9E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B9EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B9F0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251B9F4: 409A0008  bne cr6, 0x8251b9fc
	if !ctx.cr[6].eq {
	pc = 0x8251B9FC; continue 'dispatch;
	}
	// 8251B9F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251B9FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA00: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251BA04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251BA08: 4E800421  bctrl
	ctx.lr = 0x8251BA0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251BA0C: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251BA10: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251BA14: 480000AC  b 0x8251bac0
	pc = 0x8251BAC0; continue 'dispatch;
	// 8251BA18: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251BA1C: 4BFFFD5D  bl 0x8251b778
	ctx.lr = 0x8251BA20;
	sub_8251B778(ctx, base);
	// 8251BA20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BA28: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251BA2C: 409A0008  bne cr6, 0x8251ba34
	if !ctx.cr[6].eq {
	pc = 0x8251BA34; continue 'dispatch;
	}
	// 8251BA30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251BA34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA38: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251BA3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251BA40: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251BA44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251BA48: 4E800421  bctrl
	ctx.lr = 0x8251BA4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251BA4C: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251BA50: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8251BA54: 488D623D  bl 0x82df1c90
	ctx.lr = 0x8251BA58;
	sub_82DF1C90(ctx, base);
	// 8251BA58: 897F003D  lbz r11, 0x3d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(61 as u32) ) } as u64;
	// 8251BA5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251BA60: 41820064  beq 0x8251bac4
	if ctx.cr[0].eq {
	pc = 0x8251BAC4; continue 'dispatch;
	}
	// 8251BA64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251BA68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BA6C: 4BFFF73D  bl 0x8251b1a8
	ctx.lr = 0x8251BA70;
	sub_8251B1A8(ctx, base);
	// 8251BA70: 48000054  b 0x8251bac4
	pc = 0x8251BAC4; continue 'dispatch;
	// 8251BA74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251BA78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BA7C: 4BFFF8FD  bl 0x8251b378
	ctx.lr = 0x8251BA80;
	sub_8251B378(ctx, base);
	// 8251BA80: 9B9F003E  stb r28, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[28].u8 ) };
	// 8251BA84: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251BA88: 4BFFFCF1  bl 0x8251b778
	ctx.lr = 0x8251BA8C;
	sub_8251B778(ctx, base);
	// 8251BA8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BA94: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251BA98: 409A0008  bne cr6, 0x8251baa0
	if !ctx.cr[6].eq {
	pc = 0x8251BAA0; continue 'dispatch;
	}
	// 8251BA9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251BAA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BAA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251BAA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251BAAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BAB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251BAB4: 4E800421  bctrl
	ctx.lr = 0x8251BAB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251BAB8: 987F003D  stb r3, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[3].u8 ) };
	// 8251BABC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251BAC0: 488D61D1  bl 0x82df1c90
	ctx.lr = 0x8251BAC4;
	sub_82DF1C90(ctx, base);
	// 8251BAC4: 9B9F003C  stb r28, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[28].u8 ) };
	// 8251BAC8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251BACC: 4BFCEFBD  bl 0x824eaa88
	ctx.lr = 0x8251BAD0;
	sub_824EAA88(ctx, base);
	// 8251BAD0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BAD4: 4BFCE285  bl 0x824e9d58
	ctx.lr = 0x8251BAD8;
	sub_824E9D58(ctx, base);
	// 8251BAD8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251BADC: 488D61B5  bl 0x82df1c90
	ctx.lr = 0x8251BAE0;
	sub_82DF1C90(ctx, base);
	// 8251BAE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251BAE4: 4BDA4785  bl 0x822c0268
	ctx.lr = 0x8251BAE8;
	sub_822C0268(ctx, base);
	// 8251BAE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BAEC: 4BDA4515  bl 0x822c0000
	ctx.lr = 0x8251BAF0;
	sub_822C0000(ctx, base);
	// 8251BAF0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8251BAF4: 48C8C6C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BAF8 size=128
    let mut pc: u32 = 0x8251BAF8;
    'dispatch: loop {
        match pc {
            0x8251BAF8 => {
    //   block [0x8251BAF8..0x8251BB78)
	// 8251BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BAFC: 48C8C671  bl 0x831a816c
	ctx.lr = 0x8251BB00;
	sub_831A8130(ctx, base);
	// 8251BB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BB04: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8251BB08: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251BB0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251BB10: 3BEB70CC  addi r31, r11, 0x70cc
	ctx.r[31].s64 = ctx.r[11].s64 + 28876;
	// 8251BB14: 816A70D4  lwz r11, 0x70d4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28884 as u32) ) } as u64;
	// 8251BB18: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8251BB1C: 40820024  bne 0x8251bb40
	if !ctx.cr[0].eq {
	pc = 0x8251BB40; continue 'dispatch;
	}
	// 8251BB20: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 8251BB24: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251BB28: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8251BB2C: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 8251BB30: 3908B7C8  addi r8, r8, -0x4838
	ctx.r[8].s64 = ctx.r[8].s64 + -18488;
	// 8251BB34: 916A70D4  stw r11, 0x70d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28884 as u32), ctx.r[11].u32 ) };
	// 8251BB38: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251BB3C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251BB40: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251BB44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8251BB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BB4C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 8251BB50: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8251BB54: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251BB58: 480B6DC9  bl 0x825d2920
	ctx.lr = 0x8251BB5C;
	sub_825D2920(ctx, base);
	// 8251BB5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251BB60: 4182000C  beq 0x8251bb6c
	if ctx.cr[0].eq {
	pc = 0x8251BB6C; continue 'dispatch;
	}
	// 8251BB64: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8251BB68: 48000008  b 0x8251bb70
	pc = 0x8251BB70; continue 'dispatch;
	// 8251BB6C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8251BB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251BB74: 48C8C648  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BB78 size=248
    let mut pc: u32 = 0x8251BB78;
    'dispatch: loop {
        match pc {
            0x8251BB78 => {
    //   block [0x8251BB78..0x8251BC70)
	// 8251BB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BB7C: 48C8C5F1  bl 0x831a816c
	ctx.lr = 0x8251BB80;
	sub_831A8130(ctx, base);
	// 8251BB80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BB84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251BB88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BB8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251BB90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BB94: 388B24FC  addi r4, r11, 0x24fc
	ctx.r[4].s64 = ctx.r[11].s64 + 9468;
	// 8251BB98: 488D7E71  bl 0x82df3a08
	ctx.lr = 0x8251BB9C;
	sub_82DF3A08(ctx, base);
	// 8251BB9C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251BBA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251BBA4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8251BBA8: 396BB810  addi r11, r11, -0x47f0
	ctx.r[11].s64 = ctx.r[11].s64 + -18416;
	// 8251BBAC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251BBB0: 93C10070  stw r30, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 8251BBB4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251BBB8: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8251BBBC: 4BFFFF3D  bl 0x8251baf8
	ctx.lr = 0x8251BBC0;
	sub_8251BAF8(ctx, base);
	// 8251BBC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8251BBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251BBC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251BBCC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251BBD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BBD4: 488DE375  bl 0x82df9f48
	ctx.lr = 0x8251BBD8;
	sub_82DF9F48(ctx, base);
	// 8251BBD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BBDC: 488D784D  bl 0x82df3428
	ctx.lr = 0x8251BBE0;
	sub_82DF3428(ctx, base);
	// 8251BBE0: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8251BBE4: 9BDF003C  stb r30, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u8 ) };
	// 8251BBE8: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 8251BBEC: 9BDF003D  stb r30, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[30].u8 ) };
	// 8251BBF0: 9BDF003E  stb r30, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[30].u8 ) };
	// 8251BBF4: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 8251BBF8: 419A000C  beq cr6, 0x8251bc04
	if ctx.cr[6].eq {
	pc = 0x8251BC04; continue 'dispatch;
	}
	// 8251BBFC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8251BC00: 409A005C  bne cr6, 0x8251bc5c
	if !ctx.cr[6].eq {
	pc = 0x8251BC5C; continue 'dispatch;
	}
	// 8251BC04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251BC08: 4BDA4E51  bl 0x822c0a58
	ctx.lr = 0x8251BC0C;
	sub_822C0A58(ctx, base);
	// 8251BC0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BC14: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251BC18: 409A0008  bne cr6, 0x8251bc20
	if !ctx.cr[6].eq {
	pc = 0x8251BC20; continue 'dispatch;
	}
	// 8251BC1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251BC20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BC24: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8251BC28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251BC2C: 4E800421  bctrl
	ctx.lr = 0x8251BC30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251BC30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251BC34: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251BC38: 488D6059  bl 0x82df1c90
	ctx.lr = 0x8251BC3C;
	sub_82DF1C90(ctx, base);
	// 8251BC3C: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251BC40: 4182001C  beq 0x8251bc5c
	if ctx.cr[0].eq {
	pc = 0x8251BC5C; continue 'dispatch;
	}
	// 8251BC44: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251BC48: 4BFCEE41  bl 0x824eaa88
	ctx.lr = 0x8251BC4C;
	sub_824EAA88(ctx, base);
	// 8251BC4C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BC50: 4BFCE0B1  bl 0x824e9d00
	ctx.lr = 0x8251BC54;
	sub_824E9D00(ctx, base);
	// 8251BC54: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251BC58: 488D6039  bl 0x82df1c90
	ctx.lr = 0x8251BC5C;
	sub_82DF1C90(ctx, base);
	// 8251BC5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BC60: 48B827D9  bl 0x8309e438
	ctx.lr = 0x8251BC64;
	sub_8309E438(ctx, base);
	// 8251BC64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BC68: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8251BC6C: 48C8C550  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251BC70 size=20
    let mut pc: u32 = 0x8251BC70;
    'dispatch: loop {
        match pc {
            0x8251BC70 => {
    //   block [0x8251BC70..0x8251BC84)
	// 8251BC70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251BC74: 396B2510  addi r11, r11, 0x2510
	ctx.r[11].s64 = ctx.r[11].s64 + 9488;
	// 8251BC78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BC7C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8251BC80: 488D68D0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BC88 size=64
    let mut pc: u32 = 0x8251BC88;
    'dispatch: loop {
        match pc {
            0x8251BC88 => {
    //   block [0x8251BC88..0x8251BCC8)
	// 8251BC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BC90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251BC94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BC9C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251BCA0: 488D6839  bl 0x82df24d8
	ctx.lr = 0x8251BCA4;
	sub_82DF24D8(ctx, base);
	// 8251BCA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251BCA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BCAC: 396B2510  addi r11, r11, 0x2510
	ctx.r[11].s64 = ctx.r[11].s64 + 9488;
	// 8251BCB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251BCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BCC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251BCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BCC8 size=92
    let mut pc: u32 = 0x8251BCC8;
    'dispatch: loop {
        match pc {
            0x8251BCC8 => {
    //   block [0x8251BCC8..0x8251BD24)
	// 8251BCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BCD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251BCD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251BCD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BCDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BCE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251BCE4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251BCE8: 396B2510  addi r11, r11, 0x2510
	ctx.r[11].s64 = ctx.r[11].s64 + 9488;
	// 8251BCEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251BCF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BCF4: 488D685D  bl 0x82df2550
	ctx.lr = 0x8251BCF8;
	sub_82DF2550(ctx, base);
	// 8251BCF8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251BCFC: 4182000C  beq 0x8251bd08
	if ctx.cr[0].eq {
	pc = 0x8251BD08; continue 'dispatch;
	}
	// 8251BD00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BD04: 4BDA4565  bl 0x822c0268
	ctx.lr = 0x8251BD08;
	sub_822C0268(ctx, base);
	// 8251BD08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BD0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BD18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251BD1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251BD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251BD28 size=8
    let mut pc: u32 = 0x8251BD28;
    'dispatch: loop {
        match pc {
            0x8251BD28 => {
    //   block [0x8251BD28..0x8251BD30)
	// 8251BD28: 80630130  lwz r3, 0x130(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) } as u64;
	// 8251BD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251BD30 size=8
    let mut pc: u32 = 0x8251BD30;
    'dispatch: loop {
        match pc {
            0x8251BD30 => {
    //   block [0x8251BD30..0x8251BD38)
	// 8251BD30: 80630134  lwz r3, 0x134(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8251BD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251BD38 size=16
    let mut pc: u32 = 0x8251BD38;
    'dispatch: loop {
        match pc {
            0x8251BD38 => {
    //   block [0x8251BD38..0x8251BD48)
	// 8251BD38: 3964004E  addi r11, r4, 0x4e
	ctx.r[11].s64 = ctx.r[4].s64 + 78;
	// 8251BD3C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251BD40: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8251BD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251BD48 size=16
    let mut pc: u32 = 0x8251BD48;
    'dispatch: loop {
        match pc {
            0x8251BD48 => {
    //   block [0x8251BD48..0x8251BD58)
	// 8251BD48: 39640057  addi r11, r4, 0x57
	ctx.r[11].s64 = ctx.r[4].s64 + 87;
	// 8251BD4C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251BD50: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8251BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BD58 size=144
    let mut pc: u32 = 0x8251BD58;
    'dispatch: loop {
        match pc {
            0x8251BD58 => {
    //   block [0x8251BD58..0x8251BDE8)
	// 8251BD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BD60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251BD64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251BD68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BD6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251BD70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251BD74: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8251BD78: 409A0028  bne cr6, 0x8251bda0
	if !ctx.cr[6].eq {
	pc = 0x8251BDA0; continue 'dispatch;
	}
	// 8251BD7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251BD80: 419A0050  beq cr6, 0x8251bdd0
	if ctx.cr[6].eq {
	pc = 0x8251BDD0; continue 'dispatch;
	}
	// 8251BD84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BD88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BD8C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251BD90: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251BD94: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BD98: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251BD9C: 48000034  b 0x8251bdd0
	pc = 0x8251BDD0; continue 'dispatch;
	// 8251BDA0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8251BDA4: 419A002C  beq cr6, 0x8251bdd0
	if ctx.cr[6].eq {
	pc = 0x8251BDD0; continue 'dispatch;
	}
	// 8251BDA8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251BDAC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BDB0: 388BD2E0  addi r4, r11, -0x2d20
	ctx.r[4].s64 = ctx.r[11].s64 + -11552;
	// 8251BDB4: 48C8C345  bl 0x831a80f8
	ctx.lr = 0x8251BDB8;
	sub_831A80F8(ctx, base);
	// 8251BDB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251BDBC: 4182000C  beq 0x8251bdc8
	if ctx.cr[0].eq {
	pc = 0x8251BDC8; continue 'dispatch;
	}
	// 8251BDC0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8251BDC4: 4800000C  b 0x8251bdd0
	pc = 0x8251BDD0; continue 'dispatch;
	// 8251BDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251BDCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BDD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251BDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BDDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251BDE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251BDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BDE8 size=84
    let mut pc: u32 = 0x8251BDE8;
    'dispatch: loop {
        match pc {
            0x8251BDE8 => {
    //   block [0x8251BDE8..0x8251BE3C)
	// 8251BDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BDEC: 48C8C381  bl 0x831a816c
	ctx.lr = 0x8251BDF0;
	sub_831A8130(ctx, base);
	// 8251BDF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BDF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251BDF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251BDFC: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251BE00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BE04: 409A0030  bne cr6, 0x8251be34
	if !ctx.cr[6].eq {
	pc = 0x8251BE34; continue 'dispatch;
	}
	// 8251BE08: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 8251BE0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251BE10: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BE14: 4BFFFFD5  bl 0x8251bde8
	ctx.lr = 0x8251BE18;
	sub_8251BDE8(ctx, base);
	// 8251BE18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251BE1C: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251BE20: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BE24: 488D6365  bl 0x82df2188
	ctx.lr = 0x8251BE28;
	sub_82DF2188(ctx, base);
	// 8251BE28: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251BE2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BE30: 419AFFDC  beq cr6, 0x8251be0c
	if ctx.cr[6].eq {
	pc = 0x8251BE0C; continue 'dispatch;
	}
	// 8251BE34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251BE38: 48C8C384  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BE40 size=72
    let mut pc: u32 = 0x8251BE40;
    'dispatch: loop {
        match pc {
            0x8251BE40 => {
    //   block [0x8251BE40..0x8251BE88)
	// 8251BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BE4C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 8251BE50: 419A001C  beq cr6, 0x8251be6c
	if ctx.cr[6].eq {
	pc = 0x8251BE6C; continue 'dispatch;
	}
	// 8251BE54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251BE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251BE5C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8251BE60: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251BE64: 4BFFFEF5  bl 0x8251bd58
	ctx.lr = 0x8251BE68;
	sub_8251BD58(ctx, base);
	// 8251BE68: 48000010  b 0x8251be78
	pc = 0x8251BE78; continue 'dispatch;
	// 8251BE6C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251BE70: 396BD2E0  addi r11, r11, -0x2d20
	ctx.r[11].s64 = ctx.r[11].s64 + -11552;
	// 8251BE74: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251BE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251BE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BE88 size=144
    let mut pc: u32 = 0x8251BE88;
    'dispatch: loop {
        match pc {
            0x8251BE88 => {
    //   block [0x8251BE88..0x8251BF18)
	// 8251BE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251BE94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BE9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BEA0: 4BFCEBE9  bl 0x824eaa88
	ctx.lr = 0x8251BEA4;
	sub_824EAA88(ctx, base);
	// 8251BEA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251BEA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BEAC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BEB0: 4BFCF411  bl 0x824eb2c0
	ctx.lr = 0x8251BEB4;
	sub_824EB2C0(ctx, base);
	// 8251BEB4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BEB8: 488D5DD9  bl 0x82df1c90
	ctx.lr = 0x8251BEBC;
	sub_82DF1C90(ctx, base);
	// 8251BEBC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251BEC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251BEC4: 419A002C  beq cr6, 0x8251bef0
	if ctx.cr[6].eq {
	pc = 0x8251BEF0; continue 'dispatch;
	}
	// 8251BEC8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251BECC: 4BFFC665  bl 0x82518530
	ctx.lr = 0x8251BED0;
	sub_82518530(ctx, base);
	// 8251BED0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251BED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BEDC: 419A000C  beq cr6, 0x8251bee8
	if ctx.cr[6].eq {
	pc = 0x8251BEE8; continue 'dispatch;
	}
	// 8251BEE0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251BEE4: 4BDA49AD  bl 0x822c0890
	ctx.lr = 0x8251BEE8;
	sub_822C0890(ctx, base);
	// 8251BEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BEEC: 48000018  b 0x8251bf04
	pc = 0x8251BF04; continue 'dispatch;
	// 8251BEF0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251BEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251BEF8: 419A0008  beq cr6, 0x8251bf00
	if ctx.cr[6].eq {
	pc = 0x8251BF00; continue 'dispatch;
	}
	// 8251BEFC: 4BDA4995  bl 0x822c0890
	ctx.lr = 0x8251BF00;
	sub_822C0890(ctx, base);
	// 8251BF00: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8251BF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251BF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BF10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251BF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BF18 size=176
    let mut pc: u32 = 0x8251BF18;
    'dispatch: loop {
        match pc {
            0x8251BF18 => {
    //   block [0x8251BF18..0x8251BFC8)
	// 8251BF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251BF20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251BF24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251BF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BF2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251BF30: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8251BF34: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8251BF38: 409A000C  bne cr6, 0x8251bf44
	if !ctx.cr[6].eq {
	pc = 0x8251BF44; continue 'dispatch;
	}
	// 8251BF3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251BF40: 48000070  b 0x8251bfb0
	pc = 0x8251BFB0; continue 'dispatch;
	// 8251BF44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BF48: 4BFCEB41  bl 0x824eaa88
	ctx.lr = 0x8251BF4C;
	sub_824EAA88(ctx, base);
	// 8251BF4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251BF50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BF54: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BF58: 4BFCF201  bl 0x824eb158
	ctx.lr = 0x8251BF5C;
	sub_824EB158(ctx, base);
	// 8251BF5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BF60: 488D5D31  bl 0x82df1c90
	ctx.lr = 0x8251BF64;
	sub_82DF1C90(ctx, base);
	// 8251BF64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251BF68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251BF6C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251BF70: 409A0008  bne cr6, 0x8251bf78
	if !ctx.cr[6].eq {
	pc = 0x8251BF78; continue 'dispatch;
	}
	// 8251BF74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251BF78: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8251BF7C: 4BFE3E85  bl 0x824ffe00
	ctx.lr = 0x8251BF80;
	sub_824FFE00(ctx, base);
	// 8251BF80: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251BF84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BF88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251BF8C: 419A0018  beq cr6, 0x8251bfa4
	if ctx.cr[6].eq {
	pc = 0x8251BFA4; continue 'dispatch;
	}
	// 8251BF90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251BF94: 4BFFFEF5  bl 0x8251be88
	ctx.lr = 0x8251BF98;
	sub_8251BE88(ctx, base);
	// 8251BF98: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8251BF9C: 409A0008  bne cr6, 0x8251bfa4
	if !ctx.cr[6].eq {
	pc = 0x8251BFA4; continue 'dispatch;
	}
	// 8251BFA0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251BFA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BFA8: 488D5CE9  bl 0x82df1c90
	ctx.lr = 0x8251BFAC;
	sub_82DF1C90(ctx, base);
	// 8251BFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251BFB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251BFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251BFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251BFBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251BFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251BFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BFC8 size=388
    let mut pc: u32 = 0x8251BFC8;
    'dispatch: loop {
        match pc {
            0x8251BFC8 => {
    //   block [0x8251BFC8..0x8251C14C)
	// 8251BFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BFCC: 48C8C1A1  bl 0x831a816c
	ctx.lr = 0x8251BFD0;
	sub_831A8130(ctx, base);
	// 8251BFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BFD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251BFD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BFDC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251BFE0: 4BFCEAA9  bl 0x824eaa88
	ctx.lr = 0x8251BFE4;
	sub_824EAA88(ctx, base);
	// 8251BFE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251BFE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251BFEC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BFF0: 4BFCF169  bl 0x824eb158
	ctx.lr = 0x8251BFF4;
	sub_824EB158(ctx, base);
	// 8251BFF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251BFF8: 488D5C99  bl 0x82df1c90
	ctx.lr = 0x8251BFFC;
	sub_82DF1C90(ctx, base);
	// 8251BFFC: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8251C000: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C004: 4800012C  b 0x8251c130
	pc = 0x8251C130; continue 'dispatch;
	// 8251C008: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C00C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C010: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251C014: 419800E4  blt cr6, 0x8251c0f8
	if ctx.cr[6].lt {
	pc = 0x8251C0F8; continue 'dispatch;
	}
	// 8251C018: 419A00B4  beq cr6, 0x8251c0cc
	if ctx.cr[6].eq {
	pc = 0x8251C0CC; continue 'dispatch;
	}
	// 8251C01C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8251C020: 41980090  blt cr6, 0x8251c0b0
	if ctx.cr[6].lt {
	pc = 0x8251C0B0; continue 'dispatch;
	}
	// 8251C024: 419A005C  beq cr6, 0x8251c080
	if ctx.cr[6].eq {
	pc = 0x8251C080; continue 'dispatch;
	}
	// 8251C028: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8251C02C: 41980038  blt cr6, 0x8251c064
	if ctx.cr[6].lt {
	pc = 0x8251C064; continue 'dispatch;
	}
	// 8251C030: 409A00F4  bne cr6, 0x8251c124
	if !ctx.cr[6].eq {
	pc = 0x8251C124; continue 'dispatch;
	}
	// 8251C034: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C038: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C03C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C040: 409A0008  bne cr6, 0x8251c048
	if !ctx.cr[6].eq {
	pc = 0x8251C048; continue 'dispatch;
	}
	// 8251C044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C048: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251C04C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C050: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8251C054: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 8251C058: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8251C05C: 4BFE960D  bl 0x82505668
	ctx.lr = 0x8251C060;
	sub_82505668(ctx, base);
	// 8251C060: 480000C0  b 0x8251c120
	pc = 0x8251C120; continue 'dispatch;
	// 8251C064: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C06C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C070: 409A0008  bne cr6, 0x8251c078
	if !ctx.cr[6].eq {
	pc = 0x8251C078; continue 'dispatch;
	}
	// 8251C074: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C078: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251C07C: 4BFFFFD0  b 0x8251c04c
	pc = 0x8251C04C; continue 'dispatch;
	// 8251C080: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C084: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C088: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C08C: 409A0008  bne cr6, 0x8251c094
	if !ctx.cr[6].eq {
	pc = 0x8251C094; continue 'dispatch;
	}
	// 8251C090: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C094: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251C098: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C09C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8251C0A0: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 8251C0A4: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8251C0A8: 4BFE9421  bl 0x825054c8
	ctx.lr = 0x8251C0AC;
	sub_825054C8(ctx, base);
	// 8251C0AC: 48000074  b 0x8251c120
	pc = 0x8251C120; continue 'dispatch;
	// 8251C0B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C0B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C0B8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C0BC: 409A0008  bne cr6, 0x8251c0c4
	if !ctx.cr[6].eq {
	pc = 0x8251C0C4; continue 'dispatch;
	}
	// 8251C0C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251C0C8: 4BFFFFD0  b 0x8251c098
	pc = 0x8251C098; continue 'dispatch;
	// 8251C0CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C0D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C0D4: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C0D8: 409A0008  bne cr6, 0x8251c0e0
	if !ctx.cr[6].eq {
	pc = 0x8251C0E0; continue 'dispatch;
	}
	// 8251C0DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C0E0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C0E4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8251C0E8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 8251C0EC: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8251C0F0: 4BFE91F1  bl 0x825052e0
	ctx.lr = 0x8251C0F4;
	sub_825052E0(ctx, base);
	// 8251C0F4: 4800002C  b 0x8251c120
	pc = 0x8251C120; continue 'dispatch;
	// 8251C0F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C0FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C100: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251C104: 409A0008  bne cr6, 0x8251c10c
	if !ctx.cr[6].eq {
	pc = 0x8251C10C; continue 'dispatch;
	}
	// 8251C108: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C10C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C110: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8251C114: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 8251C118: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8251C11C: 4BFE8FDD  bl 0x825050f8
	ctx.lr = 0x8251C120;
	sub_825050F8(ctx, base);
	// 8251C120: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251C124: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251C128: 41820010  beq 0x8251c138
	if ctx.cr[0].eq {
	pc = 0x8251C138; continue 'dispatch;
	}
	// 8251C12C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C130: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8251C134: 409AFED4  bne cr6, 0x8251c008
	if !ctx.cr[6].eq {
	pc = 0x8251C008; continue 'dispatch;
	}
	// 8251C138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C13C: 488D5B55  bl 0x82df1c90
	ctx.lr = 0x8251C140;
	sub_82DF1C90(ctx, base);
	// 8251C140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251C148: 48C8C074  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C150 size=84
    let mut pc: u32 = 0x8251C150;
    'dispatch: loop {
        match pc {
            0x8251C150 => {
    //   block [0x8251C150..0x8251C1A4)
	// 8251C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251C15C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C164: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C168: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C16C: 4BFFFC7D  bl 0x8251bde8
	ctx.lr = 0x8251C170;
	sub_8251BDE8(ctx, base);
	// 8251C170: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251C178: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251C17C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251C180: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C184: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251C188: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C18C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251C190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251C194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C19C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251C1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251C1A8 size=44
    let mut pc: u32 = 0x8251C1A8;
    'dispatch: loop {
        match pc {
            0x8251C1A8 => {
    //   block [0x8251C1A8..0x8251C1D4)
	// 8251C1A8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C1AC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C1B0: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C1B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8251C1B8: 409A0030  bne cr6, 0x8251c1e8
	if !ctx.cr[6].eq {
		sub_8251C1D4(ctx, base);
		return;
	}
	// 8251C1BC: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C1C0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251C1C4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251C1C8: 4098000C  bge cr6, 0x8251c1d4
	if !ctx.cr[6].lt {
		sub_8251C1D4(ctx, base);
		return;
	}
	// 8251C1CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C1D0: 4800000C  b 0x8251c1dc
	sub_8251C1D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C1D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251C1D4 size=60
    let mut pc: u32 = 0x8251C1D4;
    'dispatch: loop {
        match pc {
            0x8251C1D4 => {
    //   block [0x8251C1D4..0x8251C210)
	// 8251C1D4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8251C1D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C1DC: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C1E0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8251C1E4: 419AFFDC  beq cr6, 0x8251c1c0
	if ctx.cr[6].eq {
		sub_8251C1A8(ctx, base);
		return;
	}
	// 8251C1E8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C1EC: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 8251C1F0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C1F4: 419A001C  beq cr6, 0x8251c210
	if ctx.cr[6].eq {
		sub_8251C210(ctx, base);
		return;
	}
	// 8251C1F8: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C1FC: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251C200: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251C204: 4198000C  blt cr6, 0x8251c210
	if ctx.cr[6].lt {
		sub_8251C210(ctx, base);
		return;
	}
	// 8251C208: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8251C20C: 4800000C  b 0x8251c218
	sub_8251C210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251C210 size=20
    let mut pc: u32 = 0x8251C210;
    'dispatch: loop {
        match pc {
            0x8251C210 => {
    //   block [0x8251C210..0x8251C224)
	// 8251C210: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 8251C214: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 8251C218: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C21C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251C220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C228 size=204
    let mut pc: u32 = 0x8251C228;
    'dispatch: loop {
        match pc {
            0x8251C228 => {
    //   block [0x8251C228..0x8251C2F4)
	// 8251C228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251C234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251C238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C23C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251C240: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251C244: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251C248: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251C24C: 40990080  ble cr6, 0x8251c2cc
	if !ctx.cr[6].gt {
	pc = 0x8251C2CC; continue 'dispatch;
	}
	// 8251C250: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8251C254: 40980078  bge cr6, 0x8251c2cc
	if !ctx.cr[6].lt {
	pc = 0x8251C2CC; continue 'dispatch;
	}
	// 8251C258: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8251C25C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C260: 488D79A1  bl 0x82df3c00
	ctx.lr = 0x8251C264;
	sub_82DF3C00(ctx, base);
	// 8251C264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251C268: 4BFFFC21  bl 0x8251be88
	ctx.lr = 0x8251C26C;
	sub_8251BE88(ctx, base);
	// 8251C26C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8251C270: 419A0018  beq cr6, 0x8251c288
	if ctx.cr[6].eq {
	pc = 0x8251C288; continue 'dispatch;
	}
	// 8251C274: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8251C278: 409A003C  bne cr6, 0x8251c2b4
	if !ctx.cr[6].eq {
	pc = 0x8251C2B4; continue 'dispatch;
	}
	// 8251C27C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251C280: 3BEA2570  addi r31, r10, 0x2570
	ctx.r[31].s64 = ctx.r[10].s64 + 9584;
	// 8251C284: 4800000C  b 0x8251c290
	pc = 0x8251C290; continue 'dispatch;
	// 8251C288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251C28C: 3BEA256C  addi r31, r10, 0x256c
	ctx.r[31].s64 = ctx.r[10].s64 + 9580;
	// 8251C290: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251C294: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C298: 388B2568  addi r4, r11, 0x2568
	ctx.r[4].s64 = ctx.r[11].s64 + 9576;
	// 8251C29C: 488D7D2D  bl 0x82df3fc8
	ctx.lr = 0x8251C2A0;
	sub_82DF3FC8(ctx, base);
	// 8251C2A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251C2A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8251C2A8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8251C2AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C2B0: 488D7F11  bl 0x82df41c0
	ctx.lr = 0x8251C2B4;
	sub_82DF41C0(ctx, base);
	// 8251C2B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251C2B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C2BC: 488D7945  bl 0x82df3c00
	ctx.lr = 0x8251C2C0;
	sub_82DF3C00(ctx, base);
	// 8251C2C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C2C4: 488D7165  bl 0x82df3428
	ctx.lr = 0x8251C2C8;
	sub_82DF3428(ctx, base);
	// 8251C2C8: 48000010  b 0x8251c2d8
	pc = 0x8251C2D8; continue 'dispatch;
	// 8251C2CC: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8251C2D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C2D4: 488D792D  bl 0x82df3c00
	ctx.lr = 0x8251C2D8;
	sub_82DF3C00(ctx, base);
	// 8251C2D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C2DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251C2E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C2E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C2E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251C2EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251C2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C2F8 size=88
    let mut pc: u32 = 0x8251C2F8;
    'dispatch: loop {
        match pc {
            0x8251C2F8 => {
    //   block [0x8251C2F8..0x8251C350)
	// 8251C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251C304: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C308: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 8251C30C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C310: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 8251C314: 389F0124  addi r4, r31, 0x124
	ctx.r[4].s64 = ctx.r[31].s64 + 292;
	// 8251C318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C31C: 4BFFFE8D  bl 0x8251c1a8
	ctx.lr = 0x8251C320;
	sub_8251C1A8(ctx, base);
	// 8251C320: 815F0128  lwz r10, 0x128(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 8251C324: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251C328: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251C32C: 419A000C  beq cr6, 0x8251c338
	if ctx.cr[6].eq {
	pc = 0x8251C338; continue 'dispatch;
	}
	// 8251C330: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251C334: 48000008  b 0x8251c33c
	pc = 0x8251C33C; continue 'dispatch;
	// 8251C338: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251C33C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251C340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251C34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251C350 size=24
    let mut pc: u32 = 0x8251C350;
    'dispatch: loop {
        match pc {
            0x8251C350 => {
    //   block [0x8251C350..0x8251C368)
	// 8251C350: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251C354: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C358: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C35C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251C364: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C368 size=124
    let mut pc: u32 = 0x8251C368;
    'dispatch: loop {
        match pc {
            0x8251C368 => {
    //   block [0x8251C368..0x8251C3E4)
	// 8251C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C36C: 48C8BDFD  bl 0x831a8168
	ctx.lr = 0x8251C370;
	sub_831A8130(ctx, base);
	// 8251C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C378: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8251C37C: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C380: 48000024  b 0x8251c3a4
	pc = 0x8251C3A4; continue 'dispatch;
	// 8251C384: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C388: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8251C38C: 419A0014  beq cr6, 0x8251c3a0
	if ctx.cr[6].eq {
	pc = 0x8251C3A0; continue 'dispatch;
	}
	// 8251C390: 387C000C  addi r3, r28, 0xc
	ctx.r[3].s64 = ctx.r[28].s64 + 12;
	// 8251C394: 488D7095  bl 0x82df3428
	ctx.lr = 0x8251C398;
	sub_82DF3428(ctx, base);
	// 8251C398: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251C39C: 4BDA3ECD  bl 0x822c0268
	ctx.lr = 0x8251C3A0;
	sub_822C0268(ctx, base);
	// 8251C3A0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C3A4: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8251C3A8: 409AFFDC  bne cr6, 0x8251c384
	if !ctx.cr[6].eq {
	pc = 0x8251C384; continue 'dispatch;
	}
	// 8251C3AC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8251C3B0: 4BF0C2D1  bl 0x82428680
	ctx.lr = 0x8251C3B4;
	sub_82428680(ctx, base);
	// 8251C3B4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251C3B8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8251C3BC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251C3C0: 488D5DC9  bl 0x82df2188
	ctx.lr = 0x8251C3C4;
	sub_82DF2188(ctx, base);
	// 8251C3C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251C3C8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8251C3CC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8251C3D0: 488D7059  bl 0x82df3428
	ctx.lr = 0x8251C3D4;
	sub_82DF3428(ctx, base);
	// 8251C3D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251C3D8: 488D7051  bl 0x82df3428
	ctx.lr = 0x8251C3DC;
	sub_82DF3428(ctx, base);
	// 8251C3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251C3E0: 48C8BDD8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C3E8 size=124
    let mut pc: u32 = 0x8251C3E8;
    'dispatch: loop {
        match pc {
            0x8251C3E8 => {
    //   block [0x8251C3E8..0x8251C464)
	// 8251C3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C3EC: 48C8BD7D  bl 0x831a8168
	ctx.lr = 0x8251C3F0;
	sub_831A8130(ctx, base);
	// 8251C3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C3F8: 488D6CF9  bl 0x82df30f0
	ctx.lr = 0x8251C3FC;
	sub_82DF30F0(ctx, base);
	// 8251C3FC: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 8251C400: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251C404: 488D6CED  bl 0x82df30f0
	ctx.lr = 0x8251C408;
	sub_82DF30F0(ctx, base);
	// 8251C408: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8251C40C: 4BF4A0CD  bl 0x824664d8
	ctx.lr = 0x8251C410;
	sub_824664D8(ctx, base);
	// 8251C410: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251C414: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8251C418: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251C41C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8251C420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251C424: 3B8B9BC9  addi r28, r11, -0x6437
	ctx.r[28].s64 = ctx.r[11].s64 + -25655;
	// 8251C428: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251C42C: 488D744D  bl 0x82df3878
	ctx.lr = 0x8251C430;
	sub_82DF3878(ctx, base);
	// 8251C430: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251C434: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251C438: 488D7441  bl 0x82df3878
	ctx.lr = 0x8251C43C;
	sub_82DF3878(ctx, base);
	// 8251C43C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8251C440: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8251C444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251C448: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 8251C44C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8251C450: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8251C454: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8251C458: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8251C45C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251C460: 48C8BD58  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C468 size=548
    let mut pc: u32 = 0x8251C468;
    'dispatch: loop {
        match pc {
            0x8251C468 => {
    //   block [0x8251C468..0x8251C68C)
	// 8251C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C46C: 48C8BCF5  bl 0x831a8160
	ctx.lr = 0x8251C470;
	sub_831A8130(ctx, base);
	// 8251C470: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C474: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251C478: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 8251C47C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8251C480: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 8251C484: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251C488: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C48C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251C490: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8251C494: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C498: 41980048  blt cr6, 0x8251c4e0
	if ctx.cr[6].lt {
	pc = 0x8251C4E0; continue 'dispatch;
	}
	// 8251C49C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251C4A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C4A4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 8251C4A8: 4BDA9421  bl 0x822c58c8
	ctx.lr = 0x8251C4AC;
	sub_822C58C8(ctx, base);
	// 8251C4AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251C4B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251C4B4: 4BDA9365  bl 0x822c5818
	ctx.lr = 0x8251C4B8;
	sub_822C5818(ctx, base);
	// 8251C4B8: 4BDA7DF9  bl 0x822c42b0
	ctx.lr = 0x8251C4BC;
	sub_822C42B0(ctx, base);
	// 8251C4BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251C4C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251C4C4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 8251C4C8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8251C4CC: 4BDA8FA5  bl 0x822c5470
	ctx.lr = 0x8251C4D0;
	sub_822C5470(ctx, base);
	// 8251C4D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8251C4D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251C4D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C4DC: 4BDA8805  bl 0x822c4ce0
	ctx.lr = 0x8251C4E0;
	sub_822C4CE0(ctx, base);
	// 8251C4E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8251C4E8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8251C4EC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8251C4F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251C4F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C4F8: 4BDD20E9  bl 0x822ee5e0
	ctx.lr = 0x8251C4FC;
	sub_822EE5E0(ctx, base);
	// 8251C4FC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C500: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C504: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251C508: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251C50C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C510: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251C514: 409A0018  bne cr6, 0x8251c52c
	if !ctx.cr[6].eq {
	pc = 0x8251C52C; continue 'dispatch;
	}
	// 8251C518: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8251C51C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C520: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C524: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C528: 4800003C  b 0x8251c564
	pc = 0x8251C564; continue 'dispatch;
	// 8251C52C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251C530: 41820020  beq 0x8251c550
	if ctx.cr[0].eq {
	pc = 0x8251C550; continue 'dispatch;
	}
	// 8251C534: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C538: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C53C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C540: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251C544: 409A0024  bne cr6, 0x8251c568
	if !ctx.cr[6].eq {
	pc = 0x8251C568; continue 'dispatch;
	}
	// 8251C548: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C54C: 4800001C  b 0x8251c568
	pc = 0x8251C568; continue 'dispatch;
	// 8251C550: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8251C554: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C558: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C55C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251C560: 409A0008  bne cr6, 0x8251c568
	if !ctx.cr[6].eq {
	pc = 0x8251C568; continue 'dispatch;
	}
	// 8251C564: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8251C568: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C56C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8251C570: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8251C574: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8251C578: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C57C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C580: 409A00F0  bne cr6, 0x8251c670
	if !ctx.cr[6].eq {
	pc = 0x8251C670; continue 'dispatch;
	}
	// 8251C584: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8251C588: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C58C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C590: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C594: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251C598: 409A0054  bne cr6, 0x8251c5ec
	if !ctx.cr[6].eq {
	pc = 0x8251C5EC; continue 'dispatch;
	}
	// 8251C59C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C5A0: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C5A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8251C5A8: 419A0054  beq cr6, 0x8251c5fc
	if ctx.cr[6].eq {
	pc = 0x8251C5FC; continue 'dispatch;
	}
	// 8251C5AC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C5B0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C5B4: 409A0010  bne cr6, 0x8251c5c4
	if !ctx.cr[6].eq {
	pc = 0x8251C5C4; continue 'dispatch;
	}
	// 8251C5B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C5BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251C5C0: 4869A611  bl 0x82bb6bd0
	ctx.lr = 0x8251C5C4;
	sub_82BB6BD0(ctx, base);
	// 8251C5C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C5C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C5CC: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C5D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C5D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C5D8: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8251C5DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C5E0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C5E4: 4BDD1F2D  bl 0x822ee510
	ctx.lr = 0x8251C5E8;
	sub_822EE510(ctx, base);
	// 8251C5E8: 48000074  b 0x8251c65c
	pc = 0x8251C65C; continue 'dispatch;
	// 8251C5EC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C5F0: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C5F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8251C5F8: 409A0028  bne cr6, 0x8251c620
	if !ctx.cr[6].eq {
	pc = 0x8251C620; continue 'dispatch;
	}
	// 8251C5FC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C600: 9BA90014  stb r29, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C604: 9BAA0014  stb r29, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C608: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C60C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C610: 9B6A0014  stb r27, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8251C614: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C618: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C61C: 48000040  b 0x8251c65c
	pc = 0x8251C65C; continue 'dispatch;
	// 8251C620: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C624: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C628: 409A0010  bne cr6, 0x8251c638
	if !ctx.cr[6].eq {
	pc = 0x8251C638; continue 'dispatch;
	}
	// 8251C62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C630: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251C634: 4BDD1EDD  bl 0x822ee510
	ctx.lr = 0x8251C638;
	sub_822EE510(ctx, base);
	// 8251C638: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C63C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251C640: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C644: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C648: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C64C: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 8251C650: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C654: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C658: 4869A579  bl 0x82bb6bd0
	ctx.lr = 0x8251C65C;
	sub_82BB6BD0(ctx, base);
	// 8251C65C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C660: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8251C664: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C668: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C66C: 419AFF1C  beq cr6, 0x8251c588
	if ctx.cr[6].eq {
	pc = 0x8251C588; continue 'dispatch;
	}
	// 8251C670: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C674: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8251C678: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C67C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C680: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C684: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8251C688: 48C8BB28  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C690 size=1016
    let mut pc: u32 = 0x8251C690;
    'dispatch: loop {
        match pc {
            0x8251C690 => {
    //   block [0x8251C690..0x8251CA88)
	// 8251C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C694: 48C8BAC5  bl 0x831a8158
	ctx.lr = 0x8251C698;
	sub_831A8130(ctx, base);
	// 8251C698: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C69C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8251C6A0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8251C6A4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8251C6A8: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8251C6AC: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C6B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C6B4: 419A0048  beq cr6, 0x8251c6fc
	if ctx.cr[6].eq {
	pc = 0x8251C6FC; continue 'dispatch;
	}
	// 8251C6B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251C6BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C6C0: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8251C6C4: 4BDA9205  bl 0x822c58c8
	ctx.lr = 0x8251C6C8;
	sub_822C58C8(ctx, base);
	// 8251C6C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251C6CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251C6D0: 4BDAD7E1  bl 0x822c9eb0
	ctx.lr = 0x8251C6D4;
	sub_822C9EB0(ctx, base);
	// 8251C6D4: 4BDA7BDD  bl 0x822c42b0
	ctx.lr = 0x8251C6D8;
	sub_822C42B0(ctx, base);
	// 8251C6D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251C6DC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251C6E0: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 8251C6E4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8251C6E8: 4BDA8D89  bl 0x822c5470
	ctx.lr = 0x8251C6EC;
	sub_822C5470(ctx, base);
	// 8251C6EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8251C6F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251C6F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251C6F8: 4BDA85E9  bl 0x822c4ce0
	ctx.lr = 0x8251C6FC;
	sub_822C4CE0(ctx, base);
	// 8251C6FC: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8251C700: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8251C704: 4BFCC98D  bl 0x824e9090
	ctx.lr = 0x8251C708;
	sub_824E9090(ctx, base);
	// 8251C708: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C70C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C710: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C714: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8251C718: 419A000C  beq cr6, 0x8251c724
	if ctx.cr[6].eq {
	pc = 0x8251C724; continue 'dispatch;
	}
	// 8251C71C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C720: 48000028  b 0x8251c748
	pc = 0x8251C748; continue 'dispatch;
	// 8251C724: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C728: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C72C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C730: 419A000C  beq cr6, 0x8251c73c
	if ctx.cr[6].eq {
	pc = 0x8251C73C; continue 'dispatch;
	}
	// 8251C734: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8251C738: 48000010  b 0x8251c748
	pc = 0x8251C748; continue 'dispatch;
	// 8251C73C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C740: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C744: 409A00DC  bne cr6, 0x8251c820
	if !ctx.cr[6].eq {
	pc = 0x8251C820; continue 'dispatch;
	}
	// 8251C748: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C74C: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C754: 409A0008  bne cr6, 0x8251c75c
	if !ctx.cr[6].eq {
	pc = 0x8251C75C; continue 'dispatch;
	}
	// 8251C758: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8251C75C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C760: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C764: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C768: 409A000C  bne cr6, 0x8251c774
	if !ctx.cr[6].eq {
	pc = 0x8251C774; continue 'dispatch;
	}
	// 8251C76C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8251C770: 4800001C  b 0x8251c78c
	pc = 0x8251C78C; continue 'dispatch;
	// 8251C774: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C778: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C77C: 409A000C  bne cr6, 0x8251c788
	if !ctx.cr[6].eq {
	pc = 0x8251C788; continue 'dispatch;
	}
	// 8251C780: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C784: 48000008  b 0x8251c78c
	pc = 0x8251C78C; continue 'dispatch;
	// 8251C788: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8251C78C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C790: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C794: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C798: 409A003C  bne cr6, 0x8251c7d4
	if !ctx.cr[6].eq {
	pc = 0x8251C7D4; continue 'dispatch;
	}
	// 8251C79C: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C7A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C7A4: 419A000C  beq cr6, 0x8251c7b0
	if ctx.cr[6].eq {
	pc = 0x8251C7B0; continue 'dispatch;
	}
	// 8251C7A8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8251C7AC: 48000024  b 0x8251c7d0
	pc = 0x8251C7D0; continue 'dispatch;
	// 8251C7B0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C7B4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8251C7B8: 4800000C  b 0x8251c7c4
	pc = 0x8251C7C4; continue 'dispatch;
	// 8251C7BC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8251C7C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C7C4: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C7C8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8251C7CC: 419AFFF0  beq cr6, 0x8251c7bc
	if ctx.cr[6].eq {
	pc = 0x8251C7BC; continue 'dispatch;
	}
	// 8251C7D0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251C7D4: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C7D8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C7DC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C7E0: 409A00D4  bne cr6, 0x8251c8b4
	if !ctx.cr[6].eq {
	pc = 0x8251C8B4; continue 'dispatch;
	}
	// 8251C7E4: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C7E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C7EC: 419A000C  beq cr6, 0x8251c7f8
	if ctx.cr[6].eq {
	pc = 0x8251C7F8; continue 'dispatch;
	}
	// 8251C7F0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8251C7F4: 48000024  b 0x8251c818
	pc = 0x8251C818; continue 'dispatch;
	// 8251C7F8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C7FC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8251C800: 4800000C  b 0x8251c80c
	pc = 0x8251C80C; continue 'dispatch;
	// 8251C804: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8251C808: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C80C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C810: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8251C814: 419AFFF0  beq cr6, 0x8251c804
	if ctx.cr[6].eq {
	pc = 0x8251C804; continue 'dispatch;
	}
	// 8251C818: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251C81C: 48000098  b 0x8251c8b4
	pc = 0x8251C8B4; continue 'dispatch;
	// 8251C820: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8251C824: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C828: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251C82C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C830: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C834: 409A000C  bne cr6, 0x8251c840
	if !ctx.cr[6].eq {
	pc = 0x8251C840; continue 'dispatch;
	}
	// 8251C838: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8251C83C: 4800002C  b 0x8251c868
	pc = 0x8251C868; continue 'dispatch;
	// 8251C840: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C844: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251C84C: 409A0008  bne cr6, 0x8251c854
	if !ctx.cr[6].eq {
	pc = 0x8251C854; continue 'dispatch;
	}
	// 8251C850: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8251C854: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8251C858: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C85C: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251C860: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C864: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8251C868: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C86C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C870: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C874: 409A000C  bne cr6, 0x8251c880
	if !ctx.cr[6].eq {
	pc = 0x8251C880; continue 'dispatch;
	}
	// 8251C878: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8251C87C: 48000020  b 0x8251c89c
	pc = 0x8251C89C; continue 'dispatch;
	// 8251C880: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C884: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C888: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8251C88C: 409A000C  bne cr6, 0x8251c898
	if !ctx.cr[6].eq {
	pc = 0x8251C898; continue 'dispatch;
	}
	// 8251C890: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8251C894: 48000008  b 0x8251c89c
	pc = 0x8251C89C; continue 'dispatch;
	// 8251C898: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8251C89C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C8A0: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251C8A4: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C8A8: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C8AC: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8251C8B0: 995A0014  stb r10, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8251C8B4: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C8B8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251C8BC: 409A0198  bne cr6, 0x8251ca54
	if !ctx.cr[6].eq {
	pc = 0x8251CA54; continue 'dispatch;
	}
	// 8251C8C0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C8C4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251C8C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C8CC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C8D0: 419A0180  beq cr6, 0x8251ca50
	if ctx.cr[6].eq {
	pc = 0x8251CA50; continue 'dispatch;
	}
	// 8251C8D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8251C8D8: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C8DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8251C8E0: 409A0170  bne cr6, 0x8251ca50
	if !ctx.cr[6].eq {
	pc = 0x8251CA50; continue 'dispatch;
	}
	// 8251C8E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C8E8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C8EC: 409A00A8  bne cr6, 0x8251c994
	if !ctx.cr[6].eq {
	pc = 0x8251C994; continue 'dispatch;
	}
	// 8251C8F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C8F4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C8F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C8FC: 409A001C  bne cr6, 0x8251c918
	if !ctx.cr[6].eq {
	pc = 0x8251C918; continue 'dispatch;
	}
	// 8251C900: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251C904: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251C908: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C90C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251C910: 4869A2C1  bl 0x82bb6bd0
	ctx.lr = 0x8251C914;
	sub_82BB6BD0(ctx, base);
	// 8251C914: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C918: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C91C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C920: 409A00C8  bne cr6, 0x8251c9e8
	if !ctx.cr[6].eq {
	pc = 0x8251C9E8; continue 'dispatch;
	}
	// 8251C924: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C928: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C92C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251C930: 409A0014  bne cr6, 0x8251c944
	if !ctx.cr[6].eq {
	pc = 0x8251C944; continue 'dispatch;
	}
	// 8251C934: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C938: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C93C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251C940: 419A00A4  beq cr6, 0x8251c9e4
	if ctx.cr[6].eq {
	pc = 0x8251C9E4; continue 'dispatch;
	}
	// 8251C944: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C948: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C94C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251C950: 409A0020  bne cr6, 0x8251c970
	if !ctx.cr[6].eq {
	pc = 0x8251C970; continue 'dispatch;
	}
	// 8251C954: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C958: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251C95C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251C960: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251C964: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C968: 4BDD1BA9  bl 0x822ee510
	ctx.lr = 0x8251C96C;
	sub_822EE510(ctx, base);
	// 8251C96C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C970: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C974: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251C978: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251C97C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8251C980: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251C984: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C988: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251C98C: 4869A245  bl 0x82bb6bd0
	ctx.lr = 0x8251C990;
	sub_82BB6BD0(ctx, base);
	// 8251C990: 480000C0  b 0x8251ca50
	pc = 0x8251CA50; continue 'dispatch;
	// 8251C994: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C998: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C99C: 409A001C  bne cr6, 0x8251c9b8
	if !ctx.cr[6].eq {
	pc = 0x8251C9B8; continue 'dispatch;
	}
	// 8251C9A0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251C9A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251C9A8: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C9AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251C9B0: 4BDD1B61  bl 0x822ee510
	ctx.lr = 0x8251C9B4;
	sub_822EE510(ctx, base);
	// 8251C9B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C9B8: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251C9BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251C9C0: 409A0028  bne cr6, 0x8251c9e8
	if !ctx.cr[6].eq {
	pc = 0x8251C9E8; continue 'dispatch;
	}
	// 8251C9C4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C9C8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C9CC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251C9D0: 409A0034  bne cr6, 0x8251ca04
	if !ctx.cr[6].eq {
	pc = 0x8251CA04; continue 'dispatch;
	}
	// 8251C9D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C9D8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251C9DC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251C9E0: 409A0024  bne cr6, 0x8251ca04
	if !ctx.cr[6].eq {
	pc = 0x8251CA04; continue 'dispatch;
	}
	// 8251C9E4: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251C9E8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C9EC: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8251C9F0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C9F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C9F8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251C9FC: 409AFEDC  bne cr6, 0x8251c8d8
	if !ctx.cr[6].eq {
	pc = 0x8251C8D8; continue 'dispatch;
	}
	// 8251CA00: 48000050  b 0x8251ca50
	pc = 0x8251CA50; continue 'dispatch;
	// 8251CA04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CA08: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251CA0C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8251CA10: 409A0020  bne cr6, 0x8251ca30
	if !ctx.cr[6].eq {
	pc = 0x8251CA30; continue 'dispatch;
	}
	// 8251CA14: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CA18: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251CA1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251CA20: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251CA24: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 8251CA28: 4869A1A9  bl 0x82bb6bd0
	ctx.lr = 0x8251CA2C;
	sub_82BB6BD0(ctx, base);
	// 8251CA2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CA30: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251CA34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251CA38: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8251CA3C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8251CA40: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251CA44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CA48: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251CA4C: 4BDD1AC5  bl 0x822ee510
	ctx.lr = 0x8251CA50;
	sub_822EE510(ctx, base);
	// 8251CA50: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 8251CA54: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251CA58: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8251CA5C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251CA60: 488D5729  bl 0x82df2188
	ctx.lr = 0x8251CA64;
	sub_82DF2188(ctx, base);
	// 8251CA64: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251CA6C: 419A000C  beq cr6, 0x8251ca78
	if ctx.cr[6].eq {
	pc = 0x8251CA78; continue 'dispatch;
	}
	// 8251CA70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8251CA74: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251CA78: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8251CA7C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8251CA80: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8251CA84: 48C8B724  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CA88 size=188
    let mut pc: u32 = 0x8251CA88;
    'dispatch: loop {
        match pc {
            0x8251CA88 => {
    //   block [0x8251CA88..0x8251CB44)
	// 8251CA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251CA90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251CA94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251CA98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CA9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251CAA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251CAA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251CAAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CAB0: 4BDA3E89  bl 0x822c0938
	ctx.lr = 0x8251CAB4;
	sub_822C0938(ctx, base);
	// 8251CAB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251CAB8: 41820028  beq 0x8251cae0
	if ctx.cr[0].eq {
	pc = 0x8251CAE0; continue 'dispatch;
	}
	// 8251CABC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CAC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251CAC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251CAC8: 392B2558  addi r9, r11, 0x2558
	ctx.r[9].s64 = ctx.r[11].s64 + 9560;
	// 8251CACC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251CAD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251CAD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251CAD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251CADC: 48000008  b 0x8251cae4
	pc = 0x8251CAE4; continue 'dispatch;
	// 8251CAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251CAE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CAE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251CAEC: 409A003C  bne cr6, 0x8251cb28
	if !ctx.cr[6].eq {
	pc = 0x8251CB28; continue 'dispatch;
	}
	// 8251CAF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251CAF4: 419A0014  beq cr6, 0x8251cb08
	if ctx.cr[6].eq {
	pc = 0x8251CB08; continue 'dispatch;
	}
	// 8251CAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CAFC: 4BFFF86D  bl 0x8251c368
	ctx.lr = 0x8251CB00;
	sub_8251C368(ctx, base);
	// 8251CB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CB04: 4BDA3765  bl 0x822c0268
	ctx.lr = 0x8251CB08;
	sub_822C0268(ctx, base);
	// 8251CB08: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251CB0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251CB10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CB14: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251CB18: 816BD22C  lwz r11, -0x2dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11732 as u32) ) } as u64;
	// 8251CB1C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251CB20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251CB24: 4BDA34DD  bl 0x822c0000
	ctx.lr = 0x8251CB28;
	sub_822C0000(ctx, base);
	// 8251CB28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251CB2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251CB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251CB38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251CB3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251CB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CB48 size=64
    let mut pc: u32 = 0x8251CB48;
    'dispatch: loop {
        match pc {
            0x8251CB48 => {
    //   block [0x8251CB48..0x8251CB88)
	// 8251CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251CB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251CB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CB58: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251CB5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251CB60: 419A0014  beq cr6, 0x8251cb74
	if ctx.cr[6].eq {
	pc = 0x8251CB74; continue 'dispatch;
	}
	// 8251CB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CB68: 4BFFF801  bl 0x8251c368
	ctx.lr = 0x8251CB6C;
	sub_8251C368(ctx, base);
	// 8251CB6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CB70: 4BDA36F9  bl 0x822c0268
	ctx.lr = 0x8251CB74;
	sub_822C0268(ctx, base);
	// 8251CB74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251CB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251CB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251CB80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251CB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251CB88 size=236
    let mut pc: u32 = 0x8251CB88;
    'dispatch: loop {
        match pc {
            0x8251CB88 => {
    //   block [0x8251CB88..0x8251CC74)
	// 8251CB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CB8C: 48C8B5D5  bl 0x831a8160
	ctx.lr = 0x8251CB90;
	sub_831A8130(ctx, base);
	// 8251CB90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CB94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251CB98: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8251CB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251CBA0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251CBA4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8251CBA8: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251CBAC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251CBB0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251CBB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251CBB8: 409A0038  bne cr6, 0x8251cbf0
	if !ctx.cr[6].eq {
	pc = 0x8251CBF0; continue 'dispatch;
	}
	// 8251CBBC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CBC0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251CBC4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8251CBC8: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8251CBCC: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8251CBD0: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8251CBD4: 4182000C  beq 0x8251cbe0
	if ctx.cr[0].eq {
	pc = 0x8251CBE0; continue 'dispatch;
	}
	// 8251CBD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CBDC: 48000008  b 0x8251cbe4
	pc = 0x8251CBE4; continue 'dispatch;
	// 8251CBE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CBE4: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 8251CBE8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8251CBEC: 419AFFD4  beq cr6, 0x8251cbc0
	if ctx.cr[6].eq {
	pc = 0x8251CBC0; continue 'dispatch;
	}
	// 8251CBF0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8251CBF4: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251CBF8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251CBFC: 41820044  beq 0x8251cc40
	if ctx.cr[0].eq {
	pc = 0x8251CC40; continue 'dispatch;
	}
	// 8251CC00: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251CC04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CC08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC0C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251CC10: 409A0028  bne cr6, 0x8251cc38
	if !ctx.cr[6].eq {
	pc = 0x8251CC38; continue 'dispatch;
	}
	// 8251CC14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8251CC18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251CC1C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8251CC20: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8251CC24: 4BFFF845  bl 0x8251c468
	ctx.lr = 0x8251CC28;
	sub_8251C468(ctx, base);
	// 8251CC28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251CC2C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 8251CC30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC34: 48000030  b 0x8251cc64
	pc = 0x8251CC64; continue 'dispatch;
	// 8251CC38: 4869A001  bl 0x82bb6c38
	ctx.lr = 0x8251CC3C;
	sub_82BB6C38(ctx, base);
	// 8251CC3C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251CC40: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251CC44: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC48: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251CC4C: 40980010  bge cr6, 0x8251cc5c
	if !ctx.cr[6].lt {
	pc = 0x8251CC5C; continue 'dispatch;
	}
	// 8251CC50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251CC54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CC58: 4BFFFFC0  b 0x8251cc18
	pc = 0x8251CC18; continue 'dispatch;
	// 8251CC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251CC60: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 8251CC64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CC6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251CC70: 48C8B540  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CC78 size=132
    let mut pc: u32 = 0x8251CC78;
    'dispatch: loop {
        match pc {
            0x8251CC78 => {
    //   block [0x8251CC78..0x8251CCFC)
	// 8251CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CC7C: 48C8B4ED  bl 0x831a8168
	ctx.lr = 0x8251CC80;
	sub_831A8130(ctx, base);
	// 8251CC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CC84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251CC88: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8251CC8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251CC90: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8251CC94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251CC98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC9C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251CCA0: 409A0044  bne cr6, 0x8251cce4
	if !ctx.cr[6].eq {
	pc = 0x8251CCE4; continue 'dispatch;
	}
	// 8251CCA4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251CCA8: 409A003C  bne cr6, 0x8251cce4
	if !ctx.cr[6].eq {
	pc = 0x8251CCE4; continue 'dispatch;
	}
	// 8251CCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CCB0: 4BFFF4A1  bl 0x8251c150
	ctx.lr = 0x8251CCB4;
	sub_8251C150(ctx, base);
	// 8251CCB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251CCB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CCBC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CCC0: 48000030  b 0x8251ccf0
	pc = 0x8251CCF0; continue 'dispatch;
	// 8251CCC4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8251CCC8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8251CCCC: 4BFCC3C5  bl 0x824e9090
	ctx.lr = 0x8251CCD0;
	sub_824E9090(ctx, base);
	// 8251CCD0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251CCD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251CCD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CCDC: 4BFFF9B5  bl 0x8251c690
	ctx.lr = 0x8251CCE0;
	sub_8251C690(ctx, base);
	// 8251CCE0: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8251CCE4: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251CCE8: 409AFFDC  bne cr6, 0x8251ccc4
	if !ctx.cr[6].eq {
	pc = 0x8251CCC4; continue 'dispatch;
	}
	// 8251CCEC: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8251CCF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251CCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251CCF8: 48C8B4C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CD00 size=140
    let mut pc: u32 = 0x8251CD00;
    'dispatch: loop {
        match pc {
            0x8251CD00 => {
    //   block [0x8251CD00..0x8251CD8C)
	// 8251CD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CD04: 48C8B469  bl 0x831a816c
	ctx.lr = 0x8251CD08;
	sub_831A8130(ctx, base);
	// 8251CD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CD0C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8251CD10: F8A100A8  std r5, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[5].u64 ) };
	// 8251CD14: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251CD18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CD1C: 3BEB70D8  addi r31, r11, 0x70d8
	ctx.r[31].s64 = ctx.r[11].s64 + 28888;
	// 8251CD20: 816A70E0  lwz r11, 0x70e0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28896 as u32) ) } as u64;
	// 8251CD24: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8251CD28: 40820024  bne 0x8251cd4c
	if !ctx.cr[0].eq {
	pc = 0x8251CD4C; continue 'dispatch;
	}
	// 8251CD2C: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251CD30: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251CD34: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8251CD38: 3929C350  addi r9, r9, -0x3cb0
	ctx.r[9].s64 = ctx.r[9].s64 + -15536;
	// 8251CD3C: 3908BE40  addi r8, r8, -0x41c0
	ctx.r[8].s64 = ctx.r[8].s64 + -16832;
	// 8251CD40: 916A70E0  stw r11, 0x70e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28896 as u32), ctx.r[11].u32 ) };
	// 8251CD44: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251CD48: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251CD4C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251CD50: 814100A8  lwz r10, 0xa8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 8251CD54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8251CD58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CD5C: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 8251CD60: 794507E6  rldicr r5, r10, 0x20, 0x3f
	ctx.r[5].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 8251CD64: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8251CD68: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251CD6C: 4BDEA955  bl 0x823076c0
	ctx.lr = 0x8251CD70;
	sub_823076C0(ctx, base);
	// 8251CD70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CD74: 4182000C  beq 0x8251cd80
	if ctx.cr[0].eq {
	pc = 0x8251CD80; continue 'dispatch;
	}
	// 8251CD78: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8251CD7C: 48000008  b 0x8251cd84
	pc = 0x8251CD84; continue 'dispatch;
	// 8251CD80: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8251CD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251CD88: 48C8B434  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CD90 size=740
    let mut pc: u32 = 0x8251CD90;
    'dispatch: loop {
        match pc {
            0x8251CD90 => {
    //   block [0x8251CD90..0x8251D074)
	// 8251CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CD94: 48C8B3D1  bl 0x831a8164
	ctx.lr = 0x8251CD98;
	sub_831A8130(ctx, base);
	// 8251CD98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CD9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CDA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251CDA4: 3BEBE834  addi r31, r11, -0x17cc
	ctx.r[31].s64 = ctx.r[11].s64 + -6092;
	// 8251CDA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251CDAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251CDB0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251CDB4: 480BC74D  bl 0x825d9500
	ctx.lr = 0x8251CDB8;
	sub_825D9500(ctx, base);
	// 8251CDB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CDBC: 418202B0  beq 0x8251d06c
	if ctx.cr[0].eq {
	pc = 0x8251D06C; continue 'dispatch;
	}
	// 8251CDC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CDC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251CDC8: 3B8B25C0  addi r28, r11, 0x25c0
	ctx.r[28].s64 = ctx.r[11].s64 + 9664;
	// 8251CDCC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251CDD0: 480BC731  bl 0x825d9500
	ctx.lr = 0x8251CDD4;
	sub_825D9500(ctx, base);
	// 8251CDD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CDD8: 41820294  beq 0x8251d06c
	if ctx.cr[0].eq {
	pc = 0x8251D06C; continue 'dispatch;
	}
	// 8251CDDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CDE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251CDE4: 3BAB1660  addi r29, r11, 0x1660
	ctx.r[29].s64 = ctx.r[11].s64 + 5728;
	// 8251CDE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251CDEC: 480BC715  bl 0x825d9500
	ctx.lr = 0x8251CDF0;
	sub_825D9500(ctx, base);
	// 8251CDF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CDF4: 41820278  beq 0x8251d06c
	if ctx.cr[0].eq {
	pc = 0x8251D06C; continue 'dispatch;
	}
	// 8251CDF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251CDFC: 4BFCDC8D  bl 0x824eaa88
	ctx.lr = 0x8251CE00;
	sub_824EAA88(ctx, base);
	// 8251CE00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251CE04: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251CE08: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CE0C: 4BFCE34D  bl 0x824eb158
	ctx.lr = 0x8251CE10;
	sub_824EB158(ctx, base);
	// 8251CE10: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251CE14: 488D4E7D  bl 0x82df1c90
	ctx.lr = 0x8251CE18;
	sub_82DF1C90(ctx, base);
	// 8251CE18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251CE1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251CE20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251CE24: 480BC62D  bl 0x825d9450
	ctx.lr = 0x8251CE28;
	sub_825D9450(ctx, base);
	// 8251CE28: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8251CE2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251CE30: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 8251CE34: 409A0008  bne cr6, 0x8251ce3c
	if !ctx.cr[6].eq {
	pc = 0x8251CE3C; continue 'dispatch;
	}
	// 8251CE38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251CE3C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8251CE40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251CE44: 4BFE46CD  bl 0x82501510
	ctx.lr = 0x8251CE48;
	sub_82501510(ctx, base);
	// 8251CE48: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8251CE4C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8251CE50: 419A020C  beq cr6, 0x8251d05c
	if ctx.cr[6].eq {
	pc = 0x8251D05C; continue 'dispatch;
	}
	// 8251CE54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CE58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251CE5C: 388B2580  addi r4, r11, 0x2580
	ctx.r[4].s64 = ctx.r[11].s64 + 9600;
	// 8251CE60: 38A0017A  li r5, 0x17a
	ctx.r[5].s64 = 378;
	// 8251CE64: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251CE68: 4BDA3571  bl 0x822c03d8
	ctx.lr = 0x8251CE6C;
	sub_822C03D8(ctx, base);
	// 8251CE6C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8251CE70: 41820010  beq 0x8251ce80
	if ctx.cr[0].eq {
	pc = 0x8251CE80; continue 'dispatch;
	}
	// 8251CE74: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8251CE78: 488D6279  bl 0x82df30f0
	ctx.lr = 0x8251CE7C;
	sub_82DF30F0(ctx, base);
	// 8251CE7C: 48000008  b 0x8251ce84
	pc = 0x8251CE84; continue 'dispatch;
	// 8251CE80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251CE84: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8251CE88: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251CE8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251CE90: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8251CE94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CE98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CE9C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8251CEA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251CEA4: 480BC5AD  bl 0x825d9450
	ctx.lr = 0x8251CEA8;
	sub_825D9450(ctx, base);
	// 8251CEA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251CEAC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8251CEB0: 488D6D21  bl 0x82df3bd0
	ctx.lr = 0x8251CEB4;
	sub_82DF3BD0(ctx, base);
	// 8251CEB4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CEB8: 488D6571  bl 0x82df3428
	ctx.lr = 0x8251CEBC;
	sub_82DF3428(ctx, base);
	// 8251CEBC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251CEC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251CEC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CEC8: 480BC589  bl 0x825d9450
	ctx.lr = 0x8251CECC;
	sub_825D9450(ctx, base);
	// 8251CECC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251CED0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CED4: 388BD1EC  addi r4, r11, -0x2e14
	ctx.r[4].s64 = ctx.r[11].s64 + -11796;
	// 8251CED8: 488D6B31  bl 0x82df3a08
	ctx.lr = 0x8251CEDC;
	sub_82DF3A08(ctx, base);
	// 8251CEDC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CEE4: 488D6425  bl 0x82df3308
	ctx.lr = 0x8251CEE8;
	sub_82DF3308(ctx, base);
	// 8251CEE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CEEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CEF0: 488D6539  bl 0x82df3428
	ctx.lr = 0x8251CEF4;
	sub_82DF3428(ctx, base);
	// 8251CEF4: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CEF8: 4182000C  beq 0x8251cf04
	if ctx.cr[0].eq {
	pc = 0x8251CF04; continue 'dispatch;
	}
	// 8251CEFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251CF00: 48000118  b 0x8251d018
	pc = 0x8251D018; continue 'dispatch;
	// 8251CF04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CF08: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF0C: 388B257C  addi r4, r11, 0x257c
	ctx.r[4].s64 = ctx.r[11].s64 + 9596;
	// 8251CF10: 488D6AF9  bl 0x82df3a08
	ctx.lr = 0x8251CF14;
	sub_82DF3A08(ctx, base);
	// 8251CF14: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CF1C: 488D63ED  bl 0x82df3308
	ctx.lr = 0x8251CF20;
	sub_82DF3308(ctx, base);
	// 8251CF20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CF24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF28: 488D6501  bl 0x82df3428
	ctx.lr = 0x8251CF2C;
	sub_82DF3428(ctx, base);
	// 8251CF2C: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CF30: 4182000C  beq 0x8251cf3c
	if ctx.cr[0].eq {
	pc = 0x8251CF3C; continue 'dispatch;
	}
	// 8251CF34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251CF38: 480000E0  b 0x8251d018
	pc = 0x8251D018; continue 'dispatch;
	// 8251CF3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251CF40: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF44: 388BCAC0  addi r4, r11, -0x3540
	ctx.r[4].s64 = ctx.r[11].s64 + -13632;
	// 8251CF48: 488D6AC1  bl 0x82df3a08
	ctx.lr = 0x8251CF4C;
	sub_82DF3A08(ctx, base);
	// 8251CF4C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CF50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CF54: 488D63B5  bl 0x82df3308
	ctx.lr = 0x8251CF58;
	sub_82DF3308(ctx, base);
	// 8251CF58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CF5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF60: 488D64C9  bl 0x82df3428
	ctx.lr = 0x8251CF64;
	sub_82DF3428(ctx, base);
	// 8251CF64: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CF68: 4182000C  beq 0x8251cf74
	if ctx.cr[0].eq {
	pc = 0x8251CF74; continue 'dispatch;
	}
	// 8251CF6C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8251CF70: 480000A8  b 0x8251d018
	pc = 0x8251D018; continue 'dispatch;
	// 8251CF74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CF78: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF7C: 388B2578  addi r4, r11, 0x2578
	ctx.r[4].s64 = ctx.r[11].s64 + 9592;
	// 8251CF80: 488D6A89  bl 0x82df3a08
	ctx.lr = 0x8251CF84;
	sub_82DF3A08(ctx, base);
	// 8251CF84: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CF88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CF8C: 488D637D  bl 0x82df3308
	ctx.lr = 0x8251CF90;
	sub_82DF3308(ctx, base);
	// 8251CF90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CF94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CF98: 488D6491  bl 0x82df3428
	ctx.lr = 0x8251CF9C;
	sub_82DF3428(ctx, base);
	// 8251CF9C: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CFA0: 4182000C  beq 0x8251cfac
	if ctx.cr[0].eq {
	pc = 0x8251CFAC; continue 'dispatch;
	}
	// 8251CFA4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8251CFA8: 48000070  b 0x8251d018
	pc = 0x8251D018; continue 'dispatch;
	// 8251CFAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251CFB0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CFB4: 388B1928  addi r4, r11, 0x1928
	ctx.r[4].s64 = ctx.r[11].s64 + 6440;
	// 8251CFB8: 488D6A51  bl 0x82df3a08
	ctx.lr = 0x8251CFBC;
	sub_82DF3A08(ctx, base);
	// 8251CFBC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CFC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CFC4: 488D6345  bl 0x82df3308
	ctx.lr = 0x8251CFC8;
	sub_82DF3308(ctx, base);
	// 8251CFC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CFCC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CFD0: 488D6459  bl 0x82df3428
	ctx.lr = 0x8251CFD4;
	sub_82DF3428(ctx, base);
	// 8251CFD4: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251CFD8: 4182000C  beq 0x8251cfe4
	if ctx.cr[0].eq {
	pc = 0x8251CFE4; continue 'dispatch;
	}
	// 8251CFDC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8251CFE0: 48000038  b 0x8251d018
	pc = 0x8251D018; continue 'dispatch;
	// 8251CFE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251CFE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251CFEC: 388B2574  addi r4, r11, 0x2574
	ctx.r[4].s64 = ctx.r[11].s64 + 9588;
	// 8251CFF0: 488D6A19  bl 0x82df3a08
	ctx.lr = 0x8251CFF4;
	sub_82DF3A08(ctx, base);
	// 8251CFF4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251CFF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CFFC: 488D630D  bl 0x82df3308
	ctx.lr = 0x8251D000;
	sub_82DF3308(ctx, base);
	// 8251D000: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251D004: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251D008: 488D6421  bl 0x82df3428
	ctx.lr = 0x8251D00C;
	sub_82DF3428(ctx, base);
	// 8251D00C: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D010: 4182000C  beq 0x8251d01c
	if ctx.cr[0].eq {
	pc = 0x8251D01C; continue 'dispatch;
	}
	// 8251D014: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8251D018: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251D01C: 83DB0028  lwz r30, 0x28(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) } as u64;
	// 8251D020: 3BFB0024  addi r31, r27, 0x24
	ctx.r[31].s64 = ctx.r[27].s64 + 36;
	// 8251D024: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8251D028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D02C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251D030: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D034: 4803CCCD  bl 0x82559d00
	ctx.lr = 0x8251D038;
	sub_82559D00(ctx, base);
	// 8251D038: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251D03C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251D040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D044: 486ACDB5  bl 0x82bc9df8
	ctx.lr = 0x8251D048;
	sub_82BC9DF8(ctx, base);
	// 8251D048: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8251D04C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251D054: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8251D058: 488D63D1  bl 0x82df3428
	ctx.lr = 0x8251D05C;
	sub_82DF3428(ctx, base);
	// 8251D05C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251D060: 488D63C9  bl 0x82df3428
	ctx.lr = 0x8251D064;
	sub_82DF3428(ctx, base);
	// 8251D064: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251D068: 488D4C29  bl 0x82df1c90
	ctx.lr = 0x8251D06C;
	sub_82DF1C90(ctx, base);
	// 8251D06C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8251D070: 48C8B144  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D078 size=152
    let mut pc: u32 = 0x8251D078;
    'dispatch: loop {
        match pc {
            0x8251D078 => {
    //   block [0x8251D078..0x8251D110)
	// 8251D078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D07C: 48C8B0ED  bl 0x831a8168
	ctx.lr = 0x8251D080;
	sub_831A8130(ctx, base);
	// 8251D080: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D088: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251D08C: 3BBF0124  addi r29, r31, 0x124
	ctx.r[29].s64 = ctx.r[31].s64 + 292;
	// 8251D090: 396B25D0  addi r11, r11, 0x25d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9680;
	// 8251D094: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251D098: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251D09C: 4BFFF0B5  bl 0x8251c150
	ctx.lr = 0x8251D0A0;
	sub_8251C150(ctx, base);
	// 8251D0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251D0A4: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 8251D0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251D0AC: 3B800009  li r28, 9
	ctx.r[28].s64 = 9;
	// 8251D0B0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251D0B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251D0B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251D0BC: 387EFFF0  addi r3, r30, -0x10
	ctx.r[3].s64 = ctx.r[30].s64 + -16;
	// 8251D0C0: 4BDA9FF9  bl 0x822c70b8
	ctx.lr = 0x8251D0C4;
	sub_822C70B8(ctx, base);
	// 8251D0C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251D0C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251D0CC: 4BDA9FED  bl 0x822c70b8
	ctx.lr = 0x8251D0D0;
	sub_822C70B8(ctx, base);
	// 8251D0D0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8251D0D4: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8251D0D8: 4082FFE0  bne 0x8251d0b8
	if !ctx.cr[0].eq {
	pc = 0x8251D0B8; continue 'dispatch;
	}
	// 8251D0DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251D0E0: 4BF941C1  bl 0x824b12a0
	ctx.lr = 0x8251D0E4;
	sub_824B12A0(ctx, base);
	// 8251D0E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251D0E8: 48290101  bl 0x827ad1e8
	ctx.lr = 0x8251D0EC;
	sub_827AD1E8(ctx, base);
	// 8251D0EC: 3BFF0124  addi r31, r31, 0x124
	ctx.r[31].s64 = ctx.r[31].s64 + 292;
	// 8251D0F0: 3BC00011  li r30, 0x11
	ctx.r[30].s64 = 17;
	// 8251D0F4: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 8251D0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D0FC: 4BF941A5  bl 0x824b12a0
	ctx.lr = 0x8251D100;
	sub_824B12A0(ctx, base);
	// 8251D100: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8251D104: 4080FFF0  bge 0x8251d0f4
	if !ctx.cr[0].lt {
	pc = 0x8251D0F4; continue 'dispatch;
	}
	// 8251D108: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251D10C: 48C8B0AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D110 size=152
    let mut pc: u32 = 0x8251D110;
    'dispatch: loop {
        match pc {
            0x8251D110 => {
    //   block [0x8251D110..0x8251D1A8)
	// 8251D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251D118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251D11C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251D120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251D12C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8251D130: 392A25D0  addi r9, r10, 0x25d0
	ctx.r[9].s64 = ctx.r[10].s64 + 9680;
	// 8251D134: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 8251D138: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D13C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8251D140: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251D144: 93CBFFFC  stw r30, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[30].u32 ) };
	// 8251D148: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251D14C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8251D150: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8251D154: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8251D158: 4080FFEC  bge 0x8251d144
	if !ctx.cr[0].lt {
	pc = 0x8251D144; continue 'dispatch;
	}
	// 8251D15C: 387F0124  addi r3, r31, 0x124
	ctx.r[3].s64 = ctx.r[31].s64 + 292;
	// 8251D160: 4BDEC2C9  bl 0x82309428
	ctx.lr = 0x8251D164;
	sub_82309428(ctx, base);
	// 8251D164: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 8251D168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251D16C: 93DF0130  stw r30, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[30].u32 ) };
	// 8251D170: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 8251D174: 93DF0134  stw r30, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 8251D178: 48C8B069  bl 0x831a81e0
	ctx.lr = 0x8251D17C;
	sub_831A81E0(ctx, base);
	// 8251D17C: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 8251D180: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251D184: 387F015C  addi r3, r31, 0x15c
	ctx.r[3].s64 = ctx.r[31].s64 + 348;
	// 8251D188: 48C8B059  bl 0x831a81e0
	ctx.lr = 0x8251D18C;
	sub_831A81E0(ctx, base);
	// 8251D18C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D190: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251D194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251D198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251D19C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251D1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251D1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D1A8 size=76
    let mut pc: u32 = 0x8251D1A8;
    'dispatch: loop {
        match pc {
            0x8251D1A8 => {
    //   block [0x8251D1A8..0x8251D1F4)
	// 8251D1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251D1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251D1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251D1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D1C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251D1C4: 4BFFFEB5  bl 0x8251d078
	ctx.lr = 0x8251D1C8;
	sub_8251D078(ctx, base);
	// 8251D1C8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D1CC: 4182000C  beq 0x8251d1d8
	if ctx.cr[0].eq {
	pc = 0x8251D1D8; continue 'dispatch;
	}
	// 8251D1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D1D4: 4BDA3095  bl 0x822c0268
	ctx.lr = 0x8251D1D8;
	sub_822C0268(ctx, base);
	// 8251D1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251D1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251D1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251D1E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251D1EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251D1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D1F8 size=196
    let mut pc: u32 = 0x8251D1F8;
    'dispatch: loop {
        match pc {
            0x8251D1F8 => {
    //   block [0x8251D1F8..0x8251D2BC)
	// 8251D1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251D200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251D204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251D208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D20C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251D210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251D214: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251D218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251D21C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251D220: 4BDA3719  bl 0x822c0938
	ctx.lr = 0x8251D224;
	sub_822C0938(ctx, base);
	// 8251D224: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251D228: 41820028  beq 0x8251d250
	if ctx.cr[0].eq {
	pc = 0x8251D250; continue 'dispatch;
	}
	// 8251D22C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251D230: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251D234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251D238: 392B2544  addi r9, r11, 0x2544
	ctx.r[9].s64 = ctx.r[11].s64 + 9540;
	// 8251D23C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251D240: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D244: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D248: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251D24C: 48000008  b 0x8251d254
	pc = 0x8251D254; continue 'dispatch;
	// 8251D250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251D254: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251D258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251D25C: 409A0044  bne cr6, 0x8251d2a0
	if !ctx.cr[6].eq {
	pc = 0x8251D2A0; continue 'dispatch;
	}
	// 8251D260: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251D264: 419A001C  beq cr6, 0x8251d280
	if ctx.cr[6].eq {
	pc = 0x8251D280; continue 'dispatch;
	}
	// 8251D268: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251D26C: 4BF94035  bl 0x824b12a0
	ctx.lr = 0x8251D270;
	sub_824B12A0(ctx, base);
	// 8251D270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D274: 488D61B5  bl 0x82df3428
	ctx.lr = 0x8251D278;
	sub_82DF3428(ctx, base);
	// 8251D278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D27C: 4BDA2FED  bl 0x822c0268
	ctx.lr = 0x8251D280;
	sub_822C0268(ctx, base);
	// 8251D280: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251D284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251D288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251D28C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251D290: 816BD22C  lwz r11, -0x2dd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11732 as u32) ) } as u64;
	// 8251D294: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251D298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251D29C: 4BDA2D65  bl 0x822c0000
	ctx.lr = 0x8251D2A0;
	sub_822C0000(ctx, base);
	// 8251D2A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251D2A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251D2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251D2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251D2B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251D2B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251D2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D2C0 size=72
    let mut pc: u32 = 0x8251D2C0;
    'dispatch: loop {
        match pc {
            0x8251D2C0 => {
    //   block [0x8251D2C0..0x8251D308)
	// 8251D2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251D2C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251D2CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D2D0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251D2D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251D2D8: 419A001C  beq cr6, 0x8251d2f4
	if ctx.cr[6].eq {
	pc = 0x8251D2F4; continue 'dispatch;
	}
	// 8251D2DC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251D2E0: 4BF93FC1  bl 0x824b12a0
	ctx.lr = 0x8251D2E4;
	sub_824B12A0(ctx, base);
	// 8251D2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D2E8: 488D6141  bl 0x82df3428
	ctx.lr = 0x8251D2EC;
	sub_82DF3428(ctx, base);
	// 8251D2EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D2F0: 4BDA2F79  bl 0x822c0268
	ctx.lr = 0x8251D2F4;
	sub_822C0268(ctx, base);
	// 8251D2F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251D2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251D2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251D300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D308 size=3220
    let mut pc: u32 = 0x8251D308;
    'dispatch: loop {
        match pc {
            0x8251D308 => {
    //   block [0x8251D308..0x8251DF9C)
	// 8251D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D30C: 48C8AE25  bl 0x831a8130
	ctx.lr = 0x8251D310;
	sub_831A8130(ctx, base);
	// 8251D310: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D314: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 8251D318: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 8251D31C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8251D320: 91C1030C  stw r14, 0x30c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(780 as u32), ctx.r[14].u32 ) };
	// 8251D324: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 8251D328: 93E10314  stw r31, 0x314(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(788 as u32), ctx.r[31].u32 ) };
	// 8251D32C: 488D5E85  bl 0x82df31b0
	ctx.lr = 0x8251D330;
	sub_82DF31B0(ctx, base);
	// 8251D330: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251D334: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251D338: 488D66D1  bl 0x82df3a08
	ctx.lr = 0x8251D33C;
	sub_82DF3A08(ctx, base);
	// 8251D33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251D340: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8251D344: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D348: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8251D34C: 4BF01A1D  bl 0x8241ed68
	ctx.lr = 0x8251D350;
	sub_8241ED68(ctx, base);
	// 8251D350: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251D354: 488D60D5  bl 0x82df3428
	ctx.lr = 0x8251D358;
	sub_82DF3428(ctx, base);
	// 8251D358: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 8251D35C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251D360: 409A001C  bne cr6, 0x8251d37c
	if !ctx.cr[6].eq {
	pc = 0x8251D37C; continue 'dispatch;
	}
	// 8251D364: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8251D368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251D36C: 419A0008  beq cr6, 0x8251d374
	if ctx.cr[6].eq {
	pc = 0x8251D374; continue 'dispatch;
	}
	// 8251D370: 4BDA3521  bl 0x822c0890
	ctx.lr = 0x8251D374;
	sub_822C0890(ctx, base);
	// 8251D374: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251D378: 48000C1C  b 0x8251df94
	pc = 0x8251DF94; continue 'dispatch;
	// 8251D37C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D380: 488E1461  bl 0x82dfe7e0
	ctx.lr = 0x8251D384;
	sub_82DFE7E0(ctx, base);
	// 8251D384: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251D388: 83A100D4  lwz r29, 0xd4(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8251D38C: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251D390: 3BCB2580  addi r30, r11, 0x2580
	ctx.r[30].s64 = ctx.r[11].s64 + 9600;
	// 8251D394: 93C100A8  stw r30, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 8251D398: 40820058  bne 0x8251d3f0
	if !ctx.cr[0].eq {
	pc = 0x8251D3F0; continue 'dispatch;
	}
	// 8251D39C: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 8251D3A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251D3A4: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 8251D3A8: 419A0024  beq cr6, 0x8251d3cc
	if ctx.cr[6].eq {
	pc = 0x8251D3CC; continue 'dispatch;
	}
	// 8251D3AC: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8251D3B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251D3B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251D3B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251D3BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251D3C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251D3C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251D3C8: 4082FFE8  bne 0x8251d3b0
	if !ctx.cr[0].eq {
	pc = 0x8251D3B0; continue 'dispatch;
	}
	// 8251D3CC: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 8251D3D0: 4BFCD6B9  bl 0x824eaa88
	ctx.lr = 0x8251D3D4;
	sub_824EAA88(ctx, base);
	// 8251D3D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251D3D8: 38C00088  li r6, 0x88
	ctx.r[6].s64 = 136;
	// 8251D3DC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D3E0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8251D3E4: 4869CD9D  bl 0x82bba180
	ctx.lr = 0x8251D3E8;
	sub_82BBA180(ctx, base);
	// 8251D3E8: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 8251D3EC: 488D48A5  bl 0x82df1c90
	ctx.lr = 0x8251D3F0;
	sub_82DF1C90(ctx, base);
	// 8251D3F0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251D3F4: 480BBB05  bl 0x825d8ef8
	ctx.lr = 0x8251D3F8;
	sub_825D8EF8(ctx, base);
	// 8251D3F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D3FC: 38610208  addi r3, r1, 0x208
	ctx.r[3].s64 = ctx.r[1].s64 + 520;
	// 8251D400: 4897D509  bl 0x82e9a908
	ctx.lr = 0x8251D404;
	sub_82E9A908(ctx, base);
	// 8251D404: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D408: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 8251D40C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D410: 488D5DA1  bl 0x82df31b0
	ctx.lr = 0x8251D414;
	sub_82DF31B0(ctx, base);
	// 8251D414: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251D418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D41C: 48AEAD0D  bl 0x83008128
	ctx.lr = 0x8251D420;
	sub_83008128(ctx, base);
	// 8251D420: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8251D424: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251D428: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251D42C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 8251D430: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8251D434: 480BBCF5  bl 0x825d9128
	ctx.lr = 0x8251D438;
	sub_825D9128(ctx, base);
	// 8251D438: 8061020C  lwz r3, 0x20c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(524 as u32) ) } as u64;
	// 8251D43C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251D440: 419A0008  beq cr6, 0x8251d448
	if ctx.cr[6].eq {
	pc = 0x8251D448; continue 'dispatch;
	}
	// 8251D444: 4BDA344D  bl 0x822c0890
	ctx.lr = 0x8251D448;
	sub_822C0890(ctx, base);
	// 8251D448: 808100D8  lwz r4, 0xd8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 8251D44C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8251D450: 409A002C  bne cr6, 0x8251d47c
	if !ctx.cr[6].eq {
	pc = 0x8251D47C; continue 'dispatch;
	}
	// 8251D454: 806100DC  lwz r3, 0xdc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 8251D458: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251D45C: 419A0008  beq cr6, 0x8251d464
	if ctx.cr[6].eq {
	pc = 0x8251D464; continue 'dispatch;
	}
	// 8251D460: 4BDA3431  bl 0x822c0890
	ctx.lr = 0x8251D464;
	sub_822C0890(ctx, base);
	// 8251D464: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251D468: 480BBB49  bl 0x825d8fb0
	ctx.lr = 0x8251D46C;
	sub_825D8FB0(ctx, base);
	// 8251D46C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251D470: 419AFF04  beq cr6, 0x8251d374
	if ctx.cr[6].eq {
	pc = 0x8251D374; continue 'dispatch;
	}
	// 8251D474: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251D478: 4BFFFEF8  b 0x8251d370
	pc = 0x8251D370; continue 'dispatch;
	// 8251D47C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251D480: 480BB389  bl 0x825d8808
	ctx.lr = 0x8251D484;
	sub_825D8808(ctx, base);
	// 8251D484: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251D48C: 409A0018  bne cr6, 0x8251d4a4
	if !ctx.cr[6].eq {
	pc = 0x8251D4A4; continue 'dispatch;
	}
	// 8251D490: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251D494: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251D498: 419AFFBC  beq cr6, 0x8251d454
	if ctx.cr[6].eq {
	pc = 0x8251D454; continue 'dispatch;
	}
	// 8251D49C: 4BDA33F5  bl 0x822c0890
	ctx.lr = 0x8251D4A0;
	sub_822C0890(ctx, base);
	// 8251D4A0: 4BFFFFB4  b 0x8251d454
	pc = 0x8251D454; continue 'dispatch;
	// 8251D4A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251D4A8: 480BC8E9  bl 0x825d9d90
	ctx.lr = 0x8251D4AC;
	sub_825D9D90(ctx, base);
	// 8251D4AC: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 8251D4B0: 488D5C41  bl 0x82df30f0
	ctx.lr = 0x8251D4B4;
	sub_82DF30F0(ctx, base);
	// 8251D4B4: 38610200  addi r3, r1, 0x200
	ctx.r[3].s64 = ctx.r[1].s64 + 512;
	// 8251D4B8: 4BFCD5D1  bl 0x824eaa88
	ctx.lr = 0x8251D4BC;
	sub_824EAA88(ctx, base);
	// 8251D4BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D4C0: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 8251D4C4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D4C8: 4BFCDC91  bl 0x824eb158
	ctx.lr = 0x8251D4CC;
	sub_824EB158(ctx, base);
	// 8251D4CC: 38610200  addi r3, r1, 0x200
	ctx.r[3].s64 = ctx.r[1].s64 + 512;
	// 8251D4D0: 488D47C1  bl 0x82df1c90
	ctx.lr = 0x8251D4D4;
	sub_82DF1C90(ctx, base);
	// 8251D4D4: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 8251D4D8: 4BF17B39  bl 0x82435010
	ctx.lr = 0x8251D4DC;
	sub_82435010(ctx, base);
	// 8251D4DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251D4E0: 386101C0  addi r3, r1, 0x1c0
	ctx.r[3].s64 = ctx.r[1].s64 + 448;
	// 8251D4E4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D4E8: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 8251D4EC: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8251D4F0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8251D4F4: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 8251D4F8: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 8251D4FC: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 8251D500: 93C10084  stw r30, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8251D504: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 8251D508: 480BC499  bl 0x825d99a0
	ctx.lr = 0x8251D50C;
	sub_825D99A0(ctx, base);
	// 8251D50C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D510: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251D514: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251D518: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D51C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251D520: 4BDA6F41  bl 0x822c4460
	ctx.lr = 0x8251D524;
	sub_822C4460(ctx, base);
	// 8251D524: 806101C4  lwz r3, 0x1c4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(452 as u32) ) } as u64;
	// 8251D528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251D52C: 419A0008  beq cr6, 0x8251d534
	if ctx.cr[6].eq {
	pc = 0x8251D534; continue 'dispatch;
	}
	// 8251D530: 4BDA3361  bl 0x822c0890
	ctx.lr = 0x8251D534;
	sub_822C0890(ctx, base);
	// 8251D534: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D538: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251D53C: 419A09BC  beq cr6, 0x8251def8
	if ctx.cr[6].eq {
	pc = 0x8251DEF8; continue 'dispatch;
	}
	// 8251D540: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8251D544: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 8251D548: 396B059C  addi r11, r11, 0x59c
	ctx.r[11].s64 = ctx.r[11].s64 + 1436;
	// 8251D54C: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 8251D550: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 8251D554: 91610150  stw r11, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 8251D558: 38E72680  addi r7, r7, 0x2680
	ctx.r[7].s64 = ctx.r[7].s64 + 9856;
	// 8251D55C: 38C62674  addi r6, r6, 0x2674
	ctx.r[6].s64 = ctx.r[6].s64 + 9844;
	// 8251D560: 3965266C  addi r11, r5, 0x266c
	ctx.r[11].s64 = ctx.r[5].s64 + 9836;
	// 8251D564: 90E10158  stw r7, 0x158(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), ctx.r[7].u32 ) };
	// 8251D568: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 8251D56C: 90C10130  stw r6, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[6].u32 ) };
	// 8251D570: 3C608203  lis r3, -0x7dfd
	ctx.r[3].s64 = -2113732608;
	// 8251D574: 91610148  stw r11, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 8251D578: 3FE08203  lis r31, -0x7dfd
	ctx.r[31].s64 = -2113732608;
	// 8251D57C: 38E42660  addi r7, r4, 0x2660
	ctx.r[7].s64 = ctx.r[4].s64 + 9824;
	// 8251D580: 38C32654  addi r6, r3, 0x2654
	ctx.r[6].s64 = ctx.r[3].s64 + 9812;
	// 8251D584: 397F264C  addi r11, r31, 0x264c
	ctx.r[11].s64 = ctx.r[31].s64 + 9804;
	// 8251D588: 90E10128  stw r7, 0x128(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), ctx.r[7].u32 ) };
	// 8251D58C: 3FC08203  lis r30, -0x7dfd
	ctx.r[30].s64 = -2113732608;
	// 8251D590: 90C10160  stw r6, 0x160(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.r[6].u32 ) };
	// 8251D594: 3FA08203  lis r29, -0x7dfd
	ctx.r[29].s64 = -2113732608;
	// 8251D598: 916100E0  stw r11, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8251D59C: 3F808203  lis r28, -0x7dfd
	ctx.r[28].s64 = -2113732608;
	// 8251D5A0: 38FE2644  addi r7, r30, 0x2644
	ctx.r[7].s64 = ctx.r[30].s64 + 9796;
	// 8251D5A4: 38DD263C  addi r6, r29, 0x263c
	ctx.r[6].s64 = ctx.r[29].s64 + 9788;
	// 8251D5A8: 397C2638  addi r11, r28, 0x2638
	ctx.r[11].s64 = ctx.r[28].s64 + 9784;
	// 8251D5AC: 90E10120  stw r7, 0x120(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[7].u32 ) };
	// 8251D5B0: 3F608203  lis r27, -0x7dfd
	ctx.r[27].s64 = -2113732608;
	// 8251D5B4: 90C1015C  stw r6, 0x15c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), ctx.r[6].u32 ) };
	// 8251D5B8: 3F408203  lis r26, -0x7dfd
	ctx.r[26].s64 = -2113732608;
	// 8251D5BC: 91610154  stw r11, 0x154(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 8251D5C0: 3F208203  lis r25, -0x7dfd
	ctx.r[25].s64 = -2113732608;
	// 8251D5C4: 38FB262C  addi r7, r27, 0x262c
	ctx.r[7].s64 = ctx.r[27].s64 + 9772;
	// 8251D5C8: 38DA2624  addi r6, r26, 0x2624
	ctx.r[6].s64 = ctx.r[26].s64 + 9764;
	// 8251D5CC: 3979261C  addi r11, r25, 0x261c
	ctx.r[11].s64 = ctx.r[25].s64 + 9756;
	// 8251D5D0: 90E1014C  stw r7, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[7].u32 ) };
	// 8251D5D4: 3F008203  lis r24, -0x7dfd
	ctx.r[24].s64 = -2113732608;
	// 8251D5D8: 90C10144  stw r6, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[6].u32 ) };
	// 8251D5DC: 3EE08203  lis r23, -0x7dfd
	ctx.r[23].s64 = -2113732608;
	// 8251D5E0: 9161013C  stw r11, 0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 8251D5E4: 3EC08203  lis r22, -0x7dfd
	ctx.r[22].s64 = -2113732608;
	// 8251D5E8: 38F82614  addi r7, r24, 0x2614
	ctx.r[7].s64 = ctx.r[24].s64 + 9748;
	// 8251D5EC: 38D7260C  addi r6, r23, 0x260c
	ctx.r[6].s64 = ctx.r[23].s64 + 9740;
	// 8251D5F0: 39762604  addi r11, r22, 0x2604
	ctx.r[11].s64 = ctx.r[22].s64 + 9732;
	// 8251D5F4: 90E10134  stw r7, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[7].u32 ) };
	// 8251D5F8: 3EA08203  lis r21, -0x7dfd
	ctx.r[21].s64 = -2113732608;
	// 8251D5FC: 90C10108  stw r6, 0x108(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[6].u32 ) };
	// 8251D600: 3E808203  lis r20, -0x7dfd
	ctx.r[20].s64 = -2113732608;
	// 8251D604: 916100F8  stw r11, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 8251D608: 3E608203  lis r19, -0x7dfd
	ctx.r[19].s64 = -2113732608;
	// 8251D60C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8251D610: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 8251D614: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 8251D618: 38F525FC  addi r7, r21, 0x25fc
	ctx.r[7].s64 = ctx.r[21].s64 + 9724;
	// 8251D61C: 38D425F8  addi r6, r20, 0x25f8
	ctx.r[6].s64 = ctx.r[20].s64 + 9720;
	// 8251D620: 397325F0  addi r11, r19, 0x25f0
	ctx.r[11].s64 = ctx.r[19].s64 + 9712;
	// 8251D624: 90E100E8  stw r7, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[7].u32 ) };
	// 8251D628: 3A2A9BC9  addi r17, r10, -0x6437
	ctx.r[17].s64 = ctx.r[10].s64 + -25655;
	// 8251D62C: 90C1012C  stw r6, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[6].u32 ) };
	// 8251D630: 3A0925E4  addi r16, r9, 0x25e4
	ctx.r[16].s64 = ctx.r[9].s64 + 9700;
	// 8251D634: 9161011C  stw r11, 0x11c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 8251D638: 39E825D4  addi r15, r8, 0x25d4
	ctx.r[15].s64 = ctx.r[8].s64 + 9684;
	// 8251D63C: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8251D640: 8081011C  lwz r4, 0x11c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 8251D644: 488D63C5  bl 0x82df3a08
	ctx.lr = 0x8251D648;
	sub_82DF3A08(ctx, base);
	// 8251D648: 38610124  addi r3, r1, 0x124
	ctx.r[3].s64 = ctx.r[1].s64 + 292;
	// 8251D64C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D650: 3BE10094  addi r31, r1, 0x94
	ctx.r[31].s64 = ctx.r[1].s64 + 148;
	// 8251D654: 480BBF15  bl 0x825d9568
	ctx.lr = 0x8251D658;
	sub_825D9568(ctx, base);
	// 8251D658: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D65C: 488D5CAD  bl 0x82df3308
	ctx.lr = 0x8251D660;
	sub_82DF3308(ctx, base);
	// 8251D660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D664: 38610124  addi r3, r1, 0x124
	ctx.r[3].s64 = ctx.r[1].s64 + 292;
	// 8251D668: 488D5DC1  bl 0x82df3428
	ctx.lr = 0x8251D66C;
	sub_82DF3428(ctx, base);
	// 8251D66C: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8251D670: 488D5DB9  bl 0x82df3428
	ctx.lr = 0x8251D674;
	sub_82DF3428(ctx, base);
	// 8251D674: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D678: 41820010  beq 0x8251d688
	if ctx.cr[0].eq {
	pc = 0x8251D688; continue 'dispatch;
	}
	// 8251D67C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8251D680: 3A600006  li r19, 6
	ctx.r[19].s64 = 6;
	// 8251D684: 48000260  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D688: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251D68C: 8081012C  lwz r4, 0x12c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 8251D690: 488D6379  bl 0x82df3a08
	ctx.lr = 0x8251D694;
	sub_82DF3A08(ctx, base);
	// 8251D694: 386100E4  addi r3, r1, 0xe4
	ctx.r[3].s64 = ctx.r[1].s64 + 228;
	// 8251D698: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D69C: 3BE10098  addi r31, r1, 0x98
	ctx.r[31].s64 = ctx.r[1].s64 + 152;
	// 8251D6A0: 480BBEC9  bl 0x825d9568
	ctx.lr = 0x8251D6A4;
	sub_825D9568(ctx, base);
	// 8251D6A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D6A8: 488D5C61  bl 0x82df3308
	ctx.lr = 0x8251D6AC;
	sub_82DF3308(ctx, base);
	// 8251D6AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D6B0: 386100E4  addi r3, r1, 0xe4
	ctx.r[3].s64 = ctx.r[1].s64 + 228;
	// 8251D6B4: 488D5D75  bl 0x82df3428
	ctx.lr = 0x8251D6B8;
	sub_82DF3428(ctx, base);
	// 8251D6B8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251D6BC: 488D5D6D  bl 0x82df3428
	ctx.lr = 0x8251D6C0;
	sub_82DF3428(ctx, base);
	// 8251D6C0: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D6C4: 41820010  beq 0x8251d6d4
	if ctx.cr[0].eq {
	pc = 0x8251D6D4; continue 'dispatch;
	}
	// 8251D6C8: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 8251D6CC: 3A600005  li r19, 5
	ctx.r[19].s64 = 5;
	// 8251D6D0: 48000214  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D6D4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251D6D8: 808100E8  lwz r4, 0xe8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 8251D6DC: 488D632D  bl 0x82df3a08
	ctx.lr = 0x8251D6E0;
	sub_82DF3A08(ctx, base);
	// 8251D6E0: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8251D6E4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D6E8: 3BE100A0  addi r31, r1, 0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + 160;
	// 8251D6EC: 480BBE7D  bl 0x825d9568
	ctx.lr = 0x8251D6F0;
	sub_825D9568(ctx, base);
	// 8251D6F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D6F4: 488D5C15  bl 0x82df3308
	ctx.lr = 0x8251D6F8;
	sub_82DF3308(ctx, base);
	// 8251D6F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D6FC: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8251D700: 488D5D29  bl 0x82df3428
	ctx.lr = 0x8251D704;
	sub_82DF3428(ctx, base);
	// 8251D704: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8251D708: 488D5D21  bl 0x82df3428
	ctx.lr = 0x8251D70C;
	sub_82DF3428(ctx, base);
	// 8251D70C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D710: 41820010  beq 0x8251d720
	if ctx.cr[0].eq {
	pc = 0x8251D720; continue 'dispatch;
	}
	// 8251D714: 3AC00002  li r22, 2
	ctx.r[22].s64 = 2;
	// 8251D718: 3A600003  li r19, 3
	ctx.r[19].s64 = 3;
	// 8251D71C: 480001C8  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D720: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8251D724: 808100F8  lwz r4, 0xf8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 8251D728: 488D62E1  bl 0x82df3a08
	ctx.lr = 0x8251D72C;
	sub_82DF3A08(ctx, base);
	// 8251D72C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8251D730: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D734: 3BE100B0  addi r31, r1, 0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + 176;
	// 8251D738: 480BBE31  bl 0x825d9568
	ctx.lr = 0x8251D73C;
	sub_825D9568(ctx, base);
	// 8251D73C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D740: 488D5BC9  bl 0x82df3308
	ctx.lr = 0x8251D744;
	sub_82DF3308(ctx, base);
	// 8251D744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D748: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8251D74C: 488D5CDD  bl 0x82df3428
	ctx.lr = 0x8251D750;
	sub_82DF3428(ctx, base);
	// 8251D750: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8251D754: 488D5CD5  bl 0x82df3428
	ctx.lr = 0x8251D758;
	sub_82DF3428(ctx, base);
	// 8251D758: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D75C: 41820010  beq 0x8251d76c
	if ctx.cr[0].eq {
	pc = 0x8251D76C; continue 'dispatch;
	}
	// 8251D760: 3AC00003  li r22, 3
	ctx.r[22].s64 = 3;
	// 8251D764: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 8251D768: 4800017C  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D76C: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 8251D770: 80810108  lwz r4, 0x108(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(264 as u32) ) } as u64;
	// 8251D774: 488D6295  bl 0x82df3a08
	ctx.lr = 0x8251D778;
	sub_82DF3A08(ctx, base);
	// 8251D778: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8251D77C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D780: 3BE100B8  addi r31, r1, 0xb8
	ctx.r[31].s64 = ctx.r[1].s64 + 184;
	// 8251D784: 480BBDE5  bl 0x825d9568
	ctx.lr = 0x8251D788;
	sub_825D9568(ctx, base);
	// 8251D788: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D78C: 488D5B7D  bl 0x82df3308
	ctx.lr = 0x8251D790;
	sub_82DF3308(ctx, base);
	// 8251D790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D794: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8251D798: 488D5C91  bl 0x82df3428
	ctx.lr = 0x8251D79C;
	sub_82DF3428(ctx, base);
	// 8251D79C: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 8251D7A0: 488D5C89  bl 0x82df3428
	ctx.lr = 0x8251D7A4;
	sub_82DF3428(ctx, base);
	// 8251D7A4: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D7A8: 41820010  beq 0x8251d7b8
	if ctx.cr[0].eq {
	pc = 0x8251D7B8; continue 'dispatch;
	}
	// 8251D7AC: 3AC00004  li r22, 4
	ctx.r[22].s64 = 4;
	// 8251D7B0: 3A600009  li r19, 9
	ctx.r[19].s64 = 9;
	// 8251D7B4: 48000130  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D7B8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8251D7BC: 80810134  lwz r4, 0x134(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8251D7C0: 488D6249  bl 0x82df3a08
	ctx.lr = 0x8251D7C4;
	sub_82DF3A08(ctx, base);
	// 8251D7C4: 386100EC  addi r3, r1, 0xec
	ctx.r[3].s64 = ctx.r[1].s64 + 236;
	// 8251D7C8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D7CC: 3BE100C0  addi r31, r1, 0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + 192;
	// 8251D7D0: 480BBD99  bl 0x825d9568
	ctx.lr = 0x8251D7D4;
	sub_825D9568(ctx, base);
	// 8251D7D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D7D8: 488D5B31  bl 0x82df3308
	ctx.lr = 0x8251D7DC;
	sub_82DF3308(ctx, base);
	// 8251D7DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D7E0: 386100EC  addi r3, r1, 0xec
	ctx.r[3].s64 = ctx.r[1].s64 + 236;
	// 8251D7E4: 488D5C45  bl 0x82df3428
	ctx.lr = 0x8251D7E8;
	sub_82DF3428(ctx, base);
	// 8251D7E8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8251D7EC: 488D5C3D  bl 0x82df3428
	ctx.lr = 0x8251D7F0;
	sub_82DF3428(ctx, base);
	// 8251D7F0: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D7F4: 41820010  beq 0x8251d804
	if ctx.cr[0].eq {
	pc = 0x8251D804; continue 'dispatch;
	}
	// 8251D7F8: 3AC00005  li r22, 5
	ctx.r[22].s64 = 5;
	// 8251D7FC: 3A600008  li r19, 8
	ctx.r[19].s64 = 8;
	// 8251D800: 480000E4  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D804: 386100C4  addi r3, r1, 0xc4
	ctx.r[3].s64 = ctx.r[1].s64 + 196;
	// 8251D808: 8081013C  lwz r4, 0x13c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(316 as u32) ) } as u64;
	// 8251D80C: 488D61FD  bl 0x82df3a08
	ctx.lr = 0x8251D810;
	sub_82DF3A08(ctx, base);
	// 8251D810: 386100F4  addi r3, r1, 0xf4
	ctx.r[3].s64 = ctx.r[1].s64 + 244;
	// 8251D814: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D818: 3BE100C4  addi r31, r1, 0xc4
	ctx.r[31].s64 = ctx.r[1].s64 + 196;
	// 8251D81C: 480BBD4D  bl 0x825d9568
	ctx.lr = 0x8251D820;
	sub_825D9568(ctx, base);
	// 8251D820: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D824: 488D5AE5  bl 0x82df3308
	ctx.lr = 0x8251D828;
	sub_82DF3308(ctx, base);
	// 8251D828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D82C: 386100F4  addi r3, r1, 0xf4
	ctx.r[3].s64 = ctx.r[1].s64 + 244;
	// 8251D830: 488D5BF9  bl 0x82df3428
	ctx.lr = 0x8251D834;
	sub_82DF3428(ctx, base);
	// 8251D834: 386100C4  addi r3, r1, 0xc4
	ctx.r[3].s64 = ctx.r[1].s64 + 196;
	// 8251D838: 488D5BF1  bl 0x82df3428
	ctx.lr = 0x8251D83C;
	sub_82DF3428(ctx, base);
	// 8251D83C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D840: 41820010  beq 0x8251d850
	if ctx.cr[0].eq {
	pc = 0x8251D850; continue 'dispatch;
	}
	// 8251D844: 3AC00006  li r22, 6
	ctx.r[22].s64 = 6;
	// 8251D848: 3A600002  li r19, 2
	ctx.r[19].s64 = 2;
	// 8251D84C: 48000098  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D850: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 8251D854: 80810144  lwz r4, 0x144(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 8251D858: 488D61B1  bl 0x82df3a08
	ctx.lr = 0x8251D85C;
	sub_82DF3A08(ctx, base);
	// 8251D85C: 386100FC  addi r3, r1, 0xfc
	ctx.r[3].s64 = ctx.r[1].s64 + 252;
	// 8251D860: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D864: 3BE100B4  addi r31, r1, 0xb4
	ctx.r[31].s64 = ctx.r[1].s64 + 180;
	// 8251D868: 480BBD01  bl 0x825d9568
	ctx.lr = 0x8251D86C;
	sub_825D9568(ctx, base);
	// 8251D86C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D870: 488D5A99  bl 0x82df3308
	ctx.lr = 0x8251D874;
	sub_82DF3308(ctx, base);
	// 8251D874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D878: 386100FC  addi r3, r1, 0xfc
	ctx.r[3].s64 = ctx.r[1].s64 + 252;
	// 8251D87C: 488D5BAD  bl 0x82df3428
	ctx.lr = 0x8251D880;
	sub_82DF3428(ctx, base);
	// 8251D880: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 8251D884: 488D5BA5  bl 0x82df3428
	ctx.lr = 0x8251D888;
	sub_82DF3428(ctx, base);
	// 8251D888: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D88C: 41820010  beq 0x8251d89c
	if ctx.cr[0].eq {
	pc = 0x8251D89C; continue 'dispatch;
	}
	// 8251D890: 3AC00007  li r22, 7
	ctx.r[22].s64 = 7;
	// 8251D894: 3A600007  li r19, 7
	ctx.r[19].s64 = 7;
	// 8251D898: 4800004C  b 0x8251d8e4
	pc = 0x8251D8E4; continue 'dispatch;
	// 8251D89C: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 8251D8A0: 8081014C  lwz r4, 0x14c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(332 as u32) ) } as u64;
	// 8251D8A4: 488D6165  bl 0x82df3a08
	ctx.lr = 0x8251D8A8;
	sub_82DF3A08(ctx, base);
	// 8251D8A8: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8251D8AC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D8B0: 3BE1009C  addi r31, r1, 0x9c
	ctx.r[31].s64 = ctx.r[1].s64 + 156;
	// 8251D8B4: 480BBCB5  bl 0x825d9568
	ctx.lr = 0x8251D8B8;
	sub_825D9568(ctx, base);
	// 8251D8B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D8BC: 488D5A4D  bl 0x82df3308
	ctx.lr = 0x8251D8C0;
	sub_82DF3308(ctx, base);
	// 8251D8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D8C4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 8251D8C8: 488D5B61  bl 0x82df3428
	ctx.lr = 0x8251D8CC;
	sub_82DF3428(ctx, base);
	// 8251D8CC: 3861009C  addi r3, r1, 0x9c
	ctx.r[3].s64 = ctx.r[1].s64 + 156;
	// 8251D8D0: 488D5B59  bl 0x82df3428
	ctx.lr = 0x8251D8D4;
	sub_82DF3428(ctx, base);
	// 8251D8D4: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D8D8: 418205D4  beq 0x8251deac
	if ctx.cr[0].eq {
	pc = 0x8251DEAC; continue 'dispatch;
	}
	// 8251D8DC: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	// 8251D8E0: 3A600004  li r19, 4
	ctx.r[19].s64 = 4;
	// 8251D8E4: 386101F8  addi r3, r1, 0x1f8
	ctx.r[3].s64 = ctx.r[1].s64 + 504;
	// 8251D8E8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251D8EC: 480BC0B5  bl 0x825d99a0
	ctx.lr = 0x8251D8F0;
	sub_825D99A0(ctx, base);
	// 8251D8F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D8F4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251D8F8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251D8FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D900: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8251D904: 4BDA6B5D  bl 0x822c4460
	ctx.lr = 0x8251D908;
	sub_822C4460(ctx, base);
	// 8251D908: 806101FC  lwz r3, 0x1fc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(508 as u32) ) } as u64;
	// 8251D90C: 4800055C  b 0x8251de68
	pc = 0x8251DE68; continue 'dispatch;
	// 8251D910: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8251D914: 80810154  lwz r4, 0x154(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) } as u64;
	// 8251D918: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8251D91C: 488D60ED  bl 0x82df3a08
	ctx.lr = 0x8251D920;
	sub_82DF3A08(ctx, base);
	// 8251D920: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251D924: 3861010C  addi r3, r1, 0x10c
	ctx.r[3].s64 = ctx.r[1].s64 + 268;
	// 8251D928: 3BE100A4  addi r31, r1, 0xa4
	ctx.r[31].s64 = ctx.r[1].s64 + 164;
	// 8251D92C: 480BBC3D  bl 0x825d9568
	ctx.lr = 0x8251D930;
	sub_825D9568(ctx, base);
	// 8251D930: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D934: 488D59D5  bl 0x82df3308
	ctx.lr = 0x8251D938;
	sub_82DF3308(ctx, base);
	// 8251D938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D93C: 3861010C  addi r3, r1, 0x10c
	ctx.r[3].s64 = ctx.r[1].s64 + 268;
	// 8251D940: 488D5AE9  bl 0x82df3428
	ctx.lr = 0x8251D944;
	sub_82DF3428(ctx, base);
	// 8251D944: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 8251D948: 488D5AE1  bl 0x82df3428
	ctx.lr = 0x8251D94C;
	sub_82DF3428(ctx, base);
	// 8251D94C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D950: 40820048  bne 0x8251d998
	if !ctx.cr[0].eq {
	pc = 0x8251D998; continue 'dispatch;
	}
	// 8251D954: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251D958: 8081015C  lwz r4, 0x15c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(348 as u32) ) } as u64;
	// 8251D95C: 488D60AD  bl 0x82df3a08
	ctx.lr = 0x8251D960;
	sub_82DF3A08(ctx, base);
	// 8251D960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251D964: 38610114  addi r3, r1, 0x114
	ctx.r[3].s64 = ctx.r[1].s64 + 276;
	// 8251D968: 3BE10090  addi r31, r1, 0x90
	ctx.r[31].s64 = ctx.r[1].s64 + 144;
	// 8251D96C: 480BBBFD  bl 0x825d9568
	ctx.lr = 0x8251D970;
	sub_825D9568(ctx, base);
	// 8251D970: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251D974: 488D5995  bl 0x82df3308
	ctx.lr = 0x8251D978;
	sub_82DF3308(ctx, base);
	// 8251D978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251D97C: 38610114  addi r3, r1, 0x114
	ctx.r[3].s64 = ctx.r[1].s64 + 276;
	// 8251D980: 488D5AA9  bl 0x82df3428
	ctx.lr = 0x8251D984;
	sub_82DF3428(ctx, base);
	// 8251D984: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251D988: 488D5AA1  bl 0x82df3428
	ctx.lr = 0x8251D98C;
	sub_82DF3428(ctx, base);
	// 8251D98C: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251D990: 41820008  beq 0x8251d998
	if ctx.cr[0].eq {
	pc = 0x8251D998; continue 'dispatch;
	}
	// 8251D994: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 8251D998: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251D99C: 386101D0  addi r3, r1, 0x1d0
	ctx.r[3].s64 = ctx.r[1].s64 + 464;
	// 8251D9A0: 480BC001  bl 0x825d99a0
	ctx.lr = 0x8251D9A4;
	sub_825D99A0(ctx, base);
	// 8251D9A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251D9A8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8251D9AC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251D9B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D9B4: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8251D9B8: 4BDA6AA9  bl 0x822c4460
	ctx.lr = 0x8251D9BC;
	sub_822C4460(ctx, base);
	// 8251D9BC: 806101D4  lwz r3, 0x1d4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(468 as u32) ) } as u64;
	// 8251D9C0: 48000468  b 0x8251de28
	pc = 0x8251DE28; continue 'dispatch;
	// 8251D9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251D9C8: 808100A8  lwz r4, 0xa8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 8251D9CC: 38A000E6  li r5, 0xe6
	ctx.r[5].s64 = 230;
	// 8251D9D0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8251D9D4: 4BDA2A05  bl 0x822c03d8
	ctx.lr = 0x8251D9D8;
	sub_822C03D8(ctx, base);
	// 8251D9D8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8251D9DC: 41820020  beq 0x8251d9fc
	if ctx.cr[0].eq {
	pc = 0x8251D9FC; continue 'dispatch;
	}
	// 8251D9E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251D9E4: 488D570D  bl 0x82df30f0
	ctx.lr = 0x8251D9E8;
	sub_82DF30F0(ctx, base);
	// 8251D9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251D9EC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251D9F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251D9F4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8251D9F8: 48000008  b 0x8251da00
	pc = 0x8251DA00; continue 'dispatch;
	// 8251D9FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251DA00: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 8251DA04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251DA08: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 8251DA0C: 4BFFF7ED  bl 0x8251d1f8
	ctx.lr = 0x8251DA10;
	sub_8251D1F8(ctx, base);
	// 8251DA10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251DA14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251DA18: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 8251DA1C: 4BDA25E5  bl 0x822c0000
	ctx.lr = 0x8251DA20;
	sub_822C0000(ctx, base);
	// 8251DA20: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8251DA24: 38610164  addi r3, r1, 0x164
	ctx.r[3].s64 = ctx.r[1].s64 + 356;
	// 8251DA28: 480BBB41  bl 0x825d9568
	ctx.lr = 0x8251DA2C;
	sub_825D9568(ctx, base);
	// 8251DA2C: 83E10078  lwz r31, 0x78(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 8251DA30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251DA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251DA38: 488D6199  bl 0x82df3bd0
	ctx.lr = 0x8251DA3C;
	sub_82DF3BD0(ctx, base);
	// 8251DA3C: 38610164  addi r3, r1, 0x164
	ctx.r[3].s64 = ctx.r[1].s64 + 356;
	// 8251DA40: 488D59E9  bl 0x82df3428
	ctx.lr = 0x8251DA44;
	sub_82DF3428(ctx, base);
	// 8251DA44: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8251DA48: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 8251DA4C: 480BBF55  bl 0x825d99a0
	ctx.lr = 0x8251DA50;
	sub_825D99A0(ctx, base);
	// 8251DA50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DA54: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 8251DA58: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DA5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DA60: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8251DA64: 4BDA69FD  bl 0x822c4460
	ctx.lr = 0x8251DA68;
	sub_822C4460(ctx, base);
	// 8251DA68: 806101EC  lwz r3, 0x1ec(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(492 as u32) ) } as u64;
	// 8251DA6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DA70: 419A0008  beq cr6, 0x8251da78
	if ctx.cr[6].eq {
	pc = 0x8251DA78; continue 'dispatch;
	}
	// 8251DA74: 4BDA2E1D  bl 0x822c0890
	ctx.lr = 0x8251DA78;
	sub_822C0890(ctx, base);
	// 8251DA78: 83610088  lwz r27, 0x88(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8251DA7C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8251DA80: 419A034C  beq cr6, 0x8251ddcc
	if ctx.cr[6].eq {
	pc = 0x8251DDCC; continue 'dispatch;
	}
	// 8251DA84: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251DA88: 92410178  stw r18, 0x178(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[18].u32 ) };
	// 8251DA8C: 3956004E  addi r10, r22, 0x4e
	ctx.r[10].s64 = ctx.r[22].s64 + 78;
	// 8251DA90: 39360057  addi r9, r22, 0x57
	ctx.r[9].s64 = ctx.r[22].s64 + 87;
	// 8251DA94: 396BCD90  addi r11, r11, -0x3270
	ctx.r[11].s64 = ctx.r[11].s64 + -12912;
	// 8251DA98: 5559103A  slwi r25, r10, 2
	ctx.r[25].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 8251DA9C: 553A103A  slwi r26, r9, 2
	ctx.r[26].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 8251DAA0: 91610198  stw r11, 0x198(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), ctx.r[11].u32 ) };
	// 8251DAA4: 3B1F0004  addi r24, r31, 4
	ctx.r[24].s64 = ctx.r[31].s64 + 4;
	// 8251DAA8: 3AF20124  addi r23, r18, 0x124
	ctx.r[23].s64 = ctx.r[18].s64 + 292;
	// 8251DAAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251DAB0: 808100A8  lwz r4, 0xa8(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 8251DAB4: 38A000ED  li r5, 0xed
	ctx.r[5].s64 = 237;
	// 8251DAB8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8251DABC: 4BDA291D  bl 0x822c03d8
	ctx.lr = 0x8251DAC0;
	sub_822C03D8(ctx, base);
	// 8251DAC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251DAC4: 41820010  beq 0x8251dad4
	if ctx.cr[0].eq {
	pc = 0x8251DAD4; continue 'dispatch;
	}
	// 8251DAC8: 4BFFE921  bl 0x8251c3e8
	ctx.lr = 0x8251DACC;
	sub_8251C3E8(ctx, base);
	// 8251DACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251DAD0: 48000008  b 0x8251dad8
	pc = 0x8251DAD8; continue 'dispatch;
	// 8251DAD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251DAD8: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 8251DADC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251DAE0: 386100CC  addi r3, r1, 0xcc
	ctx.r[3].s64 = ctx.r[1].s64 + 204;
	// 8251DAE4: 4BFFEFA5  bl 0x8251ca88
	ctx.lr = 0x8251DAE8;
	sub_8251CA88(ctx, base);
	// 8251DAE8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251DAEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251DAF0: 386100CC  addi r3, r1, 0xcc
	ctx.r[3].s64 = ctx.r[1].s64 + 204;
	// 8251DAF4: 4BDA250D  bl 0x822c0000
	ctx.lr = 0x8251DAF8;
	sub_822C0000(ctx, base);
	// 8251DAF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251DAFC: 488D55F5  bl 0x82df30f0
	ctx.lr = 0x8251DB00;
	sub_82DF30F0(ctx, base);
	// 8251DB00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB04: 480BC3A5  bl 0x825d9ea8
	ctx.lr = 0x8251DB08;
	sub_825D9EA8(ctx, base);
	// 8251DB08: 83E100C8  lwz r31, 0xc8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 8251DB0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB10: 80810120  lwz r4, 0x120(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(288 as u32) ) } as u64;
	// 8251DB14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251DB18: 480BC2F9  bl 0x825d9e10
	ctx.lr = 0x8251DB1C;
	sub_825D9E10(ctx, base);
	// 8251DB1C: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 8251DB20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB24: 808100E0  lwz r4, 0xe0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 8251DB28: 480BC2E9  bl 0x825d9e10
	ctx.lr = 0x8251DB2C;
	sub_825D9E10(ctx, base);
	// 8251DB2C: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 8251DB30: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB34: 80810160  lwz r4, 0x160(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(352 as u32) ) } as u64;
	// 8251DB38: 480BC311  bl 0x825d9e48
	ctx.lr = 0x8251DB3C;
	sub_825D9E48(ctx, base);
	// 8251DB3C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8251DB40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB44: 80810128  lwz r4, 0x128(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(296 as u32) ) } as u64;
	// 8251DB48: 480BC2C9  bl 0x825d9e10
	ctx.lr = 0x8251DB4C;
	sub_825D9E10(ctx, base);
	// 8251DB4C: 38A10138  addi r5, r1, 0x138
	ctx.r[5].s64 = ctx.r[1].s64 + 312;
	// 8251DB50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB54: 80810148  lwz r4, 0x148(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) } as u64;
	// 8251DB58: 480BC2F1  bl 0x825d9e48
	ctx.lr = 0x8251DB5C;
	sub_825D9E48(ctx, base);
	// 8251DB5C: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8251DB60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB64: 80810130  lwz r4, 0x130(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 8251DB68: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251DB6C: 480BC2DD  bl 0x825d9e48
	ctx.lr = 0x8251DB70;
	sub_825D9E48(ctx, base);
	// 8251DB70: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 8251DB74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB78: 80810158  lwz r4, 0x158(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(344 as u32) ) } as u64;
	// 8251DB7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251DB80: 480BC2C9  bl 0x825d9e48
	ctx.lr = 0x8251DB84;
	sub_825D9E48(ctx, base);
	// 8251DB84: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 8251DB88: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8251DB8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251DB90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DB94: 480BC2B5  bl 0x825d9e48
	ctx.lr = 0x8251DB98;
	sub_825D9E48(ctx, base);
	// 8251DB98: 93E1017C  stw r31, 0x17c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), ctx.r[31].u32 ) };
	// 8251DB9C: E9610178  ld r11, 0x178(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(376 as u32) ) };
	// 8251DBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251DBA4: F961019C  std r11, 0x19c(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), ctx.r[11].u64 ) };
	// 8251DBA8: 38610230  addi r3, r1, 0x230
	ctx.r[3].s64 = ctx.r[1].s64 + 560;
	// 8251DBAC: 816101A0  lwz r11, 0x1a0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(416 as u32) ) } as u64;
	// 8251DBB0: E8810198  ld r4, 0x198(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(408 as u32) ) };
	// 8251DBB4: 796507E6  rldicr r5, r11, 0x20, 0x3f
	ctx.r[5].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 8251DBB8: 91410230  stw r10, 0x230(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8251DBBC: 4BFFF145  bl 0x8251cd00
	ctx.lr = 0x8251DBC0;
	sub_8251CD00(ctx, base);
	// 8251DBC0: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8251DBC4: 38A10230  addi r5, r1, 0x230
	ctx.r[5].s64 = ctx.r[1].s64 + 560;
	// 8251DBC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DBCC: 480BC315  bl 0x825d9ee0
	ctx.lr = 0x8251DBD0;
	sub_825D9EE0(ctx, base);
	// 8251DBD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8251DBD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DBD8: 480BC231  bl 0x825d9e08
	ctx.lr = 0x8251DBDC;
	sub_825D9E08(ctx, base);
	// 8251DBDC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8251DBE0: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 8251DBE4: 488D5E25  bl 0x82df3a08
	ctx.lr = 0x8251DBE8;
	sub_82DF3A08(ctx, base);
	// 8251DBE8: 388100AC  addi r4, r1, 0xac
	ctx.r[4].s64 = ctx.r[1].s64 + 172;
	// 8251DBEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251DBF0: 488D56B1  bl 0x82df32a0
	ctx.lr = 0x8251DBF4;
	sub_82DF32A0(ctx, base);
	// 8251DBF4: 7C6E1B78  mr r14, r3
	ctx.r[14].u64 = ctx.r[3].u64;
	// 8251DBF8: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 8251DBFC: 488D582D  bl 0x82df3428
	ctx.lr = 0x8251DC00;
	sub_82DF3428(ctx, base);
	// 8251DC00: 55CB063F  clrlwi. r11, r14, 0x18
	ctx.r[11].u64 = ctx.r[14].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251DC04: 41820028  beq 0x8251dc2c
	if ctx.cr[0].eq {
	pc = 0x8251DC2C; continue 'dispatch;
	}
	// 8251DC08: 81610188  lwz r11, 0x188(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(392 as u32) ) } as u64;
	// 8251DC0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251DC10: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251DC14: 409A0008  bne cr6, 0x8251dc1c
	if !ctx.cr[6].eq {
	pc = 0x8251DC1C; continue 'dispatch;
	}
	// 8251DC18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251DC1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8251DC20: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8251DC24: 4BFE3855  bl 0x82501478
	ctx.lr = 0x8251DC28;
	sub_82501478(ctx, base);
	// 8251DC28: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8251DC2C: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 8251DC30: 4BFCCE59  bl 0x824eaa88
	ctx.lr = 0x8251DC34;
	sub_824EAA88(ctx, base);
	// 8251DC34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DC38: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 8251DC3C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DC40: 4BFCD681  bl 0x824eb2c0
	ctx.lr = 0x8251DC44;
	sub_824EB2C0(ctx, base);
	// 8251DC44: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 8251DC48: 488D4049  bl 0x82df1c90
	ctx.lr = 0x8251DC4C;
	sub_82DF1C90(ctx, base);
	// 8251DC4C: 80610180  lwz r3, 0x180(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(384 as u32) ) } as u64;
	// 8251DC50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DC54: 419A0024  beq cr6, 0x8251dc78
	if ctx.cr[6].eq {
	pc = 0x8251DC78; continue 'dispatch;
	}
	// 8251DC58: 7E8B0034  cntlzw r11, r20
	ctx.r[11].u64 = if ctx.r[20].u32 == 0 { 32 } else { ctx.r[20].u32.leading_zeros() as u64 };
	// 8251DC5C: 80C10138  lwz r6, 0x138(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(312 as u32) ) } as u64;
	// 8251DC60: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8251DC64: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DC68: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8251DC6C: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 8251DC70: 4BFF9021  bl 0x82516c90
	ctx.lr = 0x8251DC74;
	sub_82516C90(ctx, base);
	// 8251DC74: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8251DC78: 386100BC  addi r3, r1, 0xbc
	ctx.r[3].s64 = ctx.r[1].s64 + 188;
	// 8251DC7C: 80810150  lwz r4, 0x150(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(336 as u32) ) } as u64;
	// 8251DC80: 488D5D89  bl 0x82df3a08
	ctx.lr = 0x8251DC84;
	sub_82DF3A08(ctx, base);
	// 8251DC84: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8251DC88: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8251DC8C: 3B8100BC  addi r28, r1, 0xbc
	ctx.r[28].s64 = ctx.r[1].s64 + 188;
	// 8251DC90: 480BB8D9  bl 0x825d9568
	ctx.lr = 0x8251DC94;
	sub_825D9568(ctx, base);
	// 8251DC94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251DC98: 488D5671  bl 0x82df3308
	ctx.lr = 0x8251DC9C;
	sub_82DF3308(ctx, base);
	// 8251DC9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251DCA0: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 8251DCA4: 488D5785  bl 0x82df3428
	ctx.lr = 0x8251DCA8;
	sub_82DF3428(ctx, base);
	// 8251DCA8: 386100BC  addi r3, r1, 0xbc
	ctx.r[3].s64 = ctx.r[1].s64 + 188;
	// 8251DCAC: 488D577D  bl 0x82df3428
	ctx.lr = 0x8251DCB0;
	sub_82DF3428(ctx, base);
	// 8251DCB0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251DCB4: 4182000C  beq 0x8251dcc0
	if ctx.cr[0].eq {
	pc = 0x8251DCC0; continue 'dispatch;
	}
	// 8251DCB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251DCBC: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8251DCC0: 388100C8  addi r4, r1, 0xc8
	ctx.r[4].s64 = ctx.r[1].s64 + 200;
	// 8251DCC4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8251DCC8: 48695441  bl 0x82bb3108
	ctx.lr = 0x8251DCCC;
	sub_82BB3108(ctx, base);
	// 8251DCCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251DCD0: 93E10194  stw r31, 0x194(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(404 as u32), ctx.r[31].u32 ) };
	// 8251DCD4: 38A10190  addi r5, r1, 0x190
	ctx.r[5].s64 = ctx.r[1].s64 + 400;
	// 8251DCD8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8251DCDC: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 8251DCE0: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 8251DCE4: 91610190  stw r11, 0x190(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 8251DCE8: 4BFFEEA1  bl 0x8251cb88
	ctx.lr = 0x8251DCEC;
	sub_8251CB88(ctx, base);
	// 8251DCEC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251DCF0: 38810168  addi r4, r1, 0x168
	ctx.r[4].s64 = ctx.r[1].s64 + 360;
	// 8251DCF4: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 8251DCF8: 4BE56419  bl 0x82374110
	ctx.lr = 0x8251DCFC;
	sub_82374110(ctx, base);
	// 8251DCFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD00: 8141016C  lwz r10, 0x16c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(364 as u32) ) } as u64;
	// 8251DD04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251DD08: 409A0014  bne cr6, 0x8251dd1c
	if !ctx.cr[6].eq {
	pc = 0x8251DD1C; continue 'dispatch;
	}
	// 8251DD0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251DD10: 38810168  addi r4, r1, 0x168
	ctx.r[4].s64 = ctx.r[1].s64 + 360;
	// 8251DD14: 38610220  addi r3, r1, 0x220
	ctx.r[3].s64 = ctx.r[1].s64 + 544;
	// 8251DD18: 48032C41  bl 0x82550958
	ctx.lr = 0x8251DD1C;
	sub_82550958(ctx, base);
	// 8251DD1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD20: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8251DD24: 81320130  lwz r9, 0x130(r18)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(304 as u32) ) } as u64;
	// 8251DD28: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 8251DD2C: 81520134  lwz r10, 0x134(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(308 as u32) ) } as u64;
	// 8251DD30: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8251DD34: 91720130  stw r11, 0x130(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 8251DD38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251DD40: 91720134  stw r11, 0x134(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 8251DD44: 7D7A902E  lwzx r11, r26, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 8251DD48: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD4C: 7D39902E  lwzx r9, r25, r18
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 8251DD50: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8251DD54: 7D59912E  stwx r10, r25, r18
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[18].u32), ctx.r[10].u32) };
	// 8251DD58: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD5C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251DD60: 7D7A912E  stwx r11, r26, r18
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[18].u32), ctx.r[11].u32) };
	// 8251DD64: 480BBD55  bl 0x825d9ab8
	ctx.lr = 0x8251DD68;
	sub_825D9AB8(ctx, base);
	// 8251DD68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DD6C: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 8251DD70: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DD74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DD78: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 8251DD7C: 4BDA66E5  bl 0x822c4460
	ctx.lr = 0x8251DD80;
	sub_822C4460(ctx, base);
	// 8251DD80: 806101F4  lwz r3, 0x1f4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(500 as u32) ) } as u64;
	// 8251DD84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DD88: 419A0008  beq cr6, 0x8251dd90
	if ctx.cr[6].eq {
	pc = 0x8251DD90; continue 'dispatch;
	}
	// 8251DD8C: 4BDA2B05  bl 0x822c0890
	ctx.lr = 0x8251DD90;
	sub_822C0890(ctx, base);
	// 8251DD90: 80610184  lwz r3, 0x184(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(388 as u32) ) } as u64;
	// 8251DD94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DD98: 419A0008  beq cr6, 0x8251dda0
	if ctx.cr[6].eq {
	pc = 0x8251DDA0; continue 'dispatch;
	}
	// 8251DD9C: 4BDA2AF5  bl 0x822c0890
	ctx.lr = 0x8251DDA0;
	sub_822C0890(ctx, base);
	// 8251DDA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251DDA4: 488D5685  bl 0x82df3428
	ctx.lr = 0x8251DDA8;
	sub_82DF3428(ctx, base);
	// 8251DDA8: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251DDAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DDB0: 419A0008  beq cr6, 0x8251ddb8
	if ctx.cr[6].eq {
	pc = 0x8251DDB8; continue 'dispatch;
	}
	// 8251DDB4: 4BDA2ADD  bl 0x822c0890
	ctx.lr = 0x8251DDB8;
	sub_822C0890(ctx, base);
	// 8251DDB8: 83610088  lwz r27, 0x88(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8251DDBC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8251DDC0: 409AFCEC  bne cr6, 0x8251daac
	if !ctx.cr[6].eq {
	pc = 0x8251DAAC; continue 'dispatch;
	}
	// 8251DDC4: 81C1030C  lwz r14, 0x30c(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(780 as u32) ) } as u64;
	// 8251DDC8: 83C10068  lwz r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8251DDCC: 2F140002  cmpwi cr6, r20, 2
	ctx.cr[6].compare_i32(ctx.r[20].s32, 2, &mut ctx.xer);
	// 8251DDD0: 40980020  bge cr6, 0x8251ddf0
	if !ctx.cr[6].lt {
	pc = 0x8251DDF0; continue 'dispatch;
	}
	// 8251DDD4: 56CB083C  slwi r11, r22, 1
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251DDD8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8251DDDC: 7D6BA214  add r11, r11, r20
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 8251DDE0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251DDE4: 7D6B9214  add r11, r11, r18
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 8251DDE8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8251DDEC: 4869531D  bl 0x82bb3108
	ctx.lr = 0x8251DDF0;
	sub_82BB3108(ctx, base);
	// 8251DDF0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8251DDF4: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 8251DDF8: 480BBCC1  bl 0x825d9ab8
	ctx.lr = 0x8251DDFC;
	sub_825D9AB8(ctx, base);
	// 8251DDFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DE00: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8251DE04: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DE08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE0C: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8251DE10: 4BDA6651  bl 0x822c4460
	ctx.lr = 0x8251DE14;
	sub_822C4460(ctx, base);
	// 8251DE14: 806101B4  lwz r3, 0x1b4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 8251DE18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DE1C: 419A0008  beq cr6, 0x8251de24
	if ctx.cr[6].eq {
	pc = 0x8251DE24; continue 'dispatch;
	}
	// 8251DE20: 4BDA2A71  bl 0x822c0890
	ctx.lr = 0x8251DE24;
	sub_822C0890(ctx, base);
	// 8251DE24: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8251DE28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DE2C: 419A0008  beq cr6, 0x8251de34
	if ctx.cr[6].eq {
	pc = 0x8251DE34; continue 'dispatch;
	}
	// 8251DE30: 4BDA2A61  bl 0x822c0890
	ctx.lr = 0x8251DE34;
	sub_822C0890(ctx, base);
	// 8251DE34: 82A10080  lwz r21, 0x80(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8251DE38: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8251DE3C: 409AFB88  bne cr6, 0x8251d9c4
	if !ctx.cr[6].eq {
	pc = 0x8251D9C4; continue 'dispatch;
	}
	// 8251DE40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251DE44: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 8251DE48: 480BBC71  bl 0x825d9ab8
	ctx.lr = 0x8251DE4C;
	sub_825D9AB8(ctx, base);
	// 8251DE4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DE50: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8251DE54: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DE58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE5C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8251DE60: 4BDA6601  bl 0x822c4460
	ctx.lr = 0x8251DE64;
	sub_822C4460(ctx, base);
	// 8251DE64: 806101BC  lwz r3, 0x1bc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(444 as u32) ) } as u64;
	// 8251DE68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DE6C: 419A0008  beq cr6, 0x8251de74
	if ctx.cr[6].eq {
	pc = 0x8251DE74; continue 'dispatch;
	}
	// 8251DE70: 4BDA2A21  bl 0x822c0890
	ctx.lr = 0x8251DE74;
	sub_822C0890(ctx, base);
	// 8251DE74: 83C10068  lwz r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8251DE78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251DE7C: 409AFA94  bne cr6, 0x8251d910
	if !ctx.cr[6].eq {
	pc = 0x8251D910; continue 'dispatch;
	}
	// 8251DE80: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 8251DE84: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251DE88: 480BBC31  bl 0x825d9ab8
	ctx.lr = 0x8251DE8C;
	sub_825D9AB8(ctx, base);
	// 8251DE8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DE90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251DE94: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DE98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE9C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251DEA0: 4BDA65C1  bl 0x822c4460
	ctx.lr = 0x8251DEA4;
	sub_822C4460(ctx, base);
	// 8251DEA4: 806101CC  lwz r3, 0x1cc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(460 as u32) ) } as u64;
	// 8251DEA8: 4800002C  b 0x8251ded4
	pc = 0x8251DED4; continue 'dispatch;
	// 8251DEAC: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 8251DEB0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251DEB4: 480BBC05  bl 0x825d9ab8
	ctx.lr = 0x8251DEB8;
	sub_825D9AB8(ctx, base);
	// 8251DEB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251DEBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8251DEC0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8251DEC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DEC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251DECC: 4BDA6595  bl 0x822c4460
	ctx.lr = 0x8251DED0;
	sub_822C4460(ctx, base);
	// 8251DED0: 806101DC  lwz r3, 0x1dc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(476 as u32) ) } as u64;
	// 8251DED4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DED8: 419A0008  beq cr6, 0x8251dee0
	if ctx.cr[6].eq {
	pc = 0x8251DEE0; continue 'dispatch;
	}
	// 8251DEDC: 4BDA29B5  bl 0x822c0890
	ctx.lr = 0x8251DEE0;
	sub_822C0890(ctx, base);
	// 8251DEE0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251DEE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251DEE8: 409AF754  bne cr6, 0x8251d63c
	if !ctx.cr[6].eq {
	pc = 0x8251D63C; continue 'dispatch;
	}
	// 8251DEEC: 83A1006C  lwz r29, 0x6c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8251DEF0: 83C10084  lwz r30, 0x84(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8251DEF4: 83E1008C  lwz r31, 0x8c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8251DEF8: 81610314  lwz r11, 0x314(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(788 as u32) ) } as u64;
	// 8251DEFC: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8251DF00: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DF04: 488E32DD  bl 0x82e011e0
	ctx.lr = 0x8251DF08;
	sub_82E011E0(ctx, base);
	// 8251DF08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251DF0C: 419A000C  beq cr6, 0x8251df18
	if ctx.cr[6].eq {
	pc = 0x8251DF18; continue 'dispatch;
	}
	// 8251DF10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251DF14: 4BDA297D  bl 0x822c0890
	ctx.lr = 0x8251DF18;
	sub_822C0890(ctx, base);
	// 8251DF18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251DF1C: 419A000C  beq cr6, 0x8251df28
	if ctx.cr[6].eq {
	pc = 0x8251DF28; continue 'dispatch;
	}
	// 8251DF20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251DF24: 4BDA296D  bl 0x822c0890
	ctx.lr = 0x8251DF28;
	sub_822C0890(ctx, base);
	// 8251DF28: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251DF2C: 419A000C  beq cr6, 0x8251df38
	if ctx.cr[6].eq {
	pc = 0x8251DF38; continue 'dispatch;
	}
	// 8251DF30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251DF34: 4BDA295D  bl 0x822c0890
	ctx.lr = 0x8251DF38;
	sub_822C0890(ctx, base);
	// 8251DF38: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 8251DF3C: 4BE02FB5  bl 0x82320ef0
	ctx.lr = 0x8251DF40;
	sub_82320EF0(ctx, base);
	// 8251DF40: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 8251DF44: 488D3D4D  bl 0x82df1c90
	ctx.lr = 0x8251DF48;
	sub_82DF1C90(ctx, base);
	// 8251DF48: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 8251DF4C: 488D54DD  bl 0x82df3428
	ctx.lr = 0x8251DF50;
	sub_82DF3428(ctx, base);
	// 8251DF50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251DF54: 480BBE75  bl 0x825d9dc8
	ctx.lr = 0x8251DF58;
	sub_825D9DC8(ctx, base);
	// 8251DF58: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251DF5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DF60: 419A0008  beq cr6, 0x8251df68
	if ctx.cr[6].eq {
	pc = 0x8251DF68; continue 'dispatch;
	}
	// 8251DF64: 4BDA292D  bl 0x822c0890
	ctx.lr = 0x8251DF68;
	sub_822C0890(ctx, base);
	// 8251DF68: 806100DC  lwz r3, 0xdc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 8251DF6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DF70: 419A0008  beq cr6, 0x8251df78
	if ctx.cr[6].eq {
	pc = 0x8251DF78; continue 'dispatch;
	}
	// 8251DF74: 4BDA291D  bl 0x822c0890
	ctx.lr = 0x8251DF78;
	sub_822C0890(ctx, base);
	// 8251DF78: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251DF7C: 480BB035  bl 0x825d8fb0
	ctx.lr = 0x8251DF80;
	sub_825D8FB0(ctx, base);
	// 8251DF80: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 8251DF84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251DF88: 419A0008  beq cr6, 0x8251df90
	if ctx.cr[6].eq {
	pc = 0x8251DF90; continue 'dispatch;
	}
	// 8251DF8C: 4BDA2905  bl 0x822c0890
	ctx.lr = 0x8251DF90;
	sub_822C0890(ctx, base);
	// 8251DF90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251DF94: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 8251DF98: 48C8A1E8  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8251DFA0 size=12
    let mut pc: u32 = 0x8251DFA0;
    'dispatch: loop {
        match pc {
            0x8251DFA0 => {
    //   block [0x8251DFA0..0x8251DFAC)
	// 8251DFA0: C003011C  lfs f0, 0x11c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251DFA4: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8251DFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251DFB0 size=52
    let mut pc: u32 = 0x8251DFB0;
    'dispatch: loop {
        match pc {
            0x8251DFB0 => {
    //   block [0x8251DFB0..0x8251DFE4)
	// 8251DFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251DFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251DFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251DFBC: 4BFD523D  bl 0x824f31f8
	ctx.lr = 0x8251DFC0;
	sub_824F31F8(ctx, base);
	// 8251DFC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251DFC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251DFC8: 40820008  bne 0x8251dfd0
	if !ctx.cr[0].eq {
	pc = 0x8251DFD0; continue 'dispatch;
	}
	// 8251DFCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251DFD0: 4BFD5239  bl 0x824f3208
	ctx.lr = 0x8251DFD4;
	sub_824F3208(ctx, base);
	// 8251DFD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251DFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251DFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251DFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251DFE8 size=72
    let mut pc: u32 = 0x8251DFE8;
    'dispatch: loop {
        match pc {
            0x8251DFE8 => {
    //   block [0x8251DFE8..0x8251E030)
	// 8251DFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251DFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251DFF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251DFF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251DFF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251DFFC: 4BFF30F5  bl 0x825110f0
	ctx.lr = 0x8251E000;
	sub_825110F0(ctx, base);
	// 8251E000: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8251E008: 396B26C4  addi r11, r11, 0x26c4
	ctx.r[11].s64 = ctx.r[11].s64 + 9924;
	// 8251E00C: 394A26B0  addi r10, r10, 0x26b0
	ctx.r[10].s64 = ctx.r[10].s64 + 9904;
	// 8251E010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E018: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8251E01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251E020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E030 size=104
    let mut pc: u32 = 0x8251E030;
    'dispatch: loop {
        match pc {
            0x8251E030 => {
    //   block [0x8251E030..0x8251E098)
	// 8251E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E038: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E03C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E044: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251E048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E04C: 3BE30028  addi r31, r3, 0x28
	ctx.r[31].s64 = ctx.r[3].s64 + 40;
	// 8251E050: 409A0008  bne cr6, 0x8251e058
	if !ctx.cr[6].eq {
	pc = 0x8251E058; continue 'dispatch;
	}
	// 8251E054: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251E058: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E05C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E060: 808BD068  lwz r4, -0x2f98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12184 as u32) ) } as u64;
	// 8251E064: 488D59A5  bl 0x82df3a08
	ctx.lr = 0x8251E068;
	sub_82DF3A08(ctx, base);
	// 8251E068: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251E06C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251E070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E074: 4BFEA77D  bl 0x825087f0
	ctx.lr = 0x8251E078;
	sub_825087F0(ctx, base);
	// 8251E078: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E07C: 488D53AD  bl 0x82df3428
	ctx.lr = 0x8251E080;
	sub_82DF3428(ctx, base);
	// 8251E080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E08C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E090: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E098 size=136
    let mut pc: u32 = 0x8251E098;
    'dispatch: loop {
        match pc {
            0x8251E098 => {
    //   block [0x8251E098..0x8251E120)
	// 8251E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E0AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E0B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E0B4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8251E0B8: 409A0020  bne cr6, 0x8251e0d8
	if !ctx.cr[6].eq {
	pc = 0x8251E0D8; continue 'dispatch;
	}
	// 8251E0BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251E0C0: 419A0048  beq cr6, 0x8251e108
	if ctx.cr[6].eq {
	pc = 0x8251E108; continue 'dispatch;
	}
	// 8251E0C4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8251E0C8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8251E0CC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 8251E0D0: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251E0D4: 48000034  b 0x8251e108
	pc = 0x8251E108; continue 'dispatch;
	// 8251E0D8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8251E0DC: 419A002C  beq cr6, 0x8251e108
	if ctx.cr[6].eq {
	pc = 0x8251E108; continue 'dispatch;
	}
	// 8251E0E0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E0E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E0E8: 388BD598  addi r4, r11, -0x2a68
	ctx.r[4].s64 = ctx.r[11].s64 + -10856;
	// 8251E0EC: 48C8A00D  bl 0x831a80f8
	ctx.lr = 0x8251E0F0;
	sub_831A80F8(ctx, base);
	// 8251E0F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E0F4: 4182000C  beq 0x8251e100
	if ctx.cr[0].eq {
	pc = 0x8251E100; continue 'dispatch;
	}
	// 8251E0F8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8251E0FC: 4800000C  b 0x8251e108
	pc = 0x8251E108; continue 'dispatch;
	// 8251E100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E104: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E108: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E114: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E120 size=196
    let mut pc: u32 = 0x8251E120;
    'dispatch: loop {
        match pc {
            0x8251E120 => {
    //   block [0x8251E120..0x8251E1E4)
	// 8251E120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E12C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E13C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251E140: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E144: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E148: 4BDA27F1  bl 0x822c0938
	ctx.lr = 0x8251E14C;
	sub_822C0938(ctx, base);
	// 8251E14C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E150: 41820028  beq 0x8251e178
	if ctx.cr[0].eq {
	pc = 0x8251E178; continue 'dispatch;
	}
	// 8251E154: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E158: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251E15C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251E160: 392B26F0  addi r9, r11, 0x26f0
	ctx.r[9].s64 = ctx.r[11].s64 + 9968;
	// 8251E164: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251E168: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E16C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251E170: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251E174: 48000008  b 0x8251e17c
	pc = 0x8251E17C; continue 'dispatch;
	// 8251E178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E17C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251E184: 409A0044  bne cr6, 0x8251e1c8
	if !ctx.cr[6].eq {
	pc = 0x8251E1C8; continue 'dispatch;
	}
	// 8251E188: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251E18C: 419A001C  beq cr6, 0x8251e1a8
	if ctx.cr[6].eq {
	pc = 0x8251E1A8; continue 'dispatch;
	}
	// 8251E190: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E194: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251E198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E19C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E1A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251E1A4: 4E800421  bctrl
	ctx.lr = 0x8251E1A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251E1A8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E1AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251E1B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E1B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251E1B8: 816BD3F4  lwz r11, -0x2c0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11276 as u32) ) } as u64;
	// 8251E1BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251E1C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251E1C4: 4BDA1E3D  bl 0x822c0000
	ctx.lr = 0x8251E1C8;
	sub_822C0000(ctx, base);
	// 8251E1C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E1CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E1D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E1DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E1E8 size=196
    let mut pc: u32 = 0x8251E1E8;
    'dispatch: loop {
        match pc {
            0x8251E1E8 => {
    //   block [0x8251E1E8..0x8251E2AC)
	// 8251E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E1F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E1F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E1F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E1FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E204: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251E208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E20C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E210: 4BDA2729  bl 0x822c0938
	ctx.lr = 0x8251E214;
	sub_822C0938(ctx, base);
	// 8251E214: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E218: 41820028  beq 0x8251e240
	if ctx.cr[0].eq {
	pc = 0x8251E240; continue 'dispatch;
	}
	// 8251E21C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E220: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251E224: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251E228: 392B2704  addi r9, r11, 0x2704
	ctx.r[9].s64 = ctx.r[11].s64 + 9988;
	// 8251E22C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251E230: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E234: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251E238: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251E23C: 48000008  b 0x8251e244
	pc = 0x8251E244; continue 'dispatch;
	// 8251E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E244: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251E24C: 409A0044  bne cr6, 0x8251e290
	if !ctx.cr[6].eq {
	pc = 0x8251E290; continue 'dispatch;
	}
	// 8251E250: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251E254: 419A001C  beq cr6, 0x8251e270
	if ctx.cr[6].eq {
	pc = 0x8251E270; continue 'dispatch;
	}
	// 8251E258: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E25C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251E260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E264: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251E26C: 4E800421  bctrl
	ctx.lr = 0x8251E270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251E270: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E274: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251E278: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E27C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251E280: 816BD3F4  lwz r11, -0x2c0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11276 as u32) ) } as u64;
	// 8251E284: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251E288: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251E28C: 4BDA1D75  bl 0x822c0000
	ctx.lr = 0x8251E290;
	sub_822C0000(ctx, base);
	// 8251E290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E2A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E2A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E2B0 size=196
    let mut pc: u32 = 0x8251E2B0;
    'dispatch: loop {
        match pc {
            0x8251E2B0 => {
    //   block [0x8251E2B0..0x8251E374)
	// 8251E2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E2C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E2CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8251E2D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E2D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E2D8: 4BDA2661  bl 0x822c0938
	ctx.lr = 0x8251E2DC;
	sub_822C0938(ctx, base);
	// 8251E2DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E2E0: 41820028  beq 0x8251e308
	if ctx.cr[0].eq {
	pc = 0x8251E308; continue 'dispatch;
	}
	// 8251E2E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E2E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251E2EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251E2F0: 392B2718  addi r9, r11, 0x2718
	ctx.r[9].s64 = ctx.r[11].s64 + 10008;
	// 8251E2F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251E2F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E2FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251E300: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251E304: 48000008  b 0x8251e30c
	pc = 0x8251E30C; continue 'dispatch;
	// 8251E308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E30C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251E314: 409A0044  bne cr6, 0x8251e358
	if !ctx.cr[6].eq {
	pc = 0x8251E358; continue 'dispatch;
	}
	// 8251E318: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251E31C: 419A001C  beq cr6, 0x8251e338
	if ctx.cr[6].eq {
	pc = 0x8251E338; continue 'dispatch;
	}
	// 8251E320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E324: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251E328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E32C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251E334: 4E800421  bctrl
	ctx.lr = 0x8251E338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251E338: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E33C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251E340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E344: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8251E348: 816BD3F4  lwz r11, -0x2c0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11276 as u32) ) } as u64;
	// 8251E34C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251E350: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251E354: 4BDA1CAD  bl 0x822c0000
	ctx.lr = 0x8251E358;
	sub_822C0000(ctx, base);
	// 8251E358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E35C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E368: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E378 size=428
    let mut pc: u32 = 0x8251E378;
    'dispatch: loop {
        match pc {
            0x8251E378 => {
    //   block [0x8251E378..0x8251E524)
	// 8251E378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E37C: 48C89DED  bl 0x831a8168
	ctx.lr = 0x8251E380;
	sub_831A8130(ctx, base);
	// 8251E380: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E384: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E388: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E38C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251E390: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251E394: 4BFF0F9D  bl 0x8250f330
	ctx.lr = 0x8251E398;
	sub_8250F330(ctx, base);
	// 8251E398: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E39C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E3A0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E3A4: 4BFCCF1D  bl 0x824eb2c0
	ctx.lr = 0x8251E3A8;
	sub_824EB2C0(ctx, base);
	// 8251E3A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251E3AC: 488D38E5  bl 0x82df1c90
	ctx.lr = 0x8251E3B0;
	sub_82DF1C90(ctx, base);
	// 8251E3B0: 57FC063E  clrlwi r28, r31, 0x18
	ctx.r[28].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8251E3B4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E3B8: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8251E3BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E3C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8251E3C4: 697D0001  xori r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u64 ^ 1;
	// 8251E3C8: 419A0144  beq cr6, 0x8251e50c
	if ctx.cr[6].eq {
	pc = 0x8251E50C; continue 'dispatch;
	}
	// 8251E3CC: 1D7D003C  mulli r11, r29, 0x3c
	ctx.r[11].s64 = ctx.r[29].s64 * 60;
	// 8251E3D0: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8251E3D4: 809F0128  lwz r4, 0x128(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 8251E3D8: 4BFF8829  bl 0x82516c00
	ctx.lr = 0x8251E3DC;
	sub_82516C00(ctx, base);
	// 8251E3DC: 397D0005  addi r11, r29, 5
	ctx.r[11].s64 = ctx.r[29].s64 + 5;
	// 8251E3E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E3E4: 1FAB003C  mulli r29, r11, 0x3c
	ctx.r[29].s64 = ctx.r[11].s64 * 60;
	// 8251E3E8: 7C9DF02E  lwzx r4, r29, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8251E3EC: 4BFF8595  bl 0x82516980
	ctx.lr = 0x8251E3F0;
	sub_82516980(ctx, base);
	// 8251E3F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E3F4: 809F0130  lwz r4, 0x130(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8251E3F8: 4BFF85D9  bl 0x825169d0
	ctx.lr = 0x8251E3FC;
	sub_825169D0(ctx, base);
	// 8251E3FC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E400: 809F0134  lwz r4, 0x134(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8251E404: 4BFF85FD  bl 0x82516a00
	ctx.lr = 0x8251E408;
	sub_82516A00(ctx, base);
	// 8251E408: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E40C: 809F0138  lwz r4, 0x138(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 8251E410: 4BFF8641  bl 0x82516a50
	ctx.lr = 0x8251E414;
	sub_82516A50(ctx, base);
	// 8251E414: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E418: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 8251E41C: 4BFF8665  bl 0x82516a80
	ctx.lr = 0x8251E420;
	sub_82516A80(ctx, base);
	// 8251E420: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E424: 809F0140  lwz r4, 0x140(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 8251E428: 4BFF86A9  bl 0x82516ad0
	ctx.lr = 0x8251E42C;
	sub_82516AD0(ctx, base);
	// 8251E42C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E430: 809F0144  lwz r4, 0x144(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8251E434: 4BFF86CD  bl 0x82516b00
	ctx.lr = 0x8251E438;
	sub_82516B00(ctx, base);
	// 8251E438: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E43C: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8251E440: 4BFF8711  bl 0x82516b50
	ctx.lr = 0x8251E444;
	sub_82516B50(ctx, base);
	// 8251E444: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E448: 809F014C  lwz r4, 0x14c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8251E44C: 4BFF8735  bl 0x82516b80
	ctx.lr = 0x8251E450;
	sub_82516B80(ctx, base);
	// 8251E450: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E454: 809F0150  lwz r4, 0x150(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8251E458: 4BFF8779  bl 0x82516bd0
	ctx.lr = 0x8251E45C;
	sub_82516BD0(ctx, base);
	// 8251E45C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E460: 809F0154  lwz r4, 0x154(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8251E464: 4BFF845D  bl 0x825168c0
	ctx.lr = 0x8251E468;
	sub_825168C0(ctx, base);
	// 8251E468: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E46C: 809F0158  lwz r4, 0x158(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8251E470: 4BFF8481  bl 0x825168f0
	ctx.lr = 0x8251E474;
	sub_825168F0(ctx, base);
	// 8251E474: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E478: 809F015C  lwz r4, 0x15c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8251E47C: 4BFF84A5  bl 0x82516920
	ctx.lr = 0x8251E480;
	sub_82516920(ctx, base);
	// 8251E480: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E484: 809F0160  lwz r4, 0x160(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 8251E488: 4BFF84C9  bl 0x82516950
	ctx.lr = 0x8251E48C;
	sub_82516950(ctx, base);
	// 8251E48C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8251E490: 409A007C  bne cr6, 0x8251e50c
	if !ctx.cr[6].eq {
	pc = 0x8251E50C; continue 'dispatch;
	}
	// 8251E494: 817F0128  lwz r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 8251E498: 917E0164  stw r11, 0x164(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 8251E49C: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8251E4A0: 917E0168  stw r11, 0x168(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 8251E4A4: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8251E4A8: 917E016C  stw r11, 0x16c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 8251E4AC: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8251E4B0: 917E0170  stw r11, 0x170(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(368 as u32), ctx.r[11].u32 ) };
	// 8251E4B4: 817F0138  lwz r11, 0x138(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 8251E4B8: 917E0174  stw r11, 0x174(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 8251E4BC: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 8251E4C0: 917E0178  stw r11, 0x178(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 8251E4C4: 817F0140  lwz r11, 0x140(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 8251E4C8: 917E017C  stw r11, 0x17c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(380 as u32), ctx.r[11].u32 ) };
	// 8251E4CC: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 8251E4D0: 917E0180  stw r11, 0x180(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 8251E4D4: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8251E4D8: 917E0184  stw r11, 0x184(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 8251E4DC: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8251E4E0: 917E0188  stw r11, 0x188(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 8251E4E4: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8251E4E8: 917E018C  stw r11, 0x18c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
	// 8251E4EC: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8251E4F0: 917E0190  stw r11, 0x190(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 8251E4F4: 817F0158  lwz r11, 0x158(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8251E4F8: 917E0194  stw r11, 0x194(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(404 as u32), ctx.r[11].u32 ) };
	// 8251E4FC: 817F015C  lwz r11, 0x15c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8251E500: 917E0198  stw r11, 0x198(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(408 as u32), ctx.r[11].u32 ) };
	// 8251E504: 817F0160  lwz r11, 0x160(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 8251E508: 917E019C  stw r11, 0x19c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(412 as u32), ctx.r[11].u32 ) };
	// 8251E50C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251E510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E514: 419A0008  beq cr6, 0x8251e51c
	if ctx.cr[6].eq {
	pc = 0x8251E51C; continue 'dispatch;
	}
	// 8251E518: 4BDA2379  bl 0x822c0890
	ctx.lr = 0x8251E51C;
	sub_822C0890(ctx, base);
	// 8251E51C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251E520: 48C89C98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E528 size=68
    let mut pc: u32 = 0x8251E528;
    'dispatch: loop {
        match pc {
            0x8251E528 => {
    //   block [0x8251E528..0x8251E56C)
	// 8251E528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E538: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251E53C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E540: 4BFFFE39  bl 0x8251e378
	ctx.lr = 0x8251E544;
	sub_8251E378(ctx, base);
	// 8251E544: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E548: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 8251E54C: 917F01A0  stw r11, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[11].u32 ) };
	// 8251E550: 917F01A4  stw r11, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 8251E554: 4895EE1D  bl 0x82e7d370
	ctx.lr = 0x8251E558;
	sub_82E7D370(ctx, base);
	// 8251E558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251E55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E570 size=320
    let mut pc: u32 = 0x8251E570;
    'dispatch: loop {
        match pc {
            0x8251E570 => {
    //   block [0x8251E570..0x8251E6B0)
	// 8251E570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E57C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251E58C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E590: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251E594: 4BFF0D9D  bl 0x8250f330
	ctx.lr = 0x8251E598;
	sub_8250F330(ctx, base);
	// 8251E598: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E59C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E5A0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E5A4: 4BFCCD1D  bl 0x824eb2c0
	ctx.lr = 0x8251E5A8;
	sub_824EB2C0(ctx, base);
	// 8251E5A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251E5AC: 488D36E5  bl 0x82df1c90
	ctx.lr = 0x8251E5B0;
	sub_82DF1C90(ctx, base);
	// 8251E5B0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E5B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E5B8: 419A00D0  beq cr6, 0x8251e688
	if ctx.cr[6].eq {
	pc = 0x8251E688; continue 'dispatch;
	}
	// 8251E5BC: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251E5C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E5C4: 41820014  beq 0x8251e5d8
	if ctx.cr[0].eq {
	pc = 0x8251E5D8; continue 'dispatch;
	}
	// 8251E5C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251E5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E5D0: 4BFFFDA9  bl 0x8251e378
	ctx.lr = 0x8251E5D4;
	sub_8251E378(ctx, base);
	// 8251E5D4: 480000B4  b 0x8251e688
	pc = 0x8251E688; continue 'dispatch;
	// 8251E5D8: 4BFF86A1  bl 0x82516c78
	ctx.lr = 0x8251E5DC;
	sub_82516C78(ctx, base);
	// 8251E5DC: 907F0164  stw r3, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[3].u32 ) };
	// 8251E5E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E5E4: 4BFF83D5  bl 0x825169b8
	ctx.lr = 0x8251E5E8;
	sub_825169B8(ctx, base);
	// 8251E5E8: 907F0168  stw r3, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[3].u32 ) };
	// 8251E5EC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E5F0: 4BFF83F9  bl 0x825169e8
	ctx.lr = 0x8251E5F4;
	sub_825169E8(ctx, base);
	// 8251E5F4: 907F016C  stw r3, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[3].u32 ) };
	// 8251E5F8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E5FC: 4BFF843D  bl 0x82516a38
	ctx.lr = 0x8251E600;
	sub_82516A38(ctx, base);
	// 8251E600: 907F0170  stw r3, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[3].u32 ) };
	// 8251E604: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E608: 4BFF8461  bl 0x82516a68
	ctx.lr = 0x8251E60C;
	sub_82516A68(ctx, base);
	// 8251E60C: 907F0174  stw r3, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[3].u32 ) };
	// 8251E610: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E614: 4BFF84A5  bl 0x82516ab8
	ctx.lr = 0x8251E618;
	sub_82516AB8(ctx, base);
	// 8251E618: 907F0178  stw r3, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[3].u32 ) };
	// 8251E61C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E620: 4BFF84C9  bl 0x82516ae8
	ctx.lr = 0x8251E624;
	sub_82516AE8(ctx, base);
	// 8251E624: 907F017C  stw r3, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[3].u32 ) };
	// 8251E628: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E62C: 4BFF850D  bl 0x82516b38
	ctx.lr = 0x8251E630;
	sub_82516B38(ctx, base);
	// 8251E630: 907F0180  stw r3, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[3].u32 ) };
	// 8251E634: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E638: 4BFF8531  bl 0x82516b68
	ctx.lr = 0x8251E63C;
	sub_82516B68(ctx, base);
	// 8251E63C: 907F0184  stw r3, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[3].u32 ) };
	// 8251E640: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E644: 4BFF8575  bl 0x82516bb8
	ctx.lr = 0x8251E648;
	sub_82516BB8(ctx, base);
	// 8251E648: 907F0188  stw r3, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[3].u32 ) };
	// 8251E64C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E650: 4BFF8599  bl 0x82516be8
	ctx.lr = 0x8251E654;
	sub_82516BE8(ctx, base);
	// 8251E654: 907F018C  stw r3, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[3].u32 ) };
	// 8251E658: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E65C: 4BFF827D  bl 0x825168d8
	ctx.lr = 0x8251E660;
	sub_825168D8(ctx, base);
	// 8251E660: 907F0190  stw r3, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[3].u32 ) };
	// 8251E664: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E668: 4BFF82A1  bl 0x82516908
	ctx.lr = 0x8251E66C;
	sub_82516908(ctx, base);
	// 8251E66C: 907F0194  stw r3, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[3].u32 ) };
	// 8251E670: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E674: 4BFF82C5  bl 0x82516938
	ctx.lr = 0x8251E678;
	sub_82516938(ctx, base);
	// 8251E678: 907F0198  stw r3, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[3].u32 ) };
	// 8251E67C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E680: 4BFF82E9  bl 0x82516968
	ctx.lr = 0x8251E684;
	sub_82516968(ctx, base);
	// 8251E684: 907F019C  stw r3, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[3].u32 ) };
	// 8251E688: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251E68C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E690: 419A0008  beq cr6, 0x8251e698
	if ctx.cr[6].eq {
	pc = 0x8251E698; continue 'dispatch;
	}
	// 8251E694: 4BDA21FD  bl 0x822c0890
	ctx.lr = 0x8251E698;
	sub_822C0890(ctx, base);
	// 8251E698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E6A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E6A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E6B0 size=72
    let mut pc: u32 = 0x8251E6B0;
    'dispatch: loop {
        match pc {
            0x8251E6B0 => {
    //   block [0x8251E6B0..0x8251E6F8)
	// 8251E6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E6BC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 8251E6C0: 419A001C  beq cr6, 0x8251e6dc
	if ctx.cr[6].eq {
	pc = 0x8251E6DC; continue 'dispatch;
	}
	// 8251E6C4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251E6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251E6CC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8251E6D0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8251E6D4: 4BFFF9C5  bl 0x8251e098
	ctx.lr = 0x8251E6D8;
	sub_8251E098(ctx, base);
	// 8251E6D8: 48000010  b 0x8251e6e8
	pc = 0x8251E6E8; continue 'dispatch;
	// 8251E6DC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E6E0: 396BD598  addi r11, r11, -0x2a68
	ctx.r[11].s64 = ctx.r[11].s64 + -10856;
	// 8251E6E4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E6E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251E6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E6F8 size=276
    let mut pc: u32 = 0x8251E6F8;
    'dispatch: loop {
        match pc {
            0x8251E6F8 => {
    //   block [0x8251E6F8..0x8251E80C)
	// 8251E6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E6FC: 48C89A6D  bl 0x831a8168
	ctx.lr = 0x8251E700;
	sub_831A8130(ctx, base);
	// 8251E700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E704: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8251E708: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251E70C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E710: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8251E714: 41820038  beq 0x8251e74c
	if ctx.cr[0].eq {
	pc = 0x8251E74C; continue 'dispatch;
	}
	// 8251E718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E71C: 48C8B26D  bl 0x831a9988
	ctx.lr = 0x8251E720;
	sub_831A9988(ctx, base);
	// 8251E720: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8251E724: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251E728: 386B5A88  addi r3, r11, 0x5a88
	ctx.r[3].s64 = ctx.r[11].s64 + 23176;
	// 8251E72C: 48C899CD  bl 0x831a80f8
	ctx.lr = 0x8251E730;
	sub_831A80F8(ctx, base);
	// 8251E730: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E734: 41820018  beq 0x8251e74c
	if ctx.cr[0].eq {
	pc = 0x8251E74C; continue 'dispatch;
	}
	// 8251E738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E73C: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251E740: 4BFFFDE9  bl 0x8251e528
	ctx.lr = 0x8251E744;
	sub_8251E528(ctx, base);
	// 8251E744: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251E748: 480000BC  b 0x8251e804
	pc = 0x8251E804; continue 'dispatch;
	// 8251E74C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251E750: 419A00A4  beq cr6, 0x8251e7f4
	if ctx.cr[6].eq {
	pc = 0x8251E7F4; continue 'dispatch;
	}
	// 8251E754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E758: 48C8B231  bl 0x831a9988
	ctx.lr = 0x8251E75C;
	sub_831A9988(ctx, base);
	// 8251E75C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E760: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251E764: 386BD630  addi r3, r11, -0x29d0
	ctx.r[3].s64 = ctx.r[11].s64 + -10704;
	// 8251E768: 48C89991  bl 0x831a80f8
	ctx.lr = 0x8251E76C;
	sub_831A80F8(ctx, base);
	// 8251E76C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E770: 41820014  beq 0x8251e784
	if ctx.cr[0].eq {
	pc = 0x8251E784; continue 'dispatch;
	}
	// 8251E774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E778: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251E77C: 4BFFF825  bl 0x8251dfa0
	ctx.lr = 0x8251E780;
	sub_8251DFA0(ctx, base);
	// 8251E780: 4BFFFFC4  b 0x8251e744
	pc = 0x8251E744; continue 'dispatch;
	// 8251E784: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251E788: 419A006C  beq cr6, 0x8251e7f4
	if ctx.cr[6].eq {
	pc = 0x8251E7F4; continue 'dispatch;
	}
	// 8251E78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E790: 48C8B1F9  bl 0x831a9988
	ctx.lr = 0x8251E794;
	sub_831A9988(ctx, base);
	// 8251E794: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251E79C: 386BD750  addi r3, r11, -0x28b0
	ctx.r[3].s64 = ctx.r[11].s64 + -10416;
	// 8251E7A0: 48C89959  bl 0x831a80f8
	ctx.lr = 0x8251E7A4;
	sub_831A80F8(ctx, base);
	// 8251E7A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E7A8: 41820014  beq 0x8251e7bc
	if ctx.cr[0].eq {
	pc = 0x8251E7BC; continue 'dispatch;
	}
	// 8251E7AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E7B0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251E7B4: 4BFFF7FD  bl 0x8251dfb0
	ctx.lr = 0x8251E7B8;
	sub_8251DFB0(ctx, base);
	// 8251E7B8: 4BFFFF8C  b 0x8251e744
	pc = 0x8251E744; continue 'dispatch;
	// 8251E7BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251E7C0: 419A0034  beq cr6, 0x8251e7f4
	if ctx.cr[6].eq {
	pc = 0x8251E7F4; continue 'dispatch;
	}
	// 8251E7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E7C8: 48C8B1C1  bl 0x831a9988
	ctx.lr = 0x8251E7CC;
	sub_831A9988(ctx, base);
	// 8251E7CC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251E7D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251E7D4: 386BD720  addi r3, r11, -0x28e0
	ctx.r[3].s64 = ctx.r[11].s64 + -10464;
	// 8251E7D8: 48C89921  bl 0x831a80f8
	ctx.lr = 0x8251E7DC;
	sub_831A80F8(ctx, base);
	// 8251E7DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E7E0: 41820014  beq 0x8251e7f4
	if ctx.cr[0].eq {
	pc = 0x8251E7F4; continue 'dispatch;
	}
	// 8251E7E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E7E8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251E7EC: 4BFFFD85  bl 0x8251e570
	ctx.lr = 0x8251E7F0;
	sub_8251E570(ctx, base);
	// 8251E7F0: 4BFFFF54  b 0x8251e744
	pc = 0x8251E744; continue 'dispatch;
	// 8251E7F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251E7F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E800: 4BFF2D99  bl 0x82511598
	ctx.lr = 0x8251E804;
	sub_82511598(ctx, base);
	// 8251E804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251E808: 48C899B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E810 size=112
    let mut pc: u32 = 0x8251E810;
    'dispatch: loop {
        match pc {
            0x8251E810 => {
    //   block [0x8251E810..0x8251E880)
	// 8251E810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E814: 48C89959  bl 0x831a816c
	ctx.lr = 0x8251E818;
	sub_831A8130(ctx, base);
	// 8251E818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E81C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E820: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251E824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251E828: 388B2734  addi r4, r11, 0x2734
	ctx.r[4].s64 = ctx.r[11].s64 + 10036;
	// 8251E82C: 38A001CC  li r5, 0x1cc
	ctx.r[5].s64 = 460;
	// 8251E830: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 8251E834: 488D3BB5  bl 0x82df23e8
	ctx.lr = 0x8251E838;
	sub_82DF23E8(ctx, base);
	// 8251E838: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E83C: 41820010  beq 0x8251e84c
	if ctx.cr[0].eq {
	pc = 0x8251E84C; continue 'dispatch;
	}
	// 8251E840: 4BFFF7A9  bl 0x8251dfe8
	ctx.lr = 0x8251E844;
	sub_8251DFE8(ctx, base);
	// 8251E844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E848: 48000008  b 0x8251e850
	pc = 0x8251E850; continue 'dispatch;
	// 8251E84C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251E850: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8251E854: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8251E858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E85C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E860: 4BFFF8C1  bl 0x8251e120
	ctx.lr = 0x8251E864;
	sub_8251E120(ctx, base);
	// 8251E864: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251E868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E86C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E870: 4BDA1791  bl 0x822c0000
	ctx.lr = 0x8251E874;
	sub_822C0000(ctx, base);
	// 8251E874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251E878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E87C: 48C89940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E880 size=120
    let mut pc: u32 = 0x8251E880;
    'dispatch: loop {
        match pc {
            0x8251E880 => {
    //   block [0x8251E880..0x8251E8F8)
	// 8251E880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E884: 48C898E9  bl 0x831a816c
	ctx.lr = 0x8251E888;
	sub_831A8130(ctx, base);
	// 8251E888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E88C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251E890: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251E894: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251E898: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251E89C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8251E8A0: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8251E8A4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8251E8A8: 488D3B41  bl 0x82df23e8
	ctx.lr = 0x8251E8AC;
	sub_82DF23E8(ctx, base);
	// 8251E8AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251E8B0: 41820014  beq 0x8251e8c4
	if ctx.cr[0].eq {
	pc = 0x8251E8C4; continue 'dispatch;
	}
	// 8251E8B4: 889F0000  lbz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E8B8: 486357C9  bl 0x82b54080
	ctx.lr = 0x8251E8BC;
	sub_82B54080(ctx, base);
	// 8251E8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E8C0: 48000008  b 0x8251e8c8
	pc = 0x8251E8C8; continue 'dispatch;
	// 8251E8C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8251E8C8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8251E8CC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8251E8D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E8D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E8D8: 4BFFF9D9  bl 0x8251e2b0
	ctx.lr = 0x8251E8DC;
	sub_8251E2B0(ctx, base);
	// 8251E8DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251E8E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251E8E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251E8E8: 4BDA1719  bl 0x822c0000
	ctx.lr = 0x8251E8EC;
	sub_822C0000(ctx, base);
	// 8251E8EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251E8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E8F4: 48C898C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E8F8 size=308
    let mut pc: u32 = 0x8251E8F8;
    'dispatch: loop {
        match pc {
            0x8251E8F8 => {
    //   block [0x8251E8F8..0x8251EA2C)
	// 8251E8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E908: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E90C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E910: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E914: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251E918: 4BDB3469  bl 0x822d1d80
	ctx.lr = 0x8251E91C;
	sub_822D1D80(ctx, base);
	// 8251E91C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E924: 3BCB2690  addi r30, r11, 0x2690
	ctx.r[30].s64 = ctx.r[11].s64 + 9872;
	// 8251E928: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251E92C: 488D50DD  bl 0x82df3a08
	ctx.lr = 0x8251E930;
	sub_82DF3A08(ctx, base);
	// 8251E930: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251E934: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251E938: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251E93C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8251E940: 4BDB35C9  bl 0x822d1f08
	ctx.lr = 0x8251E944;
	sub_822D1F08(ctx, base);
	// 8251E944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E948: 488D4AE1  bl 0x82df3428
	ctx.lr = 0x8251E94C;
	sub_82DF3428(ctx, base);
	// 8251E94C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251E950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251E954: 419A0098  beq cr6, 0x8251e9ec
	if ctx.cr[6].eq {
	pc = 0x8251E9EC; continue 'dispatch;
	}
	// 8251E958: 488DFE89  bl 0x82dfe7e0
	ctx.lr = 0x8251E95C;
	sub_82DFE7E0(ctx, base);
	// 8251E95C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251E960: 40820064  bne 0x8251e9c4
	if !ctx.cr[0].eq {
	pc = 0x8251E9C4; continue 'dispatch;
	}
	// 8251E964: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251E968: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251E96C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251E970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251E974: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8251E978: 419A0024  beq cr6, 0x8251e99c
	if ctx.cr[6].eq {
	pc = 0x8251E99C; continue 'dispatch;
	}
	// 8251E97C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251E980: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251E984: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251E988: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251E98C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251E990: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251E994: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251E998: 4082FFE8  bne 0x8251e980
	if !ctx.cr[0].eq {
	pc = 0x8251E980; continue 'dispatch;
	}
	// 8251E99C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251E9A0: 4BFCC0E9  bl 0x824eaa88
	ctx.lr = 0x8251E9A4;
	sub_824EAA88(ctx, base);
	// 8251E9A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251E9A8: 38C000C9  li r6, 0xc9
	ctx.r[6].s64 = 201;
	// 8251E9AC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E9B0: 38AB2734  addi r5, r11, 0x2734
	ctx.r[5].s64 = ctx.r[11].s64 + 10036;
	// 8251E9B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251E9B8: 4869B7C9  bl 0x82bba180
	ctx.lr = 0x8251E9BC;
	sub_82BBA180(ctx, base);
	// 8251E9BC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251E9C0: 488D32D1  bl 0x82df1c90
	ctx.lr = 0x8251E9C4;
	sub_82DF1C90(ctx, base);
	// 8251E9C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251E9C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E9CC: 488D503D  bl 0x82df3a08
	ctx.lr = 0x8251E9D0;
	sub_82DF3A08(ctx, base);
	// 8251E9D0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8251E9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E9D8: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8251E9DC: 4BDB1015  bl 0x822cf9f0
	ctx.lr = 0x8251E9E0;
	sub_822CF9F0(ctx, base);
	// 8251E9E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251E9E4: 488D4A45  bl 0x82df3428
	ctx.lr = 0x8251E9E8;
	sub_82DF3428(ctx, base);
	// 8251E9E8: 48000010  b 0x8251e9f8
	pc = 0x8251E9F8; continue 'dispatch;
	// 8251E9EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E9F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251E9F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251E9F8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251E9FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251EA00: 419A0008  beq cr6, 0x8251ea08
	if ctx.cr[6].eq {
	pc = 0x8251EA08; continue 'dispatch;
	}
	// 8251EA04: 4BDA1E8D  bl 0x822c0890
	ctx.lr = 0x8251EA08;
	sub_822C0890(ctx, base);
	// 8251EA08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251EA0C: 4BDB338D  bl 0x822d1d98
	ctx.lr = 0x8251EA10;
	sub_822D1D98(ctx, base);
	// 8251EA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251EA14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251EA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251EA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251EA20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251EA24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251EA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251EA30 size=312
    let mut pc: u32 = 0x8251EA30;
    'dispatch: loop {
        match pc {
            0x8251EA30 => {
    //   block [0x8251EA30..0x8251EB68)
	// 8251EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251EA38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251EA3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251EA40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EA44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251EA48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251EA4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251EA50: 4BFF0A79  bl 0x8250f4c8
	ctx.lr = 0x8251EA54;
	sub_8250F4C8(ctx, base);
	// 8251EA54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EA58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EA5C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251EA60: 409A0008  bne cr6, 0x8251ea68
	if !ctx.cr[6].eq {
	pc = 0x8251EA68; continue 'dispatch;
	}
	// 8251EA64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251EA68: 4BFE9FF1  bl 0x82508a58
	ctx.lr = 0x8251EA6C;
	sub_82508A58(ctx, base);
	// 8251EA6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251EA70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251EA74: 488D321D  bl 0x82df1c90
	ctx.lr = 0x8251EA78;
	sub_82DF1C90(ctx, base);
	// 8251EA78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251EA7C: 419A0038  beq cr6, 0x8251eab4
	if ctx.cr[6].eq {
	pc = 0x8251EAB4; continue 'dispatch;
	}
	// 8251EA80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251EA84: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251EA88: 4BFF0A41  bl 0x8250f4c8
	ctx.lr = 0x8251EA8C;
	sub_8250F4C8(ctx, base);
	// 8251EA8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EA90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EA94: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251EA98: 409A0008  bne cr6, 0x8251eaa0
	if !ctx.cr[6].eq {
	pc = 0x8251EAA0; continue 'dispatch;
	}
	// 8251EA9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251EAA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251EAA4: 4BFE9F75  bl 0x82508a18
	ctx.lr = 0x8251EAA8;
	sub_82508A18(ctx, base);
	// 8251EAA8: 907F00CC  stw r3, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[3].u32 ) };
	// 8251EAAC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251EAB0: 488D31E1  bl 0x82df1c90
	ctx.lr = 0x8251EAB4;
	sub_82DF1C90(ctx, base);
	// 8251EAB4: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251EAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EABC: 419A0094  beq cr6, 0x8251eb50
	if ctx.cr[6].eq {
	pc = 0x8251EB50; continue 'dispatch;
	}
	// 8251EAC0: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 8251EAC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251EAC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251EACC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251EAD0: 4BDFE701  bl 0x8231d1d0
	ctx.lr = 0x8251EAD4;
	sub_8231D1D0(ctx, base);
	// 8251EAD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EAD8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251EADC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EAE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EAE4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251EAE8: 419A0024  beq cr6, 0x8251eb0c
	if ctx.cr[6].eq {
	pc = 0x8251EB0C; continue 'dispatch;
	}
	// 8251EAEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251EAF0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251EAF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251EAF8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251EAFC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251EB00: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251EB04: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251EB08: 4082FFE8  bne 0x8251eaf0
	if !ctx.cr[0].eq {
	pc = 0x8251EAF0; continue 'dispatch;
	}
	// 8251EB0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251EB10: 80DF00CC  lwz r6, 0xcc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8251EB14: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8251EB18: 388B2734  addi r4, r11, 0x2734
	ctx.r[4].s64 = ctx.r[11].s64 + 10036;
	// 8251EB1C: 38A00109  li r5, 0x109
	ctx.r[5].s64 = 265;
	// 8251EB20: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8251EB24: 489384C5  bl 0x82e56fe8
	ctx.lr = 0x8251EB28;
	sub_82E56FE8(ctx, base);
	// 8251EB28: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251EB2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251EB30: 419A0008  beq cr6, 0x8251eb38
	if ctx.cr[6].eq {
	pc = 0x8251EB38; continue 'dispatch;
	}
	// 8251EB34: 4BDA1D5D  bl 0x822c0890
	ctx.lr = 0x8251EB38;
	sub_822C0890(ctx, base);
	// 8251EB38: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8251EB3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251EB40: 419A0008  beq cr6, 0x8251eb48
	if ctx.cr[6].eq {
	pc = 0x8251EB48; continue 'dispatch;
	}
	// 8251EB44: 4BDA1D4D  bl 0x822c0890
	ctx.lr = 0x8251EB48;
	sub_822C0890(ctx, base);
	// 8251EB48: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8251EB4C: 488E79D5  bl 0x82e06520
	ctx.lr = 0x8251EB50;
	sub_82E06520(ctx, base);
	// 8251EB50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8251EB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251EB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251EB5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251EB60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251EB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251EB68 size=428
    let mut pc: u32 = 0x8251EB68;
    'dispatch: loop {
        match pc {
            0x8251EB68 => {
    //   block [0x8251EB68..0x8251ED14)
	// 8251EB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EB6C: 48C895FD  bl 0x831a8168
	ctx.lr = 0x8251EB70;
	sub_831A8130(ctx, base);
	// 8251EB70: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8251EB74: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EB78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251EB7C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251EB80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251EB84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251EB88: 4BFF0941  bl 0x8250f4c8
	ctx.lr = 0x8251EB8C;
	sub_8250F4C8(ctx, base);
	// 8251EB8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EB90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EB94: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251EB98: 409A0008  bne cr6, 0x8251eba0
	if !ctx.cr[6].eq {
	pc = 0x8251EBA0; continue 'dispatch;
	}
	// 8251EB9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251EBA0: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8251EBA4: 4BFE9ADD  bl 0x82508680
	ctx.lr = 0x8251EBA8;
	sub_82508680(ctx, base);
	// 8251EBA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251EBAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251EBB0: 488D30E1  bl 0x82df1c90
	ctx.lr = 0x8251EBB4;
	sub_82DF1C90(ctx, base);
	// 8251EBB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251EBB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251EBBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251EBC0: 3BCB2734  addi r30, r11, 0x2734
	ctx.r[30].s64 = ctx.r[11].s64 + 10036;
	// 8251EBC4: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251EBC8: 419A0084  beq cr6, 0x8251ec4c
	if ctx.cr[6].eq {
	pc = 0x8251EC4C; continue 'dispatch;
	}
	// 8251EBCC: 389C0018  addi r4, r28, 0x18
	ctx.r[4].s64 = ctx.r[28].s64 + 24;
	// 8251EBD0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8251EBD4: 4BFFFCAD  bl 0x8251e880
	ctx.lr = 0x8251EBD8;
	sub_8251E880(ctx, base);
	// 8251EBD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EBDC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251EBE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EBE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EBE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251EBEC: 419A0024  beq cr6, 0x8251ec10
	if ctx.cr[6].eq {
	pc = 0x8251EC10; continue 'dispatch;
	}
	// 8251EBF0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251EBF4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251EBF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251EBFC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251EC00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251EC04: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251EC08: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251EC0C: 4082FFE8  bne 0x8251ebf4
	if !ctx.cr[0].eq {
	pc = 0x8251EBF4; continue 'dispatch;
	}
	// 8251EC10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251EC14: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8251EC18: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251EC1C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8251EC20: 38A001E5  li r5, 0x1e5
	ctx.r[5].s64 = 485;
	// 8251EC24: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 8251EC28: 48939E19  bl 0x82e58a40
	ctx.lr = 0x8251EC2C;
	sub_82E58A40(ctx, base);
	// 8251EC2C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251EC30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251EC34: 419A0008  beq cr6, 0x8251ec3c
	if ctx.cr[6].eq {
	pc = 0x8251EC3C; continue 'dispatch;
	}
	// 8251EC38: 4BDA1C59  bl 0x822c0890
	ctx.lr = 0x8251EC3C;
	sub_822C0890(ctx, base);
	// 8251EC3C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8251EC40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251EC44: 419A0008  beq cr6, 0x8251ec4c
	if ctx.cr[6].eq {
	pc = 0x8251EC4C; continue 'dispatch;
	}
	// 8251EC48: 4BDA1C49  bl 0x822c0890
	ctx.lr = 0x8251EC4C;
	sub_822C0890(ctx, base);
	// 8251EC4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251EC50: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251EC54: 4BFF0875  bl 0x8250f4c8
	ctx.lr = 0x8251EC58;
	sub_8250F4C8(ctx, base);
	// 8251EC58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EC5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251EC60: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251EC64: 409A0008  bne cr6, 0x8251ec6c
	if !ctx.cr[6].eq {
	pc = 0x8251EC6C; continue 'dispatch;
	}
	// 8251EC68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251EC6C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8251EC70: 4BFE9A11  bl 0x82508680
	ctx.lr = 0x8251EC74;
	sub_82508680(ctx, base);
	// 8251EC74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251EC78: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251EC7C: 488D3015  bl 0x82df1c90
	ctx.lr = 0x8251EC80;
	sub_82DF1C90(ctx, base);
	// 8251EC80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251EC84: 419A0084  beq cr6, 0x8251ed08
	if ctx.cr[6].eq {
	pc = 0x8251ED08; continue 'dispatch;
	}
	// 8251EC88: 389C0018  addi r4, r28, 0x18
	ctx.r[4].s64 = ctx.r[28].s64 + 24;
	// 8251EC8C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8251EC90: 4BFFFBF1  bl 0x8251e880
	ctx.lr = 0x8251EC94;
	sub_8251E880(ctx, base);
	// 8251EC94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EC98: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8251EC9C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251ECA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251ECA4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251ECA8: 419A0024  beq cr6, 0x8251eccc
	if ctx.cr[6].eq {
	pc = 0x8251ECCC; continue 'dispatch;
	}
	// 8251ECAC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251ECB0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251ECB4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251ECB8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251ECBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251ECC0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251ECC4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251ECC8: 4082FFE8  bne 0x8251ecb0
	if !ctx.cr[0].eq {
	pc = 0x8251ECB0; continue 'dispatch;
	}
	// 8251ECCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251ECD0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8251ECD4: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8251ECD8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8251ECDC: 38A001E9  li r5, 0x1e9
	ctx.r[5].s64 = 489;
	// 8251ECE0: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 8251ECE4: 48939D5D  bl 0x82e58a40
	ctx.lr = 0x8251ECE8;
	sub_82E58A40(ctx, base);
	// 8251ECE8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251ECEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ECF0: 419A0008  beq cr6, 0x8251ecf8
	if ctx.cr[6].eq {
	pc = 0x8251ECF8; continue 'dispatch;
	}
	// 8251ECF4: 4BDA1B9D  bl 0x822c0890
	ctx.lr = 0x8251ECF8;
	sub_822C0890(ctx, base);
	// 8251ECF8: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8251ECFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251ED00: 419A0008  beq cr6, 0x8251ed08
	if ctx.cr[6].eq {
	pc = 0x8251ED08; continue 'dispatch;
	}
	// 8251ED04: 4BDA1B8D  bl 0x822c0890
	ctx.lr = 0x8251ED08;
	sub_822C0890(ctx, base);
	// 8251ED08: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8251ED0C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8251ED10: 48C894A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251ED18 size=108
    let mut pc: u32 = 0x8251ED18;
    'dispatch: loop {
        match pc {
            0x8251ED18 => {
    //   block [0x8251ED18..0x8251ED84)
	// 8251ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251ED1C: 48C89451  bl 0x831a816c
	ctx.lr = 0x8251ED20;
	sub_831A8130(ctx, base);
	// 8251ED20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251ED24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251ED28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251ED2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251ED30: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251ED34: 41820038  beq 0x8251ed6c
	if ctx.cr[0].eq {
	pc = 0x8251ED6C; continue 'dispatch;
	}
	// 8251ED38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251ED3C: 48C8AC4D  bl 0x831a9988
	ctx.lr = 0x8251ED40;
	sub_831A9988(ctx, base);
	// 8251ED40: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251ED44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251ED48: 386BD788  addi r3, r11, -0x2878
	ctx.r[3].s64 = ctx.r[11].s64 + -10360;
	// 8251ED4C: 48C893AD  bl 0x831a80f8
	ctx.lr = 0x8251ED50;
	sub_831A80F8(ctx, base);
	// 8251ED50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251ED54: 41820018  beq 0x8251ed6c
	if ctx.cr[0].eq {
	pc = 0x8251ED6C; continue 'dispatch;
	}
	// 8251ED58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251ED5C: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 8251ED60: 4BFFFE09  bl 0x8251eb68
	ctx.lr = 0x8251ED64;
	sub_8251EB68(ctx, base);
	// 8251ED64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251ED68: 48000014  b 0x8251ed7c
	pc = 0x8251ED7C; continue 'dispatch;
	// 8251ED6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251ED70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251ED74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251ED78: 4BFF2821  bl 0x82511598
	ctx.lr = 0x8251ED7C;
	sub_82511598(ctx, base);
	// 8251ED7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251ED80: 48C8943C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251ED88 size=108
    let mut pc: u32 = 0x8251ED88;
    'dispatch: loop {
        match pc {
            0x8251ED88 => {
    //   block [0x8251ED88..0x8251EDF4)
	// 8251ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251ED8C: 48C893DD  bl 0x831a8168
	ctx.lr = 0x8251ED90;
	sub_831A8130(ctx, base);
	// 8251ED90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251ED94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251ED98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251ED9C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EDA0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EDA4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251EDA8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EDAC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251EDB0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EDB4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251EDB8: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251EDBC: 419A0030  beq cr6, 0x8251edec
	if ctx.cr[6].eq {
	pc = 0x8251EDEC; continue 'dispatch;
	}
	// 8251EDC0: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 8251EDC4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8251EDC8: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EDCC: 4BDA9EED  bl 0x822c8cb8
	ctx.lr = 0x8251EDD0;
	sub_822C8CB8(ctx, base);
	// 8251EDD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251EDD4: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8251EDD8: 488D33B1  bl 0x82df2188
	ctx.lr = 0x8251EDDC;
	sub_82DF2188(ctx, base);
	// 8251EDDC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EDE0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8251EDE4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251EDE8: 409AFFDC  bne cr6, 0x8251edc4
	if !ctx.cr[6].eq {
	pc = 0x8251EDC4; continue 'dispatch;
	}
	// 8251EDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251EDF0: 48C893C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251EDF8 size=156
    let mut pc: u32 = 0x8251EDF8;
    'dispatch: loop {
        match pc {
            0x8251EDF8 => {
    //   block [0x8251EDF8..0x8251EE94)
	// 8251EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251EE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251EE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251EE08: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251EE10: 3D6007FF  lis r11, 0x7ff
	ctx.r[11].s64 = 134152192;
	// 8251EE14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251EE18: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8251EE1C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251EE20: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8251EE24: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8251EE28: 40980048  bge cr6, 0x8251ee70
	if !ctx.cr[6].lt {
	pc = 0x8251EE70; continue 'dispatch;
	}
	// 8251EE2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251EE30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251EE34: 388B960C  addi r4, r11, -0x69f4
	ctx.r[4].s64 = ctx.r[11].s64 + -27124;
	// 8251EE38: 4BDA6A91  bl 0x822c58c8
	ctx.lr = 0x8251EE3C;
	sub_822C58C8(ctx, base);
	// 8251EE3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251EE40: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251EE44: 4BDA69D5  bl 0x822c5818
	ctx.lr = 0x8251EE48;
	sub_822C5818(ctx, base);
	// 8251EE48: 4BDA5469  bl 0x822c42b0
	ctx.lr = 0x8251EE4C;
	sub_822C42B0(ctx, base);
	// 8251EE4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251EE50: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251EE54: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 8251EE58: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8251EE5C: 4BDA6615  bl 0x822c5470
	ctx.lr = 0x8251EE60;
	sub_822C5470(ctx, base);
	// 8251EE60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8251EE64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251EE68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251EE6C: 4BDA5E75  bl 0x822c4ce0
	ctx.lr = 0x8251EE70;
	sub_822C4CE0(ctx, base);
	// 8251EE70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251EE74: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8251EE78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251EE7C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8251EE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251EE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251EE88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251EE8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251EE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


