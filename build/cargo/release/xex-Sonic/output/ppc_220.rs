pub fn sub_82F4C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C070 size=28
    let mut pc: u32 = 0x82F4C070;
    'dispatch: loop {
        match pc {
            0x82F4C070 => {
    //   block [0x82F4C070..0x82F4C08C)
	// 82F4C070: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F4C074: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 82F4C078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C07C: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F4C080: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F4C084: 4200FFF8  bdnz 0x82f4c07c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F4C07C; continue 'dispatch;
	}
	// 82F4C088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C090 size=12
    let mut pc: u32 = 0x82F4C090;
    'dispatch: loop {
        match pc {
            0x82F4C090 => {
    //   block [0x82F4C090..0x82F4C09C)
	// 82F4C090: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4C094: 7CAB192E  stwx r5, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[5].u32) };
	// 82F4C098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C0A0 size=32
    let mut pc: u32 = 0x82F4C0A0;
    'dispatch: loop {
        match pc {
            0x82F4C0A0 => {
    //   block [0x82F4C0A0..0x82F4C0C0)
	// 82F4C0A0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F4C0A4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82F4C0A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F4C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F4C0B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4C0B4: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82F4C0B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4C0BC: 4BF556FC  b 0x82ea17b8
	sub_82EA17B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C0C0 size=128
    let mut pc: u32 = 0x82F4C0C0;
    'dispatch: loop {
        match pc {
            0x82F4C0C0 => {
    //   block [0x82F4C0C0..0x82F4C140)
	// 82F4C0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C0C4: 4825C0A1  bl 0x831a8164
	ctx.lr = 0x82F4C0C8;
	sub_831A8130(ctx, base);
	// 82F4C0C8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C0CC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F4C0D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F4C0D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F4C0D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F4C0DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F4C0E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4C0E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F4C0E8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82F4C0EC: 4BF55FAD  bl 0x82ea2098
	ctx.lr = 0x82F4C0F0;
	sub_82EA2098(ctx, base);
	// 82F4C0F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4C0F4: 409A0040  bne cr6, 0x82f4c134
	if !ctx.cr[6].eq {
	pc = 0x82F4C134; continue 'dispatch;
	}
	// 82F4C0F8: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4C0FC: 2B0B0064  cmplwi cr6, r11, 0x64
	ctx.cr[6].compare_u32(ctx.r[11].u32, 100 as u32, &mut ctx.xer);
	// 82F4C100: 41990024  bgt cr6, 0x82f4c124
	if ctx.cr[6].gt {
	pc = 0x82F4C124; continue 'dispatch;
	}
	// 82F4C104: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4C108: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4C10C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4C110: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F4C114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C118: 7D4BE82E  lwzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F4C11C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C120: 4E800421  bctrl
	ctx.lr = 0x82F4C124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4C124: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4C128: 419AFFD0  beq cr6, 0x82f4c0f8
	if ctx.cr[6].eq {
	pc = 0x82F4C0F8; continue 'dispatch;
	}
	// 82F4C12C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82F4C130: 4825C084  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F4C134: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F4C138: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82F4C13C: 4825C078  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C140 size=20
    let mut pc: u32 = 0x82F4C140;
    'dispatch: loop {
        match pc {
            0x82F4C140 => {
    //   block [0x82F4C140..0x82F4C154)
	// 82F4C140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C148: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C14C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C150: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C158 size=8
    let mut pc: u32 = 0x82F4C158;
    'dispatch: loop {
        match pc {
            0x82F4C158 => {
    //   block [0x82F4C158..0x82F4C160)
	// 82F4C158: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C15C: 48000084  b 0x82f4c1e0
	sub_82F4C1E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C160 size=8
    let mut pc: u32 = 0x82F4C160;
    'dispatch: loop {
        match pc {
            0x82F4C160 => {
    //   block [0x82F4C160..0x82F4C168)
	// 82F4C160: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C164: 4800007C  b 0x82f4c1e0
	sub_82F4C1E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C168 size=4
    let mut pc: u32 = 0x82F4C168;
    'dispatch: loop {
        match pc {
            0x82F4C168 => {
    //   block [0x82F4C168..0x82F4C16C)
	// 82F4C168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C170 size=4
    let mut pc: u32 = 0x82F4C170;
    'dispatch: loop {
        match pc {
            0x82F4C170 => {
    //   block [0x82F4C170..0x82F4C174)
	// 82F4C170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C178 size=8
    let mut pc: u32 = 0x82F4C178;
    'dispatch: loop {
        match pc {
            0x82F4C178 => {
    //   block [0x82F4C178..0x82F4C180)
	// 82F4C178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C17C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C180 size=28
    let mut pc: u32 = 0x82F4C180;
    'dispatch: loop {
        match pc {
            0x82F4C180 => {
    //   block [0x82F4C180..0x82F4C19C)
	// 82F4C180: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C184: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4C188: 394B02E8  addi r10, r11, 0x2e8
	ctx.r[10].s64 = ctx.r[11].s64 + 744;
	// 82F4C18C: B0830006  sth r4, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 82F4C190: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F4C194: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82F4C198: 4BFA13E0  b 0x82eed578
	sub_82EED578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C19C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C19C size=4
    let mut pc: u32 = 0x82F4C19C;
    'dispatch: loop {
        match pc {
            0x82F4C19C => {
    //   block [0x82F4C19C..0x82F4C1A0)
	// 82F4C19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C1A0 size=64
    let mut pc: u32 = 0x82F4C1A0;
    'dispatch: loop {
        match pc {
            0x82F4C1A0 => {
    //   block [0x82F4C1A0..0x82F4C1E0)
	// 82F4C1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C1A8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C1AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C1B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C1B4: 392B02E8  addi r9, r11, 0x2e8
	ctx.r[9].s64 = ctx.r[11].s64 + 744;
	// 82F4C1B8: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82F4C1BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C1C0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F4C1C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F4C1C8: 4BFA13B1  bl 0x82eed578
	ctx.lr = 0x82F4C1CC;
	sub_82EED578(ctx, base);
	// 82F4C1CC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4C1D0: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82F4C1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C1E0 size=160
    let mut pc: u32 = 0x82F4C1E0;
    'dispatch: loop {
        match pc {
            0x82F4C1E0 => {
    //   block [0x82F4C1E0..0x82F4C280)
	// 82F4C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C1E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C1EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C1F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C1F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C1F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C1FC: 48007E1D  bl 0x82f54018
	ctx.lr = 0x82F4C200;
	sub_82F54018(ctx, base);
	// 82F4C200: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C204: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C208: 419A005C  beq cr6, 0x82f4c264
	if ctx.cr[6].eq {
	pc = 0x82F4C264; continue 'dispatch;
	}
	// 82F4C20C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F4C210: 419A0054  beq cr6, 0x82f4c264
	if ctx.cr[6].eq {
	pc = 0x82F4C264; continue 'dispatch;
	}
	// 82F4C214: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C218: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C21C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C220: 812B0054  lwz r9, 0x54(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F4C224: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4C228: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F4C22C: 4198001C  blt cr6, 0x82f4c248
	if ctx.cr[6].lt {
	pc = 0x82F4C248; continue 'dispatch;
	}
	// 82F4C230: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C234: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4C238: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82F4C23C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4C240: 4BF53E21  bl 0x82ea0060
	ctx.lr = 0x82F4C244;
	sub_82EA0060(ctx, base);
	// 82F4C244: 48000020  b 0x82f4c264
	pc = 0x82F4C264; continue 'dispatch;
	// 82F4C248: 812B0054  lwz r9, 0x54(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F4C24C: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
	// 82F4C250: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4C254: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F4C258: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82F4C25C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F4C260: 93EB0050  stw r31, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82F4C264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C268: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C274: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C278: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C280 size=100
    let mut pc: u32 = 0x82F4C280;
    'dispatch: loop {
        match pc {
            0x82F4C280 => {
    //   block [0x82F4C280..0x82F4C2E4)
	// 82F4C280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C29C: 48007F0D  bl 0x82f541a8
	ctx.lr = 0x82F4C2A0;
	sub_82F541A8(ctx, base);
	// 82F4C2A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C2A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C2A8: 419A0020  beq cr6, 0x82f4c2c8
	if ctx.cr[6].eq {
	pc = 0x82F4C2C8; continue 'dispatch;
	}
	// 82F4C2AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C2B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C2B4: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C2B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C2BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4C2C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C2C4: 4BF544ED  bl 0x82ea07b0
	ctx.lr = 0x82F4C2C8;
	sub_82EA07B0(ctx, base);
	// 82F4C2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C2CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C2D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C2DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C2E8 size=4
    let mut pc: u32 = 0x82F4C2E8;
    'dispatch: loop {
        match pc {
            0x82F4C2E8 => {
    //   block [0x82F4C2E8..0x82F4C2EC)
	// 82F4C2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C2F0 size=20
    let mut pc: u32 = 0x82F4C2F0;
    'dispatch: loop {
        match pc {
            0x82F4C2F0 => {
    //   block [0x82F4C2F0..0x82F4C304)
	// 82F4C2F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C2F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C2F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C2FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C300: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C308 size=8
    let mut pc: u32 = 0x82F4C308;
    'dispatch: loop {
        match pc {
            0x82F4C308 => {
    //   block [0x82F4C308..0x82F4C310)
	// 82F4C308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C30C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C310 size=24
    let mut pc: u32 = 0x82F4C310;
    'dispatch: loop {
        match pc {
            0x82F4C310 => {
    //   block [0x82F4C310..0x82F4C328)
	// 82F4C310: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C318: 392B0404  addi r9, r11, 0x404
	ctx.r[9].s64 = ctx.r[11].s64 + 1028;
	// 82F4C31C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4C320: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C328 size=12
    let mut pc: u32 = 0x82F4C328;
    'dispatch: loop {
        match pc {
            0x82F4C328 => {
    //   block [0x82F4C328..0x82F4C334)
	// 82F4C328: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C32C: 386B0404  addi r3, r11, 0x404
	ctx.r[3].s64 = ctx.r[11].s64 + 1028;
	// 82F4C330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C338 size=100
    let mut pc: u32 = 0x82F4C338;
    'dispatch: loop {
        match pc {
            0x82F4C338 => {
    //   block [0x82F4C338..0x82F4C39C)
	// 82F4C338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C350: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C354: 48007F45  bl 0x82f54298
	ctx.lr = 0x82F4C358;
	sub_82F54298(ctx, base);
	// 82F4C358: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C35C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C360: 419A0020  beq cr6, 0x82f4c380
	if ctx.cr[6].eq {
	pc = 0x82F4C380; continue 'dispatch;
	}
	// 82F4C364: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C368: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C36C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C370: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C374: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4C378: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C37C: 4BF54435  bl 0x82ea07b0
	ctx.lr = 0x82F4C380;
	sub_82EA07B0(ctx, base);
	// 82F4C380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C3A0 size=4
    let mut pc: u32 = 0x82F4C3A0;
    'dispatch: loop {
        match pc {
            0x82F4C3A0 => {
    //   block [0x82F4C3A0..0x82F4C3A4)
	// 82F4C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C3A8 size=4
    let mut pc: u32 = 0x82F4C3A8;
    'dispatch: loop {
        match pc {
            0x82F4C3A8 => {
    //   block [0x82F4C3A8..0x82F4C3AC)
	// 82F4C3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C3B0 size=4
    let mut pc: u32 = 0x82F4C3B0;
    'dispatch: loop {
        match pc {
            0x82F4C3B0 => {
    //   block [0x82F4C3B0..0x82F4C3B4)
	// 82F4C3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C3B8 size=8
    let mut pc: u32 = 0x82F4C3B8;
    'dispatch: loop {
        match pc {
            0x82F4C3B8 => {
    //   block [0x82F4C3B8..0x82F4C3C0)
	// 82F4C3B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C3BC: 4800008C  b 0x82f4c448
	sub_82F4C448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C3C0 size=8
    let mut pc: u32 = 0x82F4C3C0;
    'dispatch: loop {
        match pc {
            0x82F4C3C0 => {
    //   block [0x82F4C3C0..0x82F4C3C8)
	// 82F4C3C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C3C4: 48000144  b 0x82f4c508
	sub_82F4C508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C3C8 size=128
    let mut pc: u32 = 0x82F4C3C8;
    'dispatch: loop {
        match pc {
            0x82F4C3C8 => {
    //   block [0x82F4C3C8..0x82F4C448)
	// 82F4C3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C3DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4C3E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4C3E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C3E8: 409A0020  bne cr6, 0x82f4c408
	if !ctx.cr[6].eq {
	pc = 0x82F4C408; continue 'dispatch;
	}
	// 82F4C3EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C3F0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4C3F4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4C3F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4C3FC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4C400: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4C404: 4BF543AD  bl 0x82ea07b0
	ctx.lr = 0x82F4C408;
	sub_82EA07B0(ctx, base);
	// 82F4C408: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4C40C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4C410: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C414: 409A0020  bne cr6, 0x82f4c434
	if !ctx.cr[6].eq {
	pc = 0x82F4C434; continue 'dispatch;
	}
	// 82F4C418: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C41C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4C420: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4C424: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C428: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4C42C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4C430: 4BF54381  bl 0x82ea07b0
	ctx.lr = 0x82F4C434;
	sub_82EA07B0(ctx, base);
	// 82F4C434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4C438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C448 size=192
    let mut pc: u32 = 0x82F4C448;
    'dispatch: loop {
        match pc {
            0x82F4C448 => {
    //   block [0x82F4C448..0x82F4C508)
	// 82F4C448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C45C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C460: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C464: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4C468: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4C46C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C470: 409A0020  bne cr6, 0x82f4c490
	if !ctx.cr[6].eq {
	pc = 0x82F4C490; continue 'dispatch;
	}
	// 82F4C474: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C478: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4C47C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4C480: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C484: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4C488: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4C48C: 4BF54325  bl 0x82ea07b0
	ctx.lr = 0x82F4C490;
	sub_82EA07B0(ctx, base);
	// 82F4C490: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C498: 419A0054  beq cr6, 0x82f4c4ec
	if ctx.cr[6].eq {
	pc = 0x82F4C4EC; continue 'dispatch;
	}
	// 82F4C49C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C4A0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C4A4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C4A8: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F4C4AC: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4C4B0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F4C4B4: 4198001C  blt cr6, 0x82f4c4d0
	if ctx.cr[6].lt {
	pc = 0x82F4C4D0; continue 'dispatch;
	}
	// 82F4C4B8: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C4BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4C4C0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82F4C4C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4C4C8: 4BF53B99  bl 0x82ea0060
	ctx.lr = 0x82F4C4CC;
	sub_82EA0060(ctx, base);
	// 82F4C4CC: 48000020  b 0x82f4c4ec
	pc = 0x82F4C4EC; continue 'dispatch;
	// 82F4C4D0: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F4C4D4: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82F4C4D8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F4C4DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F4C4E0: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82F4C4E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F4C4E8: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82F4C4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C4F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C4FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C508 size=160
    let mut pc: u32 = 0x82F4C508;
    'dispatch: loop {
        match pc {
            0x82F4C508 => {
    //   block [0x82F4C508..0x82F4C5A8)
	// 82F4C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C518: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C51C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C520: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C524: 4BFFFEA5  bl 0x82f4c3c8
	ctx.lr = 0x82F4C528;
	sub_82F4C3C8(ctx, base);
	// 82F4C528: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C52C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C530: 419A005C  beq cr6, 0x82f4c58c
	if ctx.cr[6].eq {
	pc = 0x82F4C58C; continue 'dispatch;
	}
	// 82F4C534: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F4C538: 419A0054  beq cr6, 0x82f4c58c
	if ctx.cr[6].eq {
	pc = 0x82F4C58C; continue 'dispatch;
	}
	// 82F4C53C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C540: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C544: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C548: 812B0054  lwz r9, 0x54(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F4C54C: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4C550: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F4C554: 4198001C  blt cr6, 0x82f4c570
	if ctx.cr[6].lt {
	pc = 0x82F4C570; continue 'dispatch;
	}
	// 82F4C558: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C55C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4C560: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82F4C564: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4C568: 4BF53AF9  bl 0x82ea0060
	ctx.lr = 0x82F4C56C;
	sub_82EA0060(ctx, base);
	// 82F4C56C: 48000020  b 0x82f4c58c
	pc = 0x82F4C58C; continue 'dispatch;
	// 82F4C570: 812B0054  lwz r9, 0x54(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F4C574: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
	// 82F4C578: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4C57C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F4C580: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82F4C584: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F4C588: 93EB0050  stw r31, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82F4C58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C590: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C59C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C5A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5A8 size=4
    let mut pc: u32 = 0x82F4C5A8;
    'dispatch: loop {
        match pc {
            0x82F4C5A8 => {
    //   block [0x82F4C5A8..0x82F4C5AC)
	// 82F4C5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5B0 size=4
    let mut pc: u32 = 0x82F4C5B0;
    'dispatch: loop {
        match pc {
            0x82F4C5B0 => {
    //   block [0x82F4C5B0..0x82F4C5B4)
	// 82F4C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5B8 size=20
    let mut pc: u32 = 0x82F4C5B8;
    'dispatch: loop {
        match pc {
            0x82F4C5B8 => {
    //   block [0x82F4C5B8..0x82F4C5CC)
	// 82F4C5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C5BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C5C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C5C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C5C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5D0 size=8
    let mut pc: u32 = 0x82F4C5D0;
    'dispatch: loop {
        match pc {
            0x82F4C5D0 => {
    //   block [0x82F4C5D0..0x82F4C5D8)
	// 82F4C5D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C5D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5D8 size=24
    let mut pc: u32 = 0x82F4C5D8;
    'dispatch: loop {
        match pc {
            0x82F4C5D8 => {
    //   block [0x82F4C5D8..0x82F4C5F0)
	// 82F4C5D8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C5DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C5E0: 392B06FC  addi r9, r11, 0x6fc
	ctx.r[9].s64 = ctx.r[11].s64 + 1788;
	// 82F4C5E4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4C5E8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C5F0 size=12
    let mut pc: u32 = 0x82F4C5F0;
    'dispatch: loop {
        match pc {
            0x82F4C5F0 => {
    //   block [0x82F4C5F0..0x82F4C5FC)
	// 82F4C5F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C5F4: 386B06FC  addi r3, r11, 0x6fc
	ctx.r[3].s64 = ctx.r[11].s64 + 1788;
	// 82F4C5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C600 size=100
    let mut pc: u32 = 0x82F4C600;
    'dispatch: loop {
        match pc {
            0x82F4C600 => {
    //   block [0x82F4C600..0x82F4C664)
	// 82F4C600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C60C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C618: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C61C: 48008B1D  bl 0x82f55138
	ctx.lr = 0x82F4C620;
	sub_82F55138(ctx, base);
	// 82F4C620: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C628: 419A0020  beq cr6, 0x82f4c648
	if ctx.cr[6].eq {
	pc = 0x82F4C648; continue 'dispatch;
	}
	// 82F4C62C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C630: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C634: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82F4C638: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C63C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4C640: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C644: 4BF5416D  bl 0x82ea07b0
	ctx.lr = 0x82F4C648;
	sub_82EA07B0(ctx, base);
	// 82F4C648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C64C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C658: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C65C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C668 size=4
    let mut pc: u32 = 0x82F4C668;
    'dispatch: loop {
        match pc {
            0x82F4C668 => {
    //   block [0x82F4C668..0x82F4C66C)
	// 82F4C668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C670 size=20
    let mut pc: u32 = 0x82F4C670;
    'dispatch: loop {
        match pc {
            0x82F4C670 => {
    //   block [0x82F4C670..0x82F4C684)
	// 82F4C670: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C674: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C678: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C67C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C680: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C688 size=12
    let mut pc: u32 = 0x82F4C688;
    'dispatch: loop {
        match pc {
            0x82F4C688 => {
    //   block [0x82F4C688..0x82F4C694)
	// 82F4C688: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4C68C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C690: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C694 size=8
    let mut pc: u32 = 0x82F4C694;
    'dispatch: loop {
        match pc {
            0x82F4C694 => {
    //   block [0x82F4C694..0x82F4C69C)
	// 82F4C694: 4800003C  b 0x82f4c6d0
	sub_82F4C6D0(ctx, base);
	return;
	// 82F4C698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C6A0 size=44
    let mut pc: u32 = 0x82F4C6A0;
    'dispatch: loop {
        match pc {
            0x82F4C6A0 => {
    //   block [0x82F4C6A0..0x82F4C6CC)
	// 82F4C6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C6A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C6AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C6B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4C6B4: 4800001D  bl 0x82f4c6d0
	ctx.lr = 0x82F4C6B8;
	sub_82F4C6D0(ctx, base);
	// 82F4C6B8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4C6BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F4C6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C6D0 size=152
    let mut pc: u32 = 0x82F4C6D0;
    'dispatch: loop {
        match pc {
            0x82F4C6D0 => {
    //   block [0x82F4C6D0..0x82F4C768)
	// 82F4C6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C6D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C6DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C6E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C6E4: 4BFD12DD  bl 0x82f1d9c0
	ctx.lr = 0x82F4C6E8;
	sub_82F1D9C0(ctx, base);
	// 82F4C6E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4C6EC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4C6F0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4C6F4: 388BBA8C  addi r4, r11, -0x4574
	ctx.r[4].s64 = ctx.r[11].s64 + -17780;
	// 82F4C6F8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4C6FC: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F4C700: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82F4C704: 39690810  addi r11, r9, 0x810
	ctx.r[11].s64 = ctx.r[9].s64 + 2064;
	// 82F4C708: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F4C70C: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82F4C710: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F4C714: 386A07FC  addi r3, r10, 0x7fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2044;
	// 82F4C718: 394807E8  addi r10, r8, 0x7e8
	ctx.r[10].s64 = ctx.r[8].s64 + 2024;
	// 82F4C71C: 392707DC  addi r9, r7, 0x7dc
	ctx.r[9].s64 = ctx.r[7].s64 + 2012;
	// 82F4C720: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4C724: 390607D0  addi r8, r6, 0x7d0
	ctx.r[8].s64 = ctx.r[6].s64 + 2000;
	// 82F4C728: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F4C72C: 38E507B8  addi r7, r5, 0x7b8
	ctx.r[7].s64 = ctx.r[5].s64 + 1976;
	// 82F4C730: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F4C734: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4C738: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F4C73C: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F4C740: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F4C744: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F4C748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C74C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82F4C750: 90DF003C  stw r6, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 82F4C754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4C758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C768 size=100
    let mut pc: u32 = 0x82F4C768;
    'dispatch: loop {
        match pc {
            0x82F4C768 => {
    //   block [0x82F4C768..0x82F4C7CC)
	// 82F4C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C77C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C780: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C784: 48008E5D  bl 0x82f555e0
	ctx.lr = 0x82F4C788;
	sub_82F555E0(ctx, base);
	// 82F4C788: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C78C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C790: 419A0020  beq cr6, 0x82f4c7b0
	if ctx.cr[6].eq {
	pc = 0x82F4C7B0; continue 'dispatch;
	}
	// 82F4C794: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C798: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C79C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F4C7A0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C7A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4C7A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C7AC: 4BF54005  bl 0x82ea07b0
	ctx.lr = 0x82F4C7B0;
	sub_82EA07B0(ctx, base);
	// 82F4C7B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C7B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C7C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7D0 size=8
    let mut pc: u32 = 0x82F4C7D0;
    'dispatch: loop {
        match pc {
            0x82F4C7D0 => {
    //   block [0x82F4C7D0..0x82F4C7D8)
	// 82F4C7D0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F4C7D4: 4BFFFF94  b 0x82f4c768
	sub_82F4C768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7D8 size=8
    let mut pc: u32 = 0x82F4C7D8;
    'dispatch: loop {
        match pc {
            0x82F4C7D8 => {
    //   block [0x82F4C7D8..0x82F4C7E0)
	// 82F4C7D8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F4C7DC: 4BFFFF8C  b 0x82f4c768
	sub_82F4C768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7E0 size=8
    let mut pc: u32 = 0x82F4C7E0;
    'dispatch: loop {
        match pc {
            0x82F4C7E0 => {
    //   block [0x82F4C7E0..0x82F4C7E8)
	// 82F4C7E0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F4C7E4: 4BFFFF84  b 0x82f4c768
	sub_82F4C768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7E8 size=8
    let mut pc: u32 = 0x82F4C7E8;
    'dispatch: loop {
        match pc {
            0x82F4C7E8 => {
    //   block [0x82F4C7E8..0x82F4C7F0)
	// 82F4C7E8: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82F4C7EC: 4BFFFF7C  b 0x82f4c768
	sub_82F4C768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7F0 size=8
    let mut pc: u32 = 0x82F4C7F0;
    'dispatch: loop {
        match pc {
            0x82F4C7F0 => {
    //   block [0x82F4C7F0..0x82F4C7F8)
	// 82F4C7F0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F4C7F4: 4BFFFF74  b 0x82f4c768
	sub_82F4C768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C7F8 size=4
    let mut pc: u32 = 0x82F4C7F8;
    'dispatch: loop {
        match pc {
            0x82F4C7F8 => {
    //   block [0x82F4C7F8..0x82F4C7FC)
	// 82F4C7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C800 size=8
    let mut pc: u32 = 0x82F4C800;
    'dispatch: loop {
        match pc {
            0x82F4C800 => {
    //   block [0x82F4C800..0x82F4C808)
	// 82F4C800: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C804: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C808 size=24
    let mut pc: u32 = 0x82F4C808;
    'dispatch: loop {
        match pc {
            0x82F4C808 => {
    //   block [0x82F4C808..0x82F4C820)
	// 82F4C808: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C80C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C810: 392B0D18  addi r9, r11, 0xd18
	ctx.r[9].s64 = ctx.r[11].s64 + 3352;
	// 82F4C814: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4C818: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C820 size=20
    let mut pc: u32 = 0x82F4C820;
    'dispatch: loop {
        match pc {
            0x82F4C820 => {
    //   block [0x82F4C820..0x82F4C834)
	// 82F4C820: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C824: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C828: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C82C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C830: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C838 size=12
    let mut pc: u32 = 0x82F4C838;
    'dispatch: loop {
        match pc {
            0x82F4C838 => {
    //   block [0x82F4C838..0x82F4C844)
	// 82F4C838: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C83C: 386B0D18  addi r3, r11, 0xd18
	ctx.r[3].s64 = ctx.r[11].s64 + 3352;
	// 82F4C840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C848 size=20
    let mut pc: u32 = 0x82F4C848;
    'dispatch: loop {
        match pc {
            0x82F4C848 => {
    //   block [0x82F4C848..0x82F4C85C)
	// 82F4C848: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C84C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C850: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C854: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C858: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C860 size=8
    let mut pc: u32 = 0x82F4C860;
    'dispatch: loop {
        match pc {
            0x82F4C860 => {
    //   block [0x82F4C860..0x82F4C868)
	// 82F4C860: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C864: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C868 size=24
    let mut pc: u32 = 0x82F4C868;
    'dispatch: loop {
        match pc {
            0x82F4C868 => {
    //   block [0x82F4C868..0x82F4C880)
	// 82F4C868: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C86C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C870: 392B0D6C  addi r9, r11, 0xd6c
	ctx.r[9].s64 = ctx.r[11].s64 + 3436;
	// 82F4C874: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4C878: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C880 size=12
    let mut pc: u32 = 0x82F4C880;
    'dispatch: loop {
        match pc {
            0x82F4C880 => {
    //   block [0x82F4C880..0x82F4C88C)
	// 82F4C880: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C884: 386B0D6C  addi r3, r11, 0xd6c
	ctx.r[3].s64 = ctx.r[11].s64 + 3436;
	// 82F4C888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C890 size=196
    let mut pc: u32 = 0x82F4C890;
    'dispatch: loop {
        match pc {
            0x82F4C890 => {
    //   block [0x82F4C890..0x82F4C954)
	// 82F4C890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C894: 4825B8D9  bl 0x831a816c
	ctx.lr = 0x82F4C898;
	sub_831A8130(ctx, base);
	// 82F4C898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C89C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C8A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C8A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4C8A8: 392B0D6C  addi r9, r11, 0xd6c
	ctx.r[9].s64 = ctx.r[11].s64 + 3436;
	// 82F4C8AC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4C8B0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C8B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C8B8: 4099005C  ble cr6, 0x82f4c914
	if !ctx.cr[6].gt {
	pc = 0x82F4C914; continue 'dispatch;
	}
	// 82F4C8BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4C8C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4C8C4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F4C8C8: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C8CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F4C8D0: 419A0030  beq cr6, 0x82f4c900
	if ctx.cr[6].eq {
	pc = 0x82F4C900; continue 'dispatch;
	}
	// 82F4C8D4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F4C8D8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F4C8DC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F4C8E0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F4C8E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4C8E8: 409A0018  bne cr6, 0x82f4c900
	if !ctx.cr[6].eq {
	pc = 0x82F4C900; continue 'dispatch;
	}
	// 82F4C8EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C8F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4C8F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C8F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C8FC: 4E800421  bctrl
	ctx.lr = 0x82F4C900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4C900: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4C904: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4C908: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F4C90C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4C910: 4198FFB0  blt cr6, 0x82f4c8c0
	if ctx.cr[6].lt {
	pc = 0x82F4C8C0; continue 'dispatch;
	}
	// 82F4C914: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4C918: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4C91C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C920: 409A0020  bne cr6, 0x82f4c940
	if !ctx.cr[6].eq {
	pc = 0x82F4C940; continue 'dispatch;
	}
	// 82F4C924: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C928: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4C92C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4C930: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4C934: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4C938: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4C93C: 4BF53E75  bl 0x82ea07b0
	ctx.lr = 0x82F4C940;
	sub_82EA07B0(ctx, base);
	// 82F4C940: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4C944: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F4C948: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F4C94C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C950: 4825B86C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4C958 size=100
    let mut pc: u32 = 0x82F4C958;
    'dispatch: loop {
        match pc {
            0x82F4C958 => {
    //   block [0x82F4C958..0x82F4C9BC)
	// 82F4C958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4C95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4C960: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4C964: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4C968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4C96C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4C970: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4C974: 4BFFFF1D  bl 0x82f4c890
	ctx.lr = 0x82F4C978;
	sub_82F4C890(ctx, base);
	// 82F4C978: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4C97C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4C980: 419A0020  beq cr6, 0x82f4c9a0
	if ctx.cr[6].eq {
	pc = 0x82F4C9A0; continue 'dispatch;
	}
	// 82F4C984: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C988: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4C98C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4C990: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4C994: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4C998: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4C99C: 4BF53E15  bl 0x82ea07b0
	ctx.lr = 0x82F4C9A0;
	sub_82EA07B0(ctx, base);
	// 82F4C9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4C9A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4C9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4C9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4C9B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4C9B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4C9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C9C0 size=20
    let mut pc: u32 = 0x82F4C9C0;
    'dispatch: loop {
        match pc {
            0x82F4C9C0 => {
    //   block [0x82F4C9C0..0x82F4C9D4)
	// 82F4C9C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C9C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4C9C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C9CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4C9D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C9D8 size=8
    let mut pc: u32 = 0x82F4C9D8;
    'dispatch: loop {
        match pc {
            0x82F4C9D8 => {
    //   block [0x82F4C9D8..0x82F4C9E0)
	// 82F4C9D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4C9DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C9E0 size=24
    let mut pc: u32 = 0x82F4C9E0;
    'dispatch: loop {
        match pc {
            0x82F4C9E0 => {
    //   block [0x82F4C9E0..0x82F4C9F8)
	// 82F4C9E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C9E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4C9E8: 392B0DB4  addi r9, r11, 0xdb4
	ctx.r[9].s64 = ctx.r[11].s64 + 3508;
	// 82F4C9EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4C9F0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C9F8 size=12
    let mut pc: u32 = 0x82F4C9F8;
    'dispatch: loop {
        match pc {
            0x82F4C9F8 => {
    //   block [0x82F4C9F8..0x82F4CA04)
	// 82F4C9F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4C9FC: 386B0DB4  addi r3, r11, 0xdb4
	ctx.r[3].s64 = ctx.r[11].s64 + 3508;
	// 82F4CA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CA08 size=96
    let mut pc: u32 = 0x82F4CA08;
    'dispatch: loop {
        match pc {
            0x82F4CA08 => {
    //   block [0x82F4CA08..0x82F4CA68)
	// 82F4CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CA1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4CA20: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F4CA24: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F4CA28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F4CA2C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CA30: 419A0020  beq cr6, 0x82f4ca50
	if ctx.cr[6].eq {
	pc = 0x82F4CA50; continue 'dispatch;
	}
	// 82F4CA34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CA38: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CA3C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82F4CA40: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CA44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CA48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CA4C: 4BF53D65  bl 0x82ea07b0
	ctx.lr = 0x82F4CA50;
	sub_82EA07B0(ctx, base);
	// 82F4CA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CA54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CA68 size=100
    let mut pc: u32 = 0x82F4CA68;
    'dispatch: loop {
        match pc {
            0x82F4CA68 => {
    //   block [0x82F4CA68..0x82F4CACC)
	// 82F4CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4CA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CA7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CA80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4CA84: 482B4C95  bl 0x83201718
	ctx.lr = 0x82F4CA88;
	sub_83201718(ctx, base);
	// 82F4CA88: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4CA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4CA90: 419A0020  beq cr6, 0x82f4cab0
	if ctx.cr[6].eq {
	pc = 0x82F4CAB0; continue 'dispatch;
	}
	// 82F4CA94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CA98: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CA9C: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 82F4CAA0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CAA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CAA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CAAC: 4BF53D05  bl 0x82ea07b0
	ctx.lr = 0x82F4CAB0;
	sub_82EA07B0(ctx, base);
	// 82F4CAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CAB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4CAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CAC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4CAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CAD0 size=8
    let mut pc: u32 = 0x82F4CAD0;
    'dispatch: loop {
        match pc {
            0x82F4CAD0 => {
    //   block [0x82F4CAD0..0x82F4CAD8)
	// 82F4CAD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CAD4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CAD8 size=24
    let mut pc: u32 = 0x82F4CAD8;
    'dispatch: loop {
        match pc {
            0x82F4CAD8 => {
    //   block [0x82F4CAD8..0x82F4CAF0)
	// 82F4CAD8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CADC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4CAE0: 392B0E80  addi r9, r11, 0xe80
	ctx.r[9].s64 = ctx.r[11].s64 + 3712;
	// 82F4CAE4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4CAE8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CAF0 size=20
    let mut pc: u32 = 0x82F4CAF0;
    'dispatch: loop {
        match pc {
            0x82F4CAF0 => {
    //   block [0x82F4CAF0..0x82F4CB04)
	// 82F4CAF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CAF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CAF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CAFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CB00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CB08 size=12
    let mut pc: u32 = 0x82F4CB08;
    'dispatch: loop {
        match pc {
            0x82F4CB08 => {
    //   block [0x82F4CB08..0x82F4CB14)
	// 82F4CB08: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CB0C: 386B0E80  addi r3, r11, 0xe80
	ctx.r[3].s64 = ctx.r[11].s64 + 3712;
	// 82F4CB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CB18 size=20
    let mut pc: u32 = 0x82F4CB18;
    'dispatch: loop {
        match pc {
            0x82F4CB18 => {
    //   block [0x82F4CB18..0x82F4CB2C)
	// 82F4CB18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CB1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CB20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CB24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CB28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CB30 size=8
    let mut pc: u32 = 0x82F4CB30;
    'dispatch: loop {
        match pc {
            0x82F4CB30 => {
    //   block [0x82F4CB30..0x82F4CB38)
	// 82F4CB30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CB34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CB38 size=24
    let mut pc: u32 = 0x82F4CB38;
    'dispatch: loop {
        match pc {
            0x82F4CB38 => {
    //   block [0x82F4CB38..0x82F4CB50)
	// 82F4CB38: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CB3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4CB40: 392B0EDC  addi r9, r11, 0xedc
	ctx.r[9].s64 = ctx.r[11].s64 + 3804;
	// 82F4CB44: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4CB48: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CB50 size=12
    let mut pc: u32 = 0x82F4CB50;
    'dispatch: loop {
        match pc {
            0x82F4CB50 => {
    //   block [0x82F4CB50..0x82F4CB5C)
	// 82F4CB50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CB54: 386B0EDC  addi r3, r11, 0xedc
	ctx.r[3].s64 = ctx.r[11].s64 + 3804;
	// 82F4CB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CB60 size=100
    let mut pc: u32 = 0x82F4CB60;
    'dispatch: loop {
        match pc {
            0x82F4CB60 => {
    //   block [0x82F4CB60..0x82F4CBC4)
	// 82F4CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CB68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4CB6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CB70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CB74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CB78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4CB7C: 4800958D  bl 0x82f56108
	ctx.lr = 0x82F4CB80;
	sub_82F56108(ctx, base);
	// 82F4CB80: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4CB84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4CB88: 419A0020  beq cr6, 0x82f4cba8
	if ctx.cr[6].eq {
	pc = 0x82F4CBA8; continue 'dispatch;
	}
	// 82F4CB8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CB90: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CB94: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82F4CB98: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CB9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CBA0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CBA4: 4BF53C0D  bl 0x82ea07b0
	ctx.lr = 0x82F4CBA8;
	sub_82EA07B0(ctx, base);
	// 82F4CBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CBAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4CBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CBB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4CBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CBC8 size=20
    let mut pc: u32 = 0x82F4CBC8;
    'dispatch: loop {
        match pc {
            0x82F4CBC8 => {
    //   block [0x82F4CBC8..0x82F4CBDC)
	// 82F4CBC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CBCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CBD0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CBD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CBD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CBE0 size=8
    let mut pc: u32 = 0x82F4CBE0;
    'dispatch: loop {
        match pc {
            0x82F4CBE0 => {
    //   block [0x82F4CBE0..0x82F4CBE8)
	// 82F4CBE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CBE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CBE8 size=24
    let mut pc: u32 = 0x82F4CBE8;
    'dispatch: loop {
        match pc {
            0x82F4CBE8 => {
    //   block [0x82F4CBE8..0x82F4CC00)
	// 82F4CBE8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CBEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4CBF0: 392B0F58  addi r9, r11, 0xf58
	ctx.r[9].s64 = ctx.r[11].s64 + 3928;
	// 82F4CBF4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4CBF8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CC00 size=12
    let mut pc: u32 = 0x82F4CC00;
    'dispatch: loop {
        match pc {
            0x82F4CC00 => {
    //   block [0x82F4CC00..0x82F4CC0C)
	// 82F4CC00: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CC04: 386B0F58  addi r3, r11, 0xf58
	ctx.r[3].s64 = ctx.r[11].s64 + 3928;
	// 82F4CC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CC10 size=100
    let mut pc: u32 = 0x82F4CC10;
    'dispatch: loop {
        match pc {
            0x82F4CC10 => {
    //   block [0x82F4CC10..0x82F4CC74)
	// 82F4CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CC18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4CC1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CC20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CC24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CC28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4CC2C: 48009A95  bl 0x82f566c0
	ctx.lr = 0x82F4CC30;
	sub_82F566C0(ctx, base);
	// 82F4CC30: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4CC34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4CC38: 419A0020  beq cr6, 0x82f4cc58
	if ctx.cr[6].eq {
	pc = 0x82F4CC58; continue 'dispatch;
	}
	// 82F4CC3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CC40: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CC44: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F4CC48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CC4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CC50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CC54: 4BF53B5D  bl 0x82ea07b0
	ctx.lr = 0x82F4CC58;
	sub_82EA07B0(ctx, base);
	// 82F4CC58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CC5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4CC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CC68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4CC6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CC78 size=20
    let mut pc: u32 = 0x82F4CC78;
    'dispatch: loop {
        match pc {
            0x82F4CC78 => {
    //   block [0x82F4CC78..0x82F4CC8C)
	// 82F4CC78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CC7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CC80: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CC84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CC88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CC90 size=92
    let mut pc: u32 = 0x82F4CC90;
    'dispatch: loop {
        match pc {
            0x82F4CC90 => {
    //   block [0x82F4CC90..0x82F4CCEC)
	// 82F4CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CC98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CC9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CCA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CCA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F4CCA8: 419A0030  beq cr6, 0x82f4ccd8
	if ctx.cr[6].eq {
	pc = 0x82F4CCD8; continue 'dispatch;
	}
	// 82F4CCAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F4CCB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CCB4: 482B49FD  bl 0x832016b0
	ctx.lr = 0x82F4CCB8;
	sub_832016B0(ctx, base);
	// 82F4CCB8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4CCBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4CCC0: 392A10AC  addi r9, r10, 0x10ac
	ctx.r[9].s64 = ctx.r[10].s64 + 4268;
	// 82F4CCC4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F4CCC8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CCCC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F4CCD0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F4CCD4: 911F005C  stw r8, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82F4CCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4CCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CCE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CCF0 size=52
    let mut pc: u32 = 0x82F4CCF0;
    'dispatch: loop {
        match pc {
            0x82F4CCF0 => {
    //   block [0x82F4CCF0..0x82F4CD24)
	// 82F4CCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CCF8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F4CD00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CD04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4CD08: 482B49A9  bl 0x832016b0
	ctx.lr = 0x82F4CD0C;
	sub_832016B0(ctx, base);
	// 82F4CD0C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CD10: 386B10AC  addi r3, r11, 0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + 4268;
	// 82F4CD14: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F4CD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CD28 size=148
    let mut pc: u32 = 0x82F4CD28;
    'dispatch: loop {
        match pc {
            0x82F4CD28 => {
    //   block [0x82F4CD28..0x82F4CDBC)
	// 82F4CD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CD30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4CD34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CD38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CD3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CD40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4CD44: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F4CD48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4CD4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4CD50: 409A0020  bne cr6, 0x82f4cd70
	if !ctx.cr[6].eq {
	pc = 0x82F4CD70; continue 'dispatch;
	}
	// 82F4CD54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CD58: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4CD5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4CD60: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F4CD64: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4CD68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4CD6C: 4BF53A45  bl 0x82ea07b0
	ctx.lr = 0x82F4CD70;
	sub_82EA07B0(ctx, base);
	// 82F4CD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CD74: 482B49A5  bl 0x83201718
	ctx.lr = 0x82F4CD78;
	sub_83201718(ctx, base);
	// 82F4CD78: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4CD7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4CD80: 419A0020  beq cr6, 0x82f4cda0
	if ctx.cr[6].eq {
	pc = 0x82F4CDA0; continue 'dispatch;
	}
	// 82F4CD84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CD88: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CD8C: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 82F4CD90: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CD94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CD98: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CD9C: 4BF53A15  bl 0x82ea07b0
	ctx.lr = 0x82F4CDA0;
	sub_82EA07B0(ctx, base);
	// 82F4CDA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CDA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4CDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CDB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4CDB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CDC0 size=8
    let mut pc: u32 = 0x82F4CDC0;
    'dispatch: loop {
        match pc {
            0x82F4CDC0 => {
    //   block [0x82F4CDC0..0x82F4CDC8)
	// 82F4CDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CDC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CDC8 size=24
    let mut pc: u32 = 0x82F4CDC8;
    'dispatch: loop {
        match pc {
            0x82F4CDC8 => {
    //   block [0x82F4CDC8..0x82F4CDE0)
	// 82F4CDC8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CDCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4CDD0: 392B1164  addi r9, r11, 0x1164
	ctx.r[9].s64 = ctx.r[11].s64 + 4452;
	// 82F4CDD4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4CDD8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4CDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CDE0 size=20
    let mut pc: u32 = 0x82F4CDE0;
    'dispatch: loop {
        match pc {
            0x82F4CDE0 => {
    //   block [0x82F4CDE0..0x82F4CDF4)
	// 82F4CDE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CDE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CDE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CDEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CDF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CDF8 size=12
    let mut pc: u32 = 0x82F4CDF8;
    'dispatch: loop {
        match pc {
            0x82F4CDF8 => {
    //   block [0x82F4CDF8..0x82F4CE04)
	// 82F4CDF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CDFC: 386B1164  addi r3, r11, 0x1164
	ctx.r[3].s64 = ctx.r[11].s64 + 4452;
	// 82F4CE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CE08 size=20
    let mut pc: u32 = 0x82F4CE08;
    'dispatch: loop {
        match pc {
            0x82F4CE08 => {
    //   block [0x82F4CE08..0x82F4CE1C)
	// 82F4CE08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CE0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CE10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CE14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CE18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CE20 size=12
    let mut pc: u32 = 0x82F4CE20;
    'dispatch: loop {
        match pc {
            0x82F4CE20 => {
    //   block [0x82F4CE20..0x82F4CE2C)
	// 82F4CE20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4CE24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CE28: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CE2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CE2C size=8
    let mut pc: u32 = 0x82F4CE2C;
    'dispatch: loop {
        match pc {
            0x82F4CE2C => {
    //   block [0x82F4CE2C..0x82F4CE34)
	// 82F4CE2C: 4800003C  b 0x82f4ce68
	sub_82F4CE68(ctx, base);
	return;
	// 82F4CE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CE38 size=44
    let mut pc: u32 = 0x82F4CE38;
    'dispatch: loop {
        match pc {
            0x82F4CE38 => {
    //   block [0x82F4CE38..0x82F4CE64)
	// 82F4CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CE40: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CE44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CE48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4CE4C: 4800001D  bl 0x82f4ce68
	ctx.lr = 0x82F4CE50;
	sub_82F4CE68(ctx, base);
	// 82F4CE50: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4CE54: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82F4CE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CE68 size=108
    let mut pc: u32 = 0x82F4CE68;
    'dispatch: loop {
        match pc {
            0x82F4CE68 => {
    //   block [0x82F4CE68..0x82F4CED4)
	// 82F4CE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CE70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CE74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CE78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CE7C: 4BFD0B45  bl 0x82f1d9c0
	ctx.lr = 0x82F4CE80;
	sub_82F1D9C0(ctx, base);
	// 82F4CE80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4CE84: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4CE88: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4CE8C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4CE90: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F4CE94: 38CB1228  addi r6, r11, 0x1228
	ctx.r[6].s64 = ctx.r[11].s64 + 4648;
	// 82F4CE98: 3868121C  addi r3, r8, 0x121c
	ctx.r[3].s64 = ctx.r[8].s64 + 4636;
	// 82F4CE9C: 38AA1210  addi r5, r10, 0x1210
	ctx.r[5].s64 = ctx.r[10].s64 + 4624;
	// 82F4CEA0: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F4CEA4: 388911FC  addi r4, r9, 0x11fc
	ctx.r[4].s64 = ctx.r[9].s64 + 4604;
	// 82F4CEA8: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F4CEAC: 396711F0  addi r11, r7, 0x11f0
	ctx.r[11].s64 = ctx.r[7].s64 + 4592;
	// 82F4CEB0: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F4CEB4: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F4CEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CEBC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F4CEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4CEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CED8 size=8
    let mut pc: u32 = 0x82F4CED8;
    'dispatch: loop {
        match pc {
            0x82F4CED8 => {
    //   block [0x82F4CED8..0x82F4CEE0)
	// 82F4CED8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F4CEDC: 4800000C  b 0x82f4cee8
	sub_82F4CEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CEE0 size=8
    let mut pc: u32 = 0x82F4CEE0;
    'dispatch: loop {
        match pc {
            0x82F4CEE0 => {
    //   block [0x82F4CEE0..0x82F4CEE8)
	// 82F4CEE0: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F4CEE4: 48000004  b 0x82f4cee8
	sub_82F4CEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CEE8 size=100
    let mut pc: u32 = 0x82F4CEE8;
    'dispatch: loop {
        match pc {
            0x82F4CEE8 => {
    //   block [0x82F4CEE8..0x82F4CF4C)
	// 82F4CEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CEF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4CEF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CEF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CEFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CF00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4CF04: 4B39D915  bl 0x822ea818
	ctx.lr = 0x82F4CF08;
	sub_822EA818(ctx, base);
	// 82F4CF08: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4CF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4CF10: 419A0020  beq cr6, 0x82f4cf30
	if ctx.cr[6].eq {
	pc = 0x82F4CF30; continue 'dispatch;
	}
	// 82F4CF14: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CF18: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4CF1C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F4CF20: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4CF24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4CF28: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4CF2C: 4BF53885  bl 0x82ea07b0
	ctx.lr = 0x82F4CF30;
	sub_82EA07B0(ctx, base);
	// 82F4CF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4CF34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4CF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CF40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4CF44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4CF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CF50 size=20
    let mut pc: u32 = 0x82F4CF50;
    'dispatch: loop {
        match pc {
            0x82F4CF50 => {
    //   block [0x82F4CF50..0x82F4CF64)
	// 82F4CF50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CF54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CF58: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4CF5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4CF60: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CF68 size=12
    let mut pc: u32 = 0x82F4CF68;
    'dispatch: loop {
        match pc {
            0x82F4CF68 => {
    //   block [0x82F4CF68..0x82F4CF74)
	// 82F4CF68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4CF6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4CF70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CF74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4CF74 size=8
    let mut pc: u32 = 0x82F4CF74;
    'dispatch: loop {
        match pc {
            0x82F4CF74 => {
    //   block [0x82F4CF74..0x82F4CF7C)
	// 82F4CF74: 4800003C  b 0x82f4cfb0
	sub_82F4CFB0(ctx, base);
	return;
	// 82F4CF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CF80 size=44
    let mut pc: u32 = 0x82F4CF80;
    'dispatch: loop {
        match pc {
            0x82F4CF80 => {
    //   block [0x82F4CF80..0x82F4CFAC)
	// 82F4CF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CF88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4CF90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4CF94: 4800001D  bl 0x82f4cfb0
	ctx.lr = 0x82F4CF98;
	sub_82F4CFB0(ctx, base);
	// 82F4CF98: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4CF9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F4CFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4CFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4CFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4CFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4CFB0 size=152
    let mut pc: u32 = 0x82F4CFB0;
    'dispatch: loop {
        match pc {
            0x82F4CFB0 => {
    //   block [0x82F4CFB0..0x82F4D048)
	// 82F4CFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4CFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4CFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4CFBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4CFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4CFC4: 4BFD09FD  bl 0x82f1d9c0
	ctx.lr = 0x82F4CFC8;
	sub_82F1D9C0(ctx, base);
	// 82F4CFC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4CFCC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4CFD0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4CFD4: 388BBA8C  addi r4, r11, -0x4574
	ctx.r[4].s64 = ctx.r[11].s64 + -17780;
	// 82F4CFD8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4CFDC: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F4CFE0: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82F4CFE4: 396912D0  addi r11, r9, 0x12d0
	ctx.r[11].s64 = ctx.r[9].s64 + 4816;
	// 82F4CFE8: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F4CFEC: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82F4CFF0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F4CFF4: 386A12C0  addi r3, r10, 0x12c0
	ctx.r[3].s64 = ctx.r[10].s64 + 4800;
	// 82F4CFF8: 394812AC  addi r10, r8, 0x12ac
	ctx.r[10].s64 = ctx.r[8].s64 + 4780;
	// 82F4CFFC: 392712A0  addi r9, r7, 0x12a0
	ctx.r[9].s64 = ctx.r[7].s64 + 4768;
	// 82F4D000: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4D004: 39061294  addi r8, r6, 0x1294
	ctx.r[8].s64 = ctx.r[6].s64 + 4756;
	// 82F4D008: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F4D00C: 38E5127C  addi r7, r5, 0x127c
	ctx.r[7].s64 = ctx.r[5].s64 + 4732;
	// 82F4D010: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F4D014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4D018: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F4D01C: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F4D020: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F4D024: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F4D028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D02C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82F4D030: 90DF003C  stw r6, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 82F4D034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4D038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D048 size=100
    let mut pc: u32 = 0x82F4D048;
    'dispatch: loop {
        match pc {
            0x82F4D048 => {
    //   block [0x82F4D048..0x82F4D0AC)
	// 82F4D048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4D054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4D058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D05C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4D060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4D064: 4800B43D  bl 0x82f584a0
	ctx.lr = 0x82F4D068;
	sub_82F584A0(ctx, base);
	// 82F4D068: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4D06C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4D070: 419A0020  beq cr6, 0x82f4d090
	if ctx.cr[6].eq {
	pc = 0x82F4D090; continue 'dispatch;
	}
	// 82F4D074: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D078: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4D07C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F4D080: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4D084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4D088: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4D08C: 4BF53725  bl 0x82ea07b0
	ctx.lr = 0x82F4D090;
	sub_82EA07B0(ctx, base);
	// 82F4D090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4D098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D0A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4D0A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0B0 size=8
    let mut pc: u32 = 0x82F4D0B0;
    'dispatch: loop {
        match pc {
            0x82F4D0B0 => {
    //   block [0x82F4D0B0..0x82F4D0B8)
	// 82F4D0B0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F4D0B4: 4BFFFF94  b 0x82f4d048
	sub_82F4D048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0B8 size=8
    let mut pc: u32 = 0x82F4D0B8;
    'dispatch: loop {
        match pc {
            0x82F4D0B8 => {
    //   block [0x82F4D0B8..0x82F4D0C0)
	// 82F4D0B8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F4D0BC: 4BFFFF8C  b 0x82f4d048
	sub_82F4D048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0C0 size=8
    let mut pc: u32 = 0x82F4D0C0;
    'dispatch: loop {
        match pc {
            0x82F4D0C0 => {
    //   block [0x82F4D0C0..0x82F4D0C8)
	// 82F4D0C0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F4D0C4: 4BFFFF84  b 0x82f4d048
	sub_82F4D048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0C8 size=8
    let mut pc: u32 = 0x82F4D0C8;
    'dispatch: loop {
        match pc {
            0x82F4D0C8 => {
    //   block [0x82F4D0C8..0x82F4D0D0)
	// 82F4D0C8: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82F4D0CC: 4BFFFF7C  b 0x82f4d048
	sub_82F4D048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0D0 size=8
    let mut pc: u32 = 0x82F4D0D0;
    'dispatch: loop {
        match pc {
            0x82F4D0D0 => {
    //   block [0x82F4D0D0..0x82F4D0D8)
	// 82F4D0D0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F4D0D4: 4BFFFF74  b 0x82f4d048
	sub_82F4D048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0D8 size=8
    let mut pc: u32 = 0x82F4D0D8;
    'dispatch: loop {
        match pc {
            0x82F4D0D8 => {
    //   block [0x82F4D0D8..0x82F4D0E0)
	// 82F4D0D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4D0DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0E0 size=24
    let mut pc: u32 = 0x82F4D0E0;
    'dispatch: loop {
        match pc {
            0x82F4D0E0 => {
    //   block [0x82F4D0E0..0x82F4D0F8)
	// 82F4D0E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D0E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4D0E8: 392B1364  addi r9, r11, 0x1364
	ctx.r[9].s64 = ctx.r[11].s64 + 4964;
	// 82F4D0EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4D0F0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4D0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D0F8 size=20
    let mut pc: u32 = 0x82F4D0F8;
    'dispatch: loop {
        match pc {
            0x82F4D0F8 => {
    //   block [0x82F4D0F8..0x82F4D10C)
	// 82F4D0F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D0FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4D100: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D104: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4D108: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D110 size=12
    let mut pc: u32 = 0x82F4D110;
    'dispatch: loop {
        match pc {
            0x82F4D110 => {
    //   block [0x82F4D110..0x82F4D11C)
	// 82F4D110: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D114: 386B1364  addi r3, r11, 0x1364
	ctx.r[3].s64 = ctx.r[11].s64 + 4964;
	// 82F4D118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D120 size=72
    let mut pc: u32 = 0x82F4D120;
    'dispatch: loop {
        match pc {
            0x82F4D120 => {
    //   block [0x82F4D120..0x82F4D168)
	// 82F4D120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4D12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4D134: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D138: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F4D13C: 392B13C0  addi r9, r11, 0x13c0
	ctx.r[9].s64 = ctx.r[11].s64 + 5056;
	// 82F4D140: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F4D144: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4D148: 419A000C  beq cr6, 0x82f4d154
	if ctx.cr[6].eq {
	pc = 0x82F4D154; continue 'dispatch;
	}
	// 82F4D14C: 4B37311D  bl 0x822c0268
	ctx.lr = 0x82F4D150;
	sub_822C0268(ctx, base);
	// 82F4D150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4D158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D168 size=20
    let mut pc: u32 = 0x82F4D168;
    'dispatch: loop {
        match pc {
            0x82F4D168 => {
    //   block [0x82F4D168..0x82F4D17C)
	// 82F4D168: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D16C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4D170: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D174: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4D178: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D180 size=12
    let mut pc: u32 = 0x82F4D180;
    'dispatch: loop {
        match pc {
            0x82F4D180 => {
    //   block [0x82F4D180..0x82F4D18C)
	// 82F4D180: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4D184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4D188: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D18C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D18C size=8
    let mut pc: u32 = 0x82F4D18C;
    'dispatch: loop {
        match pc {
            0x82F4D18C => {
    //   block [0x82F4D18C..0x82F4D194)
	// 82F4D18C: 4800003C  b 0x82f4d1c8
	sub_82F4D1C8(ctx, base);
	return;
	// 82F4D190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D198 size=44
    let mut pc: u32 = 0x82F4D198;
    'dispatch: loop {
        match pc {
            0x82F4D198 => {
    //   block [0x82F4D198..0x82F4D1C4)
	// 82F4D198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D1A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D1A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4D1A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4D1AC: 4800001D  bl 0x82f4d1c8
	ctx.lr = 0x82F4D1B0;
	sub_82F4D1C8(ctx, base);
	// 82F4D1B0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4D1B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F4D1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D1C8 size=132
    let mut pc: u32 = 0x82F4D1C8;
    'dispatch: loop {
        match pc {
            0x82F4D1C8 => {
    //   block [0x82F4D1C8..0x82F4D24C)
	// 82F4D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D1D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4D1D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D1D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4D1DC: 4BFD07E5  bl 0x82f1d9c0
	ctx.lr = 0x82F4D1E0;
	sub_82F1D9C0(ctx, base);
	// 82F4D1E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D1E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4D1E8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4D1EC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4D1F0: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F4D1F4: 388B13C0  addi r4, r11, 0x13c0
	ctx.r[4].s64 = ctx.r[11].s64 + 5056;
	// 82F4D1F8: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F4D1FC: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82F4D200: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82F4D204: 386A1418  addi r3, r10, 0x1418
	ctx.r[3].s64 = ctx.r[10].s64 + 5144;
	// 82F4D208: 3969140C  addi r11, r9, 0x140c
	ctx.r[11].s64 = ctx.r[9].s64 + 5132;
	// 82F4D20C: 394813F8  addi r10, r8, 0x13f8
	ctx.r[10].s64 = ctx.r[8].s64 + 5112;
	// 82F4D210: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4D214: 392713EC  addi r9, r7, 0x13ec
	ctx.r[9].s64 = ctx.r[7].s64 + 5100;
	// 82F4D218: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F4D21C: 390613E0  addi r8, r6, 0x13e0
	ctx.r[8].s64 = ctx.r[6].s64 + 5088;
	// 82F4D220: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F4D224: 38E513D0  addi r7, r5, 0x13d0
	ctx.r[7].s64 = ctx.r[5].s64 + 5072;
	// 82F4D228: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F4D22C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F4D230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D234: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F4D238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4D23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D250 size=100
    let mut pc: u32 = 0x82F4D250;
    'dispatch: loop {
        match pc {
            0x82F4D250 => {
    //   block [0x82F4D250..0x82F4D2B4)
	// 82F4D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4D25C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4D260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4D268: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4D26C: 4800B82D  bl 0x82f58a98
	ctx.lr = 0x82F4D270;
	sub_82F58A98(ctx, base);
	// 82F4D270: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4D274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4D278: 419A0020  beq cr6, 0x82f4d298
	if ctx.cr[6].eq {
	pc = 0x82F4D298; continue 'dispatch;
	}
	// 82F4D27C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D280: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4D284: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F4D288: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4D28C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4D290: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4D294: 4BF5351D  bl 0x82ea07b0
	ctx.lr = 0x82F4D298;
	sub_82EA07B0(ctx, base);
	// 82F4D298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D29C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4D2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D2A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4D2AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2B8 size=8
    let mut pc: u32 = 0x82F4D2B8;
    'dispatch: loop {
        match pc {
            0x82F4D2B8 => {
    //   block [0x82F4D2B8..0x82F4D2C0)
	// 82F4D2B8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F4D2BC: 4BFFFF94  b 0x82f4d250
	sub_82F4D250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2C0 size=8
    let mut pc: u32 = 0x82F4D2C0;
    'dispatch: loop {
        match pc {
            0x82F4D2C0 => {
    //   block [0x82F4D2C0..0x82F4D2C8)
	// 82F4D2C0: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F4D2C4: 4BFFFF8C  b 0x82f4d250
	sub_82F4D250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2C8 size=8
    let mut pc: u32 = 0x82F4D2C8;
    'dispatch: loop {
        match pc {
            0x82F4D2C8 => {
    //   block [0x82F4D2C8..0x82F4D2D0)
	// 82F4D2C8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F4D2CC: 4BFFFF84  b 0x82f4d250
	sub_82F4D250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2D0 size=8
    let mut pc: u32 = 0x82F4D2D0;
    'dispatch: loop {
        match pc {
            0x82F4D2D0 => {
    //   block [0x82F4D2D0..0x82F4D2D8)
	// 82F4D2D0: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82F4D2D4: 4BFFFF7C  b 0x82f4d250
	sub_82F4D250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2D8 size=8
    let mut pc: u32 = 0x82F4D2D8;
    'dispatch: loop {
        match pc {
            0x82F4D2D8 => {
    //   block [0x82F4D2D8..0x82F4D2E0)
	// 82F4D2D8: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F4D2DC: 4BFFFF74  b 0x82f4d250
	sub_82F4D250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2E0 size=8
    let mut pc: u32 = 0x82F4D2E0;
    'dispatch: loop {
        match pc {
            0x82F4D2E0 => {
    //   block [0x82F4D2E0..0x82F4D2E8)
	// 82F4D2E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4D2E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D2E8 size=24
    let mut pc: u32 = 0x82F4D2E8;
    'dispatch: loop {
        match pc {
            0x82F4D2E8 => {
    //   block [0x82F4D2E8..0x82F4D300)
	// 82F4D2E8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D2EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4D2F0: 392B148C  addi r9, r11, 0x148c
	ctx.r[9].s64 = ctx.r[11].s64 + 5260;
	// 82F4D2F4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4D2F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4D2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D300 size=20
    let mut pc: u32 = 0x82F4D300;
    'dispatch: loop {
        match pc {
            0x82F4D300 => {
    //   block [0x82F4D300..0x82F4D314)
	// 82F4D300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D304: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4D308: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D30C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4D310: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D318 size=12
    let mut pc: u32 = 0x82F4D318;
    'dispatch: loop {
        match pc {
            0x82F4D318 => {
    //   block [0x82F4D318..0x82F4D324)
	// 82F4D318: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4D31C: 386B148C  addi r3, r11, 0x148c
	ctx.r[3].s64 = ctx.r[11].s64 + 5260;
	// 82F4D320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D328 size=100
    let mut pc: u32 = 0x82F4D328;
    'dispatch: loop {
        match pc {
            0x82F4D328 => {
    //   block [0x82F4D328..0x82F4D38C)
	// 82F4D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4D330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4D334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4D338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D33C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4D340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4D344: 482B41A5  bl 0x832014e8
	ctx.lr = 0x82F4D348;
	sub_832014E8(ctx, base);
	// 82F4D348: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F4D34C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4D350: 419A0020  beq cr6, 0x82f4d370
	if ctx.cr[6].eq {
	pc = 0x82F4D370; continue 'dispatch;
	}
	// 82F4D354: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4D358: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4D35C: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 82F4D360: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4D364: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4D368: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4D36C: 4BF53445  bl 0x82ea07b0
	ctx.lr = 0x82F4D370;
	sub_82EA07B0(ctx, base);
	// 82F4D370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4D374: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4D378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4D37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4D380: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4D384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4D388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D390 size=224
    let mut pc: u32 = 0x82F4D390;
    'dispatch: loop {
        match pc {
            0x82F4D390 => {
    //   block [0x82F4D390..0x82F4D470)
	// 82F4D390: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4D470 size=108
    let mut pc: u32 = 0x82F4D470;
    'dispatch: loop {
        match pc {
            0x82F4D470 => {
    //   block [0x82F4D470..0x82F4D4DC)
	// 82F4D470: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82F4D474: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82F4D478: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F4D47C: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 82F4D480: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82F4D484: C00808A8  lfs f0, 0x8a8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4D488: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D4E0 size=404
    let mut pc: u32 = 0x82F4D4E0;
    'dispatch: loop {
        match pc {
            0x82F4D4E0 => {
    //   block [0x82F4D4E0..0x82F4D674)
	// 82F4D4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D4E4: 4825AC89  bl 0x831a816c
	ctx.lr = 0x82F4D4E8;
	sub_831A8130(ctx, base);
	// 82F4D4E8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F4D4EC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F4D4F0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D4F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4D678 size=696
    let mut pc: u32 = 0x82F4D678;
    'dispatch: loop {
        match pc {
            0x82F4D678 => {
    //   block [0x82F4D678..0x82F4D930)
	// 82F4D678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D67C: 4825AAED  bl 0x831a8168
	ctx.lr = 0x82F4D680;
	sub_831A8130(ctx, base);
	// 82F4D680: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82F4D684: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4D930 size=548
    let mut pc: u32 = 0x82F4D930;
    'dispatch: loop {
        match pc {
            0x82F4D930 => {
    //   block [0x82F4D930..0x82F4DB54)
	// 82F4D930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4D934: 4825A839  bl 0x831a816c
	ctx.lr = 0x82F4D938;
	sub_831A8130(ctx, base);
	// 82F4D938: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F4D93C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F4D940: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4D944: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4DB58 size=716
    let mut pc: u32 = 0x82F4DB58;
    'dispatch: loop {
        match pc {
            0x82F4DB58 => {
    //   block [0x82F4DB58..0x82F4DE24)
	// 82F4DB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4DB5C: 4825A60D  bl 0x831a8168
	ctx.lr = 0x82F4DB60;
	sub_831A8130(ctx, base);
	// 82F4DB60: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 82F4DB64: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82F4DB68: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82F4DB6C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4DE28 size=96
    let mut pc: u32 = 0x82F4DE28;
    'dispatch: loop {
        match pc {
            0x82F4DE28 => {
    //   block [0x82F4DE28..0x82F4DE88)
	// 82F4DE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4DE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4DE30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4DE34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4DE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4DE3C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82F4DE40: 4BF5C369  bl 0x82eaa1a8
	ctx.lr = 0x82F4DE44;
	sub_82EAA1A8(ctx, base);
	// 82F4DE44: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F4DE48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F4DE4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4DE50: 4BF54799  bl 0x82ea25e8
	ctx.lr = 0x82F4DE54;
	sub_82EA25E8(ctx, base);
	// 82F4DE54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4DE58: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F4DE5C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82F4DE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4DE64: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82F4DE68: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82F4DE6C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82F4DE70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F4DE74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4DE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4DE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4DE80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4DE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4DE88 size=16
    let mut pc: u32 = 0x82F4DE88;
    'dispatch: loop {
        match pc {
            0x82F4DE88 => {
    //   block [0x82F4DE88..0x82F4DE98)
	// 82F4DE88: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F4DE8C: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4DE90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4DE94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4DE98 size=16
    let mut pc: u32 = 0x82F4DE98;
    'dispatch: loop {
        match pc {
            0x82F4DE98 => {
    //   block [0x82F4DE98..0x82F4DEA8)
	// 82F4DE98: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F4DE9C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F4DEA0: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4DEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4DEA8 size=136
    let mut pc: u32 = 0x82F4DEA8;
    'dispatch: loop {
        match pc {
            0x82F4DEA8 => {
    //   block [0x82F4DEA8..0x82F4DF30)
	// 82F4DEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4DEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4DEB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4DEB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4DEB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4DEBC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4DEC0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4DEC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4DEC8: 419A004C  beq cr6, 0x82f4df14
	if ctx.cr[6].eq {
	pc = 0x82F4DF14; continue 'dispatch;
	}
	// 82F4DECC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F4DED0: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F4DED4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F4DED8: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F4DEDC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4DEE0: 409A0034  bne cr6, 0x82f4df14
	if !ctx.cr[6].eq {
	pc = 0x82F4DF14; continue 'dispatch;
	}
	// 82F4DEE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4DEE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4DEEC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4DEF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4DEF4: 4E800421  bctrl
	ctx.lr = 0x82F4DEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4DEF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F4DEFC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4DF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4DF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4DF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4DF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4DF10: 4E800020  blr
	return;
	// 82F4DF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4DF18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F4DF1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4DF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4DF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4DF28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4DF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4DF30 size=8
    let mut pc: u32 = 0x82F4DF30;
    'dispatch: loop {
        match pc {
            0x82F4DF30 => {
    //   block [0x82F4DF30..0x82F4DF38)
	// 82F4DF30: D0230008  stfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4DF38 size=112
    let mut pc: u32 = 0x82F4DF38;
    'dispatch: loop {
        match pc {
            0x82F4DF38 => {
    //   block [0x82F4DF38..0x82F4DFA8)
	// 82F4DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4DF3C: 4825A231  bl 0x831a816c
	ctx.lr = 0x82F4DF40;
	sub_831A8130(ctx, base);
	// 82F4DF40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4DF44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4DF48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F4DF4C: 997E0024  stb r11, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82F4DF50: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4DF54: 997E0164  stb r11, 0x164(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(356 as u32), ctx.r[11].u8 ) };
	// 82F4DF58: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4DF5C: 37EBFFFF  addic. r31, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F4DF60: 98BE0170  stb r5, 0x170(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(368 as u32), ctx.r[5].u8 ) };
	// 82F4DF64: 41800030  blt 0x82f4df94
	if ctx.cr[0].lt {
	pc = 0x82F4DF94; continue 'dispatch;
	}
	// 82F4DF68: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4DF6C: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82F4DF70: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4DF74: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F4DF78: 3BAB0048  addi r29, r11, 0x48
	ctx.r[29].s64 = ctx.r[11].s64 + 72;
	// 82F4DF7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4DF80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4DF84: 4BF546BD  bl 0x82ea2640
	ctx.lr = 0x82F4DF88;
	sub_82EA2640(ctx, base);
	// 82F4DF88: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F4DF8C: 3BBDFFD0  addi r29, r29, -0x30
	ctx.r[29].s64 = ctx.r[29].s64 + -48;
	// 82F4DF90: 4080FFEC  bge 0x82f4df7c
	if !ctx.cr[0].lt {
	pc = 0x82F4DF7C; continue 'dispatch;
	}
	// 82F4DF94: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 82F4DF98: 809E0148  lwz r4, 0x148(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4DF9C: 4BF546A5  bl 0x82ea2640
	ctx.lr = 0x82F4DFA0;
	sub_82EA2640(ctx, base);
	// 82F4DFA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4DFA4: 4825A218  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4DFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4DFA8 size=88
    let mut pc: u32 = 0x82F4DFA8;
    'dispatch: loop {
        match pc {
            0x82F4DFA8 => {
    //   block [0x82F4DFA8..0x82F4E000)
	// 82F4DFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4DFAC: 4825A1C1  bl 0x831a816c
	ctx.lr = 0x82F4DFB0;
	sub_831A8130(ctx, base);
	// 82F4DFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4DFB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4DFB8: 897E0024  lbz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4DFBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4DFC0: 419A0038  beq cr6, 0x82f4dff8
	if ctx.cr[6].eq {
	pc = 0x82F4DFF8; continue 'dispatch;
	}
	// 82F4DFC4: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4DFC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F4DFCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4DFD0: 40990020  ble cr6, 0x82f4dff0
	if !ctx.cr[6].gt {
	pc = 0x82F4DFF0; continue 'dispatch;
	}
	// 82F4DFD4: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82F4DFD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4DFDC: 4BF54655  bl 0x82ea2630
	ctx.lr = 0x82F4DFE0;
	sub_82EA2630(ctx, base);
	// 82F4DFE0: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4DFE4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F4DFE8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4DFEC: 4198FFEC  blt cr6, 0x82f4dfd8
	if ctx.cr[6].lt {
	pc = 0x82F4DFD8; continue 'dispatch;
	}
	// 82F4DFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4DFF4: 997E0024  stb r11, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82F4DFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4DFFC: 4825A1C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E000 size=152
    let mut pc: u32 = 0x82F4E000;
    'dispatch: loop {
        match pc {
            0x82F4E000 => {
    //   block [0x82F4E000..0x82F4E098)
	// 82F4E000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E004: 4825A169  bl 0x831a816c
	ctx.lr = 0x82F4E008;
	sub_831A8130(ctx, base);
	// 82F4E008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E00C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4E014: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4E018: 90BF0168  stw r5, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[5].u32 ) };
	// 82F4E01C: 997F0164  stb r11, 0x164(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[11].u8 ) };
	// 82F4E020: 815F0148  lwz r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E024: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4E028: 909F016C  stw r4, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[4].u32 ) };
	// 82F4E02C: 98DF0170  stb r6, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[6].u8 ) };
	// 82F4E030: 40990028  ble cr6, 0x82f4e058
	if !ctx.cr[6].gt {
	pc = 0x82F4E058; continue 'dispatch;
	}
	// 82F4E034: 3BDF0048  addi r30, r31, 0x48
	ctx.r[30].s64 = ctx.r[31].s64 + 72;
	// 82F4E038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E03C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4E040: 4BF54601  bl 0x82ea2640
	ctx.lr = 0x82F4E044;
	sub_82EA2640(ctx, base);
	// 82F4E044: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E048: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4E04C: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82F4E050: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E054: 4198FFE4  blt cr6, 0x82f4e038
	if ctx.cr[6].lt {
	pc = 0x82F4E038; continue 'dispatch;
	}
	// 82F4E058: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F4E05C: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E060: 4BF545E1  bl 0x82ea2640
	ctx.lr = 0x82F4E064;
	sub_82EA2640(ctx, base);
	// 82F4E064: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E068: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4E06C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E070: 40990020  ble cr6, 0x82f4e090
	if !ctx.cr[6].gt {
	pc = 0x82F4E090; continue 'dispatch;
	}
	// 82F4E074: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82F4E078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4E07C: 4BF545B5  bl 0x82ea2630
	ctx.lr = 0x82F4E080;
	sub_82EA2630(ctx, base);
	// 82F4E080: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E084: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F4E088: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E08C: 4198FFEC  blt cr6, 0x82f4e078
	if ctx.cr[6].lt {
	pc = 0x82F4E078; continue 'dispatch;
	}
	// 82F4E090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4E094: 4825A128  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E098 size=104
    let mut pc: u32 = 0x82F4E098;
    'dispatch: loop {
        match pc {
            0x82F4E098 => {
    //   block [0x82F4E098..0x82F4E100)
	// 82F4E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4E0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4E0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4E0A8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F4E0AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E0B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E0B4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F4E0B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F4E0BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E0C0: 4BF7ED01  bl 0x82eccdc0
	ctx.lr = 0x82F4E0C4;
	sub_82ECCDC0(ctx, base);
	// 82F4E0C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4E0C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F4E0CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E0D0: 4BFFFE69  bl 0x82f4df38
	ctx.lr = 0x82F4E0D4;
	sub_82F4DF38(ctx, base);
	// 82F4E0D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E0D8: 4BFFFED1  bl 0x82f4dfa8
	ctx.lr = 0x82F4E0DC;
	sub_82F4DFA8(ctx, base);
	// 82F4E0DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E0E0: 4BF7ECE1  bl 0x82eccdc0
	ctx.lr = 0x82F4E0E4;
	sub_82ECCDC0(ctx, base);
	// 82F4E0E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4E0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E0F0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82F4E0F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4E0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E100 size=132
    let mut pc: u32 = 0x82F4E100;
    'dispatch: loop {
        match pc {
            0x82F4E100 => {
    //   block [0x82F4E100..0x82F4E184)
	// 82F4E100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E104: 4825A065  bl 0x831a8168
	ctx.lr = 0x82F4E108;
	sub_831A8130(ctx, base);
	// 82F4E108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E10C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4E110: 4BFFFE99  bl 0x82f4dfa8
	ctx.lr = 0x82F4E114;
	sub_82F4DFA8(ctx, base);
	// 82F4E114: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E118: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4E11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E120: 40990030  ble cr6, 0x82f4e150
	if !ctx.cr[6].gt {
	pc = 0x82F4E150; continue 'dispatch;
	}
	// 82F4E124: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 82F4E128: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82F4E12C: 9B9FFFFC  stb r28, -4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[28].u8 ) };
	// 82F4E130: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E138: 4BF54509  bl 0x82ea2640
	ctx.lr = 0x82F4E13C;
	sub_82EA2640(ctx, base);
	// 82F4E13C: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E140: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4E144: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82F4E148: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E14C: 4198FFE0  blt cr6, 0x82f4e12c
	if ctx.cr[6].lt {
	pc = 0x82F4E12C; continue 'dispatch;
	}
	// 82F4E150: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E154: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F4E158: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E15C: 40990020  ble cr6, 0x82f4e17c
	if !ctx.cr[6].gt {
	pc = 0x82F4E17C; continue 'dispatch;
	}
	// 82F4E160: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82F4E164: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4E168: 4BF544C9  bl 0x82ea2630
	ctx.lr = 0x82F4E16C;
	sub_82EA2630(ctx, base);
	// 82F4E16C: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E170: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F4E174: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E178: 4198FFEC  blt cr6, 0x82f4e164
	if ctx.cr[6].lt {
	pc = 0x82F4E164; continue 'dispatch;
	}
	// 82F4E17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4E180: 4825A038  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4E188 size=8
    let mut pc: u32 = 0x82F4E188;
    'dispatch: loop {
        match pc {
            0x82F4E188 => {
    //   block [0x82F4E188..0x82F4E190)
	// 82F4E188: 80630148  lwz r3, 0x148(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E190 size=84
    let mut pc: u32 = 0x82F4E190;
    'dispatch: loop {
        match pc {
            0x82F4E190 => {
    //   block [0x82F4E190..0x82F4E1E4)
	// 82F4E190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E194: 48259FD9  bl 0x831a816c
	ctx.lr = 0x82F4E198;
	sub_831A8130(ctx, base);
	// 82F4E198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E19C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4E1A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E1A4: 4BF7EBCD  bl 0x82eccd70
	ctx.lr = 0x82F4E1A8;
	sub_82ECCD70(ctx, base);
	// 82F4E1A8: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E1AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4E1B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E1B4: 40990028  ble cr6, 0x82f4e1dc
	if !ctx.cr[6].gt {
	pc = 0x82F4E1DC; continue 'dispatch;
	}
	// 82F4E1B8: 3BFE004C  addi r31, r30, 0x4c
	ctx.r[31].s64 = ctx.r[30].s64 + 76;
	// 82F4E1BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E1C0: 4BF7EBA9  bl 0x82eccd68
	ctx.lr = 0x82F4E1C4;
	sub_82ECCD68(ctx, base);
	// 82F4E1C4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4E1C8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4E1CC: 817E0148  lwz r11, 0x148(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E1D0: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82F4E1D4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E1D8: 4198FFE4  blt cr6, 0x82f4e1bc
	if ctx.cr[6].lt {
	pc = 0x82F4E1BC; continue 'dispatch;
	}
	// 82F4E1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4E1E0: 48259FDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E1E8 size=144
    let mut pc: u32 = 0x82F4E1E8;
    'dispatch: loop {
        match pc {
            0x82F4E1E8 => {
    //   block [0x82F4E1E8..0x82F4E278)
	// 82F4E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4E1F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4E1F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E1F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E1FC: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E200: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F4E204: 4099005C  ble cr6, 0x82f4e260
	if !ctx.cr[6].gt {
	pc = 0x82F4E260; continue 'dispatch;
	}
	// 82F4E208: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F4E20C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F4E210: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4E214: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 82F4E218: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E21C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F4E220: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4E224: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4E228: 394B0028  addi r10, r11, 0x28
	ctx.r[10].s64 = ctx.r[11].s64 + 40;
	// 82F4E22C: 386A0020  addi r3, r10, 0x20
	ctx.r[3].s64 = ctx.r[10].s64 + 32;
	// 82F4E230: 992B0044  stb r9, 0x44(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u8 ) };
	// 82F4E234: 4BF5440D  bl 0x82ea2640
	ctx.lr = 0x82F4E238;
	sub_82EA2640(ctx, base);
	// 82F4E238: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82F4E23C: 4BF543F5  bl 0x82ea2630
	ctx.lr = 0x82F4E240;
	sub_82EA2630(ctx, base);
	// 82F4E240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E244: 4BFFFF4D  bl 0x82f4e190
	ctx.lr = 0x82F4E248;
	sub_82F4E190(ctx, base);
	// 82F4E248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F4E24C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4E250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E25C: 4E800020  blr
	return;
	// 82F4E260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F4E264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4E268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4E278 size=896
    let mut pc: u32 = 0x82F4E278;
    'dispatch: loop {
        match pc {
            0x82F4E278 => {
    //   block [0x82F4E278..0x82F4E5F8)
	// 82F4E278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E27C: 48259EBD  bl 0x831a8138
	ctx.lr = 0x82F4E280;
	sub_831A8130(ctx, base);
	// 82F4E280: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82F4E284: 9421FBE0  stwu r1, -0x420(r1)
	ea = ctx.r[1].u32.wrapping_add(-1056 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E288: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F4E28C: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82F4E290: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E294: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F4E298: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4E29C: 813F014C  lwz r9, 0x14c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82F4E2A0: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F4E2A4: 4BC7FC5D  bl 0x82bcdf00
	ctx.lr = 0x82F4E2A8;
	sub_82BCDF00(ctx, base);
	// 82F4E2A8: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82F4E2AC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82F4E2B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4E2B4: 80886A30  lwz r4, 0x6a30(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82F4E2B8: 4BF52059  bl 0x82ea0310
	ctx.lr = 0x82F4E2BC;
	sub_82EA0310(ctx, base);
	// 82F4E2BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4E2C0: 4BF54521  bl 0x82ea27e0
	ctx.lr = 0x82F4E2C4;
	sub_82EA27E0(ctx, base);
	// 82F4E2C4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E2C8: 3A000014  li r16, 0x14
	ctx.r[16].s64 = 20;
	// 82F4E2CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F4E2D0: 7C7C802E  lwzx r3, r28, r16
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82F4E2D4: 83DF0158  lwz r30, 0x158(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82F4E2D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4E2DC: 4BF52685  bl 0x82ea0960
	ctx.lr = 0x82F4E2E0;
	sub_82EA0960(ctx, base);
	// 82F4E2E0: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 82F4E2E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4E2E8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82F4E2EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4E2F0: 4BF51F71  bl 0x82ea0260
	ctx.lr = 0x82F4E2F4;
	sub_82EA0260(ctx, base);
	// 82F4E2F4: 88FF015C  lbz r7, 0x15c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82F4E2F8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F4E2FC: 419A0018  beq cr6, 0x82f4e314
	if ctx.cr[6].eq {
	pc = 0x82F4E314; continue 'dispatch;
	}
	// 82F4E300: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82F4E304: 3C80001E  lis r4, 0x1e
	ctx.r[4].s64 = 1966080;
	// 82F4E308: 60848480  ori r4, r4, 0x8480
	ctx.r[4].u64 = ctx.r[4].u64 | 33920;
	// 82F4E30C: 7C7C582E  lwzx r3, r28, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4E310: 4BF58349  bl 0x82ea6658
	ctx.lr = 0x82F4E314;
	sub_82EA6658(ctx, base);
	// 82F4E314: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F4E318: 3A7B0020  addi r19, r27, 0x20
	ctx.r[19].s64 = ctx.r[27].s64 + 32;
	// 82F4E31C: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82F4E320: 7D7CC02E  lwzx r11, r28, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F4E324: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E328: 915B0028  stw r10, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82F4E32C: 7D3CC02E  lwzx r9, r28, r24
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F4E330: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4E334: 911B002C  stw r8, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82F4E338: 4BF542F9  bl 0x82ea2630
	ctx.lr = 0x82F4E33C;
	sub_82EA2630(ctx, base);
	// 82F4E33C: 80FB0024  lwz r7, 0x24(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4E340: 88DB001C  lbz r6, 0x1c(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F4E344: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F4E348: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F4E34C: 409A025C  bne cr6, 0x82f4e5a8
	if !ctx.cr[6].eq {
	pc = 0x82F4E5A8; continue 'dispatch;
	}
	// 82F4E350: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4E354: 3AE0001C  li r23, 0x1c
	ctx.r[23].s64 = 28;
	// 82F4E358: 3ABF0020  addi r21, r31, 0x20
	ctx.r[21].s64 = ctx.r[31].s64 + 32;
	// 82F4E35C: 3A9F000C  addi r20, r31, 0xc
	ctx.r[20].s64 = ctx.r[31].s64 + 12;
	// 82F4E360: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 82F4E364: C3EB9534  lfs f31, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F4E368: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82F4E36C: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 82F4E370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4E374: 419A000C  beq cr6, 0x82f4e380
	if ctx.cr[6].eq {
	pc = 0x82F4E380; continue 'dispatch;
	}
	// 82F4E378: 7C7CC02E  lwzx r3, r28, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F4E37C: 4BF58225  bl 0x82ea65a0
	ctx.lr = 0x82F4E380;
	sub_82EA65A0(ctx, base);
	// 82F4E380: 7D7CC02E  lwzx r11, r28, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F4E384: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82F4E388: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4E38C: 915B002C  stw r10, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82F4E390: 4BF542A1  bl 0x82ea2630
	ctx.lr = 0x82F4E394;
	sub_82EA2630(ctx, base);
	// 82F4E394: 7E5CB92E  stwx r18, r28, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[23].u32), ctx.r[18].u32) };
	// 82F4E398: 893F0164  lbz r9, 0x164(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 82F4E39C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82F4E3A0: 409A01C0  bne cr6, 0x82f4e560
	if !ctx.cr[6].eq {
	pc = 0x82F4E560; continue 'dispatch;
	}
	// 82F4E3A4: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4E3A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E3AC: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82F4E3B0: 409A0050  bne cr6, 0x82f4e400
	if !ctx.cr[6].eq {
	pc = 0x82F4E400; continue 'dispatch;
	}
	// 82F4E3B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E3B8: 409A001C  bne cr6, 0x82f4e3d4
	if !ctx.cr[6].eq {
	pc = 0x82F4E3D4; continue 'dispatch;
	}
	// 82F4E3BC: C03F0004  lfs f1, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4E3C0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E3C4: 4BF7E995  bl 0x82eccd58
	ctx.lr = 0x82F4E3C8;
	sub_82ECCD58(ctx, base);
	// 82F4E3C8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F4E3CC: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E3D0: 4BF54271  bl 0x82ea2640
	ctx.lr = 0x82F4E3D4;
	sub_82EA2640(ctx, base);
	// 82F4E3D4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F4E3D8: 4BF54259  bl 0x82ea2630
	ctx.lr = 0x82F4E3DC;
	sub_82EA2630(ctx, base);
	// 82F4E3DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4E3E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E3E4: 4BF7E97D  bl 0x82eccd60
	ctx.lr = 0x82F4E3E8;
	sub_82ECCD60(ctx, base);
	// 82F4E3E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E3EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E3F0: 409A0188  bne cr6, 0x82f4e578
	if !ctx.cr[6].eq {
	pc = 0x82F4E578; continue 'dispatch;
	}
	// 82F4E3F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E3F8: 4BF7F841  bl 0x82ecdc38
	ctx.lr = 0x82F4E3FC;
	sub_82ECDC38(ctx, base);
	// 82F4E3FC: 4800017C  b 0x82f4e578
	pc = 0x82F4E578; continue 'dispatch;
	// 82F4E400: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E404: 409A0020  bne cr6, 0x82f4e424
	if !ctx.cr[6].eq {
	pc = 0x82F4E424; continue 'dispatch;
	}
	// 82F4E408: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E40C: 4BF7E935  bl 0x82eccd40
	ctx.lr = 0x82F4E410;
	sub_82ECCD40(ctx, base);
	// 82F4E410: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E414: 4BF7F72D  bl 0x82ecdb40
	ctx.lr = 0x82F4E418;
	sub_82ECDB40(ctx, base);
	// 82F4E418: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82F4E41C: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E420: 4BF54221  bl 0x82ea2640
	ctx.lr = 0x82F4E424;
	sub_82EA2640(ctx, base);
	// 82F4E424: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82F4E428: 4BF54209  bl 0x82ea2630
	ctx.lr = 0x82F4E42C;
	sub_82EA2630(ctx, base);
	// 82F4E42C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E430: 4BF7E919  bl 0x82eccd48
	ctx.lr = 0x82F4E434;
	sub_82ECCD48(ctx, base);
	// 82F4E434: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82F4E438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4E43C: 409A013C  bne cr6, 0x82f4e578
	if !ctx.cr[6].eq {
	pc = 0x82F4E578; continue 'dispatch;
	}
	// 82F4E440: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 82F4E444: 3B3F0014  addi r25, r31, 0x14
	ctx.r[25].s64 = ctx.r[31].s64 + 20;
	// 82F4E448: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E44C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E450: 409A0038  bne cr6, 0x82f4e488
	if !ctx.cr[6].eq {
	pc = 0x82F4E488; continue 'dispatch;
	}
	// 82F4E454: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E458: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 82F4E45C: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4E460: 40810040  ble 0x82f4e4a0
	if !ctx.cr[0].gt {
	pc = 0x82F4E4A0; continue 'dispatch;
	}
	// 82F4E464: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82F4E468: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4E46C: 4BF541C5  bl 0x82ea2630
	ctx.lr = 0x82F4E470;
	sub_82EA2630(ctx, base);
	// 82F4E470: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E474: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F4E478: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F4E47C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E480: 4198FFE8  blt cr6, 0x82f4e468
	if ctx.cr[6].lt {
	pc = 0x82F4E468; continue 'dispatch;
	}
	// 82F4E484: 48000010  b 0x82f4e494
	pc = 0x82F4E494; continue 'dispatch;
	// 82F4E488: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E48C: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82F4E490: 4BF541B1  bl 0x82ea2640
	ctx.lr = 0x82F4E494;
	sub_82EA2640(ctx, base);
	// 82F4E494: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E498: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E49C: 409A001C  bne cr6, 0x82f4e4b8
	if !ctx.cr[6].eq {
	pc = 0x82F4E4B8; continue 'dispatch;
	}
	// 82F4E4A0: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4E4A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E4A8: 4BF7E8B1  bl 0x82eccd58
	ctx.lr = 0x82F4E4AC;
	sub_82ECCD58(ctx, base);
	// 82F4E4AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F4E4B0: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E4B4: 4BF5418D  bl 0x82ea2640
	ctx.lr = 0x82F4E4B8;
	sub_82EA2640(ctx, base);
	// 82F4E4B8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F4E4BC: 4BF54175  bl 0x82ea2630
	ctx.lr = 0x82F4E4C0;
	sub_82EA2630(ctx, base);
	// 82F4E4C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4E4C4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E4C8: 4BF7E899  bl 0x82eccd60
	ctx.lr = 0x82F4E4CC;
	sub_82ECCD60(ctx, base);
	// 82F4E4CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E4D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E4D4: 409A0020  bne cr6, 0x82f4e4f4
	if !ctx.cr[6].eq {
	pc = 0x82F4E4F4; continue 'dispatch;
	}
	// 82F4E4D8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E4DC: 4BF7F75D  bl 0x82ecdc38
	ctx.lr = 0x82F4E4E0;
	sub_82ECDC38(ctx, base);
	// 82F4E4E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E4E4: 4BF7E86D  bl 0x82eccd50
	ctx.lr = 0x82F4E4E8;
	sub_82ECCD50(ctx, base);
	// 82F4E4E8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F4E4EC: 809F0148  lwz r4, 0x148(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E4F0: 4BF54151  bl 0x82ea2640
	ctx.lr = 0x82F4E4F4;
	sub_82EA2640(ctx, base);
	// 82F4E4F4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F4E4F8: 4BF54139  bl 0x82ea2630
	ctx.lr = 0x82F4E4FC;
	sub_82EA2630(ctx, base);
	// 82F4E4FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4E500: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E504: 409A0038  bne cr6, 0x82f4e53c
	if !ctx.cr[6].eq {
	pc = 0x82F4E53C; continue 'dispatch;
	}
	// 82F4E508: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E50C: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 82F4E510: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4E514: 40810034  ble 0x82f4e548
	if !ctx.cr[0].gt {
	pc = 0x82F4E548; continue 'dispatch;
	}
	// 82F4E518: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82F4E51C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4E520: 4BF54111  bl 0x82ea2630
	ctx.lr = 0x82F4E524;
	sub_82EA2630(ctx, base);
	// 82F4E524: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E528: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F4E52C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F4E530: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E534: 4198FFE8  blt cr6, 0x82f4e51c
	if ctx.cr[6].lt {
	pc = 0x82F4E51C; continue 'dispatch;
	}
	// 82F4E538: 48000010  b 0x82f4e548
	pc = 0x82F4E548; continue 'dispatch;
	// 82F4E53C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E540: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82F4E544: 4BF540FD  bl 0x82ea2640
	ctx.lr = 0x82F4E548;
	sub_82EA2640(ctx, base);
	// 82F4E548: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E54C: 4BF7E7FD  bl 0x82eccd48
	ctx.lr = 0x82F4E550;
	sub_82ECCD48(ctx, base);
	// 82F4E550: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82F4E554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4E558: 419AFEF0  beq cr6, 0x82f4e448
	if ctx.cr[6].eq {
	pc = 0x82F4E448; continue 'dispatch;
	}
	// 82F4E55C: 4800001C  b 0x82f4e578
	pc = 0x82F4E578; continue 'dispatch;
	// 82F4E560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E564: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E568: 80DF016C  lwz r6, 0x16c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 82F4E56C: 807F0168  lwz r3, 0x168(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 82F4E570: 80AB006C  lwz r5, 0x6c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82F4E574: 4BFFDB4D  bl 0x82f4c0c0
	ctx.lr = 0x82F4E578;
	sub_82F4C0C0(ctx, base);
	// 82F4E578: 7D7CC02E  lwzx r11, r28, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F4E57C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E580: 7EDCB92E  stwx r22, r28, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[23].u32), ctx.r[22].u32) };
	// 82F4E584: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F4E588: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4E58C: 915B002C  stw r10, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82F4E590: 4BF540B1  bl 0x82ea2640
	ctx.lr = 0x82F4E594;
	sub_82EA2640(ctx, base);
	// 82F4E594: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82F4E598: 4BF54099  bl 0x82ea2630
	ctx.lr = 0x82F4E59C;
	sub_82EA2630(ctx, base);
	// 82F4E59C: 893B001C  lbz r9, 0x1c(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F4E5A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F4E5A4: 419AFDC8  beq cr6, 0x82f4e36c
	if ctx.cr[6].eq {
	pc = 0x82F4E36C; continue 'dispatch;
	}
	// 82F4E5A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F4E5AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E5B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4E5B4: 4BF51CAD  bl 0x82ea0260
	ctx.lr = 0x82F4E5B8;
	sub_82EA0260(ctx, base);
	// 82F4E5B8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82F4E5BC: 7C7C802E  lwzx r3, r28, r16
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82F4E5C0: 4BF523F9  bl 0x82ea09b8
	ctx.lr = 0x82F4E5C4;
	sub_82EA09B8(ctx, base);
	// 82F4E5C4: 4BF54245  bl 0x82ea2808
	ctx.lr = 0x82F4E5C8;
	sub_82EA2808(ctx, base);
	// 82F4E5C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E5CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82F4E5D0: 4BF54071  bl 0x82ea2640
	ctx.lr = 0x82F4E5D4;
	sub_82EA2640(ctx, base);
	// 82F4E5D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4E5D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4E5DC: 394BBB5C  addi r10, r11, -0x44a4
	ctx.r[10].s64 = ctx.r[11].s64 + -17572;
	// 82F4E5E0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F4E5E4: 4BF52035  bl 0x82ea0618
	ctx.lr = 0x82F4E5E8;
	sub_82EA0618(ctx, base);
	// 82F4E5E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F4E5EC: 38210420  addi r1, r1, 0x420
	ctx.r[1].s64 = ctx.r[1].s64 + 1056;
	// 82F4E5F0: CBE1FF70  lfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82F4E5F4: 48259B94  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4E5F8 size=60
    let mut pc: u32 = 0x82F4E5F8;
    'dispatch: loop {
        match pc {
            0x82F4E5F8 => {
    //   block [0x82F4E5F8..0x82F4E634)
	// 82F4E5F8: 3D20003D  lis r9, 0x3d
	ctx.r[9].s64 = 3997696;
	// 82F4E5FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4E600: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4E604: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F4E608: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F4E60C: 61270900  ori r7, r9, 0x900
	ctx.r[7].u64 = ctx.r[9].u64 | 2304;
	// 82F4E610: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82F4E614: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F4E618: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F4E61C: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82F4E620: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F4E624: 99430018  stb r10, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82F4E628: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82F4E62C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82F4E630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4E638 size=268
    let mut pc: u32 = 0x82F4E638;
    'dispatch: loop {
        match pc {
            0x82F4E638 => {
    //   block [0x82F4E638..0x82F4E744)
	// 82F4E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E63C: 48259B29  bl 0x831a8164
	ctx.lr = 0x82F4E640;
	sub_831A8130(ctx, base);
	// 82F4E640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E644: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4E648: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E64C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E650: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82F4E654: 4BF53F95  bl 0x82ea25e8
	ctx.lr = 0x82F4E658;
	sub_82EA25E8(ctx, base);
	// 82F4E658: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E65C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E660: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82F4E664: 4BF53F85  bl 0x82ea25e8
	ctx.lr = 0x82F4E668;
	sub_82EA25E8(ctx, base);
	// 82F4E668: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E66C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E670: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82F4E674: 4BF53F75  bl 0x82ea25e8
	ctx.lr = 0x82F4E678;
	sub_82EA25E8(ctx, base);
	// 82F4E678: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E67C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E680: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82F4E684: 4BF53F65  bl 0x82ea25e8
	ctx.lr = 0x82F4E688;
	sub_82EA25E8(ctx, base);
	// 82F4E688: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E68C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E690: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 82F4E694: 4BF53F55  bl 0x82ea25e8
	ctx.lr = 0x82F4E698;
	sub_82EA25E8(ctx, base);
	// 82F4E698: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F4E69C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E6A0: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 82F4E6A4: 4BF53F45  bl 0x82ea25e8
	ctx.lr = 0x82F4E6A8;
	sub_82EA25E8(ctx, base);
	// 82F4E6A8: 3BFE0028  addi r31, r30, 0x28
	ctx.r[31].s64 = ctx.r[30].s64 + 40;
	// 82F4E6AC: 3B800005  li r28, 5
	ctx.r[28].s64 = 5;
	// 82F4E6B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4E6B4: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82F4E6B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82F4E6BC: 4BF5BAED  bl 0x82eaa1a8
	ctx.lr = 0x82F4E6C0;
	sub_82EAA1A8(ctx, base);
	// 82F4E6C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F4E6C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F4E6C8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F4E6CC: 4BF53F1D  bl 0x82ea25e8
	ctx.lr = 0x82F4E6D0;
	sub_82EA25E8(ctx, base);
	// 82F4E6D0: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82F4E6D4: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82F4E6D8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F4E6DC: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 82F4E6E0: 937F0018  stw r27, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 82F4E6E4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82F4E6E8: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82F4E6EC: 4080FFCC  bge 0x82f4e6b8
	if !ctx.cr[0].lt {
	pc = 0x82F4E6B8; continue 'dispatch;
	}
	// 82F4E6F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F4E6F4: 93BE014C  stw r29, 0x14c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(332 as u32), ctx.r[29].u32 ) };
	// 82F4E6F8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4E6FC: 93BE0150  stw r29, 0x150(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[29].u32 ) };
	// 82F4E700: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F4E704: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F4E708: 911E0154  stw r8, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[8].u32 ) };
	// 82F4E70C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4E710: 93BE0148  stw r29, 0x148(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(328 as u32), ctx.r[29].u32 ) };
	// 82F4E714: C00AC0AC  lfs f0, -0x3f54(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4E718: 9BBE0024  stb r29, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82F4E71C: C1A99534  lfs f13, -0x6acc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4E720: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4E724: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82F4E728: D1BE0008  stfs f13, 8(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4E72C: 997E0164  stb r11, 0x164(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(356 as u32), ctx.r[11].u8 ) };
	// 82F4E730: 93BE0168  stw r29, 0x168(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(360 as u32), ctx.r[29].u32 ) };
	// 82F4E734: 93BE016C  stw r29, 0x16c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(364 as u32), ctx.r[29].u32 ) };
	// 82F4E738: 997E0170  stb r11, 0x170(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(368 as u32), ctx.r[11].u8 ) };
	// 82F4E73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4E740: 48259A74  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E748 size=124
    let mut pc: u32 = 0x82F4E748;
    'dispatch: loop {
        match pc {
            0x82F4E748 => {
    //   block [0x82F4E748..0x82F4E7C4)
	// 82F4E748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4E750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4E754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E75C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E760: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4E764: 419A003C  beq cr6, 0x82f4e7a0
	if ctx.cr[6].eq {
	pc = 0x82F4E7A0; continue 'dispatch;
	}
	// 82F4E768: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4E76C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4E770: 419A0030  beq cr6, 0x82f4e7a0
	if ctx.cr[6].eq {
	pc = 0x82F4E7A0; continue 'dispatch;
	}
	// 82F4E774: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F4E778: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F4E77C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F4E780: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F4E784: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4E788: 409A0018  bne cr6, 0x82f4e7a0
	if !ctx.cr[6].eq {
	pc = 0x82F4E7A0; continue 'dispatch;
	}
	// 82F4E78C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E790: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4E794: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E798: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4E79C: 4E800421  bctrl
	ctx.lr = 0x82F4E7A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4E7A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E7A4: 4BFFF95D  bl 0x82f4e100
	ctx.lr = 0x82F4E7A8;
	sub_82F4E100(ctx, base);
	// 82F4E7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E7AC: 480003E5  bl 0x82f4eb90
	ctx.lr = 0x82F4E7B0;
	sub_82F4EB90(ctx, base);
	// 82F4E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E7BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E7C8 size=208
    let mut pc: u32 = 0x82F4E7C8;
    'dispatch: loop {
        match pc {
            0x82F4E7C8 => {
    //   block [0x82F4E7C8..0x82F4E898)
	// 82F4E7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E7CC: 482599A1  bl 0x831a816c
	ctx.lr = 0x82F4E7D0;
	sub_831A8130(ctx, base);
	// 82F4E7D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E7D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E7D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F4E7DC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F4E7E0: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4E7E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4E7E8: 419A0010  beq cr6, 0x82f4e7f8
	if ctx.cr[6].eq {
	pc = 0x82F4E7F8; continue 'dispatch;
	}
	// 82F4E7EC: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F4E7F0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F4E7F4: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4E7F8: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 82F4E7FC: 40990008  ble cr6, 0x82f4e804
	if !ctx.cr[6].gt {
	pc = 0x82F4E804; continue 'dispatch;
	}
	// 82F4E800: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 82F4E804: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4E808: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F4E80C: 40990048  ble cr6, 0x82f4e854
	if !ctx.cr[6].gt {
	pc = 0x82F4E854; continue 'dispatch;
	}
	// 82F4E810: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E814: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F4E818: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4E81C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F4E820: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F4E824: 911F0148  stw r8, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 82F4E828: 3889E278  addi r4, r9, -0x1d88
	ctx.r[4].s64 = ctx.r[9].s64 + -7560;
	// 82F4E82C: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4E830: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4E834: 38AB0028  addi r5, r11, 0x28
	ctx.r[5].s64 = ctx.r[11].s64 + 40;
	// 82F4E838: 38650008  addi r3, r5, 8
	ctx.r[3].s64 = ctx.r[5].s64 + 8;
	// 82F4E83C: 93EB0028  stw r31, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82F4E840: 93CB0040  stw r30, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82F4E844: 4BF5B9B5  bl 0x82eaa1f8
	ctx.lr = 0x82F4E848;
	sub_82EAA1F8(ctx, base);
	// 82F4E848: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F4E84C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F4E850: 4198FFC0  blt cr6, 0x82f4e810
	if ctx.cr[6].lt {
	pc = 0x82F4E810; continue 'dispatch;
	}
	// 82F4E854: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E858: 4BF7E519  bl 0x82eccd70
	ctx.lr = 0x82F4E85C;
	sub_82ECCD70(ctx, base);
	// 82F4E85C: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E860: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F4E864: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4E868: 40990028  ble cr6, 0x82f4e890
	if !ctx.cr[6].gt {
	pc = 0x82F4E890; continue 'dispatch;
	}
	// 82F4E86C: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 82F4E870: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E874: 4BF7E4F5  bl 0x82eccd68
	ctx.lr = 0x82F4E878;
	sub_82ECCD68(ctx, base);
	// 82F4E878: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4E87C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4E880: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E884: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82F4E888: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4E88C: 4198FFE4  blt cr6, 0x82f4e870
	if ctx.cr[6].lt {
	pc = 0x82F4E870; continue 'dispatch;
	}
	// 82F4E890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4E894: 48259928  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E898 size=188
    let mut pc: u32 = 0x82F4E898;
    'dispatch: loop {
        match pc {
            0x82F4E898 => {
    //   block [0x82F4E898..0x82F4E954)
	// 82F4E898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4E8A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4E8A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E8A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4E8AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E8B0: 814B00DC  lwz r10, 0xdc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(220 as u32) ) } as u64;
	// 82F4E8B4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82F4E8B8: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 82F4E8BC: 40980008  bge cr6, 0x82f4e8c4
	if !ctx.cr[6].lt {
	pc = 0x82F4E8C4; continue 'dispatch;
	}
	// 82F4E8C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4E8C4: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E8C8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F4E8CC: 40980070  bge cr6, 0x82f4e93c
	if !ctx.cr[6].lt {
	pc = 0x82F4E93C; continue 'dispatch;
	}
	// 82F4E8D0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F4E8D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F4E8D8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F4E8DC: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F4E8E0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4E8E4: 3889E278  addi r4, r9, -0x1d88
	ctx.r[4].s64 = ctx.r[9].s64 + -7560;
	// 82F4E8E8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4E8EC: 38AB0028  addi r5, r11, 0x28
	ctx.r[5].s64 = ctx.r[11].s64 + 40;
	// 82F4E8F0: 38650008  addi r3, r5, 8
	ctx.r[3].s64 = ctx.r[5].s64 + 8;
	// 82F4E8F4: 93EB0028  stw r31, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82F4E8F8: 811F0148  lwz r8, 0x148(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E8FC: 910B0040  stw r8, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82F4E900: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F4E904: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F4E908: 994B0044  stb r10, 0x44(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 82F4E90C: 4BF5B8ED  bl 0x82eaa1f8
	ctx.lr = 0x82F4E910;
	sub_82EAA1F8(ctx, base);
	// 82F4E910: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4E914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E918: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F4E91C: 90FF0148  stw r7, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[7].u32 ) };
	// 82F4E920: 4BFFF871  bl 0x82f4e190
	ctx.lr = 0x82F4E924;
	sub_82F4E190(ctx, base);
	// 82F4E924: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F4E928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4E92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E938: 4E800020  blr
	return;
	// 82F4E93C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F4E940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4E944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4E948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4E94C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4E950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4E958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4E958 size=364
    let mut pc: u32 = 0x82F4E958;
    'dispatch: loop {
        match pc {
            0x82F4E958 => {
    //   block [0x82F4E958..0x82F4EAC4)
	// 82F4E958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4E95C: 4825980D  bl 0x831a8168
	ctx.lr = 0x82F4E960;
	sub_831A8130(ctx, base);
	// 82F4E960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4E964: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F4E968: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4E96C: 4BFFFCCD  bl 0x82f4e638
	ctx.lr = 0x82F4E970;
	sub_82F4E638(ctx, base);
	// 82F4E970: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4E974: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F4E978: 3BFD014C  addi r31, r29, 0x14c
	ctx.r[31].s64 = ctx.r[29].s64 + 332;
	// 82F4E97C: 917D0158  stw r11, 0x158(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 82F4E980: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F4E984: 915D0160  stw r10, 0x160(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(352 as u32), ctx.r[10].u32 ) };
	// 82F4E988: 893E0018  lbz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F4E98C: 993D015C  stb r9, 0x15c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(348 as u32), ctx.r[9].u8 ) };
	// 82F4E990: 811E0020  lwz r8, 0x20(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4E994: 911D0168  stw r8, 0x168(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(360 as u32), ctx.r[8].u32 ) };
	// 82F4E998: 939D0150  stw r28, 0x150(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(336 as u32), ctx.r[28].u32 ) };
	// 82F4E99C: 80FD0154  lwz r7, 0x154(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82F4E9A0: 54EB00BE  clrlwi r11, r7, 2
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82F4E9A4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82F4E9A8: 40980020  bge cr6, 0x82f4e9c8
	if !ctx.cr[6].lt {
	pc = 0x82F4E9C8; continue 'dispatch;
	}
	// 82F4E9AC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F4E9B0: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 82F4E9B4: 41990008  bgt cr6, 0x82f4e9bc
	if ctx.cr[6].gt {
	pc = 0x82F4E9BC; continue 'dispatch;
	}
	// 82F4E9B8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82F4E9BC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F4E9C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4E9C4: 4BF57E35  bl 0x82ea67f8
	ctx.lr = 0x82F4E9C8;
	sub_82EA67F8(ctx, base);
	// 82F4E9C8: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82F4E9CC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F4E9D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F4E9D4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82F4E9D8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E9DC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F4E9E0: 93880000  stw r28, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82F4E9E4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82F4E9E8: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E9EC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82F4E9F0: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82F4E9F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4E9F8: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82F4E9FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EA00: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F4EA04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EA08: 90CA0010  stw r6, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82F4EA0C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EA10: 90890014  stw r4, 0x14(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82F4EA14: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4EA18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4EA1C: 4099008C  ble cr6, 0x82f4eaa8
	if !ctx.cr[6].gt {
	pc = 0x82F4EAA8; continue 'dispatch;
	}
	// 82F4EA20: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4EA24: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82F4EA28: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82F4EA2C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F4EA30: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F4EA34: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82F4EA38: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EA3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F4EA40: 7D2B312E  stwx r9, r11, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32), ctx.r[9].u32) };
	// 82F4EA44: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4EA48: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4EA4C: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82F4EA50: 7C8B482E  lwzx r4, r11, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F4EA54: 7CE32030  slw r3, r7, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[7].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 82F4EA58: 7C684378  or r8, r3, r8
	ctx.r[8].u64 = ctx.r[3].u64 | ctx.r[8].u64;
	// 82F4EA5C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F4EA60: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82F4EA64: 4198FFCC  blt cr6, 0x82f4ea30
	if ctx.cr[6].lt {
	pc = 0x82F4EA30; continue 'dispatch;
	}
	// 82F4EA68: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F4EA6C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4EA70: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 82F4EA74: 40980034  bge cr6, 0x82f4eaa8
	if !ctx.cr[6].lt {
	pc = 0x82F4EAA8; continue 'dispatch;
	}
	// 82F4EA78: 5569063E  clrlwi r9, r11, 0x18
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82F4EA7C: 5507063E  clrlwi r7, r8, 0x18
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82F4EA80: 7CE64C30  srw r6, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[7].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82F4EA84: 54C507FE  clrlwi r5, r6, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	// 82F4EA88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82F4EA8C: 409A0010  bne cr6, 0x82f4ea9c
	if !ctx.cr[6].eq {
	pc = 0x82F4EA9C; continue 'dispatch;
	}
	// 82F4EA90: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EA94: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82F4EA98: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F4EA9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F4EAA0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82F4EAA4: 4198FFCC  blt cr6, 0x82f4ea70
	if ctx.cr[6].lt {
	pc = 0x82F4EA70; continue 'dispatch;
	}
	// 82F4EAA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4EAAC: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4EAB0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EAB4: 4BFFFD15  bl 0x82f4e7c8
	ctx.lr = 0x82F4EAB8;
	sub_82F4E7C8(ctx, base);
	// 82F4EAB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4EABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4EAC0: 482596F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4EAC8 size=200
    let mut pc: u32 = 0x82F4EAC8;
    'dispatch: loop {
        match pc {
            0x82F4EAC8 => {
    //   block [0x82F4EAC8..0x82F4EB90)
	// 82F4EAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4EACC: 48259699  bl 0x831a8164
	ctx.lr = 0x82F4EAD0;
	sub_831A8130(ctx, base);
	// 82F4EAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4EAD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4EAD8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F4EADC: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4EAE0: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82F4EAE4: 40980014  bge cr6, 0x82f4eaf8
	if !ctx.cr[6].lt {
	pc = 0x82F4EAF8; continue 'dispatch;
	}
	// 82F4EAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4EAEC: 4BFFFDAD  bl 0x82f4e898
	ctx.lr = 0x82F4EAF0;
	sub_82F4E898(ctx, base);
	// 82F4EAF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4EAF4: 419AFFE8  beq cr6, 0x82f4eadc
	if ctx.cr[6].eq {
	pc = 0x82F4EADC; continue 'dispatch;
	}
	// 82F4EAF8: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82F4EAFC: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4EB00: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82F4EB04: 40990080  ble cr6, 0x82f4eb84
	if !ctx.cr[6].gt {
	pc = 0x82F4EB84; continue 'dispatch;
	}
	// 82F4EB08: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F4EB0C: 40990078  ble cr6, 0x82f4eb84
	if !ctx.cr[6].gt {
	pc = 0x82F4EB84; continue 'dispatch;
	}
	// 82F4EB10: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F4EB14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4EB18: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4EB1C: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 82F4EB20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F4EB24: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4EB28: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4EB2C: 394B0028  addi r10, r11, 0x28
	ctx.r[10].s64 = ctx.r[11].s64 + 40;
	// 82F4EB30: 386A0020  addi r3, r10, 0x20
	ctx.r[3].s64 = ctx.r[10].s64 + 32;
	// 82F4EB34: 9B8B0044  stb r28, 0x44(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[28].u8 ) };
	// 82F4EB38: 4BF53B09  bl 0x82ea2640
	ctx.lr = 0x82F4EB3C;
	sub_82EA2640(ctx, base);
	// 82F4EB3C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82F4EB40: 4BF53AF1  bl 0x82ea2630
	ctx.lr = 0x82F4EB44;
	sub_82EA2630(ctx, base);
	// 82F4EB44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EB48: 4BF7E229  bl 0x82eccd70
	ctx.lr = 0x82F4EB4C;
	sub_82ECCD70(ctx, base);
	// 82F4EB4C: 815F0148  lwz r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4EB50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4EB54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4EB58: 4099FFA4  ble cr6, 0x82f4eafc
	if !ctx.cr[6].gt {
	pc = 0x82F4EAFC; continue 'dispatch;
	}
	// 82F4EB5C: 3BBF004C  addi r29, r31, 0x4c
	ctx.r[29].s64 = ctx.r[31].s64 + 76;
	// 82F4EB60: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EB64: 4BF7E205  bl 0x82eccd68
	ctx.lr = 0x82F4EB68;
	sub_82ECCD68(ctx, base);
	// 82F4EB68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F4EB6C: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4EB70: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4EB74: 3BBD0030  addi r29, r29, 0x30
	ctx.r[29].s64 = ctx.r[29].s64 + 48;
	// 82F4EB78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4EB7C: 4198FFE4  blt cr6, 0x82f4eb60
	if ctx.cr[6].lt {
	pc = 0x82F4EB60; continue 'dispatch;
	}
	// 82F4EB80: 4BFFFF7C  b 0x82f4eafc
	pc = 0x82F4EAFC; continue 'dispatch;
	// 82F4EB84: 807F0148  lwz r3, 0x148(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 82F4EB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4EB8C: 48259628  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4EB90 size=156
    let mut pc: u32 = 0x82F4EB90;
    'dispatch: loop {
        match pc {
            0x82F4EB90 => {
    //   block [0x82F4EB90..0x82F4EC2C)
	// 82F4EB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4EB94: 482595D9  bl 0x831a816c
	ctx.lr = 0x82F4EB98;
	sub_831A8130(ctx, base);
	// 82F4EB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4EB9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F4EBA0: 817D0154  lwz r11, 0x154(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82F4EBA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F4EBA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4EBAC: 409A0020  bne cr6, 0x82f4ebcc
	if !ctx.cr[6].eq {
	pc = 0x82F4EBCC; continue 'dispatch;
	}
	// 82F4EBB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4EBB4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F4EBB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F4EBBC: 809D014C  lwz r4, 0x14c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 82F4EBC0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F4EBC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4EBC8: 4BF51BE9  bl 0x82ea07b0
	ctx.lr = 0x82F4EBCC;
	sub_82EA07B0(ctx, base);
	// 82F4EBCC: 397D0148  addi r11, r29, 0x148
	ctx.r[11].s64 = ctx.r[29].s64 + 328;
	// 82F4EBD0: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 82F4EBD4: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82F4EBD8: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 82F4EBDC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82F4EBE0: 4BF53A49  bl 0x82ea2628
	ctx.lr = 0x82F4EBE4;
	sub_82EA2628(ctx, base);
	// 82F4EBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4EBE8: 4BF5B6F1  bl 0x82eaa2d8
	ctx.lr = 0x82F4EBEC;
	sub_82EAA2D8(ctx, base);
	// 82F4EBEC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F4EBF0: 4080FFE8  bge 0x82f4ebd8
	if !ctx.cr[0].lt {
	pc = 0x82F4EBD8; continue 'dispatch;
	}
	// 82F4EBF4: 387D0020  addi r3, r29, 0x20
	ctx.r[3].s64 = ctx.r[29].s64 + 32;
	// 82F4EBF8: 4BF53A31  bl 0x82ea2628
	ctx.lr = 0x82F4EBFC;
	sub_82EA2628(ctx, base);
	// 82F4EBFC: 387D001C  addi r3, r29, 0x1c
	ctx.r[3].s64 = ctx.r[29].s64 + 28;
	// 82F4EC00: 4BF53A29  bl 0x82ea2628
	ctx.lr = 0x82F4EC04;
	sub_82EA2628(ctx, base);
	// 82F4EC04: 387D0018  addi r3, r29, 0x18
	ctx.r[3].s64 = ctx.r[29].s64 + 24;
	// 82F4EC08: 4BF53A21  bl 0x82ea2628
	ctx.lr = 0x82F4EC0C;
	sub_82EA2628(ctx, base);
	// 82F4EC0C: 387D0014  addi r3, r29, 0x14
	ctx.r[3].s64 = ctx.r[29].s64 + 20;
	// 82F4EC10: 4BF53A19  bl 0x82ea2628
	ctx.lr = 0x82F4EC14;
	sub_82EA2628(ctx, base);
	// 82F4EC14: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82F4EC18: 4BF53A11  bl 0x82ea2628
	ctx.lr = 0x82F4EC1C;
	sub_82EA2628(ctx, base);
	// 82F4EC1C: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82F4EC20: 4BF53A09  bl 0x82ea2628
	ctx.lr = 0x82F4EC24;
	sub_82EA2628(ctx, base);
	// 82F4EC24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4EC28: 48259594  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EC30 size=24
    let mut pc: u32 = 0x82F4EC30;
    'dispatch: loop {
        match pc {
            0x82F4EC30 => {
    //   block [0x82F4EC30..0x82F4EC48)
	// 82F4EC30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4EC34: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EC38: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82F4EC3C: 4199000C  bgt cr6, 0x82f4ec48
	if ctx.cr[6].gt {
		sub_82F4EC48(ctx, base);
		return;
	}
	// 82F4EC40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F4EC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4EC48 size=144
    let mut pc: u32 = 0x82F4EC48;
    'dispatch: loop {
        match pc {
            0x82F4EC48 => {
    //   block [0x82F4EC48..0x82F4ECD8)
	// 82F4EC48: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82F4EC4C: 4099FFF4  ble cr6, 0x82f4ec40
	if !ctx.cr[6].gt {
		sub_82F4EC30(ctx, base);
		return;
	}
	// 82F4EC50: EC010072  fmuls f0, f1, f1
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4EC54: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82F4EC58: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4EC5C: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82F4EC60: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F4EC64: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4ECD8 size=24
    let mut pc: u32 = 0x82F4ECD8;
    'dispatch: loop {
        match pc {
            0x82F4ECD8 => {
    //   block [0x82F4ECD8..0x82F4ECF0)
	// 82F4ECD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4ECDC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4ECE0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82F4ECE4: 4199000C  bgt cr6, 0x82f4ecf0
	if ctx.cr[6].gt {
		sub_82F4ECF0(ctx, base);
		return;
	}
	// 82F4ECE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F4ECEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4ECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4ECF0 size=180
    let mut pc: u32 = 0x82F4ECF0;
    'dispatch: loop {
        match pc {
            0x82F4ECF0 => {
    //   block [0x82F4ECF0..0x82F4EDA4)
	// 82F4ECF0: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4ECF4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82F4ECF8: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4ECFC: ED6D0372  fmuls f11, f13, f13
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4ED00: C1430008  lfs f10, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4ED04: ED2C0332  fmuls f9, f12, f12
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4ED08: ED0A02B2  fmuls f8, f10, f10
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[10].f64) as f32) as f64);
	// 82F4ED0C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82F4ED10: ECEA0332  fmuls f7, f10, f12
	ctx.f[7].f64 = (((ctx.f[10].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4ED14: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82F4ED18: C00A603C  lfs f0, 0x603c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24636 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4ED1C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F4ED20: ECC10032  fmuls f6, f1, f0
	ctx.f[6].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4ED24: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82F4ED28: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EDA8 size=64
    let mut pc: u32 = 0x82F4EDA8;
    'dispatch: loop {
        match pc {
            0x82F4EDA8 => {
    //   block [0x82F4EDA8..0x82F4EDE8)
	// 82F4EDA8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82F4EDAC: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EDB0: D0040090  stfs f0, 0x90(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82F4EDB4: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 82F4EDB8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F4EDBC: 39440050  addi r10, r4, 0x50
	ctx.r[10].s64 = ctx.r[4].s64 + 80;
	// 82F4EDC0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4EDE8 size=128
    let mut pc: u32 = 0x82F4EDE8;
    'dispatch: loop {
        match pc {
            0x82F4EDE8 => {
    //   block [0x82F4EDE8..0x82F4EE68)
	// 82F4EDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4EDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4EDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4EDF4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82F4EDF8: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EDFC: D0250090  stfs f1, 0x90(r5)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82F4EE00: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 82F4EE04: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F4EE08: EDA10024  fdivs f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 82F4EE0C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F4EE10: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F4EE14: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EE68 size=12
    let mut pc: u32 = 0x82F4EE68;
    'dispatch: loop {
        match pc {
            0x82F4EE68 => {
    //   block [0x82F4EE68..0x82F4EE74)
	// 82F4EE68: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EE6C: EC200072  fmuls f1, f0, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4EE70: 4BFFFF78  b 0x82f4ede8
	sub_82F4EDE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EE78 size=72
    let mut pc: u32 = 0x82F4EE78;
    'dispatch: loop {
        match pc {
            0x82F4EE78 => {
    //   block [0x82F4EE78..0x82F4EEC0)
	// 82F4EE78: C0040050  lfs f0, 0x50(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EE7C: C1A40064  lfs f13, 0x64(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4EE80: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82F4EE84: C1640078  lfs f11, 0x78(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4EE88: FD4C682E  fsel f10, f12, f0, f13
	ctx.f[10].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82F4EE8C: ED2A5828  fsubs f9, f10, f11
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 82F4EE90: FD095AAE  fsel f8, f9, f10, f11
	ctx.f[8].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[11].f64 };
	// 82F4EE94: ECE80824  fdivs f7, f8, f1
	ctx.f[7].f64 = ((ctx.f[8].f64 / ctx.f[1].f64) as f32) as f64;
	// 82F4EE98: ECC03828  fsubs f6, f0, f7
	ctx.f[6].f64 = (((ctx.f[0].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4EE9C: ECAD3828  fsubs f5, f13, f7
	ctx.f[5].f64 = (((ctx.f[13].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4EEA0: EC8B3828  fsubs f4, f11, f7
	ctx.f[4].f64 = (((ctx.f[11].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4EEA4: FC66382E  fsel f3, f6, f0, f7
	ctx.f[3].f64 = if ctx.f[6].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[7].f64 };
	// 82F4EEA8: D0640050  stfs f3, 0x50(r4)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F4EEAC: FC453B6E  fsel f2, f5, f13, f7
	ctx.f[2].f64 = if ctx.f[5].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[7].f64 };
	// 82F4EEB0: D0440064  stfs f2, 0x64(r4)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F4EEB4: FC243AEE  fsel f1, f4, f11, f7
	ctx.f[1].f64 = if ctx.f[4].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[7].f64 };
	// 82F4EEB8: D0240078  stfs f1, 0x78(r4)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82F4EEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EEC0 size=172
    let mut pc: u32 = 0x82F4EEC0;
    'dispatch: loop {
        match pc {
            0x82F4EEC0 => {
    //   block [0x82F4EEC0..0x82F4EF6C)
	// 82F4EEC0: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EEC4: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4EEC8: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4EECC: C1650000  lfs f11, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4EED0: ED4C6B3A  fmadds f10, f12, f12, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F4EED4: ED2A587C  fnmsubs f9, f10, f1, f11
	ctx.f[9].f64 = -(((ctx.f[10].f64 * ctx.f[1].f64 - ctx.f[11].f64) as f32) as f64);
	// 82F4EED8: D1250000  stfs f9, 0(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F4EEDC: C1030000  lfs f8, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4EEE0: C0E50014  lfs f7, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4EEE4: C0C30008  lfs f6, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4EEE8: ECA601B2  fmuls f5, f6, f6
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4EEEC: EC882A3A  fmadds f4, f8, f8, f5
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4EEF0: EC64387C  fnmsubs f3, f4, f1, f7
	ctx.f[3].f64 = -(((ctx.f[4].f64 * ctx.f[1].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4EEF4: D0650014  stfs f3, 0x14(r5)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4EEF8: C0430000  lfs f2, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4EEFC: C0050028  lfs f0, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EF00: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4EF04: ED8D0372  fmuls f12, f13, f13
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4EF08: ED6260BA  fmadds f11, f2, f2, f12
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[2].f64 + ctx.f[12].f64) as f32) as f64);
	// 82F4EF0C: ED4B007C  fnmsubs f10, f11, f1, f0
	ctx.f[10].f64 = -(((ctx.f[11].f64 * ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4EF10: D1450028  stfs f10, 0x28(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4EF14: C1230004  lfs f9, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4EF18: C1050004  lfs f8, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4EF1C: C0E30000  lfs f7, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4EF20: ECC70072  fmuls f6, f7, f1
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4EF24: ECA6427A  fmadds f5, f6, f9, f8
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[9].f64 + ctx.f[8].f64) as f32) as f64);
	// 82F4EF28: D0A50004  stfs f5, 4(r5)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4EF2C: D0A50010  stfs f5, 0x10(r5)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4EF30: C0830004  lfs f4, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4EF34: C0650018  lfs f3, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4EF38: C0430008  lfs f2, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4EF3C: EC0100B2  fmuls f0, f1, f2
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4EF40: EDA0193A  fmadds f13, f0, f4, f3
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[4].f64 + ctx.f[3].f64) as f32) as f64);
	// 82F4EF44: D1A50018  stfs f13, 0x18(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4EF48: D1A50024  stfs f13, 0x24(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4EF4C: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4EF50: C1650020  lfs f11, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4EF54: C1430000  lfs f10, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4EF58: ED2A0072  fmuls f9, f10, f1
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4EF5C: ED095B3A  fmadds f8, f9, f12, f11
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 82F4EF60: D1050020  stfs f8, 0x20(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4EF64: D1050008  stfs f8, 8(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4EF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4EF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4EF70 size=172
    let mut pc: u32 = 0x82F4EF70;
    'dispatch: loop {
        match pc {
            0x82F4EF70 => {
    //   block [0x82F4EF70..0x82F4F01C)
	// 82F4EF70: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EF74: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4EF78: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4EF7C: C1650000  lfs f11, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4EF80: ED4C6B3A  fmadds f10, f12, f12, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F4EF84: ED2A587A  fmadds f9, f10, f1, f11
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[1].f64 + ctx.f[11].f64) as f32) as f64);
	// 82F4EF88: D1250000  stfs f9, 0(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F4EF8C: C1030000  lfs f8, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4EF90: C0E50014  lfs f7, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4EF94: C0C30008  lfs f6, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4EF98: ECA601B2  fmuls f5, f6, f6
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4EF9C: EC882A3A  fmadds f4, f8, f8, f5
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4EFA0: EC64387A  fmadds f3, f4, f1, f7
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[1].f64 + ctx.f[7].f64) as f32) as f64);
	// 82F4EFA4: D0650014  stfs f3, 0x14(r5)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4EFA8: C0430000  lfs f2, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4EFAC: C0050028  lfs f0, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4EFB0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4EFB4: ED8D0372  fmuls f12, f13, f13
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4EFB8: ED6260BA  fmadds f11, f2, f2, f12
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[2].f64 + ctx.f[12].f64) as f32) as f64);
	// 82F4EFBC: ED4B007A  fmadds f10, f11, f1, f0
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 82F4EFC0: D1450028  stfs f10, 0x28(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4EFC4: C1230004  lfs f9, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4EFC8: C1050004  lfs f8, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4EFCC: C0E30000  lfs f7, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4EFD0: ECC70072  fmuls f6, f7, f1
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4EFD4: ECA6427C  fnmsubs f5, f6, f9, f8
	ctx.f[5].f64 = -(((ctx.f[6].f64 * ctx.f[9].f64 - ctx.f[8].f64) as f32) as f64);
	// 82F4EFD8: D0A50004  stfs f5, 4(r5)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4EFDC: D0A50010  stfs f5, 0x10(r5)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4EFE0: C0830004  lfs f4, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4EFE4: C0650018  lfs f3, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4EFE8: C0430008  lfs f2, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4EFEC: EC0100B2  fmuls f0, f1, f2
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4EFF0: EDA0193C  fnmsubs f13, f0, f4, f3
	ctx.f[13].f64 = -(((ctx.f[0].f64 * ctx.f[4].f64 - ctx.f[3].f64) as f32) as f64);
	// 82F4EFF4: D1A50018  stfs f13, 0x18(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4EFF8: D1A50024  stfs f13, 0x24(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4EFFC: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F000: C1650020  lfs f11, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F004: C1430000  lfs f10, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4F008: ED2A0072  fmuls f9, f10, f1
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F00C: ED095B3C  fnmsubs f8, f9, f12, f11
	ctx.f[8].f64 = -(((ctx.f[9].f64 * ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82F4F010: D1050020  stfs f8, 0x20(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F014: D1050008  stfs f8, 8(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4F018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4F020 size=80
    let mut pc: u32 = 0x82F4F020;
    'dispatch: loop {
        match pc {
            0x82F4F020 => {
    //   block [0x82F4F020..0x82F4F070)
	// 82F4F020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4F024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4F028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4F02C: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 82F4F030: D0210084  stfs f1, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82F4F034: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F4F038: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F4F03C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4F040: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4F070 size=76
    let mut pc: u32 = 0x82F4F070;
    'dispatch: loop {
        match pc {
            0x82F4F070 => {
    //   block [0x82F4F070..0x82F4F0BC)
	// 82F4F070: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F074: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82F4F078: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F07C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F4F080: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4F084: C1630028  lfs f11, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F088: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4F0C0 size=720
    let mut pc: u32 = 0x82F4F0C0;
    'dispatch: loop {
        match pc {
            0x82F4F0C0 => {
    //   block [0x82F4F0C0..0x82F4F390)
	// 82F4F0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4F0C4: 482590A9  bl 0x831a816c
	ctx.lr = 0x82F4F0C8;
	sub_831A8130(ctx, base);
	// 82F4F0C8: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82F4F0CC: 48259975  bl 0x831a8a40
	ctx.lr = 0x82F4F0D0;
	sub_831A8A40(ctx, base);
	// 82F4F0D0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82F4F0D4: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4F0D8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4F0DC: 3CC08208  lis r6, -0x7df8
	ctx.r[6].s64 = -2113404928;
	// 82F4F0E0: 3FA08201  lis r29, -0x7dff
	ctx.r[29].s64 = -2113863680;
	// 82F4F0E4: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4F0E8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4F0EC: C00708A4  lfs f0, 0x8a4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F0F0: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82F4F0F4: 3CA05555  lis r5, 0x5555
	ctx.r[5].s64 = 1431633920;
	// 82F4F0F8: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82F4F0FC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F4F100: 7D2B2214  add r9, r11, r4
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F4F104: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4F108: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F4F10C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4F110: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82F4F114: C1879524  lfs f12, -0x6adc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F118: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82F4F11C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F120: 60A55556  ori r5, r5, 0x5556
	ctx.r[5].u64 = ctx.r[5].u64 | 21846;
	// 82F4F124: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82F4F128: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4F12C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4F130: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4F134: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F4F138: C1A6E830  lfs f13, -0x17d0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F13C: C01DA1C4  lfs f0, -0x5e3c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F140: D181FF48  stfs f12, -0xb8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 82F4F144: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82F4F148: C06A0000  lfs f3, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4F14C: C0290000  lfs f1, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4F150: EC4300F2  fmuls f2, f3, f3
	ctx.f[2].f64 = (((ctx.f[3].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F154: 7CFD2896  mulhw r7, r29, r5
	ctx.r[7].s64 = ((ctx.r[29].s32 as i64 * ctx.r[5].s32 as i64) >> 32);
	// 82F4F158: ED610072  fmuls f11, f1, f1
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F15C: C143000C  lfs f10, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4F160: C0C30014  lfs f6, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4F164: C1230010  lfs f9, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4F168: C1030018  lfs f8, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F16C: C0A30020  lfs f5, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F4F170: C0830030  lfs f4, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4F174: C0E30024  lfs f7, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4F178: C3E3001C  lfs f31, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F4F17C: EFC200F2  fmuls f30, f2, f3
	ctx.f[30].f64 = (((ctx.f[2].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F180: C3A30028  lfs f29, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82F4F184: EF8B0072  fmuls f28, f11, f1
	ctx.f[28].f64 = (((ctx.f[11].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F188: C363002C  lfs f27, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82F4F18C: 54E60FFE  srwi r6, r7, 0x1f
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F4F190: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82F4F194: 54E6083C  slwi r6, r7, 1
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F4F198: EF5E0372  fmuls f26, f30, f13
	ctx.f[26].f64 = (((ctx.f[30].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4F19C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82F4F1A0: 7CC7E850  subf r6, r7, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[7].s64;
	// 82F4F1A4: 54C7103A  slwi r7, r6, 2
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F4F1A8: 7CC7FA14  add r6, r7, r31
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82F4F1AC: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82F4F1B0: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F4F1B4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F4F1B8: 7F26242E  lfsx f25, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82F4F1BC: EF1900F2  fmuls f24, f25, f3
	ctx.f[24].f64 = (((ctx.f[25].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F1C0: 7EE7242E  lfsx f23, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82F4F1C4: EE97082A  fadds f20, f23, f1
	ctx.f[20].f64 = ((ctx.f[23].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F4F1C8: EE770828  fsubs f19, f23, f1
	ctx.f[19].f64 = (((ctx.f[23].f64 - ctx.f[1].f64) as f32) as f64);
	// 82F4F1CC: EEB9182A  fadds f21, f25, f3
	ctx.f[21].f64 = ((ctx.f[25].f64 + ctx.f[3].f64) as f32) as f64;
	// 82F4F1D0: EED90672  fmuls f22, f25, f25
	ctx.f[22].f64 = (((ctx.f[25].f64 * ctx.f[25].f64) as f32) as f64);
	// 82F4F1D4: EE591828  fsubs f18, f25, f3
	ctx.f[18].f64 = (((ctx.f[25].f64 - ctx.f[3].f64) as f32) as f64);
	// 82F4F1D8: EE0B05F2  fmuls f16, f11, f23
	ctx.f[16].f64 = (((ctx.f[11].f64 * ctx.f[23].f64) as f32) as f64);
	// 82F4F1DC: EE3705F2  fmuls f17, f23, f23
	ctx.f[17].f64 = (((ctx.f[23].f64 * ctx.f[23].f64) as f32) as f64);
	// 82F4F1E0: EF180332  fmuls f24, f24, f12
	ctx.f[24].f64 = (((ctx.f[24].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4F1E4: EE945DFA  fmadds f20, f20, f23, f11
	ctx.f[20].f64 = (((ctx.f[20].f64 * ctx.f[23].f64 + ctx.f[11].f64) as f32) as f64);
	// 82F4F1E8: ED9554FA  fmadds f12, f21, f19, f10
	ctx.f[12].f64 = (((ctx.f[21].f64 * ctx.f[19].f64 + ctx.f[10].f64) as f32) as f64);
	// 82F4F1EC: D181FF4C  stfs f12, -0xb4(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 82F4F1F0: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F4F1F4: EDD5167A  fmadds f14, f21, f25, f2
	ctx.f[14].f64 = (((ctx.f[21].f64 * ctx.f[25].f64 + ctx.f[2].f64) as f32) as f64);
	// 82F4F1F8: EDF60672  fmuls f15, f22, f25
	ctx.f[15].f64 = (((ctx.f[22].f64 * ctx.f[25].f64) as f32) as f64);
	// 82F4F1FC: EEB105F2  fmuls f21, f17, f23
	ctx.f[21].f64 = (((ctx.f[17].f64 * ctx.f[23].f64) as f32) as f64);
	// 82F4F200: EE310072  fmuls f17, f17, f1
	ctx.f[17].f64 = (((ctx.f[17].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F204: ED96C03A  fmadds f12, f22, f0, f24
	ctx.f[12].f64 = (((ctx.f[22].f64 * ctx.f[0].f64 + ctx.f[24].f64) as f32) as f64);
	// 82F4F208: EF02C03A  fmadds f24, f2, f0, f24
	ctx.f[24].f64 = (((ctx.f[2].f64 * ctx.f[0].f64 + ctx.f[24].f64) as f32) as f64);
	// 82F4F20C: ED54E5FA  fmadds f10, f20, f23, f28
	ctx.f[10].f64 = (((ctx.f[20].f64 * ctx.f[23].f64 + ctx.f[28].f64) as f32) as f64);
	// 82F4F210: D141FF44  stfs f10, -0xbc(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 82F4F214: ED5434BA  fmadds f10, f20, f18, f6
	ctx.f[10].f64 = (((ctx.f[20].f64 * ctx.f[18].f64 + ctx.f[6].f64) as f32) as f64);
	// 82F4F218: D1430014  stfs f10, 0x14(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4F21C: ED6EF67A  fmadds f11, f14, f25, f30
	ctx.f[11].f64 = (((ctx.f[14].f64 * ctx.f[25].f64 + ctx.f[30].f64) as f32) as f64);
	// 82F4F220: D161FF40  stfs f11, -0xc0(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 82F4F224: ED6E4CFA  fmadds f11, f14, f19, f9
	ctx.f[11].f64 = (((ctx.f[14].f64 * ctx.f[19].f64 + ctx.f[9].f64) as f32) as f64);
	// 82F4F228: D1630010  stfs f11, 0x10(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4F22C: ECCC102A  fadds f6, f12, f2
	ctx.f[6].f64 = ((ctx.f[12].f64 + ctx.f[2].f64) as f32) as f64;
	// 82F4F230: C181FF40  lfs f12, -0xc0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-192 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F234: EC58B02A  fadds f2, f24, f22
	ctx.f[2].f64 = ((ctx.f[24].f64 + ctx.f[22].f64) as f32) as f64;
	// 82F4F238: C2C1FF44  lfs f22, -0xbc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-188 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82F4F23C: ED2C44FA  fmadds f9, f12, f19, f8
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[19].f64 + ctx.f[8].f64) as f32) as f64);
	// 82F4F240: D1230018  stfs f9, 0x18(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4F244: EF0C0672  fmuls f24, f12, f25
	ctx.f[24].f64 = (((ctx.f[12].f64 * ctx.f[25].f64) as f32) as f64);
	// 82F4F248: ED162CBA  fmadds f8, f22, f18, f5
	ctx.f[8].f64 = (((ctx.f[22].f64 * ctx.f[18].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F24C: D1030020  stfs f8, 0x20(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F250: EE9605F2  fmuls f20, f22, f23
	ctx.f[20].f64 = (((ctx.f[22].f64 * ctx.f[23].f64) as f32) as f64);
	// 82F4F254: ED82D67A  fmadds f12, f2, f25, f26
	ctx.f[12].f64 = (((ctx.f[2].f64 * ctx.f[25].f64 + ctx.f[26].f64) as f32) as f64);
	// 82F4F258: ECA600F2  fmuls f5, f6, f3
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F25C: EC420072  fmuls f2, f2, f1
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F260: EFDEC0FA  fmadds f30, f30, f3, f24
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[3].f64 + ctx.f[24].f64) as f32) as f64);
	// 82F4F264: EF5CA07A  fmadds f26, f28, f1, f20
	ctx.f[26].f64 = (((ctx.f[28].f64 * ctx.f[1].f64 + ctx.f[20].f64) as f32) as f64);
	// 82F4F268: EC2C0072  fmuls f1, f12, f1
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F26C: ECAF2B7A  fmadds f5, f15, f13, f5
	ctx.f[5].f64 = (((ctx.f[15].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F270: ED8615FA  fmadds f12, f6, f23, f2
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[23].f64 + ctx.f[2].f64) as f32) as f64);
	// 82F4F274: ECFE3CFA  fmadds f7, f30, f19, f7
	ctx.f[7].f64 = (((ctx.f[30].f64 * ctx.f[19].f64 + ctx.f[7].f64) as f32) as f64);
	// 82F4F278: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4F27C: ECDA24BA  fmadds f6, f26, f18, f4
	ctx.f[6].f64 = (((ctx.f[26].f64 * ctx.f[18].f64 + ctx.f[4].f64) as f32) as f64);
	// 82F4F280: D0C30030  stfs f6, 0x30(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82F4F284: EC850DFA  fmadds f4, f5, f23, f1
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[23].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F288: EC510032  fmuls f2, f17, f0
	ctx.f[2].f64 = (((ctx.f[17].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F28C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4F290: ECACFCFA  fmadds f5, f12, f19, f31
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[19].f64 + ctx.f[31].f64) as f32) as f64);
	// 82F4F294: C181FF48  lfs f12, -0xb8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F298: EC310332  fmuls f1, f17, f12
	ctx.f[1].f64 = (((ctx.f[17].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4F29C: D0A3001C  stfs f5, 0x1c(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82F4F2A0: EC84ECFA  fmadds f4, f4, f19, f29
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[19].f64 + ctx.f[29].f64) as f32) as f64);
	// 82F4F2A4: D0830028  stfs f4, 0x28(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4F2A8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82F4F2AC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82F4F2B0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82F4F2B4: EC50133A  fmadds f2, f16, f12, f2
	ctx.f[2].f64 = (((ctx.f[16].f64 * ctx.f[12].f64 + ctx.f[2].f64) as f32) as f64);
	// 82F4F2B8: EC30083A  fmadds f1, f16, f0, f1
	ctx.f[1].f64 = (((ctx.f[16].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F2BC: EC55137A  fmadds f2, f21, f13, f2
	ctx.f[2].f64 = (((ctx.f[21].f64 * ctx.f[13].f64 + ctx.f[2].f64) as f32) as f64);
	// 82F4F2C0: EC3C0B7A  fmadds f1, f28, f13, f1
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F2C4: EC42E02A  fadds f2, f2, f28
	ctx.f[2].f64 = ((ctx.f[2].f64 + ctx.f[28].f64) as f32) as f64;
	// 82F4F2C8: EC21A82A  fadds f1, f1, f21
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[21].f64) as f32) as f64;
	// 82F4F2CC: EC420672  fmuls f2, f2, f25
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[25].f64) as f32) as f64);
	// 82F4F2D0: EC2110FA  fmadds f1, f1, f3, f2
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[3].f64 + ctx.f[2].f64) as f32) as f64);
	// 82F4F2D4: EC61DCBA  fmadds f3, f1, f18, f27
	ctx.f[3].f64 = (((ctx.f[1].f64 * ctx.f[18].f64 + ctx.f[27].f64) as f32) as f64);
	// 82F4F2D8: D063002C  stfs f3, 0x2c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F4F2DC: 4082FE68  bne 0x82f4f144
	if !ctx.cr[0].eq {
	pc = 0x82F4F144; continue 'dispatch;
	}
	// 82F4F2E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4F2E4: C041FF4C  lfs f2, -0xb4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-180 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F2E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F4F2EC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4F2F0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F4F2F4: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F4F2F8: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F2FC: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F4F300: C1AA952C  lfs f13, -0x6ad4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27348 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F304: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 82F4F308: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F4F30C: C1899520  lfs f12, -0x6ae0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27360 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F310: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4F314: EFA20032  fmuls f29, f2, f0
	ctx.f[29].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82F4F31C: C0489F78  lfs f2, -0x6088(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24712 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F320: C027EE84  lfs f1, -0x117c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-4476 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4F324: EFCB0372  fmuls f30, f11, f13
	ctx.f[30].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4F328: ED290332  fmuls f9, f9, f12
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4F32C: C3E614CC  lfs f31, 0x14cc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(5324 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F4F330: C005296C  lfs f0, 0x296c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(10604 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F334: ECE700B2  fmuls f7, f7, f2
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F338: C1A414C8  lfs f13, 0x14c8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(5320 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F33C: EC4A0072  fmuls f2, f10, f1
	ctx.f[2].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F340: C18BC0AC  lfs f12, -0x3f54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F344: EC2807F2  fmuls f1, f8, f31
	ctx.f[1].f64 = (((ctx.f[8].f64 * ctx.f[31].f64) as f32) as f64);
	// 82F4F348: C16A54D0  lfs f11, 0x54d0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(21712 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F34C: EC060032  fmuls f0, f6, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F350: EDA50372  fmuls f13, f5, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4F354: D3A3000C  stfs f29, 0xc(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F4F358: ED840332  fmuls f12, f4, f12
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 82F4F35C: D3C30010  stfs f30, 0x10(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4F360: ED6302F2  fmuls f11, f3, f11
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 82F4F364: D1230018  stfs f9, 0x18(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4F368: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4F36C: D0430014  stfs f2, 0x14(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4F370: D0230020  stfs f1, 0x20(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F374: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82F4F378: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82F4F37C: D1830028  stfs f12, 0x28(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4F380: D163002C  stfs f11, 0x2c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F4F384: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82F4F388: 48259705  bl 0x831a8a8c
	ctx.lr = 0x82F4F38C;
	sub_831A8A8C(ctx, base);
	// 82F4F38C: 48258E30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4F390 size=576
    let mut pc: u32 = 0x82F4F390;
    'dispatch: loop {
        match pc {
            0x82F4F390 => {
    //   block [0x82F4F390..0x82F4F5D0)
	// 82F4F390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4F394: 48258DD9  bl 0x831a816c
	ctx.lr = 0x82F4F398;
	sub_831A8130(ctx, base);
	// 82F4F398: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82F4F39C: 482596BD  bl 0x831a8a58
	ctx.lr = 0x82F4F3A0;
	sub_831A8A40(ctx, base);
	// 82F4F3A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4F3A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4F3A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F4F3AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F4F3B0: 4BFFFD11  bl 0x82f4f0c0
	ctx.lr = 0x82F4F3B4;
	sub_82F4F0C0(ctx, base);
	// 82F4F3B4: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4F3B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4F3BC: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4F3C0: C15E0010  lfs f10, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4F3C4: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F4F3C8: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F3CC: C17F0000  lfs f11, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F3D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F4F3D4: C0DF0004  lfs f6, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4F3D8: ED0C02F2  fmuls f8, f12, f11
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82F4F3DC: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F3E0: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4F3E4: C0BD0008  lfs f5, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F4F3E8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4F3EC: 7C86FC2E  lfsx f4, r6, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4F3F0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4F3F4: EC6D2024  fdivs f3, f13, f4
	ctx.f[3].f64 = ((ctx.f[13].f64 / ctx.f[4].f64) as f32) as f64;
	// 82F4F3F8: C05F0008  lfs f2, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F3FC: C0FD0004  lfs f7, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4F400: C13E0014  lfs f9, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4F404: C00A9524  lfs f0, -0x6adc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F408: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4F40C: C03E000C  lfs f1, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4F410: FDA04050  fneg f13, f8
	ctx.f[13].u64 = ctx.f[8].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F414: EF8A00F2  fmuls f28, f10, f3
	ctx.f[28].f64 = (((ctx.f[10].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F418: D39E0034  stfs f28, 0x34(r30)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82F4F41C: EF8900F2  fmuls f28, f9, f3
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F420: D39E0038  stfs f28, 0x38(r30)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82F4F424: 7F8BFC2E  lfsx f28, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82F4F428: EF6300F2  fmuls f27, f3, f3
	ctx.f[27].f64 = (((ctx.f[3].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F42C: 7F4AFC2E  lfsx f26, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82F4F430: EF4906B2  fmuls f26, f9, f26
	ctx.f[26].f64 = (((ctx.f[9].f64 * ctx.f[26].f64) as f32) as f64);
	// 82F4F434: ECE769BC  fnmsubs f7, f7, f6, f13
	ctx.f[7].f64 = -(((ctx.f[7].f64 * ctx.f[6].f64 - ctx.f[13].f64) as f32) as f64);
	// 82F4F438: C17E0018  lfs f11, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F43C: ECDCD2BA  fmadds f6, f28, f10, f26
	ctx.f[6].f64 = (((ctx.f[28].f64 * ctx.f[10].f64 + ctx.f[26].f64) as f32) as f64);
	// 82F4F440: C19E0020  lfs f12, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F444: EDAB00F2  fmuls f13, f11, f3
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F448: D1BE0040  stfs f13, 0x40(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82F4F44C: EDAC00F2  fmuls f13, f12, f3
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F450: D1BE0044  stfs f13, 0x44(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82F4F454: C09E001C  lfs f4, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4F458: EF9B00F2  fmuls f28, f27, f3
	ctx.f[28].f64 = (((ctx.f[27].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F45C: C3FE0024  lfs f31, 0x24(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F4F460: EDBF00F2  fmuls f13, f31, f3
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F464: C11E0030  lfs f8, 0x30(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F468: EF4800F2  fmuls f26, f8, f3
	ctx.f[26].f64 = (((ctx.f[8].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F46C: C3DE002C  lfs f30, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82F4F470: C3BE0028  lfs f29, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82F4F474: ECE538BC  fnmsubs f7, f5, f2, f7
	ctx.f[7].f64 = -(((ctx.f[5].f64 * ctx.f[2].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4F478: ECA101F2  fmuls f5, f1, f7
	ctx.f[5].f64 = (((ctx.f[1].f64 * ctx.f[7].f64) as f32) as f64);
	// 82F4F47C: EC46282A  fadds f2, f6, f5
	ctx.f[2].f64 = ((ctx.f[6].f64 + ctx.f[5].f64) as f32) as f64;
	// 82F4F480: EC2206F2  fmuls f1, f2, f27
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[27].f64) as f32) as f64);
	// 82F4F484: FCC00850  fneg f6, f1
	ctx.f[6].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F488: D0DE003C  stfs f6, 0x3c(r30)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82F4F48C: 7C4AFC2E  lfsx f2, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F490: 7C2BFC2E  lfsx f1, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4F494: ECC102B2  fmuls f6, f1, f10
	ctx.f[6].f64 = (((ctx.f[1].f64 * ctx.f[10].f64) as f32) as f64);
	// 82F4F498: EF240072  fmuls f25, f4, f1
	ctx.f[25].f64 = (((ctx.f[4].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F49C: D1BE004C  stfs f13, 0x4c(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82F4F4A0: EDA2327A  fmadds f13, f2, f9, f6
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[9].f64 + ctx.f[6].f64) as f32) as f64);
	// 82F4F4A4: D35E0050  stfs f26, 0x50(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F4F4A8: ECCC00B2  fmuls f6, f12, f2
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F4AC: EF4B0072  fmuls f26, f11, f1
	ctx.f[26].f64 = (((ctx.f[11].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F4B0: EF3900B2  fmuls f25, f25, f2
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F4B4: EDAD283A  fmadds f13, f13, f0, f5
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F4B8: EF390032  fmuls f25, f25, f0
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F4BC: EDADC9FA  fmadds f13, f13, f7, f25
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[7].f64 + ctx.f[25].f64) as f32) as f64);
	// 82F4F4C0: ECC668BA  fmadds f6, f6, f2, f13
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[2].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F4F4C4: EC5A307A  fmadds f2, f26, f1, f6
	ctx.f[2].f64 = (((ctx.f[26].f64 * ctx.f[1].f64 + ctx.f[6].f64) as f32) as f64);
	// 82F4F4C8: EC220732  fmuls f1, f2, f28
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[28].f64) as f32) as f64);
	// 82F4F4CC: D03E0048  stfs f1, 0x48(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82F4F4D0: 7CCAFC2E  lfsx f6, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4F4D4: 7C4BFC2E  lfsx f2, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F4D8: EC2400B2  fmuls f1, f4, f2
	ctx.f[1].f64 = (((ctx.f[4].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F4DC: EDAC01B2  fmuls f13, f12, f6
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F4E0: EF4202B2  fmuls f26, f2, f10
	ctx.f[26].f64 = (((ctx.f[2].f64 * ctx.f[10].f64) as f32) as f64);
	// 82F4F4E4: EF3E00B2  fmuls f25, f30, f2
	ctx.f[25].f64 = (((ctx.f[30].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F4E8: EC2101B2  fmuls f1, f1, f6
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F4EC: EF0D01B2  fmuls f24, f13, f6
	ctx.f[24].f64 = (((ctx.f[13].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F4F0: C1A9A1C4  lfs f13, -0x5e3c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F4F4: ED26D27A  fmadds f9, f6, f9, f26
	ctx.f[9].f64 = (((ctx.f[6].f64 * ctx.f[9].f64 + ctx.f[26].f64) as f32) as f64);
	// 82F4F4F8: EE9C00F2  fmuls f20, f28, f3
	ctx.f[20].f64 = (((ctx.f[28].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F4FC: EC7D00F2  fmuls f3, f29, f3
	ctx.f[3].f64 = (((ctx.f[29].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F500: D07E0058  stfs f3, 0x58(r30)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82F4F504: EF4B00B2  fmuls f26, f11, f2
	ctx.f[26].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F508: EF3901B2  fmuls f25, f25, f6
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F50C: EEFD01B2  fmuls f23, f29, f6
	ctx.f[23].f64 = (((ctx.f[29].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F510: EEC801B2  fmuls f22, f8, f6
	ctx.f[22].f64 = (((ctx.f[8].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F514: EEBF00B2  fmuls f21, f31, f2
	ctx.f[21].f64 = (((ctx.f[31].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F518: ED4A01F2  fmuls f10, f10, f7
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[7].f64) as f32) as f64);
	// 82F4F51C: EC61C03A  fmadds f3, f1, f0, f24
	ctx.f[3].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 + ctx.f[24].f64) as f32) as f64);
	// 82F4F520: EC292B7A  fmadds f1, f9, f13, f5
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F524: ED3901B2  fmuls f9, f25, f6
	ctx.f[9].f64 = (((ctx.f[25].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F528: ECB700B2  fmuls f5, f23, f2
	ctx.f[5].f64 = (((ctx.f[23].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F52C: EF3601B2  fmuls f25, f22, f6
	ctx.f[25].f64 = (((ctx.f[22].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F530: EF1500B2  fmuls f24, f21, f2
	ctx.f[24].f64 = (((ctx.f[21].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F534: EC7A18BA  fmadds f3, f26, f2, f3
	ctx.f[3].f64 = (((ctx.f[26].f64 * ctx.f[2].f64 + ctx.f[3].f64) as f32) as f64);
	// 82F4F538: EC2101F2  fmuls f1, f1, f7
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[7].f64) as f32) as f64);
	// 82F4F53C: ED290372  fmuls f9, f9, f13
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4F540: ECA500B2  fmuls f5, f5, f2
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F544: EC630B7A  fmadds f3, f3, f13, f1
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F548: EC2349FA  fmadds f1, f3, f7, f9
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64);
	// 82F4F54C: EDA50B7A  fmadds f13, f5, f13, f1
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F550: ED3969BA  fmadds f9, f25, f6, f13
	ctx.f[9].f64 = (((ctx.f[25].f64 * ctx.f[6].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F4F554: ECD848BA  fmadds f6, f24, f2, f9
	ctx.f[6].f64 = (((ctx.f[24].f64 * ctx.f[2].f64 + ctx.f[9].f64) as f32) as f64);
	// 82F4F558: ECA60532  fmuls f5, f6, f20
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[20].f64) as f32) as f64);
	// 82F4F55C: FC602850  fneg f3, f5
	ctx.f[3].u64 = ctx.f[5].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F560: D07E0054  stfs f3, 0x54(r30)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F4F564: 7C4BFC2E  lfsx f2, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F568: 7C2AFC2E  lfsx f1, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4F56C: EDA80072  fmuls f13, f8, f1
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4F570: ED226FBA  fmadds f9, f2, f30, f13
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82F4F574: ED0C49FA  fmadds f8, f12, f7, f9
	ctx.f[8].f64 = (((ctx.f[12].f64 * ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64);
	// 82F4F578: ECC806F2  fmuls f6, f8, f27
	ctx.f[6].f64 = (((ctx.f[8].f64 * ctx.f[27].f64) as f32) as f64);
	// 82F4F57C: FCA03050  fneg f5, f6
	ctx.f[5].u64 = ctx.f[6].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F580: D0BE005C  stfs f5, 0x5c(r30)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F4F584: 7C6AFC2E  lfsx f3, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4F588: 7C4BFC2E  lfsx f2, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F58C: EC2B00B2  fmuls f1, f11, f2
	ctx.f[1].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F590: EDBD00B2  fmuls f13, f29, f2
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F594: ED8408FA  fmadds f12, f4, f3, f1
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[1].f64) as f32) as f64);
	// 82F4F598: ED7E00F2  fmuls f11, f30, f3
	ctx.f[11].f64 = (((ctx.f[30].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F59C: ED3F00B2  fmuls f9, f31, f2
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F5A0: ED0D00F2  fmuls f8, f13, f3
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[3].f64) as f32) as f64);
	// 82F4F5A4: ECCC503A  fmadds f6, f12, f0, f10
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 82F4F5A8: ECA80032  fmuls f5, f8, f0
	ctx.f[5].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F5AC: EC8629FA  fmadds f4, f6, f7, f5
	ctx.f[4].f64 = (((ctx.f[6].f64 * ctx.f[7].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F5B0: EC6B20FA  fmadds f3, f11, f3, f4
	ctx.f[3].f64 = (((ctx.f[11].f64 * ctx.f[3].f64 + ctx.f[4].f64) as f32) as f64);
	// 82F4F5B4: EC4918BA  fmadds f2, f9, f2, f3
	ctx.f[2].f64 = (((ctx.f[9].f64 * ctx.f[2].f64 + ctx.f[3].f64) as f32) as f64);
	// 82F4F5B8: EC220732  fmuls f1, f2, f28
	ctx.f[1].f64 = (((ctx.f[2].f64 * ctx.f[28].f64) as f32) as f64);
	// 82F4F5BC: D03E0060  stfs f1, 0x60(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82F4F5C0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F4F5C4: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82F4F5C8: 482594DD  bl 0x831a8aa4
	ctx.lr = 0x82F4F5CC;
	sub_831A8A8C(ctx, base);
	// 82F4F5CC: 48258BF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4F5D0 size=340
    let mut pc: u32 = 0x82F4F5D0;
    'dispatch: loop {
        match pc {
            0x82F4F5D0 => {
    //   block [0x82F4F5D0..0x82F4F724)
	// 82F4F5D0: C0030068  lfs f0, 0x68(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F5D4: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F5D8: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82F4F5DC: D1860000  stfs f12, 0(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F4F5E0: C163006C  lfs f11, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F5E4: C1430064  lfs f10, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4F5E8: ED2B5024  fdivs f9, f11, f10
	ctx.f[9].f64 = ((ctx.f[11].f64 / ctx.f[10].f64) as f32) as f64;
	// 82F4F5EC: D1260004  stfs f9, 4(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4F5F0: C1030070  lfs f8, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F5F4: C0E30064  lfs f7, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4F5F8: ECC83824  fdivs f6, f8, f7
	ctx.f[6].f64 = ((ctx.f[8].f64 / ctx.f[7].f64) as f32) as f64;
	// 82F4F5FC: D0C60008  stfs f6, 8(r6)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4F600: C0A30078  lfs f5, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F4F604: C083007C  lfs f4, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F4F608: EC65202A  fadds f3, f5, f4
	ctx.f[3].f64 = ((ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64;
	// 82F4F60C: EC0300B2  fmuls f0, f3, f2
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F610: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F4F614: C1A30074  lfs f13, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F618: C183007C  lfs f12, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4F61C: ED6D602A  fadds f11, f13, f12
	ctx.f[11].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 82F4F620: ED4B00B2  fmuls f10, f11, f2
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F624: D1470014  stfs f10, 0x14(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4F628: C1230074  lfs f9, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4F62C: C1030078  lfs f8, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F630: ECE9402A  fadds f7, f9, f8
	ctx.f[7].f64 = ((ctx.f[9].f64 + ctx.f[8].f64) as f32) as f64;
	// 82F4F634: ECC700B2  fmuls f6, f7, f2
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F638: D0C70028  stfs f6, 0x28(r7)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4F63C: C0A30080  lfs f5, 0x80(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F4F640: EC8500B2  fmuls f4, f5, f2
	ctx.f[4].f64 = (((ctx.f[5].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F644: FC602050  fneg f3, f4
	ctx.f[3].u64 = ctx.f[4].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F648: D0670004  stfs f3, 4(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4F64C: D0670010  stfs f3, 0x10(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4F650: C0030084  lfs f0, 0x84(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F654: EDA000B2  fmuls f13, f0, f2
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F658: FD806850  fneg f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F65C: D1870018  stfs f12, 0x18(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4F660: D1870024  stfs f12, 0x24(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4F664: C1630088  lfs f11, 0x88(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F668: ED4B00B2  fmuls f10, f11, f2
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 82F4F66C: FD205050  fneg f9, f10
	ctx.f[9].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4F670: D1270020  stfs f9, 0x20(r7)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F674: D1270008  stfs f9, 8(r7)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4F678: C1060004  lfs f8, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F67C: C0E70000  lfs f7, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4F680: C0C60008  lfs f6, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4F684: ECA601B2  fmuls f5, f6, f6
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[6].f64) as f32) as f64);
	// 82F4F688: EC882A3A  fmadds f4, f8, f8, f5
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 82F4F68C: EC64387C  fnmsubs f3, f4, f1, f7
	ctx.f[3].f64 = -(((ctx.f[4].f64 * ctx.f[1].f64 - ctx.f[7].f64) as f32) as f64);
	// 82F4F690: D0670000  stfs f3, 0(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F4F694: C0460000  lfs f2, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F698: C0070014  lfs f0, 0x14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F69C: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4F6A0: ED8D0372  fmuls f12, f13, f13
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F4F6A4: ED6260BA  fmadds f11, f2, f2, f12
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[2].f64 + ctx.f[12].f64) as f32) as f64);
	// 82F4F6A8: ED4B007C  fnmsubs f10, f11, f1, f0
	ctx.f[10].f64 = -(((ctx.f[11].f64 * ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4F6AC: D1470014  stfs f10, 0x14(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F4F6B0: C1260000  lfs f9, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4F6B4: C1070028  lfs f8, 0x28(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F4F6B8: C0E60004  lfs f7, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F4F6BC: ECC701F2  fmuls f6, f7, f7
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[7].f64) as f32) as f64);
	// 82F4F6C0: ECA9327A  fmadds f5, f9, f9, f6
	ctx.f[5].f64 = (((ctx.f[9].f64 * ctx.f[9].f64 + ctx.f[6].f64) as f32) as f64);
	// 82F4F6C4: EC85407C  fnmsubs f4, f5, f1, f8
	ctx.f[4].f64 = -(((ctx.f[5].f64 * ctx.f[1].f64 - ctx.f[8].f64) as f32) as f64);
	// 82F4F6C8: D0870028  stfs f4, 0x28(r7)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F4F6CC: C0670004  lfs f3, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4F6D0: C0460000  lfs f2, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F4F6D4: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F6D8: EDA20032  fmuls f13, f2, f0
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F4F6DC: ED8D187A  fmadds f12, f13, f1, f3
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64);
	// 82F4F6E0: D1870004  stfs f12, 4(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4F6E4: D1870010  stfs f12, 0x10(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F4F6E8: C1670018  lfs f11, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4F6EC: C1460008  lfs f10, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4F6F0: C1260004  lfs f9, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F4F6F4: ED0A0272  fmuls f8, f10, f9
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[9].f64) as f32) as f64);
	// 82F4F6F8: ECE8587A  fmadds f7, f8, f1, f11
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[1].f64 + ctx.f[11].f64) as f32) as f64);
	// 82F4F6FC: D0E70018  stfs f7, 0x18(r7)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4F700: D0E70024  stfs f7, 0x24(r7)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F4F704: C0C60008  lfs f6, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F4F708: C0A60000  lfs f5, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F4F70C: EC860172  fmuls f4, f6, f5
	ctx.f[4].f64 = (((ctx.f[6].f64 * ctx.f[5].f64) as f32) as f64);
	// 82F4F710: C0670020  lfs f3, 0x20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F4F714: EC44187A  fmadds f2, f4, f1, f3
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[1].f64 + ctx.f[3].f64) as f32) as f64);
	// 82F4F718: D0470020  stfs f2, 0x20(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82F4F71C: D0470008  stfs f2, 8(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F4F720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4F728 size=112
    let mut pc: u32 = 0x82F4F728;
    'dispatch: loop {
        match pc {
            0x82F4F728 => {
    //   block [0x82F4F728..0x82F4F798)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4F798 size=68
    let mut pc: u32 = 0x82F4F798;
    'dispatch: loop {
        match pc {
            0x82F4F798 => {
    //   block [0x82F4F798..0x82F4F7DC)
	// 82F4F798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4F79C: 114D004A  vsubfp v10, v13, v0
	ctx.fpscr.enable_flush_mode_unconditional();
	for i in 0..4 {
		ctx.v[10].f32[i] = ctx.v[13].f32[i] - ctx.v[0].f32[i];
	}
	// 82F4F7A0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82F4F7A4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82F4F7A8: 3909FFA0  addi r8, r9, -0x60
	ctx.r[8].s64 = ctx.r[9].s64 + -96;
	// 82F4F7AC: C00B9450  lfs f0, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4F7B0: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4F7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4F7E0 size=772
    let mut pc: u32 = 0x82F4F7E0;
    'dispatch: loop {
        match pc {
            0x82F4F7E0 => {
    //   block [0x82F4F7E0..0x82F4FAE4)
	// 82F4F7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4F7E4: 48258971  bl 0x831a8154
	ctx.lr = 0x82F4F7E8;
	sub_831A8130(ctx, base);
	// 82F4F7E8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82F4F7EC: 48259281  bl 0x831a8a6c
	ctx.lr = 0x82F4F7F0;
	sub_831A8A40(ctx, base);
	// 82F4F7F0: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4FAE8 size=76
    let mut pc: u32 = 0x82F4FAE8;
    'dispatch: loop {
        match pc {
            0x82F4FAE8 => {
    //   block [0x82F4FAE8..0x82F4FB34)
	// 82F4FAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4FAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4FAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4FAF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4FAF8: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4FAFC: EDA10024  fdivs f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 82F4FB00: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F4FB04: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82F4FB08: D0230004  stfs f1, 4(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4FB0C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F4FB10: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4FB38 size=84
    let mut pc: u32 = 0x82F4FB38;
    'dispatch: loop {
        match pc {
            0x82F4FB38 => {
    //   block [0x82F4FB38..0x82F4FB8C)
	// 82F4FB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4FB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4FB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4FB44: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4FB48: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4FB4C: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4FB50: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4FB54: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82F4FB58: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F4FB5C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F4FB60: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82F4FB64: ED6D6024  fdivs f11, f13, f12
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[12].f64) as f32) as f64;
	// 82F4FB68: D1610050  stfs f11, 0x50(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4FB90 size=424
    let mut pc: u32 = 0x82F4FB90;
    'dispatch: loop {
        match pc {
            0x82F4FB90 => {
    //   block [0x82F4FB90..0x82F4FD38)
	// 82F4FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4FB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4FB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4FB9C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82F4FBA0: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82F4FBA4: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82F4FBA8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4FBAC: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82F4FBB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4FBB4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F4FBB8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4FBBC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82F4FBC0: 4199000C  bgt cr6, 0x82f4fbcc
	if ctx.cr[6].gt {
	pc = 0x82F4FBCC; continue 'dispatch;
	}
	// 82F4FBC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F4FBC8: 48000150  b 0x82f4fd18
	pc = 0x82F4FD18; continue 'dispatch;
	// 82F4FBCC: FF011800  fcmpu cr6, f1, f3
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[3].f64);
	// 82F4FBD0: 4099FFF4  ble cr6, 0x82f4fbc4
	if !ctx.cr[6].gt {
	pc = 0x82F4FBC4; continue 'dispatch;
	}
	// 82F4FBD4: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 82F4FBD8: 4099FFEC  ble cr6, 0x82f4fbc4
	if !ctx.cr[6].gt {
	pc = 0x82F4FBC4; continue 'dispatch;
	}
	// 82F4FBDC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F4FBE0: ED011828  fsubs f8, f1, f3
	ctx.f[8].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 82F4FBE4: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F4FBE8: ECE10072  fmuls f7, f1, f1
	ctx.f[7].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82F4FBEC: 392100F0  addi r9, r1, 0xf0
	ctx.r[9].s64 = ctx.r[1].s64 + 240;
	// 82F4FBF0: 390100F0  addi r8, r1, 0xf0
	ctx.r[8].s64 = ctx.r[1].s64 + 240;
	// 82F4FBF4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4FD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4FD38 size=516
    let mut pc: u32 = 0x82F4FD38;
    'dispatch: loop {
        match pc {
            0x82F4FD38 => {
    //   block [0x82F4FD38..0x82F4FF3C)
	// 82F4FD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4FD3C: 48258431  bl 0x831a816c
	ctx.lr = 0x82F4FD40;
	sub_831A8130(ctx, base);
	// 82F4FD40: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82F4FD44: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F4FD48: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F4FD4C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4FD50: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F4FD54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4FD58: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F4FD5C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4FD60: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82F4FD64: 409901C0  ble cr6, 0x82f4ff24
	if !ctx.cr[6].gt {
	pc = 0x82F4FF24; continue 'dispatch;
	}
	// 82F4FD68: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4FD6C: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82F4FD70: 409901B4  ble cr6, 0x82f4ff24
	if !ctx.cr[6].gt {
	pc = 0x82F4FF24; continue 'dispatch;
	}
	// 82F4FD74: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4FD78: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82F4FD7C: 409901A8  ble cr6, 0x82f4ff24
	if !ctx.cr[6].gt {
	pc = 0x82F4FF24; continue 'dispatch;
	}
	// 82F4FD80: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4FD84: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82F4FD88: 4099019C  ble cr6, 0x82f4ff24
	if !ctx.cr[6].gt {
	pc = 0x82F4FF24; continue 'dispatch;
	}
	// 82F4FD8C: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82F4FD90: 40990194  ble cr6, 0x82f4ff24
	if !ctx.cr[6].gt {
	pc = 0x82F4FF24; continue 'dispatch;
	}
	// 82F4FD94: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4FD98: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4FD9C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4FDA0: C1630004  lfs f11, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4FDA4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4FDA8: C1430008  lfs f10, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4FDAC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F4FDB0: ED8D1028  fsubs f12, f13, f2
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[2].f64) as f32) as f64);
	// 82F4FDB4: ED2B1028  fsubs f9, f11, f2
	ctx.f[9].f64 = (((ctx.f[11].f64 - ctx.f[2].f64) as f32) as f64);
	// 82F4FDB8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F4FDBC: ED0A1028  fsubs f8, f10, f2
	ctx.f[8].f64 = (((ctx.f[10].f64 - ctx.f[2].f64) as f32) as f64);
	// 82F4FDC0: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F4FDC4: D1210054  stfs f9, 0x54(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F4FDC8: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F4FDCC: D1010058  stfs f8, 0x58(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82F4FDD0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4FF40 size=608
    let mut pc: u32 = 0x82F4FF40;
    'dispatch: loop {
        match pc {
            0x82F4FF40 => {
    //   block [0x82F4FF40..0x82F501A0)
	// 82F4FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4FF44: 48258219  bl 0x831a815c
	ctx.lr = 0x82F4FF48;
	sub_831A8130(ctx, base);
	// 82F4FF48: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82F4FF4C: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82F4FF50: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82F4FF54: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F501A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F501A0 size=1124
    let mut pc: u32 = 0x82F501A0;
    'dispatch: loop {
        match pc {
            0x82F501A0 => {
    //   block [0x82F501A0..0x82F50604)
	// 82F501A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F501A4: 48257F8D  bl 0x831a8130
	ctx.lr = 0x82F501A8;
	sub_831A8130(ctx, base);
	// 82F501A8: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82F501AC: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82F501B0: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82F501B4: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F501B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F501BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F501C0: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82F501C4: 3B830088  addi r28, r3, 0x88
	ctx.r[28].s64 = ctx.r[3].s64 + 136;
	// 82F501C8: 3B630084  addi r27, r3, 0x84
	ctx.r[27].s64 = ctx.r[3].s64 + 132;
	// 82F501CC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F501D0: 3B430080  addi r26, r3, 0x80
	ctx.r[26].s64 = ctx.r[3].s64 + 128;
	// 82F501D4: C3AA08A4  lfs f29, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82F501D8: 3B23007C  addi r25, r3, 0x7c
	ctx.r[25].s64 = ctx.r[3].s64 + 124;
	// 82F501DC: D3A30088  stfs f29, 0x88(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82F501E0: 3B030078  addi r24, r3, 0x78
	ctx.r[24].s64 = ctx.r[3].s64 + 120;
	// 82F501E4: D3A30084  stfs f29, 0x84(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82F501E8: 3AE30074  addi r23, r3, 0x74
	ctx.r[23].s64 = ctx.r[3].s64 + 116;
	// 82F501EC: D3A30080  stfs f29, 0x80(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82F501F0: 3AC30070  addi r22, r3, 0x70
	ctx.r[22].s64 = ctx.r[3].s64 + 112;
	// 82F501F4: D3A3007C  stfs f29, 0x7c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82F501F8: 3AA3006C  addi r21, r3, 0x6c
	ctx.r[21].s64 = ctx.r[3].s64 + 108;
	// 82F501FC: D3A30078  stfs f29, 0x78(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82F50200: 3A830068  addi r20, r3, 0x68
	ctx.r[20].s64 = ctx.r[3].s64 + 104;
	// 82F50204: D3A30074  stfs f29, 0x74(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82F50208: 3BE30064  addi r31, r3, 0x64
	ctx.r[31].s64 = ctx.r[3].s64 + 100;
	// 82F5020C: D3A30070  stfs f29, 0x70(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82F50210: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F50214: D3A3006C  stfs f29, 0x6c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82F50218: D3A30068  stfs f29, 0x68(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82F5021C: D3A30064  stfs f29, 0x64(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F50220: 40990354  ble cr6, 0x82f50574
	if !ctx.cr[6].gt {
	pc = 0x82F50574; continue 'dispatch;
	}
	// 82F50224: 7D705B78  mr r16, r11
	ctx.r[16].u64 = ctx.r[11].u64;
	// 82F50228: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F5022C: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82F50230: 3D405555  lis r10, 0x5555
	ctx.r[10].s64 = 1431633920;
	// 82F50234: 3A3D000C  addi r17, r29, 0xc
	ctx.r[17].s64 = ctx.r[29].s64 + 12;
	// 82F50238: 7DF37B78  mr r19, r15
	ctx.r[19].u64 = ctx.r[15].u64;
	// 82F5023C: C3CB08A8  lfs f30, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82F50240: 615E5556  ori r30, r10, 0x5556
	ctx.r[30].u64 = ctx.r[10].u64 | 21846;
	// 82F50244: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F50608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F50608 size=564
    let mut pc: u32 = 0x82F50608;
    'dispatch: loop {
        match pc {
            0x82F50608 => {
    //   block [0x82F50608..0x82F5083C)
	// 82F50608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F50610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F50614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F50618: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82F5061C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F50620: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F50624: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F50628: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F5062C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F50630: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F50634: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82F50638: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82F5063C: 4199000C  bgt cr6, 0x82f50648
	if ctx.cr[6].gt {
	pc = 0x82F50648; continue 'dispatch;
	}
	// 82F50640: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F50644: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F50648: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F5064C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F50650: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 82F50654: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F50658: 4BFFF0D1  bl 0x82f4f728
	ctx.lr = 0x82F5065C;
	sub_82F4F728(ctx, base);
	// 82F5065C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F50660: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F50664: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F50668: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82F5066C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F50670: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F50840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F50840 size=496
    let mut pc: u32 = 0x82F50840;
    'dispatch: loop {
        match pc {
            0x82F50840 => {
    //   block [0x82F50840..0x82F50A30)
	// 82F50840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F50844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F50848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F5084C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F50850: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82F50854: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F50858: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5085C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F50860: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F50864: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F50868: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F5086C: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82F50870: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82F50874: 4199000C  bgt cr6, 0x82f50880
	if ctx.cr[6].gt {
	pc = 0x82F50880; continue 'dispatch;
	}
	// 82F50878: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F5087C: 48000194  b 0x82f50a10
	pc = 0x82F50A10; continue 'dispatch;
	// 82F50880: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F50884: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F50888: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F5088C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F50890: 4BFFEE99  bl 0x82f4f728
	ctx.lr = 0x82F50894;
	sub_82F4F728(ctx, base);
	// 82F50894: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82F50898: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F5089C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F508A0: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82F508A4: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82F508A8: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F50A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F50A30 size=1644
    let mut pc: u32 = 0x82F50A30;
    'dispatch: loop {
        match pc {
            0x82F50A30 => {
    //   block [0x82F50A30..0x82F5109C)
	// 82F50A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F50A34: 48257735  bl 0x831a8168
	ctx.lr = 0x82F50A38;
	sub_831A8130(ctx, base);
	// 82F50A38: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82F50A3C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F50A40: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F50A44: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F50A48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F50A4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F50A50: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82F50A54: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F50A58: 41990014  bgt cr6, 0x82f50a6c
	if ctx.cr[6].gt {
	pc = 0x82F50A6C; continue 'dispatch;
	}
	// 82F50A5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F50A60: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F50A64: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82F50A68: 48257750  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F50A6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F50A70: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82F50A74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F50A78: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F50A7C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82F50A80: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F50A84: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82F50A88: 40990018  ble cr6, 0x82f50aa0
	if !ctx.cr[6].gt {
	pc = 0x82F50AA0; continue 'dispatch;
	}
	// 82F50A8C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82F50A90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F50A94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F50A98: 4BF55D61  bl 0x82ea67f8
	ctx.lr = 0x82F50A9C;
	sub_82EA67F8(ctx, base);
	// 82F50A9C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50AA0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F50AA4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F50AA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F50AAC: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82F50AB0: 419800F0  blt cr6, 0x82f50ba0
	if ctx.cr[6].lt {
	pc = 0x82F50BA0; continue 'dispatch;
	}
	// 82F50AB4: 393CFFFC  addi r9, r28, -4
	ctx.r[9].s64 = ctx.r[28].s64 + -4;
	// 82F50AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F50ABC: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F50AC0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F50AC4: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F50AC8: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F50ACC: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82F50AD0: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82F50AD4: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50AD8: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F50ADC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50AE0: D1A70004  stfs f13, 4(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F50AE4: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50AE8: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F50AEC: 7CAB3A14  add r5, r11, r7
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50AF0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F50AF4: D1850008  stfs f12, 8(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F50AF8: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50AFC: 7C8B3A14  add r4, r11, r7
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50B00: C16A0000  lfs f11, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F50B04: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F50B08: D1640010  stfs f11, 0x10(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F50B0C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B10: C14A0004  lfs f10, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F50B14: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50B18: D1430014  stfs f10, 0x14(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F50B1C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B20: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F50B24: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50B28: D1270018  stfs f9, 0x18(r7)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F50B2C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F50B30: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B34: 7CA83A14  add r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F50B38: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F50B3C: D105FFF0  stfs f8, -0x10(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82F50B40: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B44: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F50B48: 7C8B3A14  add r4, r11, r7
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50B4C: D0E40024  stfs f7, 0x24(r4)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F50B50: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B54: C0CA0008  lfs f6, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F50B58: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F50B5C: D0C30028  stfs f6, 0x28(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F50B60: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F50B64: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B68: C0AA0000  lfs f5, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F50B6C: 7CA83D2E  stfsx f5, r8, r7
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F50B70: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B74: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F50B78: 7CAB4214  add r5, r11, r8
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F50B7C: D0850034  stfs f4, 0x34(r5)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82F50B80: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B84: C06A0008  lfs f3, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F50B88: 7C8B4214  add r4, r11, r8
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F50B8C: D0640038  stfs f3, 0x38(r4)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82F50B90: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50B94: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F50B98: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82F50B9C: 4082FF2C  bne 0x82f50ac8
	if !ctx.cr[0].eq {
	pc = 0x82F50AC8; continue 'dispatch;
	}
	// 82F50BA0: 7F06E000  cmpw cr6, r6, r28
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82F50BA4: 40980048  bge cr6, 0x82f50bec
	if !ctx.cr[6].lt {
	pc = 0x82F50BEC; continue 'dispatch;
	}
	// 82F50BA8: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F50BAC: 7D26E050  subf r9, r6, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[6].s64;
	// 82F50BB0: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F50BB4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F50BB8: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82F50BBC: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50BC0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F50BC4: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F50BC8: D1A80004  stfs f13, 4(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F50BCC: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50BD0: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F50BD4: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F50BD8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82F50BDC: D1870008  stfs f12, 8(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F50BE0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82F50BE4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F50BE8: 4082FFC8  bne 0x82f50bb0
	if !ctx.cr[0].eq {
	pc = 0x82F50BB0; continue 'dispatch;
	}
	// 82F50BEC: 7F8B07B4  extsw r11, r28
	ctx.r[11].s64 = ctx.r[28].s32 as i64;
	// 82F50BF0: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F50BF4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82F50BF8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82F50BFC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82F50C00: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82F50C04: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82F50C08: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F510A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F510A0 size=660
    let mut pc: u32 = 0x82F510A0;
    'dispatch: loop {
        match pc {
            0x82F510A0 => {
    //   block [0x82F510A0..0x82F51334)
	// 82F510A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F510A4: 482570BD  bl 0x831a8160
	ctx.lr = 0x82F510A8;
	sub_831A8130(ctx, base);
	// 82F510A8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82F510AC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F510B0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F510B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F510B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F510BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F510C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F510C4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F510C8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F510CC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82F510D0: 41990014  bgt cr6, 0x82f510e4
	if ctx.cr[6].gt {
	pc = 0x82F510E4; continue 'dispatch;
	}
	// 82F510D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F510D8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F510DC: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82F510E0: 482570D0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82F510E4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F510E8: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82F510EC: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82F510F0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F510F4: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82F510F8: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82F510FC: 4099001C  ble cr6, 0x82f51118
	if !ctx.cr[6].gt {
	pc = 0x82F51118; continue 'dispatch;
	}
	// 82F51100: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F51104: 41980008  blt cr6, 0x82f5110c
	if ctx.cr[6].lt {
	pc = 0x82F5110C; continue 'dispatch;
	}
	// 82F51108: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F5110C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82F51110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F51114: 4BF556E5  bl 0x82ea67f8
	ctx.lr = 0x82F51118;
	sub_82EA67F8(ctx, base);
	// 82F51118: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82F5111C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82F51120: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F51124: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82F51128: 419800F0  blt cr6, 0x82f51218
	if ctx.cr[6].lt {
	pc = 0x82F51218; continue 'dispatch;
	}
	// 82F5112C: 393DFFFC  addi r9, r29, -4
	ctx.r[9].s64 = ctx.r[29].s64 + -4;
	// 82F51130: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F51134: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F51138: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F5113C: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F51140: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51144: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F51148: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82F5114C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F51150: 7C0B3D2E  stfsx f0, r11, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82F51154: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51158: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F5115C: 7CAB3A14  add r5, r11, r7
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F51160: D1A50004  stfs f13, 4(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F51164: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51168: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F5116C: 7C8B3A14  add r4, r11, r7
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F51170: D1840008  stfs f12, 8(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F51174: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51178: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F5117C: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F51180: C16A0000  lfs f11, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F51184: D1630010  stfs f11, 0x10(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F51188: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5118C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F51190: C14A0004  lfs f10, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F51194: D1470014  stfs f10, 0x14(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F51198: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5119C: 7CAB3A14  add r5, r11, r7
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F511A0: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82F511A4: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F511A8: D1250018  stfs f9, 0x18(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F511AC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F511B0: 7C883A14  add r4, r8, r7
	ctx.r[4].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82F511B4: C10A0000  lfs f8, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82F511B8: D104FFF0  stfs f8, -0x10(r4)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82F511BC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F511C0: 7C6B3A14  add r3, r11, r7
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F511C4: C0EA0004  lfs f7, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82F511C8: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82F511CC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F511D0: C0CA0008  lfs f6, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82F511D4: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F511D8: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F511DC: D0C70028  stfs f6, 0x28(r7)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82F511E0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F511E4: C0AA0000  lfs f5, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82F511E8: 7CA82D2E  stfsx f5, r8, r5
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 82F511EC: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F511F0: 7C8B4214  add r4, r11, r8
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F511F4: C08A0004  lfs f4, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82F511F8: D0840034  stfs f4, 0x34(r4)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82F511FC: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51200: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F51204: C06A0008  lfs f3, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82F51208: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F5120C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82F51210: D0630038  stfs f3, 0x38(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82F51214: 4082FF2C  bne 0x82f51140
	if !ctx.cr[0].eq {
	pc = 0x82F51140; continue 'dispatch;
	}
	// 82F51218: 7F06E800  cmpw cr6, r6, r29
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82F5121C: 40980048  bge cr6, 0x82f51264
	if !ctx.cr[6].lt {
	pc = 0x82F51264; continue 'dispatch;
	}
	// 82F51220: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F51224: 7D26E850  subf r9, r6, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 82F51228: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5122C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F51230: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F51234: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82F51238: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5123C: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F51240: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F51244: D1A70004  stfs f13, 4(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F51248: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5124C: 7CCB4214  add r6, r11, r8
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F51250: C18A0008  lfs f12, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F51254: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82F51258: D1860008  stfs f12, 8(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82F5125C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82F51260: 4082FFC8  bne 0x82f51228
	if !ctx.cr[0].eq {
	pc = 0x82F51228; continue 'dispatch;
	}
	// 82F51264: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51268: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F5126C: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82F51270: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F51274: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82F51278: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F5127C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82F51280: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82F51284: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82F51288: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F5128C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82F51290: 93610088  stw r27, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 82F51294: 9381008C  stw r28, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 82F51298: 93810090  stw r28, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 82F5129C: 93610094  stw r27, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 82F512A0: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82F512A4: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F512A8: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82F512AC: 4800E96D  bl 0x82f5fc18
	ctx.lr = 0x82F512B0;
	sub_82F5FC18(ctx, base);
	// 82F512B0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82F512B4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F512B8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F512BC: 4BFFF34D  bl 0x82f50608
	ctx.lr = 0x82F512C0;
	sub_82F50608(ctx, base);
	// 82F512C0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F512C4: D3FA0004  stfs f31, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F512C8: 55690000  rlwinm r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F512CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F512D0: 409A0020  bne cr6, 0x82f512f0
	if !ctx.cr[6].eq {
	pc = 0x82F512F0; continue 'dispatch;
	}
	// 82F512D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F512D8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F512DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F512E0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F512E4: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F512E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F512EC: 4BF4F4C5  bl 0x82ea07b0
	ctx.lr = 0x82F512F0;
	sub_82EA07B0(ctx, base);
	// 82F512F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F512F4: 4B3953AD  bl 0x822e66a0
	ctx.lr = 0x82F512F8;
	sub_822E66A0(ctx, base);
	// 82F512F8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F512FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F51300: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F51304: 409A0020  bne cr6, 0x82f51324
	if !ctx.cr[6].eq {
	pc = 0x82F51324; continue 'dispatch;
	}
	// 82F51308: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5130C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F51310: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F51314: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F51318: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F5131C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F51320: 4BF4F491  bl 0x82ea07b0
	ctx.lr = 0x82F51324;
	sub_82EA07B0(ctx, base);
	// 82F51324: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F51328: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F5132C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82F51330: 48256E80  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F51338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F51338 size=2668
    let mut pc: u32 = 0x82F51338;
    'dispatch: loop {
        match pc {
            0x82F51338 => {
    //   block [0x82F51338..0x82F51DA4)
	// 82F51338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5133C: 48256E1D  bl 0x831a8158
	ctx.lr = 0x82F51340;
	sub_831A8130(ctx, base);
	// 82F51340: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 82F51344: 48257721  bl 0x831a8a64
	ctx.lr = 0x82F51348;
	sub_831A8A40(ctx, base);
	// 82F51348: 9421FA90  stwu r1, -0x570(r1)
	ea = ctx.r[1].u32.wrapping_add(-1392 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5134C: FF001090  fmr f24, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[24].f64 = ctx.f[2].f64;
	// 82F51350: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F51354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F51358: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82F5135C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F51360: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F51364: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F51368: FF18F800  fcmpu cr6, f24, f31
	ctx.cr[6].compare_f64(ctx.f[24].f64, ctx.f[31].f64);
	// 82F5136C: 41990018  bgt cr6, 0x82f51384
	if ctx.cr[6].gt {
	pc = 0x82F51384; continue 'dispatch;
	}
	// 82F51370: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82F51374: 38210570  addi r1, r1, 0x570
	ctx.r[1].s64 = ctx.r[1].s64 + 1392;
	// 82F51378: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 82F5137C: 48257735  bl 0x831a8ab0
	ctx.lr = 0x82F51380;
	sub_831A8A8C(ctx, base);
	// 82F51380: 48256E28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82F51384: FF1DF800  fcmpu cr6, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 82F51388: 4099FFE8  ble cr6, 0x82f51370
	if !ctx.cr[6].gt {
	pc = 0x82F51370; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F51DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F51DA8 size=852
    let mut pc: u32 = 0x82F51DA8;
    'dispatch: loop {
        match pc {
            0x82F51DA8 => {
    //   block [0x82F51DA8..0x82F520FC)
	// 82F51DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F51DAC: 482563C1  bl 0x831a816c
	ctx.lr = 0x82F51DB0;
	sub_831A8130(ctx, base);
	// 82F51DB0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82F51DB4: 48256CC5  bl 0x831a8a78
	ctx.lr = 0x82F51DB8;
	sub_831A8A40(ctx, base);
	// 82F51DB8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F51DBC: FF801090  fmr f28, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[2].f64;
	// 82F51DC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F51DC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F51DC8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F51DCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F51DD0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F51DD4: C3AB08A4  lfs f29, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82F51DD8: FF1CE800  fcmpu cr6, f28, f29
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[29].f64);
	// 82F51DDC: 4099030C  ble cr6, 0x82f520e8
	if !ctx.cr[6].gt {
	pc = 0x82F520E8; continue 'dispatch;
	}
	// 82F51DE0: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 82F51DE4: 40990304  ble cr6, 0x82f520e8
	if !ctx.cr[6].gt {
	pc = 0x82F520E8; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F52100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F52100 size=628
    let mut pc: u32 = 0x82F52100;
    'dispatch: loop {
        match pc {
            0x82F52100 => {
    //   block [0x82F52100..0x82F52374)
	// 82F52100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F52104: 48256061  bl 0x831a8164
	ctx.lr = 0x82F52108;
	sub_831A8130(ctx, base);
	// 82F52108: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82F5210C: 4825696D  bl 0x831a8a78
	ctx.lr = 0x82F52110;
	sub_831A8A40(ctx, base);
	// 82F52110: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F52114: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F52118: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 82F5211C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F52120: 3901008C  addi r8, r1, 0x8c
	ctx.r[8].s64 = ctx.r[1].s64 + 140;
	// 82F52124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F52128: 61470010  ori r7, r10, 0x10
	ctx.r[7].u64 = ctx.r[10].u64 | 16;
	// 82F5212C: 91010080  stw r8, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 82F52130: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F52134: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F52138: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82F5213C: 378BFFFF  addic. r28, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F52140: 90E10088  stw r7, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[7].u32 ) };
	// 82F52144: 41800098  blt 0x82f521dc
	if ctx.cr[0].lt {
	pc = 0x82F521DC; continue 'dispatch;
	}
	// 82F52148: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82F5214C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F52150: 7FFE502E  lwzx r31, r30, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F52154: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F52158: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82F5215C: 409A000C  bne cr6, 0x82f52168
	if !ctx.cr[6].eq {
	pc = 0x82F52168; continue 'dispatch;
	}
	// 82F52160: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F52164: 48000010  b 0x82f52174
	pc = 0x82F52174; continue 'dispatch;
	// 82F52168: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F5216C: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82F52170: 409A0060  bne cr6, 0x82f521d0
	if !ctx.cr[6].eq {
	pc = 0x82F521D0; continue 'dispatch;
	}
	// 82F52174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F52178: 419A0058  beq cr6, 0x82f521d0
	if ctx.cr[6].eq {
	pc = 0x82F521D0; continue 'dispatch;
	}
	// 82F5217C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F52180: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F52184: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F52188: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F5218C: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F52190: 7D1E512E  stwx r8, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82F52194: 80E10084  lwz r7, 0x84(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F52198: 80C10088  lwz r6, 0x88(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F5219C: 54C500BE  clrlwi r5, r6, 2
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x3FFFFFFFu64;
	// 82F521A0: 7F072800  cmpw cr6, r7, r5
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82F521A4: 409A0010  bne cr6, 0x82f521b4
	if !ctx.cr[6].eq {
	pc = 0x82F521B4; continue 'dispatch;
	}
	// 82F521A8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F521AC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F521B0: 4BF546D1  bl 0x82ea6880
	ctx.lr = 0x82F521B4;
	sub_82EA6880(ctx, base);
	// 82F521B4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F521B8: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F521BC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F521C0: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82F521C4: 81010084  lwz r8, 0x84(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82F521C8: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 82F521CC: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82F521D0: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82F521D4: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82F521D8: 4080FF74  bge 0x82f5214c
	if !ctx.cr[0].lt {
	pc = 0x82F5214C; continue 'dispatch;
	}
	// 82F521DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F521E0: 37C9FFFF  addic. r30, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F521E4: C3AB08A4  lfs f29, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82F521E8: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 82F521EC: FFC0E890  fmr f30, f29
	ctx.f[30].f64 = ctx.f[29].f64;
	// 82F521F0: 41800048  blt 0x82f52238
	if ctx.cr[0].lt {
	pc = 0x82F52238; continue 'dispatch;
	}
	// 82F521F4: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82F521F8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F521FC: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F52200: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F52204: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82F52208: 409A0008  bne cr6, 0x82f52210
	if !ctx.cr[6].eq {
	pc = 0x82F52210; continue 'dispatch;
	}
	// 82F5220C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F52210: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82F52214: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 82F52218: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F5221C: 4BFFFEE5  bl 0x82f52100
	ctx.lr = 0x82F52220;
	sub_82F52100(ctx, base);
	// 82F52220: EC01F028  fsubs f0, f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F52224: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F52228: EFE1F82A  fadds f31, f1, f31
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 82F5222C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82F52230: FFC0F06E  fsel f30, f0, f1, f30
	ctx.f[30].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[30].f64 };
	// 82F52234: 4080FFC4  bge 0x82f521f8
	if !ctx.cr[0].lt {
	pc = 0x82F521F8; continue 'dispatch;
	}
	// 82F52238: 897B00D8  lbz r11, 0xd8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F5223C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82F52240: 419A0010  beq cr6, 0x82f52250
	if ctx.cr[6].eq {
	pc = 0x82F52250; continue 'dispatch;
	}
	// 82F52244: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82F52248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F5224C: 409A0008  bne cr6, 0x82f52254
	if !ctx.cr[6].eq {
	pc = 0x82F52254; continue 'dispatch;
	}
	// 82F52250: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F52254: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82F52258: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F5225C: 419A0044  beq cr6, 0x82f522a0
	if ctx.cr[6].eq {
	pc = 0x82F522A0; continue 'dispatch;
	}
	// 82F52260: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F52264: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F52268: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F5226C: 409A0020  bne cr6, 0x82f5228c
	if !ctx.cr[6].eq {
	pc = 0x82F5228C; continue 'dispatch;
	}
	// 82F52270: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F52274: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F52278: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F5227C: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F52280: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F52284: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F52288: 4BF4E529  bl 0x82ea07b0
	ctx.lr = 0x82F5228C;
	sub_82EA07B0(ctx, base);
	// 82F5228C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82F52290: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82F52294: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82F52298: 4825682D  bl 0x831a8ac4
	ctx.lr = 0x82F5229C;
	sub_831A8A8C(ctx, base);
	// 82F5229C: 48255F18  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82F522A0: 817B00D0  lwz r11, 0xd0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(208 as u32) ) } as u64;
	// 82F522A4: 387B00D0  addi r3, r27, 0xd0
	ctx.r[3].s64 = ctx.r[27].s64 + 208;
	// 82F522A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F522AC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F522B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F522B4: 4E800421  bctrl
	ctx.lr = 0x82F522B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F522B8: C0010064  lfs f0, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F522BC: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F522C0: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 82F522C4: ED606828  fsubs f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82F522C8: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F522CC: FD4B682E  fsel f10, f11, f0, f13
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82F522D0: ED2C5028  fsubs f9, f12, f10
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 82F522D4: FFA9532E  fsel f29, f9, f12, f10
	ctx.f[29].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[10].f64 };
	// 82F522D8: 419AFF88  beq cr6, 0x82f52260
	if ctx.cr[6].eq {
	pc = 0x82F52260; continue 'dispatch;
	}
	// 82F522DC: EF9D0732  fmuls f28, f29, f28
	ctx.f[28].f64 = (((ctx.f[29].f64 * ctx.f[28].f64) as f32) as f64);
	// 82F522E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F522E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F522E8: ED7FE028  fsubs f11, f31, f28
	ctx.f[11].f64 = (((ctx.f[31].f64 - ctx.f[28].f64) as f32) as f64);
	// 82F522EC: FD4BFF2E  fsel f10, f11, f28, f31
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[28].f64 } else { ctx.f[31].f64 };
	// 82F522F0: ED2AF028  fsubs f9, f10, f30
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F522F4: FFC9F2AE  fsel f30, f9, f10, f30
	ctx.f[30].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[30].f64 };
	// 82F522F8: ED0CF028  fsubs f8, f12, f30
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F522FC: ECE0F028  fsubs f7, f0, f30
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F52300: ECCDF028  fsubs f6, f13, f30
	ctx.f[6].f64 = (((ctx.f[13].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F52304: FCA8F32E  fsel f5, f8, f12, f30
	ctx.f[5].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[30].f64 };
	// 82F52308: D0A10050  stfs f5, 0x50(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F5230C: FC87F02E  fsel f4, f7, f0, f30
	ctx.f[4].f64 = if ctx.f[7].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[30].f64 };
	// 82F52310: D0810064  stfs f4, 0x64(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F52314: FC66F36E  fsel f3, f6, f13, f30
	ctx.f[3].f64 = if ctx.f[6].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[30].f64 };
	// 82F52318: D0610078  stfs f3, 0x78(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82F5231C: 4BF79215  bl 0x82ecb530
	ctx.lr = 0x82F52320;
	sub_82ECB530(ctx, base);
	// 82F52320: EC5DF82A  fadds f2, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ((ctx.f[29].f64 + ctx.f[31].f64) as f32) as f64;
	// 82F52324: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F52328: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5232C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F52330: EC3C1028  fsubs f1, f28, f2
	ctx.f[1].f64 = (((ctx.f[28].f64 - ctx.f[2].f64) as f32) as f64);
	// 82F52334: FC01E0AE  fsel f0, f1, f2, f28
	ctx.f[0].f64 = if ctx.f[1].f64 >= 0.0 { ctx.f[2].f64 } else { ctx.f[28].f64 };
	// 82F52338: EDA0F028  fsubs f13, f0, f30
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82F5233C: FFEDF02E  fsel f31, f13, f0, f30
	ctx.f[31].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[30].f64 };
	// 82F52340: 409A0020  bne cr6, 0x82f52360
	if !ctx.cr[6].eq {
	pc = 0x82F52360; continue 'dispatch;
	}
	// 82F52344: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F52348: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F5234C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F52350: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F52354: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F52358: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F5235C: 4BF4E455  bl 0x82ea07b0
	ctx.lr = 0x82F52360;
	sub_82EA07B0(ctx, base);
	// 82F52360: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F52364: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82F52368: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82F5236C: 48256759  bl 0x831a8ac4
	ctx.lr = 0x82F52370;
	sub_831A8A8C(ctx, base);
	// 82F52370: 48255E44  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F52378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F52378 size=300
    let mut pc: u32 = 0x82F52378;
    'dispatch: loop {
        match pc {
            0x82F52378 => {
    //   block [0x82F52378..0x82F524A4)
	// 82F52378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5237C: 48255DE5  bl 0x831a8160
	ctx.lr = 0x82F52380;
	sub_831A8130(ctx, base);
	// 82F52380: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82F52384: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F52388: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5238C: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 82F52390: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F52394: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F52398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F5239C: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82F523A0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F523A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F523A8: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F523AC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F523B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F523B4: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82F523B8: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82F523BC: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82F523C0: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F523C4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F523C8: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F523CC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F523D0: 4199000C  bgt cr6, 0x82f523dc
	if ctx.cr[6].gt {
	pc = 0x82F523DC; continue 'dispatch;
	}
	// 82F523D4: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F523D8: 48000018  b 0x82f523f0
	pc = 0x82F523F0; continue 'dispatch;
	// 82F523DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F523E0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F523E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F523E8: 4E800421  bctrl
	ctx.lr = 0x82F523EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F523EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F523F0: 7FEAF378  or r10, r31, r30
	ctx.r[10].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 82F523F4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F523F8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F523FC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F52400: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F52404: 40990038  ble cr6, 0x82f5243c
	if !ctx.cr[6].gt {
	pc = 0x82F5243C; continue 'dispatch;
	}
	// 82F52408: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F5240C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F52410: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F52414: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F52418: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F5241C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F52420: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F52424: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F52428: 7D07312E  stwx r8, r7, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), ctx.r[8].u32) };
	// 82F5242C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F52430: 39250001  addi r9, r5, 1
	ctx.r[9].s64 = ctx.r[5].s64 + 1;
	// 82F52434: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82F52438: 4082FFDC  bne 0x82f52414
	if !ctx.cr[0].eq {
	pc = 0x82F52414; continue 'dispatch;
	}
	// 82F5243C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F52440: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F52444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F52448: 4BFFFCB9  bl 0x82f52100
	ctx.lr = 0x82F5244C;
	sub_82F52100(ctx, base);
	// 82F5244C: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F52450: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F52454: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F52458: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82F5245C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82F52460: 409A0014  bne cr6, 0x82f52474
	if !ctx.cr[6].eq {
	pc = 0x82F52474; continue 'dispatch;
	}
	// 82F52464: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F52468: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F5246C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F52470: 4E800421  bctrl
	ctx.lr = 0x82F52474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F52474: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F52478: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5247C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F52480: 409A0018  bne cr6, 0x82f52498
	if !ctx.cr[6].eq {
	pc = 0x82F52498; continue 'dispatch;
	}
	// 82F52484: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F52488: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F5248C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F52490: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F52494: 4BF4E31D  bl 0x82ea07b0
	ctx.lr = 0x82F52498;
	sub_82EA07B0(ctx, base);
	// 82F52498: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F5249C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82F524A0: 48255D10  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F524A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F524A8 size=2224
    let mut pc: u32 = 0x82F524A8;
    'dispatch: loop {
        match pc {
            0x82F524A8 => {
    //   block [0x82F524A8..0x82F52D58)
	// 82F524A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F524AC: 48255CA1  bl 0x831a814c
	ctx.lr = 0x82F524B0;
	sub_831A8130(ctx, base);
	// 82F524B0: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82F524B4: 482565B9  bl 0x831a8a6c
	ctx.lr = 0x82F524B8;
	sub_831A8A40(ctx, base);
	// 82F524B8: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F52D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F52D58 size=1228
    let mut pc: u32 = 0x82F52D58;
    'dispatch: loop {
        match pc {
            0x82F52D58 => {
    //   block [0x82F52D58..0x82F53224)
	// 82F52D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F52D5C: 482553ED  bl 0x831a8148
	ctx.lr = 0x82F52D60;
	sub_831A8130(ctx, base);
	// 82F52D60: 3981FF98  addi r12, r1, -0x68
	ctx.r[12].s64 = ctx.r[1].s64 + -104;
	// 82F52D64: 48255D0D  bl 0x831a8a70
	ctx.lr = 0x82F52D68;
	sub_831A8A40(ctx, base);
	// 82F52D68: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F53228 size=1688
    //   switch @ 0x82F532C4: r11 with 27 label(s)
    //       case  0 → 0x82F53334
    //       case  1 → 0x82F53688
    //       case  2 → 0x82F53428
    //       case  3 → 0x82F5334C
    //       case  4 → 0x82F53668
    //       case  5 → 0x82F53370
    //       case  6 → 0x82F535DC
    //       case  7 → 0x82F535DC
    //       case  8 → 0x82F535DC
    //       case  9 → 0x82F535DC
    //       case 10 → 0x82F53488
    //       case 11 → 0x82F534F4
    //       case 12 → 0x82F538A8
    //       case 13 → 0x82F535DC
    //       case 14 → 0x82F538A8
    //       case 15 → 0x82F538A8
    //       case 16 → 0x82F538A8
    //       case 17 → 0x82F538A8
    //       case 18 → 0x82F5353C
    //       case 19 → 0x82F535DC
    //       case 20 → 0x82F535DC
    //       case 21 → 0x82F538A8
    //       case 22 → 0x82F538A8
    //       case 23 → 0x82F538A8
    //       case 24 → 0x82F53474
    //       case 25 → 0x82F538A8
    //       case 26 → 0x82F53518
    let mut pc: u32 = 0x82F53228;
    'dispatch: loop {
        match pc {
            0x82F53228 => {
    //   block [0x82F53228..0x82F53334)
	// 82F53228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5322C: 48254F29  bl 0x831a8154
	ctx.lr = 0x82F53230;
	sub_831A8130(ctx, base);
	// 82F53230: DBC1FFA0  stfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 82F53234: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82F53238: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82F5323C: 482579B9  bl 0x831aabf4
	ctx.lr = 0x82F53240;
	sub_831AA9A0(ctx, base);
	// 82F53240: 9421F9C0  stwu r1, -0x640(r1)
	ea = ctx.r[1].u32.wrapping_add(-1600 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53244: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F53248: 392100C0  addi r9, r1, 0xc0
	ctx.r[9].s64 = ctx.r[1].s64 + 192;
	// 82F5324C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F53250: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82F53254: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	pc = 0x82F53334; continue 'dispatch;
            }
            0x82F53334 => {
    //   block [0x82F53334..0x82F5334C)
	// 82F53334: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F53338: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F5333C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F53340: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F53344: 4BFFB8ED  bl 0x82f4ec30
	ctx.lr = 0x82F53348;
	sub_82F4EC30(ctx, base);
	// 82F53348: 48000364  b 0x82f536ac
	pc = 0x82F536AC; continue 'dispatch;
            }
            0x82F5334C => {
    //   block [0x82F5334C..0x82F53370)
	// 82F5334C: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	pc = 0x82F53370; continue 'dispatch;
            }
            0x82F53370 => {
    //   block [0x82F53370..0x82F53428)
	// 82F53370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F53378: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F5337C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F53380: 4E800421  bctrl
	ctx.lr = 0x82F53384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F53384: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53388: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82F5338C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F53390: 393B0001  addi r9, r27, 1
	ctx.r[9].s64 = ctx.r[27].s64 + 1;
	// 82F53394: 7C79D02E  lwzx r3, r25, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F53398: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F5339C: 83A30020  lwz r29, 0x20(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F533A0: 8103002C  lwz r8, 0x2c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F533A4: 7D7D2214  add r11, r29, r4
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[4].u64;
	// 82F533A8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F533AC: 4199000C  bgt cr6, 0x82f533b8
	if ctx.cr[6].gt {
	pc = 0x82F533B8; continue 'dispatch;
	}
	// 82F533B0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82F533B4: 48000018  b 0x82f533cc
	pc = 0x82F533CC; continue 'dispatch;
	// 82F533B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F533BC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F533C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F533C4: 4E800421  bctrl
	ctx.lr = 0x82F533C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F533C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F533CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F533D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F533D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F533D8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F533DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F533E0: 4E800421  bctrl
	ctx.lr = 0x82F533E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F533E4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82F533E8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F533EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F533F0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F533F4: C02908A8  lfs f1, 0x8a8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F533F8: 4BFFDCA9  bl 0x82f510a0
	ctx.lr = 0x82F533FC;
	sub_82F510A0(ctx, base);
	// 82F533FC: 7C79D02E  lwzx r3, r25, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F53400: 81030028  lwz r8, 0x28(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F53404: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82F53408: 7F1D4040  cmplw cr6, r29, r8
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F5340C: 409A02A0  bne cr6, 0x82f536ac
	if !ctx.cr[6].eq {
	pc = 0x82F536AC; continue 'dispatch;
	}
	// 82F53410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F53418: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F5341C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F53420: 4E800421  bctrl
	ctx.lr = 0x82F53424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F53424: 48000288  b 0x82f536ac
	pc = 0x82F536AC; continue 'dispatch;
            }
            0x82F53428 => {
    //   block [0x82F53428..0x82F53474)
	// 82F53428: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	pc = 0x82F53474; continue 'dispatch;
            }
            0x82F53474 => {
    //   block [0x82F53474..0x82F53488)
	// 82F53474: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82F53478: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F5347C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F53480: 4BFFFDA9  bl 0x82f53228
	ctx.lr = 0x82F53484;
	sub_82F53228(ctx, base);
	// 82F53484: 48000424  b 0x82f538a8
	pc = 0x82F538A8; continue 'dispatch;
            }
            0x82F53488 => {
    //   block [0x82F53488..0x82F534F4)
	// 82F53488: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82F5348C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F53490: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82F53494: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 82F53498: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	pc = 0x82F534F4; continue 'dispatch;
            }
            0x82F534F4 => {
    //   block [0x82F534F4..0x82F53518)
	// 82F534F4: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82F534F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F534FC: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 82F53500: 4BF4DAA9  bl 0x82ea0fa8
	ctx.lr = 0x82F53504;
	sub_82EA0FA8(ctx, base);
	// 82F53504: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82F53508: 38810290  addi r4, r1, 0x290
	ctx.r[4].s64 = ctx.r[1].s64 + 656;
	// 82F5350C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F53510: 4BFFFD19  bl 0x82f53228
	ctx.lr = 0x82F53514;
	sub_82F53228(ctx, base);
	// 82F53514: 48000394  b 0x82f538a8
	pc = 0x82F538A8; continue 'dispatch;
            }
            0x82F53518 => {
    //   block [0x82F53518..0x82F5353C)
	// 82F53518: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F5351C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F53520: 38610250  addi r3, r1, 0x250
	ctx.r[3].s64 = ctx.r[1].s64 + 592;
	// 82F53524: 4BF4DA85  bl 0x82ea0fa8
	ctx.lr = 0x82F53528;
	sub_82EA0FA8(ctx, base);
	// 82F53528: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82F5352C: 38810250  addi r4, r1, 0x250
	ctx.r[4].s64 = ctx.r[1].s64 + 592;
	// 82F53530: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F53534: 4BFFFCF5  bl 0x82f53228
	ctx.lr = 0x82F53538;
	sub_82F53228(ctx, base);
	// 82F53538: 48000370  b 0x82f538a8
	pc = 0x82F538A8; continue 'dispatch;
            }
            0x82F5353C => {
    //   block [0x82F5353C..0x82F535DC)
	// 82F5353C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F53540: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F53544: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F53548: 40990360  ble cr6, 0x82f538a8
	if !ctx.cr[6].gt {
	pc = 0x82F538A8; continue 'dispatch;
	}
	// 82F5354C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F53550: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82F53554: 3B5C0020  addi r26, r28, 0x20
	ctx.r[26].s64 = ctx.r[28].s64 + 32;
	// 82F53558: 3B3C0030  addi r25, r28, 0x30
	ctx.r[25].s64 = ctx.r[28].s64 + 48;
	// 82F5355C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82F53560: 3B0B9EAC  addi r24, r11, -0x6154
	ctx.r[24].s64 = ctx.r[11].s64 + -24916;
	// 82F53564: 39610120  addi r11, r1, 0x120
	ctx.r[11].s64 = ctx.r[1].s64 + 288;
	pc = 0x82F535DC; continue 'dispatch;
            }
            0x82F535DC => {
    //   block [0x82F535DC..0x82F53668)
	// 82F535DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F535E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F535E4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F535E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F535EC: 4E800421  bctrl
	ctx.lr = 0x82F535F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F535F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F535F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F535F8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F535FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F53600: 4E800421  bctrl
	ctx.lr = 0x82F53604;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F53604: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F53608: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F5360C: 419A029C  beq cr6, 0x82f538a8
	if ctx.cr[6].eq {
	pc = 0x82F538A8; continue 'dispatch;
	}
	// 82F53610: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53614: 38A10360  addi r5, r1, 0x360
	ctx.r[5].s64 = ctx.r[1].s64 + 864;
	// 82F53618: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F5361C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F53620: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F53624: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F53628: 4E800421  bctrl
	ctx.lr = 0x82F5362C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F5362C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F53630: 419A0010  beq cr6, 0x82f53640
	if ctx.cr[6].eq {
	pc = 0x82F53640; continue 'dispatch;
	}
	// 82F53634: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82F53638: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F5363C: 4BFFFBED  bl 0x82f53228
	ctx.lr = 0x82F53640;
	sub_82F53228(ctx, base);
	// 82F53640: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53644: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F53648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F5364C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F53650: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F53654: 4E800421  bctrl
	ctx.lr = 0x82F53658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F53658: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F5365C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F53660: 409AFFB0  bne cr6, 0x82f53610
	if !ctx.cr[6].eq {
	pc = 0x82F53610; continue 'dispatch;
	}
	// 82F53664: 48000244  b 0x82f538a8
	pc = 0x82F538A8; continue 'dispatch;
            }
            0x82F53668 => {
    //   block [0x82F53668..0x82F53688)
	// 82F53668: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F5366C: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F53670: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F53674: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82F53678: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F5367C: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F53680: 4BFFDCB9  bl 0x82f51338
	ctx.lr = 0x82F53684;
	sub_82F51338(ctx, base);
	// 82F53684: 48000028  b 0x82f536ac
	pc = 0x82F536AC; continue 'dispatch;
            }
            0x82F53688 => {
    //   block [0x82F53688..0x82F538A8)
	// 82F53688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F5368C: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82F53690: 4BFCAE01  bl 0x82f1e490
	ctx.lr = 0x82F53694;
	sub_82F1E490(ctx, base);
	// 82F53694: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F53698: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82F5369C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82F536A0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82F536A4: C04B08A8  lfs f2, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F536A8: 4BFFE701  bl 0x82f51da8
	ctx.lr = 0x82F536AC;
	sub_82F51DA8(ctx, base);
	// 82F536AC: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F536B0: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82F536B4: 419A01F4  beq cr6, 0x82f538a8
	if ctx.cr[6].eq {
	pc = 0x82F538A8; continue 'dispatch;
	}
	// 82F536B8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F536BC: 4BFFB965  bl 0x82f4f020
	ctx.lr = 0x82F536C0;
	sub_82F4F020(ctx, base);
	// 82F536C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F536C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F536C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F536CC: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82F536D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F536D4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F536D8: 48000419  bl 0x82f53af0
	ctx.lr = 0x82F536DC;
	sub_82F53AF0(ctx, base);
	// 82F536DC: 39770020  addi r11, r23, 0x20
	ctx.r[11].s64 = ctx.r[23].s64 + 32;
	// 82F536E0: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82F536E4: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F536E8: 39210190  addi r9, r1, 0x190
	ctx.r[9].s64 = ctx.r[1].s64 + 400;
	// 82F536EC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F536F0: 390101A0  addi r8, r1, 0x1a0
	ctx.r[8].s64 = ctx.r[1].s64 + 416;
	// 82F536F4: 38E101B0  addi r7, r1, 0x1b0
	ctx.r[7].s64 = ctx.r[1].s64 + 432;
	// 82F536F8: 38C101C0  addi r6, r1, 0x1c0
	ctx.r[6].s64 = ctx.r[1].s64 + 448;
	// 82F536FC: 54A400BE  clrlwi r4, r5, 2
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 82F53700: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82F53704: C3F70000  lfs f31, 0(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F53708: C3D70004  lfs f30, 4(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	pc = 0x82F538A8; continue 'dispatch;
            }
            0x82F538A8 => {
    //   block [0x82F538A8..0x82F538C0)
	// 82F538A8: 38210640  addi r1, r1, 0x640
	ctx.r[1].s64 = ctx.r[1].s64 + 1600;
	// 82F538AC: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82F538B0: 482575DD  bl 0x831aae8c
	ctx.lr = 0x82F538B4;
	sub_831AAC38(ctx, base);
	// 82F538B4: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82F538B8: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82F538BC: 482548E8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F538C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F538C0 size=368
    let mut pc: u32 = 0x82F538C0;
    'dispatch: loop {
        match pc {
            0x82F538C0 => {
    //   block [0x82F538C0..0x82F53A30)
	// 82F538C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F538C4: 482548A9  bl 0x831a816c
	ctx.lr = 0x82F538C8;
	sub_831A8130(ctx, base);
	// 82F538C8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F538CC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F538D0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F538D4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F538D8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82F538DC: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F538E0: 392100F0  addi r9, r1, 0xf0
	ctx.r[9].s64 = ctx.r[1].s64 + 240;
	// 82F538E4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F538E8: 38E100F0  addi r7, r1, 0xf0
	ctx.r[7].s64 = ctx.r[1].s64 + 240;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F53A30 size=192
    let mut pc: u32 = 0x82F53A30;
    'dispatch: loop {
        match pc {
            0x82F53A30 => {
    //   block [0x82F53A30..0x82F53AF0)
	// 82F53A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F53A38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F53A3C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53A40: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F53A44: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F53A48: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82F53A4C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F53A50: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F53AF0 size=136
    let mut pc: u32 = 0x82F53AF0;
    'dispatch: loop {
        match pc {
            0x82F53AF0 => {
    //   block [0x82F53AF0..0x82F53B78)
	// 82F53AF0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F53AF4: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82F53AF8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82F53AFC: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 82F53B00: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82F53B04: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F53B08: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F53B0C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82F53B10: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82F53B14: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F53B18: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F53B78 size=108
    let mut pc: u32 = 0x82F53B78;
    'dispatch: loop {
        match pc {
            0x82F53B78 => {
    //   block [0x82F53B78..0x82F53BE4)
	// 82F53B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F53B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F53B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F53B8C: 482AD8CD  bl 0x83201458
	ctx.lr = 0x82F53B90;
	sub_83201458(ctx, base);
	// 82F53B90: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F53B94: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82F53B98: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F53B9C: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F53BA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F53BA4: 38C70D18  addi r6, r7, 0xd18
	ctx.r[6].s64 = ctx.r[7].s64 + 3352;
	// 82F53BA8: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F53BAC: C1A9DD6C  lfs f13, -0x2294(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F53BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F53BB4: C188964C  lfs f12, -0x69b4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F53BB8: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F53BBC: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82F53BC0: D1BF0054  stfs f13, 0x54(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F53BC4: D19F0058  stfs f12, 0x58(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82F53BC8: 997F005C  stb r11, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u8 ) };
	// 82F53BCC: 997F005D  stb r11, 0x5d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(93 as u32), ctx.r[11].u8 ) };
	// 82F53BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F53BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F53BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F53BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F53BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F53BE8 size=136
    let mut pc: u32 = 0x82F53BE8;
    'dispatch: loop {
        match pc {
            0x82F53BE8 => {
    //   block [0x82F53BE8..0x82F53C70)
	// 82F53BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53BEC: 4825457D  bl 0x831a8168
	ctx.lr = 0x82F53BF0;
	sub_831A8130(ctx, base);
	// 82F53BF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F53BF8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F53BFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F53C00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F53C04: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F53C08: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F53C0C: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F53C10: 388B00E0  addi r4, r11, 0xe0
	ctx.r[4].s64 = ctx.r[11].s64 + 224;
	// 82F53C14: 4BF5329D  bl 0x82ea6eb0
	ctx.lr = 0x82F53C18;
	sub_82EA6EB0(ctx, base);
	// 82F53C18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F53C1C: 389E00E0  addi r4, r30, 0xe0
	ctx.r[4].s64 = ctx.r[30].s64 + 224;
	// 82F53C20: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82F53C24: 4BF5328D  bl 0x82ea6eb0
	ctx.lr = 0x82F53C28;
	sub_82EA6EB0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F53C70 size=164
    let mut pc: u32 = 0x82F53C70;
    'dispatch: loop {
        match pc {
            0x82F53C70 => {
    //   block [0x82F53C70..0x82F53D14)
	// 82F53C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53C74: 482544F9  bl 0x831a816c
	ctx.lr = 0x82F53C78;
	sub_831A8130(ctx, base);
	// 82F53C78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53C7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F53D18 size=200
    let mut pc: u32 = 0x82F53D18;
    'dispatch: loop {
        match pc {
            0x82F53D18 => {
    //   block [0x82F53D18..0x82F53DE0)
	// 82F53D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F53D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F53D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F53D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F53D2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F53D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F53D34: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F53D38: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82F53D3C: 419A000C  beq cr6, 0x82f53d48
	if ctx.cr[6].eq {
	pc = 0x82F53D48; continue 'dispatch;
	}
	// 82F53D40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F53D44: 48000084  b 0x82f53dc8
	pc = 0x82F53DC8; continue 'dispatch;
	// 82F53D48: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F53D4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F53D50: 409AFFF0  bne cr6, 0x82f53d40
	if !ctx.cr[6].eq {
	pc = 0x82F53D40; continue 'dispatch;
	}
	// 82F53D54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53D58: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F53D5C: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82F53D60: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82F53D64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F53D68: 4BF4C9C9  bl 0x82ea0730
	ctx.lr = 0x82F53D6C;
	sub_82EA0730(ctx, base);
	// 82F53D6C: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 82F53D70: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F53D74: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53D78: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F53D7C: 80A80004  lwz r5, 4(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F53D80: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F53D84: 4BFFFDF5  bl 0x82f53b78
	ctx.lr = 0x82F53D88;
	sub_82F53B78(ctx, base);
	// 82F53D88: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82F53D8C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F53DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F53DE0 size=564
    let mut pc: u32 = 0x82F53DE0;
    'dispatch: loop {
        match pc {
            0x82F53DE0 => {
    //   block [0x82F53DE0..0x82F54014)
	// 82F53DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F53DE4: 48254379  bl 0x831a815c
	ctx.lr = 0x82F53DE8;
	sub_831A8130(ctx, base);
	// 82F53DE8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82F53DEC: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F54018 size=396
    let mut pc: u32 = 0x82F54018;
    'dispatch: loop {
        match pc {
            0x82F54018 => {
    //   block [0x82F54018..0x82F541A4)
	// 82F54018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5401C: 4825414D  bl 0x831a8168
	ctx.lr = 0x82F54020;
	sub_831A8130(ctx, base);
	// 82F54020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F54024: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F54028: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5402C: 556A17FE  rlwinm r10, r11, 2, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54030: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54034: 409A0110  bne cr6, 0x82f54144
	if !ctx.cr[6].eq {
	pc = 0x82F54144; continue 'dispatch;
	}
	// 82F54038: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5403C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F54040: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F54044: 40990074  ble cr6, 0x82f540b8
	if !ctx.cr[6].gt {
	pc = 0x82F540B8; continue 'dispatch;
	}
	// 82F54048: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F5404C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54050: 7CBF582E  lwzx r5, r31, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F54054: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82F54058: 419A004C  beq cr6, 0x82f540a4
	if ctx.cr[6].eq {
	pc = 0x82F540A4; continue 'dispatch;
	}
	// 82F5405C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54060: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54064: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F54068: 8123009C  lwz r9, 0x9c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82F5406C: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F54070: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F54074: 41980014  blt cr6, 0x82f54088
	if ctx.cr[6].lt {
	pc = 0x82F54088; continue 'dispatch;
	}
	// 82F54078: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F5407C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82F54080: 4BF4BFE1  bl 0x82ea0060
	ctx.lr = 0x82F54084;
	sub_82EA0060(ctx, base);
	// 82F54084: 48000020  b 0x82f540a4
	pc = 0x82F540A4; continue 'dispatch;
	// 82F54088: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82F5408C: 39630098  addi r11, r3, 0x98
	ctx.r[11].s64 = ctx.r[3].s64 + 152;
	// 82F54090: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82F54094: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F54098: 9143009C  stw r10, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 82F5409C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F540A0: 90A30098  stw r5, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[5].u32 ) };
	// 82F540A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F540A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F540AC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F540B0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F540B4: 4198FF98  blt cr6, 0x82f5404c
	if ctx.cr[6].lt {
	pc = 0x82F5404C; continue 'dispatch;
	}
	// 82F540B8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F540BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F540C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F540C4: 40990080  ble cr6, 0x82f54144
	if !ctx.cr[6].gt {
	pc = 0x82F54144; continue 'dispatch;
	}
	// 82F540C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F540CC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F540D0: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F540D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F540D8: 419A0058  beq cr6, 0x82f54130
	if ctx.cr[6].eq {
	pc = 0x82F54130; continue 'dispatch;
	}
	// 82F540DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F540E0: 4BFFFF39  bl 0x82f54018
	ctx.lr = 0x82F540E4;
	sub_82F54018(ctx, base);
	// 82F540E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F540E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F540EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F540F0: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F540F4: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F540F8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F540FC: 41980018  blt cr6, 0x82f54114
	if ctx.cr[6].lt {
	pc = 0x82F54114; continue 'dispatch;
	}
	// 82F54100: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F54104: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F54108: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82F5410C: 4BF4BF55  bl 0x82ea0060
	ctx.lr = 0x82F54110;
	sub_82EA0060(ctx, base);
	// 82F54110: 48000020  b 0x82f54130
	pc = 0x82F54130; continue 'dispatch;
	// 82F54114: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F54118: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 82F5411C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F54120: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F54124: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F54128: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F5412C: 93E30050  stw r31, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82F54130: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F54134: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F54138: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82F5413C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54140: 4198FF8C  blt cr6, 0x82f540cc
	if ctx.cr[6].lt {
	pc = 0x82F540CC; continue 'dispatch;
	}
	// 82F54144: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F54148: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5414C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54150: 409A0020  bne cr6, 0x82f54170
	if !ctx.cr[6].eq {
	pc = 0x82F54170; continue 'dispatch;
	}
	// 82F54154: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54158: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F5415C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54160: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54164: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54168: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F5416C: 4BF4C645  bl 0x82ea07b0
	ctx.lr = 0x82F54170;
	sub_82EA07B0(ctx, base);
	// 82F54170: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F54174: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54178: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F5417C: 409A0020  bne cr6, 0x82f5419c
	if !ctx.cr[6].eq {
	pc = 0x82F5419C; continue 'dispatch;
	}
	// 82F54180: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54184: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54188: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F5418C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54190: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54194: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F54198: 4BF4C619  bl 0x82ea07b0
	ctx.lr = 0x82F5419C;
	sub_82EA07B0(ctx, base);
	// 82F5419C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F541A0: 48254018  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F541A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F541A8 size=236
    let mut pc: u32 = 0x82F541A8;
    'dispatch: loop {
        match pc {
            0x82F541A8 => {
    //   block [0x82F541A8..0x82F54294)
	// 82F541A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F541AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F541B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F541B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F541B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F541BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F541C0: 394B02E8  addi r10, r11, 0x2e8
	ctx.r[10].s64 = ctx.r[11].s64 + 744;
	// 82F541C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F541C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F541CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F541D0: 419A0008  beq cr6, 0x82f541d8
	if ctx.cr[6].eq {
	pc = 0x82F541D8; continue 'dispatch;
	}
	// 82F541D4: 4BF80F35  bl 0x82ed5108
	ctx.lr = 0x82F541D8;
	sub_82ED5108(ctx, base);
	// 82F541D8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F541DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F541E0: 419A0008  beq cr6, 0x82f541e8
	if ctx.cr[6].eq {
	pc = 0x82F541E8; continue 'dispatch;
	}
	// 82F541E4: 4BF80F25  bl 0x82ed5108
	ctx.lr = 0x82F541E8;
	sub_82ED5108(ctx, base);
	// 82F541E8: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 82F541EC: 4BFFFE2D  bl 0x82f54018
	ctx.lr = 0x82F541F0;
	sub_82F54018(ctx, base);
	// 82F541F0: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F541F4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F541F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F541FC: 409A0020  bne cr6, 0x82f5421c
	if !ctx.cr[6].eq {
	pc = 0x82F5421C; continue 'dispatch;
	}
	// 82F54200: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54204: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54208: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F5420C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F54210: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54214: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F54218: 4BF4C599  bl 0x82ea07b0
	ctx.lr = 0x82F5421C;
	sub_82EA07B0(ctx, base);
	// 82F5421C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F54220: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54224: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54228: 409A0020  bne cr6, 0x82f54248
	if !ctx.cr[6].eq {
	pc = 0x82F54248; continue 'dispatch;
	}
	// 82F5422C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54230: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54234: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54238: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F5423C: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54240: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F54244: 4BF4C56D  bl 0x82ea07b0
	ctx.lr = 0x82F54248;
	sub_82EA07B0(ctx, base);
	// 82F54248: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F5424C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54250: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54254: 409A0020  bne cr6, 0x82f54274
	if !ctx.cr[6].eq {
	pc = 0x82F54274; continue 'dispatch;
	}
	// 82F54258: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5425C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54260: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54264: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F54268: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F5426C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F54270: 4BF4C541  bl 0x82ea07b0
	ctx.lr = 0x82F54274;
	sub_82EA07B0(ctx, base);
	// 82F54274: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F54278: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F5427C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F54280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F54284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F54288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F5428C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F54290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F54298 size=144
    let mut pc: u32 = 0x82F54298;
    'dispatch: loop {
        match pc {
            0x82F54298 => {
    //   block [0x82F54298..0x82F54328)
	// 82F54298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5429C: 48253ED1  bl 0x831a816c
	ctx.lr = 0x82F542A0;
	sub_831A8130(ctx, base);
	// 82F542A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F542A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F542A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F542AC: 394B0404  addi r10, r11, 0x404
	ctx.r[10].s64 = ctx.r[11].s64 + 1028;
	// 82F542B0: 83FD000C  lwz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F542B4: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F542B8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F542BC: 40990020  ble cr6, 0x82f542dc
	if !ctx.cr[6].gt {
	pc = 0x82F542DC; continue 'dispatch;
	}
	// 82F542C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F542C4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F542C8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F542CC: 4BF80E3D  bl 0x82ed5108
	ctx.lr = 0x82F542D0;
	sub_82ED5108(ctx, base);
	// 82F542D0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F542D4: 3BDE0050  addi r30, r30, 0x50
	ctx.r[30].s64 = ctx.r[30].s64 + 80;
	// 82F542D8: 4082FFEC  bne 0x82f542c4
	if !ctx.cr[0].eq {
	pc = 0x82F542C4; continue 'dispatch;
	}
	// 82F542DC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F542E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F542E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F542E8: 409A002C  bne cr6, 0x82f54314
	if !ctx.cr[6].eq {
	pc = 0x82F54314; continue 'dispatch;
	}
	// 82F542EC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F542F0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F542F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F542F8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F542FC: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82F54300: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54304: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54308: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F5430C: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F54310: 4BF4C4A1  bl 0x82ea07b0
	ctx.lr = 0x82F54314;
	sub_82EA07B0(ctx, base);
	// 82F54314: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F54318: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F5431C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F54320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F54324: 48253E98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F54328 size=276
    let mut pc: u32 = 0x82F54328;
    'dispatch: loop {
        match pc {
            0x82F54328 => {
    //   block [0x82F54328..0x82F5443C)
	// 82F54328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5432C: 48253E31  bl 0x831a815c
	ctx.lr = 0x82F54330;
	sub_831A8130(ctx, base);
	// 82F54330: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F54334: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F54338: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F5433C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F54340: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82F54344: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F54348: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5434C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54350: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F54354: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F54358: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5435C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54360: 409900D4  ble cr6, 0x82f54434
	if !ctx.cr[6].gt {
	pc = 0x82F54434; continue 'dispatch;
	}
	// 82F54364: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54368: 815A0014  lwz r10, 0x14(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F5436C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82F54370: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54374: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F54378: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F5437C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54380: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54384: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F54388: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F5438C: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54390: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F54394: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82F54398: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F5439C: 7C66482E  lwzx r3, r6, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F543A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F543A4: 419A003C  beq cr6, 0x82f543e0
	if ctx.cr[6].eq {
	pc = 0x82F543E0; continue 'dispatch;
	}
	// 82F543A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F543AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F543B0: 419A0030  beq cr6, 0x82f543e0
	if ctx.cr[6].eq {
	pc = 0x82F543E0; continue 'dispatch;
	}
	// 82F543B4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F543B8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F543BC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F543C0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F543C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F543C8: 409A0018  bne cr6, 0x82f543e0
	if !ctx.cr[6].eq {
	pc = 0x82F543E0; continue 'dispatch;
	}
	// 82F543CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F543D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F543D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F543D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F543DC: 4E800421  bctrl
	ctx.lr = 0x82F543E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F543E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F543E4: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82F543E8: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F543EC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F543F0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F543F4: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F543F8: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F543FC: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 82F54400: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F54404: 7F26492E  stwx r25, r6, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u32) };
	// 82F54408: 419A001C  beq cr6, 0x82f54424
	if ctx.cr[6].eq {
	pc = 0x82F54424; continue 'dispatch;
	}
	// 82F5440C: A1790004  lhz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F54414: 419A0010  beq cr6, 0x82f54424
	if ctx.cr[6].eq {
	pc = 0x82F54424; continue 'dispatch;
	}
	// 82F54418: A1790006  lhz r11, 6(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F5441C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F54420: B1590006  sth r10, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F54424: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54428: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82F5442C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54430: 4198FF34  blt cr6, 0x82f54364
	if ctx.cr[6].lt {
	pc = 0x82F54364; continue 'dispatch;
	}
	// 82F54434: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F54438: 48253D74  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F54440 size=36
    let mut pc: u32 = 0x82F54440;
    'dispatch: loop {
        match pc {
            0x82F54440 => {
    //   block [0x82F54440..0x82F54464)
	// 82F54440: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54444: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F54448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F5444C: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82F54450: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54454: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F54458: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5445C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F54460: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54464(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F54464 size=80
    let mut pc: u32 = 0x82F54464;
    'dispatch: loop {
        match pc {
            0x82F54464 => {
    //   block [0x82F54464..0x82F544B4)
	// 82F54464: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82F54468: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F544B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F544B8 size=212
    let mut pc: u32 = 0x82F544B8;
    'dispatch: loop {
        match pc {
            0x82F544B8 => {
    //   block [0x82F544B8..0x82F5458C)
	// 82F544B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F544BC: 48253CA5  bl 0x831a8160
	ctx.lr = 0x82F544C0;
	sub_831A8130(ctx, base);
	// 82F544C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F544C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F544C8: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F544CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F544D0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82F544D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F544D8: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F544DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F544E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F544E4: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F544E8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F544EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F544F0: 40990094  ble cr6, 0x82f54584
	if !ctx.cr[6].gt {
	pc = 0x82F54584; continue 'dispatch;
	}
	// 82F544F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F544F8: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F544FC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F54500: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54504: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F54508: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5450C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54510: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54514: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54518: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F5451C: 80E90018  lwz r7, 0x18(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F54520: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82F54524: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82F54528: 54C5103A  slwi r5, r6, 2
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F5452C: 7FE5382E  lwzx r31, r5, r7
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82F54530: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F54534: 419A0040  beq cr6, 0x82f54574
	if ctx.cr[6].eq {
	pc = 0x82F54574; continue 'dispatch;
	}
	// 82F54538: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5453C: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54540: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54544: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F54548: 409A0010  bne cr6, 0x82f54558
	if !ctx.cr[6].eq {
	pc = 0x82F54558; continue 'dispatch;
	}
	// 82F5454C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F54550: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F54554: 4BF5232D  bl 0x82ea6880
	ctx.lr = 0x82F54558;
	sub_82EA6880(ctx, base);
	// 82F54558: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5455C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54560: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F54564: 7FE9512E  stwx r31, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82F54568: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5456C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F54570: 911A0004  stw r8, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F54574: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54578: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F5457C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54580: 4198FF74  blt cr6, 0x82f544f4
	if ctx.cr[6].lt {
	pc = 0x82F544F4; continue 'dispatch;
	}
	// 82F54584: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F54588: 48253C28  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F54590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F54590 size=2984
    let mut pc: u32 = 0x82F54590;
    'dispatch: loop {
        match pc {
            0x82F54590 => {
    //   block [0x82F54590..0x82F55138)
	// 82F54590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F54594: 48253B9D  bl 0x831a8130
	ctx.lr = 0x82F54598;
	sub_831A8130(ctx, base);
	// 82F54598: 9421FC70  stwu r1, -0x390(r1)
	ea = ctx.r[1].u32.wrapping_add(-912 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5459C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F545A0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82F545A4: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 82F545A8: B06103A6  sth r3, 0x3a6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(934 as u32), ctx.r[3].u16 ) };
	// 82F545AC: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 82F545B0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F545B4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82F545B8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82F545BC: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F545C0: 936103BC  stw r27, 0x3bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(956 as u32), ctx.r[27].u32 ) };
	// 82F545C4: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82F545C8: 4BF4C169  bl 0x82ea0730
	ctx.lr = 0x82F545CC;
	sub_82EA0730(ctx, base);
	// 82F545CC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82F545D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F545D4: 3940002C  li r10, 0x2c
	ctx.r[10].s64 = 44;
	// 82F545D8: 392B06FC  addi r9, r11, 0x6fc
	ctx.r[9].s64 = ctx.r[11].s64 + 1788;
	// 82F545DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F545E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F545E4: 91340000  stw r9, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F545E8: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 82F545EC: B1540004  sth r10, 4(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82F545F0: B1140006  sth r8, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F545F4: 3B340008  addi r25, r20, 8
	ctx.r[25].s64 = ctx.r[20].s64 + 8;
	// 82F545F8: 93B40008  stw r29, 8(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F545FC: 3A140014  addi r16, r20, 0x14
	ctx.r[16].s64 = ctx.r[20].s64 + 20;
	// 82F54600: 93B4000C  stw r29, 0xc(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82F54604: 3AF40020  addi r23, r20, 0x20
	ctx.r[23].s64 = ctx.r[20].s64 + 32;
	// 82F54608: 92D40010  stw r22, 0x10(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(16 as u32), ctx.r[22].u32 ) };
	// 82F5460C: 93B40014  stw r29, 0x14(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82F54610: 93B40018  stw r29, 0x18(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82F54614: 92D4001C  stw r22, 0x1c(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(28 as u32), ctx.r[22].u32 ) };
	// 82F54618: 93B40020  stw r29, 0x20(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82F5461C: 93B40024  stw r29, 0x24(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82F54620: 92D40028  stw r22, 0x28(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(40 as u32), ctx.r[22].u32 ) };
	// 82F54624: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54628: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F5462C: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82F54630: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F54634: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82F54638: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82F5463C: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82F54640: 92C10080  stw r22, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[22].u32 ) };
	// 82F54644: 4099001C  ble cr6, 0x82f54660
	if !ctx.cr[6].gt {
	pc = 0x82F54660; continue 'dispatch;
	}
	// 82F54648: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F5464C: 41980008  blt cr6, 0x82f54654
	if ctx.cr[6].lt {
	pc = 0x82F54654; continue 'dispatch;
	}
	// 82F54650: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F54654: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F54658: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82F5465C: 4BF5219D  bl 0x82ea67f8
	ctx.lr = 0x82F54660;
	sub_82EA67F8(ctx, base);
	// 82F54660: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 82F54664: 57E5103A  slwi r5, r31, 2
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54668: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F5466C: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54670: 4BF560E9  bl 0x82eaa758
	ctx.lr = 0x82F54674;
	sub_82EAA758(ctx, base);
	// 82F54674: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F54678: 4BF57379  bl 0x82eab9f0
	ctx.lr = 0x82F5467C;
	sub_82EAB9F0(ctx, base);
	// 82F5467C: 816E0004  lwz r11, 4(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54680: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82F54684: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F54688: 40990030  ble cr6, 0x82f546b8
	if !ctx.cr[6].gt {
	pc = 0x82F546B8; continue 'dispatch;
	}
	// 82F5468C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82F54690: 816E0000  lwz r11, 0(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54694: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F54698: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F5469C: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F546A0: 4BF57409  bl 0x82eabaa8
	ctx.lr = 0x82F546A4;
	sub_82EABAA8(ctx, base);
	// 82F546A4: 814E0004  lwz r10, 4(r14)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F546A8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F546AC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F546B0: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F546B4: 4198FFDC  blt cr6, 0x82f54690
	if ctx.cr[6].lt {
	pc = 0x82F54690; continue 'dispatch;
	}
	// 82F546B8: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F546BC: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F546C0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F546C4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F546C8: 40980024  bge cr6, 0x82f546ec
	if !ctx.cr[6].lt {
	pc = 0x82F546EC; continue 'dispatch;
	}
	// 82F546CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F546D0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F546D4: 41980008  blt cr6, 0x82f546dc
	if ctx.cr[6].lt {
	pc = 0x82F546DC; continue 'dispatch;
	}
	// 82F546D8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F546DC: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82F546E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F546E4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F546E8: 4BF52111  bl 0x82ea67f8
	ctx.lr = 0x82F546EC;
	sub_82EA67F8(ctx, base);
	// 82F546EC: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F546F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F546F4: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F546F8: 8174000C  lwz r11, 0xc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F546FC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54700: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F54704: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54708: 4BF56051  bl 0x82eaa758
	ctx.lr = 0x82F5470C;
	sub_82EAA758(ctx, base);
	// 82F5470C: 81510004  lwz r10, 4(r17)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54710: 7FB2EB78  mr r18, r29
	ctx.r[18].u64 = ctx.r[29].u64;
	// 82F54714: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54718: 40990240  ble cr6, 0x82f54958
	if !ctx.cr[6].gt {
	pc = 0x82F54958; continue 'dispatch;
	}
	// 82F5471C: 89E103A7  lbz r15, 0x3a7(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(935 as u32) ) } as u64;
	// 82F54720: 7FB5EB78  mr r21, r29
	ctx.r[21].u64 = ctx.r[29].u64;
	// 82F54724: 8A6103A6  lbz r19, 0x3a6(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(934 as u32) ) } as u64;
	// 82F54728: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82F5472C: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 82F54730: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82F54734: 38C10088  addi r6, r1, 0x88
	ctx.r[6].s64 = ctx.r[1].s64 + 136;
	// 82F54738: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 82F5473C: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 82F54740: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82F54744: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82F54748: 92C10070  stw r22, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[22].u32 ) };
	// 82F5474C: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54750: 7D755A14  add r11, r21, r11
	ctx.r[11].u64 = ctx.r[21].u64 + ctx.r[11].u64;
	// 82F54754: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54758: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5475C: 48005E85  bl 0x82f5a5e0
	ctx.lr = 0x82F54760;
	sub_82F5A5E0(ctx, base);
	// 82F54760: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82F54764: 419A03F4  beq cr6, 0x82f54b58
	if ctx.cr[6].eq {
	pc = 0x82F54B58; continue 'dispatch;
	}
	// 82F54768: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82F5476C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82F54770: 48005611  bl 0x82f59d80
	ctx.lr = 0x82F54774;
	sub_82F59D80(ctx, base);
	// 82F54774: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F54778: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82F5477C: 419A0534  beq cr6, 0x82f54cb0
	if ctx.cr[6].eq {
	pc = 0x82F54CB0; continue 'dispatch;
	}
	// 82F54780: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F54784: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54788: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F5478C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F54790: 409A0010  bne cr6, 0x82f547a0
	if !ctx.cr[6].eq {
	pc = 0x82F547A0; continue 'dispatch;
	}
	// 82F54794: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F54798: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F5479C: 4BF520E5  bl 0x82ea6880
	ctx.lr = 0x82F547A0;
	sub_82EA6880(ctx, base);
	// 82F547A0: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F547A4: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 82F547A8: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F547AC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F547B0: 7F49512E  stwx r26, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 82F547B4: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F547B8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F547BC: 91170004  stw r8, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F547C0: 80E1006C  lwz r7, 0x6c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82F547C4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F547C8: 40990120  ble cr6, 0x82f548e8
	if !ctx.cr[6].gt {
	pc = 0x82F548E8; continue 'dispatch;
	}
	// 82F547CC: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82F547D0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F547D4: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F547D8: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F547DC: 4BF573A5  bl 0x82eabb80
	ctx.lr = 0x82F547E0;
	sub_82EABB80(ctx, base);
	// 82F547E0: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82F547E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F547E8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F547EC: 40990008  ble cr6, 0x82f547f4
	if !ctx.cr[6].gt {
	pc = 0x82F547F4; continue 'dispatch;
	}
	// 82F547F0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F547F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82F547F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F547FC: 419A060C  beq cr6, 0x82f54e08
	if ctx.cr[6].eq {
	pc = 0x82F54E08; continue 'dispatch;
	}
	// 82F54800: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82F54804: 81010098  lwz r8, 0x98(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82F54808: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F5480C: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 82F54810: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54814: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F54818: 54E500BE  clrlwi r5, r7, 2
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82F5481C: 54C4103A  slwi r4, r6, 2
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F54820: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82F54824: 7FE4402E  lwzx r31, r4, r8
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F54828: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F5482C: 7C7F5214  add r3, r31, r10
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82F54830: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54834: 7F6A4A14  add r27, r10, r9
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F54838: 409A0014  bne cr6, 0x82f5484c
	if !ctx.cr[6].eq {
	pc = 0x82F5484C; continue 'dispatch;
	}
	// 82F5483C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82F54840: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F54844: 4BF5203D  bl 0x82ea6880
	ctx.lr = 0x82F54848;
	sub_82EA6880(ctx, base);
	// 82F54848: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F5484C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54850: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F54854: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F54858: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F5485C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82F54860: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F54864: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54868: 7FCB4A14  add r30, r11, r9
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F5486C: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F54870: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54874: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F54878: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82F5487C: 7CCA592E  stwx r6, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	// 82F54880: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54884: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54888: 80850010  lwz r4, 0x10(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F5488C: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82F54890: 4E800421  bctrl
	ctx.lr = 0x82F54894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54894: 2F030066  cmpwi cr6, r3, 0x66
	ctx.cr[6].compare_i32(ctx.r[3].s32, 102, &mut ctx.xer);
	// 82F54898: 409A0708  bne cr6, 0x82f54fa0
	if !ctx.cr[6].eq {
	pc = 0x82F54FA0; continue 'dispatch;
	}
	// 82F5489C: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F548A0: 7E6A0774  extsb r10, r19
	ctx.r[10].s64 = ctx.r[19].s8 as i64;
	// 82F548A4: 931E0004  stw r24, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82F548A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F548AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F548B0: 419A0020  beq cr6, 0x82f548d0
	if ctx.cr[6].eq {
	pc = 0x82F548D0; continue 'dispatch;
	}
	// 82F548B4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F548B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F548BC: 409A0014  bne cr6, 0x82f548d0
	if !ctx.cr[6].eq {
	pc = 0x82F548D0; continue 'dispatch;
	}
	// 82F548C0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F548C4: 7C7C582E  lwzx r3, r28, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F548C8: 48004999  bl 0x82f59260
	ctx.lr = 0x82F548CC;
	sub_82F59260(ctx, base);
	// 82F548CC: 907B0008  stw r3, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82F548D0: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82F548D4: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82F548D8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82F548DC: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F548E0: 4198FEF0  blt cr6, 0x82f547d0
	if ctx.cr[6].lt {
	pc = 0x82F547D0; continue 'dispatch;
	}
	// 82F548E4: 836103BC  lwz r27, 0x3bc(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(956 as u32) ) } as u64;
	// 82F548E8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F548EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F548F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F548F4: 409A0020  bne cr6, 0x82f54914
	if !ctx.cr[6].eq {
	pc = 0x82F54914; continue 'dispatch;
	}
	// 82F548F8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F548FC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54900: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54904: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F54908: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F5490C: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F54910: 4BF4BEA1  bl 0x82ea07b0
	ctx.lr = 0x82F54914;
	sub_82EA07B0(ctx, base);
	// 82F54914: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F54918: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5491C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54920: 409A0020  bne cr6, 0x82f54940
	if !ctx.cr[6].eq {
	pc = 0x82F54940; continue 'dispatch;
	}
	// 82F54924: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F54928: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F5492C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54930: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F54934: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54938: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F5493C: 4BF4BE75  bl 0x82ea07b0
	ctx.lr = 0x82F54940;
	sub_82EA07B0(ctx, base);
	// 82F54940: 81710004  lwz r11, 4(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54944: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82F54948: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5494C: 3AB50008  addi r21, r21, 8
	ctx.r[21].s64 = ctx.r[21].s64 + 8;
	// 82F54950: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54954: 4198FDD4  blt cr6, 0x82f54728
	if ctx.cr[6].lt {
	pc = 0x82F54728; continue 'dispatch;
	}
	// 82F54958: 8174000C  lwz r11, 0xc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F5495C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82F54960: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82F54964: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F54968: 40990044  ble cr6, 0x82f549ac
	if !ctx.cr[6].gt {
	pc = 0x82F549AC; continue 'dispatch;
	}
	// 82F5496C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F54970: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F54974: 81190000  lwz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54978: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F5497C: 7FEB412E  stwx r31, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[31].u32) };
	// 82F54980: 81190000  lwz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54984: 7CEB4214  add r7, r11, r8
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F54988: 80C10078  lwz r6, 0x78(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F5498C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82F54990: 7D0A302E  lwzx r8, r10, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82F54994: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F54998: 93A70004  stw r29, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82F5499C: 80B4000C  lwz r5, 0xc(r20)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F549A0: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82F549A4: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82F549A8: 4198FFCC  blt cr6, 0x82f54974
	if ctx.cr[6].lt {
	pc = 0x82F54974; continue 'dispatch;
	}
	// 82F549AC: 81700008  lwz r11, 8(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F549B0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F549B4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F549B8: 40980024  bge cr6, 0x82f549dc
	if !ctx.cr[6].lt {
	pc = 0x82F549DC; continue 'dispatch;
	}
	// 82F549BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F549C0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F549C4: 41980008  blt cr6, 0x82f549cc
	if ctx.cr[6].lt {
	pc = 0x82F549CC; continue 'dispatch;
	}
	// 82F549C8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F549CC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F549D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F549D4: 7E038378  mr r3, r16
	ctx.r[3].u64 = ctx.r[16].u64;
	// 82F549D8: 4BF51E21  bl 0x82ea67f8
	ctx.lr = 0x82F549DC;
	sub_82EA67F8(ctx, base);
	// 82F549DC: 93F00004  stw r31, 4(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F549E0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82F549E4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F549E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F549EC: 40990070  ble cr6, 0x82f54a5c
	if !ctx.cr[6].gt {
	pc = 0x82F54A5C; continue 'dispatch;
	}
	// 82F549F0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F549F4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F549F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F549FC: 80F90000  lwz r7, 0(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54A00: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54A04: 80D00000  lwz r6, 0(r16)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54A08: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F54A0C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82F54A10: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F54A14: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54A18: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82F54A1C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F54A20: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82F54A24: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54A28: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54A2C: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82F54A30: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F54A34: 7CA73214  add r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82F54A38: 7C87312E  stwx r4, r7, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), ctx.r[4].u32) };
	// 82F54A3C: 80880004  lwz r4, 4(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54A40: 90850004  stw r4, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F54A44: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54A48: 38680001  addi r3, r8, 1
	ctx.r[3].s64 = ctx.r[8].s64 + 1;
	// 82F54A4C: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F54A50: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F54A54: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54A58: 4198FF9C  blt cr6, 0x82f549f4
	if ctx.cr[6].lt {
	pc = 0x82F549F4; continue 'dispatch;
	}
	// 82F54A5C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82F54A60: 419A0088  beq cr6, 0x82f54ae8
	if ctx.cr[6].eq {
	pc = 0x82F54AE8; continue 'dispatch;
	}
	// 82F54A64: 8174000C  lwz r11, 0xc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54A68: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82F54A6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F54A70: 40990078  ble cr6, 0x82f54ae8
	if !ctx.cr[6].gt {
	pc = 0x82F54AE8; continue 'dispatch;
	}
	// 82F54A74: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82F54A78: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54A7C: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82F54A80: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54A84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54A88: 409A0048  bne cr6, 0x82f54ad0
	if !ctx.cr[6].eq {
	pc = 0x82F54AD0; continue 'dispatch;
	}
	// 82F54A8C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F54A90: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54A94: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54A98: 83CE0000  lwz r30, 0(r14)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54A9C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F54AA0: 409A0010  bne cr6, 0x82f54ab0
	if !ctx.cr[6].eq {
	pc = 0x82F54AB0; continue 'dispatch;
	}
	// 82F54AA4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F54AA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F54AAC: 4BF51DD5  bl 0x82ea6880
	ctx.lr = 0x82F54AB0;
	sub_82EA6880(ctx, base);
	// 82F54AB0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54AB4: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F54AB8: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54ABC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F54AC0: 7D48492E  stwx r10, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82F54AC4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54AC8: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F54ACC: 90FB0004  stw r7, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F54AD0: 8174000C  lwz r11, 0xc(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54AD4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F54AD8: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82F54ADC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F54AE0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F54AE4: 4198FF94  blt cr6, 0x82f54a78
	if ctx.cr[6].lt {
	pc = 0x82F54A78; continue 'dispatch;
	}
	// 82F54AE8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F54AEC: 4BF56F85  bl 0x82eaba70
	ctx.lr = 0x82F54AF0;
	sub_82EABA70(ctx, base);
	// 82F54AF0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F54AF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54AF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54AFC: 409A001C  bne cr6, 0x82f54b18
	if !ctx.cr[6].eq {
	pc = 0x82F54B18; continue 'dispatch;
	}
	// 82F54B00: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54B04: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54B08: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54B0C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54B10: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54B14: 4BF4BC9D  bl 0x82ea07b0
	ctx.lr = 0x82F54B18;
	sub_82EA07B0(ctx, base);
	// 82F54B18: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F54B1C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54B20: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54B24: 409A0028  bne cr6, 0x82f54b4c
	if !ctx.cr[6].eq {
	pc = 0x82F54B4C; continue 'dispatch;
	}
	// 82F54B28: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54B2C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F54B30: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F54B34: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54B38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54B3C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54B40: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54B44: 7C69C02E  lwzx r3, r9, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54B48: 4BF4BC69  bl 0x82ea07b0
	ctx.lr = 0x82F54B4C;
	sub_82EA07B0(ctx, base);
	// 82F54B4C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F54B50: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82F54B54: 4825362C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82F54B58: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82F54B5C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82F54B60: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82F54B64: 4BF5182D  bl 0x82ea6390
	ctx.lr = 0x82F54B68;
	sub_82EA6390(ctx, base);
	// 82F54B68: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F54B6C: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82F54B70: 388B1558  addi r4, r11, 0x1558
	ctx.r[4].s64 = ctx.r[11].s64 + 5464;
	// 82F54B74: 4BF50C65  bl 0x82ea57d8
	ctx.lr = 0x82F54B78;
	sub_82EA57D8(ctx, base);
	// 82F54B78: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82F54B7C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F54B80: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82F54B84: 39000044  li r8, 0x44
	ctx.r[8].s64 = 68;
	// 82F54B88: 38E91528  addi r7, r9, 0x1528
	ctx.r[7].s64 = ctx.r[9].s64 + 5416;
	// 82F54B8C: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82F54B90: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82F54B94: 60A5AA88  ori r5, r5, 0xaa88
	ctx.r[5].u64 = ctx.r[5].u64 | 43656;
	// 82F54B98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54B9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54BA0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54BA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54BA8: 4E800421  bctrl
	ctx.lr = 0x82F54BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54BAC: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82F54BB0: 4BF51269  bl 0x82ea5e18
	ctx.lr = 0x82F54BB4;
	sub_82EA5E18(ctx, base);
	// 82F54BB4: A1340004  lhz r9, 4(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54BB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F54BBC: 419A0034  beq cr6, 0x82f54bf0
	if ctx.cr[6].eq {
	pc = 0x82F54BF0; continue 'dispatch;
	}
	// 82F54BC0: A1740006  lhz r11, 6(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F54BC4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F54BC8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F54BCC: B1340006  sth r9, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F54BD0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F54BD4: 409A001C  bne cr6, 0x82f54bf0
	if !ctx.cr[6].eq {
	pc = 0x82F54BF0; continue 'dispatch;
	}
	// 82F54BD8: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54BDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54BE0: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F54BE4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54BE8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54BEC: 4E800421  bctrl
	ctx.lr = 0x82F54BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54BF0: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F54BF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54BF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54BFC: 409A001C  bne cr6, 0x82f54c18
	if !ctx.cr[6].eq {
	pc = 0x82F54C18; continue 'dispatch;
	}
	// 82F54C00: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54C04: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F54C08: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54C0C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54C10: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54C14: 4BF4BB9D  bl 0x82ea07b0
	ctx.lr = 0x82F54C18;
	sub_82EA07B0(ctx, base);
	// 82F54C18: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F54C1C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54C20: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54C24: 409A001C  bne cr6, 0x82f54c40
	if !ctx.cr[6].eq {
	pc = 0x82F54C40; continue 'dispatch;
	}
	// 82F54C28: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54C2C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F54C30: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54C34: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54C38: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54C3C: 4BF4BB75  bl 0x82ea07b0
	ctx.lr = 0x82F54C40;
	sub_82EA07B0(ctx, base);
	// 82F54C40: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F54C44: 4BF56E2D  bl 0x82eaba70
	ctx.lr = 0x82F54C48;
	sub_82EABA70(ctx, base);
	// 82F54C48: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F54C4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54C50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54C54: 409A001C  bne cr6, 0x82f54c70
	if !ctx.cr[6].eq {
	pc = 0x82F54C70; continue 'dispatch;
	}
	// 82F54C58: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54C5C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54C60: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54C64: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54C68: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54C6C: 4BF4BB45  bl 0x82ea07b0
	ctx.lr = 0x82F54C70;
	sub_82EA07B0(ctx, base);
	// 82F54C70: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F54C74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54C78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54C7C: 409A0028  bne cr6, 0x82f54ca4
	if !ctx.cr[6].eq {
	pc = 0x82F54CA4; continue 'dispatch;
	}
	// 82F54C80: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54C84: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F54C88: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F54C8C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54C90: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54C94: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54C98: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54C9C: 7C69C02E  lwzx r3, r9, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54CA0: 4BF4BB11  bl 0x82ea07b0
	ctx.lr = 0x82F54CA4;
	sub_82EA07B0(ctx, base);
	// 82F54CA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F54CA8: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82F54CAC: 482534D4  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82F54CB0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82F54CB4: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82F54CB8: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82F54CBC: 4BF516D5  bl 0x82ea6390
	ctx.lr = 0x82F54CC0;
	sub_82EA6390(ctx, base);
	// 82F54CC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F54CC4: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82F54CC8: 388B150C  addi r4, r11, 0x150c
	ctx.r[4].s64 = ctx.r[11].s64 + 5388;
	// 82F54CCC: 4BF50B0D  bl 0x82ea57d8
	ctx.lr = 0x82F54CD0;
	sub_82EA57D8(ctx, base);
	// 82F54CD0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82F54CD4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F54CD8: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82F54CDC: 3900004E  li r8, 0x4e
	ctx.r[8].s64 = 78;
	// 82F54CE0: 38E91528  addi r7, r9, 0x1528
	ctx.r[7].s64 = ctx.r[9].s64 + 5416;
	// 82F54CE4: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82F54CE8: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82F54CEC: 60A5DDAA  ori r5, r5, 0xddaa
	ctx.r[5].u64 = ctx.r[5].u64 | 56746;
	// 82F54CF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54CF8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54CFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54D00: 4E800421  bctrl
	ctx.lr = 0x82F54D04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54D04: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82F54D08: 4BF51111  bl 0x82ea5e18
	ctx.lr = 0x82F54D0C;
	sub_82EA5E18(ctx, base);
	// 82F54D0C: A1340004  lhz r9, 4(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54D10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F54D14: 419A0034  beq cr6, 0x82f54d48
	if ctx.cr[6].eq {
	pc = 0x82F54D48; continue 'dispatch;
	}
	// 82F54D18: A1740006  lhz r11, 6(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F54D1C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F54D20: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F54D24: B1340006  sth r9, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F54D28: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F54D2C: 409A001C  bne cr6, 0x82f54d48
	if !ctx.cr[6].eq {
	pc = 0x82F54D48; continue 'dispatch;
	}
	// 82F54D30: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54D34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54D38: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F54D3C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54D40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54D44: 4E800421  bctrl
	ctx.lr = 0x82F54D48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54D48: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F54D4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54D50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54D54: 409A001C  bne cr6, 0x82f54d70
	if !ctx.cr[6].eq {
	pc = 0x82F54D70; continue 'dispatch;
	}
	// 82F54D58: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54D5C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F54D60: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54D64: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54D68: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54D6C: 4BF4BA45  bl 0x82ea07b0
	ctx.lr = 0x82F54D70;
	sub_82EA07B0(ctx, base);
	// 82F54D70: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F54D74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54D78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54D7C: 409A001C  bne cr6, 0x82f54d98
	if !ctx.cr[6].eq {
	pc = 0x82F54D98; continue 'dispatch;
	}
	// 82F54D80: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54D84: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F54D88: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54D8C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54D90: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54D94: 4BF4BA1D  bl 0x82ea07b0
	ctx.lr = 0x82F54D98;
	sub_82EA07B0(ctx, base);
	// 82F54D98: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F54D9C: 4BF56CD5  bl 0x82eaba70
	ctx.lr = 0x82F54DA0;
	sub_82EABA70(ctx, base);
	// 82F54DA0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F54DA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54DA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54DAC: 409A001C  bne cr6, 0x82f54dc8
	if !ctx.cr[6].eq {
	pc = 0x82F54DC8; continue 'dispatch;
	}
	// 82F54DB0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54DB4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54DB8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54DBC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54DC0: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54DC4: 4BF4B9ED  bl 0x82ea07b0
	ctx.lr = 0x82F54DC8;
	sub_82EA07B0(ctx, base);
	// 82F54DC8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F54DCC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54DD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54DD4: 409AFED0  bne cr6, 0x82f54ca4
	if !ctx.cr[6].eq {
	pc = 0x82F54CA4; continue 'dispatch;
	}
	// 82F54DD8: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54DDC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F54DE0: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F54DE4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54DE8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54DEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54DF0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54DF4: 7C69C02E  lwzx r3, r9, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F54DF8: 4BF4B9B9  bl 0x82ea07b0
	ctx.lr = 0x82F54DFC;
	sub_82EA07B0(ctx, base);
	// 82F54DFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F54E00: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82F54E04: 4825337C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82F54E08: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82F54E0C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82F54E10: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82F54E14: 4BF5157D  bl 0x82ea6390
	ctx.lr = 0x82F54E18;
	sub_82EA6390(ctx, base);
	// 82F54E18: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F54E1C: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82F54E20: 388B14FC  addi r4, r11, 0x14fc
	ctx.r[4].s64 = ctx.r[11].s64 + 5372;
	// 82F54E24: 4BF509B5  bl 0x82ea57d8
	ctx.lr = 0x82F54E28;
	sub_82EA57D8(ctx, base);
	// 82F54E28: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82F54E2C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F54E30: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82F54E34: 3900005D  li r8, 0x5d
	ctx.r[8].s64 = 93;
	// 82F54E38: 38E91528  addi r7, r9, 0x1528
	ctx.r[7].s64 = ctx.r[9].s64 + 5416;
	// 82F54E3C: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82F54E40: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82F54E44: 60A599DD  ori r5, r5, 0x99dd
	ctx.r[5].u64 = ctx.r[5].u64 | 39389;
	// 82F54E48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54E4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54E50: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54E54: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54E58: 4E800421  bctrl
	ctx.lr = 0x82F54E5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54E5C: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82F54E60: 4BF50FB9  bl 0x82ea5e18
	ctx.lr = 0x82F54E64;
	sub_82EA5E18(ctx, base);
	// 82F54E64: A13A0004  lhz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54E68: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F54E6C: 419A0034  beq cr6, 0x82f54ea0
	if ctx.cr[6].eq {
	pc = 0x82F54EA0; continue 'dispatch;
	}
	// 82F54E70: A17A0006  lhz r11, 6(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F54E74: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F54E78: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F54E7C: B13A0006  sth r9, 6(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F54E80: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F54E84: 409A001C  bne cr6, 0x82f54ea0
	if !ctx.cr[6].eq {
	pc = 0x82F54EA0; continue 'dispatch;
	}
	// 82F54E88: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54E8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54E90: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F54E94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54E98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54E9C: 4E800421  bctrl
	ctx.lr = 0x82F54EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54EA0: A1740004  lhz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F54EA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F54EA8: 419A0034  beq cr6, 0x82f54edc
	if ctx.cr[6].eq {
	pc = 0x82F54EDC; continue 'dispatch;
	}
	// 82F54EAC: A1740006  lhz r11, 6(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F54EB0: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F54EB4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F54EB8: B1340006  sth r9, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F54EBC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F54EC0: 409A001C  bne cr6, 0x82f54edc
	if !ctx.cr[6].eq {
	pc = 0x82F54EDC; continue 'dispatch;
	}
	// 82F54EC4: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54EC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54ECC: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F54ED0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54ED4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54ED8: 4E800421  bctrl
	ctx.lr = 0x82F54EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54EDC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F54EE0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F54EE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54EE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54EEC: 409A001C  bne cr6, 0x82f54f08
	if !ctx.cr[6].eq {
	pc = 0x82F54F08; continue 'dispatch;
	}
	// 82F54EF0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54EF4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F54EF8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54EFC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54F00: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F54F04: 4BF4B8AD  bl 0x82ea07b0
	ctx.lr = 0x82F54F08;
	sub_82EA07B0(ctx, base);
	// 82F54F08: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F54F0C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54F10: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54F14: 409A001C  bne cr6, 0x82f54f30
	if !ctx.cr[6].eq {
	pc = 0x82F54F30; continue 'dispatch;
	}
	// 82F54F18: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54F1C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F54F20: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54F24: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54F28: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F54F2C: 4BF4B885  bl 0x82ea07b0
	ctx.lr = 0x82F54F30;
	sub_82EA07B0(ctx, base);
	// 82F54F30: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F54F34: 4BF56B3D  bl 0x82eaba70
	ctx.lr = 0x82F54F38;
	sub_82EABA70(ctx, base);
	// 82F54F38: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F54F3C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54F40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54F44: 409A001C  bne cr6, 0x82f54f60
	if !ctx.cr[6].eq {
	pc = 0x82F54F60; continue 'dispatch;
	}
	// 82F54F48: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F54F4C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F54F50: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54F54: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54F58: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F54F5C: 4BF4B855  bl 0x82ea07b0
	ctx.lr = 0x82F54F60;
	sub_82EA07B0(ctx, base);
	// 82F54F60: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F54F64: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F54F68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F54F6C: 409A0028  bne cr6, 0x82f54f94
	if !ctx.cr[6].eq {
	pc = 0x82F54F94; continue 'dispatch;
	}
	// 82F54F70: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F54F74: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F54F78: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F54F7C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F54F80: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F54F84: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F54F88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F54F8C: 7C69F82E  lwzx r3, r9, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F54F90: 4BF4B821  bl 0x82ea07b0
	ctx.lr = 0x82F54F94;
	sub_82EA07B0(ctx, base);
	// 82F54F94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F54F98: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82F54F9C: 482531E4  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 82F54FA0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82F54FA4: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82F54FA8: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82F54FAC: 4BF513E5  bl 0x82ea6390
	ctx.lr = 0x82F54FB0;
	sub_82EA6390(ctx, base);
	// 82F54FB0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F54FB4: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82F54FB8: 388B14D8  addi r4, r11, 0x14d8
	ctx.r[4].s64 = ctx.r[11].s64 + 5336;
	// 82F54FBC: 4BF5081D  bl 0x82ea57d8
	ctx.lr = 0x82F54FC0;
	sub_82EA57D8(ctx, base);
	// 82F54FC0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82F54FC4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F54FC8: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82F54FCC: 3900006C  li r8, 0x6c
	ctx.r[8].s64 = 108;
	// 82F54FD0: 38E91528  addi r7, r9, 0x1528
	ctx.r[7].s64 = ctx.r[9].s64 + 5416;
	// 82F54FD4: 806A6E5C  lwz r3, 0x6e5c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28252 as u32) ) } as u64;
	// 82F54FD8: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82F54FDC: 60A59D6D  ori r5, r5, 0x9d6d
	ctx.r[5].u64 = ctx.r[5].u64 | 40301;
	// 82F54FE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F54FE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F54FE8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F54FEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F54FF0: 4E800421  bctrl
	ctx.lr = 0x82F54FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F54FF4: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82F54FF8: 4BF50E21  bl 0x82ea5e18
	ctx.lr = 0x82F54FFC;
	sub_82EA5E18(ctx, base);
	// 82F54FFC: A13A0004  lhz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55000: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F55004: 419A0034  beq cr6, 0x82f55038
	if ctx.cr[6].eq {
	pc = 0x82F55038; continue 'dispatch;
	}
	// 82F55008: A17A0006  lhz r11, 6(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F5500C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F55010: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F55014: B13A0006  sth r9, 6(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F55018: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F5501C: 409A001C  bne cr6, 0x82f55038
	if !ctx.cr[6].eq {
	pc = 0x82F55038; continue 'dispatch;
	}
	// 82F55020: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F55028: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F5502C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55030: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F55034: 4E800421  bctrl
	ctx.lr = 0x82F55038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F55038: A1740004  lhz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5503C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F55040: 419A0034  beq cr6, 0x82f55074
	if ctx.cr[6].eq {
	pc = 0x82F55074; continue 'dispatch;
	}
	// 82F55044: A1740006  lhz r11, 6(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F55048: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F5504C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F55050: B1340006  sth r9, 6(r20)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[20].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F55054: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55058: 409A001C  bne cr6, 0x82f55074
	if !ctx.cr[6].eq {
	pc = 0x82F55074; continue 'dispatch;
	}
	// 82F5505C: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55060: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F55064: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82F55068: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5506C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F55070: 4E800421  bctrl
	ctx.lr = 0x82F55074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F55074: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F55078: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F5507C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55080: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55084: 409A001C  bne cr6, 0x82f550a0
	if !ctx.cr[6].eq {
	pc = 0x82F550A0; continue 'dispatch;
	}
	// 82F55088: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F5508C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F55090: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F55094: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F55098: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F5509C: 4BF4B715  bl 0x82ea07b0
	ctx.lr = 0x82F550A0;
	sub_82EA07B0(ctx, base);
	// 82F550A0: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82F550A4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F550A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F550AC: 409A001C  bne cr6, 0x82f550c8
	if !ctx.cr[6].eq {
	pc = 0x82F550C8; continue 'dispatch;
	}
	// 82F550B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F550B4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82F550B8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F550BC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F550C0: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F550C4: 4BF4B6ED  bl 0x82ea07b0
	ctx.lr = 0x82F550C8;
	sub_82EA07B0(ctx, base);
	// 82F550C8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82F550CC: 4BF569A5  bl 0x82eaba70
	ctx.lr = 0x82F550D0;
	sub_82EABA70(ctx, base);
	// 82F550D0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82F550D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F550D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F550DC: 409A001C  bne cr6, 0x82f550f8
	if !ctx.cr[6].eq {
	pc = 0x82F550F8; continue 'dispatch;
	}
	// 82F550E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F550E4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82F550E8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F550EC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F550F0: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F550F4: 4BF4B6BD  bl 0x82ea07b0
	ctx.lr = 0x82F550F8;
	sub_82EA07B0(ctx, base);
	// 82F550F8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F550FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55100: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55104: 409AFE90  bne cr6, 0x82f54f94
	if !ctx.cr[6].eq {
	pc = 0x82F54F94; continue 'dispatch;
	}
	// 82F55108: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F5510C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F55110: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F55114: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F55118: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F5511C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F55120: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F55124: 7C69F82E  lwzx r3, r9, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F55128: 4BF4B689  bl 0x82ea07b0
	ctx.lr = 0x82F5512C;
	sub_82EA07B0(ctx, base);
	// 82F5512C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F55130: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82F55134: 4825304C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55138 size=412
    let mut pc: u32 = 0x82F55138;
    'dispatch: loop {
        match pc {
            0x82F55138 => {
    //   block [0x82F55138..0x82F552D4)
	// 82F55138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5513C: 48253031  bl 0x831a816c
	ctx.lr = 0x82F55140;
	sub_831A8130(ctx, base);
	// 82F55140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55148: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F5514C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F55150: 392B06FC  addi r9, r11, 0x6fc
	ctx.r[9].s64 = ctx.r[11].s64 + 1788;
	// 82F55154: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F55158: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F5515C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55160: 40990068  ble cr6, 0x82f551c8
	if !ctx.cr[6].gt {
	pc = 0x82F551C8; continue 'dispatch;
	}
	// 82F55164: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F55168: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5516C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F55170: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F55174: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F55178: 419A003C  beq cr6, 0x82f551b4
	if ctx.cr[6].eq {
	pc = 0x82F551B4; continue 'dispatch;
	}
	// 82F5517C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F55184: 419A0030  beq cr6, 0x82f551b4
	if ctx.cr[6].eq {
	pc = 0x82F551B4; continue 'dispatch;
	}
	// 82F55188: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F5518C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F55190: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F55194: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F55198: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F5519C: 409A0018  bne cr6, 0x82f551b4
	if !ctx.cr[6].eq {
	pc = 0x82F551B4; continue 'dispatch;
	}
	// 82F551A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F551A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F551A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F551AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F551B0: 4E800421  bctrl
	ctx.lr = 0x82F551B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F551B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F551B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F551BC: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82F551C0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F551C4: 4198FFA4  blt cr6, 0x82f55168
	if ctx.cr[6].lt {
	pc = 0x82F55168; continue 'dispatch;
	}
	// 82F551C8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F551CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F551D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F551D4: 4099005C  ble cr6, 0x82f55230
	if !ctx.cr[6].gt {
	pc = 0x82F55230; continue 'dispatch;
	}
	// 82F551D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F551DC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F551E0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F551E4: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F551E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F551EC: 419A0030  beq cr6, 0x82f5521c
	if ctx.cr[6].eq {
	pc = 0x82F5521C; continue 'dispatch;
	}
	// 82F551F0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F551F4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F551F8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F551FC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F55200: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55204: 409A0018  bne cr6, 0x82f5521c
	if !ctx.cr[6].eq {
	pc = 0x82F5521C; continue 'dispatch;
	}
	// 82F55208: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5520C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F55210: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55214: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F55218: 4E800421  bctrl
	ctx.lr = 0x82F5521C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F5521C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F55220: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F55224: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F55228: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F5522C: 4198FFB0  blt cr6, 0x82f551dc
	if ctx.cr[6].lt {
	pc = 0x82F551DC; continue 'dispatch;
	}
	// 82F55230: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F55234: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55238: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F5523C: 409A0020  bne cr6, 0x82f5525c
	if !ctx.cr[6].eq {
	pc = 0x82F5525C; continue 'dispatch;
	}
	// 82F55240: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55244: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F55248: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F5524C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F55250: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F55254: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F55258: 4BF4B559  bl 0x82ea07b0
	ctx.lr = 0x82F5525C;
	sub_82EA07B0(ctx, base);
	// 82F5525C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F55260: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55264: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55268: 409A0020  bne cr6, 0x82f55288
	if !ctx.cr[6].eq {
	pc = 0x82F55288; continue 'dispatch;
	}
	// 82F5526C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55270: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F55274: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F55278: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F5527C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F55280: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F55284: 4BF4B52D  bl 0x82ea07b0
	ctx.lr = 0x82F55288;
	sub_82EA07B0(ctx, base);
	// 82F55288: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F5528C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55290: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55294: 409A002C  bne cr6, 0x82f552c0
	if !ctx.cr[6].eq {
	pc = 0x82F552C0; continue 'dispatch;
	}
	// 82F55298: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F5529C: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F552A0: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82F552A4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F552A8: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82F552AC: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F552B0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F552B4: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F552B8: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F552BC: 4BF4B4F5  bl 0x82ea07b0
	ctx.lr = 0x82F552C0;
	sub_82EA07B0(ctx, base);
	// 82F552C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F552C4: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F552C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F552CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F552D0: 48252EEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F552D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F552D8 size=236
    let mut pc: u32 = 0x82F552D8;
    'dispatch: loop {
        match pc {
            0x82F552D8 => {
    //   block [0x82F552D8..0x82F553C4)
	// 82F552D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F552DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F552E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F552E4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82F552E8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82F552EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F552F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F552F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F552F8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82F552FC: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F55300: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82F55304: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82F55308: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82F5530C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F55310: 4BFFF1A9  bl 0x82f544b8
	ctx.lr = 0x82F55314;
	sub_82F544B8(ctx, base);
	// 82F55314: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F55318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F5531C: 419A0060  beq cr6, 0x82f5537c
	if ctx.cr[6].eq {
	pc = 0x82F5537C; continue 'dispatch;
	}
	// 82F55320: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82F55324: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82F55328: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82F5532C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82F55330: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82F55334: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82F55338: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82F5533C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F55340: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F55344: ED606024  fdivs f11, f0, f12
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 82F55348: EC0B07F2  fmuls f0, f11, f31
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[31].f64) as f32) as f64);
	// 82F5534C: EDAB07B2  fmuls f13, f11, f30
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[30].f64) as f32) as f64);
	// 82F55350: 4099002C  ble cr6, 0x82f5537c
	if !ctx.cr[6].gt {
	pc = 0x82F5537C; continue 'dispatch;
	}
	// 82F55354: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F55358: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F5535C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F55360: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F55364: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F55368: D008000C  stfs f0, 0xc(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82F5536C: D1A80010  stfs f13, 0x10(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F55370: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F55374: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82F55378: 4198FFE0  blt cr6, 0x82f55358
	if ctx.cr[6].lt {
	pc = 0x82F55358; continue 'dispatch;
	}
	// 82F5537C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F55380: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55384: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55388: 409A0020  bne cr6, 0x82f553a8
	if !ctx.cr[6].eq {
	pc = 0x82F553A8; continue 'dispatch;
	}
	// 82F5538C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55390: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F55394: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F55398: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F5539C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F553A0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F553A4: 4BF4B40D  bl 0x82ea07b0
	ctx.lr = 0x82F553A8;
	sub_82EA07B0(ctx, base);
	// 82F553A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F553AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F553B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F553B4: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82F553B8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F553BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F553C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F553C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F553C8 size=12
    let mut pc: u32 = 0x82F553C8;
    'dispatch: loop {
        match pc {
            0x82F553C8 => {
    //   block [0x82F553C8..0x82F553D4)
	// 82F553C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F553CC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F553D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F553D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F553D8 size=12
    let mut pc: u32 = 0x82F553D8;
    'dispatch: loop {
        match pc {
            0x82F553D8 => {
    //   block [0x82F553D8..0x82F553E4)
	// 82F553D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F553DC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F553E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F553E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F553E8 size=12
    let mut pc: u32 = 0x82F553E8;
    'dispatch: loop {
        match pc {
            0x82F553E8 => {
    //   block [0x82F553E8..0x82F553F4)
	// 82F553E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F553EC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F553F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F553F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F553F8 size=12
    let mut pc: u32 = 0x82F553F8;
    'dispatch: loop {
        match pc {
            0x82F553F8 => {
    //   block [0x82F553F8..0x82F55404)
	// 82F553F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F553FC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F55400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F55408 size=4
    let mut pc: u32 = 0x82F55408;
    'dispatch: loop {
        match pc {
            0x82F55408 => {
    //   block [0x82F55408..0x82F5540C)
	// 82F55408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F55410 size=40
    let mut pc: u32 = 0x82F55410;
    'dispatch: loop {
        match pc {
            0x82F55410 => {
    //   block [0x82F55410..0x82F55438)
	// 82F55410: 89650010  lbz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F55414: 89460010  lbz r10, 0x10(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F55418: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82F5541C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82F55420: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82F55424: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82F55428: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F5542C: 4099000C  ble cr6, 0x82f55438
	if !ctx.cr[6].gt {
		sub_82F55438(ctx, base);
		return;
	}
	// 82F55430: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82F55434: 4800000C  b 0x82f55440
	sub_82F55438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F55438 size=112
    let mut pc: u32 = 0x82F55438;
    'dispatch: loop {
        match pc {
            0x82F55438 => {
    //   block [0x82F55438..0x82F554A8)
	// 82F55438: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82F5543C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F55440: 80E40030  lwz r7, 0x30(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F55444: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55448: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F5544C: 40990044  ble cr6, 0x82f55490
	if !ctx.cr[6].gt {
	pc = 0x82F55490; continue 'dispatch;
	}
	// 82F55450: 8124002C  lwz r9, 0x2c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F55454: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55458: 7F083040  cmplw cr6, r8, r6
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82F5545C: 409A0014  bne cr6, 0x82f55470
	if !ctx.cr[6].eq {
	pc = 0x82F55470; continue 'dispatch;
	}
	// 82F55460: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55464: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F55468: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F5546C: 419A0008  beq cr6, 0x82f55474
	if ctx.cr[6].eq {
	pc = 0x82F55474; continue 'dispatch;
	}
	// 82F55470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F55474: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82F55478: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F5547C: 409A0018  bne cr6, 0x82f55494
	if !ctx.cr[6].eq {
	pc = 0x82F55494; continue 'dispatch;
	}
	// 82F55480: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F55484: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82F55488: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82F5548C: 4198FFC8  blt cr6, 0x82f55454
	if ctx.cr[6].lt {
	pc = 0x82F55454; continue 'dispatch;
	}
	// 82F55490: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82F55494: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82F55498: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82F5549C: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82F554A0: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82F554A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F554A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F554A8 size=72
    let mut pc: u32 = 0x82F554A8;
    'dispatch: loop {
        match pc {
            0x82F554A8 => {
    //   block [0x82F554A8..0x82F554F0)
	// 82F554A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F554AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F554B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F554B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F554B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F554BC: 4099006C  ble cr6, 0x82f55528
	if !ctx.cr[6].gt {
		sub_82F554F0(ctx, base);
		return;
	}
	// 82F554C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F554C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F554C8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82F554CC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F554D0: 7F091840  cmplw cr6, r9, r3
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82F554D4: 419A001C  beq cr6, 0x82f554f0
	if ctx.cr[6].eq {
		sub_82F554F0(ctx, base);
		return;
	}
	// 82F554D8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F554DC: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82F554E0: 419A0010  beq cr6, 0x82f554f0
	if ctx.cr[6].eq {
		sub_82F554F0(ctx, base);
		return;
	}
	// 82F554E4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82F554E8: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F554EC: 48000030  b 0x82f5551c
	sub_82F554F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F554F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F554F0 size=76
    let mut pc: u32 = 0x82F554F0;
    'dispatch: loop {
        match pc {
            0x82F554F0 => {
    //   block [0x82F554F0..0x82F5553C)
	// 82F554F0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F554F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F554F8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82F554FC: 7CA85214  add r5, r8, r10
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82F55500: 55271838  slwi r7, r9, 3
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F55504: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82F55508: 7C875214  add r4, r7, r10
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82F5550C: 7D27502E  lwzx r9, r7, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F55510: 7D28512E  stwx r9, r8, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82F55514: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55518: 90E50004  stw r7, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F5551C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F55520: 7F065000  cmpw cr6, r6, r10
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F55524: 4198FFA0  blt cr6, 0x82f554c4
	if ctx.cr[6].lt {
		sub_82F554A8(ctx, base);
		return;
	}
	// 82F55528: 354BFFD0  addic. r10, r11, -0x30
	ctx.xer.ca = (ctx.r[11].u32 > (!(-48 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F5552C: 40820008  bne 0x82f55534
	if !ctx.cr[0].eq {
	pc = 0x82F55534; continue 'dispatch;
	}
	// 82F55530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55534: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F55538: 4BF84070  b 0x82ed95a8
	sub_82ED95A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F55540 size=8
    let mut pc: u32 = 0x82F55540;
    'dispatch: loop {
        match pc {
            0x82F55540 => {
    //   block [0x82F55540..0x82F55548)
	// 82F55540: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F55544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55548 size=152
    let mut pc: u32 = 0x82F55548;
    'dispatch: loop {
        match pc {
            0x82F55548 => {
    //   block [0x82F55548..0x82F555E0)
	// 82F55548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F55550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F55554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F5555C: 4BFC8465  bl 0x82f1d9c0
	ctx.lr = 0x82F55560;
	sub_82F1D9C0(ctx, base);
	// 82F55560: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F55564: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F55568: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F5556C: 388BBA8C  addi r4, r11, -0x4574
	ctx.r[4].s64 = ctx.r[11].s64 + -17780;
	// 82F55570: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F55574: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F55578: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82F5557C: 39690810  addi r11, r9, 0x810
	ctx.r[11].s64 = ctx.r[9].s64 + 2064;
	// 82F55580: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F55584: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82F55588: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F5558C: 386A07FC  addi r3, r10, 0x7fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2044;
	// 82F55590: 394807E8  addi r10, r8, 0x7e8
	ctx.r[10].s64 = ctx.r[8].s64 + 2024;
	// 82F55594: 392707DC  addi r9, r7, 0x7dc
	ctx.r[9].s64 = ctx.r[7].s64 + 2012;
	// 82F55598: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F5559C: 390607D0  addi r8, r6, 0x7d0
	ctx.r[8].s64 = ctx.r[6].s64 + 2000;
	// 82F555A0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F555A4: 38E507B8  addi r7, r5, 0x7b8
	ctx.r[7].s64 = ctx.r[5].s64 + 1976;
	// 82F555A8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F555AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F555B0: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F555B4: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F555B8: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F555BC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F555C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F555C4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82F555C8: 90DF003C  stw r6, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 82F555CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F555D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F555D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F555D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F555DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F555E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F555E0 size=440
    let mut pc: u32 = 0x82F555E0;
    'dispatch: loop {
        match pc {
            0x82F555E0 => {
    //   block [0x82F555E0..0x82F55798)
	// 82F555E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F555E4: 48252B85  bl 0x831a8168
	ctx.lr = 0x82F555E8;
	sub_831A8130(ctx, base);
	// 82F555E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F555EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F555F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F555F4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F555F8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F555FC: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F55600: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F55604: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F55608: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82F5560C: 388B07FC  addi r4, r11, 0x7fc
	ctx.r[4].s64 = ctx.r[11].s64 + 2044;
	// 82F55610: 38690810  addi r3, r9, 0x810
	ctx.r[3].s64 = ctx.r[9].s64 + 2064;
	// 82F55614: 396807E8  addi r11, r8, 0x7e8
	ctx.r[11].s64 = ctx.r[8].s64 + 2024;
	// 82F55618: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F5561C: 392707DC  addi r9, r7, 0x7dc
	ctx.r[9].s64 = ctx.r[7].s64 + 2012;
	// 82F55620: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82F55624: 390607D0  addi r8, r6, 0x7d0
	ctx.r[8].s64 = ctx.r[6].s64 + 2000;
	// 82F55628: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F5562C: 38E507B8  addi r7, r5, 0x7b8
	ctx.r[7].s64 = ctx.r[5].s64 + 1976;
	// 82F55630: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F55634: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82F55638: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F5563C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F55640: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F55644: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55648: 409900D4  ble cr6, 0x82f5571c
	if !ctx.cr[6].gt {
	pc = 0x82F5571C; continue 'dispatch;
	}
	// 82F5564C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F55650: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F55654: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F55658: 480004A9  bl 0x82f55b00
	ctx.lr = 0x82F5565C;
	sub_82F55B00(ctx, base);
	// 82F5565C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55664: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55668: 40990040  ble cr6, 0x82f556a8
	if !ctx.cr[6].gt {
	pc = 0x82F556A8; continue 'dispatch;
	}
	// 82F5566C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55670: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55674: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F55678: 419A0018  beq cr6, 0x82f55690
	if ctx.cr[6].eq {
	pc = 0x82F55690; continue 'dispatch;
	}
	// 82F5567C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F55680: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F55684: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F55688: 4198FFE8  blt cr6, 0x82f55670
	if ctx.cr[6].lt {
	pc = 0x82F55670; continue 'dispatch;
	}
	// 82F5568C: 4800001C  b 0x82f556a8
	pc = 0x82F556A8; continue 'dispatch;
	// 82F55690: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F55694: 41980014  blt cr6, 0x82f556a8
	if ctx.cr[6].lt {
	pc = 0x82F556A8; continue 'dispatch;
	}
	// 82F55698: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F5569C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F556A0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F556A4: 4BF83F05  bl 0x82ed95a8
	ctx.lr = 0x82F556A8;
	sub_82ED95A8(ctx, base);
	// 82F556A8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F556AC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F556B0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F556B4: 4800044D  bl 0x82f55b00
	ctx.lr = 0x82F556B8;
	sub_82F55B00(ctx, base);
	// 82F556B8: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F556BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F556C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F556C4: 40990044  ble cr6, 0x82f55708
	if !ctx.cr[6].gt {
	pc = 0x82F55708; continue 'dispatch;
	}
	// 82F556C8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F556CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F556D0: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F556D4: 419A0018  beq cr6, 0x82f556ec
	if ctx.cr[6].eq {
	pc = 0x82F556EC; continue 'dispatch;
	}
	// 82F556D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F556DC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F556E0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F556E4: 4198FFE8  blt cr6, 0x82f556cc
	if ctx.cr[6].lt {
	pc = 0x82F556CC; continue 'dispatch;
	}
	// 82F556E8: 48000020  b 0x82f55708
	pc = 0x82F55708; continue 'dispatch;
	// 82F556EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F556F0: 41980018  blt cr6, 0x82f55708
	if ctx.cr[6].lt {
	pc = 0x82F55708; continue 'dispatch;
	}
	// 82F556F4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F556F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F556FC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F55700: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55704: 4BF83EA5  bl 0x82ed95a8
	ctx.lr = 0x82F55708;
	sub_82ED95A8(ctx, base);
	// 82F55708: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F5570C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F55710: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F55714: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F55718: 4198FF38  blt cr6, 0x82f55650
	if ctx.cr[6].lt {
	pc = 0x82F55650; continue 'dispatch;
	}
	// 82F5571C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F55720: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F55724: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55728: 409A0020  bne cr6, 0x82f55748
	if !ctx.cr[6].eq {
	pc = 0x82F55748; continue 'dispatch;
	}
	// 82F5572C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55730: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F55734: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F55738: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F5573C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F55740: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F55744: 4BF4B06D  bl 0x82ea07b0
	ctx.lr = 0x82F55748;
	sub_82EA07B0(ctx, base);
	// 82F55748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F5574C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F55750: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F55754: 390BBA8C  addi r8, r11, -0x4574
	ctx.r[8].s64 = ctx.r[11].s64 + -17780;
	// 82F55758: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82F5575C: 38CABD74  addi r6, r10, -0x428c
	ctx.r[6].s64 = ctx.r[10].s64 + -17036;
	// 82F55760: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F55764: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F55768: 3889BD94  addi r4, r9, -0x426c
	ctx.r[4].s64 = ctx.r[9].s64 + -17004;
	// 82F5576C: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F55770: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 82F55774: 3967BD80  addi r11, r7, -0x4280
	ctx.r[11].s64 = ctx.r[7].s64 + -17024;
	// 82F55778: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82F5577C: 3945BD68  addi r10, r5, -0x4298
	ctx.r[10].s64 = ctx.r[5].s64 + -17048;
	// 82F55780: 39239EAC  addi r9, r3, -0x6154
	ctx.r[9].s64 = ctx.r[3].s64 + -24916;
	// 82F55784: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F55788: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82F5578C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F55790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F55794: 48252A24  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55798 size=444
    let mut pc: u32 = 0x82F55798;
    'dispatch: loop {
        match pc {
            0x82F55798 => {
    //   block [0x82F55798..0x82F55954)
	// 82F55798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5579C: 482529CD  bl 0x831a8168
	ctx.lr = 0x82F557A0;
	sub_831A8130(ctx, base);
	// 82F557A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F557A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F557A8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F557AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F557B0: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82F557B4: 40990010  ble cr6, 0x82f557c4
	if !ctx.cr[6].gt {
	pc = 0x82F557C4; continue 'dispatch;
	}
	// 82F557B8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82F557BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F557C0: 4800000C  b 0x82f557cc
	pc = 0x82F557CC; continue 'dispatch;
	// 82F557C4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F557C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F557CC: 811D0038  lwz r8, 0x38(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F557D0: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 82F557D4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F557D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F557DC: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F557E0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F557E4: 40990050  ble cr6, 0x82f55834
	if !ctx.cr[6].gt {
	pc = 0x82F55834; continue 'dispatch;
	}
	// 82F557E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F557EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F557F0: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F557F4: 409A0014  bne cr6, 0x82f55808
	if !ctx.cr[6].eq {
	pc = 0x82F55808; continue 'dispatch;
	}
	// 82F557F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F557FC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82F55800: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F55804: 419A0008  beq cr6, 0x82f5580c
	if ctx.cr[6].eq {
	pc = 0x82F5580C; continue 'dispatch;
	}
	// 82F55808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F5580C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82F55810: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55814: 409A0018  bne cr6, 0x82f5582c
	if !ctx.cr[6].eq {
	pc = 0x82F5582C; continue 'dispatch;
	}
	// 82F55818: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F5581C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F55820: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F55824: 4198FFC8  blt cr6, 0x82f557ec
	if ctx.cr[6].lt {
	pc = 0x82F557EC; continue 'dispatch;
	}
	// 82F55828: 4800000C  b 0x82f55834
	pc = 0x82F55834; continue 'dispatch;
	// 82F5582C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55830: 4199011C  bgt cr6, 0x82f5594c
	if ctx.cr[6].gt {
	pc = 0x82F5594C; continue 'dispatch;
	}
	// 82F55834: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F55838: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5583C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F55840: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F55844: 409A0010  bne cr6, 0x82f55854
	if !ctx.cr[6].eq {
	pc = 0x82F55854; continue 'dispatch;
	}
	// 82F55848: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82F5584C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F55850: 4BF51031  bl 0x82ea6880
	ctx.lr = 0x82F55854;
	sub_82EA6880(ctx, base);
	// 82F55854: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55858: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82F5585C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55860: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82F55864: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F55868: 7D28512A  stdx r9, r8, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 82F5586C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55870: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F55874: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F55878: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82F5587C: 409A0008  bne cr6, 0x82f55884
	if !ctx.cr[6].eq {
	pc = 0x82F55884; continue 'dispatch;
	}
	// 82F55880: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F55884: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F55888: 48000279  bl 0x82f55b00
	ctx.lr = 0x82F5588C;
	sub_82F55B00(ctx, base);
	// 82F5588C: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55894: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55898: 40990030  ble cr6, 0x82f558c8
	if !ctx.cr[6].gt {
	pc = 0x82F558C8; continue 'dispatch;
	}
	// 82F5589C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F558A0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F558A4: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82F558A8: 419A0018  beq cr6, 0x82f558c0
	if ctx.cr[6].eq {
	pc = 0x82F558C0; continue 'dispatch;
	}
	// 82F558AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F558B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F558B4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F558B8: 4198FFE8  blt cr6, 0x82f558a0
	if ctx.cr[6].lt {
	pc = 0x82F558A0; continue 'dispatch;
	}
	// 82F558BC: 4800000C  b 0x82f558c8
	pc = 0x82F558C8; continue 'dispatch;
	// 82F558C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F558C4: 4098001C  bge cr6, 0x82f558e0
	if !ctx.cr[6].lt {
	pc = 0x82F558E0; continue 'dispatch;
	}
	// 82F558C8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82F558CC: 389D0030  addi r4, r29, 0x30
	ctx.r[4].s64 = ctx.r[29].s64 + 48;
	// 82F558D0: 409A0008  bne cr6, 0x82f558d8
	if !ctx.cr[6].eq {
	pc = 0x82F558D8; continue 'dispatch;
	}
	// 82F558D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F558D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F558DC: 4BF83B9D  bl 0x82ed9478
	ctx.lr = 0x82F558E0;
	sub_82ED9478(ctx, base);
	// 82F558E0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82F558E4: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82F558E8: 409A0008  bne cr6, 0x82f558f0
	if !ctx.cr[6].eq {
	pc = 0x82F558F0; continue 'dispatch;
	}
	// 82F558EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F558F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F558F4: 4800020D  bl 0x82f55b00
	ctx.lr = 0x82F558F8;
	sub_82F55B00(ctx, base);
	// 82F558F8: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F558FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55900: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55904: 40990030  ble cr6, 0x82f55934
	if !ctx.cr[6].gt {
	pc = 0x82F55934; continue 'dispatch;
	}
	// 82F55908: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5590C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55910: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82F55914: 419A0018  beq cr6, 0x82f5592c
	if ctx.cr[6].eq {
	pc = 0x82F5592C; continue 'dispatch;
	}
	// 82F55918: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F5591C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F55920: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F55924: 4198FFE8  blt cr6, 0x82f5590c
	if ctx.cr[6].lt {
	pc = 0x82F5590C; continue 'dispatch;
	}
	// 82F55928: 4800000C  b 0x82f55934
	pc = 0x82F55934; continue 'dispatch;
	// 82F5592C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F55930: 4098001C  bge cr6, 0x82f5594c
	if !ctx.cr[6].lt {
	pc = 0x82F5594C; continue 'dispatch;
	}
	// 82F55934: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82F55938: 389D0030  addi r4, r29, 0x30
	ctx.r[4].s64 = ctx.r[29].s64 + 48;
	// 82F5593C: 409A0008  bne cr6, 0x82f55944
	if !ctx.cr[6].eq {
	pc = 0x82F55944; continue 'dispatch;
	}
	// 82F55940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F55944: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F55948: 4BF83B31  bl 0x82ed9478
	ctx.lr = 0x82F5594C;
	sub_82ED9478(ctx, base);
	// 82F5594C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F55950: 48252868  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55958 size=424
    let mut pc: u32 = 0x82F55958;
    'dispatch: loop {
        match pc {
            0x82F55958 => {
    //   block [0x82F55958..0x82F55B00)
	// 82F55958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5595C: 48252811  bl 0x831a816c
	ctx.lr = 0x82F55960;
	sub_831A8130(ctx, base);
	// 82F55960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55964: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F55968: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F5596C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55970: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F55974: 40990010  ble cr6, 0x82f55984
	if !ctx.cr[6].gt {
	pc = 0x82F55984; continue 'dispatch;
	}
	// 82F55978: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82F5597C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F55980: 4800000C  b 0x82f5598c
	pc = 0x82F5598C; continue 'dispatch;
	// 82F55984: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82F55988: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F5598C: 811F0038  lwz r8, 0x38(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F55990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F55994: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F55998: 40990160  ble cr6, 0x82f55af8
	if !ctx.cr[6].gt {
	pc = 0x82F55AF8; continue 'dispatch;
	}
	// 82F5599C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F559A0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F559A4: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F559A8: 409A0014  bne cr6, 0x82f559bc
	if !ctx.cr[6].eq {
	pc = 0x82F559BC; continue 'dispatch;
	}
	// 82F559AC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F559B0: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82F559B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F559B8: 419A0008  beq cr6, 0x82f559c0
	if ctx.cr[6].eq {
	pc = 0x82F559C0; continue 'dispatch;
	}
	// 82F559BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F559C0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82F559C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F559C8: 409A001C  bne cr6, 0x82f559e4
	if !ctx.cr[6].eq {
	pc = 0x82F559E4; continue 'dispatch;
	}
	// 82F559CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F559D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F559D4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F559D8: 4198FFC8  blt cr6, 0x82f559a0
	if ctx.cr[6].lt {
	pc = 0x82F559A0; continue 'dispatch;
	}
	// 82F559DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F559E0: 482527DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82F559E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F559E8: 41980110  blt cr6, 0x82f55af8
	if ctx.cr[6].lt {
	pc = 0x82F55AF8; continue 'dispatch;
	}
	// 82F559EC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F559F0: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F559F4: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F559F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F559FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F55A00: 7D095214  add r8, r9, r10
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F55A04: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F55A08: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82F55A0C: 7CE95214  add r7, r9, r10
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F55A10: 7CC9502E  lwzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F55A14: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F55A18: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55A1C: 90A80004  stw r5, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F55A20: 480000E1  bl 0x82f55b00
	ctx.lr = 0x82F55A24;
	sub_82F55B00(ctx, base);
	// 82F55A24: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F55A28: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82F55A2C: 409A0008  bne cr6, 0x82f55a34
	if !ctx.cr[6].eq {
	pc = 0x82F55A34; continue 'dispatch;
	}
	// 82F55A30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F55A34: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F55A3C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55A40: 40990048  ble cr6, 0x82f55a88
	if !ctx.cr[6].gt {
	pc = 0x82F55A88; continue 'dispatch;
	}
	// 82F55A44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55A48: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55A4C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F55A50: 419A0018  beq cr6, 0x82f55a68
	if ctx.cr[6].eq {
	pc = 0x82F55A68; continue 'dispatch;
	}
	// 82F55A54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F55A58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F55A5C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F55A60: 4198FFE8  blt cr6, 0x82f55a48
	if ctx.cr[6].lt {
	pc = 0x82F55A48; continue 'dispatch;
	}
	// 82F55A64: 48000024  b 0x82f55a88
	pc = 0x82F55A88; continue 'dispatch;
	// 82F55A68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55A6C: 4198001C  blt cr6, 0x82f55a88
	if ctx.cr[6].lt {
	pc = 0x82F55A88; continue 'dispatch;
	}
	// 82F55A70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F55A74: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82F55A78: 409A0008  bne cr6, 0x82f55a80
	if !ctx.cr[6].eq {
	pc = 0x82F55A80; continue 'dispatch;
	}
	// 82F55A7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F55A80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F55A84: 4BF83B25  bl 0x82ed95a8
	ctx.lr = 0x82F55A88;
	sub_82ED95A8(ctx, base);
	// 82F55A88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F55A8C: 48000075  bl 0x82f55b00
	ctx.lr = 0x82F55A90;
	sub_82F55B00(ctx, base);
	// 82F55A90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F55A94: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82F55A98: 409A0008  bne cr6, 0x82f55aa0
	if !ctx.cr[6].eq {
	pc = 0x82F55AA0; continue 'dispatch;
	}
	// 82F55A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F55AA0: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F55AA8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F55AAC: 4099004C  ble cr6, 0x82f55af8
	if !ctx.cr[6].gt {
	pc = 0x82F55AF8; continue 'dispatch;
	}
	// 82F55AB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55AB4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55AB8: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F55ABC: 419A001C  beq cr6, 0x82f55ad8
	if ctx.cr[6].eq {
	pc = 0x82F55AD8; continue 'dispatch;
	}
	// 82F55AC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F55AC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F55AC8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F55ACC: 4198FFE8  blt cr6, 0x82f55ab4
	if ctx.cr[6].lt {
	pc = 0x82F55AB4; continue 'dispatch;
	}
	// 82F55AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F55AD4: 482526E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82F55AD8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F55ADC: 4198001C  blt cr6, 0x82f55af8
	if ctx.cr[6].lt {
	pc = 0x82F55AF8; continue 'dispatch;
	}
	// 82F55AE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F55AE4: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82F55AE8: 409A0008  bne cr6, 0x82f55af0
	if !ctx.cr[6].eq {
	pc = 0x82F55AF0; continue 'dispatch;
	}
	// 82F55AEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F55AF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F55AF4: 4BF83AB5  bl 0x82ed95a8
	ctx.lr = 0x82F55AF8;
	sub_82ED95A8(ctx, base);
	// 82F55AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F55AFC: 482526C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55B00 size=180
    let mut pc: u32 = 0x82F55B00;
    'dispatch: loop {
        match pc {
            0x82F55B00 => {
    //   block [0x82F55B00..0x82F55BB4)
	// 82F55B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F55B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F55B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F55B0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55B14: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82F55B18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F55B1C: 409A007C  bne cr6, 0x82f55b98
	if !ctx.cr[6].eq {
	pc = 0x82F55B98; continue 'dispatch;
	}
	// 82F55B20: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55B24: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F55B28: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F55B2C: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F55B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F55B34: 419A001C  beq cr6, 0x82f55b50
	if ctx.cr[6].eq {
	pc = 0x82F55B50; continue 'dispatch;
	}
	// 82F55B38: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F55B3C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82F55B40: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82F55B44: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55B48: 91230048  stw r9, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82F55B4C: 48000014  b 0x82f55b60
	pc = 0x82F55B60; continue 'dispatch;
	// 82F55B50: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F55B54: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82F55B58: 4BF4AB09  bl 0x82ea0660
	ctx.lr = 0x82F55B5C;
	sub_82EA0660(ctx, base);
	// 82F55B5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F55B60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F55B64: 419A002C  beq cr6, 0x82f55b90
	if ctx.cr[6].eq {
	pc = 0x82F55B90; continue 'dispatch;
	}
	// 82F55B68: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82F55B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F55B70: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82F55B74: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F55B78: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F55B7C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F55B80: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82F55B84: B12B000C  sth r9, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82F55B88: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82F55B8C: 48000008  b 0x82f55b94
	pc = 0x82F55B94; continue 'dispatch;
	// 82F55B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F55B94: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82F55B98: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82F55B9C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82F55BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F55BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F55BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F55BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F55BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F55BB8 size=116
    let mut pc: u32 = 0x82F55BB8;
    'dispatch: loop {
        match pc {
            0x82F55BB8 => {
    //   block [0x82F55BB8..0x82F55C2C)
	// 82F55BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F55BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F55BC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F55BC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55BC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F55BCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F55BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55BD4: 482ABADD  bl 0x832016b0
	ctx.lr = 0x82F55BD8;
	sub_832016B0(ctx, base);
	// 82F55BD8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82F55BDC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F55BE0: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F55BE4: 38E80E80  addi r7, r8, 0xe80
	ctx.r[7].s64 = ctx.r[8].s64 + 3712;
	// 82F55BE8: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82F55BEC: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F55BF0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F55BF4: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82F55BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F55BFC: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F55C30 size=112
    let mut pc: u32 = 0x82F55C30;
    'dispatch: loop {
        match pc {
            0x82F55C30 => {
    //   block [0x82F55C30..0x82F55CA0)
	// 82F55C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F55C34: 48252539  bl 0x831a816c
	ctx.lr = 0x82F55C38;
	sub_831A8130(ctx, base);
	// 82F55C38: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82F55C3C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F55C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55C44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F55C48: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F55C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F55C50: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F55C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55C58: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F55C5C: 482ABA55  bl 0x832016b0
	ctx.lr = 0x82F55C60;
	sub_832016B0(ctx, base);
	// 82F55C60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F55C64: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F55C68: 392B0E80  addi r9, r11, 0xe80
	ctx.r[9].s64 = ctx.r[11].s64 + 3712;
	// 82F55C6C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82F55C70: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F55CA0 size=152
    let mut pc: u32 = 0x82F55CA0;
    'dispatch: loop {
        match pc {
            0x82F55CA0 => {
    //   block [0x82F55CA0..0x82F55D38)
	// 82F55CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F55CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F55CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F55CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F55CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F55CB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F55CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F55CBC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55CC0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F55CC4: 419A000C  beq cr6, 0x82f55cd0
	if ctx.cr[6].eq {
	pc = 0x82F55CD0; continue 'dispatch;
	}
	// 82F55CC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F55CCC: 48000054  b 0x82f55d20
	pc = 0x82F55D20; continue 'dispatch;
	// 82F55CD0: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F55CD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F55CD8: 409AFFF0  bne cr6, 0x82f55cc8
	if !ctx.cr[6].eq {
	pc = 0x82F55CC8; continue 'dispatch;
	}
	// 82F55CDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55CE0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F55CE4: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82F55CE8: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82F55CEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F55CF0: 4BF4AA41  bl 0x82ea0730
	ctx.lr = 0x82F55CF4;
	sub_82EA0730(ctx, base);
	// 82F55CF4: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82F55CF8: 38DF0030  addi r6, r31, 0x30
	ctx.r[6].s64 = ctx.r[31].s64 + 48;
	// 82F55CFC: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F55D00: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82F55D04: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55D08: C05F0044  lfs f2, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F55D0C: C03F0040  lfs f1, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F55D10: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F55D14: 4BFFFF1D  bl 0x82f55c30
	ctx.lr = 0x82F55D18;
	sub_82F55C30(ctx, base);
	// 82F55D18: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F55D1C: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F55D20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F55D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F55D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F55D2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F55D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F55D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F55D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F55D38 size=820
    let mut pc: u32 = 0x82F55D38;
    'dispatch: loop {
        match pc {
            0x82F55D38 => {
    //   block [0x82F55D38..0x82F5606C)
	// 82F55D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F55D3C: 48252431  bl 0x831a816c
	ctx.lr = 0x82F55D40;
	sub_831A8130(ctx, base);
	// 82F55D40: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82F55D44: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F56070 size=8
    let mut pc: u32 = 0x82F56070;
    'dispatch: loop {
        match pc {
            0x82F56070 => {
    //   block [0x82F56070..0x82F56078)
	// 82F56070: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F56074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56078 size=144
    let mut pc: u32 = 0x82F56078;
    'dispatch: loop {
        match pc {
            0x82F56078 => {
    //   block [0x82F56078..0x82F56108)
	// 82F56078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F56080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F56084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F56088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5608C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F56090: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82F56094: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F56098: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F5609C: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F560A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F560A4: 419A0030  beq cr6, 0x82f560d4
	if ctx.cr[6].eq {
	pc = 0x82F560D4; continue 'dispatch;
	}
	// 82F560A8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F560AC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F560B0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F560B4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F560B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F560BC: 409A0018  bne cr6, 0x82f560d4
	if !ctx.cr[6].eq {
	pc = 0x82F560D4; continue 'dispatch;
	}
	// 82F560C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F560C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F560C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F560CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F560D0: 4E800421  bctrl
	ctx.lr = 0x82F560D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F560D4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F560D8: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F560DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F560E0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F560E4: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82F560E8: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F560EC: 7D0AF12E  stwx r8, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[8].u32) };
	// 82F560F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F560F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F560F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F560FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F56100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F56104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56108 size=184
    let mut pc: u32 = 0x82F56108;
    'dispatch: loop {
        match pc {
            0x82F56108 => {
    //   block [0x82F56108..0x82F561C0)
	// 82F56108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5610C: 48252061  bl 0x831a816c
	ctx.lr = 0x82F56110;
	sub_831A8130(ctx, base);
	// 82F56110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F56114: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F56118: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F5611C: 392A0EDC  addi r9, r10, 0xedc
	ctx.r[9].s64 = ctx.r[10].s64 + 3804;
	// 82F56120: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F56124: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F56128: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F5612C: 40990058  ble cr6, 0x82f56184
	if !ctx.cr[6].gt {
	pc = 0x82F56184; continue 'dispatch;
	}
	// 82F56130: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F56134: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82F56138: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F5613C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56140: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56144: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F56148: 419A0030  beq cr6, 0x82f56178
	if ctx.cr[6].eq {
	pc = 0x82F56178; continue 'dispatch;
	}
	// 82F5614C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F56150: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F56154: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F56158: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F5615C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F56160: 409A0018  bne cr6, 0x82f56178
	if !ctx.cr[6].eq {
	pc = 0x82F56178; continue 'dispatch;
	}
	// 82F56164: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56168: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F5616C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56170: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56174: 4E800421  bctrl
	ctx.lr = 0x82F56178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56178: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F5617C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F56180: 4082FFB8  bne 0x82f56138
	if !ctx.cr[0].eq {
	pc = 0x82F56138; continue 'dispatch;
	}
	// 82F56184: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F56188: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5618C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56190: 409A0020  bne cr6, 0x82f561b0
	if !ctx.cr[6].eq {
	pc = 0x82F561B0; continue 'dispatch;
	}
	// 82F56194: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56198: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F5619C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F561A0: 809E0044  lwz r4, 0x44(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F561A4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F561A8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F561AC: 4BF4A605  bl 0x82ea07b0
	ctx.lr = 0x82F561B0;
	sub_82EA07B0(ctx, base);
	// 82F561B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F561B4: 4BF950AD  bl 0x82eeb260
	ctx.lr = 0x82F561B8;
	sub_82EEB260(ctx, base);
	// 82F561B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F561BC: 48252000  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F561C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F561C0 size=224
    let mut pc: u32 = 0x82F561C0;
    'dispatch: loop {
        match pc {
            0x82F561C0 => {
    //   block [0x82F561C0..0x82F562A0)
	// 82F561C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F561C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F561C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F561CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F561D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F561D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F561D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F561DC: 4BF952AD  bl 0x82eeb488
	ctx.lr = 0x82F561E0;
	sub_82EEB488(ctx, base);
	// 82F561E0: 3BFF0044  addi r31, r31, 0x44
	ctx.r[31].s64 = ctx.r[31].s64 + 68;
	// 82F561E4: 3BDE0044  addi r30, r30, 0x44
	ctx.r[30].s64 = ctx.r[30].s64 + 68;
	// 82F561E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F561EC: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F561F0: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F561F4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F561F8: 40980060  bge cr6, 0x82f56258
	if !ctx.cr[6].lt {
	pc = 0x82F56258; continue 'dispatch;
	}
	// 82F561FC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F56200: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56204: 409A0020  bne cr6, 0x82f56224
	if !ctx.cr[6].eq {
	pc = 0x82F56224; continue 'dispatch;
	}
	// 82F56208: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5620C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F56210: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F56214: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56218: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F5621C: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56220: 4BF4A591  bl 0x82ea07b0
	ctx.lr = 0x82F56224;
	sub_82EA07B0(ctx, base);
	// 82F56224: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56228: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F5622C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56230: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82F56234: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F56238: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F5623C: 4BF4A4F5  bl 0x82ea0730
	ctx.lr = 0x82F56240;
	sub_82EA0730(ctx, base);
	// 82F56240: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56244: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F56248: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82F5624C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56250: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82F56254: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F56258: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F5625C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56260: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56264: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82F56268: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5626C: 4099001C  ble cr6, 0x82f56288
	if !ctx.cr[6].gt {
	pc = 0x82F56288; continue 'dispatch;
	}
	// 82F56270: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82F56274: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56278: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F5627C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F56280: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F56284: 4082FFF0  bne 0x82f56274
	if !ctx.cr[0].eq {
	pc = 0x82F56274; continue 'dispatch;
	}
	// 82F56288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F5628C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F56290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F56294: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F56298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F5629C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F562A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F562A0 size=144
    let mut pc: u32 = 0x82F562A0;
    'dispatch: loop {
        match pc {
            0x82F562A0 => {
    //   block [0x82F562A0..0x82F56330)
	// 82F562A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F562A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F562A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F562AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F562B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F562B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F562B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82F562BC: 419A005C  beq cr6, 0x82f56318
	if ctx.cr[6].eq {
	pc = 0x82F56318; continue 'dispatch;
	}
	// 82F562C0: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F562C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F562C8: 419A0010  beq cr6, 0x82f562d8
	if ctx.cr[6].eq {
	pc = 0x82F562D8; continue 'dispatch;
	}
	// 82F562CC: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F562D0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F562D4: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F562D8: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F562DC: 3BE30044  addi r31, r3, 0x44
	ctx.r[31].s64 = ctx.r[3].s64 + 68;
	// 82F562E0: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F562E4: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F562E8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F562EC: 409A0010  bne cr6, 0x82f562fc
	if !ctx.cr[6].eq {
	pc = 0x82F562FC; continue 'dispatch;
	}
	// 82F562F0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F562F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F562F8: 4BF50589  bl 0x82ea6880
	ctx.lr = 0x82F562FC;
	sub_82EA6880(ctx, base);
	// 82F562FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56300: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56304: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F56308: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82F5630C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56310: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F56314: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F56318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F5631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F56320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F56324: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F56328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F5632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56330 size=332
    let mut pc: u32 = 0x82F56330;
    'dispatch: loop {
        match pc {
            0x82F56330 => {
    //   block [0x82F56330..0x82F5647C)
	// 82F56330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F56334: 48251E2D  bl 0x831a8160
	ctx.lr = 0x82F56338;
	sub_831A8130(ctx, base);
	// 82F56338: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5633C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F56340: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F56344: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F56348: 4BF84661  bl 0x82eda9a8
	ctx.lr = 0x82F5634C;
	sub_82EDA9A8(ctx, base);
	// 82F5634C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56350: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F56354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56358: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5635C: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56360: 38800310  li r4, 0x310
	ctx.r[4].s64 = 784;
	// 82F56364: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56368: 409A0018  bne cr6, 0x82f56380
	if !ctx.cr[6].eq {
	pc = 0x82F56380; continue 'dispatch;
	}
	// 82F5636C: 4BF4A3C5  bl 0x82ea0730
	ctx.lr = 0x82F56370;
	sub_82EA0730(ctx, base);
	// 82F56370: 39200310  li r9, 0x310
	ctx.r[9].s64 = 784;
	// 82F56374: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F56378: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F5637C: 48000014  b 0x82f56390
	pc = 0x82F56390; continue 'dispatch;
	// 82F56380: 4BF4A3B1  bl 0x82ea0730
	ctx.lr = 0x82F56384;
	sub_82EA0730(ctx, base);
	// 82F56384: 39200310  li r9, 0x310
	ctx.r[9].s64 = 784;
	// 82F56388: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F5638C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56390: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 82F56394: 60A5C544  ori r5, r5, 0xc544
	ctx.r[5].u64 = ctx.r[5].u64 | 50500;
	// 82F56398: 4BF79751  bl 0x82ecfae8
	ctx.lr = 0x82F5639C;
	sub_82ECFAE8(ctx, base);
	// 82F5639C: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 82F563A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F563A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F563A8: 419A000C  beq cr6, 0x82f563b4
	if ctx.cr[6].eq {
	pc = 0x82F563B4; continue 'dispatch;
	}
	// 82F563AC: 807C0074  lwz r3, 0x74(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(116 as u32) ) } as u64;
	// 82F563B0: 4BFC7B81  bl 0x82f1df30
	ctx.lr = 0x82F563B4;
	sub_82F1DF30(ctx, base);
	// 82F563B4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F563B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F563BC: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82F563C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F563C4: 40990084  ble cr6, 0x82f56448
	if !ctx.cr[6].gt {
	pc = 0x82F56448; continue 'dispatch;
	}
	// 82F563C8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82F563CC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82F563D0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F563D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F563D8: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F563DC: 4BF7DF4D  bl 0x82ed4328
	ctx.lr = 0x82F563E0;
	sub_82ED4328(ctx, base);
	// 82F563E0: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F563E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F563E8: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F563EC: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F563F0: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F563F4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F563F8: 4E800421  bctrl
	ctx.lr = 0x82F563FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F563FC: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56400: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F56404: 419A0030  beq cr6, 0x82f56434
	if ctx.cr[6].eq {
	pc = 0x82F56434; continue 'dispatch;
	}
	// 82F56408: 9BA10059  stb r29, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[29].u8 ) };
	// 82F5640C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F56410: 9BA10058  stb r29, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u8 ) };
	// 82F56414: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F56418: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F5641C: 9B610060  stb r27, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 82F56420: 9B610061  stb r27, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[27].u8 ) };
	// 82F56424: 9BA10062  stb r29, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[29].u8 ) };
	// 82F56428: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F5642C: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56430: 480064C1  bl 0x82f5c8f0
	ctx.lr = 0x82F56434;
	sub_82F5C8F0(ctx, base);
	// 82F56434: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56438: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82F5643C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F56440: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56444: 4198FF8C  blt cr6, 0x82f563d0
	if ctx.cr[6].lt {
	pc = 0x82F563D0; continue 'dispatch;
	}
	// 82F56448: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5644C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F56450: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56454: 419A0020  beq cr6, 0x82f56474
	if ctx.cr[6].eq {
	pc = 0x82F56474; continue 'dispatch;
	}
	// 82F56458: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F5645C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56460: 419A0014  beq cr6, 0x82f56474
	if ctx.cr[6].eq {
	pc = 0x82F56474; continue 'dispatch;
	}
	// 82F56464: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F56468: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F5646C: 4BF7C57D  bl 0x82ed29e8
	ctx.lr = 0x82F56470;
	sub_82ED29E8(ctx, base);
	// 82F56470: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F56474: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82F56478: 48251D38  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56480 size=232
    let mut pc: u32 = 0x82F56480;
    'dispatch: loop {
        match pc {
            0x82F56480 => {
    //   block [0x82F56480..0x82F56568)
	// 82F56480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F56484: 48251CE5  bl 0x831a8168
	ctx.lr = 0x82F56488;
	sub_831A8130(ctx, base);
	// 82F56488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F5648C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F56490: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F56494: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F56498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F5649C: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F564A0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F564A4: 40990024  ble cr6, 0x82f564c8
	if !ctx.cr[6].gt {
	pc = 0x82F564C8; continue 'dispatch;
	}
	// 82F564A8: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F564AC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F564B0: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F564B4: 419A004C  beq cr6, 0x82f56500
	if ctx.cr[6].eq {
	pc = 0x82F56500; continue 'dispatch;
	}
	// 82F564B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F564BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82F564C0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F564C4: 4198FFE8  blt cr6, 0x82f564ac
	if ctx.cr[6].lt {
	pc = 0x82F564AC; continue 'dispatch;
	}
	// 82F564C8: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82F564CC: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F564D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F564D4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F564D8: 40990038  ble cr6, 0x82f56510
	if !ctx.cr[6].gt {
	pc = 0x82F56510; continue 'dispatch;
	}
	// 82F564DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F564E0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F564E4: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F564E8: 419A0020  beq cr6, 0x82f56508
	if ctx.cr[6].eq {
	pc = 0x82F56508; continue 'dispatch;
	}
	// 82F564EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F564F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F564F4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F564F8: 4198FFE8  blt cr6, 0x82f564e0
	if ctx.cr[6].lt {
	pc = 0x82F564E0; continue 'dispatch;
	}
	// 82F564FC: 48000014  b 0x82f56510
	pc = 0x82F56510; continue 'dispatch;
	// 82F56500: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82F56504: 4BFFFFC8  b 0x82f564cc
	pc = 0x82F564CC; continue 'dispatch;
	// 82F56508: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82F5650C: 409A0054  bne cr6, 0x82f56560
	if !ctx.cr[6].eq {
	pc = 0x82F56560; continue 'dispatch;
	}
	// 82F56510: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82F56514: 419A004C  beq cr6, 0x82f56560
	if ctx.cr[6].eq {
	pc = 0x82F56560; continue 'dispatch;
	}
	// 82F56518: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F5651C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56520: 409A0020  bne cr6, 0x82f56540
	if !ctx.cr[6].eq {
	pc = 0x82F56540; continue 'dispatch;
	}
	// 82F56524: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F56528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F5652C: 4BF824DD  bl 0x82ed8a08
	ctx.lr = 0x82F56530;
	sub_82ED8A08(ctx, base);
	// 82F56530: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F5653C: 419A0008  beq cr6, 0x82f56544
	if ctx.cr[6].eq {
	pc = 0x82F56544; continue 'dispatch;
	}
	// 82F56540: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F56544: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82F56548: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F5654C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F56550: 4BF95211  bl 0x82eeb760
	ctx.lr = 0x82F56554;
	sub_82EEB760(ctx, base);
	// 82F56554: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F56558: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F5655C: 4BF94AF5  bl 0x82eeb050
	ctx.lr = 0x82F56560;
	sub_82EEB050(ctx, base);
	// 82F56560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F56564: 48251C54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56568 size=128
    let mut pc: u32 = 0x82F56568;
    'dispatch: loop {
        match pc {
            0x82F56568 => {
    //   block [0x82F56568..0x82F565E8)
	// 82F56568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5656C: 48251BFD  bl 0x831a8168
	ctx.lr = 0x82F56570;
	sub_831A8130(ctx, base);
	// 82F56570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F56574: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F56578: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F5657C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F56580: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56584: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56588: 40990040  ble cr6, 0x82f565c8
	if !ctx.cr[6].gt {
	pc = 0x82F565C8; continue 'dispatch;
	}
	// 82F5658C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F56590: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56594: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F56598: 806A0038  lwz r3, 0x38(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F5659C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F565A0: 419A0014  beq cr6, 0x82f565b4
	if ctx.cr[6].eq {
	pc = 0x82F565B4; continue 'dispatch;
	}
	// 82F565A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F565A8: 4BF53E81  bl 0x82eaa428
	ctx.lr = 0x82F565AC;
	sub_82EAA428(ctx, base);
	// 82F565AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F565B0: 419A0024  beq cr6, 0x82f565d4
	if ctx.cr[6].eq {
	pc = 0x82F565D4; continue 'dispatch;
	}
	// 82F565B4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F565B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F565BC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F565C0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F565C4: 4198FFCC  blt cr6, 0x82f56590
	if ctx.cr[6].lt {
	pc = 0x82F56590; continue 'dispatch;
	}
	// 82F565C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F565CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F565D0: 48251BE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F565D4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F565D8: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F565DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F565E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F565E4: 48251BD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F565E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F565E8 size=168
    let mut pc: u32 = 0x82F565E8;
    'dispatch: loop {
        match pc {
            0x82F565E8 => {
    //   block [0x82F565E8..0x82F56690)
	// 82F565E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F565EC: 48251B6D  bl 0x831a8158
	ctx.lr = 0x82F565F0;
	sub_831A8130(ctx, base);
	// 82F565F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F565F4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F565F8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82F565FC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82F56600: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56604: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56608: 40990070  ble cr6, 0x82f56678
	if !ctx.cr[6].gt {
	pc = 0x82F56678; continue 'dispatch;
	}
	// 82F5660C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F56610: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56614: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F56618: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F5661C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56620: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56624: 40990040  ble cr6, 0x82f56664
	if !ctx.cr[6].gt {
	pc = 0x82F56664; continue 'dispatch;
	}
	// 82F56628: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F5662C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56630: 7FBF582E  lwzx r29, r31, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56634: 807D0070  lwz r3, 0x70(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82F56638: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F5663C: 419A0014  beq cr6, 0x82f56650
	if ctx.cr[6].eq {
	pc = 0x82F56650; continue 'dispatch;
	}
	// 82F56640: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82F56644: 4BF53DE5  bl 0x82eaa428
	ctx.lr = 0x82F56648;
	sub_82EAA428(ctx, base);
	// 82F56648: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F5664C: 419A0038  beq cr6, 0x82f56684
	if ctx.cr[6].eq {
	pc = 0x82F56684; continue 'dispatch;
	}
	// 82F56650: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56654: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F56658: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F5665C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56660: 4198FFCC  blt cr6, 0x82f5662c
	if ctx.cr[6].lt {
	pc = 0x82F5662C; continue 'dispatch;
	}
	// 82F56664: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56668: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82F5666C: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82F56670: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56674: 4198FF9C  blt cr6, 0x82f56610
	if ctx.cr[6].lt {
	pc = 0x82F56610; continue 'dispatch;
	}
	// 82F56678: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F5667C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F56680: 48251B28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82F56684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56688: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F5668C: 48251B1C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F56690 size=48
    let mut pc: u32 = 0x82F56690;
    'dispatch: loop {
        match pc {
            0x82F56690 => {
    //   block [0x82F56690..0x82F566C0)
	// 82F56690: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F56694: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F56698: 392B0F58  addi r9, r11, 0xf58
	ctx.r[9].s64 = ctx.r[11].s64 + 3928;
	// 82F5669C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F566A0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F566A4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F566A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F566AC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F566B0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F566B4: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F566B8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F566BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F566C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F566C0 size=264
    let mut pc: u32 = 0x82F566C0;
    'dispatch: loop {
        match pc {
            0x82F566C0 => {
    //   block [0x82F566C0..0x82F567C8)
	// 82F566C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F566C4: 48251AA9  bl 0x831a816c
	ctx.lr = 0x82F566C8;
	sub_831A8130(ctx, base);
	// 82F566C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F566CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F566D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F566D4: 394B0F58  addi r10, r11, 0xf58
	ctx.r[10].s64 = ctx.r[11].s64 + 3928;
	// 82F566D8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F566DC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F566E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F566E4: 419A003C  beq cr6, 0x82f56720
	if ctx.cr[6].eq {
	pc = 0x82F56720; continue 'dispatch;
	}
	// 82F566E8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F566EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F566F0: 419A0030  beq cr6, 0x82f56720
	if ctx.cr[6].eq {
	pc = 0x82F56720; continue 'dispatch;
	}
	// 82F566F4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F566F8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F566FC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F56700: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F56704: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F56708: 409A0018  bne cr6, 0x82f56720
	if !ctx.cr[6].eq {
	pc = 0x82F56720; continue 'dispatch;
	}
	// 82F5670C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56710: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56714: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56718: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F5671C: 4E800421  bctrl
	ctx.lr = 0x82F56720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56720: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56724: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F56728: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F5672C: 4099005C  ble cr6, 0x82f56788
	if !ctx.cr[6].gt {
	pc = 0x82F56788; continue 'dispatch;
	}
	// 82F56730: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F56734: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56738: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F5673C: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56740: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F56744: 419A0030  beq cr6, 0x82f56774
	if ctx.cr[6].eq {
	pc = 0x82F56774; continue 'dispatch;
	}
	// 82F56748: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F5674C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F56750: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F56754: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F56758: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F5675C: 409A0018  bne cr6, 0x82f56774
	if !ctx.cr[6].eq {
	pc = 0x82F56774; continue 'dispatch;
	}
	// 82F56760: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56764: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56768: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5676C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56770: 4E800421  bctrl
	ctx.lr = 0x82F56774;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56774: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56778: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F5677C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F56780: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56784: 4198FFB0  blt cr6, 0x82f56734
	if ctx.cr[6].lt {
	pc = 0x82F56734; continue 'dispatch;
	}
	// 82F56788: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F5678C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F56790: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56794: 409A0020  bne cr6, 0x82f567b4
	if !ctx.cr[6].eq {
	pc = 0x82F567B4; continue 'dispatch;
	}
	// 82F56798: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5679C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F567A0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F567A4: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F567A8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F567AC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F567B0: 4BF4A001  bl 0x82ea07b0
	ctx.lr = 0x82F567B4;
	sub_82EA07B0(ctx, base);
	// 82F567B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F567B8: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F567BC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F567C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F567C4: 482519F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F567C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F567C8 size=256
    let mut pc: u32 = 0x82F567C8;
    'dispatch: loop {
        match pc {
            0x82F567C8 => {
    //   block [0x82F567C8..0x82F568C8)
	// 82F567C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F567CC: 4825199D  bl 0x831a8168
	ctx.lr = 0x82F567D0;
	sub_831A8130(ctx, base);
	// 82F567D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F567D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F567D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F567DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F567E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F567E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F567E8: 409A002C  bne cr6, 0x82f56814
	if !ctx.cr[6].eq {
	pc = 0x82F56814; continue 'dispatch;
	}
	// 82F567EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F567F0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F567F4: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82F567F8: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 82F567FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56800: 4BF49F31  bl 0x82ea0730
	ctx.lr = 0x82F56804;
	sub_82EA0730(ctx, base);
	// 82F56804: 392000D0  li r9, 0xd0
	ctx.r[9].s64 = 208;
	// 82F56808: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F5680C: 4BF8419D  bl 0x82eda9a8
	ctx.lr = 0x82F56810;
	sub_82EDA9A8(ctx, base);
	// 82F56810: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82F56814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56818: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5681C: 4BF774F5  bl 0x82ecdd10
	ctx.lr = 0x82F56820;
	sub_82ECDD10(ctx, base);
	// 82F56820: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82F56824: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56828: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F5682C: 4BF7A585  bl 0x82ed0db0
	ctx.lr = 0x82F56830;
	sub_82ED0DB0(ctx, base);
	// 82F56830: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82F56834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56838: 419A0088  beq cr6, 0x82f568c0
	if ctx.cr[6].eq {
	pc = 0x82F568C0; continue 'dispatch;
	}
	// 82F5683C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F56840: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56844: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F56848: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82F5684C: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56850: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82F56854: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82F56858: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F5685C: 4BF49ED5  bl 0x82ea0730
	ctx.lr = 0x82F56860;
	sub_82EA0730(ctx, base);
	// 82F56860: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82F56864: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F56868: 48000789  bl 0x82f56ff0
	ctx.lr = 0x82F5686C;
	sub_82F56FF0(ctx, base);
	// 82F5686C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F56870: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F56874: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F56878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F5687C: 48005375  bl 0x82f5bbf0
	ctx.lr = 0x82F56880;
	sub_82F5BBF0(ctx, base);
	// 82F56880: 9B9E0040  stb r28, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82F56884: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56888: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F5688C: 54E600BE  clrlwi r6, r7, 2
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82F56890: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F56894: 409A0010  bne cr6, 0x82f568a4
	if !ctx.cr[6].eq {
	pc = 0x82F568A4; continue 'dispatch;
	}
	// 82F56898: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F5689C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F568A0: 4BF4FFE1  bl 0x82ea6880
	ctx.lr = 0x82F568A4;
	sub_82EA6880(ctx, base);
	// 82F568A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F568A8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F568AC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F568B0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82F568B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F568B8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F568BC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F568C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F568C4: 482518F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F568C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F568C8 size=1832
    let mut pc: u32 = 0x82F568C8;
    'dispatch: loop {
        match pc {
            0x82F568C8 => {
    //   block [0x82F568C8..0x82F56FF0)
	// 82F568C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F568CC: 48251865  bl 0x831a8130
	ctx.lr = 0x82F568D0;
	sub_831A8130(ctx, base);
	// 82F568D0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F568D4: 81CD0000  lwz r14, 0(r13)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F568D8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82F568DC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82F568E0: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F568E4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F568E8: 92C1012C  stw r22, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[22].u32 ) };
	// 82F568EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F568F0: 7C6B702E  lwzx r3, r11, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F568F4: 4BF49E3D  bl 0x82ea0730
	ctx.lr = 0x82F568F8;
	sub_82EA0730(ctx, base);
	// 82F568F8: 3B000044  li r24, 0x44
	ctx.r[24].s64 = 68;
	// 82F568FC: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56900: 4BF94901  bl 0x82eeb200
	ctx.lr = 0x82F56904;
	sub_82EEB200(ctx, base);
	// 82F56904: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56908: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F5690C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F56910: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56914: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56918: 4099002C  ble cr6, 0x82f56944
	if !ctx.cr[6].gt {
	pc = 0x82F56944; continue 'dispatch;
	}
	// 82F5691C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82F56920: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56924: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56928: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F5692C: 4BF94E35  bl 0x82eeb760
	ctx.lr = 0x82F56930;
	sub_82EEB760(ctx, base);
	// 82F56930: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56934: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F56938: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82F5693C: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F56940: 4198FFE0  blt cr6, 0x82f56920
	if ctx.cr[6].lt {
	pc = 0x82F56920; continue 'dispatch;
	}
	// 82F56944: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F56948: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82F5694C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56950: 4099002C  ble cr6, 0x82f5697c
	if !ctx.cr[6].gt {
	pc = 0x82F5697C; continue 'dispatch;
	}
	// 82F56954: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56958: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F5695C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56960: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56964: 4BF94F9D  bl 0x82eeb900
	ctx.lr = 0x82F56968;
	sub_82EEB900(ctx, base);
	// 82F56968: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F5696C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F56970: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F56974: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F56978: 4198FFE0  blt cr6, 0x82f56958
	if ctx.cr[6].lt {
	pc = 0x82F56958; continue 'dispatch;
	}
	// 82F5697C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F56980: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82F56984: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56988: 4099002C  ble cr6, 0x82f569b4
	if !ctx.cr[6].gt {
	pc = 0x82F569B4; continue 'dispatch;
	}
	// 82F5698C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56990: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F56994: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56998: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F5699C: 4BF94ED5  bl 0x82eeb870
	ctx.lr = 0x82F569A0;
	sub_82EEB870(ctx, base);
	// 82F569A0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F569A4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F569A8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F569AC: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F569B0: 4198FFE0  blt cr6, 0x82f56990
	if ctx.cr[6].lt {
	pc = 0x82F56990; continue 'dispatch;
	}
	// 82F569B4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F569B8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82F569BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F569C0: 4099002C  ble cr6, 0x82f569ec
	if !ctx.cr[6].gt {
	pc = 0x82F569EC; continue 'dispatch;
	}
	// 82F569C4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F569C8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F569CC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F569D0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F569D4: 4BF94E15  bl 0x82eeb7e8
	ctx.lr = 0x82F569D8;
	sub_82EEB7E8(ctx, base);
	// 82F569D8: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F569DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F569E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F569E4: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F569E8: 4198FFE0  blt cr6, 0x82f569c8
	if ctx.cr[6].lt {
	pc = 0x82F569C8; continue 'dispatch;
	}
	// 82F569EC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82F569F0: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F569F4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F569F8: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82F569FC: 7C6B702E  lwzx r3, r11, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56A00: 4BF49D31  bl 0x82ea0730
	ctx.lr = 0x82F56A04;
	sub_82EA0730(ctx, base);
	// 82F56A04: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56A08: 4BF947F9  bl 0x82eeb200
	ctx.lr = 0x82F56A0C;
	sub_82EEB200(ctx, base);
	// 82F56A0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F56A10: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F56A14: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56A18: 394B15E4  addi r10, r11, 0x15e4
	ctx.r[10].s64 = ctx.r[11].s64 + 5604;
	// 82F56A1C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82F56A20: 8139003C  lwz r9, 0x3c(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F56A24: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82F56A28: 9B9F0040  stb r28, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82F56A2C: 8119000C  lwz r8, 0xc(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56A30: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F56A34: 40990048  ble cr6, 0x82f56a7c
	if !ctx.cr[6].gt {
	pc = 0x82F56A7C; continue 'dispatch;
	}
	// 82F56A38: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82F56A3C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56A40: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56A44: 894400D8  lbz r10, 0xd8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F56A48: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82F56A4C: 409A001C  bne cr6, 0x82f56a68
	if !ctx.cr[6].eq {
	pc = 0x82F56A68; continue 'dispatch;
	}
	// 82F56A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F56A54: 4BF94D0D  bl 0x82eeb760
	ctx.lr = 0x82F56A58;
	sub_82EEB760(ctx, base);
	// 82F56A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F56A5C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56A60: 4BF945F1  bl 0x82eeb050
	ctx.lr = 0x82F56A64;
	sub_82EEB050(ctx, base);
	// 82F56A64: 4800000C  b 0x82f56a70
	pc = 0x82F56A70; continue 'dispatch;
	// 82F56A68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F56A6C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82F56A70: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56A74: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56A78: 4198FFC4  blt cr6, 0x82f56a3c
	if ctx.cr[6].lt {
	pc = 0x82F56A3C; continue 'dispatch;
	}
	// 82F56A7C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56A80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56A84: 419A000C  beq cr6, 0x82f56a90
	if ctx.cr[6].eq {
	pc = 0x82F56A90; continue 'dispatch;
	}
	// 82F56A88: 93F60000  stw r31, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82F56A8C: 48000020  b 0x82f56aac
	pc = 0x82F56AAC; continue 'dispatch;
	// 82F56A90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56A94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F56A9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56AA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56AA4: 4E800421  bctrl
	ctx.lr = 0x82F56AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56AA8: 93960000  stw r28, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82F56AAC: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 82F56AB0: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56AB4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F56AB8: 7C7F702E  lwzx r3, r31, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56ABC: 4BF49C75  bl 0x82ea0730
	ctx.lr = 0x82F56AC0;
	sub_82EA0730(ctx, base);
	// 82F56AC0: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56AC4: 4BF9473D  bl 0x82eeb200
	ctx.lr = 0x82F56AC8;
	sub_82EEB200(ctx, base);
	// 82F56AC8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82F56ACC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F56AD0: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56AD4: 92E10058  stw r23, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[23].u32 ) };
	// 82F56AD8: 394B15C8  addi r10, r11, 0x15c8
	ctx.r[10].s64 = ctx.r[11].s64 + 5576;
	// 82F56ADC: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F56AE0: 91570038  stw r10, 0x38(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82F56AE4: 8139003C  lwz r9, 0x3c(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F56AE8: 9137003C  stw r9, 0x3c(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82F56AEC: 9B970040  stb r28, 0x40(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82F56AF0: 7C7F702E  lwzx r3, r31, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56AF4: 4BF49C3D  bl 0x82ea0730
	ctx.lr = 0x82F56AF8;
	sub_82EA0730(ctx, base);
	// 82F56AF8: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56AFC: 4BF94705  bl 0x82eeb200
	ctx.lr = 0x82F56B00;
	sub_82EEB200(ctx, base);
	// 82F56B00: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F56B04: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F56B08: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82F56B0C: 3A590014  addi r18, r25, 0x14
	ctx.r[18].s64 = ctx.r[25].s64 + 20;
	// 82F56B10: 38E815B0  addi r7, r8, 0x15b0
	ctx.r[7].s64 = ctx.r[8].s64 + 5552;
	// 82F56B14: 3A790020  addi r19, r25, 0x20
	ctx.r[19].s64 = ctx.r[25].s64 + 32;
	// 82F56B18: 90FA0038  stw r7, 0x38(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(56 as u32), ctx.r[7].u32 ) };
	// 82F56B1C: 80D9003C  lwz r6, 0x3c(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F56B20: 90DA003C  stw r6, 0x3c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 82F56B24: 9B9A0040  stb r28, 0x40(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82F56B28: 80BB0004  lwz r5, 4(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56B2C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82F56B30: 409903BC  ble cr6, 0x82f56eec
	if !ctx.cr[6].gt {
	pc = 0x82F56EEC; continue 'dispatch;
	}
	// 82F56B34: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F56B38: 3E008000  lis r16, -0x8000
	ctx.r[16].s64 = -2147483648;
	// 82F56B3C: 396B159C  addi r11, r11, 0x159c
	ctx.r[11].s64 = ctx.r[11].s64 + 5532;
	// 82F56B40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F56B44: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82F56B48: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56B4C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F56B50: 7C6B702E  lwzx r3, r11, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56B54: 4BF49BDD  bl 0x82ea0730
	ctx.lr = 0x82F56B58;
	sub_82EA0730(ctx, base);
	// 82F56B58: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56B5C: 4BF946A5  bl 0x82eeb200
	ctx.lr = 0x82F56B60;
	sub_82EEB200(ctx, base);
	// 82F56B60: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F56B64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F56B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F56B6C: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82F56B70: 8159003C  lwz r10, 0x3c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F56B74: 915D003C  stw r10, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82F56B78: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56B7C: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56B80: 4BF81E89  bl 0x82ed8a08
	ctx.lr = 0x82F56B84;
	sub_82ED8A08(ctx, base);
	// 82F56B84: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82F56B88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56B8C: 88E80000  lbz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56B90: 98FD0040  stb r7, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[7].u8 ) };
	// 82F56B94: 80DB0000  lwz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56B98: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56B9C: 4BF94BC5  bl 0x82eeb760
	ctx.lr = 0x82F56BA0;
	sub_82EEB760(ctx, base);
	// 82F56BA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F56BA4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56BA8: 4BF944A9  bl 0x82eeb050
	ctx.lr = 0x82F56BAC;
	sub_82EEB050(ctx, base);
	// 82F56BAC: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56BB0: 7F94E378  mr r20, r28
	ctx.r[20].u64 = ctx.r[28].u64;
	// 82F56BB4: 7F8FE378  mr r15, r28
	ctx.r[15].u64 = ctx.r[28].u64;
	// 82F56BB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56BBC: 40990204  ble cr6, 0x82f56dc0
	if !ctx.cr[6].gt {
	pc = 0x82F56DC0; continue 'dispatch;
	}
	// 82F56BC0: 7F91E378  mr r17, r28
	ctx.r[17].u64 = ctx.r[28].u64;
	// 82F56BC4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56BC8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56BCC: 81520004  lwz r10, 4(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56BD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56BD4: 7EAB882E  lwzx r21, r11, r17
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82F56BD8: 40990090  ble cr6, 0x82f56c68
	if !ctx.cr[6].gt {
	pc = 0x82F56C68; continue 'dispatch;
	}
	// 82F56BDC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82F56BE0: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56BE4: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56BE8: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F56BEC: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82F56BF0: 419A001C  beq cr6, 0x82f56c0c
	if ctx.cr[6].eq {
	pc = 0x82F56C0C; continue 'dispatch;
	}
	// 82F56BF4: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F56BF8: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82F56BFC: 419A0010  beq cr6, 0x82f56c0c
	if ctx.cr[6].eq {
	pc = 0x82F56C0C; continue 'dispatch;
	}
	// 82F56C00: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F56C04: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F56C08: 48000054  b 0x82f56c5c
	pc = 0x82F56C5C; continue 'dispatch;
	// 82F56C0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56C10: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82F56C14: 4BF94C5D  bl 0x82eeb870
	ctx.lr = 0x82F56C18;
	sub_82EEB870(ctx, base);
	// 82F56C18: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56C1C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56C20: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F56C24: 810A0014  lwz r8, 0x14(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F56C28: 7D274278  xor r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 ^ ctx.r[8].u64;
	// 82F56C2C: 7CE5AA78  xor r5, r7, r21
	ctx.r[5].u64 = ctx.r[7].u64 ^ ctx.r[21].u64;
	// 82F56C30: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82F56C34: 419A001C  beq cr6, 0x82f56c50
	if ctx.cr[6].eq {
	pc = 0x82F56C50; continue 'dispatch;
	}
	// 82F56C38: 896500D8  lbz r11, 0xd8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F56C3C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82F56C40: 419A0010  beq cr6, 0x82f56c50
	if ctx.cr[6].eq {
	pc = 0x82F56C50; continue 'dispatch;
	}
	// 82F56C44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F56C48: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56C4C: 4BFFF835  bl 0x82f56480
	ctx.lr = 0x82F56C50;
	sub_82F56480(ctx, base);
	// 82F56C50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F56C54: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56C58: 4BF94489  bl 0x82eeb0e0
	ctx.lr = 0x82F56C5C;
	sub_82EEB0E0(ctx, base);
	// 82F56C5C: 81720004  lwz r11, 4(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56C60: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56C64: 4198FF7C  blt cr6, 0x82f56be0
	if ctx.cr[6].lt {
	pc = 0x82F56BE0; continue 'dispatch;
	}
	// 82F56C68: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56C6C: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 82F56C70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56C74: 40990134  ble cr6, 0x82f56da8
	if !ctx.cr[6].gt {
	pc = 0x82F56DA8; continue 'dispatch;
	}
	// 82F56C78: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82F56C7C: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82F56C80: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F56C84: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F56C88: 92010068  stw r16, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[16].u32 ) };
	// 82F56C8C: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56C90: 7C7A582E  lwzx r3, r26, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56C94: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56C98: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F56C9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F56CA0: 4E800421  bctrl
	ctx.lr = 0x82F56CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56CA4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F56CA8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F56CAC: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82F56CB0: 7F96E378  mr r22, r28
	ctx.r[22].u64 = ctx.r[28].u64;
	// 82F56CB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56CB8: 409900A0  ble cr6, 0x82f56d58
	if !ctx.cr[6].gt {
	pc = 0x82F56D58; continue 'dispatch;
	}
	// 82F56CBC: 7F97E378  mr r23, r28
	ctx.r[23].u64 = ctx.r[28].u64;
	// 82F56CC0: 7D57202E  lwzx r10, r23, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82F56CC4: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82F56CC8: 409A0080  bne cr6, 0x82f56d48
	if !ctx.cr[6].eq {
	pc = 0x82F56D48; continue 'dispatch;
	}
	// 82F56CCC: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56CD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56CD4: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82F56CD8: 7C9A582E  lwzx r4, r26, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56CDC: 4BF94C25  bl 0x82eeb900
	ctx.lr = 0x82F56CE0;
	sub_82EEB900(ctx, base);
	// 82F56CE0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82F56CE4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56CE8: 4BF94489  bl 0x82eeb170
	ctx.lr = 0x82F56CEC;
	sub_82EEB170(ctx, base);
	// 82F56CEC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F56CF0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F56CF4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82F56CF8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56CFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56D00: 40990048  ble cr6, 0x82f56d48
	if !ctx.cr[6].gt {
	pc = 0x82F56D48; continue 'dispatch;
	}
	// 82F56D04: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82F56D08: 7D5F202E  lwzx r10, r31, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82F56D0C: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82F56D10: 419A0028  beq cr6, 0x82f56d38
	if ctx.cr[6].eq {
	pc = 0x82F56D38; continue 'dispatch;
	}
	// 82F56D14: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F56D18: 894500D8  lbz r10, 0xd8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F56D1C: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82F56D20: 419A0018  beq cr6, 0x82f56d38
	if ctx.cr[6].eq {
	pc = 0x82F56D38; continue 'dispatch;
	}
	// 82F56D24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F56D28: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56D2C: 4BFFF755  bl 0x82f56480
	ctx.lr = 0x82F56D30;
	sub_82F56480(ctx, base);
	// 82F56D30: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F56D34: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F56D38: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F56D3C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F56D40: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56D44: 4198FFC4  blt cr6, 0x82f56d08
	if ctx.cr[6].lt {
	pc = 0x82F56D08; continue 'dispatch;
	}
	// 82F56D48: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82F56D4C: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 82F56D50: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56D54: 4198FF6C  blt cr6, 0x82f56cc0
	if ctx.cr[6].lt {
	pc = 0x82F56CC0; continue 'dispatch;
	}
	// 82F56D58: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82F56D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56D60: 409A000C  bne cr6, 0x82f56d6c
	if !ctx.cr[6].eq {
	pc = 0x82F56D6C; continue 'dispatch;
	}
	// 82F56D64: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82F56D68: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82F56D6C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F56D70: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F56D74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56D78: 409A0018  bne cr6, 0x82f56d90
	if !ctx.cr[6].eq {
	pc = 0x82F56D90; continue 'dispatch;
	}
	// 82F56D7C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F56D80: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F56D84: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F56D88: 7C6A702E  lwzx r3, r10, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56D8C: 4BF49A25  bl 0x82ea07b0
	ctx.lr = 0x82F56D90;
	sub_82EA07B0(ctx, base);
	// 82F56D90: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56D94: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56D98: 4198FEE4  blt cr6, 0x82f56c7c
	if ctx.cr[6].lt {
	pc = 0x82F56C7C; continue 'dispatch;
	}
	// 82F56D9C: 82C1012C  lwz r22, 0x12c(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82F56DA0: 82E10058  lwz r23, 0x58(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F56DA4: 8341005C  lwz r26, 0x5c(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82F56DA8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56DAC: 39EF0001  addi r15, r15, 1
	ctx.r[15].s64 = ctx.r[15].s64 + 1;
	// 82F56DB0: 3A310004  addi r17, r17, 4
	ctx.r[17].s64 = ctx.r[17].s64 + 4;
	// 82F56DB4: 7F0F5800  cmpw cr6, r15, r11
	ctx.cr[6].compare_i32(ctx.r[15].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F56DB8: 4198FE0C  blt cr6, 0x82f56bc4
	if ctx.cr[6].lt {
	pc = 0x82F56BC4; continue 'dispatch;
	}
	// 82F56DBC: 3B000044  li r24, 0x44
	ctx.r[24].s64 = 68;
	// 82F56DC0: 568B063E  clrlwi r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	// 82F56DC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56DC8: 409AFDE4  bne cr6, 0x82f56bac
	if !ctx.cr[6].eq {
	pc = 0x82F56BAC; continue 'dispatch;
	}
	// 82F56DCC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F56DD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56DD4: 409A00C8  bne cr6, 0x82f56e9c
	if !ctx.cr[6].eq {
	pc = 0x82F56E9C; continue 'dispatch;
	}
	// 82F56DD8: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F56DDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56DE0: 409A00BC  bne cr6, 0x82f56e9c
	if !ctx.cr[6].eq {
	pc = 0x82F56E9C; continue 'dispatch;
	}
	// 82F56DE4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F56DE8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56DEC: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82F56DF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F56DF4: 419A0020  beq cr6, 0x82f56e14
	if ctx.cr[6].eq {
	pc = 0x82F56E14; continue 'dispatch;
	}
	// 82F56DF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F56DFC: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82F56E00: 4BF81C09  bl 0x82ed8a08
	ctx.lr = 0x82F56E04;
	sub_82ED8A08(ctx, base);
	// 82F56E04: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56E0C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F56E10: 419A0008  beq cr6, 0x82f56e18
	if ctx.cr[6].eq {
	pc = 0x82F56E18; continue 'dispatch;
	}
	// 82F56E14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F56E18: 895F00D8  lbz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82F56E1C: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 82F56E20: 409A0030  bne cr6, 0x82f56e50
	if !ctx.cr[6].eq {
	pc = 0x82F56E50; continue 'dispatch;
	}
	// 82F56E24: 895A0040  lbz r10, 0x40(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F56E28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F56E2C: 409A0014  bne cr6, 0x82f56e40
	if !ctx.cr[6].eq {
	pc = 0x82F56E40; continue 'dispatch;
	}
	// 82F56E30: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82F56E34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56E38: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F56E3C: 419A0008  beq cr6, 0x82f56e44
	if ctx.cr[6].eq {
	pc = 0x82F56E44; continue 'dispatch;
	}
	// 82F56E40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F56E44: 997A0040  stb r11, 0x40(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82F56E48: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F56E4C: 4800002C  b 0x82f56e78
	pc = 0x82F56E78; continue 'dispatch;
	// 82F56E50: 89570040  lbz r10, 0x40(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F56E54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F56E58: 409A0014  bne cr6, 0x82f56e6c
	if !ctx.cr[6].eq {
	pc = 0x82F56E6C; continue 'dispatch;
	}
	// 82F56E5C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82F56E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F56E64: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F56E68: 419A0008  beq cr6, 0x82f56e70
	if ctx.cr[6].eq {
	pc = 0x82F56E70; continue 'dispatch;
	}
	// 82F56E6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F56E70: 99770040  stb r11, 0x40(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82F56E74: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F56E78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F56E7C: 4BF948E5  bl 0x82eeb760
	ctx.lr = 0x82F56E80;
	sub_82EEB760(ctx, base);
	// 82F56E80: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56E84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56E88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F56E8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56E90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56E94: 4E800421  bctrl
	ctx.lr = 0x82F56E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56E98: 48000044  b 0x82f56edc
	pc = 0x82F56EDC; continue 'dispatch;
	// 82F56E9C: 81760018  lwz r11, 0x18(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F56EA0: 3BF60010  addi r31, r22, 0x10
	ctx.r[31].s64 = ctx.r[22].s64 + 16;
	// 82F56EA4: 81560014  lwz r10, 0x14(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F56EA8: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F56EAC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F56EB0: 409A0010  bne cr6, 0x82f56ec0
	if !ctx.cr[6].eq {
	pc = 0x82F56EC0; continue 'dispatch;
	}
	// 82F56EB4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F56EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F56EBC: 4BF4F9C5  bl 0x82ea6880
	ctx.lr = 0x82F56EC0;
	sub_82EA6880(ctx, base);
	// 82F56EC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56EC4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56EC8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F56ECC: 7FA9512E  stwx r29, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82F56ED0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56ED4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F56ED8: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F56EDC: 8179000C  lwz r11, 0xc(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56EE0: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82F56EE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56EE8: 4199FC5C  bgt cr6, 0x82f56b44
	if ctx.cr[6].gt {
	pc = 0x82F56B44; continue 'dispatch;
	}
	// 82F56EEC: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56EF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56EF4: 419A000C  beq cr6, 0x82f56f00
	if ctx.cr[6].eq {
	pc = 0x82F56F00; continue 'dispatch;
	}
	// 82F56EF8: 93560004  stw r26, 4(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82F56EFC: 48000020  b 0x82f56f1c
	pc = 0x82F56F1C; continue 'dispatch;
	// 82F56F00: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56F04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56F08: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F56F0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56F10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56F14: 4E800421  bctrl
	ctx.lr = 0x82F56F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56F18: 93960004  stw r28, 4(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82F56F1C: 8177000C  lwz r11, 0xc(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F56F20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56F24: 419A000C  beq cr6, 0x82f56f30
	if ctx.cr[6].eq {
	pc = 0x82F56F30; continue 'dispatch;
	}
	// 82F56F28: 92F60008  stw r23, 8(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82F56F2C: 48000020  b 0x82f56f4c
	pc = 0x82F56F4C; continue 'dispatch;
	// 82F56F30: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56F34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56F38: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F56F3C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56F40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56F44: 4E800421  bctrl
	ctx.lr = 0x82F56F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56F48: 93960008  stw r28, 8(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82F56F4C: 81790030  lwz r11, 0x30(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F56F50: 3BB9002C  addi r29, r25, 0x2c
	ctx.r[29].s64 = ctx.r[25].s64 + 44;
	// 82F56F54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F56F58: 40990074  ble cr6, 0x82f56fcc
	if !ctx.cr[6].gt {
	pc = 0x82F56FCC; continue 'dispatch;
	}
	// 82F56F5C: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82F56F60: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82F56F64: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82F56F68: 7C6B702E  lwzx r3, r11, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82F56F6C: 4BF497C5  bl 0x82ea0730
	ctx.lr = 0x82F56F70;
	sub_82EA0730(ctx, base);
	// 82F56F70: B3030004  sth r24, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[24].u16 ) };
	// 82F56F74: 4BF9428D  bl 0x82eeb200
	ctx.lr = 0x82F56F78;
	sub_82EEB200(ctx, base);
	// 82F56F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F56F7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F56F80: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F56F84: 394B44E8  addi r10, r11, 0x44e8
	ctx.r[10].s64 = ctx.r[11].s64 + 17640;
	// 82F56F88: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82F56F8C: 8139003C  lwz r9, 0x3c(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F56F90: 913F003C  stw r9, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82F56F94: 93F6000C  stw r31, 0xc(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82F56F98: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56F9C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F56FA0: 40990030  ble cr6, 0x82f56fd0
	if !ctx.cr[6].gt {
	pc = 0x82F56FD0; continue 'dispatch;
	}
	// 82F56FA4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F56FAC: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F56FB0: 4BF94839  bl 0x82eeb7e8
	ctx.lr = 0x82F56FB4;
	sub_82EEB7E8(ctx, base);
	// 82F56FB4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F56FB8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82F56FBC: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82F56FC0: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F56FC4: 4198FFE0  blt cr6, 0x82f56fa4
	if ctx.cr[6].lt {
	pc = 0x82F56FA4; continue 'dispatch;
	}
	// 82F56FC8: 48000008  b 0x82f56fd0
	pc = 0x82F56FD0; continue 'dispatch;
	// 82F56FCC: 9396000C  stw r28, 0xc(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82F56FD0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56FD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F56FD8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F56FDC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F56FE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F56FE4: 4E800421  bctrl
	ctx.lr = 0x82F56FE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F56FE8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82F56FEC: 48251194  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F56FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F56FF0 size=80
    let mut pc: u32 = 0x82F56FF0;
    'dispatch: loop {
        match pc {
            0x82F56FF0 => {
    //   block [0x82F56FF0..0x82F57040)
	// 82F56FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F56FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F56FF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F56FFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F57004: 4BF941FD  bl 0x82eeb200
	ctx.lr = 0x82F57008;
	sub_82EEB200(ctx, base);
	// 82F57008: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F5700C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F57010: 392A0EDC  addi r9, r10, 0xedc
	ctx.r[9].s64 = ctx.r[10].s64 + 3804;
	// 82F57014: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82F57018: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F5701C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F57020: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F57024: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82F57028: 911F004C  stw r8, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[8].u32 ) };
	// 82F5702C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F57030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F57034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F57038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F5703C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F57040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F57040 size=176
    let mut pc: u32 = 0x82F57040;
    'dispatch: loop {
        match pc {
            0x82F57040 => {
    //   block [0x82F57040..0x82F570F0)
	// 82F57040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F57044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F57048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F5704C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F57050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57054: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F57058: 117F038C  vspltisw v11, -1
	for i in 0..4 {
		ctx.v[11].u32[i] = 4294967295;
	}
	// 82F5705C: 3BC30030  addi r30, r3, 0x30
	ctx.r[30].s64 = ctx.r[3].s64 + 48;
	// 82F57060: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F57064: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F57068: 392BBD40  addi r9, r11, -0x42c0
	ctx.r[9].s64 = ctx.r[11].s64 + -17088;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F570F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F570F0 size=8
    let mut pc: u32 = 0x82F570F0;
    'dispatch: loop {
        match pc {
            0x82F570F0 => {
    //   block [0x82F570F0..0x82F570F8)
	// 82F570F0: D0230048  stfs f1, 0x48(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82F570F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F570F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F570F8 size=52
    let mut pc: u32 = 0x82F570F8;
    'dispatch: loop {
        match pc {
            0x82F570F8 => {
    //   block [0x82F570F8..0x82F5712C)
	// 82F570F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F570FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F57100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F57104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F5710C: 482AA505  bl 0x83201610
	ctx.lr = 0x82F57110;
	sub_83201610(ctx, base);
	// 82F57110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F57114: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82F57118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F5711C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F57120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F57124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F57128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F57130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F57130 size=184
    let mut pc: u32 = 0x82F57130;
    'dispatch: loop {
        match pc {
            0x82F57130 => {
    //   block [0x82F57130..0x82F571E8)
	// 82F57130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F57134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F57138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F5713C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57140: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F57144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F57148: 482AA569  bl 0x832016b0
	ctx.lr = 0x82F5714C;
	sub_832016B0(ctx, base);
	// 82F5714C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82F57150: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F57154: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F57158: 3CE08209  lis r7, -0x7df7
	ctx.r[7].s64 = -2113339392;
	// 82F5715C: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F57160: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F57164: C1A97BC8  lfs f13, 0x7bc8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(31688 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F57168: 38A610AC  addi r5, r6, 0x10ac
	ctx.r[5].s64 = ctx.r[6].s64 + 4268;
	// 82F5716C: C00A9450  lfs f0, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F57170: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F57174: C1889690  lfs f12, -0x6970(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-26992 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F57178: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 82F5717C: C167AD00  lfs f11, -0x5300(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-21248 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F57180: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82F57184: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82F57188: D1BF0044  stfs f13, 0x44(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82F5718C: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82F57190: D19F0048  stfs f12, 0x48(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82F57194: 3C608332  lis r3, -0x7cce
	ctx.r[3].s64 = -2093875200;
	// 82F57198: D17F004C  stfs f11, 0x4c(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82F5719C: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82F571A0: 913F0058  stw r9, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82F571A4: 390319CC  addi r8, r3, 0x19cc
	ctx.r[8].s64 = ctx.r[3].s64 + 6604;
	// 82F571A8: 909F005C  stw r4, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F571E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F571E8 size=684
    let mut pc: u32 = 0x82F571E8;
    'dispatch: loop {
        match pc {
            0x82F571E8 => {
    //   block [0x82F571E8..0x82F57494)
	// 82F571E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F571EC: 48250F6D  bl 0x831a8158
	ctx.lr = 0x82F571F0;
	sub_831A8130(ctx, base);
	// 82F571F0: DBA1FFA0  stfd f29, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[29].u64 ) };
	// 82F571F4: DBC1FFA8  stfd f30, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 82F571F8: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82F571FC: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57200: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 82F57204: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F57208: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F5720C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F57210: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82F57214: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82F57218: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F5721C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F57220: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F57224: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82F57228: 482AA489  bl 0x832016b0
	ctx.lr = 0x82F5722C;
	sub_832016B0(ctx, base);
	// 82F5722C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F57230: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82F57234: 394B10AC  addi r10, r11, 0x10ac
	ctx.r[10].s64 = ctx.r[11].s64 + 4268;
	// 82F57238: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82F5723C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F57240: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F57498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F57498 size=836
    let mut pc: u32 = 0x82F57498;
    'dispatch: loop {
        match pc {
            0x82F57498 => {
    //   block [0x82F57498..0x82F577DC)
	// 82F57498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5749C: 48250CC5  bl 0x831a8160
	ctx.lr = 0x82F574A0;
	sub_831A8130(ctx, base);
	// 82F574A0: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F577E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F577E0 size=340
    let mut pc: u32 = 0x82F577E0;
    'dispatch: loop {
        match pc {
            0x82F577E0 => {
    //   block [0x82F577E0..0x82F57934)
	// 82F577E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F577E4: 4825097D  bl 0x831a8160
	ctx.lr = 0x82F577E8;
	sub_831A8130(ctx, base);
	// 82F577E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F577EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F577F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F577F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F577F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F577FC: 419A0010  beq cr6, 0x82f5780c
	if ctx.cr[6].eq {
	pc = 0x82F5780C; continue 'dispatch;
	}
	// 82F57800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F57804: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F57808: 482509A8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82F5780C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F57810: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F57814: 409AFFEC  bne cr6, 0x82f57800
	if !ctx.cr[6].eq {
	pc = 0x82F57800; continue 'dispatch;
	}
	// 82F57818: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5781C: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82F57820: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82F57824: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82F57828: 7C7AD82E  lwzx r3, r26, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F5782C: 4BF48F05  bl 0x82ea0730
	ctx.lr = 0x82F57830;
	sub_82EA0730(ctx, base);
	// 82F57830: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82F57834: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82F57838: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F5783C: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F57840: 4BFFF8F1  bl 0x82f57130
	ctx.lr = 0x82F57844;
	sub_82F57130(ctx, base);
	// 82F57844: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F57848: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F57938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F57938 size=188
    let mut pc: u32 = 0x82F57938;
    'dispatch: loop {
        match pc {
            0x82F57938 => {
    //   block [0x82F57938..0x82F579F4)
	// 82F57938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F5793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F57940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F57944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F57948: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82F5794C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F57950: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57954: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F57958: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F5795C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F57960: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F57964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F57968: 482A9D49  bl 0x832016b0
	ctx.lr = 0x82F5796C;
	sub_832016B0(ctx, base);
	// 82F5796C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F57970: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F57974: 392A1164  addi r9, r10, 0x1164
	ctx.r[9].s64 = ctx.r[10].s64 + 4452;
	// 82F57978: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F5797C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F579F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F579F8 size=156
    let mut pc: u32 = 0x82F579F8;
    'dispatch: loop {
        match pc {
            0x82F579F8 => {
    //   block [0x82F579F8..0x82F57A94)
	// 82F579F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F579FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F57A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F57A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F57A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F57A0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F57A10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F57A14: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F57A18: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F57A1C: 419A000C  beq cr6, 0x82f57a28
	if ctx.cr[6].eq {
	pc = 0x82F57A28; continue 'dispatch;
	}
	// 82F57A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F57A24: 48000058  b 0x82f57a7c
	pc = 0x82F57A7C; continue 'dispatch;
	// 82F57A28: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F57A2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F57A30: 409AFFF0  bne cr6, 0x82f57a20
	if !ctx.cr[6].eq {
	pc = 0x82F57A20; continue 'dispatch;
	}
	// 82F57A34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F57A38: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F57A3C: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 82F57A40: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82F57A44: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F57A48: 4BF48CE9  bl 0x82ea0730
	ctx.lr = 0x82F57A4C;
	sub_82EA0730(ctx, base);
	// 82F57A4C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82F57A50: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82F57A54: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F57A58: C05F0034  lfs f2, 0x34(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82F57A5C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F57A60: C03F0030  lfs f1, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F57A64: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F57A68: 4BFFFED1  bl 0x82f57938
	ctx.lr = 0x82F57A6C;
	sub_82F57938(ctx, base);
	// 82F57A6C: 88FF0038  lbz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82F57A70: 98E30038  stb r7, 0x38(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[7].u8 ) };
	// 82F57A74: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F57A78: 90C30010  stw r6, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82F57A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F57A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F57A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F57A88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F57A8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F57A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


