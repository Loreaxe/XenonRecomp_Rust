pub fn sub_83187C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187C58 size=48
    let mut pc: u32 = 0x83187C58;
    'dispatch: loop {
        match pc {
            0x83187C58 => {
    //   block [0x83187C58..0x83187C88)
	// 83187C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83187C5C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83187C60: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83187C64: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83187C68: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83187C6C: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83187C70: 91440010  stw r10, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83187C74: 81631FC0  lwz r11, 0x1fc0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8128 as u32) ) } as u64;
	// 83187C78: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83187C7C: 81631FB8  lwz r11, 0x1fb8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83187C80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83187C84: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187C88 size=48
    let mut pc: u32 = 0x83187C88;
    'dispatch: loop {
        match pc {
            0x83187C88 => {
    //   block [0x83187C88..0x83187CB8)
	// 83187C88: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83187C8C: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83187C90: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83187C94: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 83187C98: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83187C9C: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83187CA0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83187CA4: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83187CA8: 81031FB8  lwz r8, 0x1fb8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83187CAC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83187CB0: 4198FFDC  blt cr6, 0x83187c8c
	if ctx.cr[6].lt {
	pc = 0x83187C8C; continue 'dispatch;
	}
	// 83187CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187CB8 size=60
    let mut pc: u32 = 0x83187CB8;
    'dispatch: loop {
        match pc {
            0x83187CB8 => {
    //   block [0x83187CB8..0x83187CF4)
	// 83187CB8: 81231FB8  lwz r9, 0x1fb8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83187CBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187CC0: 81031F78  lwz r8, 0x1f78(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83187CC4: 81431514  lwz r10, 0x1514(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5396 as u32) ) } as u64;
	// 83187CC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83187CCC: 40990020  ble cr6, 0x83187cec
	if !ctx.cr[6].gt {
	pc = 0x83187CEC; continue 'dispatch;
	}
	// 83187CD0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83187CD4: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83187CD8: 419A001C  beq cr6, 0x83187cf4
	if ctx.cr[6].eq {
		sub_83187CF4(ctx, base);
		return;
	}
	// 83187CDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83187CE0: 394A0088  addi r10, r10, 0x88
	ctx.r[10].s64 = ctx.r[10].s64 + 136;
	// 83187CE4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83187CE8: 4198FFEC  blt cr6, 0x83187cd4
	if ctx.cr[6].lt {
	pc = 0x83187CD4; continue 'dispatch;
	}
	// 83187CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187CF4 size=12
    let mut pc: u32 = 0x83187CF4;
    'dispatch: loop {
        match pc {
            0x83187CF4 => {
    //   block [0x83187CF4..0x83187D00)
	// 83187CF4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83187CF8: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 83187CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187D00 size=32
    let mut pc: u32 = 0x83187D00;
    'dispatch: loop {
        match pc {
            0x83187D00 => {
    //   block [0x83187D00..0x83187D20)
	// 83187D00: 81431FB8  lwz r10, 0x1fb8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83187D04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187D08: 80631F78  lwz r3, 0x1f78(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83187D0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83187D10: 40990020  ble cr6, 0x83187d30
	if !ctx.cr[6].gt {
		sub_83187D20(ctx, base);
		return;
	}
	// 83187D14: 81230064  lwz r9, 0x64(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 83187D18: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83187D1C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187D20 size=24
    let mut pc: u32 = 0x83187D20;
    'dispatch: loop {
        match pc {
            0x83187D20 => {
    //   block [0x83187D20..0x83187D38)
	// 83187D20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83187D24: 38630100  addi r3, r3, 0x100
	ctx.r[3].s64 = ctx.r[3].s64 + 256;
	// 83187D28: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187D2C: 4198FFE8  blt cr6, 0x83187d14
	if ctx.cr[6].lt {
		sub_83187D00(ctx, base);
		return;
	}
	// 83187D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187D38 size=52
    let mut pc: u32 = 0x83187D38;
    'dispatch: loop {
        match pc {
            0x83187D38 => {
    //   block [0x83187D38..0x83187D6C)
	// 83187D38: 81231F68  lwz r9, 0x1f68(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8040 as u32) ) } as u64;
	// 83187D3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187D40: 81431F78  lwz r10, 0x1f78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83187D44: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83187D48: 4099001C  ble cr6, 0x83187d64
	if !ctx.cr[6].gt {
	pc = 0x83187D64; continue 'dispatch;
	}
	// 83187D4C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83187D50: 419A001C  beq cr6, 0x83187d6c
	if ctx.cr[6].eq {
		sub_83187D6C(ctx, base);
		return;
	}
	// 83187D54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83187D58: 394A0100  addi r10, r10, 0x100
	ctx.r[10].s64 = ctx.r[10].s64 + 256;
	// 83187D5C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83187D60: 4198FFEC  blt cr6, 0x83187d4c
	if ctx.cr[6].lt {
	pc = 0x83187D4C; continue 'dispatch;
	}
	// 83187D64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83187D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187D6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187D6C size=16
    let mut pc: u32 = 0x83187D6C;
    'dispatch: loop {
        match pc {
            0x83187D6C => {
    //   block [0x83187D6C..0x83187D7C)
	// 83187D6C: 81431514  lwz r10, 0x1514(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(5396 as u32) ) } as u64;
	// 83187D70: 1D6B0088  mulli r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 * 136;
	// 83187D74: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83187D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187D80 size=52
    let mut pc: u32 = 0x83187D80;
    'dispatch: loop {
        match pc {
            0x83187D80 => {
    //   block [0x83187D80..0x83187DB4)
	// 83187D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187D8C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83187D90: 4BFFFFA9  bl 0x83187d38
	ctx.lr = 0x83187D94;
	sub_83187D38(ctx, base);
	// 83187D94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83187D98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187D9C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 83187DA0: 90881F74  stw r4, 0x1f74(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8052 as u32), ctx.r[4].u32 ) };
	// 83187DA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83187DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187DB8 size=8
    let mut pc: u32 = 0x83187DB8;
    'dispatch: loop {
        match pc {
            0x83187DB8 => {
    //   block [0x83187DB8..0x83187DC0)
	// 83187DB8: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 83187DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187DC0 size=12
    let mut pc: u32 = 0x83187DC0;
    'dispatch: loop {
        match pc {
            0x83187DC0 => {
    //   block [0x83187DC0..0x83187DCC)
	// 83187DC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83187DC4: 91631F6C  stw r11, 0x1f6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8044 as u32), ctx.r[11].u32 ) };
	// 83187DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187DD0 size=8
    let mut pc: u32 = 0x83187DD0;
    'dispatch: loop {
        match pc {
            0x83187DD0 => {
    //   block [0x83187DD0..0x83187DD8)
	// 83187DD0: 80631F6C  lwz r3, 0x1f6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8044 as u32) ) } as u64;
	// 83187DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187DD8 size=8
    let mut pc: u32 = 0x83187DD8;
    'dispatch: loop {
        match pc {
            0x83187DD8 => {
    //   block [0x83187DD8..0x83187DE0)
	// 83187DD8: 90831F70  stw r4, 0x1f70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8048 as u32), ctx.r[4].u32 ) };
	// 83187DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187DE0 size=168
    let mut pc: u32 = 0x83187DE0;
    'dispatch: loop {
        match pc {
            0x83187DE0 => {
    //   block [0x83187DE0..0x83187E88)
	// 83187DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187DE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83187DEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83187DF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187DF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187DF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83187DFC: 4BFFF5F5  bl 0x831873f0
	ctx.lr = 0x83187E00;
	sub_831873F0(ctx, base);
	// 83187E00: 815F1F68  lwz r10, 0x1f68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8040 as u32) ) } as u64;
	// 83187E04: 817F1F78  lwz r11, 0x1f78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83187E08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83187E0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83187E10: 4099003C  ble cr6, 0x83187e4c
	if !ctx.cr[6].gt {
	pc = 0x83187E4C; continue 'dispatch;
	}
	// 83187E14: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83187E18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187E1C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 83187E20: 419A000C  beq cr6, 0x83187e2c
	if ctx.cr[6].eq {
	pc = 0x83187E2C; continue 'dispatch;
	}
	// 83187E24: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83187E28: 409A0014  bne cr6, 0x83187e3c
	if !ctx.cr[6].eq {
	pc = 0x83187E3C; continue 'dispatch;
	}
	// 83187E2C: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 83187E30: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 83187E34: 409A0008  bne cr6, 0x83187e3c
	if !ctx.cr[6].eq {
	pc = 0x83187E3C; continue 'dispatch;
	}
	// 83187E38: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83187E3C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83187E40: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 83187E44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83187E48: 409AFFD0  bne cr6, 0x83187e18
	if !ctx.cr[6].eq {
	pc = 0x83187E18; continue 'dispatch;
	}
	// 83187E4C: 817F1F6C  lwz r11, 0x1f6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8044 as u32) ) } as u64;
	// 83187E50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83187E54: 409A0010  bne cr6, 0x83187e64
	if !ctx.cr[6].eq {
	pc = 0x83187E64; continue 'dispatch;
	}
	// 83187E58: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83187E5C: 409A0008  bne cr6, 0x83187e64
	if !ctx.cr[6].eq {
	pc = 0x83187E64; continue 'dispatch;
	}
	// 83187E60: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83187E64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83187E68: 4BFFF599  bl 0x83187400
	ctx.lr = 0x83187E6C;
	sub_83187400(ctx, base);
	// 83187E6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83187E70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83187E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187E7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83187E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83187E88 size=144
    let mut pc: u32 = 0x83187E88;
    'dispatch: loop {
        match pc {
            0x83187E88 => {
    //   block [0x83187E88..0x83187F18)
	// 83187E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83187E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83187E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83187E94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83187E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83187E9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83187EA0: 4BFFF551  bl 0x831873f0
	ctx.lr = 0x83187EA4;
	sub_831873F0(ctx, base);
	// 83187EA4: 815F1F68  lwz r10, 0x1f68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8040 as u32) ) } as u64;
	// 83187EA8: 83FF1F78  lwz r31, 0x1f78(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8056 as u32) ) } as u64;
	// 83187EAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187EB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83187EB4: 40990038  ble cr6, 0x83187eec
	if !ctx.cr[6].gt {
	pc = 0x83187EEC; continue 'dispatch;
	}
	// 83187EB8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187EBC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83187EC0: 409A0010  bne cr6, 0x83187ed0
	if !ctx.cr[6].eq {
	pc = 0x83187ED0; continue 'dispatch;
	}
	// 83187EC4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83187EC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83187ECC: 419A0018  beq cr6, 0x83187ee4
	if ctx.cr[6].eq {
	pc = 0x83187EE4; continue 'dispatch;
	}
	// 83187ED0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83187ED4: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 83187ED8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187EDC: 4198FFDC  blt cr6, 0x83187eb8
	if ctx.cr[6].lt {
	pc = 0x83187EB8; continue 'dispatch;
	}
	// 83187EE0: 4800000C  b 0x83187eec
	pc = 0x83187EEC; continue 'dispatch;
	// 83187EE4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83187EE8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83187EEC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187EF0: 409A0008  bne cr6, 0x83187ef8
	if !ctx.cr[6].eq {
	pc = 0x83187EF8; continue 'dispatch;
	}
	// 83187EF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83187EF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83187EFC: 4BFFF505  bl 0x83187400
	ctx.lr = 0x83187F00;
	sub_83187400(ctx, base);
	// 83187F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83187F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83187F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83187F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83187F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83187F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F18 size=8
    let mut pc: u32 = 0x83187F18;
    'dispatch: loop {
        match pc {
            0x83187F18 => {
    //   block [0x83187F18..0x83187F20)
	// 83187F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187F1C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F20 size=12
    let mut pc: u32 = 0x83187F20;
    'dispatch: loop {
        match pc {
            0x83187F20 => {
    //   block [0x83187F20..0x83187F2C)
	// 83187F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187F24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F30 size=8
    let mut pc: u32 = 0x83187F30;
    'dispatch: loop {
        match pc {
            0x83187F30 => {
    //   block [0x83187F30..0x83187F38)
	// 83187F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187F34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F38 size=12
    let mut pc: u32 = 0x83187F38;
    'dispatch: loop {
        match pc {
            0x83187F38 => {
    //   block [0x83187F38..0x83187F44)
	// 83187F38: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83187F3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F48 size=8
    let mut pc: u32 = 0x83187F48;
    'dispatch: loop {
        match pc {
            0x83187F48 => {
    //   block [0x83187F48..0x83187F50)
	// 83187F48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187F4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F50 size=12
    let mut pc: u32 = 0x83187F50;
    'dispatch: loop {
        match pc {
            0x83187F50 => {
    //   block [0x83187F50..0x83187F5C)
	// 83187F50: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83187F54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F60 size=8
    let mut pc: u32 = 0x83187F60;
    'dispatch: loop {
        match pc {
            0x83187F60 => {
    //   block [0x83187F60..0x83187F68)
	// 83187F60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187F64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F68 size=36
    let mut pc: u32 = 0x83187F68;
    'dispatch: loop {
        match pc {
            0x83187F68 => {
    //   block [0x83187F68..0x83187F8C)
	// 83187F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187F6C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83187F70: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83187F74: 419A0008  beq cr6, 0x83187f7c
	if ctx.cr[6].eq {
	pc = 0x83187F7C; continue 'dispatch;
	}
	// 83187F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83187F7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187F80: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83187F84: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83187F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F90 size=8
    let mut pc: u32 = 0x83187F90;
    'dispatch: loop {
        match pc {
            0x83187F90 => {
    //   block [0x83187F90..0x83187F98)
	// 83187F90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187F94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187F98 size=24
    let mut pc: u32 = 0x83187F98;
    'dispatch: loop {
        match pc {
            0x83187F98 => {
    //   block [0x83187F98..0x83187FB0)
	// 83187F98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83187F9C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83187FA0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83187FA4: 556BE7BC  rlwinm r11, r11, 0x1c, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83187FA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83187FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187FB0 size=16
    let mut pc: u32 = 0x83187FB0;
    'dispatch: loop {
        match pc {
            0x83187FB0 => {
    //   block [0x83187FB0..0x83187FC0)
	// 83187FB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83187FB4: 409A000C  bne cr6, 0x83187fc0
	if !ctx.cr[6].eq {
		sub_83187FC0(ctx, base);
		return;
	}
	// 83187FB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83187FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83187FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83187FC0 size=80
    let mut pc: u32 = 0x83187FC0;
    'dispatch: loop {
        match pc {
            0x83187FC0 => {
    //   block [0x83187FC0..0x83188010)
	// 83187FC0: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 83187FC4: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 83187FC8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187FCC: 4198FFEC  blt cr6, 0x83187fb8
	if ctx.cr[6].lt {
		sub_83187FB0(ctx, base);
		return;
	}
	// 83187FD0: 41990040  bgt cr6, 0x83188010
	if ctx.cr[6].gt {
		sub_83188010(ctx, base);
		return;
	}
	// 83187FD4: 816400F0  lwz r11, 0xf0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(240 as u32) ) } as u64;
	// 83187FD8: 814300F0  lwz r10, 0xf0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 83187FDC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187FE0: 4198FFD8  blt cr6, 0x83187fb8
	if ctx.cr[6].lt {
		sub_83187FB0(ctx, base);
		return;
	}
	// 83187FE4: 4199002C  bgt cr6, 0x83188010
	if ctx.cr[6].gt {
		sub_83188010(ctx, base);
		return;
	}
	// 83187FE8: 816400F4  lwz r11, 0xf4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(244 as u32) ) } as u64;
	// 83187FEC: 814300F4  lwz r10, 0xf4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(244 as u32) ) } as u64;
	// 83187FF0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83187FF4: 4198FFC4  blt cr6, 0x83187fb8
	if ctx.cr[6].lt {
		sub_83187FB0(ctx, base);
		return;
	}
	// 83187FF8: 41990018  bgt cr6, 0x83188010
	if ctx.cr[6].gt {
		sub_83188010(ctx, base);
		return;
	}
	// 83187FFC: 814300F8  lwz r10, 0xf8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 83188000: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83188004: 816400F8  lwz r11, 0xf8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(248 as u32) ) } as u64;
	// 83188008: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8318800C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188010 size=8
    let mut pc: u32 = 0x83188010;
    'dispatch: loop {
        match pc {
            0x83188010 => {
    //   block [0x83188010..0x83188018)
	// 83188010: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188018 size=20
    let mut pc: u32 = 0x83188018;
    'dispatch: loop {
        match pc {
            0x83188018 => {
    //   block [0x83188018..0x8318802C)
	// 83188018: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318801C: 806B0060  lwz r3, 0x60(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 83188020: 35430001  addic. r10, r3, 1
	ctx.xer.ca = (ctx.r[3].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83188024: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 83188028: 4C800020  bgelr
	if !ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318802C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318802C size=12
    let mut pc: u32 = 0x8318802C;
    'dispatch: loop {
        match pc {
            0x8318802C => {
    //   block [0x8318802C..0x83188038)
	// 8318802C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83188030: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 83188034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188038 size=120
    let mut pc: u32 = 0x83188038;
    'dispatch: loop {
        match pc {
            0x83188038 => {
    //   block [0x83188038..0x831880B0)
	// 83188038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318803C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188044: 81630910  lwz r11, 0x910(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2320 as u32) ) } as u64;
	// 83188048: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318804C: 4099002C  ble cr6, 0x83188078
	if !ctx.cr[6].gt {
	pc = 0x83188078; continue 'dispatch;
	}
	// 83188050: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83188054: 409A0010  bne cr6, 0x83188064
	if !ctx.cr[6].eq {
	pc = 0x83188064; continue 'dispatch;
	}
	// 83188058: 81630914  lwz r11, 0x914(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2324 as u32) ) } as u64;
	// 8318805C: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83188060: 419A0018  beq cr6, 0x83188078
	if ctx.cr[6].eq {
	pc = 0x83188078; continue 'dispatch;
	}
	// 83188064: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83188068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318806C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188074: 4E800020  blr
	return;
	// 83188078: 81630D6C  lwz r11, 0xd6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3436 as u32) ) } as u64;
	// 8318807C: 80630D70  lwz r3, 0xd70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3440 as u32) ) } as u64;
	// 83188080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83188084: 419A0018  beq cr6, 0x8318809c
	if ctx.cr[6].eq {
	pc = 0x8318809C; continue 'dispatch;
	}
	// 83188088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318808C: 4E800421  bctrl
	ctx.lr = 0x83188090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83188090: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188094: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83188098: 409A0008  bne cr6, 0x831880a0
	if !ctx.cr[6].eq {
	pc = 0x831880A0; continue 'dispatch;
	}
	// 8318809C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831880A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831880A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831880A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831880AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831880B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831880B0 size=232
    let mut pc: u32 = 0x831880B0;
    'dispatch: loop {
        match pc {
            0x831880B0 => {
    //   block [0x831880B0..0x83188198)
	// 831880B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831880B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831880B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831880BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831880C0: 39631F7C  addi r11, r3, 0x1f7c
	ctx.r[11].s64 = ctx.r[3].s64 + 8060;
	// 831880C4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831880C8: 419A00B0  beq cr6, 0x83188178
	if ctx.cr[6].eq {
	pc = 0x83188178; continue 'dispatch;
	}
	// 831880CC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 831880D0: 419A00A8  beq cr6, 0x83188178
	if ctx.cr[6].eq {
	pc = 0x83188178; continue 'dispatch;
	}
	// 831880D4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831880D8: 419A00A0  beq cr6, 0x83188178
	if ctx.cr[6].eq {
	pc = 0x83188178; continue 'dispatch;
	}
	// 831880DC: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 831880E0: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 831880E4: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831880E8: 40980024  bge cr6, 0x8318810c
	if !ctx.cr[6].lt {
	pc = 0x8318810C; continue 'dispatch;
	}
	// 831880EC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831880F0: 60840F1D  ori r4, r4, 0xf1d
	ctx.r[4].u64 = ctx.r[4].u64 | 3869;
	// 831880F4: 4BFFF405  bl 0x831874f8
	ctx.lr = 0x831880F8;
	sub_831874F8(ctx, base);
	// 831880F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831880FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188108: 4E800020  blr
	return;
	// 8318810C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83188110: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83188114: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83188118: 7D243214  add r9, r4, r6
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[6].u64;
	// 8318811C: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83188120: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 83188124: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 83188128: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8318812C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83188130: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83188134: 80E31FC0  lwz r7, 0x1fc0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8128 as u32) ) } as u64;
	// 83188138: 90EB0014  stw r7, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8318813C: 80E31FB8  lwz r7, 0x1fb8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8120 as u32) ) } as u64;
	// 83188140: 7F053800  cmpw cr6, r5, r7
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83188144: 40980008  bge cr6, 0x8318814c
	if !ctx.cr[6].lt {
	pc = 0x8318814C; continue 'dispatch;
	}
	// 83188148: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 8318814C: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83188150: 40980030  bge cr6, 0x83188180
	if !ctx.cr[6].lt {
	pc = 0x83188180; continue 'dispatch;
	}
	// 83188154: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83188158: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8318815C: 7D27512E  stwx r9, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 83188160: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83188164: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83188168: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 8318816C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83188170: 93E70004  stw r31, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83188174: 4BFFFFC8  b 0x8318813c
	pc = 0x8318813C; continue 'dispatch;
	// 83188178: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8318817C: 4BFFFADD  bl 0x83187c58
	ctx.lr = 0x83188180;
	sub_83187C58(ctx, base);
	// 83188180: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83188188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318818C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188198 size=228
    let mut pc: u32 = 0x83188198;
    'dispatch: loop {
        match pc {
            0x83188198 => {
    //   block [0x83188198..0x8318827C)
	// 83188198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318819C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831881A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831881A4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 831881A8: 8167005C  lwz r11, 0x5c(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(92 as u32) ) } as u64;
	// 831881AC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831881B0: 409A0064  bne cr6, 0x83188214
	if !ctx.cr[6].eq {
	pc = 0x83188214; continue 'dispatch;
	}
	// 831881B4: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 831881B8: 4BFFFB49  bl 0x83187d00
	ctx.lr = 0x831881BC;
	sub_83187D00(ctx, base);
	// 831881BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831881C0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 831881C4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831881C8: 409A0020  bne cr6, 0x831881e8
	if !ctx.cr[6].eq {
	pc = 0x831881E8; continue 'dispatch;
	}
	// 831881CC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831881D0: 60840F1F  ori r4, r4, 0xf1f
	ctx.r[4].u64 = ctx.r[4].u64 | 3871;
	// 831881D4: 4BFFF325  bl 0x831874f8
	ctx.lr = 0x831881D8;
	sub_831874F8(ctx, base);
	// 831881D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831881DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831881E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831881E4: 4E800020  blr
	return;
	// 831881E8: 4BFFFB51  bl 0x83187d38
	ctx.lr = 0x831881EC;
	sub_83187D38(ctx, base);
	// 831881EC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 831881F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831881F4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831881F8: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831881FC: 4BFFFD65  bl 0x83187f60
	ctx.lr = 0x83188200;
	sub_83187F60(ctx, base);
	// 83188200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83188208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318820C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188210: 4E800020  blr
	return;
	// 83188214: 4BFFFBA5  bl 0x83187db8
	ctx.lr = 0x83188218;
	sub_83187DB8(ctx, base);
	// 83188218: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8318821C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83188220: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188224: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83188228: 419A0020  beq cr6, 0x83188248
	if ctx.cr[6].eq {
	pc = 0x83188248; continue 'dispatch;
	}
	// 8318822C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83188230: 60840F0E  ori r4, r4, 0xf0e
	ctx.r[4].u64 = ctx.r[4].u64 | 3854;
	// 83188234: 4BFFF2C5  bl 0x831874f8
	ctx.lr = 0x83188238;
	sub_831874F8(ctx, base);
	// 83188238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318823C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188244: 4E800020  blr
	return;
	// 83188248: 4BFFFA71  bl 0x83187cb8
	ctx.lr = 0x8318824C;
	sub_83187CB8(ctx, base);
	// 8318824C: 81671F74  lwz r11, 0x1f74(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8052 as u32) ) } as u64;
	// 83188250: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83188254: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83188258: 419AFF98  beq cr6, 0x831881f0
	if ctx.cr[6].eq {
	pc = 0x831881F0; continue 'dispatch;
	}
	// 8318825C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83188260: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83188264: 60840F0F  ori r4, r4, 0xf0f
	ctx.r[4].u64 = ctx.r[4].u64 | 3855;
	// 83188268: 4BFFF291  bl 0x831874f8
	ctx.lr = 0x8318826C;
	sub_831874F8(ctx, base);
	// 8318826C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83188270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188280 size=204
    let mut pc: u32 = 0x83188280;
    'dispatch: loop {
        match pc {
            0x83188280 => {
    //   block [0x83188280..0x8318834C)
	// 83188280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318828C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83188290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83188298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318829C: 4BFFF155  bl 0x831873f0
	ctx.lr = 0x831882A0;
	sub_831873F0(ctx, base);
	// 831882A0: 817E1F68  lwz r11, 0x1f68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8040 as u32) ) } as u64;
	// 831882A4: 809E1F78  lwz r4, 0x1f78(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8056 as u32) ) } as u64;
	// 831882A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831882AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831882B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831882B4: 40990074  ble cr6, 0x83188328
	if !ctx.cr[6].gt {
	pc = 0x83188328; continue 'dispatch;
	}
	// 831882B8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 831882BC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831882C0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831882C4: 419A000C  beq cr6, 0x831882d0
	if ctx.cr[6].eq {
	pc = 0x831882D0; continue 'dispatch;
	}
	// 831882C8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831882CC: 409A0028  bne cr6, 0x831882f4
	if !ctx.cr[6].eq {
	pc = 0x831882F4; continue 'dispatch;
	}
	// 831882D0: 81640064  lwz r11, 0x64(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) } as u64;
	// 831882D4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831882D8: 409A001C  bne cr6, 0x831882f4
	if !ctx.cr[6].eq {
	pc = 0x831882F4; continue 'dispatch;
	}
	// 831882DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831882E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831882E4: 4BFFFCCD  bl 0x83187fb0
	ctx.lr = 0x831882E8;
	sub_83187FB0(ctx, base);
	// 831882E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831882EC: 419A0008  beq cr6, 0x831882f4
	if ctx.cr[6].eq {
	pc = 0x831882F4; continue 'dispatch;
	}
	// 831882F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831882F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 831882F8: 38840100  addi r4, r4, 0x100
	ctx.r[4].s64 = ctx.r[4].s64 + 256;
	// 831882FC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83188300: 409AFFBC  bne cr6, 0x831882bc
	if !ctx.cr[6].eq {
	pc = 0x831882BC; continue 'dispatch;
	}
	// 83188304: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 83188308: 409A0020  bne cr6, 0x83188328
	if !ctx.cr[6].eq {
	pc = 0x83188328; continue 'dispatch;
	}
	// 8318830C: 817E1F6C  lwz r11, 0x1f6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8044 as u32) ) } as u64;
	// 83188310: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83188314: 409A0014  bne cr6, 0x83188328
	if !ctx.cr[6].eq {
	pc = 0x83188328; continue 'dispatch;
	}
	// 83188318: 817E1F70  lwz r11, 0x1f70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8048 as u32) ) } as u64;
	// 8318831C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83188320: 409A0008  bne cr6, 0x83188328
	if !ctx.cr[6].eq {
	pc = 0x83188328; continue 'dispatch;
	}
	// 83188324: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83188328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318832C: 4BFFF0D5  bl 0x83187400
	ctx.lr = 0x83188330;
	sub_83187400(ctx, base);
	// 83188330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83188338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318833C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83188344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188350 size=236
    let mut pc: u32 = 0x83188350;
    'dispatch: loop {
        match pc {
            0x83188350 => {
    //   block [0x83188350..0x8318843C)
	// 83188350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318835C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83188360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188364: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83188368: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8318836C: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83188370: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83188374: 81661F68  lwz r11, 0x1f68(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8040 as u32) ) } as u64;
	// 83188378: 80861F78  lwz r4, 0x1f78(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8056 as u32) ) } as u64;
	// 8318837C: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83188380: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83188384: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83188388: 40990070  ble cr6, 0x831883f8
	if !ctx.cr[6].gt {
	pc = 0x831883F8; continue 'dispatch;
	}
	// 8318838C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83188390: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188394: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83188398: 419A000C  beq cr6, 0x831883a4
	if ctx.cr[6].eq {
	pc = 0x831883A4; continue 'dispatch;
	}
	// 8318839C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831883A0: 409A0048  bne cr6, 0x831883e8
	if !ctx.cr[6].eq {
	pc = 0x831883E8; continue 'dispatch;
	}
	// 831883A4: 81640064  lwz r11, 0x64(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) } as u64;
	// 831883A8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 831883AC: 409A003C  bne cr6, 0x831883e8
	if !ctx.cr[6].eq {
	pc = 0x831883E8; continue 'dispatch;
	}
	// 831883B0: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 831883B4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 831883B8: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 831883BC: 4BFFFBF5  bl 0x83187fb0
	ctx.lr = 0x831883C0;
	sub_83187FB0(ctx, base);
	// 831883C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831883C4: 419A0010  beq cr6, 0x831883d4
	if ctx.cr[6].eq {
	pc = 0x831883D4; continue 'dispatch;
	}
	// 831883C8: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831883CC: 90880000  stw r4, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831883D0: 48000018  b 0x831883e8
	pc = 0x831883E8; continue 'dispatch;
	// 831883D4: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 831883D8: 4BFFFBD9  bl 0x83187fb0
	ctx.lr = 0x831883DC;
	sub_83187FB0(ctx, base);
	// 831883DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831883E0: 419A0008  beq cr6, 0x831883e8
	if ctx.cr[6].eq {
	pc = 0x831883E8; continue 'dispatch;
	}
	// 831883E4: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831883E8: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 831883EC: 38840100  addi r4, r4, 0x100
	ctx.r[4].s64 = ctx.r[4].s64 + 256;
	// 831883F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831883F4: 409AFF9C  bne cr6, 0x83188390
	if !ctx.cr[6].eq {
	pc = 0x83188390; continue 'dispatch;
	}
	// 831883F8: 81661F6C  lwz r11, 0x1f6c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8044 as u32) ) } as u64;
	// 831883FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83188400: 409A0008  bne cr6, 0x83188408
	if !ctx.cr[6].eq {
	pc = 0x83188408; continue 'dispatch;
	}
	// 83188404: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 83188408: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8318840C: 4199000C  bgt cr6, 0x83188418
	if ctx.cr[6].gt {
	pc = 0x83188418; continue 'dispatch;
	}
	// 83188410: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83188414: 4800000C  b 0x83188420
	pc = 0x83188420; continue 'dispatch;
	// 83188418: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 8318841C: 409A0008  bne cr6, 0x83188424
	if !ctx.cr[6].eq {
	pc = 0x83188424; continue 'dispatch;
	}
	// 83188420: 93C50000  stw r30, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83188424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83188428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318842C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188430: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83188434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188440 size=92
    let mut pc: u32 = 0x83188440;
    'dispatch: loop {
        match pc {
            0x83188440 => {
    //   block [0x83188440..0x8318849C)
	// 83188440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188444: 4801FD25  bl 0x831a8168
	ctx.lr = 0x83188448;
	sub_831A8130(ctx, base);
	// 83188448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318844C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83188450: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83188454: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83188458: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8318845C: 4BFFEE85  bl 0x831872e0
	ctx.lr = 0x83188460;
	sub_831872E0(ctx, base);
	// 83188460: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188464: 419A001C  beq cr6, 0x83188480
	if ctx.cr[6].eq {
	pc = 0x83188480; continue 'dispatch;
	}
	// 83188468: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318846C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188470: 60840185  ori r4, r4, 0x185
	ctx.r[4].u64 = ctx.r[4].u64 | 389;
	// 83188474: 4BFFF085  bl 0x831874f8
	ctx.lr = 0x83188478;
	sub_831874F8(ctx, base);
	// 83188478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318847C: 4801FD3C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83188480: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83188484: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83188488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318848C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188490: 4BFFFC21  bl 0x831880b0
	ctx.lr = 0x83188494;
	sub_831880B0(ctx, base);
	// 83188494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83188498: 4801FD20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831884A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831884A0 size=24
    let mut pc: u32 = 0x831884A0;
    'dispatch: loop {
        match pc {
            0x831884A0 => {
    //   block [0x831884A0..0x831884B8)
	// 831884A0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831884A4: 396BC6AC  addi r11, r11, -0x3954
	ctx.r[11].s64 = ctx.r[11].s64 + -14676;
	// 831884A8: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831884AC: 80AB0038  lwz r5, 0x38(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 831884B0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831884B4: 4BFFFBFC  b 0x831880b0
	sub_831880B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831884B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831884B8 size=196
    let mut pc: u32 = 0x831884B8;
    'dispatch: loop {
        match pc {
            0x831884B8 => {
    //   block [0x831884B8..0x8318857C)
	// 831884B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831884BC: 4801FCA9  bl 0x831a8164
	ctx.lr = 0x831884C0;
	sub_831A8130(ctx, base);
	// 831884C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831884C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831884C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831884CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831884D0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831884D4: 4BFFFDAD  bl 0x83188280
	ctx.lr = 0x831884D8;
	sub_83188280(ctx, base);
	// 831884D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831884DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831884E0: 409A0018  bne cr6, 0x831884f8
	if !ctx.cr[6].eq {
	pc = 0x831884F8; continue 'dispatch;
	}
	// 831884E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831884E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831884EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831884F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831884F4: 4801FCC0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831884F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831884FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188500: 4BFFF881  bl 0x83187d80
	ctx.lr = 0x83188504;
	sub_83187D80(ctx, base);
	// 83188504: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83188508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318850C: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83188510: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 83188514: 4E800421  bctrl
	ctx.lr = 0x83188518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83188518: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318851C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188520: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83188524: 917F1008  stw r11, 0x1008(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4104 as u32), ctx.r[11].u32 ) };
	// 83188528: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318852C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83188530: 917F100C  stw r11, 0x100c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4108 as u32), ctx.r[11].u32 ) };
	// 83188534: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188538: 4BFF4909  bl 0x8317ce40
	ctx.lr = 0x8318853C;
	sub_8317CE40(ctx, base);
	// 8318853C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188540: 419AFFA4  beq cr6, 0x831884e4
	if ctx.cr[6].eq {
	pc = 0x831884E4; continue 'dispatch;
	}
	// 83188544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188548: 4BFF3CB1  bl 0x8317c1f8
	ctx.lr = 0x8318854C;
	sub_8317C1F8(ctx, base);
	// 8318854C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188550: 419AFF94  beq cr6, 0x831884e4
	if ctx.cr[6].eq {
	pc = 0x831884E4; continue 'dispatch;
	}
	// 83188554: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83188558: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8318855C: 409A0014  bne cr6, 0x83188570
	if !ctx.cr[6].eq {
	pc = 0x83188570; continue 'dispatch;
	}
	// 83188560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188564: 4BFFFAB5  bl 0x83188018
	ctx.lr = 0x83188568;
	sub_83188018(ctx, base);
	// 83188568: 907D0064  stw r3, 0x64(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8318856C: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83188570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83188578: 4801FC3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188580 size=72
    let mut pc: u32 = 0x83188580;
    'dispatch: loop {
        match pc {
            0x83188580 => {
    //   block [0x83188580..0x831885C8)
	// 83188580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188584: 4801FBE9  bl 0x831a816c
	ctx.lr = 0x83188588;
	sub_831A8130(ctx, base);
	// 83188588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318858C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83188590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83188594: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83188598: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318859C: 4BFFEE55  bl 0x831873f0
	ctx.lr = 0x831885A0;
	sub_831873F0(ctx, base);
	// 831885A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831885A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831885A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831885AC: 4BFFFBED  bl 0x83188198
	ctx.lr = 0x831885B0;
	sub_83188198(ctx, base);
	// 831885B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831885B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831885B8: 4BFFEE49  bl 0x83187400
	ctx.lr = 0x831885BC;
	sub_83187400(ctx, base);
	// 831885BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831885C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831885C4: 4801FBF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831885C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831885C8 size=156
    let mut pc: u32 = 0x831885C8;
    'dispatch: loop {
        match pc {
            0x831885C8 => {
    //   block [0x831885C8..0x83188664)
	// 831885C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831885CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831885D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831885D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831885D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831885DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831885E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 831885E4: 4BFFEE0D  bl 0x831873f0
	ctx.lr = 0x831885E8;
	sub_831873F0(ctx, base);
	// 831885E8: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831885EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831885F0: 409A004C  bne cr6, 0x8318863c
	if !ctx.cr[6].eq {
	pc = 0x8318863C; continue 'dispatch;
	}
	// 831885F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831885F8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831885FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188600: 4BFFFD51  bl 0x83188350
	ctx.lr = 0x83188604;
	sub_83188350(ctx, base);
	// 83188604: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83188608: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318860C: 419A0034  beq cr6, 0x83188640
	if ctx.cr[6].eq {
	pc = 0x83188640; continue 'dispatch;
	}
	// 83188610: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83188614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188618: 4BFF0DF9  bl 0x83179410
	ctx.lr = 0x8318861C;
	sub_83179410(ctx, base);
	// 8318861C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188620: 419A0020  beq cr6, 0x83188640
	if ctx.cr[6].eq {
	pc = 0x83188640; continue 'dispatch;
	}
	// 83188624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188628: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318862C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83188630: 4BFF46A9  bl 0x8317ccd8
	ctx.lr = 0x83188634;
	sub_8317CCD8(ctx, base);
	// 83188634: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188638: 409A0008  bne cr6, 0x83188640
	if !ctx.cr[6].eq {
	pc = 0x83188640; continue 'dispatch;
	}
	// 8318863C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83188640: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83188644: 4BFFEDBD  bl 0x83187400
	ctx.lr = 0x83188648;
	sub_83187400(ctx, base);
	// 83188648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318864C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83188650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188658: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318865C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188668 size=112
    let mut pc: u32 = 0x83188668;
    'dispatch: loop {
        match pc {
            0x83188668 => {
    //   block [0x83188668..0x831886D8)
	// 83188668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318866C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83188674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318867C: 4BFFEC65  bl 0x831872e0
	ctx.lr = 0x83188680;
	sub_831872E0(ctx, base);
	// 83188680: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188684: 419A002C  beq cr6, 0x831886b0
	if ctx.cr[6].eq {
	pc = 0x831886B0; continue 'dispatch;
	}
	// 83188688: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8318868C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188690: 60840183  ori r4, r4, 0x183
	ctx.r[4].u64 = ctx.r[4].u64 | 387;
	// 83188694: 4BFFEE65  bl 0x831874f8
	ctx.lr = 0x83188698;
	sub_831874F8(ctx, base);
	// 83188698: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318869C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831886A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831886A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831886A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831886AC: 4E800020  blr
	return;
	// 831886B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831886B4: 4BFFFF15  bl 0x831885c8
	ctx.lr = 0x831886B8;
	sub_831885C8(ctx, base);
	// 831886B8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 831886BC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831886C0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831886C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831886C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831886CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831886D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831886D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831886D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831886D8 size=12
    let mut pc: u32 = 0x831886D8;
    'dispatch: loop {
        match pc {
            0x831886D8 => {
    //   block [0x831886D8..0x831886E4)
	// 831886D8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831886DC: 386BB5B4  addi r3, r11, -0x4a4c
	ctx.r[3].s64 = ctx.r[11].s64 + -19020;
	// 831886E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831886E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831886E8 size=52
    let mut pc: u32 = 0x831886E8;
    'dispatch: loop {
        match pc {
            0x831886E8 => {
    //   block [0x831886E8..0x8318871C)
	// 831886E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831886EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831886F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831886F4: 4800C07D  bl 0x83194770
	ctx.lr = 0x831886F8;
	sub_83194770(ctx, base);
	// 831886F8: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831886FC: 396BC6F0  addi r11, r11, -0x3910
	ctx.r[11].s64 = ctx.r[11].s64 + -14608;
	// 83188700: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83188704: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83188708: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8318870C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83188710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188720 size=16
    let mut pc: u32 = 0x83188720;
    'dispatch: loop {
        match pc {
            0x83188720 => {
    //   block [0x83188720..0x83188730)
	// 83188720: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188724: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188728: 40980008  bge cr6, 0x83188730
	if !ctx.cr[6].lt {
		sub_83188730(ctx, base);
		return;
	}
	// 8318872C: 4800DD14  b 0x83196440
	sub_83196440(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188730 size=12
    let mut pc: u32 = 0x83188730;
    'dispatch: loop {
        match pc {
            0x83188730 => {
    //   block [0x83188730..0x8318873C)
	// 83188730: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188734: 40980008  bge cr6, 0x8318873c
	if !ctx.cr[6].lt {
		sub_8318873C(ctx, base);
		return;
	}
	// 83188738: 4800D650  b 0x83195d88
	sub_83195D88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318873C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318873C size=8
    let mut pc: u32 = 0x8318873C;
    'dispatch: loop {
        match pc {
            0x8318873C => {
    //   block [0x8318873C..0x83188744)
	// 8318873C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188748 size=16
    let mut pc: u32 = 0x83188748;
    'dispatch: loop {
        match pc {
            0x83188748 => {
    //   block [0x83188748..0x83188758)
	// 83188748: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318874C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188750: 40980008  bge cr6, 0x83188758
	if !ctx.cr[6].lt {
		sub_83188758(ctx, base);
		return;
	}
	// 83188754: 4800E574  b 0x83196cc8
	sub_83196CC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188758 size=12
    let mut pc: u32 = 0x83188758;
    'dispatch: loop {
        match pc {
            0x83188758 => {
    //   block [0x83188758..0x83188764)
	// 83188758: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8318875C: 40980008  bge cr6, 0x83188764
	if !ctx.cr[6].lt {
		sub_83188764(ctx, base);
		return;
	}
	// 83188760: 4800D668  b 0x83195dc8
	sub_83195DC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188764 size=8
    let mut pc: u32 = 0x83188764;
    'dispatch: loop {
        match pc {
            0x83188764 => {
    //   block [0x83188764..0x8318876C)
	// 83188764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188770 size=16
    let mut pc: u32 = 0x83188770;
    'dispatch: loop {
        match pc {
            0x83188770 => {
    //   block [0x83188770..0x83188780)
	// 83188770: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188774: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188778: 40980008  bge cr6, 0x83188780
	if !ctx.cr[6].lt {
		sub_83188780(ctx, base);
		return;
	}
	// 8318877C: 4800DE24  b 0x831965a0
	sub_831965A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188780 size=12
    let mut pc: u32 = 0x83188780;
    'dispatch: loop {
        match pc {
            0x83188780 => {
    //   block [0x83188780..0x8318878C)
	// 83188780: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188784: 40980008  bge cr6, 0x8318878c
	if !ctx.cr[6].lt {
		sub_8318878C(ctx, base);
		return;
	}
	// 83188788: 4800D6A8  b 0x83195e30
	sub_83195E30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318878C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318878C size=8
    let mut pc: u32 = 0x8318878C;
    'dispatch: loop {
        match pc {
            0x8318878C => {
    //   block [0x8318878C..0x83188794)
	// 8318878C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188798 size=16
    let mut pc: u32 = 0x83188798;
    'dispatch: loop {
        match pc {
            0x83188798 => {
    //   block [0x83188798..0x831887A8)
	// 83188798: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318879C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831887A0: 40980008  bge cr6, 0x831887a8
	if !ctx.cr[6].lt {
		sub_831887A8(ctx, base);
		return;
	}
	// 831887A4: 4800D854  b 0x83195ff8
	sub_83195FF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887A8 size=12
    let mut pc: u32 = 0x831887A8;
    'dispatch: loop {
        match pc {
            0x831887A8 => {
    //   block [0x831887A8..0x831887B4)
	// 831887A8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831887AC: 40980008  bge cr6, 0x831887b4
	if !ctx.cr[6].lt {
		sub_831887B4(ctx, base);
		return;
	}
	// 831887B0: 4800CCD8  b 0x83195488
	sub_83195488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887B4 size=8
    let mut pc: u32 = 0x831887B4;
    'dispatch: loop {
        match pc {
            0x831887B4 => {
    //   block [0x831887B4..0x831887BC)
	// 831887B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831887B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887C0 size=16
    let mut pc: u32 = 0x831887C0;
    'dispatch: loop {
        match pc {
            0x831887C0 => {
    //   block [0x831887C0..0x831887D0)
	// 831887C0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831887C4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831887C8: 40980008  bge cr6, 0x831887d0
	if !ctx.cr[6].lt {
		sub_831887D0(ctx, base);
		return;
	}
	// 831887CC: 4800D87C  b 0x83196048
	sub_83196048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887D0 size=12
    let mut pc: u32 = 0x831887D0;
    'dispatch: loop {
        match pc {
            0x831887D0 => {
    //   block [0x831887D0..0x831887DC)
	// 831887D0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831887D4: 40980008  bge cr6, 0x831887dc
	if !ctx.cr[6].lt {
		sub_831887DC(ctx, base);
		return;
	}
	// 831887D8: 4800CCE0  b 0x831954b8
	sub_831954B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887DC size=8
    let mut pc: u32 = 0x831887DC;
    'dispatch: loop {
        match pc {
            0x831887DC => {
    //   block [0x831887DC..0x831887E4)
	// 831887DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831887E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887E8 size=16
    let mut pc: u32 = 0x831887E8;
    'dispatch: loop {
        match pc {
            0x831887E8 => {
    //   block [0x831887E8..0x831887F8)
	// 831887E8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831887EC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831887F0: 40980008  bge cr6, 0x831887f8
	if !ctx.cr[6].lt {
		sub_831887F8(ctx, base);
		return;
	}
	// 831887F4: 4800D874  b 0x83196068
	sub_83196068(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831887F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831887F8 size=12
    let mut pc: u32 = 0x831887F8;
    'dispatch: loop {
        match pc {
            0x831887F8 => {
    //   block [0x831887F8..0x83188804)
	// 831887F8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831887FC: 40980008  bge cr6, 0x83188804
	if !ctx.cr[6].lt {
		sub_83188804(ctx, base);
		return;
	}
	// 83188800: 4800CCC8  b 0x831954c8
	sub_831954C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188804 size=8
    let mut pc: u32 = 0x83188804;
    'dispatch: loop {
        match pc {
            0x83188804 => {
    //   block [0x83188804..0x8318880C)
	// 83188804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188810 size=16
    let mut pc: u32 = 0x83188810;
    'dispatch: loop {
        match pc {
            0x83188810 => {
    //   block [0x83188810..0x83188820)
	// 83188810: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188814: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188818: 40980008  bge cr6, 0x83188820
	if !ctx.cr[6].lt {
		sub_83188820(ctx, base);
		return;
	}
	// 8318881C: 4800D884  b 0x831960a0
	sub_831960A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188820 size=12
    let mut pc: u32 = 0x83188820;
    'dispatch: loop {
        match pc {
            0x83188820 => {
    //   block [0x83188820..0x8318882C)
	// 83188820: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188824: 40980008  bge cr6, 0x8318882c
	if !ctx.cr[6].lt {
		sub_8318882C(ctx, base);
		return;
	}
	// 83188828: 4800CC60  b 0x83195488
	sub_83195488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318882C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318882C size=8
    let mut pc: u32 = 0x8318882C;
    'dispatch: loop {
        match pc {
            0x8318882C => {
    //   block [0x8318882C..0x83188834)
	// 8318882C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188838 size=16
    let mut pc: u32 = 0x83188838;
    'dispatch: loop {
        match pc {
            0x83188838 => {
    //   block [0x83188838..0x83188848)
	// 83188838: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318883C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188840: 40980008  bge cr6, 0x83188848
	if !ctx.cr[6].lt {
		sub_83188848(ctx, base);
		return;
	}
	// 83188844: 4800D90C  b 0x83196150
	sub_83196150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188848 size=12
    let mut pc: u32 = 0x83188848;
    'dispatch: loop {
        match pc {
            0x83188848 => {
    //   block [0x83188848..0x83188854)
	// 83188848: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8318884C: 40980008  bge cr6, 0x83188854
	if !ctx.cr[6].lt {
		sub_83188854(ctx, base);
		return;
	}
	// 83188850: 4800CC88  b 0x831954d8
	sub_831954D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188854 size=8
    let mut pc: u32 = 0x83188854;
    'dispatch: loop {
        match pc {
            0x83188854 => {
    //   block [0x83188854..0x8318885C)
	// 83188854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188860 size=16
    let mut pc: u32 = 0x83188860;
    'dispatch: loop {
        match pc {
            0x83188860 => {
    //   block [0x83188860..0x83188870)
	// 83188860: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188864: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188868: 40980008  bge cr6, 0x83188870
	if !ctx.cr[6].lt {
		sub_83188870(ctx, base);
		return;
	}
	// 8318886C: 4800D8FC  b 0x83196168
	sub_83196168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188870 size=12
    let mut pc: u32 = 0x83188870;
    'dispatch: loop {
        match pc {
            0x83188870 => {
    //   block [0x83188870..0x8318887C)
	// 83188870: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188874: 40980008  bge cr6, 0x8318887c
	if !ctx.cr[6].lt {
		sub_8318887C(ctx, base);
		return;
	}
	// 83188878: 4800CC78  b 0x831954f0
	sub_831954F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318887C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318887C size=8
    let mut pc: u32 = 0x8318887C;
    'dispatch: loop {
        match pc {
            0x8318887C => {
    //   block [0x8318887C..0x83188884)
	// 8318887C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188888 size=16
    let mut pc: u32 = 0x83188888;
    'dispatch: loop {
        match pc {
            0x83188888 => {
    //   block [0x83188888..0x83188898)
	// 83188888: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318888C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188890: 40980008  bge cr6, 0x83188898
	if !ctx.cr[6].lt {
		sub_83188898(ctx, base);
		return;
	}
	// 83188894: 4800D8EC  b 0x83196180
	sub_83196180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188898 size=12
    let mut pc: u32 = 0x83188898;
    'dispatch: loop {
        match pc {
            0x83188898 => {
    //   block [0x83188898..0x831888A4)
	// 83188898: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8318889C: 40980008  bge cr6, 0x831888a4
	if !ctx.cr[6].lt {
		sub_831888A4(ctx, base);
		return;
	}
	// 831888A0: 4800CC68  b 0x83195508
	sub_83195508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888A4 size=8
    let mut pc: u32 = 0x831888A4;
    'dispatch: loop {
        match pc {
            0x831888A4 => {
    //   block [0x831888A4..0x831888AC)
	// 831888A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831888A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888B0 size=16
    let mut pc: u32 = 0x831888B0;
    'dispatch: loop {
        match pc {
            0x831888B0 => {
    //   block [0x831888B0..0x831888C0)
	// 831888B0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831888B4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831888B8: 40980008  bge cr6, 0x831888c0
	if !ctx.cr[6].lt {
		sub_831888C0(ctx, base);
		return;
	}
	// 831888BC: 4800D8DC  b 0x83196198
	sub_83196198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888C0 size=12
    let mut pc: u32 = 0x831888C0;
    'dispatch: loop {
        match pc {
            0x831888C0 => {
    //   block [0x831888C0..0x831888CC)
	// 831888C0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831888C4: 40980008  bge cr6, 0x831888cc
	if !ctx.cr[6].lt {
		sub_831888CC(ctx, base);
		return;
	}
	// 831888C8: 4800CC58  b 0x83195520
	sub_83195520(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888CC size=8
    let mut pc: u32 = 0x831888CC;
    'dispatch: loop {
        match pc {
            0x831888CC => {
    //   block [0x831888CC..0x831888D4)
	// 831888CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831888D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888D8 size=16
    let mut pc: u32 = 0x831888D8;
    'dispatch: loop {
        match pc {
            0x831888D8 => {
    //   block [0x831888D8..0x831888E8)
	// 831888D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831888DC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831888E0: 40980008  bge cr6, 0x831888e8
	if !ctx.cr[6].lt {
		sub_831888E8(ctx, base);
		return;
	}
	// 831888E4: 4800D8CC  b 0x831961b0
	sub_831961B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888E8 size=12
    let mut pc: u32 = 0x831888E8;
    'dispatch: loop {
        match pc {
            0x831888E8 => {
    //   block [0x831888E8..0x831888F4)
	// 831888E8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831888EC: 40980008  bge cr6, 0x831888f4
	if !ctx.cr[6].lt {
		sub_831888F4(ctx, base);
		return;
	}
	// 831888F0: 4800CC48  b 0x83195538
	sub_83195538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831888F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831888F4 size=8
    let mut pc: u32 = 0x831888F4;
    'dispatch: loop {
        match pc {
            0x831888F4 => {
    //   block [0x831888F4..0x831888FC)
	// 831888F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831888F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188900 size=16
    let mut pc: u32 = 0x83188900;
    'dispatch: loop {
        match pc {
            0x83188900 => {
    //   block [0x83188900..0x83188910)
	// 83188900: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188904: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188908: 40980008  bge cr6, 0x83188910
	if !ctx.cr[6].lt {
		sub_83188910(ctx, base);
		return;
	}
	// 8318890C: 4800D8FC  b 0x83196208
	sub_83196208(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188910 size=12
    let mut pc: u32 = 0x83188910;
    'dispatch: loop {
        match pc {
            0x83188910 => {
    //   block [0x83188910..0x8318891C)
	// 83188910: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188914: 40980008  bge cr6, 0x8318891c
	if !ctx.cr[6].lt {
		sub_8318891C(ctx, base);
		return;
	}
	// 83188918: 4800CC58  b 0x83195570
	sub_83195570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318891C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318891C size=8
    let mut pc: u32 = 0x8318891C;
    'dispatch: loop {
        match pc {
            0x8318891C => {
    //   block [0x8318891C..0x83188924)
	// 8318891C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188928 size=16
    let mut pc: u32 = 0x83188928;
    'dispatch: loop {
        match pc {
            0x83188928 => {
    //   block [0x83188928..0x83188938)
	// 83188928: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318892C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188930: 40980008  bge cr6, 0x83188938
	if !ctx.cr[6].lt {
		sub_83188938(ctx, base);
		return;
	}
	// 83188934: 4800D914  b 0x83196248
	sub_83196248(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188938 size=12
    let mut pc: u32 = 0x83188938;
    'dispatch: loop {
        match pc {
            0x83188938 => {
    //   block [0x83188938..0x83188944)
	// 83188938: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8318893C: 40980008  bge cr6, 0x83188944
	if !ctx.cr[6].lt {
		sub_83188944(ctx, base);
		return;
	}
	// 83188940: 4800CC68  b 0x831955a8
	sub_831955A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188944 size=8
    let mut pc: u32 = 0x83188944;
    'dispatch: loop {
        match pc {
            0x83188944 => {
    //   block [0x83188944..0x8318894C)
	// 83188944: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188950 size=16
    let mut pc: u32 = 0x83188950;
    'dispatch: loop {
        match pc {
            0x83188950 => {
    //   block [0x83188950..0x83188960)
	// 83188950: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188954: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188958: 40980008  bge cr6, 0x83188960
	if !ctx.cr[6].lt {
		sub_83188960(ctx, base);
		return;
	}
	// 8318895C: 4800D92C  b 0x83196288
	sub_83196288(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188960 size=12
    let mut pc: u32 = 0x83188960;
    'dispatch: loop {
        match pc {
            0x83188960 => {
    //   block [0x83188960..0x8318896C)
	// 83188960: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188964: 40980008  bge cr6, 0x8318896c
	if !ctx.cr[6].lt {
		sub_8318896C(ctx, base);
		return;
	}
	// 83188968: 4800CC78  b 0x831955e0
	sub_831955E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318896C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318896C size=8
    let mut pc: u32 = 0x8318896C;
    'dispatch: loop {
        match pc {
            0x8318896C => {
    //   block [0x8318896C..0x83188974)
	// 8318896C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188978 size=16
    let mut pc: u32 = 0x83188978;
    'dispatch: loop {
        match pc {
            0x83188978 => {
    //   block [0x83188978..0x83188988)
	// 83188978: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318897C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188980: 40980008  bge cr6, 0x83188988
	if !ctx.cr[6].lt {
		sub_83188988(ctx, base);
		return;
	}
	// 83188984: 4800D944  b 0x831962c8
	sub_831962C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188988 size=12
    let mut pc: u32 = 0x83188988;
    'dispatch: loop {
        match pc {
            0x83188988 => {
    //   block [0x83188988..0x83188994)
	// 83188988: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 8318898C: 40980008  bge cr6, 0x83188994
	if !ctx.cr[6].lt {
		sub_83188994(ctx, base);
		return;
	}
	// 83188990: 4800CC88  b 0x83195618
	sub_83195618(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188994 size=8
    let mut pc: u32 = 0x83188994;
    'dispatch: loop {
        match pc {
            0x83188994 => {
    //   block [0x83188994..0x8318899C)
	// 83188994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889A0 size=16
    let mut pc: u32 = 0x831889A0;
    'dispatch: loop {
        match pc {
            0x831889A0 => {
    //   block [0x831889A0..0x831889B0)
	// 831889A0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831889A4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831889A8: 40980008  bge cr6, 0x831889b0
	if !ctx.cr[6].lt {
		sub_831889B0(ctx, base);
		return;
	}
	// 831889AC: 4800DCE4  b 0x83196690
	sub_83196690(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889B0 size=12
    let mut pc: u32 = 0x831889B0;
    'dispatch: loop {
        match pc {
            0x831889B0 => {
    //   block [0x831889B0..0x831889BC)
	// 831889B0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831889B4: 40980008  bge cr6, 0x831889bc
	if !ctx.cr[6].lt {
		sub_831889BC(ctx, base);
		return;
	}
	// 831889B8: 4800CCA8  b 0x83195660
	sub_83195660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889BC size=8
    let mut pc: u32 = 0x831889BC;
    'dispatch: loop {
        match pc {
            0x831889BC => {
    //   block [0x831889BC..0x831889C4)
	// 831889BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831889C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889C8 size=16
    let mut pc: u32 = 0x831889C8;
    'dispatch: loop {
        match pc {
            0x831889C8 => {
    //   block [0x831889C8..0x831889D8)
	// 831889C8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831889CC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831889D0: 40980008  bge cr6, 0x831889d8
	if !ctx.cr[6].lt {
		sub_831889D8(ctx, base);
		return;
	}
	// 831889D4: 4800DD64  b 0x83196738
	sub_83196738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889D8 size=12
    let mut pc: u32 = 0x831889D8;
    'dispatch: loop {
        match pc {
            0x831889D8 => {
    //   block [0x831889D8..0x831889E4)
	// 831889D8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 831889DC: 40980008  bge cr6, 0x831889e4
	if !ctx.cr[6].lt {
		sub_831889E4(ctx, base);
		return;
	}
	// 831889E0: 4800CD60  b 0x83195740
	sub_83195740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889E4 size=8
    let mut pc: u32 = 0x831889E4;
    'dispatch: loop {
        match pc {
            0x831889E4 => {
    //   block [0x831889E4..0x831889EC)
	// 831889E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831889E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831889F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831889F0 size=16
    let mut pc: u32 = 0x831889F0;
    'dispatch: loop {
        match pc {
            0x831889F0 => {
    //   block [0x831889F0..0x83188A00)
	// 831889F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831889F4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 831889F8: 40980008  bge cr6, 0x83188a00
	if !ctx.cr[6].lt {
		sub_83188A00(ctx, base);
		return;
	}
	// 831889FC: 4800DD94  b 0x83196790
	sub_83196790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A00 size=12
    let mut pc: u32 = 0x83188A00;
    'dispatch: loop {
        match pc {
            0x83188A00 => {
    //   block [0x83188A00..0x83188A0C)
	// 83188A00: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188A04: 40980008  bge cr6, 0x83188a0c
	if !ctx.cr[6].lt {
		sub_83188A0C(ctx, base);
		return;
	}
	// 83188A08: 4800CDD0  b 0x831957d8
	sub_831957D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A0C size=8
    let mut pc: u32 = 0x83188A0C;
    'dispatch: loop {
        match pc {
            0x83188A0C => {
    //   block [0x83188A0C..0x83188A14)
	// 83188A0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A18 size=16
    let mut pc: u32 = 0x83188A18;
    'dispatch: loop {
        match pc {
            0x83188A18 => {
    //   block [0x83188A18..0x83188A28)
	// 83188A18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188A1C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188A20: 40980008  bge cr6, 0x83188a28
	if !ctx.cr[6].lt {
		sub_83188A28(ctx, base);
		return;
	}
	// 83188A24: 4800DDB4  b 0x831967d8
	sub_831967D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A28 size=12
    let mut pc: u32 = 0x83188A28;
    'dispatch: loop {
        match pc {
            0x83188A28 => {
    //   block [0x83188A28..0x83188A34)
	// 83188A28: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188A2C: 40980008  bge cr6, 0x83188a34
	if !ctx.cr[6].lt {
		sub_83188A34(ctx, base);
		return;
	}
	// 83188A30: 4800CDF8  b 0x83195828
	sub_83195828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A34 size=8
    let mut pc: u32 = 0x83188A34;
    'dispatch: loop {
        match pc {
            0x83188A34 => {
    //   block [0x83188A34..0x83188A3C)
	// 83188A34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A40 size=16
    let mut pc: u32 = 0x83188A40;
    'dispatch: loop {
        match pc {
            0x83188A40 => {
    //   block [0x83188A40..0x83188A50)
	// 83188A40: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188A44: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188A48: 40980008  bge cr6, 0x83188a50
	if !ctx.cr[6].lt {
		sub_83188A50(ctx, base);
		return;
	}
	// 83188A4C: 4800DE04  b 0x83196850
	sub_83196850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A50 size=12
    let mut pc: u32 = 0x83188A50;
    'dispatch: loop {
        match pc {
            0x83188A50 => {
    //   block [0x83188A50..0x83188A5C)
	// 83188A50: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188A54: 40980008  bge cr6, 0x83188a5c
	if !ctx.cr[6].lt {
		sub_83188A5C(ctx, base);
		return;
	}
	// 83188A58: 4800CE38  b 0x83195890
	sub_83195890(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A5C size=8
    let mut pc: u32 = 0x83188A5C;
    'dispatch: loop {
        match pc {
            0x83188A5C => {
    //   block [0x83188A5C..0x83188A64)
	// 83188A5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A68 size=16
    let mut pc: u32 = 0x83188A68;
    'dispatch: loop {
        match pc {
            0x83188A68 => {
    //   block [0x83188A68..0x83188A78)
	// 83188A68: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188A6C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188A70: 40980008  bge cr6, 0x83188a78
	if !ctx.cr[6].lt {
		sub_83188A78(ctx, base);
		return;
	}
	// 83188A74: 4800DE8C  b 0x83196900
	sub_83196900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A78 size=12
    let mut pc: u32 = 0x83188A78;
    'dispatch: loop {
        match pc {
            0x83188A78 => {
    //   block [0x83188A78..0x83188A84)
	// 83188A78: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188A7C: 40980008  bge cr6, 0x83188a84
	if !ctx.cr[6].lt {
		sub_83188A84(ctx, base);
		return;
	}
	// 83188A80: 4800CEE8  b 0x83195968
	sub_83195968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A84 size=8
    let mut pc: u32 = 0x83188A84;
    'dispatch: loop {
        match pc {
            0x83188A84 => {
    //   block [0x83188A84..0x83188A8C)
	// 83188A84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188A90 size=16
    let mut pc: u32 = 0x83188A90;
    'dispatch: loop {
        match pc {
            0x83188A90 => {
    //   block [0x83188A90..0x83188AA0)
	// 83188A90: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188A94: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188A98: 40980008  bge cr6, 0x83188aa0
	if !ctx.cr[6].lt {
		sub_83188AA0(ctx, base);
		return;
	}
	// 83188A9C: 4800DED4  b 0x83196970
	sub_83196970(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AA0 size=12
    let mut pc: u32 = 0x83188AA0;
    'dispatch: loop {
        match pc {
            0x83188AA0 => {
    //   block [0x83188AA0..0x83188AAC)
	// 83188AA0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188AA4: 40980008  bge cr6, 0x83188aac
	if !ctx.cr[6].lt {
		sub_83188AAC(ctx, base);
		return;
	}
	// 83188AA8: 4800CF38  b 0x831959e0
	sub_831959E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AAC size=8
    let mut pc: u32 = 0x83188AAC;
    'dispatch: loop {
        match pc {
            0x83188AAC => {
    //   block [0x83188AAC..0x83188AB4)
	// 83188AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AB8 size=16
    let mut pc: u32 = 0x83188AB8;
    'dispatch: loop {
        match pc {
            0x83188AB8 => {
    //   block [0x83188AB8..0x83188AC8)
	// 83188AB8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188ABC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188AC0: 40980008  bge cr6, 0x83188ac8
	if !ctx.cr[6].lt {
		sub_83188AC8(ctx, base);
		return;
	}
	// 83188AC4: 4800DF2C  b 0x831969f0
	sub_831969F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AC8 size=12
    let mut pc: u32 = 0x83188AC8;
    'dispatch: loop {
        match pc {
            0x83188AC8 => {
    //   block [0x83188AC8..0x83188AD4)
	// 83188AC8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188ACC: 40980008  bge cr6, 0x83188ad4
	if !ctx.cr[6].lt {
		sub_83188AD4(ctx, base);
		return;
	}
	// 83188AD0: 4800CF88  b 0x83195a58
	sub_83195A58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AD4 size=8
    let mut pc: u32 = 0x83188AD4;
    'dispatch: loop {
        match pc {
            0x83188AD4 => {
    //   block [0x83188AD4..0x83188ADC)
	// 83188AD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AE0 size=16
    let mut pc: u32 = 0x83188AE0;
    'dispatch: loop {
        match pc {
            0x83188AE0 => {
    //   block [0x83188AE0..0x83188AF0)
	// 83188AE0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188AE4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188AE8: 40980008  bge cr6, 0x83188af0
	if !ctx.cr[6].lt {
		sub_83188AF0(ctx, base);
		return;
	}
	// 83188AEC: 4800DF54  b 0x83196a40
	sub_83196A40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AF0 size=12
    let mut pc: u32 = 0x83188AF0;
    'dispatch: loop {
        match pc {
            0x83188AF0 => {
    //   block [0x83188AF0..0x83188AFC)
	// 83188AF0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188AF4: 40980008  bge cr6, 0x83188afc
	if !ctx.cr[6].lt {
		sub_83188AFC(ctx, base);
		return;
	}
	// 83188AF8: 4800CFC0  b 0x83195ab8
	sub_83195AB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188AFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188AFC size=8
    let mut pc: u32 = 0x83188AFC;
    'dispatch: loop {
        match pc {
            0x83188AFC => {
    //   block [0x83188AFC..0x83188B04)
	// 83188AFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B08 size=16
    let mut pc: u32 = 0x83188B08;
    'dispatch: loop {
        match pc {
            0x83188B08 => {
    //   block [0x83188B08..0x83188B18)
	// 83188B08: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188B0C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188B10: 40980008  bge cr6, 0x83188b18
	if !ctx.cr[6].lt {
		sub_83188B18(ctx, base);
		return;
	}
	// 83188B14: 4800DF74  b 0x83196a88
	sub_83196A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B18 size=12
    let mut pc: u32 = 0x83188B18;
    'dispatch: loop {
        match pc {
            0x83188B18 => {
    //   block [0x83188B18..0x83188B24)
	// 83188B18: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188B1C: 40980008  bge cr6, 0x83188b24
	if !ctx.cr[6].lt {
		sub_83188B24(ctx, base);
		return;
	}
	// 83188B20: 4800CFE0  b 0x83195b00
	sub_83195B00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B24 size=8
    let mut pc: u32 = 0x83188B24;
    'dispatch: loop {
        match pc {
            0x83188B24 => {
    //   block [0x83188B24..0x83188B2C)
	// 83188B24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B30 size=16
    let mut pc: u32 = 0x83188B30;
    'dispatch: loop {
        match pc {
            0x83188B30 => {
    //   block [0x83188B30..0x83188B40)
	// 83188B30: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188B34: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188B38: 40980008  bge cr6, 0x83188b40
	if !ctx.cr[6].lt {
		sub_83188B40(ctx, base);
		return;
	}
	// 83188B3C: 4800DF94  b 0x83196ad0
	sub_83196AD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B40 size=12
    let mut pc: u32 = 0x83188B40;
    'dispatch: loop {
        match pc {
            0x83188B40 => {
    //   block [0x83188B40..0x83188B4C)
	// 83188B40: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188B44: 40980008  bge cr6, 0x83188b4c
	if !ctx.cr[6].lt {
		sub_83188B4C(ctx, base);
		return;
	}
	// 83188B48: 4800D000  b 0x83195b48
	sub_83195B48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B4C size=8
    let mut pc: u32 = 0x83188B4C;
    'dispatch: loop {
        match pc {
            0x83188B4C => {
    //   block [0x83188B4C..0x83188B54)
	// 83188B4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B58 size=16
    let mut pc: u32 = 0x83188B58;
    'dispatch: loop {
        match pc {
            0x83188B58 => {
    //   block [0x83188B58..0x83188B68)
	// 83188B58: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188B5C: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188B60: 40980008  bge cr6, 0x83188b68
	if !ctx.cr[6].lt {
		sub_83188B68(ctx, base);
		return;
	}
	// 83188B64: 4800DFBC  b 0x83196b20
	sub_83196B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B68 size=12
    let mut pc: u32 = 0x83188B68;
    'dispatch: loop {
        match pc {
            0x83188B68 => {
    //   block [0x83188B68..0x83188B74)
	// 83188B68: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188B6C: 40980008  bge cr6, 0x83188b74
	if !ctx.cr[6].lt {
		sub_83188B74(ctx, base);
		return;
	}
	// 83188B70: 4800D028  b 0x83195b98
	sub_83195B98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B74 size=8
    let mut pc: u32 = 0x83188B74;
    'dispatch: loop {
        match pc {
            0x83188B74 => {
    //   block [0x83188B74..0x83188B7C)
	// 83188B74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B80 size=16
    let mut pc: u32 = 0x83188B80;
    'dispatch: loop {
        match pc {
            0x83188B80 => {
    //   block [0x83188B80..0x83188B90)
	// 83188B80: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188B84: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188B88: 40980008  bge cr6, 0x83188b90
	if !ctx.cr[6].lt {
		sub_83188B90(ctx, base);
		return;
	}
	// 83188B8C: 4800DFE4  b 0x83196b70
	sub_83196B70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B90 size=12
    let mut pc: u32 = 0x83188B90;
    'dispatch: loop {
        match pc {
            0x83188B90 => {
    //   block [0x83188B90..0x83188B9C)
	// 83188B90: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188B94: 40980008  bge cr6, 0x83188b9c
	if !ctx.cr[6].lt {
		sub_83188B9C(ctx, base);
		return;
	}
	// 83188B98: 4800D050  b 0x83195be8
	sub_83195BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188B9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188B9C size=8
    let mut pc: u32 = 0x83188B9C;
    'dispatch: loop {
        match pc {
            0x83188B9C => {
    //   block [0x83188B9C..0x83188BA4)
	// 83188B9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BA8 size=16
    let mut pc: u32 = 0x83188BA8;
    'dispatch: loop {
        match pc {
            0x83188BA8 => {
    //   block [0x83188BA8..0x83188BB8)
	// 83188BA8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188BAC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188BB0: 40980008  bge cr6, 0x83188bb8
	if !ctx.cr[6].lt {
		sub_83188BB8(ctx, base);
		return;
	}
	// 83188BB4: 4800E004  b 0x83196bb8
	sub_83196BB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BB8 size=12
    let mut pc: u32 = 0x83188BB8;
    'dispatch: loop {
        match pc {
            0x83188BB8 => {
    //   block [0x83188BB8..0x83188BC4)
	// 83188BB8: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188BBC: 40980008  bge cr6, 0x83188bc4
	if !ctx.cr[6].lt {
		sub_83188BC4(ctx, base);
		return;
	}
	// 83188BC0: 4800D070  b 0x83195c30
	sub_83195C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BC4 size=8
    let mut pc: u32 = 0x83188BC4;
    'dispatch: loop {
        match pc {
            0x83188BC4 => {
    //   block [0x83188BC4..0x83188BCC)
	// 83188BC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BD0 size=16
    let mut pc: u32 = 0x83188BD0;
    'dispatch: loop {
        match pc {
            0x83188BD0 => {
    //   block [0x83188BD0..0x83188BE0)
	// 83188BD0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188BD4: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188BD8: 40980008  bge cr6, 0x83188be0
	if !ctx.cr[6].lt {
		sub_83188BE0(ctx, base);
		return;
	}
	// 83188BDC: 4800E034  b 0x83196c10
	sub_83196C10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BE0 size=12
    let mut pc: u32 = 0x83188BE0;
    'dispatch: loop {
        match pc {
            0x83188BE0 => {
    //   block [0x83188BE0..0x83188BEC)
	// 83188BE0: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188BE4: 40980008  bge cr6, 0x83188bec
	if !ctx.cr[6].lt {
		sub_83188BEC(ctx, base);
		return;
	}
	// 83188BE8: 4800D0A0  b 0x83195c88
	sub_83195C88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BEC size=8
    let mut pc: u32 = 0x83188BEC;
    'dispatch: loop {
        match pc {
            0x83188BEC => {
    //   block [0x83188BEC..0x83188BF4)
	// 83188BEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188BF8 size=16
    let mut pc: u32 = 0x83188BF8;
    'dispatch: loop {
        match pc {
            0x83188BF8 => {
    //   block [0x83188BF8..0x83188C08)
	// 83188BF8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188BFC: 2F0B00C8  cmpwi cr6, r11, 0xc8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 200, &mut ctx.xer);
	// 83188C00: 40980008  bge cr6, 0x83188c08
	if !ctx.cr[6].lt {
		sub_83188C08(ctx, base);
		return;
	}
	// 83188C04: 4800E064  b 0x83196c68
	sub_83196C68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188C08 size=12
    let mut pc: u32 = 0x83188C08;
    'dispatch: loop {
        match pc {
            0x83188C08 => {
    //   block [0x83188C08..0x83188C14)
	// 83188C08: 2F0B012C  cmpwi cr6, r11, 0x12c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 300, &mut ctx.xer);
	// 83188C0C: 40980008  bge cr6, 0x83188c14
	if !ctx.cr[6].lt {
		sub_83188C14(ctx, base);
		return;
	}
	// 83188C10: 4800D0D0  b 0x83195ce0
	sub_83195CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188C14 size=8
    let mut pc: u32 = 0x83188C14;
    'dispatch: loop {
        match pc {
            0x83188C14 => {
    //   block [0x83188C14..0x83188C1C)
	// 83188C14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188C20 size=32
    let mut pc: u32 = 0x83188C20;
    'dispatch: loop {
        match pc {
            0x83188C20 => {
    //   block [0x83188C20..0x83188C40)
	// 83188C20: 3965000F  addi r11, r5, 0xf
	ctx.r[11].s64 = ctx.r[5].s64 + 15;
	// 83188C24: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83188C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83188C2C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 83188C30: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83188C34: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83188C38: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83188C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188C40 size=204
    let mut pc: u32 = 0x83188C40;
    'dispatch: loop {
        match pc {
            0x83188C40 => {
    //   block [0x83188C40..0x83188D0C)
	// 83188C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188C44: 4801F529  bl 0x831a816c
	ctx.lr = 0x83188C48;
	sub_831A8130(ctx, base);
	// 83188C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188C4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83188C50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83188C54: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83188C58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83188C5C: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 83188C60: 389D0020  addi r4, r29, 0x20
	ctx.r[4].s64 = ctx.r[29].s64 + 32;
	// 83188C64: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 83188C68: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83188C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188C70: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83188C74: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83188C78: 4801F899  bl 0x831a8510
	ctx.lr = 0x83188C7C;
	sub_831A8510(ctx, base);
	// 83188C7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83188C80: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83188C84: 392B0018  addi r9, r11, 0x18
	ctx.r[9].s64 = ctx.r[11].s64 + 24;
	// 83188C88: 394AB608  addi r10, r10, -0x49f8
	ctx.r[10].s64 = ctx.r[10].s64 + -18936;
	// 83188C8C: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188C90: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188C94: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83188C98: 40820014  bne 0x83188cac
	if !ctx.cr[0].eq {
	pc = 0x83188CAC; continue 'dispatch;
	}
	// 83188C9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83188CA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83188CA4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83188CA8: 409AFFE4  bne cr6, 0x83188c8c
	if !ctx.cr[6].eq {
	pc = 0x83188C8C; continue 'dispatch;
	}
	// 83188CAC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83188CB0: 409A0010  bne cr6, 0x83188cc0
	if !ctx.cr[6].eq {
	pc = 0x83188CC0; continue 'dispatch;
	}
	// 83188CB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83188CB8: 387E0030  addi r3, r30, 0x30
	ctx.r[3].s64 = ctx.r[30].s64 + 48;
	// 83188CBC: 4800BAED  bl 0x831947a8
	ctx.lr = 0x83188CC0;
	sub_831947A8(ctx, base);
	// 83188CC0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83188CC4: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83188CC8: 390B0018  addi r8, r11, 0x18
	ctx.r[8].s64 = ctx.r[11].s64 + 24;
	// 83188CCC: 394AB5EC  addi r10, r10, -0x4a14
	ctx.r[10].s64 = ctx.r[10].s64 + -18964;
	// 83188CD0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188CD4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188CD8: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83188CDC: 40820014  bne 0x83188cf0
	if !ctx.cr[0].eq {
	pc = 0x83188CF0; continue 'dispatch;
	}
	// 83188CE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83188CE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83188CE8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83188CEC: 409AFFE4  bne cr6, 0x83188cd0
	if !ctx.cr[6].eq {
	pc = 0x83188CD0; continue 'dispatch;
	}
	// 83188CF0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83188CF4: 409A0010  bne cr6, 0x83188d04
	if !ctx.cr[6].eq {
	pc = 0x83188D04; continue 'dispatch;
	}
	// 83188CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83188CFC: 387E0030  addi r3, r30, 0x30
	ctx.r[3].s64 = ctx.r[30].s64 + 48;
	// 83188D00: 4800C099  bl 0x83194d98
	ctx.lr = 0x83188D04;
	sub_83194D98(ctx, base);
	// 83188D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83188D08: 4801F4B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188D10 size=84
    let mut pc: u32 = 0x83188D10;
    'dispatch: loop {
        match pc {
            0x83188D10 => {
    //   block [0x83188D10..0x83188D64)
	// 83188D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188D18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83188D1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83188D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188D24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188D28: 40990024  ble cr6, 0x83188d4c
	if !ctx.cr[6].gt {
	pc = 0x83188D4C; continue 'dispatch;
	}
	// 83188D2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83188D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83188D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188D38: 4800BA39  bl 0x83194770
	ctx.lr = 0x83188D3C;
	sub_83194770(ctx, base);
	// 83188D3C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 83188D40: 3BDE0830  addi r30, r30, 0x830
	ctx.r[30].s64 = ctx.r[30].s64 + 2096;
	// 83188D44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83188D48: 409AFFEC  bne cr6, 0x83188d34
	if !ctx.cr[6].eq {
	pc = 0x83188D34; continue 'dispatch;
	}
	// 83188D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83188D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188D58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83188D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188D68 size=16
    let mut pc: u32 = 0x83188D68;
    'dispatch: loop {
        match pc {
            0x83188D68 => {
    //   block [0x83188D68..0x83188D78)
	// 83188D68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188D6C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83188D70: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83188D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188D78 size=100
    let mut pc: u32 = 0x83188D78;
    'dispatch: loop {
        match pc {
            0x83188D78 => {
    //   block [0x83188D78..0x83188DDC)
	// 83188D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188D84: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83188D88: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83188D8C: 392BC6E8  addi r9, r11, -0x3918
	ctx.r[9].s64 = ctx.r[11].s64 + -14616;
	// 83188D90: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83188D94: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83188D98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83188D9C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83188DA0: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83188DA4: 41990028  bgt cr6, 0x83188dcc
	if ctx.cr[6].gt {
	pc = 0x83188DCC; continue 'dispatch;
	}
	// 83188DA8: 4BFFF931  bl 0x831886d8
	ctx.lr = 0x83188DAC;
	sub_831886D8(ctx, base);
	// 83188DAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83188DB0: 38690008  addi r3, r9, 8
	ctx.r[3].s64 = ctx.r[9].s64 + 8;
	// 83188DB4: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 83188DB8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83188DBC: 4BFFFE65  bl 0x83188c20
	ctx.lr = 0x83188DC0;
	sub_83188C20(ctx, base);
	// 83188DC0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 83188DC4: 80890010  lwz r4, 0x10(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 83188DC8: 4BFFFF49  bl 0x83188d10
	ctx.lr = 0x83188DCC;
	sub_83188D10(ctx, base);
	// 83188DCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83188DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188DE0 size=28
    let mut pc: u32 = 0x83188DE0;
    'dispatch: loop {
        match pc {
            0x83188DE0 => {
    //   block [0x83188DE0..0x83188DFC)
	// 83188DE0: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83188DE4: 386BC6F0  addi r3, r11, -0x3910
	ctx.r[3].s64 = ctx.r[11].s64 + -14608;
	// 83188DE8: 8163FFFC  lwz r11, -4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83188DEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83188DF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83188DF4: 9163FFFC  stw r11, -4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83188DF8: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188DFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83188DFC size=8
    let mut pc: u32 = 0x83188DFC;
    'dispatch: loop {
        match pc {
            0x83188DFC => {
    //   block [0x83188DFC..0x83188E04)
	// 83188DFC: 4800B974  b 0x83194770
	sub_83194770(ctx, base);
	return;
	// 83188E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188E08 size=176
    let mut pc: u32 = 0x83188E08;
    'dispatch: loop {
        match pc {
            0x83188E08 => {
    //   block [0x83188E08..0x83188EB8)
	// 83188E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83188E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83188E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83188E18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188E1C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83188E20: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83188E24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83188E28: 2F050800  cmpwi cr6, r5, 0x800
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2048, &mut ctx.xer);
	// 83188E2C: 4098000C  bge cr6, 0x83188e38
	if !ctx.cr[6].lt {
	pc = 0x83188E38; continue 'dispatch;
	}
	// 83188E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188E34: 4800006C  b 0x83188ea0
	pc = 0x83188EA0; continue 'dispatch;
	// 83188E38: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 83188E3C: 3BEBC6F0  addi r31, r11, -0x3910
	ctx.r[31].s64 = ctx.r[11].s64 + -14608;
	// 83188E40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83188E44: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188E48: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83188E4C: 4098FFE4  bge cr6, 0x83188e30
	if !ctx.cr[6].lt {
	pc = 0x83188E30; continue 'dispatch;
	}
	// 83188E50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83188E54: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83188E58: 4099002C  ble cr6, 0x83188e84
	if !ctx.cr[6].gt {
	pc = 0x83188E84; continue 'dispatch;
	}
	// 83188E5C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83188E60: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83188E64: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 83188E68: 4BFFFF01  bl 0x83188d68
	ctx.lr = 0x83188E6C;
	sub_83188D68(ctx, base);
	// 83188E6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188E70: 409A0014  bne cr6, 0x83188e84
	if !ctx.cr[6].eq {
	pc = 0x83188E84; continue 'dispatch;
	}
	// 83188E74: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83188E78: 394A0830  addi r10, r10, 0x830
	ctx.r[10].s64 = ctx.r[10].s64 + 2096;
	// 83188E7C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83188E80: 4198FFE0  blt cr6, 0x83188e60
	if ctx.cr[6].lt {
	pc = 0x83188E60; continue 'dispatch;
	}
	// 83188E84: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83188E88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188E8C: 4BFFFDB5  bl 0x83188c40
	ctx.lr = 0x83188E90;
	sub_83188C40(ctx, base);
	// 83188E90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83188E94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83188E98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83188E9C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83188EA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83188EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83188EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83188EAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83188EB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83188EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83188EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83188EB8 size=392
    let mut pc: u32 = 0x83188EB8;
    'dispatch: loop {
        match pc {
            0x83188EB8 => {
    //   block [0x83188EB8..0x83189040)
	// 83188EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83188EBC: 4801F2B1  bl 0x831a816c
	ctx.lr = 0x83188EC0;
	sub_831A8130(ctx, base);
	// 83188EC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83188EC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83188EC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83188ECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83188ED0: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83188ED4: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83188ED8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83188EDC: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 83188EE0: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83188EE4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83188EE8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83188EEC: 4BFFFE7D  bl 0x83188d68
	ctx.lr = 0x83188EF0;
	sub_83188D68(ctx, base);
	// 83188EF0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83188EF4: 419A00E4  beq cr6, 0x83188fd8
	if ctx.cr[6].eq {
	pc = 0x83188FD8; continue 'dispatch;
	}
	// 83188EF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83188EFC: 2B0B0800  cmplwi cr6, r11, 0x800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2048 as u32, &mut ctx.xer);
	// 83188F00: 40980018  bge cr6, 0x83188f18
	if !ctx.cr[6].lt {
	pc = 0x83188F18; continue 'dispatch;
	}
	// 83188F04: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83188F08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188F0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83188F10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83188F14: 4801F2A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83188F18: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 83188F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188F20: 4800CF99  bl 0x83195eb8
	ctx.lr = 0x83188F24;
	sub_83195EB8(ctx, base);
	// 83188F24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188F2C: 419A005C  beq cr6, 0x83188f88
	if ctx.cr[6].eq {
	pc = 0x83188F88; continue 'dispatch;
	}
	// 83188F30: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83188F34: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83188F38: 4800D669  bl 0x831965a0
	ctx.lr = 0x83188F3C;
	sub_831965A0(ctx, base);
	// 83188F3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188F40: 419A0098  beq cr6, 0x83188fd8
	if ctx.cr[6].eq {
	pc = 0x83188FD8; continue 'dispatch;
	}
	// 83188F44: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 83188F48: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 83188F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188F50: 4800D079  bl 0x83195fc8
	ctx.lr = 0x83188F54;
	sub_83195FC8(ctx, base);
	// 83188F54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188F58: 419A0080  beq cr6, 0x83188fd8
	if ctx.cr[6].eq {
	pc = 0x83188FD8; continue 'dispatch;
	}
	// 83188F5C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83188F60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83188F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188F68: 4800D079  bl 0x83195fe0
	ctx.lr = 0x83188F6C;
	sub_83195FE0(ctx, base);
	// 83188F6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188F70: 409A000C  bne cr6, 0x83188f7c
	if !ctx.cr[6].eq {
	pc = 0x83188F7C; continue 'dispatch;
	}
	// 83188F74: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83188F78: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83188F7C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83188F80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83188F84: 4800006C  b 0x83188ff0
	pc = 0x83188FF0; continue 'dispatch;
	// 83188F88: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 83188F8C: 4800C2ED  bl 0x83195278
	ctx.lr = 0x83188F90;
	sub_83195278(ctx, base);
	// 83188F90: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188F94: 419A0050  beq cr6, 0x83188fe4
	if ctx.cr[6].eq {
	pc = 0x83188FE4; continue 'dispatch;
	}
	// 83188F98: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83188F9C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83188FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188FA4: 4800CE8D  bl 0x83195e30
	ctx.lr = 0x83188FA8;
	sub_83195E30(ctx, base);
	// 83188FA8: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 83188FAC: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 83188FB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188FB4: 4800C475  bl 0x83195428
	ctx.lr = 0x83188FB8;
	sub_83195428(ctx, base);
	// 83188FB8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188FBC: 419A001C  beq cr6, 0x83188fd8
	if ctx.cr[6].eq {
	pc = 0x83188FD8; continue 'dispatch;
	}
	// 83188FC0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83188FC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83188FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83188FCC: 4800C48D  bl 0x83195458
	ctx.lr = 0x83188FD0;
	sub_83195458(ctx, base);
	// 83188FD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83188FD4: 409AFFA8  bne cr6, 0x83188f7c
	if !ctx.cr[6].eq {
	pc = 0x83188F7C; continue 'dispatch;
	}
	// 83188FD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83188FDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83188FE0: 4801F1DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83188FE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83188FE8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83188FEC: 409AFF18  bne cr6, 0x83188f04
	if !ctx.cr[6].eq {
	pc = 0x83188F04; continue 'dispatch;
	}
	// 83188FF0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83188FF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83188FF8: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83188FFC: 1D2B0064  mulli r9, r11, 0x64
	ctx.r[9].s64 = ctx.r[11].s64 * 100;
	// 83189000: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83189004: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83189008: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8318900C: 1D4B0064  mulli r10, r11, 0x64
	ctx.r[10].s64 = ctx.r[11].s64 * 100;
	// 83189010: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83189014: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83189018: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8318901C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83189020: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 83189024: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83189028: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8318902C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83189030: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83189034: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83189038: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318903C: 4801F180  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189040 size=108
    let mut pc: u32 = 0x83189040;
    'dispatch: loop {
        match pc {
            0x83189040 => {
    //   block [0x83189040..0x831890AC)
	// 83189040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83189048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318904C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189050: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 83189054: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83189058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318905C: 4801F185  bl 0x831a81e0
	ctx.lr = 0x83189060;
	sub_831A81E0(ctx, base);
	// 83189060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83189064: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83189068: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8318906C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83189070: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83189074: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83189078: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318907C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83189080: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83189084: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83189088: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318908C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83189090: 997F0028  stb r11, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 83189094: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83189098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318909C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831890A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831890A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831890A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831890B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831890B0 size=112
    let mut pc: u32 = 0x831890B0;
    'dispatch: loop {
        match pc {
            0x831890B0 => {
    //   block [0x831890B0..0x83189120)
	// 831890B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831890B4: 4801F0B5  bl 0x831a8168
	ctx.lr = 0x831890B8;
	sub_831A8130(ctx, base);
	// 831890B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831890BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831890C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831890C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831890C8: 480035C1  bl 0x8318c688
	ctx.lr = 0x831890CC;
	sub_8318C688(ctx, base);
	// 831890CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831890D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831890D4: 419A0044  beq cr6, 0x83189118
	if ctx.cr[6].eq {
	pc = 0x83189118; continue 'dispatch;
	}
	// 831890D8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 831890DC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 831890E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831890E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831890E8: 480036D1  bl 0x8318c7b8
	ctx.lr = 0x831890EC;
	sub_8318C7B8(ctx, base);
	// 831890EC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831890F0: 556B03DE  rlwinm r11, r11, 0, 0xf, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831890F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831890F8: 419A0020  beq cr6, 0x83189118
	if ctx.cr[6].eq {
	pc = 0x83189118; continue 'dispatch;
	}
	// 831890FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83189100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189104: 48005C95  bl 0x8318ed98
	ctx.lr = 0x83189108;
	sub_8318ED98(ctx, base);
	// 83189108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318910C: 48003425  bl 0x8318c530
	ctx.lr = 0x83189110;
	sub_8318C530(ctx, base);
	// 83189110: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83189114: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83189118: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318911C: 4801F09C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189120 size=92
    let mut pc: u32 = 0x83189120;
    'dispatch: loop {
        match pc {
            0x83189120 => {
    //   block [0x83189120..0x8318917C)
	// 83189120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83189128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318912C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83189130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189134: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83189138: 2F040800  cmpwi cr6, r4, 0x800
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2048, &mut ctx.xer);
	// 8318913C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83189140: 41980008  blt cr6, 0x83189148
	if ctx.cr[6].lt {
	pc = 0x83189148; continue 'dispatch;
	}
	// 83189144: 3BE00800  li r31, 0x800
	ctx.r[31].s64 = 2048;
	// 83189148: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8318914C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189150: 387E0094  addi r3, r30, 0x94
	ctx.r[3].s64 = ctx.r[30].s64 + 148;
	// 83189154: 48005845  bl 0x8318e998
	ctx.lr = 0x83189158;
	sub_8318E998(ctx, base);
	// 83189158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318915C: 93FE0090  stw r31, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 83189160: 480066A9  bl 0x8318f808
	ctx.lr = 0x83189164;
	sub_8318F808(ctx, base);
	// 83189164: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318916C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189170: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83189174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189180 size=88
    let mut pc: u32 = 0x83189180;
    'dispatch: loop {
        match pc {
            0x83189180 => {
    //   block [0x83189180..0x831891D8)
	// 83189180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189184: 4801EFE9  bl 0x831a816c
	ctx.lr = 0x83189188;
	sub_831A8130(ctx, base);
	// 83189188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318918C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83189190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83189194: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83189198: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 8318919C: 41980024  blt cr6, 0x831891c0
	if ctx.cr[6].lt {
	pc = 0x831891C0; continue 'dispatch;
	}
	// 831891A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831891A4: 48005DD5  bl 0x8318ef78
	ctx.lr = 0x831891A8;
	sub_8318EF78(ctx, base);
	// 831891A8: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831891AC: 419A0020  beq cr6, 0x831891cc
	if ctx.cr[6].eq {
	pc = 0x831891CC; continue 'dispatch;
	}
	// 831891B0: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 831891B4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831891B8: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831891BC: 4098FFE4  bge cr6, 0x831891a0
	if !ctx.cr[6].lt {
	pc = 0x831891A0; continue 'dispatch;
	}
	// 831891C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831891C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831891C8: 4801EFF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831891CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831891D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831891D4: 4801EFE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831891D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831891D8 size=108
    let mut pc: u32 = 0x831891D8;
    'dispatch: loop {
        match pc {
            0x831891D8 => {
    //   block [0x831891D8..0x83189244)
	// 831891D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831891DC: 4801EF91  bl 0x831a816c
	ctx.lr = 0x831891E0;
	sub_831A8130(ctx, base);
	// 831891E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831891E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831891E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831891EC: 4800349D  bl 0x8318c688
	ctx.lr = 0x831891F0;
	sub_8318C688(ctx, base);
	// 831891F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831891F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831891F8: 409A001C  bne cr6, 0x83189214
	if !ctx.cr[6].eq {
	pc = 0x83189214; continue 'dispatch;
	}
	// 831891FC: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 83189200: 40990008  ble cr6, 0x83189208
	if !ctx.cr[6].gt {
	pc = 0x83189208; continue 'dispatch;
	}
	// 83189204: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 83189208: 7C7EEA14  add r3, r30, r29
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 8318920C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189210: 4801EFAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83189214: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 83189218: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318921C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83189220: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189228: 48003591  bl 0x8318c7b8
	ctx.lr = 0x8318922C;
	sub_8318C7B8(ctx, base);
	// 8318922C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189230: 48003301  bl 0x8318c530
	ctx.lr = 0x83189234;
	sub_8318C530(ctx, base);
	// 83189234: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83189238: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8318923C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189240: 4801EF7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189248 size=192
    let mut pc: u32 = 0x83189248;
    'dispatch: loop {
        match pc {
            0x83189248 => {
    //   block [0x83189248..0x83189308)
	// 83189248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318924C: 4801EF21  bl 0x831a816c
	ctx.lr = 0x83189250;
	sub_831A8130(ctx, base);
	// 83189250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189254: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83189258: 2F040800  cmpwi cr6, r4, 0x800
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2048, &mut ctx.xer);
	// 8318925C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83189260: 41980008  blt cr6, 0x83189268
	if ctx.cr[6].lt {
	pc = 0x83189268; continue 'dispatch;
	}
	// 83189264: 3BC00800  li r30, 0x800
	ctx.r[30].s64 = 2048;
	// 83189268: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318926C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83189270: 3BEBCF98  addi r31, r11, -0x3068
	ctx.r[31].s64 = ctx.r[11].s64 + -12392;
	// 83189274: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83189278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318927C: 4801F295  bl 0x831a8510
	ctx.lr = 0x83189280;
	sub_831A8510(ctx, base);
	// 83189280: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83189284: 4099002C  ble cr6, 0x831892b0
	if !ctx.cr[6].gt {
	pc = 0x831892B0; continue 'dispatch;
	}
	// 83189288: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318928C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83189290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189294: 4BFFC545  bl 0x831857d8
	ctx.lr = 0x83189298;
	sub_831857D8(ctx, base);
	// 83189298: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318929C: 409A0020  bne cr6, 0x831892bc
	if !ctx.cr[6].eq {
	pc = 0x831892BC; continue 'dispatch;
	}
	// 831892A0: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 831892A4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831892A8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831892AC: 4199FFDC  bgt cr6, 0x83189288
	if ctx.cr[6].gt {
	pc = 0x83189288; continue 'dispatch;
	}
	// 831892B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831892B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831892B8: 4801EF04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831892BC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831892C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831892C4: 396BB190  addi r11, r11, -0x4e70
	ctx.r[11].s64 = ctx.r[11].s64 + -20080;
	// 831892C8: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831892CC: 897F0007  lbz r11, 7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 831892D0: 997D0028  stb r11, 0x28(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 831892D4: 897F0008  lbz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831892D8: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 831892DC: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 831892E0: 893F000A  lbz r9, 0xa(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 831892E4: 891F000B  lbz r8, 0xb(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(11 as u32) ) } as u64;
	// 831892E8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 831892EC: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831892F0: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 831892F4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831892F8: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 831892FC: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83189300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189304: 4801EEB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189308 size=116
    let mut pc: u32 = 0x83189308;
    'dispatch: loop {
        match pc {
            0x83189308 => {
    //   block [0x83189308..0x8318937C)
	// 83189308: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8318930C: 89240002  lbz r9, 2(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 83189310: 89040003  lbz r8, 3(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(3 as u32) ) } as u64;
	// 83189314: 88E40001  lbz r7, 1(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83189318: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8318931C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83189320: 54E6FFBE  rlwinm r6, r7, 0x1f, 0x1e, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 83189324: 5529E13E  srwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83189328: 5508D1BE  srwi r8, r8, 6
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(6);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8318932C: 54E707FE  clrlwi r7, r7, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	// 83189330: 5565E7BE  rlwinm r5, r11, 0x1c, 0x1e, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 83189334: 98C30000  stb r6, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 83189338: 5566EFFE  rlwinm r6, r11, 0x1d, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 8318933C: 99230002  stb r9, 2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 83189340: 5549F7BE  rlwinm r9, r10, 0x1e, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 83189344: 99030006  stb r8, 6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u8 ) };
	// 83189348: 5548FFFE  rlwinm r8, r10, 0x1f, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8318934C: 98E30001  stb r7, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 83189350: 5567F7FE  rlwinm r7, r11, 0x1e, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 83189354: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 83189358: 98A30007  stb r5, 7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7 as u32), ctx.r[5].u8 ) };
	// 8318935C: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 83189360: 98C30008  stb r6, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u8 ) };
	// 83189364: 99230003  stb r9, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 83189368: 99030004  stb r8, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u8 ) };
	// 8318936C: 98E30009  stb r7, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[7].u8 ) };
	// 83189370: 99430005  stb r10, 5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 83189374: 9963000A  stb r11, 0xa(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 83189378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189380 size=316
    let mut pc: u32 = 0x83189380;
    'dispatch: loop {
        match pc {
            0x83189380 => {
    //   block [0x83189380..0x831894BC)
	// 83189380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189384: 4801EDE9  bl 0x831a816c
	ctx.lr = 0x83189388;
	sub_831A8130(ctx, base);
	// 83189388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318938C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83189390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83189394: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83189398: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318939C: 40990114  ble cr6, 0x831894b0
	if !ctx.cr[6].gt {
	pc = 0x831894B0; continue 'dispatch;
	}
	// 831893A0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831893A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831893A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831893AC: 4800942D  bl 0x831927d8
	ctx.lr = 0x831893B0;
	sub_831927D8(ctx, base);
	// 831893B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831893B4: 419A0078  beq cr6, 0x8318942c
	if ctx.cr[6].eq {
	pc = 0x8318942C; continue 'dispatch;
	}
	// 831893B8: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 831893BC: 89430007  lbz r10, 7(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 831893C0: 89030004  lbz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831893C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831893C8: 88E30005  lbz r7, 5(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 831893CC: 88C30006  lbz r6, 6(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 831893D0: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831893D4: 88A30008  lbz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831893D8: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 831893DC: 88830009  lbz r4, 9(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 831893E0: 554B0036  rlwinm r11, r10, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831893E4: 8923000A  lbz r9, 0xa(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 831893E8: 8863000B  lbz r3, 0xb(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11 as u32) ) } as u64;
	// 831893EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831893F0: 419A0028  beq cr6, 0x83189418
	if ctx.cr[6].eq {
	pc = 0x83189418; continue 'dispatch;
	}
	// 831893F4: 554A073E  clrlwi r10, r10, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 831893F8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831893FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83189400: 41980018  blt cr6, 0x83189418
	if ctx.cr[6].lt {
	pc = 0x83189418; continue 'dispatch;
	}
	// 83189404: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 83189408: 41990010  bgt cr6, 0x83189418
	if ctx.cr[6].gt {
	pc = 0x83189418; continue 'dispatch;
	}
	// 8318940C: 552B06B4  rlwinm r11, r9, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 83189410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83189414: 409A0024  bne cr6, 0x83189438
	if !ctx.cr[6].eq {
	pc = 0x83189438; continue 'dispatch;
	}
	// 83189418: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8318941C: 4199FF84  bgt cr6, 0x831893a0
	if ctx.cr[6].gt {
	pc = 0x831893A0; continue 'dispatch;
	}
	// 83189420: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83189424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189428: 4801ED94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318942C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83189430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189434: 4801ED88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83189438: 54EB063E  clrlwi r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 8318943C: 80FD001C  lwz r7, 0x1c(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83189440: 55082536  rlwinm r8, r8, 4, 0x14, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0FFFFFFFu64;
	// 83189444: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83189448: 5567E13E  srwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318944C: 5166452E  rlwimi r6, r11, 8, 0x14, 0x17
	ctx.r[6].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x0000000000000F00) | (ctx.r[6].u64 & 0xFFFFFFFFFFFFF0FF);
	// 83189450: 7CEB4378  or r11, r7, r8
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 83189454: 54C8053E  clrlwi r8, r6, 0x14
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x00000FFFu64;
	// 83189458: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318945C: 911D0018  stw r8, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 83189460: 409A001C  bne cr6, 0x8318947c
	if !ctx.cr[6].eq {
	pc = 0x8318947C; continue 'dispatch;
	}
	// 83189464: 50A4442E  rlwimi r4, r5, 8, 0x10, 0x17
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(8) as u64) & 0x000000000000FF00) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF00FF);
	// 83189468: 552BD7BE  rlwinm r11, r9, 0x1a, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000003Fu64;
	// 8318946C: 548813BA  rlwinm r8, r4, 2, 0xe, 0x1d
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x3FFFFFFFu64;
	// 83189470: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 83189474: 1D6B0032  mulli r11, r11, 0x32
	ctx.r[11].s64 = ctx.r[11].s64 * 50;
	// 83189478: 917D001C  stw r11, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318947C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83189480: 554815BA  rlwinm r8, r10, 2, 0x16, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 83189484: 55292DB4  rlwinm r9, r9, 5, 0x16, 0x1a
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x07FFFFFFu64;
	// 83189488: 396BB628  addi r11, r11, -0x49d8
	ctx.r[11].s64 = ctx.r[11].s64 + -18904;
	// 8318948C: 5467EEFE  rlwinm r7, r3, 0x1d, 0x1b, 0x1f
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 83189490: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83189494: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 83189498: 394AAE4C  addi r10, r10, -0x51b4
	ctx.r[10].s64 = ctx.r[10].s64 + -20916;
	// 8318949C: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831894A0: 55295828  slwi r9, r9, 0xb
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(11);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831894A4: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831894A8: 917D0020  stw r11, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 831894AC: 913D0024  stw r9, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 831894B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831894B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831894B8: 4801ED04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831894C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831894C0 size=208
    let mut pc: u32 = 0x831894C0;
    'dispatch: loop {
        match pc {
            0x831894C0 => {
    //   block [0x831894C0..0x83189590)
	// 831894C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831894C4: 4801EC9D  bl 0x831a8160
	ctx.lr = 0x831894C8;
	sub_831A8130(ctx, base);
	// 831894C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831894CC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 831894D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831894D4: 3CA00001  lis r5, 1
	ctx.r[5].s64 = 65536;
	// 831894D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831894DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831894E0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831894E4: 4BFFFC9D  bl 0x83189180
	ctx.lr = 0x831894E8;
	sub_83189180(ctx, base);
	// 831894E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831894EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831894F0: 409A0010  bne cr6, 0x83189500
	if !ctx.cr[6].eq {
	pc = 0x83189500; continue 'dispatch;
	}
	// 831894F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831894F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831894FC: 4801ECB4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83189500: 7D7EE050  subf r11, r30, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 83189504: 3CA00001  lis r5, 1
	ctx.r[5].s64 = 65536;
	// 83189508: 7F6BEA14  add r27, r11, r29
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8318950C: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 83189510: 389BFFFF  addi r4, r27, -1
	ctx.r[4].s64 = ctx.r[27].s64 + -1;
	// 83189514: 4BFFFC6D  bl 0x83189180
	ctx.lr = 0x83189518;
	sub_83189180(ctx, base);
	// 83189518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318951C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83189520: 419AFFD4  beq cr6, 0x831894f4
	if ctx.cr[6].eq {
	pc = 0x831894F4; continue 'dispatch;
	}
	// 83189524: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 83189528: 3CA00001  lis r5, 1
	ctx.r[5].s64 = 65536;
	// 8318952C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83189530: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 83189534: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 83189538: 4BFFFC49  bl 0x83189180
	ctx.lr = 0x8318953C;
	sub_83189180(ctx, base);
	// 8318953C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83189540: 419AFFB4  beq cr6, 0x831894f4
	if ctx.cr[6].eq {
	pc = 0x831894F4; continue 'dispatch;
	}
	// 83189544: 7FBEF850  subf r29, r30, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83189548: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8318954C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83189550: 419A0010  beq cr6, 0x83189560
	if ctx.cr[6].eq {
	pc = 0x83189560; continue 'dispatch;
	}
	// 83189554: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83189558: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318955C: 4801EC54  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83189560: 7D7CF050  subf r11, r28, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 83189564: 7D4BEBD6  divw r10, r11, r29
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[29].s32;
	// 83189568: 7D4AE9D6  mullw r10, r10, r29
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8318956C: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189570: 4082FFE4  bne 0x83189554
	if !ctx.cr[0].eq {
	pc = 0x83189554; continue 'dispatch;
	}
	// 83189574: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83189578: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8318957C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189580: 4BFFFB31  bl 0x831890b0
	ctx.lr = 0x83189584;
	sub_831890B0(ctx, base);
	// 83189584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83189588: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8318958C: 4801EC24  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189590 size=220
    let mut pc: u32 = 0x83189590;
    'dispatch: loop {
        match pc {
            0x83189590 => {
    //   block [0x83189590..0x8318966C)
	// 83189590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189594: 4801EBD1  bl 0x831a8164
	ctx.lr = 0x83189598;
	sub_831A8130(ctx, base);
	// 83189598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318959C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831895A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831895A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831895A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831895AC: 839B0010  lwz r28, 0x10(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 831895B0: 48005AB9  bl 0x8318f068
	ctx.lr = 0x831895B4;
	sub_8318F068(ctx, base);
	// 831895B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831895B8: 409A0034  bne cr6, 0x831895ec
	if !ctx.cr[6].eq {
	pc = 0x831895EC; continue 'dispatch;
	}
	// 831895BC: 7FDCF214  add r30, r28, r30
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 831895C0: 7FFCF850  subf r31, r28, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 831895C4: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 831895C8: 4098009C  bge cr6, 0x83189664
	if !ctx.cr[6].lt {
	pc = 0x83189664; continue 'dispatch;
	}
	// 831895CC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831895D0: 40990094  ble cr6, 0x83189664
	if !ctx.cr[6].gt {
	pc = 0x83189664; continue 'dispatch;
	}
	// 831895D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831895D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831895DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831895E0: 48005A89  bl 0x8318f068
	ctx.lr = 0x831895E4;
	sub_8318F068(ctx, base);
	// 831895E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831895E8: 419AFFD4  beq cr6, 0x831895bc
	if ctx.cr[6].eq {
	pc = 0x831895BC; continue 'dispatch;
	}
	// 831895EC: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 831895F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831895F4: 3BABC700  addi r29, r11, -0x3900
	ctx.r[29].s64 = ctx.r[11].s64 + -14592;
	// 831895F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831895FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83189600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189604: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83189608: 4BFFFB19  bl 0x83189120
	ctx.lr = 0x8318960C;
	sub_83189120(ctx, base);
	// 8318960C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83189610: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189614: 419A0050  beq cr6, 0x83189664
	if ctx.cr[6].eq {
	pc = 0x83189664; continue 'dispatch;
	}
	// 83189618: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318961C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189620: 40990008  ble cr6, 0x83189628
	if !ctx.cr[6].gt {
	pc = 0x83189628; continue 'dispatch;
	}
	// 83189624: 917B001C  stw r11, 0x1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83189628: 817D0064  lwz r11, 0x64(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(100 as u32) ) } as u64;
	// 8318962C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189630: 40990008  ble cr6, 0x83189638
	if !ctx.cr[6].gt {
	pc = 0x83189638; continue 'dispatch;
	}
	// 83189634: 917B0014  stw r11, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83189638: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8318963C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189640: 40990008  ble cr6, 0x83189648
	if !ctx.cr[6].gt {
	pc = 0x83189648; continue 'dispatch;
	}
	// 83189644: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83189648: 815D006C  lwz r10, 0x6c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 8318964C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83189650: 40990014  ble cr6, 0x83189664
	if !ctx.cr[6].gt {
	pc = 0x83189664; continue 'dispatch;
	}
	// 83189654: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83189658: 915B0020  stw r10, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8318965C: 396BAE4C  addi r11, r11, -0x51b4
	ctx.r[11].s64 = ctx.r[11].s64 + -20916;
	// 83189660: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83189664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189668: 4801EB4C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189670 size=132
    let mut pc: u32 = 0x83189670;
    'dispatch: loop {
        match pc {
            0x83189670 => {
    //   block [0x83189670..0x831896F4)
	// 83189670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189674: 4801EAF9  bl 0x831a816c
	ctx.lr = 0x83189678;
	sub_831A8130(ctx, base);
	// 83189678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318967C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83189680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83189684: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83189688: 4BFFFBC1  bl 0x83189248
	ctx.lr = 0x8318968C;
	sub_83189248(ctx, base);
	// 8318968C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83189690: 419A0010  beq cr6, 0x831896a0
	if ctx.cr[6].eq {
	pc = 0x831896A0; continue 'dispatch;
	}
	// 83189694: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83189698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318969C: 4801EB20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831896A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831896A4: 389EFFFE  addi r4, r30, -2
	ctx.r[4].s64 = ctx.r[30].s64 + -2;
	// 831896A8: 387F0002  addi r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 2;
	// 831896AC: 4BFFFB9D  bl 0x83189248
	ctx.lr = 0x831896B0;
	sub_83189248(ctx, base);
	// 831896B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831896B4: 409AFFE0  bne cr6, 0x83189694
	if !ctx.cr[6].eq {
	pc = 0x83189694; continue 'dispatch;
	}
	// 831896B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831896BC: 389EFFFF  addi r4, r30, -1
	ctx.r[4].s64 = ctx.r[30].s64 + -1;
	// 831896C0: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 831896C4: 4BFFFB85  bl 0x83189248
	ctx.lr = 0x831896C8;
	sub_83189248(ctx, base);
	// 831896C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831896CC: 409AFFC8  bne cr6, 0x83189694
	if !ctx.cr[6].eq {
	pc = 0x83189694; continue 'dispatch;
	}
	// 831896D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831896D4: 389EFFFD  addi r4, r30, -3
	ctx.r[4].s64 = ctx.r[30].s64 + -3;
	// 831896D8: 387F0003  addi r3, r31, 3
	ctx.r[3].s64 = ctx.r[31].s64 + 3;
	// 831896DC: 4BFFFB6D  bl 0x83189248
	ctx.lr = 0x831896E0;
	sub_83189248(ctx, base);
	// 831896E0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 831896E4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831896E8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831896EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831896F0: 4801EACC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831896F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831896F8 size=160
    let mut pc: u32 = 0x831896F8;
    'dispatch: loop {
        match pc {
            0x831896F8 => {
    //   block [0x831896F8..0x83189798)
	// 831896F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831896FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83189700: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83189704: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189708: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318970C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83189710: 41980058  blt cr6, 0x83189768
	if ctx.cr[6].lt {
	pc = 0x83189768; continue 'dispatch;
	}
	// 83189714: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83189718: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 8318971C: 409A003C  bne cr6, 0x83189758
	if !ctx.cr[6].eq {
	pc = 0x83189758; continue 'dispatch;
	}
	// 83189720: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 83189724: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83189728: 2B0B00F8  cmplwi cr6, r11, 0xf8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 248 as u32, &mut ctx.xer);
	// 8318972C: 409A002C  bne cr6, 0x83189758
	if !ctx.cr[6].eq {
	pc = 0x83189758; continue 'dispatch;
	}
	// 83189730: 4BFFFBD9  bl 0x83189308
	ctx.lr = 0x83189734;
	sub_83189308(ctx, base);
	// 83189734: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83189738: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318973C: 419A001C  beq cr6, 0x83189758
	if ctx.cr[6].eq {
	pc = 0x83189758; continue 'dispatch;
	}
	// 83189740: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83189744: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 83189748: 419A0010  beq cr6, 0x83189758
	if ctx.cr[6].eq {
	pc = 0x83189758; continue 'dispatch;
	}
	// 8318974C: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 83189750: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83189754: 409A002C  bne cr6, 0x83189780
	if !ctx.cr[6].eq {
	pc = 0x83189780; continue 'dispatch;
	}
	// 83189758: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8318975C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 83189760: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83189764: 4098FFB0  bge cr6, 0x83189714
	if !ctx.cr[6].lt {
	pc = 0x83189714; continue 'dispatch;
	}
	// 83189768: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318976C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318977C: 4E800020  blr
	return;
	// 83189780: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83189784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318978C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189798 size=164
    let mut pc: u32 = 0x83189798;
    'dispatch: loop {
        match pc {
            0x83189798 => {
    //   block [0x83189798..0x8318983C)
	// 83189798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318979C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831897A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831897A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831897A8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831897AC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831897B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831897B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831897B8: 4BFFFF41  bl 0x831896f8
	ctx.lr = 0x831897BC;
	sub_831896F8(ctx, base);
	// 831897BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831897C0: 419A0064  beq cr6, 0x83189824
	if ctx.cr[6].eq {
	pc = 0x83189824; continue 'dispatch;
	}
	// 831897C4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831897C8: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 831897CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831897D0: 88E10053  lbz r7, 0x53(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 831897D4: 396BB65C  addi r11, r11, -0x49a4
	ctx.r[11].s64 = ctx.r[11].s64 + -18852;
	// 831897D8: 5508103E  rotlwi r8, r8, 2
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(2)) as u64;
	// 831897DC: 392BFFF0  addi r9, r11, -0x10
	ctx.r[9].s64 = ctx.r[11].s64 + -16;
	// 831897E0: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 831897E4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831897E8: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 831897EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831897F0: 386AA0F0  addi r3, r10, -0x5f10
	ctx.r[3].s64 = ctx.r[10].s64 + -24336;
	// 831897F4: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831897F8: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 831897FC: 993F0028  stb r9, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 83189800: 7D67582E  lwzx r11, r7, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83189804: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83189808: 4801ED09  bl 0x831a8510
	ctx.lr = 0x8318980C;
	sub_831A8510(ctx, base);
	// 8318980C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83189810: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318981C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189820: 4E800020  blr
	return;
	// 83189824: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83189828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318982C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189834: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189840 size=200
    let mut pc: u32 = 0x83189840;
    'dispatch: loop {
        match pc {
            0x83189840 => {
    //   block [0x83189840..0x83189908)
	// 83189840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189844: 4801E915  bl 0x831a8158
	ctx.lr = 0x83189848;
	sub_831A8130(ctx, base);
	// 83189848: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318984C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83189850: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83189854: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83189858: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8318985C: 7F5BE214  add r26, r27, r28
	ctx.r[26].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 83189860: 83190010  lwz r24, 0x10(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 83189864: 4099009C  ble cr6, 0x83189900
	if !ctx.cr[6].gt {
	pc = 0x83189900; continue 'dispatch;
	}
	// 83189868: 3CA00004  lis r5, 4
	ctx.r[5].s64 = 262144;
	// 8318986C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83189870: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83189874: 4BFFF90D  bl 0x83189180
	ctx.lr = 0x83189878;
	sub_83189180(ctx, base);
	// 83189878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318987C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83189880: 419A0080  beq cr6, 0x83189900
	if ctx.cr[6].eq {
	pc = 0x83189900; continue 'dispatch;
	}
	// 83189884: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83189888: 2B0B00C0  cmplwi cr6, r11, 0xc0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 192 as u32, &mut ctx.xer);
	// 8318988C: 4198005C  blt cr6, 0x831898e8
	if ctx.cr[6].lt {
	pc = 0x831898E8; continue 'dispatch;
	}
	// 83189890: 2B0B00DF  cmplwi cr6, r11, 0xdf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 223 as u32, &mut ctx.xer);
	// 83189894: 41990054  bgt cr6, 0x831898e8
	if ctx.cr[6].gt {
	pc = 0x831898E8; continue 'dispatch;
	}
	// 83189898: 7C9FD050  subf r4, r31, r26
	ctx.r[4].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 8318989C: 4BFFF93D  bl 0x831891d8
	ctx.lr = 0x831898A0;
	sub_831891D8(ctx, base);
	// 831898A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831898A4: 7D7ED050  subf r11, r30, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[30].s64;
	// 831898A8: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 831898AC: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 831898B0: 41980008  blt cr6, 0x831898b8
	if ctx.cr[6].lt {
	pc = 0x831898B8; continue 'dispatch;
	}
	// 831898B4: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 831898B8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831898BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831898C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831898C4: 4BFFFDAD  bl 0x83189670
	ctx.lr = 0x831898C8;
	sub_83189670(ctx, base);
	// 831898C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831898CC: 409A0034  bne cr6, 0x83189900
	if !ctx.cr[6].eq {
	pc = 0x83189900; continue 'dispatch;
	}
	// 831898D0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831898D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831898D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831898DC: 4BFFFEBD  bl 0x83189798
	ctx.lr = 0x831898E0;
	sub_83189798(ctx, base);
	// 831898E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831898E4: 409A001C  bne cr6, 0x83189900
	if !ctx.cr[6].eq {
	pc = 0x83189900; continue 'dispatch;
	}
	// 831898E8: 7D7BF850  subf r11, r27, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[27].s64;
	// 831898EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831898F0: 7F8BE050  subf r28, r11, r28
	ctx.r[28].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 831898F4: 7F6BDA14  add r27, r11, r27
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 831898F8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831898FC: 4199FF6C  bgt cr6, 0x83189868
	if ctx.cr[6].gt {
	pc = 0x83189868; continue 'dispatch;
	}
	// 83189900: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83189904: 4801E8A4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189908 size=160
    let mut pc: u32 = 0x83189908;
    'dispatch: loop {
        match pc {
            0x83189908 => {
    //   block [0x83189908..0x831899A8)
	// 83189908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318990C: 4801E861  bl 0x831a816c
	ctx.lr = 0x83189910;
	sub_831A8130(ctx, base);
	// 83189910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189914: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83189918: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318991C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83189920: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83189924: 4BFFFB9D  bl 0x831894c0
	ctx.lr = 0x83189928;
	sub_831894C0(ctx, base);
	// 83189928: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318992C: 409A000C  bne cr6, 0x83189938
	if !ctx.cr[6].eq {
	pc = 0x83189938; continue 'dispatch;
	}
	// 83189930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189934: 4801E888  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83189938: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8318993C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83189940: 419A005C  beq cr6, 0x8318999c
	if ctx.cr[6].eq {
	pc = 0x8318999C; continue 'dispatch;
	}
	// 83189944: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83189948: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8318994C: 419A0014  beq cr6, 0x83189960
	if ctx.cr[6].eq {
	pc = 0x83189960; continue 'dispatch;
	}
	// 83189950: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83189954: 4099000C  ble cr6, 0x83189960
	if !ctx.cr[6].gt {
	pc = 0x83189960; continue 'dispatch;
	}
	// 83189958: 1D6B0032  mulli r11, r11, 0x32
	ctx.r[11].s64 = ctx.r[11].s64 * 50;
	// 8318995C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83189960: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83189964: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189968: 396BACA8  addi r11, r11, -0x5358
	ctx.r[11].s64 = ctx.r[11].s64 + -21336;
	// 8318996C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189974: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83189978: 4BFFFC19  bl 0x83189590
	ctx.lr = 0x8318997C;
	sub_83189590(ctx, base);
	// 8318997C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189980: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189988: 4BFFFEB9  bl 0x83189840
	ctx.lr = 0x8318998C;
	sub_83189840(ctx, base);
	// 8318998C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189990: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189998: 4BFFF9E9  bl 0x83189380
	ctx.lr = 0x8318999C;
	sub_83189380(ctx, base);
	// 8318999C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831899A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831899A4: 4801E818  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831899A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831899A8 size=144
    let mut pc: u32 = 0x831899A8;
    'dispatch: loop {
        match pc {
            0x831899A8 => {
    //   block [0x831899A8..0x83189A38)
	// 831899A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831899AC: 4801E7C1  bl 0x831a816c
	ctx.lr = 0x831899B0;
	sub_831A8130(ctx, base);
	// 831899B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831899B4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831899B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831899BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831899C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831899C4: 4BFFF67D  bl 0x83189040
	ctx.lr = 0x831899C8;
	sub_83189040(ctx, base);
	// 831899C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831899CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831899D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831899D4: 4BFF4AB5  bl 0x8317e488
	ctx.lr = 0x831899D8;
	sub_8317E488(ctx, base);
	// 831899D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831899DC: 409A0054  bne cr6, 0x83189a30
	if !ctx.cr[6].eq {
	pc = 0x83189A30; continue 'dispatch;
	}
	// 831899E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831899E4: 4BFFFF25  bl 0x83189908
	ctx.lr = 0x831899E8;
	sub_83189908(ctx, base);
	// 831899E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831899EC: 409A0044  bne cr6, 0x83189a30
	if !ctx.cr[6].eq {
	pc = 0x83189A30; continue 'dispatch;
	}
	// 831899F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831899F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831899F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831899FC: 4BFFF985  bl 0x83189380
	ctx.lr = 0x83189A00;
	sub_83189380(ctx, base);
	// 83189A00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83189A04: 409A002C  bne cr6, 0x83189a30
	if !ctx.cr[6].eq {
	pc = 0x83189A30; continue 'dispatch;
	}
	// 83189A08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189A0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189A10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189A14: 4BFFFC5D  bl 0x83189670
	ctx.lr = 0x83189A18;
	sub_83189670(ctx, base);
	// 83189A18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83189A1C: 409A0014  bne cr6, 0x83189a30
	if !ctx.cr[6].eq {
	pc = 0x83189A30; continue 'dispatch;
	}
	// 83189A20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189A24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189A28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189A2C: 4BFFFD6D  bl 0x83189798
	ctx.lr = 0x83189A30;
	sub_83189798(ctx, base);
	// 83189A30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189A34: 4801E788  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189A38 size=132
    let mut pc: u32 = 0x83189A38;
    'dispatch: loop {
        match pc {
            0x83189A38 => {
    //   block [0x83189A38..0x83189ABC)
	// 83189A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189A3C: 4801E731  bl 0x831a816c
	ctx.lr = 0x83189A40;
	sub_831A8130(ctx, base);
	// 83189A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189A44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83189A48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83189A4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83189A50: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83189A54: 4BFFD99D  bl 0x831873f0
	ctx.lr = 0x83189A58;
	sub_831873F0(ctx, base);
	// 83189A58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83189A5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83189A60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83189A64: 4BFFFF45  bl 0x831899a8
	ctx.lr = 0x83189A68;
	sub_831899A8(ctx, base);
	// 83189A68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83189A6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83189A70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83189A74: 409A001C  bne cr6, 0x83189a90
	if !ctx.cr[6].eq {
	pc = 0x83189A90; continue 'dispatch;
	}
	// 83189A78: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83189A7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83189A80: 409A0010  bne cr6, 0x83189a90
	if !ctx.cr[6].eq {
	pc = 0x83189A90; continue 'dispatch;
	}
	// 83189A84: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83189A88: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 83189A8C: 409A0008  bne cr6, 0x83189a94
	if !ctx.cr[6].eq {
	pc = 0x83189A94; continue 'dispatch;
	}
	// 83189A90: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83189A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83189A98: 409A0010  bne cr6, 0x83189aa8
	if !ctx.cr[6].eq {
	pc = 0x83189AA8; continue 'dispatch;
	}
	// 83189A9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83189AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83189AA4: 419A0008  beq cr6, 0x83189aac
	if ctx.cr[6].eq {
	pc = 0x83189AAC; continue 'dispatch;
	}
	// 83189AA8: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 83189AAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83189AB0: 4BFFD951  bl 0x83187400
	ctx.lr = 0x83189AB4;
	sub_83187400(ctx, base);
	// 83189AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83189AB8: 4801E704  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189AC0 size=148
    let mut pc: u32 = 0x83189AC0;
    'dispatch: loop {
        match pc {
            0x83189AC0 => {
    //   block [0x83189AC0..0x83189B54)
	// 83189AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189AC4: 4801E6A9  bl 0x831a816c
	ctx.lr = 0x83189AC8;
	sub_831A8130(ctx, base);
	// 83189AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83189AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83189AD4: 3D60831A  lis r11, -0x7ce6
	ctx.r[11].s64 = -2095448064;
	// 83189AD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83189ADC: 388B8400  addi r4, r11, -0x7c00
	ctx.r[4].s64 = ctx.r[11].s64 + -31744;
	// 83189AE0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83189AE4: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83189AE8: 48000479  bl 0x83189f60
	ctx.lr = 0x83189AEC;
	sub_83189F60(ctx, base);
	// 83189AEC: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 83189AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189AF4: 388B72F0  addi r4, r11, 0x72f0
	ctx.r[4].s64 = ctx.r[11].s64 + 29424;
	// 83189AF8: 48000471  bl 0x83189f68
	ctx.lr = 0x83189AFC;
	sub_83189F68(ctx, base);
	// 83189AFC: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 83189B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189B04: 388B7390  addi r4, r11, 0x7390
	ctx.r[4].s64 = ctx.r[11].s64 + 29584;
	// 83189B08: 48000469  bl 0x83189f70
	ctx.lr = 0x83189B0C;
	sub_83189F70(ctx, base);
	// 83189B0C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 83189B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189B14: 388B7680  addi r4, r11, 0x7680
	ctx.r[4].s64 = ctx.r[11].s64 + 30336;
	// 83189B18: 48000469  bl 0x83189f80
	ctx.lr = 0x83189B1C;
	sub_83189F80(ctx, base);
	// 83189B1C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 83189B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189B24: 388B7518  addi r4, r11, 0x7518
	ctx.r[4].s64 = ctx.r[11].s64 + 29976;
	// 83189B28: 48000469  bl 0x83189f90
	ctx.lr = 0x83189B2C;
	sub_83189F90(ctx, base);
	// 83189B2C: 3D608319  lis r11, -0x7ce7
	ctx.r[11].s64 = -2095513600;
	// 83189B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189B34: 388B79E8  addi r4, r11, 0x79e8
	ctx.r[4].s64 = ctx.r[11].s64 + 31208;
	// 83189B38: 48000469  bl 0x83189fa0
	ctx.lr = 0x83189B3C;
	sub_83189FA0(ctx, base);
	// 83189B3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83189B40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83189B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189B48: 48000949  bl 0x8318a490
	ctx.lr = 0x83189B4C;
	sub_8318A490(ctx, base);
	// 83189B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83189B50: 4801E66C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189B58 size=24
    let mut pc: u32 = 0x83189B58;
    'dispatch: loop {
        match pc {
            0x83189B58 => {
    //   block [0x83189B58..0x83189B70)
	// 83189B58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83189B5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83189B60: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83189B64: 90E30044  stw r7, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 83189B68: 91030048  stw r8, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 83189B6C: 4800101C  b 0x8318ab88
	sub_8318AB88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189B70 size=40
    let mut pc: u32 = 0x83189B70;
    'dispatch: loop {
        match pc {
            0x83189B70 => {
    //   block [0x83189B70..0x83189B98)
	// 83189B70: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83189B74: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83189B78: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83189B7C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83189B80: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83189B84: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83189B88: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83189B8C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83189B90: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83189B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189B98 size=428
    let mut pc: u32 = 0x83189B98;
    'dispatch: loop {
        match pc {
            0x83189B98 => {
    //   block [0x83189B98..0x83189CF8)
	// 83189B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83189BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189BA4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83189BA8: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83189BAC: 41990130  bgt cr6, 0x83189cdc
	if ctx.cr[6].gt {
	pc = 0x83189CDC; continue 'dispatch;
	}
	// 83189BB0: 419A0180  beq cr6, 0x83189d30
	if ctx.cr[6].eq {
	pc = 0x83189D30; continue 'dispatch;
	}
	// 83189BB4: 396BFFEF  addi r11, r11, -0x11
	ctx.r[11].s64 = ctx.r[11].s64 + -17;
	// 83189BB8: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 83189BBC: 41990160  bgt cr6, 0x83189d1c
	if ctx.cr[6].gt {
	pc = 0x83189D1C; continue 'dispatch;
	}
	// 83189BC0: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 83189BC4: 398C9BD8  addi r12, r12, -0x6428
	ctx.r[12].s64 = ctx.r[12].s64 + -25640;
	// 83189BC8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83189BCC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83189BD0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83189BD4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83189D30; continue 'dispatch;
		},
		1 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		2 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		3 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		4 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		5 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		6 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		7 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		8 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		9 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		10 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		11 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		12 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		13 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		14 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		15 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		16 => {
	pc = 0x83189CF8; continue 'dispatch;
		},
		17 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		18 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		19 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		20 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		21 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		22 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		23 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		24 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		25 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		26 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		27 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		28 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		29 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		30 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		31 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		32 => {
	pc = 0x83189D30; continue 'dispatch;
		},
		33 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		34 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		35 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		36 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		37 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		38 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		39 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		40 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		41 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		42 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		43 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		44 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		45 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		46 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		47 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		48 => {
	pc = 0x83189D30; continue 'dispatch;
		},
		49 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		50 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		51 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		52 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		53 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		54 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		55 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		56 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		57 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		58 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		59 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		60 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		61 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		62 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		63 => {
	pc = 0x83189D1C; continue 'dispatch;
		},
		64 => {
	pc = 0x83189D30; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83189BD8: 83189D30  lwz r24, -0x62d0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 83189BDC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BE0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BE4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BE8: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BEC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BF0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BF4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BF8: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189BFC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C00: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C04: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C08: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C0C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C10: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C14: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C18: 83189CF8  lwz r24, -0x6308(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25352 as u32) ) } as u64;
	// 83189C1C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C20: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C24: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C28: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C2C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C30: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C34: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C38: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C3C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C40: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C44: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C48: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C4C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C50: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C54: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C58: 83189D30  lwz r24, -0x62d0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 83189C5C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C60: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C64: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C68: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C6C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C70: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C74: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C78: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C7C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C80: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C84: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C88: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C8C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C90: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C94: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189C98: 83189D30  lwz r24, -0x62d0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 83189C9C: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CA0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CA4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CA8: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CAC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CB0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CB4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CB8: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CBC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CC0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CC4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CC8: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CCC: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CD0: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CD4: 83189D1C  lwz r24, -0x62e4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25316 as u32) ) } as u64;
	// 83189CD8: 83189D30  lwz r24, -0x62d0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 83189CDC: 2F0B0101  cmpwi cr6, r11, 0x101
	ctx.cr[6].compare_i32(ctx.r[11].s32, 257, &mut ctx.xer);
	// 83189CE0: 4199002C  bgt cr6, 0x83189d0c
	if ctx.cr[6].gt {
	pc = 0x83189D0C; continue 'dispatch;
	}
	// 83189CE4: 419A0014  beq cr6, 0x83189cf8
	if ctx.cr[6].eq {
	pc = 0x83189CF8; continue 'dispatch;
	}
	// 83189CE8: 2F0B0071  cmpwi cr6, r11, 0x71
	ctx.cr[6].compare_i32(ctx.r[11].s32, 113, &mut ctx.xer);
	// 83189CEC: 419A0044  beq cr6, 0x83189d30
	if ctx.cr[6].eq {
	pc = 0x83189D30; continue 'dispatch;
	}
	// 83189CF0: 2F0B00F1  cmpwi cr6, r11, 0xf1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 241, &mut ctx.xer);
	// 83189CF4: 48000024  b 0x83189d18
	pc = 0x83189D18; continue 'dispatch;
            }
            0x83189CF8 => {
    //   block [0x83189CF8..0x83189D1C)
	// 83189CF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83189CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189D08: 4E800020  blr
	return;
	// 83189D0C: 2F0B0111  cmpwi cr6, r11, 0x111
	ctx.cr[6].compare_i32(ctx.r[11].s32, 273, &mut ctx.xer);
	// 83189D10: 419A0020  beq cr6, 0x83189d30
	if ctx.cr[6].eq {
	pc = 0x83189D30; continue 'dispatch;
	}
	// 83189D14: 2F0B1001  cmpwi cr6, r11, 0x1001
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4097, &mut ctx.xer);
	// 83189D18: 419A0018  beq cr6, 0x83189d30
	if ctx.cr[6].eq {
	pc = 0x83189D30; continue 'dispatch;
	}
            }
            0x83189D1C => {
    //   block [0x83189D1C..0x83189D30)
	// 83189D1C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83189D20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83189D24: 38ABB780  addi r5, r11, -0x4880
	ctx.r[5].s64 = ctx.r[11].s64 + -18560;
	// 83189D28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83189D2C: 48000B05  bl 0x8318a830
	ctx.lr = 0x83189D30;
	sub_8318A830(ctx, base);
	pc = 0x83189D30; continue 'dispatch;
            }
            0x83189D30 => {
    //   block [0x83189D30..0x83189D44)
	// 83189D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83189D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189D48 size=20
    let mut pc: u32 = 0x83189D48;
    'dispatch: loop {
        match pc {
            0x83189D48 => {
    //   block [0x83189D48..0x83189D5C)
	// 83189D48: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83189D4C: 2F0B0051  cmpwi cr6, r11, 0x51
	ctx.cr[6].compare_i32(ctx.r[11].s32, 81, &mut ctx.xer);
	// 83189D50: 409A000C  bne cr6, 0x83189d5c
	if !ctx.cr[6].eq {
		sub_83189D5C(ctx, base);
		return;
	}
	// 83189D54: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83189D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189D5C size=24
    let mut pc: u32 = 0x83189D5C;
    'dispatch: loop {
        match pc {
            0x83189D5C => {
    //   block [0x83189D5C..0x83189D74)
	// 83189D5C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83189D60: 419A0014  beq cr6, 0x83189d74
	if ctx.cr[6].eq {
		sub_83189D74(ctx, base);
		return;
	}
	// 83189D64: 81640094  lwz r11, 0x94(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 83189D68: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83189D6C: 2F0B0051  cmpwi cr6, r11, 0x51
	ctx.cr[6].compare_i32(ctx.r[11].s32, 81, &mut ctx.xer);
	// 83189D70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189D74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189D74 size=8
    let mut pc: u32 = 0x83189D74;
    'dispatch: loop {
        match pc {
            0x83189D74 => {
    //   block [0x83189D74..0x83189D7C)
	// 83189D74: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83189D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83189D80 size=228
    let mut pc: u32 = 0x83189D80;
    'dispatch: loop {
        match pc {
            0x83189D80 => {
    //   block [0x83189D80..0x83189E38)
	// 83189D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83189D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83189D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83189D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83189D90: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83189D94: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83189D98: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 83189D9C: 409A001C  bne cr6, 0x83189db8
	if !ctx.cr[6].eq {
	pc = 0x83189DB8; continue 'dispatch;
	}
	// 83189DA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83189DA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189DB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189DB4: 4E800020  blr
	return;
	// 83189DB8: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83189DBC: 409A0090  bne cr6, 0x83189e4c
	if !ctx.cr[6].eq {
	pc = 0x83189E4C; continue 'dispatch;
	}
	// 83189DC0: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 83189DC4: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 83189DC8: 41990084  bgt cr6, 0x83189e4c
	if ctx.cr[6].gt {
	pc = 0x83189E4C; continue 'dispatch;
	}
	// 83189DCC: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 83189DD0: 398C9DE4  addi r12, r12, -0x621c
	ctx.r[12].s64 = ctx.r[12].s64 + -25116;
	// 83189DD4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83189DD8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83189DDC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83189DE0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83189E48; continue 'dispatch;
		},
		1 => {
	pc = 0x83189E38; continue 'dispatch;
		},
		2 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		3 => {
	pc = 0x83189E48; continue 'dispatch;
		},
		4 => {
	pc = 0x83189E48; continue 'dispatch;
		},
		5 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		6 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		7 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		8 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		9 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		10 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		11 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		12 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		13 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		14 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		15 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		16 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		17 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		18 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		19 => {
	pc = 0x83189E4C; continue 'dispatch;
		},
		20 => {
	pc = 0x83189E48; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83189DE4: 83189E48  lwz r24, -0x61b8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25016 as u32) ) } as u64;
	// 83189DE8: 83189E38  lwz r24, -0x61c8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25032 as u32) ) } as u64;
	// 83189DEC: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189DF0: 83189E48  lwz r24, -0x61b8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25016 as u32) ) } as u64;
	// 83189DF4: 83189E48  lwz r24, -0x61b8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25016 as u32) ) } as u64;
	// 83189DF8: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189DFC: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E00: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E04: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E08: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E0C: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E10: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E14: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E18: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E1C: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E20: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E24: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E28: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E2C: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E30: 83189E4C  lwz r24, -0x61b4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25012 as u32) ) } as u64;
	// 83189E34: 83189E48  lwz r24, -0x61b8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-25016 as u32) ) } as u64;
            }
            0x83189E38 => {
    //   block [0x83189E38..0x83189E48)
	// 83189E38: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189E3C: 4BEC890D  bl 0x83052748
	ctx.lr = 0x83189E40;
	sub_83052748(ctx, base);
	// 83189E40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83189E44: 419A0008  beq cr6, 0x83189e4c
	if ctx.cr[6].eq {
	pc = 0x83189E4C; continue 'dispatch;
	}
	pc = 0x83189E48; continue 'dispatch;
            }
            0x83189E48 => {
    //   block [0x83189E48..0x83189E4C)
	// 83189E48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x83189E4C; continue 'dispatch;
            }
            0x83189E4C => {
    //   block [0x83189E4C..0x83189E64)
	// 83189E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83189E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83189E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83189E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83189E5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189E68 size=16
    let mut pc: u32 = 0x83189E68;
    'dispatch: loop {
        match pc {
            0x83189E68 => {
    //   block [0x83189E68..0x83189E78)
	// 83189E68: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83189E6C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 83189E70: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83189E74: 4800190C  b 0x8318b780
	sub_8318B780(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83189E78 size=148
    let mut pc: u32 = 0x83189E78;
    'dispatch: loop {
        match pc {
            0x83189E78 => {
    //   block [0x83189E78..0x83189F0C)
	// 83189E78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83189E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83189E80: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83189E84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83189E88: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83189E8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83189E90: 4200FFF8  bdnz 0x83189e88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83189E88; continue 'dispatch;
	}
	// 83189E94: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 83189E98: 3D00821A  lis r8, -0x7de6
	ctx.r[8].s64 = -2112225280;
	// 83189E9C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 83189EA0: 2123FFF0  subfic r9, r3, -0x10
	ctx.xer.ca = ctx.r[3].u32 <= -16 as u32;
	ctx.r[9].s64 = (-16 as i64) - ctx.r[3].s64;
	// 83189EA4: 394000DC  li r10, 0xdc
	ctx.r[10].s64 = 220;
	// 83189EA8: C1A79450  lfs f13, -0x6bb0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83189EAC: C008B7B0  lfs f0, -0x4850(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-18512 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83189EB0: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83189EB4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83189EB8: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 83189EBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83189EC0: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 83189EC4: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83189EC8: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 83189ECC: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 83189ED0: ED8C683A  fmadds f12, f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 83189ED4: FD80665E  fctidz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 83189ED8: D981FFF8  stfd f12, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[12].u64 ) };
	// 83189EDC: 8901FFFF  lbz r8, -1(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-1 as u32) ) } as u64;
	// 83189EE0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83189EE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83189EE8: 409AFFC8  bne cr6, 0x83189eb0
	if !ctx.cr[6].eq {
	pc = 0x83189EB0; continue 'dispatch;
	}
	// 83189EEC: 396300EC  addi r11, r3, 0xec
	ctx.r[11].s64 = ctx.r[3].s64 + 236;
	// 83189EF0: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 83189EF4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 83189EF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83189EFC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83189F00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83189F04: 4200FFF8  bdnz 0x83189efc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83189EFC; continue 'dispatch;
	}
	// 83189F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F10 size=16
    let mut pc: u32 = 0x83189F10;
    'dispatch: loop {
        match pc {
            0x83189F10 => {
    //   block [0x83189F10..0x83189F20)
	// 83189F10: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83189F14: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 83189F18: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F1C: 4800E654  b 0x83198570
	sub_83198570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F20 size=16
    let mut pc: u32 = 0x83189F20;
    'dispatch: loop {
        match pc {
            0x83189F20 => {
    //   block [0x83189F20..0x83189F30)
	// 83189F20: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83189F24: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 83189F28: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F2C: 4800E69C  b 0x831985c8
	sub_831985C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F30 size=16
    let mut pc: u32 = 0x83189F30;
    'dispatch: loop {
        match pc {
            0x83189F30 => {
    //   block [0x83189F30..0x83189F40)
	// 83189F30: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83189F34: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 83189F38: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F3C: 4800E6BC  b 0x831985f8
	sub_831985F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F40 size=12
    let mut pc: u32 = 0x83189F40;
    'dispatch: loop {
        match pc {
            0x83189F40 => {
    //   block [0x83189F40..0x83189F4C)
	// 83189F40: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 83189F44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83189F48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F4C size=12
    let mut pc: u32 = 0x83189F4C;
    'dispatch: loop {
        match pc {
            0x83189F4C => {
    //   block [0x83189F4C..0x83189F58)
	// 83189F4C: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83189F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83189F54: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F58 size=4
    let mut pc: u32 = 0x83189F58;
    'dispatch: loop {
        match pc {
            0x83189F58 => {
    //   block [0x83189F58..0x83189F5C)
	// 83189F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F60 size=8
    let mut pc: u32 = 0x83189F60;
    'dispatch: loop {
        match pc {
            0x83189F60 => {
    //   block [0x83189F60..0x83189F68)
	// 83189F60: 90830068  stw r4, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 83189F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F68 size=8
    let mut pc: u32 = 0x83189F68;
    'dispatch: loop {
        match pc {
            0x83189F68 => {
    //   block [0x83189F68..0x83189F70)
	// 83189F68: 9083006C  stw r4, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 83189F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F70 size=12
    let mut pc: u32 = 0x83189F70;
    'dispatch: loop {
        match pc {
            0x83189F70 => {
    //   block [0x83189F70..0x83189F7C)
	// 83189F70: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F74: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83189F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F80 size=12
    let mut pc: u32 = 0x83189F80;
    'dispatch: loop {
        match pc {
            0x83189F80 => {
    //   block [0x83189F80..0x83189F8C)
	// 83189F80: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F84: 908B0020  stw r4, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 83189F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189F90 size=12
    let mut pc: u32 = 0x83189F90;
    'dispatch: loop {
        match pc {
            0x83189F90 => {
    //   block [0x83189F90..0x83189F9C)
	// 83189F90: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83189F94: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83189F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189FA0 size=8
    let mut pc: u32 = 0x83189FA0;
    'dispatch: loop {
        match pc {
            0x83189FA0 => {
    //   block [0x83189FA0..0x83189FA8)
	// 83189FA0: 90830070  stw r4, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 83189FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83189FA8 size=40
    let mut pc: u32 = 0x83189FA8;
    'dispatch: loop {
        match pc {
            0x83189FA8 => {
    //   block [0x83189FA8..0x83189FD0)
	// 83189FA8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83189FAC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83189FB0: 419A009C  beq cr6, 0x8318a04c
	if ctx.cr[6].eq {
		sub_8318A04C(ctx, base);
		return;
	}
	// 83189FB4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83189FB8: 419A0094  beq cr6, 0x8318a04c
	if ctx.cr[6].eq {
		sub_8318A04C(ctx, base);
		return;
	}
	// 83189FBC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83189FC0: 419A0010  beq cr6, 0x83189fd0
	if ctx.cr[6].eq {
		sub_83189FD0(ctx, base);
		return;
	}
	// 83189FC4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83189FC8: 38ABB7B4  addi r5, r11, -0x484c
	ctx.r[5].s64 = ctx.r[11].s64 + -18508;
	// 83189FCC: 48000864  b 0x8318a830
	sub_8318A830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83189FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83189FD0 size=124
    let mut pc: u32 = 0x83189FD0;
    'dispatch: loop {
        match pc {
            0x83189FD0 => {
    //   block [0x83189FD0..0x8318A04C)
	// 83189FD0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83189FD4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83189FD8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83189FDC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83189FE0: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 83189FE4: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83189FE8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83189FEC: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83189FF0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83189FF4: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83189FF8: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 83189FFC: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318A000: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318A004: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8318A008: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318A00C: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318A010: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318A014: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318A018: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318A01C: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318A020: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318A024: 91650024  stw r11, 0x24(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8318A028: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318A02C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8318A030: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318A034: 91650028  stw r11, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8318A038: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 8318A03C: 9165002C  stw r11, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318A040: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 8318A044: 91650030  stw r11, 0x30(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8318A048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A04C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A04C size=44
    let mut pc: u32 = 0x8318A04C;
    'dispatch: loop {
        match pc {
            0x8318A04C => {
    //   block [0x8318A04C..0x8318A078)
	// 8318A04C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318A050: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318A054: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318A058: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318A05C: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318A060: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318A064: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318A068: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318A06C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318A070: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318A074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318A078 size=280
    let mut pc: u32 = 0x8318A078;
    'dispatch: loop {
        match pc {
            0x8318A078 => {
    //   block [0x8318A078..0x8318A190)
	// 8318A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A07C: 4801E0F1  bl 0x831a816c
	ctx.lr = 0x8318A080;
	sub_831A8130(ctx, base);
	// 8318A080: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A084: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8318A088: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318A08C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8318A090: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8318A094: 4BFFFF15  bl 0x83189fa8
	ctx.lr = 0x8318A098;
	sub_83189FA8(ctx, base);
	// 8318A098: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318A09C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318A0A0: 40990098  ble cr6, 0x8318a138
	if !ctx.cr[6].gt {
	pc = 0x8318A138; continue 'dispatch;
	}
	// 8318A0A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8318A0A8: 409900A4  ble cr6, 0x8318a14c
	if !ctx.cr[6].gt {
	pc = 0x8318A14C; continue 'dispatch;
	}
	// 8318A0AC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8318A0B0: 409A0088  bne cr6, 0x8318a138
	if !ctx.cr[6].eq {
	pc = 0x8318A138; continue 'dispatch;
	}
	// 8318A0B4: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318A0B8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8318A0BC: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318A0C0: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8318A0C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318A0C8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8318A0CC: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318A0D0: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 8318A0D4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318A0D8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318A0DC: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 8318A0E0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318A0E4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318A0E8: 914100EC  stw r10, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 8318A0EC: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8318A0F0: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8318A0F4: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8318A0F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8318A0FC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8318A100: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318A104: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8318A108: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8318A10C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318A110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8318A114: 409A001C  bne cr6, 0x8318a130
	if !ctx.cr[6].eq {
	pc = 0x8318A130; continue 'dispatch;
	}
	// 8318A118: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A11C: 38ABB834  addi r5, r11, -0x47cc
	ctx.r[5].s64 = ctx.r[11].s64 + -18380;
	// 8318A120: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318A124: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318A128: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 8318A12C: 48000014  b 0x8318a140
	pc = 0x8318A140; continue 'dispatch;
	// 8318A130: 912100F0  stw r9, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u32 ) };
	// 8318A134: 48000018  b 0x8318a14c
	pc = 0x8318A14C; continue 'dispatch;
	// 8318A138: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A13C: 38ABB7F0  addi r5, r11, -0x4810
	ctx.r[5].s64 = ctx.r[11].s64 + -18448;
	// 8318A140: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318A144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318A148: 480006E9  bl 0x8318a830
	ctx.lr = 0x8318A14C;
	sub_8318A830(ctx, base);
	// 8318A14C: 4800072D  bl 0x8318a878
	ctx.lr = 0x8318A150;
	sub_8318A878(ctx, base);
	// 8318A150: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318A154: 409A000C  bne cr6, 0x8318a160
	if !ctx.cr[6].eq {
	pc = 0x8318A160; continue 'dispatch;
	}
	// 8318A158: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8318A15C: 48000008  b 0x8318a164
	pc = 0x8318A164; continue 'dispatch;
	// 8318A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318A164: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8318A168: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 8318A16C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318A170: 419A0018  beq cr6, 0x8318a188
	if ctx.cr[6].eq {
	pc = 0x8318A188; continue 'dispatch;
	}
	// 8318A174: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 8318A178: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 8318A17C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318A180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318A184: 4E800421  bctrl
	ctx.lr = 0x8318A188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318A188: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 8318A18C: 4801E030  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318A190 size=128
    let mut pc: u32 = 0x8318A190;
    'dispatch: loop {
        match pc {
            0x8318A190 => {
    //   block [0x8318A190..0x8318A210)
	// 8318A190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A194: 4801DFD9  bl 0x831a816c
	ctx.lr = 0x8318A198;
	sub_831A8130(ctx, base);
	// 8318A198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A19C: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318A1A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A1A4: 7FEB3214  add r31, r11, r6
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8318A1A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8318A1AC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8318A1B0: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318A1B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318A1B8: 4BFFF9E1  bl 0x83189b98
	ctx.lr = 0x8318A1BC;
	sub_83189B98(ctx, base);
	// 8318A1BC: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8318A1C0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318A1C4: 409A000C  bne cr6, 0x8318a1d0
	if !ctx.cr[6].eq {
	pc = 0x8318A1D0; continue 'dispatch;
	}
	// 8318A1C8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8318A1CC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318A1D0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318A1D4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318A1D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318A1DC: 409A0028  bne cr6, 0x8318a204
	if !ctx.cr[6].eq {
	pc = 0x8318A204; continue 'dispatch;
	}
	// 8318A1E0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A1E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A1E8: 83DE0044  lwz r30, 0x44(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318A1EC: 38ABB874  addi r5, r11, -0x478c
	ctx.r[5].s64 = ctx.r[11].s64 + -18316;
	// 8318A1F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318A1F4: 4800063D  bl 0x8318a830
	ctx.lr = 0x8318A1F8;
	sub_8318A830(ctx, base);
	// 8318A1F8: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8318A1FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A200: 4801DFBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318A204: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8318A208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A20C: 4801DFB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A210 size=8
    let mut pc: u32 = 0x8318A210;
    'dispatch: loop {
        match pc {
            0x8318A210 => {
    //   block [0x8318A210..0x8318A218)
	// 8318A210: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8318A214: 4BFFFC64  b 0x83189e78
	sub_83189E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A218 size=164
    let mut pc: u32 = 0x8318A218;
    'dispatch: loop {
        match pc {
            0x8318A218 => {
    //   block [0x8318A218..0x8318A2BC)
	// 8318A218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A21C: 4801DF51  bl 0x831a816c
	ctx.lr = 0x8318A220;
	sub_831A8130(ctx, base);
	// 8318A220: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A228: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318A22C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8318A230: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8318A234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318A238: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8318A23C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318A240: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8318A244: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8318A248: 4200FFF8  bdnz 0x8318a240
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318A240; continue 'dispatch;
	}
	// 8318A24C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8318A250: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318A254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A258: 4BFFFD51  bl 0x83189fa8
	ctx.lr = 0x8318A25C;
	sub_83189FA8(ctx, base);
	// 8318A25C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8318A260: 409A000C  bne cr6, 0x8318a26c
	if !ctx.cr[6].eq {
	pc = 0x8318A26C; continue 'dispatch;
	}
	// 8318A264: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8318A268: 48000008  b 0x8318a270
	pc = 0x8318A270; continue 'dispatch;
	// 8318A26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318A270: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318A274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A278: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318A27C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8318A280: 480008D9  bl 0x8318ab58
	ctx.lr = 0x8318A284;
	sub_8318AB58(ctx, base);
	// 8318A284: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318A288: 409A000C  bne cr6, 0x8318a294
	if !ctx.cr[6].eq {
	pc = 0x8318A294; continue 'dispatch;
	}
	// 8318A28C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8318A290: 4BFFF8E1  bl 0x83189b70
	ctx.lr = 0x8318A294;
	sub_83189B70(ctx, base);
	// 8318A294: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8318A298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318A29C: 419A0018  beq cr6, 0x8318a2b4
	if ctx.cr[6].eq {
	pc = 0x8318A2B4; continue 'dispatch;
	}
	// 8318A2A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8318A2A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318A2A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8318A2AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318A2B0: 4E800421  bctrl
	ctx.lr = 0x8318A2B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8318A2B4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8318A2B8: 4801DF04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A2C0 size=56
    let mut pc: u32 = 0x8318A2C0;
    'dispatch: loop {
        match pc {
            0x8318A2C0 => {
    //   block [0x8318A2C0..0x8318A2F8)
	// 8318A2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A2C4: 4801DEA9  bl 0x831a816c
	ctx.lr = 0x8318A2C8;
	sub_831A8130(ctx, base);
	// 8318A2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318A2D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A2D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A2D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318A2DC: 4BFFFF3D  bl 0x8318a218
	ctx.lr = 0x8318A2E0;
	sub_8318A218(ctx, base);
	// 8318A2E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318A2E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A2EC: 4BFFFD8D  bl 0x8318a078
	ctx.lr = 0x8318A2F0;
	sub_8318A078(ctx, base);
	// 8318A2F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A2F4: 4801DEC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A2F8 size=88
    let mut pc: u32 = 0x8318A2F8;
    'dispatch: loop {
        match pc {
            0x8318A2F8 => {
    //   block [0x8318A2F8..0x8318A350)
	// 8318A2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A2FC: 4801DE6D  bl 0x831a8168
	ctx.lr = 0x8318A300;
	sub_831A8130(ctx, base);
	// 8318A300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A304: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8318A308: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8318A30C: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 8318A310: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8318A314: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 8318A318: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8318A31C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A320: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A324: 4BFFF835  bl 0x83189b58
	ctx.lr = 0x8318A328;
	sub_83189B58(ctx, base);
	// 8318A328: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8318A32C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8318A330: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8318A334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A33C: 4BFFFE55  bl 0x8318a190
	ctx.lr = 0x8318A340;
	sub_8318A190(ctx, base);
	// 8318A340: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318A344: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318A348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318A34C: 4801DE6C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A350 size=312
    let mut pc: u32 = 0x8318A350;
    'dispatch: loop {
        match pc {
            0x8318A350 => {
    //   block [0x8318A350..0x8318A3F4)
	// 8318A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A354: 4801DE19  bl 0x831a816c
	ctx.lr = 0x8318A358;
	sub_831A8130(ctx, base);
	// 8318A358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A35C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318A360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A364: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8318A368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A36C: 4BFFFA15  bl 0x83189d80
	ctx.lr = 0x8318A370;
	sub_83189D80(ctx, base);
	// 8318A370: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318A374: 409A010C  bne cr6, 0x8318a480
	if !ctx.cr[6].eq {
	pc = 0x8318A480; continue 'dispatch;
	}
	// 8318A378: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 8318A37C: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8318A380: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 8318A384: 419900E8  bgt cr6, 0x8318a46c
	if ctx.cr[6].gt {
	pc = 0x8318A46C; continue 'dispatch;
	}
	// 8318A388: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 8318A38C: 398CA3A0  addi r12, r12, -0x5c60
	ctx.r[12].s64 = ctx.r[12].s64 + -23648;
	// 8318A390: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8318A394: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8318A398: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8318A39C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8318A458; continue 'dispatch;
		},
		1 => {
	pc = 0x8318A408; continue 'dispatch;
		},
		2 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		3 => {
	pc = 0x8318A41C; continue 'dispatch;
		},
		4 => {
	pc = 0x8318A430; continue 'dispatch;
		},
		5 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		6 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		7 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		8 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		9 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		10 => {
	pc = 0x8318A3F4; continue 'dispatch;
		},
		11 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		12 => {
	pc = 0x8318A3F4; continue 'dispatch;
		},
		13 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		14 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		15 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		16 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		17 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		18 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		19 => {
	pc = 0x8318A46C; continue 'dispatch;
		},
		20 => {
	pc = 0x8318A444; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8318A3A0: 8318A458  lwz r24, -0x5ba8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23464 as u32) ) } as u64;
	// 8318A3A4: 8318A408  lwz r24, -0x5bf8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23544 as u32) ) } as u64;
	// 8318A3A8: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3AC: 8318A41C  lwz r24, -0x5be4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23524 as u32) ) } as u64;
	// 8318A3B0: 8318A430  lwz r24, -0x5bd0(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23504 as u32) ) } as u64;
	// 8318A3B4: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3B8: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3BC: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3C0: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3C4: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3C8: 8318A3F4  lwz r24, -0x5c0c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23564 as u32) ) } as u64;
	// 8318A3CC: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3D0: 8318A3F4  lwz r24, -0x5c0c(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23564 as u32) ) } as u64;
	// 8318A3D4: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3D8: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3DC: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3E0: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3E4: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3E8: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3EC: 8318A46C  lwz r24, -0x5b94(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23444 as u32) ) } as u64;
	// 8318A3F0: 8318A444  lwz r24, -0x5bbc(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23484 as u32) ) } as u64;
            }
            0x8318A3F4 => {
    //   block [0x8318A3F4..0x8318A408)
	// 8318A3F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A3FC: 4BFFFA6D  bl 0x83189e68
	ctx.lr = 0x8318A400;
	sub_83189E68(ctx, base);
	// 8318A400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A404: 4801DDB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A408 => {
    //   block [0x8318A408..0x8318A41C)
	// 8318A408: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A40C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A410: 4BFFFB01  bl 0x83189f10
	ctx.lr = 0x8318A414;
	sub_83189F10(ctx, base);
	// 8318A414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A418: 4801DDA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A41C => {
    //   block [0x8318A41C..0x8318A430)
	// 8318A41C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A424: 4BFFFAFD  bl 0x83189f20
	ctx.lr = 0x8318A428;
	sub_83189F20(ctx, base);
	// 8318A428: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A42C: 4801DD90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A430 => {
    //   block [0x8318A430..0x8318A444)
	// 8318A430: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A438: 4BFFFAF9  bl 0x83189f30
	ctx.lr = 0x8318A43C;
	sub_83189F30(ctx, base);
	// 8318A43C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A440: 4801DD7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A444 => {
    //   block [0x8318A444..0x8318A458)
	// 8318A444: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A44C: 4BFFFAF5  bl 0x83189f40
	ctx.lr = 0x8318A450;
	sub_83189F40(ctx, base);
	// 8318A450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A454: 4801DD68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A458 => {
    //   block [0x8318A458..0x8318A46C)
	// 8318A458: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A45C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A460: 4BFFFDB1  bl 0x8318a210
	ctx.lr = 0x8318A464;
	sub_8318A210(ctx, base);
	// 8318A464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A468: 4801DD54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A46C => {
    //   block [0x8318A46C..0x8318A488)
	// 8318A46C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A470: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A474: 38ABB8B4  addi r5, r11, -0x474c
	ctx.r[5].s64 = ctx.r[11].s64 + -18252;
	// 8318A478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A47C: 480003B5  bl 0x8318a830
	ctx.lr = 0x8318A480;
	sub_8318A830(ctx, base);
	// 8318A480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A484: 4801DD38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A488 size=4
    let mut pc: u32 = 0x8318A488;
    'dispatch: loop {
        match pc {
            0x8318A488 => {
    //   block [0x8318A488..0x8318A48C)
	// 8318A488: 4BFFFEC8  b 0x8318a350
	sub_8318A350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A490 size=484
    let mut pc: u32 = 0x8318A490;
    'dispatch: loop {
        match pc {
            0x8318A490 => {
    //   block [0x8318A490..0x8318A5BC)
	// 8318A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A494: 4801DCD9  bl 0x831a816c
	ctx.lr = 0x8318A498;
	sub_831A8130(ctx, base);
	// 8318A498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A49C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A4A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A4A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318A4A8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318A4AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318A4B0: 409A0018  bne cr6, 0x8318a4c8
	if !ctx.cr[6].eq {
	pc = 0x8318A4C8; continue 'dispatch;
	}
	// 8318A4B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A4B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318A4BC: 388BB924  addi r4, r11, -0x46dc
	ctx.r[4].s64 = ctx.r[11].s64 + -18140;
	// 8318A4C0: 4800E1C9  bl 0x83198688
	ctx.lr = 0x8318A4C4;
	sub_83198688(ctx, base);
	// 8318A4C4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8318A4C8: 2F030051  cmpwi cr6, r3, 0x51
	ctx.cr[6].compare_i32(ctx.r[3].s32, 81, &mut ctx.xer);
	// 8318A4CC: 41990134  bgt cr6, 0x8318a600
	if ctx.cr[6].gt {
	pc = 0x8318A600; continue 'dispatch;
	}
	// 8318A4D0: 419A016C  beq cr6, 0x8318a63c
	if ctx.cr[6].eq {
	pc = 0x8318A63C; continue 'dispatch;
	}
	// 8318A4D4: 3963FFEF  addi r11, r3, -0x11
	ctx.r[11].s64 = ctx.r[3].s64 + -17;
	// 8318A4D8: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8318A4DC: 4199013C  bgt cr6, 0x8318a618
	if ctx.cr[6].gt {
	pc = 0x8318A618; continue 'dispatch;
	}
	// 8318A4E0: 3D808319  lis r12, -0x7ce7
	ctx.r[12].s64 = -2095513600;
	// 8318A4E4: 398CA4F8  addi r12, r12, -0x5b08
	ctx.r[12].s64 = ctx.r[12].s64 + -23304;
	// 8318A4E8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8318A4EC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8318A4F0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8318A4F4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8318A5BC; continue 'dispatch;
		},
		1 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		2 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		3 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		4 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		5 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		6 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		7 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		8 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		9 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		10 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		11 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		12 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		13 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		14 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		15 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		16 => {
	pc = 0x8318A5D0; continue 'dispatch;
		},
		17 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		18 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		19 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		20 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		21 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		22 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		23 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		24 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		25 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		26 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		27 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		28 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		29 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		30 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		31 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		32 => {
	pc = 0x8318A5F8; continue 'dispatch;
		},
		33 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		34 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		35 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		36 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		37 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		38 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		39 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		40 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		41 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		42 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		43 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		44 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		45 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		46 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		47 => {
	pc = 0x8318A618; continue 'dispatch;
		},
		48 => {
	pc = 0x8318A63C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8318A4F8: 8318A5BC  lwz r24, -0x5a44(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23108 as u32) ) } as u64;
	// 8318A4FC: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A500: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A504: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A508: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A50C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A510: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A514: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A518: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A51C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A520: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A524: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A528: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A52C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A530: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A534: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A538: 8318A5D0  lwz r24, -0x5a30(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23088 as u32) ) } as u64;
	// 8318A53C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A540: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A544: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A548: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A54C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A550: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A554: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A558: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A55C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A560: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A564: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A568: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A56C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A570: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A574: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A578: 8318A5F8  lwz r24, -0x5a08(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23048 as u32) ) } as u64;
	// 8318A57C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A580: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A584: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A588: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A58C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A590: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A594: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A598: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A59C: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5A0: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5A4: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5A8: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5AC: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5B0: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5B4: 8318A618  lwz r24, -0x59e8(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-23016 as u32) ) } as u64;
	// 8318A5B8: 8318A63C  lwz r24, -0x59c4(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-22980 as u32) ) } as u64;
            }
            0x8318A5BC => {
    //   block [0x8318A5BC..0x8318A5D0)
	// 8318A5BC: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8318A5C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318A5C4: 419A0070  beq cr6, 0x8318a634
	if ctx.cr[6].eq {
	pc = 0x8318A634; continue 'dispatch;
	}
	// 8318A5C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8318A5CC: 48000090  b 0x8318a65c
	pc = 0x8318A65C; continue 'dispatch;
            }
            0x8318A5D0 => {
    //   block [0x8318A5D0..0x8318A5F8)
	// 8318A5D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318A5D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A5DC: 4BFFFEAD  bl 0x8318a488
	ctx.lr = 0x8318A5E0;
	sub_8318A488(ctx, base);
	// 8318A5E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318A5E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A5E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A5EC: 4BFFFCD5  bl 0x8318a2c0
	ctx.lr = 0x8318A5F0;
	sub_8318A2C0(ctx, base);
	// 8318A5F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A5F4: 4801DBC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            0x8318A5F8 => {
    //   block [0x8318A5F8..0x8318A618)
	// 8318A5F8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8318A5FC: 48000050  b 0x8318a64c
	pc = 0x8318A64C; continue 'dispatch;
	// 8318A600: 2F030061  cmpwi cr6, r3, 0x61
	ctx.cr[6].compare_i32(ctx.r[3].s32, 97, &mut ctx.xer);
	// 8318A604: 419A0038  beq cr6, 0x8318a63c
	if ctx.cr[6].eq {
	pc = 0x8318A63C; continue 'dispatch;
	}
	// 8318A608: 2F030101  cmpwi cr6, r3, 0x101
	ctx.cr[6].compare_i32(ctx.r[3].s32, 257, &mut ctx.xer);
	// 8318A60C: 419AFFBC  beq cr6, 0x8318a5c8
	if ctx.cr[6].eq {
	pc = 0x8318A5C8; continue 'dispatch;
	}
	// 8318A610: 2F031001  cmpwi cr6, r3, 0x1001
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4097, &mut ctx.xer);
	// 8318A614: 419A0020  beq cr6, 0x8318a634
	if ctx.cr[6].eq {
	pc = 0x8318A634; continue 'dispatch;
	}
            }
            0x8318A618 => {
    //   block [0x8318A618..0x8318A63C)
	// 8318A618: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A61C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A620: 38ABB8E8  addi r5, r11, -0x4718
	ctx.r[5].s64 = ctx.r[11].s64 + -18200;
	// 8318A624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A628: 48000209  bl 0x8318a830
	ctx.lr = 0x8318A62C;
	sub_8318A830(ctx, base);
	// 8318A62C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A630: 4801DB8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318A634: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8318A638: 48000014  b 0x8318a64c
	pc = 0x8318A64C; continue 'dispatch;
            }
            0x8318A63C => {
    //   block [0x8318A63C..0x8318A674)
	// 8318A63C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A644: 4BFFF705  bl 0x83189d48
	ctx.lr = 0x8318A648;
	sub_83189D48(ctx, base);
	// 8318A648: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8318A64C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A654: 4BFFFE35  bl 0x8318a488
	ctx.lr = 0x8318A658;
	sub_8318A488(ctx, base);
	// 8318A658: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8318A65C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318A660: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318A664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A668: 4BFFFBB1  bl 0x8318a218
	ctx.lr = 0x8318A66C;
	sub_8318A218(ctx, base);
	// 8318A66C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A670: 4801DB4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A678 size=12
    let mut pc: u32 = 0x8318A678;
    'dispatch: loop {
        match pc {
            0x8318A678 => {
    //   block [0x8318A678..0x8318A684)
	// 8318A678: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A67C: 386BB92C  addi r3, r11, -0x46d4
	ctx.r[3].s64 = ctx.r[11].s64 + -18132;
	// 8318A680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A688 size=80
    let mut pc: u32 = 0x8318A688;
    'dispatch: loop {
        match pc {
            0x8318A688 => {
    //   block [0x8318A688..0x8318A6D8)
	// 8318A688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318A690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318A694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A698: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 8318A69C: 817FD79C  lwz r11, -0x2864(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-10340 as u32) ) } as u64;
	// 8318A6A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318A6A4: 40990020  ble cr6, 0x8318a6c4
	if !ctx.cr[6].gt {
	pc = 0x8318A6C4; continue 'dispatch;
	}
	// 8318A6A8: 4BF3D439  bl 0x830c7ae0
	ctx.lr = 0x8318A6AC;
	sub_830C7AE0(ctx, base);
	// 8318A6AC: 4BF3D435  bl 0x830c7ae0
	ctx.lr = 0x8318A6B0;
	sub_830C7AE0(ctx, base);
	// 8318A6B0: 4800E001  bl 0x831986b0
	ctx.lr = 0x8318A6B4;
	sub_831986B0(ctx, base);
	// 8318A6B4: 4BF3D42D  bl 0x830c7ae0
	ctx.lr = 0x8318A6B8;
	sub_830C7AE0(ctx, base);
	// 8318A6B8: 817FD79C  lwz r11, -0x2864(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-10340 as u32) ) } as u64;
	// 8318A6BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318A6C0: 917FD79C  stw r11, -0x2864(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-10340 as u32), ctx.r[11].u32 ) };
	// 8318A6C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A6D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A6D8 size=20
    let mut pc: u32 = 0x8318A6D8;
    'dispatch: loop {
        match pc {
            0x8318A6D8 => {
    //   block [0x8318A6D8..0x8318A6EC)
	// 8318A6D8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A6DC: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 8318A6E0: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8318A6E4: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8318A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A6F0 size=40
    let mut pc: u32 = 0x8318A6F0;
    'dispatch: loop {
        match pc {
            0x8318A6F0 => {
    //   block [0x8318A6F0..0x8318A718)
	// 8318A6F0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A6F4: 394B9BE0  addi r10, r11, -0x6420
	ctx.r[10].s64 = ctx.r[11].s64 + -25632;
	// 8318A6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318A6FC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318A700: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318A704: 40990024  ble cr6, 0x8318a728
	if !ctx.cr[6].gt {
		sub_8318A718(ctx, base);
		return;
	}
	// 8318A708: 386A0018  addi r3, r10, 0x18
	ctx.r[3].s64 = ctx.r[10].s64 + 24;
	// 8318A70C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318A710: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318A714: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A718 size=24
    let mut pc: u32 = 0x8318A718;
    'dispatch: loop {
        match pc {
            0x8318A718 => {
    //   block [0x8318A718..0x8318A730)
	// 8318A718: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318A71C: 3863009C  addi r3, r3, 0x9c
	ctx.r[3].s64 = ctx.r[3].s64 + 156;
	// 8318A720: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8318A724: 4198FFE8  blt cr6, 0x8318a70c
	if ctx.cr[6].lt {
		sub_8318A6F0(ctx, base);
		return;
	}
	// 8318A728: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A730 size=136
    let mut pc: u32 = 0x8318A730;
    'dispatch: loop {
        match pc {
            0x8318A730 => {
    //   block [0x8318A730..0x8318A7B8)
	// 8318A730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A734: 4801DA39  bl 0x831a816c
	ctx.lr = 0x8318A738;
	sub_831A8130(ctx, base);
	// 8318A738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A73C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318A740: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318A744: 38A0009C  li r5, 0x9c
	ctx.r[5].s64 = 156;
	// 8318A748: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318A74C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A750: 4801DA91  bl 0x831a81e0
	ctx.lr = 0x8318A754;
	sub_831A81E0(ctx, base);
	// 8318A754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318A758: 395E001F  addi r10, r30, 0x1f
	ctx.r[10].s64 = ctx.r[30].s64 + 31;
	// 8318A75C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8318A760: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8318A764: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8318A768: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8318A76C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8318A770: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318A774: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318A778: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8318A77C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318A780: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8318A784: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8318A788: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 8318A78C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8318A790: 394B0400  addi r10, r11, 0x400
	ctx.r[10].s64 = ctx.r[11].s64 + 1024;
	// 8318A794: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8318A798: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 8318A79C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318A7A0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8318A7A4: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 8318A7A8: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8318A7AC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318A7B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318A7B4: 4801DA08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A7B8 size=12
    let mut pc: u32 = 0x8318A7B8;
    'dispatch: loop {
        match pc {
            0x8318A7B8 => {
    //   block [0x8318A7B8..0x8318A7C4)
	// 8318A7B8: 2F03201F  cmpwi cr6, r3, 0x201f
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8223, &mut ctx.xer);
	// 8318A7BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318A7C0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A7C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A7C4 size=8
    let mut pc: u32 = 0x8318A7C4;
    'dispatch: loop {
        match pc {
            0x8318A7C4 => {
    //   block [0x8318A7C4..0x8318A7CC)
	// 8318A7C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A7D0 size=92
    let mut pc: u32 = 0x8318A7D0;
    'dispatch: loop {
        match pc {
            0x8318A7D0 => {
    //   block [0x8318A7D0..0x8318A82C)
	// 8318A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318A7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318A7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A7E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318A7E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318A7E8: 419A0030  beq cr6, 0x8318a818
	if ctx.cr[6].eq {
	pc = 0x8318A818; continue 'dispatch;
	}
	// 8318A7EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8318A7F0: 83EB0030  lwz r31, 0x30(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8318A7F4: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318A7F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318A7FC: 480006A5  bl 0x8318aea0
	ctx.lr = 0x8318A800;
	sub_8318AEA0(ctx, base);
	// 8318A800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A804: 4800DD45  bl 0x83198548
	ctx.lr = 0x8318A808;
	sub_83198548(ctx, base);
	// 8318A808: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A80C: 814B9BE0  lwz r10, -0x6420(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25632 as u32) ) } as u64;
	// 8318A810: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318A814: 914B9BE0  stw r10, -0x6420(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25632 as u32), ctx.r[10].u32 ) };
	// 8318A818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A830 size=32
    let mut pc: u32 = 0x8318A830;
    'dispatch: loop {
        match pc {
            0x8318A830 => {
    //   block [0x8318A830..0x8318A850)
	// 8318A830: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A834: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 8318A838: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318A83C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318A840: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8318A844: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318A848: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318A84C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A850 size=20
    let mut pc: u32 = 0x8318A850;
    'dispatch: loop {
        match pc {
            0x8318A850 => {
    //   block [0x8318A850..0x8318A864)
	// 8318A850: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318A854: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8318A858: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8318A85C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8318A860: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A864 size=4
    let mut pc: u32 = 0x8318A864;
    'dispatch: loop {
        match pc {
            0x8318A864 => {
    //   block [0x8318A864..0x8318A868)
	// 8318A864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A868 size=16
    let mut pc: u32 = 0x8318A868;
    'dispatch: loop {
        match pc {
            0x8318A868 => {
    //   block [0x8318A868..0x8318A878)
	// 8318A868: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A86C: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 8318A870: 906B0014  stw r3, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8318A874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318A878 size=16
    let mut pc: u32 = 0x8318A878;
    'dispatch: loop {
        match pc {
            0x8318A878 => {
    //   block [0x8318A878..0x8318A888)
	// 8318A878: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A87C: 396B9BE0  addi r11, r11, -0x6420
	ctx.r[11].s64 = ctx.r[11].s64 + -25632;
	// 8318A880: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A888 size=80
    let mut pc: u32 = 0x8318A888;
    'dispatch: loop {
        match pc {
            0x8318A888 => {
    //   block [0x8318A888..0x8318A8D8)
	// 8318A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318A890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318A894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A898: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A89C: 38A00508  li r5, 0x508
	ctx.r[5].s64 = 1288;
	// 8318A8A0: 3BEB9BE0  addi r31, r11, -0x6420
	ctx.r[31].s64 = ctx.r[11].s64 + -25632;
	// 8318A8A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318A8A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A8AC: 4801D935  bl 0x831a81e0
	ctx.lr = 0x8318A8B0;
	sub_831A81E0(ctx, base);
	// 8318A8B0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8318A8B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8318A8B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318A8BC: 4BFFFFAD  bl 0x8318a868
	ctx.lr = 0x8318A8C0;
	sub_8318A868(ctx, base);
	// 8318A8C0: 4800DDD1  bl 0x83198690
	ctx.lr = 0x8318A8C4;
	sub_83198690(ctx, base);
	// 8318A8C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A8D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318A8D8 size=296
    let mut pc: u32 = 0x8318A8D8;
    'dispatch: loop {
        match pc {
            0x8318A8D8 => {
    //   block [0x8318A8D8..0x8318AA00)
	// 8318A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318A8E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318A8E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318A8E8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8318A8EC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8318A8F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8318A8F4: 4BFFFDFD  bl 0x8318a6f0
	ctx.lr = 0x8318A8F8;
	sub_8318A6F0(ctx, base);
	// 8318A8F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318A8FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8318A900: 419A002C  beq cr6, 0x8318a92c
	if ctx.cr[6].eq {
	pc = 0x8318A92C; continue 'dispatch;
	}
	// 8318A904: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318A908: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8318A90C: 4BFFFEAD  bl 0x8318a7b8
	ctx.lr = 0x8318A910;
	sub_8318A7B8(ctx, base);
	// 8318A910: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318A914: 419A0030  beq cr6, 0x8318a944
	if ctx.cr[6].eq {
	pc = 0x8318A944; continue 'dispatch;
	}
	// 8318A918: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A91C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318A920: 38ABB9A4  addi r5, r11, -0x465c
	ctx.r[5].s64 = ctx.r[11].s64 + -18012;
	// 8318A924: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A928: 4BFFFF09  bl 0x8318a830
	ctx.lr = 0x8318A92C;
	sub_8318A830(ctx, base);
	// 8318A92C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A940: 4E800020  blr
	return;
	// 8318A944: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8318A948: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8318A94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A950: 4BFFFDE1  bl 0x8318a730
	ctx.lr = 0x8318A954;
	sub_8318A730(ctx, base);
	// 8318A954: 480009E5  bl 0x8318b338
	ctx.lr = 0x8318A958;
	sub_8318B338(ctx, base);
	// 8318A958: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318A95C: 409A0034  bne cr6, 0x8318a990
	if !ctx.cr[6].eq {
	pc = 0x8318A990; continue 'dispatch;
	}
	// 8318A960: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318A968: 38ABB984  addi r5, r11, -0x467c
	ctx.r[5].s64 = ctx.r[11].s64 + -18044;
	// 8318A96C: 4BFFFEC5  bl 0x8318a830
	ctx.lr = 0x8318A970;
	sub_8318A830(ctx, base);
	// 8318A970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A974: 4BFFFE5D  bl 0x8318a7d0
	ctx.lr = 0x8318A978;
	sub_8318A7D0(ctx, base);
	// 8318A978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A98C: 4E800020  blr
	return;
	// 8318A990: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8318A994: 4800DC9D  bl 0x83198630
	ctx.lr = 0x8318A998;
	sub_83198630(ctx, base);
	// 8318A998: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8318A99C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318A9A0: 409A0034  bne cr6, 0x8318a9d4
	if !ctx.cr[6].eq {
	pc = 0x8318A9D4; continue 'dispatch;
	}
	// 8318A9A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318A9A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318A9AC: 38ABB964  addi r5, r11, -0x469c
	ctx.r[5].s64 = ctx.r[11].s64 + -18076;
	// 8318A9B0: 4BFFFE81  bl 0x8318a830
	ctx.lr = 0x8318A9B4;
	sub_8318A830(ctx, base);
	// 8318A9B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A9B8: 4BFFFE19  bl 0x8318a7d0
	ctx.lr = 0x8318A9BC;
	sub_8318A7D0(ctx, base);
	// 8318A9BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318A9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A9CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A9D0: 4E800020  blr
	return;
	// 8318A9D4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8318A9D8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318A9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318A9E0: 814B9BE0  lwz r10, -0x6420(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25632 as u32) ) } as u64;
	// 8318A9E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318A9E8: 914B9BE0  stw r10, -0x6420(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25632 as u32), ctx.r[10].u32 ) };
	// 8318A9EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318A9F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318A9F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318A9F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AA00 size=104
    let mut pc: u32 = 0x8318AA00;
    'dispatch: loop {
        match pc {
            0x8318AA00 => {
    //   block [0x8318AA00..0x8318AA68)
	// 8318AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318AA0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AA10: 3FE08340  lis r31, -0x7cc0
	ctx.r[31].s64 = -2092957696;
	// 8318AA14: 817FD79C  lwz r11, -0x2864(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-10340 as u32) ) } as u64;
	// 8318AA18: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318AA1C: 40980038  bge cr6, 0x8318aa54
	if !ctx.cr[6].lt {
	pc = 0x8318AA54; continue 'dispatch;
	}
	// 8318AA20: 4BFFFC59  bl 0x8318a678
	ctx.lr = 0x8318AA24;
	sub_8318A678(ctx, base);
	// 8318AA24: 3D608340  lis r11, -0x7cc0
	ctx.r[11].s64 = -2092957696;
	// 8318AA28: 906BD798  stw r3, -0x2868(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-10344 as u32), ctx.r[3].u32 ) };
	// 8318AA2C: 4BFFFE5D  bl 0x8318a888
	ctx.lr = 0x8318AA30;
	sub_8318A888(ctx, base);
	// 8318AA30: 4800DC79  bl 0x831986a8
	ctx.lr = 0x8318AA34;
	sub_831986A8(ctx, base);
	// 8318AA34: 48000CA5  bl 0x8318b6d8
	ctx.lr = 0x8318AA38;
	sub_8318B6D8(ctx, base);
	// 8318AA38: 4800DBF1  bl 0x83198628
	ctx.lr = 0x8318AA3C;
	sub_83198628(ctx, base);
	// 8318AA3C: 817FD79C  lwz r11, -0x2864(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-10340 as u32) ) } as u64;
	// 8318AA40: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 8318AA44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318AA48: 917FD79C  stw r11, -0x2864(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-10340 as u32), ctx.r[11].u32 ) };
	// 8318AA4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AA50: 916AD7A0  stw r11, -0x2860(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10336 as u32), ctx.r[11].u32 ) };
	// 8318AA54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318AA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318AA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AA68 size=8
    let mut pc: u32 = 0x8318AA68;
    'dispatch: loop {
        match pc {
            0x8318AA68 => {
    //   block [0x8318AA68..0x8318AA70)
	// 8318AA68: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8318AA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AA70 size=12
    let mut pc: u32 = 0x8318AA70;
    'dispatch: loop {
        match pc {
            0x8318AA70 => {
    //   block [0x8318AA70..0x8318AA7C)
	// 8318AA70: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8318AA74: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8318AA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AA80 size=148
    let mut pc: u32 = 0x8318AA80;
    'dispatch: loop {
        match pc {
            0x8318AA80 => {
    //   block [0x8318AA80..0x8318AB14)
	// 8318AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318AA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318AA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AA94: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8318AA98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318AA9C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8318AAA0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8318AAA4: 38A9A7E0  addi r5, r9, -0x5820
	ctx.r[5].s64 = ctx.r[9].s64 + -22560;
	// 8318AAA8: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8318AAAC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8318AAB0: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318AAB4: 3889B9D0  addi r4, r9, -0x4630
	ctx.r[4].s64 = ctx.r[9].s64 + -17968;
	// 8318AAB8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8318AABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318AAC0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318AAC4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8318AAC8: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8318AACC: 4BFAA7CD  bl 0x83135298
	ctx.lr = 0x8318AAD0;
	sub_83135298(ctx, base);
	// 8318AAD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318AAD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8318AAD8: 409A0010  bne cr6, 0x8318aae8
	if !ctx.cr[6].eq {
	pc = 0x8318AAE8; continue 'dispatch;
	}
	// 8318AADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8318AAE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318AAE4: 4800000C  b 0x8318aaf0
	pc = 0x8318AAF0; continue 'dispatch;
	// 8318AAE8: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318AAEC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318AAF0: 480008A1  bl 0x8318b390
	ctx.lr = 0x8318AAF4;
	sub_8318B390(ctx, base);
	// 8318AAF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318AAF8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8318AAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318AB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AB08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318AB0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318AB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AB18 size=28
    let mut pc: u32 = 0x8318AB18;
    'dispatch: loop {
        match pc {
            0x8318AB18 => {
    //   block [0x8318AB18..0x8318AB34)
	// 8318AB18: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8318AB1C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318AB20: 419A0014  beq cr6, 0x8318ab34
	if ctx.cr[6].eq {
		sub_8318AB34(ctx, base);
		return;
	}
	// 8318AB24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AB28: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AB2C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AB34 size=20
    let mut pc: u32 = 0x8318AB34;
    'dispatch: loop {
        match pc {
            0x8318AB34 => {
    //   block [0x8318AB34..0x8318AB48)
	// 8318AB34: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8318AB38: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AB3C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8318AB40: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AB48 size=12
    let mut pc: u32 = 0x8318AB48;
    'dispatch: loop {
        match pc {
            0x8318AB48 => {
    //   block [0x8318AB48..0x8318AB54)
	// 8318AB48: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 8318AB4C: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8318AB50: 48000858  b 0x8318b3a8
	sub_8318B3A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AB58 size=8
    let mut pc: u32 = 0x8318AB58;
    'dispatch: loop {
        match pc {
            0x8318AB58 => {
    //   block [0x8318AB58..0x8318AB60)
	// 8318AB58: 80630064  lwz r3, 0x64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 8318AB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AB60 size=40
    let mut pc: u32 = 0x8318AB60;
    'dispatch: loop {
        match pc {
            0x8318AB60 => {
    //   block [0x8318AB60..0x8318AB88)
	// 8318AB60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318AB64: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318AB68: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8318AB6C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318AB70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8318AB74: 7D444850  subf r10, r4, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8318AB78: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8318AB7C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8318AB80: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8318AB88 size=132
    let mut pc: u32 = 0x8318AB88;
    'dispatch: loop {
        match pc {
            0x8318AB88 => {
    //   block [0x8318AB88..0x8318AC0C)
	// 8318AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AB90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AB94: 7CAB0E70  srawi r11, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 8318AB98: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8318AB9C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318ABA0: 7C8A0E70  srawi r10, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8318ABA4: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8318ABA8: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8318ABAC: 38680004  addi r3, r8, 4
	ctx.r[3].s64 = ctx.r[8].s64 + 4;
	// 8318ABB0: 5566083C  slwi r6, r11, 1
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8318ABB4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8318ABB8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318ABBC: 4BFFFFA5  bl 0x8318ab60
	ctx.lr = 0x8318ABC0;
	sub_8318AB60(ctx, base);
	// 8318ABC0: 7CCB0E70  srawi r11, r6, 1
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[6].s32 >> 1) as i64;
	// 8318ABC4: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 8318ABC8: 7CAB0194  addze r5, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[5].s64 = tmp.s64;
	// 8318ABCC: 7CEB0E70  srawi r11, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 8318ABD0: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 8318ABD4: 4BFFFF8D  bl 0x8318ab60
	ctx.lr = 0x8318ABD8;
	sub_8318AB60(ctx, base);
	// 8318ABD8: 38680024  addi r3, r8, 0x24
	ctx.r[3].s64 = ctx.r[8].s64 + 36;
	// 8318ABDC: 4BFFFF85  bl 0x8318ab60
	ctx.lr = 0x8318ABE0;
	sub_8318AB60(ctx, base);
	// 8318ABE0: 81680044  lwz r11, 0x44(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(68 as u32) ) } as u64;
	// 8318ABE4: 38680044  addi r3, r8, 0x44
	ctx.r[3].s64 = ctx.r[8].s64 + 68;
	// 8318ABE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318ABEC: 419A0010  beq cr6, 0x8318abfc
	if ctx.cr[6].eq {
	pc = 0x8318ABFC; continue 'dispatch;
	}
	// 8318ABF0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8318ABF4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8318ABF8: 4BFFFF69  bl 0x8318ab60
	ctx.lr = 0x8318ABFC;
	sub_8318AB60(ctx, base);
	// 8318ABFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318AC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AC10 size=12
    let mut pc: u32 = 0x8318AC10;
    'dispatch: loop {
        match pc {
            0x8318AC10 => {
    //   block [0x8318AC10..0x8318AC1C)
	// 8318AC10: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318AC14: 386BB9D8  addi r3, r11, -0x4628
	ctx.r[3].s64 = ctx.r[11].s64 + -17960;
	// 8318AC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AC20 size=16
    let mut pc: u32 = 0x8318AC20;
    'dispatch: loop {
        match pc {
            0x8318AC20 => {
    //   block [0x8318AC20..0x8318AC30)
	// 8318AC20: 3D408340  lis r10, -0x7cc0
	ctx.r[10].s64 = -2092957696;
	// 8318AC24: 816AD7A8  lwz r11, -0x2858(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10328 as u32) ) } as u64;
	// 8318AC28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8318AC2C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AC30 size=12
    let mut pc: u32 = 0x8318AC30;
    'dispatch: loop {
        match pc {
            0x8318AC30 => {
    //   block [0x8318AC30..0x8318AC3C)
	// 8318AC30: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318AC34: 916AD7A8  stw r11, -0x2858(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10328 as u32), ctx.r[11].u32 ) };
	// 8318AC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AC40 size=92
    let mut pc: u32 = 0x8318AC40;
    'dispatch: loop {
        match pc {
            0x8318AC40 => {
    //   block [0x8318AC40..0x8318AC9C)
	// 8318AC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AC4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318AC50: 419A0038  beq cr6, 0x8318ac88
	if ctx.cr[6].eq {
	pc = 0x8318AC88; continue 'dispatch;
	}
	// 8318AC54: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8318AC58: 41980030  blt cr6, 0x8318ac88
	if ctx.cr[6].lt {
	pc = 0x8318AC88; continue 'dispatch;
	}
	// 8318AC5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8318AC60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318AC64: 388BE074  addi r4, r11, -0x1f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -8076;
	// 8318AC68: 38630012  addi r3, r3, 0x12
	ctx.r[3].s64 = ctx.r[3].s64 + 18;
	// 8318AC6C: 48022165  bl 0x831acdd0
	ctx.lr = 0x8318AC70;
	sub_831ACDD0(ctx, base);
	// 8318AC70: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8318AC74: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318AC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318AC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AC84: 4E800020  blr
	return;
	// 8318AC88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318AC8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318AC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318ACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318ACA0 size=92
    let mut pc: u32 = 0x8318ACA0;
    'dispatch: loop {
        match pc {
            0x8318ACA0 => {
    //   block [0x8318ACA0..0x8318ACFC)
	// 8318ACA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318ACA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318ACA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318ACAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318ACB0: 419A0038  beq cr6, 0x8318ace8
	if ctx.cr[6].eq {
	pc = 0x8318ACE8; continue 'dispatch;
	}
	// 8318ACB4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8318ACB8: 41980030  blt cr6, 0x8318ace8
	if ctx.cr[6].lt {
	pc = 0x8318ACE8; continue 'dispatch;
	}
	// 8318ACBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8318ACC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8318ACC4: 388BD080  addi r4, r11, -0x2f80
	ctx.r[4].s64 = ctx.r[11].s64 + -12160;
	// 8318ACC8: 38630013  addi r3, r3, 0x13
	ctx.r[3].s64 = ctx.r[3].s64 + 19;
	// 8318ACCC: 48022105  bl 0x831acdd0
	ctx.lr = 0x8318ACD0;
	sub_831ACDD0(ctx, base);
	// 8318ACD0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8318ACD4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8318ACD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318ACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318ACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318ACE4: 4E800020  blr
	return;
	// 8318ACE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318ACEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318ACF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318ACF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318ACF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8318AD00 size=16
    let mut pc: u32 = 0x8318AD00;
    'dispatch: loop {
        match pc {
            0x8318AD00 => {
    //   block [0x8318AD00..0x8318AD10)
	// 8318AD00: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 8318AD04: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8318AD08: 71630023  andi. r3, r11, 0x23
	ctx.r[3].u64 = ctx.r[11].u64 & 35;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8318AD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AD10 size=64
    let mut pc: u32 = 0x8318AD10;
    'dispatch: loop {
        match pc {
            0x8318AD10 => {
    //   block [0x8318AD10..0x8318AD50)
	// 8318AD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AD18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AD1C: 3D008340  lis r8, -0x7cc0
	ctx.r[8].s64 = -2092957696;
	// 8318AD20: 8148D7A8  lwz r10, -0x2858(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10328 as u32) ) } as u64;
	// 8318AD24: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8318AD28: 40980018  bge cr6, 0x8318ad40
	if !ctx.cr[6].lt {
	pc = 0x8318AD40; continue 'dispatch;
	}
	// 8318AD2C: 4BFFFEE5  bl 0x8318ac10
	ctx.lr = 0x8318AD30;
	sub_8318AC10(ctx, base);
	// 8318AD30: 3D208340  lis r9, -0x7cc0
	ctx.r[9].s64 = -2092957696;
	// 8318AD34: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8318AD38: 9069D7A4  stw r3, -0x285c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-10332 as u32), ctx.r[3].u32 ) };
	// 8318AD3C: 9168D7A8  stw r11, -0x2858(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-10328 as u32), ctx.r[11].u32 ) };
	// 8318AD40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318AD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AD50 size=228
    let mut pc: u32 = 0x8318AD50;
    'dispatch: loop {
        match pc {
            0x8318AD50 => {
    //   block [0x8318AD50..0x8318AE34)
	// 8318AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AD58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318AD5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318AD60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AD64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AD68: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8318AD6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318AD70: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AD74: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AD78: 419A00A4  beq cr6, 0x8318ae1c
	if ctx.cr[6].eq {
	pc = 0x8318AE1C; continue 'dispatch;
	}
	// 8318AD7C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8318AD80: 4099009C  ble cr6, 0x8318ae1c
	if !ctx.cr[6].gt {
	pc = 0x8318AE1C; continue 'dispatch;
	}
	// 8318AD84: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318AD88: 3BEBBA10  addi r31, r11, -0x45f0
	ctx.r[31].s64 = ctx.r[11].s64 + -17904;
	// 8318AD8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8318AD90: 3BCB160C  addi r30, r11, 0x160c
	ctx.r[30].s64 = ctx.r[11].s64 + 5644;
	// 8318AD94: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8318AD98: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8318AD9C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 8318ADA0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ADA4: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ADA8: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318ADAC: 40820014  bne 0x8318adc0
	if !ctx.cr[0].eq {
	pc = 0x8318ADC0; continue 'dispatch;
	}
	// 8318ADB0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318ADB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318ADB8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8318ADBC: 409AFFE4  bne cr6, 0x8318ada0
	if !ctx.cr[6].eq {
	pc = 0x8318ADA0; continue 'dispatch;
	}
	// 8318ADC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318ADC4: 409A0048  bne cr6, 0x8318ae0c
	if !ctx.cr[6].eq {
	pc = 0x8318AE0C; continue 'dispatch;
	}
	// 8318ADC8: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8318ADCC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8318ADD0: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 8318ADD4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ADD8: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318ADDC: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318ADE0: 40820014  bne 0x8318adf4
	if !ctx.cr[0].eq {
	pc = 0x8318ADF4; continue 'dispatch;
	}
	// 8318ADE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318ADE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318ADEC: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8318ADF0: 409AFFE4  bne cr6, 0x8318add4
	if !ctx.cr[6].eq {
	pc = 0x8318ADD4; continue 'dispatch;
	}
	// 8318ADF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318ADF8: 409A0014  bne cr6, 0x8318ae0c
	if !ctx.cr[6].eq {
	pc = 0x8318AE0C; continue 'dispatch;
	}
	// 8318ADFC: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 8318AE00: 90E50000  stw r7, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318AE04: 4BFFFEFD  bl 0x8318ad00
	ctx.lr = 0x8318AE08;
	sub_8318AD00(ctx, base);
	// 8318AE08: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8318AE0C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8318AE10: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8318AE14: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8318AE18: 409AFF7C  bne cr6, 0x8318ad94
	if !ctx.cr[6].eq {
	pc = 0x8318AD94; continue 'dispatch;
	}
	// 8318AE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8318AE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318AE28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318AE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318AE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AE38 size=40
    let mut pc: u32 = 0x8318AE38;
    'dispatch: loop {
        match pc {
            0x8318AE38 => {
    //   block [0x8318AE38..0x8318AE60)
	// 8318AE38: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318AE3C: 394B9960  addi r10, r11, -0x66a0
	ctx.r[10].s64 = ctx.r[11].s64 + -26272;
	// 8318AE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AE44: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8318AE48: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8318AE4C: 40990024  ble cr6, 0x8318ae70
	if !ctx.cr[6].gt {
		sub_8318AE60(ctx, base);
		return;
	}
	// 8318AE50: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 8318AE54: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318AE58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8318AE5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AE60 size=24
    let mut pc: u32 = 0x8318AE60;
    'dispatch: loop {
        match pc {
            0x8318AE60 => {
    //   block [0x8318AE60..0x8318AE78)
	// 8318AE60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318AE64: 3863004C  addi r3, r3, 0x4c
	ctx.r[3].s64 = ctx.r[3].s64 + 76;
	// 8318AE68: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8318AE6C: 4198FFE8  blt cr6, 0x8318ae54
	if ctx.cr[6].lt {
		sub_8318AE38(ctx, base);
		return;
	}
	// 8318AE70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318AE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8318AE78 size=36
    let mut pc: u32 = 0x8318AE78;
    'dispatch: loop {
        match pc {
            0x8318AE78 => {
    //   block [0x8318AE78..0x8318AE9C)
	// 8318AE78: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8318AE7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AE80: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8318AE84: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8318AE88: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8318AE8C: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8318AE90: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8318AE94: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8318AE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AEA0 size=8
    let mut pc: u32 = 0x8318AEA0;
    'dispatch: loop {
        match pc {
            0x8318AEA0 => {
    //   block [0x8318AEA0..0x8318AEA8)
	// 8318AEA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318AEA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318AEA8 size=28
    let mut pc: u32 = 0x8318AEA8;
    'dispatch: loop {
        match pc {
            0x8318AEA8 => {
    //   block [0x8318AEA8..0x8318AEC4)
	// 8318AEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AEAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AEB0: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318AEB4: 814B9960  lwz r10, -0x66a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26272 as u32) ) } as u64;
	// 8318AEB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8318AEBC: 914B9960  stw r10, -0x66a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26272 as u32), ctx.r[10].u32 ) };
	// 8318AEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AEC8 size=188
    let mut pc: u32 = 0x8318AEC8;
    'dispatch: loop {
        match pc {
            0x8318AEC8 => {
    //   block [0x8318AEC8..0x8318AF84)
	// 8318AEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AECC: 4801D2A1  bl 0x831a816c
	ctx.lr = 0x8318AED0;
	sub_831A8130(ctx, base);
	// 8318AED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318AED8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8318AEDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8318AEE0: 409A002C  bne cr6, 0x8318af0c
	if !ctx.cr[6].eq {
	pc = 0x8318AF0C; continue 'dispatch;
	}
	// 8318AEE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AEE8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8318AEEC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8318AEF0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8318AEF4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8318AEF8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8318AEFC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318AF00: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 8318AF04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318AF08: 4801D2B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8318AF0C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318AF10: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8318AF14: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318AF18: 3BCBA7E0  addi r30, r11, -0x5820
	ctx.r[30].s64 = ctx.r[11].s64 + -22560;
	// 8318AF1C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8318AF20: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8318AF24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318AF28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8318AF2C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318AF30: 388BBA24  addi r4, r11, -0x45dc
	ctx.r[4].s64 = ctx.r[11].s64 + -17884;
	// 8318AF34: 4BFAA365  bl 0x83135298
	ctx.lr = 0x8318AF38;
	sub_83135298(ctx, base);
	// 8318AF38: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318AF3C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318AF40: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8318AF44: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318AF48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318AF4C: 388BBA1C  addi r4, r11, -0x45e4
	ctx.r[4].s64 = ctx.r[11].s64 + -17892;
	// 8318AF50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318AF54: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8318AF58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8318AF5C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8318AF60: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8318AF64: 4BFAA335  bl 0x83135298
	ctx.lr = 0x8318AF68;
	sub_83135298(ctx, base);
	// 8318AF68: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318AF6C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8318AF70: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 8318AF74: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8318AF78: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8318AF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318AF80: 4801D23C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318AF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318AF88 size=132
    let mut pc: u32 = 0x8318AF88;
    'dispatch: loop {
        match pc {
            0x8318AF88 => {
    //   block [0x8318AF88..0x8318B00C)
	// 8318AF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318AF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318AF90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8318AF94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318AF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318AF9C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8318AFA0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8318AFA4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8318AFA8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8318AFAC: 409A003C  bne cr6, 0x8318afe8
	if !ctx.cr[6].eq {
	pc = 0x8318AFE8; continue 'dispatch;
	}
	// 8318AFB0: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8318AFB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318AFB8: 419A0030  beq cr6, 0x8318afe8
	if ctx.cr[6].eq {
	pc = 0x8318AFE8; continue 'dispatch;
	}
	// 8318AFBC: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8318AFC0: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8318AFC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8318AFC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8318AFCC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8318AFD0: 4BFAA2C9  bl 0x83135298
	ctx.lr = 0x8318AFD4;
	sub_83135298(ctx, base);
	// 8318AFD4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318AFD8: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318AFDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AFE0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318AFE4: 48000010  b 0x8318aff4
	pc = 0x8318AFF4; continue 'dispatch;
	// 8318AFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318AFEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AFF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318AFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318AFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318AFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8318B004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B010 size=128
    let mut pc: u32 = 0x8318B010;
    'dispatch: loop {
        match pc {
            0x8318B010 => {
    //   block [0x8318B010..0x8318B090)
	// 8318B010: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8318B014: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8318B018: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8318B01C: 6127FFFF  ori r7, r9, 0xffff
	ctx.r[7].u64 = ctx.r[9].u64 | 65535;
	// 8318B020: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318B024: 409A0008  bne cr6, 0x8318b02c
	if !ctx.cr[6].eq {
	pc = 0x8318B02C; continue 'dispatch;
	}
	// 8318B028: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 8318B02C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318B030: 409A0008  bne cr6, 0x8318b038
	if !ctx.cr[6].eq {
	pc = 0x8318B038; continue 'dispatch;
	}
	// 8318B034: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8318B038: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8318B03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8318B040: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8318B044: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8318B048: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318B04C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B050: 4200FFF8  bdnz 0x8318b048
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B048; continue 'dispatch;
	}
	// 8318B054: 39660024  addi r11, r6, 0x24
	ctx.r[11].s64 = ctx.r[6].s64 + 36;
	// 8318B058: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8318B05C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B060: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B064: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B068: 4200FFF8  bdnz 0x8318b060
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B060; continue 'dispatch;
	}
	// 8318B06C: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8318B070: 409A0020  bne cr6, 0x8318b090
	if !ctx.cr[6].eq {
		sub_8318B090(ctx, base);
		return;
	}
	// 8318B074: 39660044  addi r11, r6, 0x44
	ctx.r[11].s64 = ctx.r[6].s64 + 68;
	// 8318B078: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 8318B07C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B080: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B084: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B088: 4200FFF8  bdnz 0x8318b080
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B080; continue 'dispatch;
	}
	// 8318B08C: 4800002C  b 0x8318b0b8
	sub_8318B090(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B090 size=96
    let mut pc: u32 = 0x8318B090;
    'dispatch: loop {
        match pc {
            0x8318B090 => {
    //   block [0x8318B090..0x8318B0F0)
	// 8318B090: 7D0A2850  subf r8, r10, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 8318B094: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	// 8318B098: 39260044  addi r9, r6, 0x44
	ctx.r[9].s64 = ctx.r[6].s64 + 68;
	// 8318B09C: 7D085B96  divwu r8, r8, r11
	ctx.r[8].u32 = ctx.r[8].u32 / ctx.r[11].u32;
	// 8318B0A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318B0A4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B0A8: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8318B0AC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8318B0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318B0B4: 409AFFEC  bne cr6, 0x8318b0a0
	if !ctx.cr[6].eq {
	pc = 0x8318B0A0; continue 'dispatch;
	}
	// 8318B0B8: 39660380  addi r11, r6, 0x380
	ctx.r[11].s64 = ctx.r[6].s64 + 896;
	// 8318B0BC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8318B0C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318B0C4: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8318B0C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B0CC: 4200FFF8  bdnz 0x8318b0c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B0C4; continue 'dispatch;
	}
	// 8318B0D0: 396603C0  addi r11, r6, 0x3c0
	ctx.r[11].s64 = ctx.r[6].s64 + 960;
	// 8318B0D4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8318B0D8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8318B0DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318B0E0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318B0E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B0E8: 4200FFF8  bdnz 0x8318b0e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B0E0; continue 'dispatch;
	}
	// 8318B0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8318B0F0 size=472
    let mut pc: u32 = 0x8318B0F0;
    'dispatch: loop {
        match pc {
            0x8318B0F0 => {
    //   block [0x8318B0F0..0x8318B2C8)
	// 8318B0F0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8318B0F4: 39060400  addi r8, r6, 0x400
	ctx.r[8].s64 = ctx.r[6].s64 + 1024;
	// 8318B0F8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8318B0FC: 3BE80400  addi r31, r8, 0x400
	ctx.r[31].s64 = ctx.r[8].s64 + 1024;
	// 8318B100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8318B104: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8318B108: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8318B10C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B110: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 8318B114: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318B118: 4200FFF8  bdnz 0x8318b110
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B110; continue 'dispatch;
	}
	// 8318B11C: 3C60821A  lis r3, -0x7de6
	ctx.r[3].s64 = -2112225280;
	// 8318B120: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 8318B124: 20FFFFF0  subfic r7, r31, -0x10
	ctx.xer.ca = ctx.r[31].u32 <= -16 as u32;
	ctx.r[7].s64 = (-16 as i64) - ctx.r[31].s64;
	// 8318B128: 392000DC  li r9, 0xdc
	ctx.r[9].s64 = 220;
	// 8318B12C: C003B7B0  lfs f0, -0x4850(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18512 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8318B130: 7C875A14  add r4, r7, r11
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8318B134: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8318B138: 7C8407B4  extsw r4, r4
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 8318B13C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8318B140: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 8318B144: C9A1FFE0  lfd f13, -0x20(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8318B148: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8318B14C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8318B150: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8318B154: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 8318B158: D9A1FFE8  stfd f13, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[13].u64 ) };
	// 8318B15C: 8881FFEF  lbz r4, -0x11(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-17 as u32) ) } as u64;
	// 8318B160: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 8318B164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318B168: 409AFFC8  bne cr6, 0x8318b130
	if !ctx.cr[6].eq {
	pc = 0x8318B130; continue 'dispatch;
	}
	// 8318B16C: 397F00EC  addi r11, r31, 0xec
	ctx.r[11].s64 = ctx.r[31].s64 + 236;
	// 8318B170: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 8318B174: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8318B178: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B17C: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 8318B180: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8318B184: 4200FFF8  bdnz 0x8318b17c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B17C; continue 'dispatch;
	}
	// 8318B188: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8318B18C: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8318B190: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318B194: 6124FFFF  ori r4, r9, 0xffff
	ctx.r[4].u64 = ctx.r[9].u64 | 65535;
	// 8318B198: 409A0008  bne cr6, 0x8318b1a0
	if !ctx.cr[6].eq {
	pc = 0x8318B1A0; continue 'dispatch;
	}
	// 8318B19C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8318B1A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8318B1A4: 409A0008  bne cr6, 0x8318b1ac
	if !ctx.cr[6].eq {
	pc = 0x8318B1AC; continue 'dispatch;
	}
	// 8318B1A8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8318B1AC: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8318B1B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8318B1B4: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8318B1B8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B1BC: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318B1C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B1C4: 4200FFF8  bdnz 0x8318b1bc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B1BC; continue 'dispatch;
	}
	// 8318B1C8: 39680024  addi r11, r8, 0x24
	ctx.r[11].s64 = ctx.r[8].s64 + 36;
	// 8318B1CC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8318B1D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B1D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B1D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B1DC: 4200FFF8  bdnz 0x8318b1d4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B1D4; continue 'dispatch;
	}
	// 8318B1E0: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8318B1E4: 409A0020  bne cr6, 0x8318b204
	if !ctx.cr[6].eq {
	pc = 0x8318B204; continue 'dispatch;
	}
	// 8318B1E8: 39680044  addi r11, r8, 0x44
	ctx.r[11].s64 = ctx.r[8].s64 + 68;
	// 8318B1EC: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 8318B1F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8318B1F4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B1F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B1FC: 4200FFF8  bdnz 0x8318b1f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B1F4; continue 'dispatch;
	}
	// 8318B200: 4800002C  b 0x8318b22c
	pc = 0x8318B22C; continue 'dispatch;
	// 8318B204: 7CEA2850  subf r7, r10, r5
	ctx.r[7].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 8318B208: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	// 8318B20C: 39280044  addi r9, r8, 0x44
	ctx.r[9].s64 = ctx.r[8].s64 + 68;
	// 8318B210: 7CE75B96  divwu r7, r7, r11
	ctx.r[7].u32 = ctx.r[7].u32 / ctx.r[11].u32;
	// 8318B214: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8318B218: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B21C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 8318B220: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8318B224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318B228: 409AFFEC  bne cr6, 0x8318b214
	if !ctx.cr[6].eq {
	pc = 0x8318B214; continue 'dispatch;
	}
	// 8318B22C: 39680380  addi r11, r8, 0x380
	ctx.r[11].s64 = ctx.r[8].s64 + 896;
	// 8318B230: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8318B234: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318B238: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8318B23C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B240: 4200FFF8  bdnz 0x8318b238
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B238; continue 'dispatch;
	}
	// 8318B244: 396803C0  addi r11, r8, 0x3c0
	ctx.r[11].s64 = ctx.r[8].s64 + 960;
	// 8318B248: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8318B24C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8318B250: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8318B254: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318B258: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B25C: 4200FFF8  bdnz 0x8318b254
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8318B254; continue 'dispatch;
	}
	// 8318B260: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8318B264: 39460008  addi r10, r6, 8
	ctx.r[10].s64 = ctx.r[6].s64 + 8;
	// 8318B268: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 8318B26C: 88EBFFFF  lbz r7, -1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8318B270: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8318B274: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 8318B278: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8318B27C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318B280: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 8318B284: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8318B288: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 8318B28C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318B290: 90EAFFFC  stw r7, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 8318B294: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8318B298: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 8318B29C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318B2A0: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8318B2A4: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8318B2A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8318B2AC: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 8318B2B0: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8318B2B4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8318B2B8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8318B2BC: 409AFFB0  bne cr6, 0x8318b26c
	if !ctx.cr[6].eq {
	pc = 0x8318B26C; continue 'dispatch;
	}
	// 8318B2C0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8318B2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B2C8 size=16
    let mut pc: u32 = 0x8318B2C8;
    'dispatch: loop {
        match pc {
            0x8318B2C8 => {
    //   block [0x8318B2C8..0x8318B2D8)
	// 8318B2C8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318B2CC: 396B9960  addi r11, r11, -0x66a0
	ctx.r[11].s64 = ctx.r[11].s64 + -26272;
	// 8318B2D0: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8318B2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B2D8 size=16
    let mut pc: u32 = 0x8318B2D8;
    'dispatch: loop {
        match pc {
            0x8318B2D8 => {
    //   block [0x8318B2D8..0x8318B2E8)
	// 8318B2D8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318B2DC: 396B9960  addi r11, r11, -0x66a0
	ctx.r[11].s64 = ctx.r[11].s64 + -26272;
	// 8318B2E0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8318B2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B2E8 size=76
    let mut pc: u32 = 0x8318B2E8;
    'dispatch: loop {
        match pc {
            0x8318B2E8 => {
    //   block [0x8318B2E8..0x8318B334)
	// 8318B2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B2F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8318B2F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B2F8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318B2FC: 38A0026C  li r5, 0x26c
	ctx.r[5].s64 = 620;
	// 8318B300: 3BEB9960  addi r31, r11, -0x66a0
	ctx.r[31].s64 = ctx.r[11].s64 + -26272;
	// 8318B304: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8318B308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B30C: 4801CED5  bl 0x831a81e0
	ctx.lr = 0x8318B310;
	sub_831A81E0(ctx, base);
	// 8318B310: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8318B314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8318B318: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318B31C: 4BFFFFAD  bl 0x8318b2c8
	ctx.lr = 0x8318B320;
	sub_8318B2C8(ctx, base);
	// 8318B320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B32C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8318B330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B338 size=84
    let mut pc: u32 = 0x8318B338;
    'dispatch: loop {
        match pc {
            0x8318B338 => {
    //   block [0x8318B338..0x8318B38C)
	// 8318B338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8318B340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B344: 4BFFFAF5  bl 0x8318ae38
	ctx.lr = 0x8318B348;
	sub_8318AE38(ctx, base);
	// 8318B348: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318B34C: 409A0014  bne cr6, 0x8318b360
	if !ctx.cr[6].eq {
	pc = 0x8318B360; continue 'dispatch;
	}
	// 8318B350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B35C: 4E800020  blr
	return;
	// 8318B360: 4BFFFB19  bl 0x8318ae78
	ctx.lr = 0x8318B364;
	sub_8318AE78(ctx, base);
	// 8318B364: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8318B368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8318B36C: 814B9960  lwz r10, -0x66a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26272 as u32) ) } as u64;
	// 8318B370: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8318B374: 914B9960  stw r10, -0x66a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26272 as u32), ctx.r[10].u32 ) };
	// 8318B378: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8318B37C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8318B380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8318B384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8318B388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8318B390 size=20
    let mut pc: u32 = 0x8318B390;
    'dispatch: loop {
        match pc {
            0x8318B390 => {
    //   block [0x8318B390..0x8318B3A4)
	// 8318B390: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8318B394: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8318B398: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 8318B39C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8318B3A0: 4BFFFB28  b 0x8318aec8
	sub_8318AEC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B3A8 size=236
    let mut pc: u32 = 0x8318B3A8;
    'dispatch: loop {
        match pc {
            0x8318B3A8 => {
    //   block [0x8318B3A8..0x8318B494)
	// 8318B3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B3AC: 4801CDB9  bl 0x831a8164
	ctx.lr = 0x8318B3B0;
	sub_831A8130(ctx, base);
	// 8318B3B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B3B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318B3B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8318B3BC: 3BCBA7E0  addi r30, r11, -0x5820
	ctx.r[30].s64 = ctx.r[11].s64 + -22560;
	// 8318B3C0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318B3C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318B3C8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8318B3CC: 388BBA44  addi r4, r11, -0x45bc
	ctx.r[4].s64 = ctx.r[11].s64 + -17852;
	// 8318B3D0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8318B3D4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318B3D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318B3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318B3E0: 4BFFFBA9  bl 0x8318af88
	ctx.lr = 0x8318B3E4;
	sub_8318AF88(ctx, base);
	// 8318B3E4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318B3E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8318B3EC: 409A0020  bne cr6, 0x8318b40c
	if !ctx.cr[6].eq {
	pc = 0x8318B40C; continue 'dispatch;
	}
	// 8318B3F0: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8318B3F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8318B3F8: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8318B3FC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B400: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B404: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318B408: 4801CDAC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318B40C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318B410: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8318B414: 388BBA40  addi r4, r11, -0x45c0
	ctx.r[4].s64 = ctx.r[11].s64 + -17856;
	// 8318B418: 4801F399  bl 0x831aa7b0
	ctx.lr = 0x8318B41C;
	sub_831AA7B0(ctx, base);
	// 8318B41C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8318B420: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8318B424: 388BBA38  addi r4, r11, -0x45c8
	ctx.r[4].s64 = ctx.r[11].s64 + -17864;
	// 8318B428: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8318B42C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8318B430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B434: 4BFFFB55  bl 0x8318af88
	ctx.lr = 0x8318B438;
	sub_8318AF88(ctx, base);
	// 8318B438: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8318B43C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8318B440: 409A0018  bne cr6, 0x8318b458
	if !ctx.cr[6].eq {
	pc = 0x8318B458; continue 'dispatch;
	}
	// 8318B444: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8318B448: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8318B44C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B450: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318B454: 4801CD60  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8318B458: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8318B45C: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 8318B460: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8318B464: 7D4AD9D6  mullw r10, r10, r27
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[27].s32 as i64);
	// 8318B468: 3889BA2C  addi r4, r9, -0x45d4
	ctx.r[4].s64 = ctx.r[9].s64 + -17876;
	// 8318B46C: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8318B470: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8318B474: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8318B478: 4801F339  bl 0x831aa7b0
	ctx.lr = 0x8318B47C;
	sub_831AA7B0(ctx, base);
	// 8318B47C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8318B480: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8318B484: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8318B488: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8318B48C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8318B490: 4801CD24  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8318B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8318B498 size=80
    let mut pc: u32 = 0x8318B498;
    'dispatch: loop {
        match pc {
            0x8318B498 => {
    //   block [0x8318B498..0x8318B4E8)
	// 8318B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8318B49C: 4801CCCD  bl 0x831a8168
	ctx.lr = 0x8318B4A0;
	sub_831A8130(ctx, base);
	// 8318B4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8318B4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8318B4A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8318B4AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8318B4B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8318B4B4: 4BFFF3C5  bl 0x8318a878
	ctx.lr = 0x8318B4B8;
	sub_8318A878(ctx, base);
	// 8318B4B8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8318B4BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8318B4C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8318B4C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8318B4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8318B4CC: 409A0010  bne cr6, 0x8318b4dc
	if !ctx.cr[6].eq {
	pc = 0x8318B4DC; continue 'dispatch;
	}
	// 8318B4D0: 4BFFFC21  bl 0x8318b0f0
	ctx.lr = 0x8318B4D4;
	sub_8318B0F0(ctx, base);
	// 8318B4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318B4D8: 4801CCE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8318B4DC: 4BFFFB35  bl 0x8318b010
	ctx.lr = 0x8318B4E0;
	sub_8318B010(ctx, base);
	// 8318B4E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8318B4E4: 4801CCD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


