pub fn sub_832B4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F00 size=16
    let mut pc: u32 = 0x832B4F00;
    'dispatch: loop {
        match pc {
            0x832B4F00 => {
    //   block [0x832B4F00..0x832B4F10)
	// 832B4F00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F04: 806B4DD0  lwz r3, 0x4dd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19920 as u32) ) } as u64;
	// 832B4F08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4F0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F10 size=8
    let mut pc: u32 = 0x832B4F10;
    'dispatch: loop {
        match pc {
            0x832B4F10 => {
    //   block [0x832B4F10..0x832B4F18)
	// 832B4F10: 4AF47410  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B4F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4F18 size=84
    let mut pc: u32 = 0x832B4F18;
    'dispatch: loop {
        match pc {
            0x832B4F18 => {
    //   block [0x832B4F18..0x832B4F6C)
	// 832B4F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4F28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F2C: 3BEB4DE4  addi r31, r11, 0x4de4
	ctx.r[31].s64 = ctx.r[11].s64 + 19940;
	// 832B4F30: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4F34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4F38: 419A0008  beq cr6, 0x832b4f40
	if ctx.cr[6].eq {
	pc = 0x832B4F40; continue 'dispatch;
	}
	// 832B4F3C: 4AF66DFD  bl 0x8221bd38
	ctx.lr = 0x832B4F40;
	sub_8221BD38(ctx, base);
	// 832B4F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4F44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4F48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B4F4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4F50: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B4F54: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B4F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F70 size=12
    let mut pc: u32 = 0x832B4F70;
    'dispatch: loop {
        match pc {
            0x832B4F70 => {
    //   block [0x832B4F70..0x832B4F7C)
	// 832B4F70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F74: 386B4DF4  addi r3, r11, 0x4df4
	ctx.r[3].s64 = ctx.r[11].s64 + 19956;
	// 832B4F78: 4AF5FE60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F80 size=12
    let mut pc: u32 = 0x832B4F80;
    'dispatch: loop {
        match pc {
            0x832B4F80 => {
    //   block [0x832B4F80..0x832B4F8C)
	// 832B4F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F84: 386B4DF8  addi r3, r11, 0x4df8
	ctx.r[3].s64 = ctx.r[11].s64 + 19960;
	// 832B4F88: 4AF5FE50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F90 size=12
    let mut pc: u32 = 0x832B4F90;
    'dispatch: loop {
        match pc {
            0x832B4F90 => {
    //   block [0x832B4F90..0x832B4F9C)
	// 832B4F90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F94: 386B4DFC  addi r3, r11, 0x4dfc
	ctx.r[3].s64 = ctx.r[11].s64 + 19964;
	// 832B4F98: 4AF5FE40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FA0 size=12
    let mut pc: u32 = 0x832B4FA0;
    'dispatch: loop {
        match pc {
            0x832B4FA0 => {
    //   block [0x832B4FA0..0x832B4FAC)
	// 832B4FA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FA4: 386B4E00  addi r3, r11, 0x4e00
	ctx.r[3].s64 = ctx.r[11].s64 + 19968;
	// 832B4FA8: 4AF5FE30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FB0 size=12
    let mut pc: u32 = 0x832B4FB0;
    'dispatch: loop {
        match pc {
            0x832B4FB0 => {
    //   block [0x832B4FB0..0x832B4FBC)
	// 832B4FB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FB4: 386B4E04  addi r3, r11, 0x4e04
	ctx.r[3].s64 = ctx.r[11].s64 + 19972;
	// 832B4FB8: 4AF5FE20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FC0 size=12
    let mut pc: u32 = 0x832B4FC0;
    'dispatch: loop {
        match pc {
            0x832B4FC0 => {
    //   block [0x832B4FC0..0x832B4FCC)
	// 832B4FC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FC4: 386B4E08  addi r3, r11, 0x4e08
	ctx.r[3].s64 = ctx.r[11].s64 + 19976;
	// 832B4FC8: 4AF5FE10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4FD0 size=80
    let mut pc: u32 = 0x832B4FD0;
    'dispatch: loop {
        match pc {
            0x832B4FD0 => {
    //   block [0x832B4FD0..0x832B5020)
	// 832B4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4FE4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FE8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B4FEC: 396B4E0C  addi r11, r11, 0x4e0c
	ctx.r[11].s64 = ctx.r[11].s64 + 19980;
	// 832B4FF0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B4FF4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B4FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B4FFC: 4AF02B1D  bl 0x821b7b18
	ctx.lr = 0x832B5000;
	sub_821B7B18(ctx, base);
	// 832B5000: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5004: 4080FFF0  bge 0x832b4ff4
	if !ctx.cr[0].lt {
	pc = 0x832B4FF4; continue 'dispatch;
	}
	// 832B5008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B500C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5014: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B501C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5020 size=80
    let mut pc: u32 = 0x832B5020;
    'dispatch: loop {
        match pc {
            0x832B5020 => {
    //   block [0x832B5020..0x832B5070)
	// 832B5020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B502C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5034: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5038: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B503C: 396B4E1C  addi r11, r11, 0x4e1c
	ctx.r[11].s64 = ctx.r[11].s64 + 19996;
	// 832B5040: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5044: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B504C: 4AF02ACD  bl 0x821b7b18
	ctx.lr = 0x832B5050;
	sub_821B7B18(ctx, base);
	// 832B5050: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5054: 4080FFF0  bge 0x832b5044
	if !ctx.cr[0].lt {
	pc = 0x832B5044; continue 'dispatch;
	}
	// 832B5058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B505C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B506C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5070 size=80
    let mut pc: u32 = 0x832B5070;
    'dispatch: loop {
        match pc {
            0x832B5070 => {
    //   block [0x832B5070..0x832B50C0)
	// 832B5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B507C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5084: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5088: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B508C: 396B4E2C  addi r11, r11, 0x4e2c
	ctx.r[11].s64 = ctx.r[11].s64 + 20012;
	// 832B5090: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5094: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B509C: 4AF02A7D  bl 0x821b7b18
	ctx.lr = 0x832B50A0;
	sub_821B7B18(ctx, base);
	// 832B50A0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50A4: 4080FFF0  bge 0x832b5094
	if !ctx.cr[0].lt {
	pc = 0x832B5094; continue 'dispatch;
	}
	// 832B50A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B50B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B50B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B50B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B50BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B50C0 size=80
    let mut pc: u32 = 0x832B50C0;
    'dispatch: loop {
        match pc {
            0x832B50C0 => {
    //   block [0x832B50C0..0x832B5110)
	// 832B50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B50C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B50C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B50CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B50D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B50D4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B50D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B50DC: 396B4E3C  addi r11, r11, 0x4e3c
	ctx.r[11].s64 = ctx.r[11].s64 + 20028;
	// 832B50E0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B50E4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B50E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B50EC: 4AF02A2D  bl 0x821b7b18
	ctx.lr = 0x832B50F0;
	sub_821B7B18(ctx, base);
	// 832B50F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50F4: 4080FFF0  bge 0x832b50e4
	if !ctx.cr[0].lt {
	pc = 0x832B50E4; continue 'dispatch;
	}
	// 832B50F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5110 size=80
    let mut pc: u32 = 0x832B5110;
    'dispatch: loop {
        match pc {
            0x832B5110 => {
    //   block [0x832B5110..0x832B5160)
	// 832B5110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B511C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5124: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5128: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B512C: 396B4E4C  addi r11, r11, 0x4e4c
	ctx.r[11].s64 = ctx.r[11].s64 + 20044;
	// 832B5130: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5134: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B513C: 4AF029DD  bl 0x821b7b18
	ctx.lr = 0x832B5140;
	sub_821B7B18(ctx, base);
	// 832B5140: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5144: 4080FFF0  bge 0x832b5134
	if !ctx.cr[0].lt {
	pc = 0x832B5134; continue 'dispatch;
	}
	// 832B5148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B514C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B515C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5160 size=80
    let mut pc: u32 = 0x832B5160;
    'dispatch: loop {
        match pc {
            0x832B5160 => {
    //   block [0x832B5160..0x832B51B0)
	// 832B5160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B516C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5174: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5178: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B517C: 396B4E5C  addi r11, r11, 0x4e5c
	ctx.r[11].s64 = ctx.r[11].s64 + 20060;
	// 832B5180: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5184: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B518C: 4AF0298D  bl 0x821b7b18
	ctx.lr = 0x832B5190;
	sub_821B7B18(ctx, base);
	// 832B5190: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5194: 4080FFF0  bge 0x832b5184
	if !ctx.cr[0].lt {
	pc = 0x832B5184; continue 'dispatch;
	}
	// 832B5198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B519C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B51A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B51A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B51AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B51B0 size=92
    let mut pc: u32 = 0x832B51B0;
    'dispatch: loop {
        match pc {
            0x832B51B0 => {
    //   block [0x832B51B0..0x832B520C)
	// 832B51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B51B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B51BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B51C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B51C4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B51C8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51CC: 3BDFAC18  addi r30, r31, -0x53e8
	ctx.r[30].s64 = ctx.r[31].s64 + -21480;
	// 832B51D0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B51D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51D8: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51DC: 4AF46E2D  bl 0x821fc008
	ctx.lr = 0x832B51E0;
	sub_821FC008(ctx, base);
	// 832B51E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51E8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B51EC: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51F0: 4AF46E19  bl 0x821fc008
	ctx.lr = 0x832B51F4;
	sub_821FC008(ctx, base);
	// 832B51F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5210 size=12
    let mut pc: u32 = 0x832B5210;
    'dispatch: loop {
        match pc {
            0x832B5210 => {
    //   block [0x832B5210..0x832B521C)
	// 832B5210: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5214: 386B4E6C  addi r3, r11, 0x4e6c
	ctx.r[3].s64 = ctx.r[11].s64 + 20076;
	// 832B5218: 4B7D3CA8  b 0x82a88ec0
	sub_82A88EC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5220 size=12
    let mut pc: u32 = 0x832B5220;
    'dispatch: loop {
        match pc {
            0x832B5220 => {
    //   block [0x832B5220..0x832B522C)
	// 832B5220: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5224: 386B3528  addi r3, r11, 0x3528
	ctx.r[3].s64 = ctx.r[11].s64 + 13608;
	// 832B5228: 4AF5FBB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5230 size=12
    let mut pc: u32 = 0x832B5230;
    'dispatch: loop {
        match pc {
            0x832B5230 => {
    //   block [0x832B5230..0x832B523C)
	// 832B5230: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5234: 386B4E78  addi r3, r11, 0x4e78
	ctx.r[3].s64 = ctx.r[11].s64 + 20088;
	// 832B5238: 4AF5FBA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5240 size=12
    let mut pc: u32 = 0x832B5240;
    'dispatch: loop {
        match pc {
            0x832B5240 => {
    //   block [0x832B5240..0x832B524C)
	// 832B5240: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5244: 386B4E7C  addi r3, r11, 0x4e7c
	ctx.r[3].s64 = ctx.r[11].s64 + 20092;
	// 832B5248: 4AF5FB90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5250 size=12
    let mut pc: u32 = 0x832B5250;
    'dispatch: loop {
        match pc {
            0x832B5250 => {
    //   block [0x832B5250..0x832B525C)
	// 832B5250: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5254: 386B4E80  addi r3, r11, 0x4e80
	ctx.r[3].s64 = ctx.r[11].s64 + 20096;
	// 832B5258: 4B7D4D80  b 0x82a89fd8
	sub_82A89FD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5260 size=12
    let mut pc: u32 = 0x832B5260;
    'dispatch: loop {
        match pc {
            0x832B5260 => {
    //   block [0x832B5260..0x832B526C)
	// 832B5260: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5264: 386B4E8C  addi r3, r11, 0x4e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 20108;
	// 832B5268: 4AF5FB70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5270 size=12
    let mut pc: u32 = 0x832B5270;
    'dispatch: loop {
        match pc {
            0x832B5270 => {
    //   block [0x832B5270..0x832B527C)
	// 832B5270: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5274: 386B4E90  addi r3, r11, 0x4e90
	ctx.r[3].s64 = ctx.r[11].s64 + 20112;
	// 832B5278: 4AF5FB60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5280 size=84
    let mut pc: u32 = 0x832B5280;
    'dispatch: loop {
        match pc {
            0x832B5280 => {
    //   block [0x832B5280..0x832B52D4)
	// 832B5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5290: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5294: 3BEB4E94  addi r31, r11, 0x4e94
	ctx.r[31].s64 = ctx.r[11].s64 + 20116;
	// 832B5298: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B529C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B52A0: 419A0008  beq cr6, 0x832b52a8
	if ctx.cr[6].eq {
	pc = 0x832B52A8; continue 'dispatch;
	}
	// 832B52A4: 4AF66A95  bl 0x8221bd38
	ctx.lr = 0x832B52A8;
	sub_8221BD38(ctx, base);
	// 832B52A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B52AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B52B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B52B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B52B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B52BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B52C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B52C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B52C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B52CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B52D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B52D8 size=12
    let mut pc: u32 = 0x832B52D8;
    'dispatch: loop {
        match pc {
            0x832B52D8 => {
    //   block [0x832B52D8..0x832B52E4)
	// 832B52D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B52DC: 386B4EA4  addi r3, r11, 0x4ea4
	ctx.r[3].s64 = ctx.r[11].s64 + 20132;
	// 832B52E0: 4AF5FAF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B52E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B52E8 size=12
    let mut pc: u32 = 0x832B52E8;
    'dispatch: loop {
        match pc {
            0x832B52E8 => {
    //   block [0x832B52E8..0x832B52F4)
	// 832B52E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B52EC: 386B4EA8  addi r3, r11, 0x4ea8
	ctx.r[3].s64 = ctx.r[11].s64 + 20136;
	// 832B52F0: 4AF5FAE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B52F8 size=88
    let mut pc: u32 = 0x832B52F8;
    'dispatch: loop {
        match pc {
            0x832B52F8 => {
    //   block [0x832B52F8..0x832B5350)
	// 832B52F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B52FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B530C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5310: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B5314: 396B4EC0  addi r11, r11, 0x4ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 20160;
	// 832B5318: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B531C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B5320: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5328: 419A0008  beq cr6, 0x832b5330
	if ctx.cr[6].eq {
	pc = 0x832B5330; continue 'dispatch;
	}
	// 832B532C: 4AF46FF5  bl 0x821fc320
	ctx.lr = 0x832B5330;
	sub_821FC320(ctx, base);
	// 832B5330: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5334: 4080FFE8  bge 0x832b531c
	if !ctx.cr[0].lt {
	pc = 0x832B531C; continue 'dispatch;
	}
	// 832B5338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5350 size=16
    let mut pc: u32 = 0x832B5350;
    'dispatch: loop {
        match pc {
            0x832B5350 => {
    //   block [0x832B5350..0x832B5360)
	// 832B5350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5354: 806B4EAC  lwz r3, 0x4eac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20140 as u32) ) } as u64;
	// 832B5358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B535C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5360 size=8
    let mut pc: u32 = 0x832B5360;
    'dispatch: loop {
        match pc {
            0x832B5360 => {
    //   block [0x832B5360..0x832B5368)
	// 832B5360: 4AF46FC0  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B5364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5368 size=16
    let mut pc: u32 = 0x832B5368;
    'dispatch: loop {
        match pc {
            0x832B5368 => {
    //   block [0x832B5368..0x832B5378)
	// 832B5368: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B536C: 806B4F10  lwz r3, 0x4f10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20240 as u32) ) } as u64;
	// 832B5370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5374: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5378 size=8
    let mut pc: u32 = 0x832B5378;
    'dispatch: loop {
        match pc {
            0x832B5378 => {
    //   block [0x832B5378..0x832B5380)
	// 832B5378: 4AF46FA8  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B537C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5380 size=12
    let mut pc: u32 = 0x832B5380;
    'dispatch: loop {
        match pc {
            0x832B5380 => {
    //   block [0x832B5380..0x832B538C)
	// 832B5380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5384: 386B4F24  addi r3, r11, 0x4f24
	ctx.r[3].s64 = ctx.r[11].s64 + 20260;
	// 832B5388: 4AF5FA50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5390 size=12
    let mut pc: u32 = 0x832B5390;
    'dispatch: loop {
        match pc {
            0x832B5390 => {
    //   block [0x832B5390..0x832B539C)
	// 832B5390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5394: 386B4F28  addi r3, r11, 0x4f28
	ctx.r[3].s64 = ctx.r[11].s64 + 20264;
	// 832B5398: 4AF5FA40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53A0 size=88
    let mut pc: u32 = 0x832B53A0;
    'dispatch: loop {
        match pc {
            0x832B53A0 => {
    //   block [0x832B53A0..0x832B53F8)
	// 832B53A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B53A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B53AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B53B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B53B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B53B8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B53BC: 396B4F30  addi r11, r11, 0x4f30
	ctx.r[11].s64 = ctx.r[11].s64 + 20272;
	// 832B53C0: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B53C4: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B53C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B53CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B53D0: 419A0008  beq cr6, 0x832b53d8
	if ctx.cr[6].eq {
	pc = 0x832B53D8; continue 'dispatch;
	}
	// 832B53D4: 4AF46F4D  bl 0x821fc320
	ctx.lr = 0x832B53D8;
	sub_821FC320(ctx, base);
	// 832B53D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B53DC: 4080FFE8  bge 0x832b53c4
	if !ctx.cr[0].lt {
	pc = 0x832B53C4; continue 'dispatch;
	}
	// 832B53E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B53E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B53E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B53EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B53F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B53F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53F8 size=116
    let mut pc: u32 = 0x832B53F8;
    'dispatch: loop {
        match pc {
            0x832B53F8 => {
    //   block [0x832B53F8..0x832B546C)
	// 832B53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5408: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B540C: 807F4F80  lwz r3, 0x4f80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20352 as u32) ) } as u64;
	// 832B5410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5414: 419A0044  beq cr6, 0x832b5458
	if ctx.cr[6].eq {
	pc = 0x832B5458; continue 'dispatch;
	}
	// 832B5418: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B541C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B5420: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5424: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B5428: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B542C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B5430: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5434: 4082FFE8  bne 0x832b541c
	if !ctx.cr[0].eq {
	pc = 0x832B541C; continue 'dispatch;
	}
	// 832B5438: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B543C: 409A0014  bne cr6, 0x832b5450
	if !ctx.cr[6].eq {
	pc = 0x832B5450; continue 'dispatch;
	}
	// 832B5440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5444: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5448: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B544C: 4E800421  bctrl
	ctx.lr = 0x832B5450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B5450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5454: 917F4F80  stw r11, 0x4f80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20352 as u32), ctx.r[11].u32 ) };
	// 832B5458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B545C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5470 size=12
    let mut pc: u32 = 0x832B5470;
    'dispatch: loop {
        match pc {
            0x832B5470 => {
    //   block [0x832B5470..0x832B547C)
	// 832B5470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5474: 386B4F84  addi r3, r11, 0x4f84
	ctx.r[3].s64 = ctx.r[11].s64 + 20356;
	// 832B5478: 4AF5F960  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5480 size=12
    let mut pc: u32 = 0x832B5480;
    'dispatch: loop {
        match pc {
            0x832B5480 => {
    //   block [0x832B5480..0x832B548C)
	// 832B5480: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5484: 386B4F88  addi r3, r11, 0x4f88
	ctx.r[3].s64 = ctx.r[11].s64 + 20360;
	// 832B5488: 4AF5F950  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5490 size=16
    let mut pc: u32 = 0x832B5490;
    'dispatch: loop {
        match pc {
            0x832B5490 => {
    //   block [0x832B5490..0x832B54A0)
	// 832B5490: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5494: 806B4F8C  lwz r3, 0x4f8c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20364 as u32) ) } as u64;
	// 832B5498: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B549C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54A0 size=8
    let mut pc: u32 = 0x832B54A0;
    'dispatch: loop {
        match pc {
            0x832B54A0 => {
    //   block [0x832B54A0..0x832B54A8)
	// 832B54A0: 4AF46E80  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54A8 size=16
    let mut pc: u32 = 0x832B54A8;
    'dispatch: loop {
        match pc {
            0x832B54A8 => {
    //   block [0x832B54A8..0x832B54B8)
	// 832B54A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54AC: 806B4FA0  lwz r3, 0x4fa0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20384 as u32) ) } as u64;
	// 832B54B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54B8 size=8
    let mut pc: u32 = 0x832B54B8;
    'dispatch: loop {
        match pc {
            0x832B54B8 => {
    //   block [0x832B54B8..0x832B54C0)
	// 832B54B8: 4AF46E68  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B54BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54C0 size=16
    let mut pc: u32 = 0x832B54C0;
    'dispatch: loop {
        match pc {
            0x832B54C0 => {
    //   block [0x832B54C0..0x832B54D0)
	// 832B54C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54C4: 806B4FB4  lwz r3, 0x4fb4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20404 as u32) ) } as u64;
	// 832B54C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54D0 size=8
    let mut pc: u32 = 0x832B54D0;
    'dispatch: loop {
        match pc {
            0x832B54D0 => {
    //   block [0x832B54D0..0x832B54D8)
	// 832B54D0: 4AF46E50  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B54D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54D8 size=16
    let mut pc: u32 = 0x832B54D8;
    'dispatch: loop {
        match pc {
            0x832B54D8 => {
    //   block [0x832B54D8..0x832B54E8)
	// 832B54D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54DC: 806B4FC8  lwz r3, 0x4fc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20424 as u32) ) } as u64;
	// 832B54E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54E8 size=8
    let mut pc: u32 = 0x832B54E8;
    'dispatch: loop {
        match pc {
            0x832B54E8 => {
    //   block [0x832B54E8..0x832B54F0)
	// 832B54E8: 4AF46E38  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B54EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54F0 size=16
    let mut pc: u32 = 0x832B54F0;
    'dispatch: loop {
        match pc {
            0x832B54F0 => {
    //   block [0x832B54F0..0x832B5500)
	// 832B54F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54F4: 806B4FDC  lwz r3, 0x4fdc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20444 as u32) ) } as u64;
	// 832B54F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5500 size=8
    let mut pc: u32 = 0x832B5500;
    'dispatch: loop {
        match pc {
            0x832B5500 => {
    //   block [0x832B5500..0x832B5508)
	// 832B5500: 4AF46E20  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B5504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5508 size=84
    let mut pc: u32 = 0x832B5508;
    'dispatch: loop {
        match pc {
            0x832B5508 => {
    //   block [0x832B5508..0x832B555C)
	// 832B5508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5518: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B551C: 3BEB1A50  addi r31, r11, 0x1a50
	ctx.r[31].s64 = ctx.r[11].s64 + 6736;
	// 832B5520: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5528: 419A0008  beq cr6, 0x832b5530
	if ctx.cr[6].eq {
	pc = 0x832B5530; continue 'dispatch;
	}
	// 832B552C: 4AF6680D  bl 0x8221bd38
	ctx.lr = 0x832B5530;
	sub_8221BD38(ctx, base);
	// 832B5530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5538: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B553C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5540: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5544: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B554C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5560 size=12
    let mut pc: u32 = 0x832B5560;
    'dispatch: loop {
        match pc {
            0x832B5560 => {
    //   block [0x832B5560..0x832B556C)
	// 832B5560: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5564: 386B4FF0  addi r3, r11, 0x4ff0
	ctx.r[3].s64 = ctx.r[11].s64 + 20464;
	// 832B5568: 4AF5F870  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5570 size=12
    let mut pc: u32 = 0x832B5570;
    'dispatch: loop {
        match pc {
            0x832B5570 => {
    //   block [0x832B5570..0x832B557C)
	// 832B5570: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5574: 386B4FF4  addi r3, r11, 0x4ff4
	ctx.r[3].s64 = ctx.r[11].s64 + 20468;
	// 832B5578: 4AF5F860  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5580 size=84
    let mut pc: u32 = 0x832B5580;
    'dispatch: loop {
        match pc {
            0x832B5580 => {
    //   block [0x832B5580..0x832B55D4)
	// 832B5580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B558C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5590: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5594: 3BEB4FF8  addi r31, r11, 0x4ff8
	ctx.r[31].s64 = ctx.r[11].s64 + 20472;
	// 832B5598: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B559C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55A0: 419A0008  beq cr6, 0x832b55a8
	if ctx.cr[6].eq {
	pc = 0x832B55A8; continue 'dispatch;
	}
	// 832B55A4: 4AF66795  bl 0x8221bd38
	ctx.lr = 0x832B55A8;
	sub_8221BD38(ctx, base);
	// 832B55A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B55AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B55B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B55B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B55B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B55BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B55C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B55C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B55C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B55CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B55D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B55D8 size=84
    let mut pc: u32 = 0x832B55D8;
    'dispatch: loop {
        match pc {
            0x832B55D8 => {
    //   block [0x832B55D8..0x832B562C)
	// 832B55D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B55DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B55E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B55E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B55E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B55EC: 3BEB5008  addi r31, r11, 0x5008
	ctx.r[31].s64 = ctx.r[11].s64 + 20488;
	// 832B55F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B55F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55F8: 419A0008  beq cr6, 0x832b5600
	if ctx.cr[6].eq {
	pc = 0x832B5600; continue 'dispatch;
	}
	// 832B55FC: 4AF6673D  bl 0x8221bd38
	ctx.lr = 0x832B5600;
	sub_8221BD38(ctx, base);
	// 832B5600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5608: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B560C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5610: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5614: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B561C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5630 size=84
    let mut pc: u32 = 0x832B5630;
    'dispatch: loop {
        match pc {
            0x832B5630 => {
    //   block [0x832B5630..0x832B5684)
	// 832B5630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B563C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5640: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5644: 3BEB5018  addi r31, r11, 0x5018
	ctx.r[31].s64 = ctx.r[11].s64 + 20504;
	// 832B5648: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B564C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5650: 419A0008  beq cr6, 0x832b5658
	if ctx.cr[6].eq {
	pc = 0x832B5658; continue 'dispatch;
	}
	// 832B5654: 4AF666E5  bl 0x8221bd38
	ctx.lr = 0x832B5658;
	sub_8221BD38(ctx, base);
	// 832B5658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B565C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5660: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5664: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5668: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B566C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B567C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5688 size=12
    let mut pc: u32 = 0x832B5688;
    'dispatch: loop {
        match pc {
            0x832B5688 => {
    //   block [0x832B5688..0x832B5694)
	// 832B5688: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B568C: 386B5028  addi r3, r11, 0x5028
	ctx.r[3].s64 = ctx.r[11].s64 + 20520;
	// 832B5690: 4AF5F748  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5698 size=12
    let mut pc: u32 = 0x832B5698;
    'dispatch: loop {
        match pc {
            0x832B5698 => {
    //   block [0x832B5698..0x832B56A4)
	// 832B5698: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B569C: 386B502C  addi r3, r11, 0x502c
	ctx.r[3].s64 = ctx.r[11].s64 + 20524;
	// 832B56A0: 4AF5F738  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B56A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56A8 size=64
    let mut pc: u32 = 0x832B56A8;
    'dispatch: loop {
        match pc {
            0x832B56A8 => {
    //   block [0x832B56A8..0x832B56E8)
	// 832B56A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B56B8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B56BC: 807F5030  lwz r3, 0x5030(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20528 as u32) ) } as u64;
	// 832B56C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B56C4: 419A0008  beq cr6, 0x832b56cc
	if ctx.cr[6].eq {
	pc = 0x832B56CC; continue 'dispatch;
	}
	// 832B56C8: 4AF66671  bl 0x8221bd38
	ctx.lr = 0x832B56CC;
	sub_8221BD38(ctx, base);
	// 832B56CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B56D0: 917F5030  stw r11, 0x5030(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20528 as u32), ctx.r[11].u32 ) };
	// 832B56D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B56D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B56DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B56E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B56E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56E8 size=68
    let mut pc: u32 = 0x832B56E8;
    'dispatch: loop {
        match pc {
            0x832B56E8 => {
    //   block [0x832B56E8..0x832B572C)
	// 832B56E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B56F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B56FC: 3BEB5034  addi r31, r11, 0x5034
	ctx.r[31].s64 = ctx.r[11].s64 + 20532;
	// 832B5700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5704: 4B75F68D  bl 0x82a14d90
	ctx.lr = 0x832B5708;
	sub_82A14D90(ctx, base);
	// 832B5708: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B570C: 4AF6662D  bl 0x8221bd38
	ctx.lr = 0x832B5710;
	sub_8221BD38(ctx, base);
	// 832B5710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5714: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5730 size=68
    let mut pc: u32 = 0x832B5730;
    'dispatch: loop {
        match pc {
            0x832B5730 => {
    //   block [0x832B5730..0x832B5774)
	// 832B5730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B573C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5740: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5744: 3BEB5040  addi r31, r11, 0x5040
	ctx.r[31].s64 = ctx.r[11].s64 + 20544;
	// 832B5748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B574C: 4B75F645  bl 0x82a14d90
	ctx.lr = 0x832B5750;
	sub_82A14D90(ctx, base);
	// 832B5750: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5754: 4AF665E5  bl 0x8221bd38
	ctx.lr = 0x832B5758;
	sub_8221BD38(ctx, base);
	// 832B5758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B575C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B576C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5778 size=132
    let mut pc: u32 = 0x832B5778;
    'dispatch: loop {
        match pc {
            0x832B5778 => {
    //   block [0x832B5778..0x832B57FC)
	// 832B5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B578C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5790: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5794: 3BEB504C  addi r31, r11, 0x504c
	ctx.r[31].s64 = ctx.r[11].s64 + 20556;
	// 832B5798: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B579C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57A0: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B57A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57A8: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B57AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57B0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57B4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B57B8: 419A0020  beq cr6, 0x832b57d8
	if ctx.cr[6].eq {
	pc = 0x832B57D8; continue 'dispatch;
	}
	// 832B57BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B57C0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57C4: 4AF66575  bl 0x8221bd38
	ctx.lr = 0x832B57C8;
	sub_8221BD38(ctx, base);
	// 832B57C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B57D0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57D4: 409AFFE8  bne cr6, 0x832b57bc
	if !ctx.cr[6].eq {
	pc = 0x832B57BC; continue 'dispatch;
	}
	// 832B57D8: 4AF66561  bl 0x8221bd38
	ctx.lr = 0x832B57DC;
	sub_8221BD38(ctx, base);
	// 832B57DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B57E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B57E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B57E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B57EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B57F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B57F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B57F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5800 size=68
    let mut pc: u32 = 0x832B5800;
    'dispatch: loop {
        match pc {
            0x832B5800 => {
    //   block [0x832B5800..0x832B5844)
	// 832B5800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B580C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5814: 3BEB5058  addi r31, r11, 0x5058
	ctx.r[31].s64 = ctx.r[11].s64 + 20568;
	// 832B5818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B581C: 4B7E18C5  bl 0x82a970e0
	ctx.lr = 0x832B5820;
	sub_82A970E0(ctx, base);
	// 832B5820: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5824: 4AF66515  bl 0x8221bd38
	ctx.lr = 0x832B5828;
	sub_8221BD38(ctx, base);
	// 832B5828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B582C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B583C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5848 size=12
    let mut pc: u32 = 0x832B5848;
    'dispatch: loop {
        match pc {
            0x832B5848 => {
    //   block [0x832B5848..0x832B5854)
	// 832B5848: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B584C: 386B5080  addi r3, r11, 0x5080
	ctx.r[3].s64 = ctx.r[11].s64 + 20608;
	// 832B5850: 4AF5F588  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5858 size=12
    let mut pc: u32 = 0x832B5858;
    'dispatch: loop {
        match pc {
            0x832B5858 => {
    //   block [0x832B5858..0x832B5864)
	// 832B5858: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B585C: 386B5084  addi r3, r11, 0x5084
	ctx.r[3].s64 = ctx.r[11].s64 + 20612;
	// 832B5860: 4AF5F578  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5868 size=64
    let mut pc: u32 = 0x832B5868;
    'dispatch: loop {
        match pc {
            0x832B5868 => {
    //   block [0x832B5868..0x832B58A8)
	// 832B5868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B586C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5878: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B587C: 807F5120  lwz r3, 0x5120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20768 as u32) ) } as u64;
	// 832B5880: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5884: 419A0010  beq cr6, 0x832b5894
	if ctx.cr[6].eq {
	pc = 0x832B5894; continue 'dispatch;
	}
	// 832B5888: 4B8D1D41  bl 0x82b875c8
	ctx.lr = 0x832B588C;
	sub_82B875C8(ctx, base);
	// 832B588C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5890: 917F5120  stw r11, 0x5120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20768 as u32), ctx.r[11].u32 ) };
	// 832B5894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B589C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B58A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B58A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B58A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58A8 size=64
    let mut pc: u32 = 0x832B58A8;
    'dispatch: loop {
        match pc {
            0x832B58A8 => {
    //   block [0x832B58A8..0x832B58E8)
	// 832B58A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B58B8: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B58BC: 807FAD00  lwz r3, -0x5300(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-21248 as u32) ) } as u64;
	// 832B58C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B58C4: 419A0010  beq cr6, 0x832b58d4
	if ctx.cr[6].eq {
	pc = 0x832B58D4; continue 'dispatch;
	}
	// 832B58C8: 4B8D1D01  bl 0x82b875c8
	ctx.lr = 0x832B58CC;
	sub_82B875C8(ctx, base);
	// 832B58CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B58D0: 917FAD00  stw r11, -0x5300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21248 as u32), ctx.r[11].u32 ) };
	// 832B58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B58E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B58E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58E8 size=64
    let mut pc: u32 = 0x832B58E8;
    'dispatch: loop {
        match pc {
            0x832B58E8 => {
    //   block [0x832B58E8..0x832B5928)
	// 832B58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B58F8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B58FC: 807F512C  lwz r3, 0x512c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20780 as u32) ) } as u64;
	// 832B5900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5904: 419A0010  beq cr6, 0x832b5914
	if ctx.cr[6].eq {
	pc = 0x832B5914; continue 'dispatch;
	}
	// 832B5908: 4B8D1CC1  bl 0x82b875c8
	ctx.lr = 0x832B590C;
	sub_82B875C8(ctx, base);
	// 832B590C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5910: 917F512C  stw r11, 0x512c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20780 as u32), ctx.r[11].u32 ) };
	// 832B5914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5928 size=64
    let mut pc: u32 = 0x832B5928;
    'dispatch: loop {
        match pc {
            0x832B5928 => {
    //   block [0x832B5928..0x832B5968)
	// 832B5928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B592C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5938: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B593C: 807FAD0C  lwz r3, -0x52f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-21236 as u32) ) } as u64;
	// 832B5940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5944: 419A0010  beq cr6, 0x832b5954
	if ctx.cr[6].eq {
	pc = 0x832B5954; continue 'dispatch;
	}
	// 832B5948: 4B8D1C81  bl 0x82b875c8
	ctx.lr = 0x832B594C;
	sub_82B875C8(ctx, base);
	// 832B594C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5950: 917FAD0C  stw r11, -0x52f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21236 as u32), ctx.r[11].u32 ) };
	// 832B5954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5960: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5968 size=64
    let mut pc: u32 = 0x832B5968;
    'dispatch: loop {
        match pc {
            0x832B5968 => {
    //   block [0x832B5968..0x832B59A8)
	// 832B5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5978: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B597C: 807F5138  lwz r3, 0x5138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20792 as u32) ) } as u64;
	// 832B5980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5984: 419A0010  beq cr6, 0x832b5994
	if ctx.cr[6].eq {
	pc = 0x832B5994; continue 'dispatch;
	}
	// 832B5988: 4B8D1C41  bl 0x82b875c8
	ctx.lr = 0x832B598C;
	sub_82B875C8(ctx, base);
	// 832B598C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5990: 917F5138  stw r11, 0x5138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20792 as u32), ctx.r[11].u32 ) };
	// 832B5994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B59A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B59A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B59A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B59A8 size=16
    let mut pc: u32 = 0x832B59A8;
    'dispatch: loop {
        match pc {
            0x832B59A8 => {
    //   block [0x832B59A8..0x832B59B8)
	// 832B59A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B59AC: 806B5144  lwz r3, 0x5144(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20804 as u32) ) } as u64;
	// 832B59B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B59B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B59B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B59B8 size=8
    let mut pc: u32 = 0x832B59B8;
    'dispatch: loop {
        match pc {
            0x832B59B8 => {
    //   block [0x832B59B8..0x832B59C0)
	// 832B59B8: 4AF46968  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B59BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B59C0 size=72
    let mut pc: u32 = 0x832B59C0;
    'dispatch: loop {
        match pc {
            0x832B59C0 => {
    //   block [0x832B59C0..0x832B5A08)
	// 832B59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B59C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B59CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B59D0: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B59D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59D8: 387FAD18  addi r3, r31, -0x52e8
	ctx.r[3].s64 = ctx.r[31].s64 + -21224;
	// 832B59DC: 396B7EFC  addi r11, r11, 0x7efc
	ctx.r[11].s64 = ctx.r[11].s64 + 32508;
	// 832B59E0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59E4: 4B935955  bl 0x82beb338
	ctx.lr = 0x832B59E8;
	sub_82BEB338(ctx, base);
	// 832B59E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59EC: 396B2B90  addi r11, r11, 0x2b90
	ctx.r[11].s64 = ctx.r[11].s64 + 11152;
	// 832B59F0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B59F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B59FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A08 size=12
    let mut pc: u32 = 0x832B5A08;
    'dispatch: loop {
        match pc {
            0x832B5A08 => {
    //   block [0x832B5A08..0x832B5A14)
	// 832B5A08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A0C: 386B5158  addi r3, r11, 0x5158
	ctx.r[3].s64 = ctx.r[11].s64 + 20824;
	// 832B5A10: 4B7E38C8  b 0x82a992d8
	sub_82A992D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A18 size=12
    let mut pc: u32 = 0x832B5A18;
    'dispatch: loop {
        match pc {
            0x832B5A18 => {
    //   block [0x832B5A18..0x832B5A24)
	// 832B5A18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A1C: 386B5164  addi r3, r11, 0x5164
	ctx.r[3].s64 = ctx.r[11].s64 + 20836;
	// 832B5A20: 4B7E3A68  b 0x82a99488
	sub_82A99488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A28 size=12
    let mut pc: u32 = 0x832B5A28;
    'dispatch: loop {
        match pc {
            0x832B5A28 => {
    //   block [0x832B5A28..0x832B5A34)
	// 832B5A28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A2C: 386B5170  addi r3, r11, 0x5170
	ctx.r[3].s64 = ctx.r[11].s64 + 20848;
	// 832B5A30: 4B7E3C08  b 0x82a99638
	sub_82A99638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A38 size=12
    let mut pc: u32 = 0x832B5A38;
    'dispatch: loop {
        match pc {
            0x832B5A38 => {
    //   block [0x832B5A38..0x832B5A44)
	// 832B5A38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A3C: 386B517C  addi r3, r11, 0x517c
	ctx.r[3].s64 = ctx.r[11].s64 + 20860;
	// 832B5A40: 4B7E3DA8  b 0x82a997e8
	sub_82A997E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A48 size=12
    let mut pc: u32 = 0x832B5A48;
    'dispatch: loop {
        match pc {
            0x832B5A48 => {
    //   block [0x832B5A48..0x832B5A54)
	// 832B5A48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A4C: 386B5188  addi r3, r11, 0x5188
	ctx.r[3].s64 = ctx.r[11].s64 + 20872;
	// 832B5A50: 4B7E3F48  b 0x82a99998
	sub_82A99998(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A58 size=12
    let mut pc: u32 = 0x832B5A58;
    'dispatch: loop {
        match pc {
            0x832B5A58 => {
    //   block [0x832B5A58..0x832B5A64)
	// 832B5A58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A5C: 386B5194  addi r3, r11, 0x5194
	ctx.r[3].s64 = ctx.r[11].s64 + 20884;
	// 832B5A60: 4AF5F378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A68 size=12
    let mut pc: u32 = 0x832B5A68;
    'dispatch: loop {
        match pc {
            0x832B5A68 => {
    //   block [0x832B5A68..0x832B5A74)
	// 832B5A68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A6C: 386B5198  addi r3, r11, 0x5198
	ctx.r[3].s64 = ctx.r[11].s64 + 20888;
	// 832B5A70: 4AF5F368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A78 size=4
    let mut pc: u32 = 0x832B5A78;
    'dispatch: loop {
        match pc {
            0x832B5A78 => {
    //   block [0x832B5A78..0x832B5A7C)
	// 832B5A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A80 size=12
    let mut pc: u32 = 0x832B5A80;
    'dispatch: loop {
        match pc {
            0x832B5A80 => {
    //   block [0x832B5A80..0x832B5A8C)
	// 832B5A80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A84: 386B5290  addi r3, r11, 0x5290
	ctx.r[3].s64 = ctx.r[11].s64 + 21136;
	// 832B5A88: 4AF5F350  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A90 size=12
    let mut pc: u32 = 0x832B5A90;
    'dispatch: loop {
        match pc {
            0x832B5A90 => {
    //   block [0x832B5A90..0x832B5A9C)
	// 832B5A90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A94: 386B5294  addi r3, r11, 0x5294
	ctx.r[3].s64 = ctx.r[11].s64 + 21140;
	// 832B5A98: 4AF5F340  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AA0 size=12
    let mut pc: u32 = 0x832B5AA0;
    'dispatch: loop {
        match pc {
            0x832B5AA0 => {
    //   block [0x832B5AA0..0x832B5AAC)
	// 832B5AA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AA4: 386B5298  addi r3, r11, 0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + 21144;
	// 832B5AA8: 4AFBF860  b 0x82275308
	sub_82275308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AB0 size=12
    let mut pc: u32 = 0x832B5AB0;
    'dispatch: loop {
        match pc {
            0x832B5AB0 => {
    //   block [0x832B5AB0..0x832B5ABC)
	// 832B5AB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AB4: 386B5654  addi r3, r11, 0x5654
	ctx.r[3].s64 = ctx.r[11].s64 + 22100;
	// 832B5AB8: 4AFE7FE0  b 0x8229da98
	sub_8229DA98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AC0 size=12
    let mut pc: u32 = 0x832B5AC0;
    'dispatch: loop {
        match pc {
            0x832B5AC0 => {
    //   block [0x832B5AC0..0x832B5ACC)
	// 832B5AC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AC4: 386B56A0  addi r3, r11, 0x56a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22176;
	// 832B5AC8: 4AF5F310  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AD0 size=12
    let mut pc: u32 = 0x832B5AD0;
    'dispatch: loop {
        match pc {
            0x832B5AD0 => {
    //   block [0x832B5AD0..0x832B5ADC)
	// 832B5AD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AD4: 386B56A4  addi r3, r11, 0x56a4
	ctx.r[3].s64 = ctx.r[11].s64 + 22180;
	// 832B5AD8: 4AF5F300  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AE0 size=12
    let mut pc: u32 = 0x832B5AE0;
    'dispatch: loop {
        match pc {
            0x832B5AE0 => {
    //   block [0x832B5AE0..0x832B5AEC)
	// 832B5AE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AE4: 386B56A8  addi r3, r11, 0x56a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22184;
	// 832B5AE8: 4AF5F2F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AF0 size=12
    let mut pc: u32 = 0x832B5AF0;
    'dispatch: loop {
        match pc {
            0x832B5AF0 => {
    //   block [0x832B5AF0..0x832B5AFC)
	// 832B5AF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AF4: 386B56AC  addi r3, r11, 0x56ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22188;
	// 832B5AF8: 4AF5F2E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B00 size=12
    let mut pc: u32 = 0x832B5B00;
    'dispatch: loop {
        match pc {
            0x832B5B00 => {
    //   block [0x832B5B00..0x832B5B0C)
	// 832B5B00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B04: 386B56B0  addi r3, r11, 0x56b0
	ctx.r[3].s64 = ctx.r[11].s64 + 22192;
	// 832B5B08: 4AF5F2D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B10 size=12
    let mut pc: u32 = 0x832B5B10;
    'dispatch: loop {
        match pc {
            0x832B5B10 => {
    //   block [0x832B5B10..0x832B5B1C)
	// 832B5B10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B14: 386B56B4  addi r3, r11, 0x56b4
	ctx.r[3].s64 = ctx.r[11].s64 + 22196;
	// 832B5B18: 4AF5F2C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B20 size=12
    let mut pc: u32 = 0x832B5B20;
    'dispatch: loop {
        match pc {
            0x832B5B20 => {
    //   block [0x832B5B20..0x832B5B2C)
	// 832B5B20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B24: 386B56B8  addi r3, r11, 0x56b8
	ctx.r[3].s64 = ctx.r[11].s64 + 22200;
	// 832B5B28: 4AF5F2B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B30 size=12
    let mut pc: u32 = 0x832B5B30;
    'dispatch: loop {
        match pc {
            0x832B5B30 => {
    //   block [0x832B5B30..0x832B5B3C)
	// 832B5B30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B34: 386B56BC  addi r3, r11, 0x56bc
	ctx.r[3].s64 = ctx.r[11].s64 + 22204;
	// 832B5B38: 4AF5F2A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B40 size=68
    let mut pc: u32 = 0x832B5B40;
    'dispatch: loop {
        match pc {
            0x832B5B40 => {
    //   block [0x832B5B40..0x832B5B84)
	// 832B5B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B54: 3BEB56C0  addi r31, r11, 0x56c0
	ctx.r[31].s64 = ctx.r[11].s64 + 22208;
	// 832B5B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5B5C: 4AEC67DD  bl 0x8217c338
	ctx.lr = 0x832B5B60;
	sub_8217C338(ctx, base);
	// 832B5B60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5B64: 4AF661D5  bl 0x8221bd38
	ctx.lr = 0x832B5B68;
	sub_8221BD38(ctx, base);
	// 832B5B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5B6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B88 size=68
    let mut pc: u32 = 0x832B5B88;
    'dispatch: loop {
        match pc {
            0x832B5B88 => {
    //   block [0x832B5B88..0x832B5BCC)
	// 832B5B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B9C: 3BEB56CC  addi r31, r11, 0x56cc
	ctx.r[31].s64 = ctx.r[11].s64 + 22220;
	// 832B5BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BA4: 4AEC69ED  bl 0x8217c590
	ctx.lr = 0x832B5BA8;
	sub_8217C590(ctx, base);
	// 832B5BA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BAC: 4AF6618D  bl 0x8221bd38
	ctx.lr = 0x832B5BB0;
	sub_8221BD38(ctx, base);
	// 832B5BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5BC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5BD0 size=68
    let mut pc: u32 = 0x832B5BD0;
    'dispatch: loop {
        match pc {
            0x832B5BD0 => {
    //   block [0x832B5BD0..0x832B5C14)
	// 832B5BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5BE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5BE4: 3BEB56D8  addi r31, r11, 0x56d8
	ctx.r[31].s64 = ctx.r[11].s64 + 22232;
	// 832B5BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BEC: 4AEC683D  bl 0x8217c428
	ctx.lr = 0x832B5BF0;
	sub_8217C428(ctx, base);
	// 832B5BF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BF4: 4AF66145  bl 0x8221bd38
	ctx.lr = 0x832B5BF8;
	sub_8221BD38(ctx, base);
	// 832B5BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C18 size=68
    let mut pc: u32 = 0x832B5C18;
    'dispatch: loop {
        match pc {
            0x832B5C18 => {
    //   block [0x832B5C18..0x832B5C5C)
	// 832B5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C2C: 3BEB56E4  addi r31, r11, 0x56e4
	ctx.r[31].s64 = ctx.r[11].s64 + 22244;
	// 832B5C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C34: 4AEC68E5  bl 0x8217c518
	ctx.lr = 0x832B5C38;
	sub_8217C518(ctx, base);
	// 832B5C38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C3C: 4AF660FD  bl 0x8221bd38
	ctx.lr = 0x832B5C40;
	sub_8221BD38(ctx, base);
	// 832B5C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C60 size=68
    let mut pc: u32 = 0x832B5C60;
    'dispatch: loop {
        match pc {
            0x832B5C60 => {
    //   block [0x832B5C60..0x832B5CA4)
	// 832B5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C74: 3BEB56F0  addi r31, r11, 0x56f0
	ctx.r[31].s64 = ctx.r[11].s64 + 22256;
	// 832B5C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C7C: 4AEC6735  bl 0x8217c3b0
	ctx.lr = 0x832B5C80;
	sub_8217C3B0(ctx, base);
	// 832B5C80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C84: 4AF660B5  bl 0x8221bd38
	ctx.lr = 0x832B5C88;
	sub_8221BD38(ctx, base);
	// 832B5C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C8C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CA8 size=68
    let mut pc: u32 = 0x832B5CA8;
    'dispatch: loop {
        match pc {
            0x832B5CA8 => {
    //   block [0x832B5CA8..0x832B5CEC)
	// 832B5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5CB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5CBC: 3BEB56FC  addi r31, r11, 0x56fc
	ctx.r[31].s64 = ctx.r[11].s64 + 22268;
	// 832B5CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5CC4: 4AEC67DD  bl 0x8217c4a0
	ctx.lr = 0x832B5CC8;
	sub_8217C4A0(ctx, base);
	// 832B5CC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5CCC: 4AF6606D  bl 0x8221bd38
	ctx.lr = 0x832B5CD0;
	sub_8221BD38(ctx, base);
	// 832B5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5CD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CF0 size=84
    let mut pc: u32 = 0x832B5CF0;
    'dispatch: loop {
        match pc {
            0x832B5CF0 => {
    //   block [0x832B5CF0..0x832B5D44)
	// 832B5CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5D04: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B5D08: 83FE5708  lwz r31, 0x5708(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(22280 as u32) ) } as u64;
	// 832B5D0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B5D10: 419A0014  beq cr6, 0x832b5d24
	if ctx.cr[6].eq {
	pc = 0x832B5D24; continue 'dispatch;
	}
	// 832B5D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D18: 4B8C2689  bl 0x82b783a0
	ctx.lr = 0x832B5D1C;
	sub_82B783A0(ctx, base);
	// 832B5D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D20: 4AF66019  bl 0x8221bd38
	ctx.lr = 0x832B5D24;
	sub_8221BD38(ctx, base);
	// 832B5D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5D28: 917E5708  stw r11, 0x5708(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(22280 as u32), ctx.r[11].u32 ) };
	// 832B5D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D48 size=12
    let mut pc: u32 = 0x832B5D48;
    'dispatch: loop {
        match pc {
            0x832B5D48 => {
    //   block [0x832B5D48..0x832B5D54)
	// 832B5D48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D4C: 386B570C  addi r3, r11, 0x570c
	ctx.r[3].s64 = ctx.r[11].s64 + 22284;
	// 832B5D50: 4AF7BF68  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D58 size=12
    let mut pc: u32 = 0x832B5D58;
    'dispatch: loop {
        match pc {
            0x832B5D58 => {
    //   block [0x832B5D58..0x832B5D64)
	// 832B5D58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D5C: 386B5710  addi r3, r11, 0x5710
	ctx.r[3].s64 = ctx.r[11].s64 + 22288;
	// 832B5D60: 4AF7BF58  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D68 size=12
    let mut pc: u32 = 0x832B5D68;
    'dispatch: loop {
        match pc {
            0x832B5D68 => {
    //   block [0x832B5D68..0x832B5D74)
	// 832B5D68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D6C: 386B5714  addi r3, r11, 0x5714
	ctx.r[3].s64 = ctx.r[11].s64 + 22292;
	// 832B5D70: 4AF5F068  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D78 size=12
    let mut pc: u32 = 0x832B5D78;
    'dispatch: loop {
        match pc {
            0x832B5D78 => {
    //   block [0x832B5D78..0x832B5D84)
	// 832B5D78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D7C: 386B5718  addi r3, r11, 0x5718
	ctx.r[3].s64 = ctx.r[11].s64 + 22296;
	// 832B5D80: 4AF5F058  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D88 size=12
    let mut pc: u32 = 0x832B5D88;
    'dispatch: loop {
        match pc {
            0x832B5D88 => {
    //   block [0x832B5D88..0x832B5D94)
	// 832B5D88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D8C: 386B571C  addi r3, r11, 0x571c
	ctx.r[3].s64 = ctx.r[11].s64 + 22300;
	// 832B5D90: 4B7F7F28  b 0x82aadcb8
	sub_82AADCB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D98 size=12
    let mut pc: u32 = 0x832B5D98;
    'dispatch: loop {
        match pc {
            0x832B5D98 => {
    //   block [0x832B5D98..0x832B5DA4)
	// 832B5D98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D9C: 386B572C  addi r3, r11, 0x572c
	ctx.r[3].s64 = ctx.r[11].s64 + 22316;
	// 832B5DA0: 4AF7BF18  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DA8 size=12
    let mut pc: u32 = 0x832B5DA8;
    'dispatch: loop {
        match pc {
            0x832B5DA8 => {
    //   block [0x832B5DA8..0x832B5DB4)
	// 832B5DA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DAC: 386B5730  addi r3, r11, 0x5730
	ctx.r[3].s64 = ctx.r[11].s64 + 22320;
	// 832B5DB0: 4AF7BF08  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DB8 size=4
    let mut pc: u32 = 0x832B5DB8;
    'dispatch: loop {
        match pc {
            0x832B5DB8 => {
    //   block [0x832B5DB8..0x832B5DBC)
	// 832B5DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DC0 size=12
    let mut pc: u32 = 0x832B5DC0;
    'dispatch: loop {
        match pc {
            0x832B5DC0 => {
    //   block [0x832B5DC0..0x832B5DCC)
	// 832B5DC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DC4: 386B5758  addi r3, r11, 0x5758
	ctx.r[3].s64 = ctx.r[11].s64 + 22360;
	// 832B5DC8: 4AF5F010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DD0 size=12
    let mut pc: u32 = 0x832B5DD0;
    'dispatch: loop {
        match pc {
            0x832B5DD0 => {
    //   block [0x832B5DD0..0x832B5DDC)
	// 832B5DD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DD4: 386B575C  addi r3, r11, 0x575c
	ctx.r[3].s64 = ctx.r[11].s64 + 22364;
	// 832B5DD8: 4AF5F000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5DE0 size=80
    let mut pc: u32 = 0x832B5DE0;
    'dispatch: loop {
        match pc {
            0x832B5DE0 => {
    //   block [0x832B5DE0..0x832B5E30)
	// 832B5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5DF0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B5DF4: 807F5760  lwz r3, 0x5760(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22368 as u32) ) } as u64;
	// 832B5DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5DFC: 419A0018  beq cr6, 0x832b5e14
	if ctx.cr[6].eq {
	pc = 0x832B5E14; continue 'dispatch;
	}
	// 832B5E00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B5E08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B5E10: 4E800421  bctrl
	ctx.lr = 0x832B5E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B5E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5E18: 917F5760  stw r11, 0x5760(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22368 as u32), ctx.r[11].u32 ) };
	// 832B5E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E30 size=12
    let mut pc: u32 = 0x832B5E30;
    'dispatch: loop {
        match pc {
            0x832B5E30 => {
    //   block [0x832B5E30..0x832B5E3C)
	// 832B5E30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E34: 386B5764  addi r3, r11, 0x5764
	ctx.r[3].s64 = ctx.r[11].s64 + 22372;
	// 832B5E38: 4AF5EFA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E40 size=12
    let mut pc: u32 = 0x832B5E40;
    'dispatch: loop {
        match pc {
            0x832B5E40 => {
    //   block [0x832B5E40..0x832B5E4C)
	// 832B5E40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E44: 386B5768  addi r3, r11, 0x5768
	ctx.r[3].s64 = ctx.r[11].s64 + 22376;
	// 832B5E48: 4AF5EF90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E50 size=12
    let mut pc: u32 = 0x832B5E50;
    'dispatch: loop {
        match pc {
            0x832B5E50 => {
    //   block [0x832B5E50..0x832B5E5C)
	// 832B5E50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E54: 386B576C  addi r3, r11, 0x576c
	ctx.r[3].s64 = ctx.r[11].s64 + 22380;
	// 832B5E58: 4AF5EF80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E60 size=12
    let mut pc: u32 = 0x832B5E60;
    'dispatch: loop {
        match pc {
            0x832B5E60 => {
    //   block [0x832B5E60..0x832B5E6C)
	// 832B5E60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E64: 386B5770  addi r3, r11, 0x5770
	ctx.r[3].s64 = ctx.r[11].s64 + 22384;
	// 832B5E68: 4AF5EF70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5E70 size=84
    let mut pc: u32 = 0x832B5E70;
    'dispatch: loop {
        match pc {
            0x832B5E70 => {
    //   block [0x832B5E70..0x832B5EC4)
	// 832B5E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5E80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E84: 3BEB5774  addi r31, r11, 0x5774
	ctx.r[31].s64 = ctx.r[11].s64 + 22388;
	// 832B5E88: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5E8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5E90: 419A0008  beq cr6, 0x832b5e98
	if ctx.cr[6].eq {
	pc = 0x832B5E98; continue 'dispatch;
	}
	// 832B5E94: 4AF65EA5  bl 0x8221bd38
	ctx.lr = 0x832B5E98;
	sub_8221BD38(ctx, base);
	// 832B5E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5EA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5EA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5EA8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5EAC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5EC8 size=84
    let mut pc: u32 = 0x832B5EC8;
    'dispatch: loop {
        match pc {
            0x832B5EC8 => {
    //   block [0x832B5EC8..0x832B5F1C)
	// 832B5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5ED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5ED8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5EDC: 3BEB5784  addi r31, r11, 0x5784
	ctx.r[31].s64 = ctx.r[11].s64 + 22404;
	// 832B5EE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5EE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5EE8: 419A0008  beq cr6, 0x832b5ef0
	if ctx.cr[6].eq {
	pc = 0x832B5EF0; continue 'dispatch;
	}
	// 832B5EEC: 4AF65E4D  bl 0x8221bd38
	ctx.lr = 0x832B5EF0;
	sub_8221BD38(ctx, base);
	// 832B5EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5EF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5EFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5F00: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5F04: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5F20 size=84
    let mut pc: u32 = 0x832B5F20;
    'dispatch: loop {
        match pc {
            0x832B5F20 => {
    //   block [0x832B5F20..0x832B5F74)
	// 832B5F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5F30: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5F34: 3BEB2240  addi r31, r11, 0x2240
	ctx.r[31].s64 = ctx.r[11].s64 + 8768;
	// 832B5F38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5F3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5F40: 419A0008  beq cr6, 0x832b5f48
	if ctx.cr[6].eq {
	pc = 0x832B5F48; continue 'dispatch;
	}
	// 832B5F44: 4AF65DF5  bl 0x8221bd38
	ctx.lr = 0x832B5F48;
	sub_8221BD38(ctx, base);
	// 832B5F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5F50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5F54: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5F58: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5F5C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5F78 size=12
    let mut pc: u32 = 0x832B5F78;
    'dispatch: loop {
        match pc {
            0x832B5F78 => {
    //   block [0x832B5F78..0x832B5F84)
	// 832B5F78: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5F7C: 386B19D8  addi r3, r11, 0x19d8
	ctx.r[3].s64 = ctx.r[11].s64 + 6616;
	// 832B5F80: 4B800548  b 0x82ab64c8
	sub_82AB64C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5F88 size=12
    let mut pc: u32 = 0x832B5F88;
    'dispatch: loop {
        match pc {
            0x832B5F88 => {
    //   block [0x832B5F88..0x832B5F94)
	// 832B5F88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5F8C: 386B5794  addi r3, r11, 0x5794
	ctx.r[3].s64 = ctx.r[11].s64 + 22420;
	// 832B5F90: 4AF5EE48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5F98 size=12
    let mut pc: u32 = 0x832B5F98;
    'dispatch: loop {
        match pc {
            0x832B5F98 => {
    //   block [0x832B5F98..0x832B5FA4)
	// 832B5F98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5F9C: 386B5798  addi r3, r11, 0x5798
	ctx.r[3].s64 = ctx.r[11].s64 + 22424;
	// 832B5FA0: 4AF5EE38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5FA8 size=84
    let mut pc: u32 = 0x832B5FA8;
    'dispatch: loop {
        match pc {
            0x832B5FA8 => {
    //   block [0x832B5FA8..0x832B5FFC)
	// 832B5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5FB8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5FBC: 3BEB19C4  addi r31, r11, 0x19c4
	ctx.r[31].s64 = ctx.r[11].s64 + 6596;
	// 832B5FC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5FC8: 419A0008  beq cr6, 0x832b5fd0
	if ctx.cr[6].eq {
	pc = 0x832B5FD0; continue 'dispatch;
	}
	// 832B5FCC: 4AF65D6D  bl 0x8221bd38
	ctx.lr = 0x832B5FD0;
	sub_8221BD38(ctx, base);
	// 832B5FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5FD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5FDC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5FE0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5FE4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6000 size=12
    let mut pc: u32 = 0x832B6000;
    'dispatch: loop {
        match pc {
            0x832B6000 => {
    //   block [0x832B6000..0x832B600C)
	// 832B6000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6004: 386B579C  addi r3, r11, 0x579c
	ctx.r[3].s64 = ctx.r[11].s64 + 22428;
	// 832B6008: 4AF5EDD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6010 size=12
    let mut pc: u32 = 0x832B6010;
    'dispatch: loop {
        match pc {
            0x832B6010 => {
    //   block [0x832B6010..0x832B601C)
	// 832B6010: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6014: 386B57A0  addi r3, r11, 0x57a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22432;
	// 832B6018: 4AF5EDC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6020 size=12
    let mut pc: u32 = 0x832B6020;
    'dispatch: loop {
        match pc {
            0x832B6020 => {
    //   block [0x832B6020..0x832B602C)
	// 832B6020: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6024: 386B57A4  addi r3, r11, 0x57a4
	ctx.r[3].s64 = ctx.r[11].s64 + 22436;
	// 832B6028: 4AF5EDB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6030 size=12
    let mut pc: u32 = 0x832B6030;
    'dispatch: loop {
        match pc {
            0x832B6030 => {
    //   block [0x832B6030..0x832B603C)
	// 832B6030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6034: 386B57A8  addi r3, r11, 0x57a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22440;
	// 832B6038: 4AF5EDA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6040 size=84
    let mut pc: u32 = 0x832B6040;
    'dispatch: loop {
        match pc {
            0x832B6040 => {
    //   block [0x832B6040..0x832B6094)
	// 832B6040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B604C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6054: 3BEB57AC  addi r31, r11, 0x57ac
	ctx.r[31].s64 = ctx.r[11].s64 + 22444;
	// 832B6058: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B605C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6060: 419A0008  beq cr6, 0x832b6068
	if ctx.cr[6].eq {
	pc = 0x832B6068; continue 'dispatch;
	}
	// 832B6064: 4AF65CD5  bl 0x8221bd38
	ctx.lr = 0x832B6068;
	sub_8221BD38(ctx, base);
	// 832B6068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B606C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6070: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6074: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6078: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B607C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B608C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6098 size=12
    let mut pc: u32 = 0x832B6098;
    'dispatch: loop {
        match pc {
            0x832B6098 => {
    //   block [0x832B6098..0x832B60A4)
	// 832B6098: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B609C: 386B57C0  addi r3, r11, 0x57c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22464;
	// 832B60A0: 4B806398  b 0x82abc438
	sub_82ABC438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60A8 size=12
    let mut pc: u32 = 0x832B60A8;
    'dispatch: loop {
        match pc {
            0x832B60A8 => {
    //   block [0x832B60A8..0x832B60B4)
	// 832B60A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60AC: 386B5830  addi r3, r11, 0x5830
	ctx.r[3].s64 = ctx.r[11].s64 + 22576;
	// 832B60B0: 4B8623D8  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60B8 size=12
    let mut pc: u32 = 0x832B60B8;
    'dispatch: loop {
        match pc {
            0x832B60B8 => {
    //   block [0x832B60B8..0x832B60C4)
	// 832B60B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60BC: 386B5844  addi r3, r11, 0x5844
	ctx.r[3].s64 = ctx.r[11].s64 + 22596;
	// 832B60C0: 4B8623C8  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60C8 size=12
    let mut pc: u32 = 0x832B60C8;
    'dispatch: loop {
        match pc {
            0x832B60C8 => {
    //   block [0x832B60C8..0x832B60D4)
	// 832B60C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60CC: 386B5858  addi r3, r11, 0x5858
	ctx.r[3].s64 = ctx.r[11].s64 + 22616;
	// 832B60D0: 4B8623B8  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60D8 size=12
    let mut pc: u32 = 0x832B60D8;
    'dispatch: loop {
        match pc {
            0x832B60D8 => {
    //   block [0x832B60D8..0x832B60E4)
	// 832B60D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60DC: 386B586C  addi r3, r11, 0x586c
	ctx.r[3].s64 = ctx.r[11].s64 + 22636;
	// 832B60E0: 4B8623A8  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60E8 size=12
    let mut pc: u32 = 0x832B60E8;
    'dispatch: loop {
        match pc {
            0x832B60E8 => {
    //   block [0x832B60E8..0x832B60F4)
	// 832B60E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60EC: 386B5880  addi r3, r11, 0x5880
	ctx.r[3].s64 = ctx.r[11].s64 + 22656;
	// 832B60F0: 4B862398  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B60F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B60F8 size=12
    let mut pc: u32 = 0x832B60F8;
    'dispatch: loop {
        match pc {
            0x832B60F8 => {
    //   block [0x832B60F8..0x832B6104)
	// 832B60F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B60FC: 386B5894  addi r3, r11, 0x5894
	ctx.r[3].s64 = ctx.r[11].s64 + 22676;
	// 832B6100: 4B862388  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6108 size=12
    let mut pc: u32 = 0x832B6108;
    'dispatch: loop {
        match pc {
            0x832B6108 => {
    //   block [0x832B6108..0x832B6114)
	// 832B6108: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B610C: 386B58A8  addi r3, r11, 0x58a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22696;
	// 832B6110: 4B862378  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6118 size=12
    let mut pc: u32 = 0x832B6118;
    'dispatch: loop {
        match pc {
            0x832B6118 => {
    //   block [0x832B6118..0x832B6124)
	// 832B6118: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B611C: 386B58BC  addi r3, r11, 0x58bc
	ctx.r[3].s64 = ctx.r[11].s64 + 22716;
	// 832B6120: 4B862368  b 0x82b18488
	sub_82B18488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6128 size=12
    let mut pc: u32 = 0x832B6128;
    'dispatch: loop {
        match pc {
            0x832B6128 => {
    //   block [0x832B6128..0x832B6134)
	// 832B6128: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B612C: 386B58D0  addi r3, r11, 0x58d0
	ctx.r[3].s64 = ctx.r[11].s64 + 22736;
	// 832B6130: 4AF5ECA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6138 size=12
    let mut pc: u32 = 0x832B6138;
    'dispatch: loop {
        match pc {
            0x832B6138 => {
    //   block [0x832B6138..0x832B6144)
	// 832B6138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B613C: 386B58D4  addi r3, r11, 0x58d4
	ctx.r[3].s64 = ctx.r[11].s64 + 22740;
	// 832B6140: 4AF5EC98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6148 size=12
    let mut pc: u32 = 0x832B6148;
    'dispatch: loop {
        match pc {
            0x832B6148 => {
    //   block [0x832B6148..0x832B6154)
	// 832B6148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B614C: 386B58D8  addi r3, r11, 0x58d8
	ctx.r[3].s64 = ctx.r[11].s64 + 22744;
	// 832B6150: 4AF5EC88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6158 size=12
    let mut pc: u32 = 0x832B6158;
    'dispatch: loop {
        match pc {
            0x832B6158 => {
    //   block [0x832B6158..0x832B6164)
	// 832B6158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B615C: 386B58DC  addi r3, r11, 0x58dc
	ctx.r[3].s64 = ctx.r[11].s64 + 22748;
	// 832B6160: 4AF5EC78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6168 size=12
    let mut pc: u32 = 0x832B6168;
    'dispatch: loop {
        match pc {
            0x832B6168 => {
    //   block [0x832B6168..0x832B6174)
	// 832B6168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B616C: 386B58E0  addi r3, r11, 0x58e0
	ctx.r[3].s64 = ctx.r[11].s64 + 22752;
	// 832B6170: 4AF5EC68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6178 size=12
    let mut pc: u32 = 0x832B6178;
    'dispatch: loop {
        match pc {
            0x832B6178 => {
    //   block [0x832B6178..0x832B6184)
	// 832B6178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B617C: 386B58E4  addi r3, r11, 0x58e4
	ctx.r[3].s64 = ctx.r[11].s64 + 22756;
	// 832B6180: 4AF5EC58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6188 size=12
    let mut pc: u32 = 0x832B6188;
    'dispatch: loop {
        match pc {
            0x832B6188 => {
    //   block [0x832B6188..0x832B6194)
	// 832B6188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B618C: 386B5930  addi r3, r11, 0x5930
	ctx.r[3].s64 = ctx.r[11].s64 + 22832;
	// 832B6190: 4AF5EC48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6198 size=12
    let mut pc: u32 = 0x832B6198;
    'dispatch: loop {
        match pc {
            0x832B6198 => {
    //   block [0x832B6198..0x832B61A4)
	// 832B6198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B619C: 386B5934  addi r3, r11, 0x5934
	ctx.r[3].s64 = ctx.r[11].s64 + 22836;
	// 832B61A0: 4AF5EC38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61A8 size=12
    let mut pc: u32 = 0x832B61A8;
    'dispatch: loop {
        match pc {
            0x832B61A8 => {
    //   block [0x832B61A8..0x832B61B4)
	// 832B61A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B61AC: 386B5938  addi r3, r11, 0x5938
	ctx.r[3].s64 = ctx.r[11].s64 + 22840;
	// 832B61B0: 4AF5EC28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61B8 size=12
    let mut pc: u32 = 0x832B61B8;
    'dispatch: loop {
        match pc {
            0x832B61B8 => {
    //   block [0x832B61B8..0x832B61C4)
	// 832B61B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B61BC: 386B593C  addi r3, r11, 0x593c
	ctx.r[3].s64 = ctx.r[11].s64 + 22844;
	// 832B61C0: 4AF5EC18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61C8 size=16
    let mut pc: u32 = 0x832B61C8;
    'dispatch: loop {
        match pc {
            0x832B61C8 => {
    //   block [0x832B61C8..0x832B61D8)
	// 832B61C8: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832B61CC: 806BD5B8  lwz r3, -0x2a48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10824 as u32) ) } as u64;
	// 832B61D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B61D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61D8 size=8
    let mut pc: u32 = 0x832B61D8;
    'dispatch: loop {
        match pc {
            0x832B61D8 => {
    //   block [0x832B61D8..0x832B61E0)
	// 832B61D8: 4AF46148  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B61DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61E0 size=12
    let mut pc: u32 = 0x832B61E0;
    'dispatch: loop {
        match pc {
            0x832B61E0 => {
    //   block [0x832B61E0..0x832B61EC)
	// 832B61E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B61E4: 386B5A10  addi r3, r11, 0x5a10
	ctx.r[3].s64 = ctx.r[11].s64 + 23056;
	// 832B61E8: 4AF5EBF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B61F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B61F0 size=12
    let mut pc: u32 = 0x832B61F0;
    'dispatch: loop {
        match pc {
            0x832B61F0 => {
    //   block [0x832B61F0..0x832B61FC)
	// 832B61F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B61F4: 386B5A14  addi r3, r11, 0x5a14
	ctx.r[3].s64 = ctx.r[11].s64 + 23060;
	// 832B61F8: 4AF5EBE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6200 size=12
    let mut pc: u32 = 0x832B6200;
    'dispatch: loop {
        match pc {
            0x832B6200 => {
    //   block [0x832B6200..0x832B620C)
	// 832B6200: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6204: 386B5A18  addi r3, r11, 0x5a18
	ctx.r[3].s64 = ctx.r[11].s64 + 23064;
	// 832B6208: 4AF5EBD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6210 size=12
    let mut pc: u32 = 0x832B6210;
    'dispatch: loop {
        match pc {
            0x832B6210 => {
    //   block [0x832B6210..0x832B621C)
	// 832B6210: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6214: 386B5A1C  addi r3, r11, 0x5a1c
	ctx.r[3].s64 = ctx.r[11].s64 + 23068;
	// 832B6218: 4AF5EBC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6220 size=12
    let mut pc: u32 = 0x832B6220;
    'dispatch: loop {
        match pc {
            0x832B6220 => {
    //   block [0x832B6220..0x832B622C)
	// 832B6220: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6224: 386B5A20  addi r3, r11, 0x5a20
	ctx.r[3].s64 = ctx.r[11].s64 + 23072;
	// 832B6228: 4AF5EBB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6230 size=12
    let mut pc: u32 = 0x832B6230;
    'dispatch: loop {
        match pc {
            0x832B6230 => {
    //   block [0x832B6230..0x832B623C)
	// 832B6230: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6234: 386B5A24  addi r3, r11, 0x5a24
	ctx.r[3].s64 = ctx.r[11].s64 + 23076;
	// 832B6238: 4AF5EBA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6240 size=12
    let mut pc: u32 = 0x832B6240;
    'dispatch: loop {
        match pc {
            0x832B6240 => {
    //   block [0x832B6240..0x832B624C)
	// 832B6240: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6244: 386B5A28  addi r3, r11, 0x5a28
	ctx.r[3].s64 = ctx.r[11].s64 + 23080;
	// 832B6248: 4AF5EB90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6250 size=12
    let mut pc: u32 = 0x832B6250;
    'dispatch: loop {
        match pc {
            0x832B6250 => {
    //   block [0x832B6250..0x832B625C)
	// 832B6250: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6254: 386B5A2C  addi r3, r11, 0x5a2c
	ctx.r[3].s64 = ctx.r[11].s64 + 23084;
	// 832B6258: 4AF5EB80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6260 size=12
    let mut pc: u32 = 0x832B6260;
    'dispatch: loop {
        match pc {
            0x832B6260 => {
    //   block [0x832B6260..0x832B626C)
	// 832B6260: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6264: 386B5A30  addi r3, r11, 0x5a30
	ctx.r[3].s64 = ctx.r[11].s64 + 23088;
	// 832B6268: 4AF5EB70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6270 size=12
    let mut pc: u32 = 0x832B6270;
    'dispatch: loop {
        match pc {
            0x832B6270 => {
    //   block [0x832B6270..0x832B627C)
	// 832B6270: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6274: 386B5A34  addi r3, r11, 0x5a34
	ctx.r[3].s64 = ctx.r[11].s64 + 23092;
	// 832B6278: 4AF5EB60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6280 size=12
    let mut pc: u32 = 0x832B6280;
    'dispatch: loop {
        match pc {
            0x832B6280 => {
    //   block [0x832B6280..0x832B628C)
	// 832B6280: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6284: 386B5A38  addi r3, r11, 0x5a38
	ctx.r[3].s64 = ctx.r[11].s64 + 23096;
	// 832B6288: 4AF5EB50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6290 size=12
    let mut pc: u32 = 0x832B6290;
    'dispatch: loop {
        match pc {
            0x832B6290 => {
    //   block [0x832B6290..0x832B629C)
	// 832B6290: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6294: 386B5A3C  addi r3, r11, 0x5a3c
	ctx.r[3].s64 = ctx.r[11].s64 + 23100;
	// 832B6298: 4AF5EB40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62A0 size=12
    let mut pc: u32 = 0x832B62A0;
    'dispatch: loop {
        match pc {
            0x832B62A0 => {
    //   block [0x832B62A0..0x832B62AC)
	// 832B62A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62A4: 386B5A40  addi r3, r11, 0x5a40
	ctx.r[3].s64 = ctx.r[11].s64 + 23104;
	// 832B62A8: 4AF5EB30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62B0 size=12
    let mut pc: u32 = 0x832B62B0;
    'dispatch: loop {
        match pc {
            0x832B62B0 => {
    //   block [0x832B62B0..0x832B62BC)
	// 832B62B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62B4: 386B5A44  addi r3, r11, 0x5a44
	ctx.r[3].s64 = ctx.r[11].s64 + 23108;
	// 832B62B8: 4AF5EB20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62C0 size=12
    let mut pc: u32 = 0x832B62C0;
    'dispatch: loop {
        match pc {
            0x832B62C0 => {
    //   block [0x832B62C0..0x832B62CC)
	// 832B62C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62C4: 386B5A48  addi r3, r11, 0x5a48
	ctx.r[3].s64 = ctx.r[11].s64 + 23112;
	// 832B62C8: 4AF5EB10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62D0 size=12
    let mut pc: u32 = 0x832B62D0;
    'dispatch: loop {
        match pc {
            0x832B62D0 => {
    //   block [0x832B62D0..0x832B62DC)
	// 832B62D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62D4: 386B5A4C  addi r3, r11, 0x5a4c
	ctx.r[3].s64 = ctx.r[11].s64 + 23116;
	// 832B62D8: 4AF5EB00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62E0 size=12
    let mut pc: u32 = 0x832B62E0;
    'dispatch: loop {
        match pc {
            0x832B62E0 => {
    //   block [0x832B62E0..0x832B62EC)
	// 832B62E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62E4: 386B5A50  addi r3, r11, 0x5a50
	ctx.r[3].s64 = ctx.r[11].s64 + 23120;
	// 832B62E8: 4AF5EAF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B62F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B62F0 size=12
    let mut pc: u32 = 0x832B62F0;
    'dispatch: loop {
        match pc {
            0x832B62F0 => {
    //   block [0x832B62F0..0x832B62FC)
	// 832B62F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B62F4: 386B5A54  addi r3, r11, 0x5a54
	ctx.r[3].s64 = ctx.r[11].s64 + 23124;
	// 832B62F8: 4AF5EAE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6300 size=12
    let mut pc: u32 = 0x832B6300;
    'dispatch: loop {
        match pc {
            0x832B6300 => {
    //   block [0x832B6300..0x832B630C)
	// 832B6300: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6304: 386B5AA0  addi r3, r11, 0x5aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 23200;
	// 832B6308: 4AF5EAD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6310 size=12
    let mut pc: u32 = 0x832B6310;
    'dispatch: loop {
        match pc {
            0x832B6310 => {
    //   block [0x832B6310..0x832B631C)
	// 832B6310: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6314: 386B5AA4  addi r3, r11, 0x5aa4
	ctx.r[3].s64 = ctx.r[11].s64 + 23204;
	// 832B6318: 4AF5EAC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6320 size=12
    let mut pc: u32 = 0x832B6320;
    'dispatch: loop {
        match pc {
            0x832B6320 => {
    //   block [0x832B6320..0x832B632C)
	// 832B6320: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6324: 386B5AA8  addi r3, r11, 0x5aa8
	ctx.r[3].s64 = ctx.r[11].s64 + 23208;
	// 832B6328: 4AF5EAB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6330 size=12
    let mut pc: u32 = 0x832B6330;
    'dispatch: loop {
        match pc {
            0x832B6330 => {
    //   block [0x832B6330..0x832B633C)
	// 832B6330: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6334: 386B5AAC  addi r3, r11, 0x5aac
	ctx.r[3].s64 = ctx.r[11].s64 + 23212;
	// 832B6338: 4AF5EAA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6340 size=12
    let mut pc: u32 = 0x832B6340;
    'dispatch: loop {
        match pc {
            0x832B6340 => {
    //   block [0x832B6340..0x832B634C)
	// 832B6340: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6344: 386B5AB0  addi r3, r11, 0x5ab0
	ctx.r[3].s64 = ctx.r[11].s64 + 23216;
	// 832B6348: 4AF5EA90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6350 size=12
    let mut pc: u32 = 0x832B6350;
    'dispatch: loop {
        match pc {
            0x832B6350 => {
    //   block [0x832B6350..0x832B635C)
	// 832B6350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6354: 386B5AB4  addi r3, r11, 0x5ab4
	ctx.r[3].s64 = ctx.r[11].s64 + 23220;
	// 832B6358: 4AF5EA80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6360 size=12
    let mut pc: u32 = 0x832B6360;
    'dispatch: loop {
        match pc {
            0x832B6360 => {
    //   block [0x832B6360..0x832B636C)
	// 832B6360: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6364: 386B5AB8  addi r3, r11, 0x5ab8
	ctx.r[3].s64 = ctx.r[11].s64 + 23224;
	// 832B6368: 4AF5EA70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6370 size=12
    let mut pc: u32 = 0x832B6370;
    'dispatch: loop {
        match pc {
            0x832B6370 => {
    //   block [0x832B6370..0x832B637C)
	// 832B6370: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6374: 386B5ABC  addi r3, r11, 0x5abc
	ctx.r[3].s64 = ctx.r[11].s64 + 23228;
	// 832B6378: 4AF5EA60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6380 size=12
    let mut pc: u32 = 0x832B6380;
    'dispatch: loop {
        match pc {
            0x832B6380 => {
    //   block [0x832B6380..0x832B638C)
	// 832B6380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6384: 386B5AC0  addi r3, r11, 0x5ac0
	ctx.r[3].s64 = ctx.r[11].s64 + 23232;
	// 832B6388: 4AF5EA50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6390 size=12
    let mut pc: u32 = 0x832B6390;
    'dispatch: loop {
        match pc {
            0x832B6390 => {
    //   block [0x832B6390..0x832B639C)
	// 832B6390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6394: 386B5AC4  addi r3, r11, 0x5ac4
	ctx.r[3].s64 = ctx.r[11].s64 + 23236;
	// 832B6398: 4AF5EA40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63A0 size=12
    let mut pc: u32 = 0x832B63A0;
    'dispatch: loop {
        match pc {
            0x832B63A0 => {
    //   block [0x832B63A0..0x832B63AC)
	// 832B63A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63A4: 386B5AC8  addi r3, r11, 0x5ac8
	ctx.r[3].s64 = ctx.r[11].s64 + 23240;
	// 832B63A8: 4AF5EA30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63B0 size=12
    let mut pc: u32 = 0x832B63B0;
    'dispatch: loop {
        match pc {
            0x832B63B0 => {
    //   block [0x832B63B0..0x832B63BC)
	// 832B63B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63B4: 386B5ACC  addi r3, r11, 0x5acc
	ctx.r[3].s64 = ctx.r[11].s64 + 23244;
	// 832B63B8: 4AF5EA20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63C0 size=12
    let mut pc: u32 = 0x832B63C0;
    'dispatch: loop {
        match pc {
            0x832B63C0 => {
    //   block [0x832B63C0..0x832B63CC)
	// 832B63C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63C4: 386B5AD0  addi r3, r11, 0x5ad0
	ctx.r[3].s64 = ctx.r[11].s64 + 23248;
	// 832B63C8: 4AF5EA10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63D0 size=12
    let mut pc: u32 = 0x832B63D0;
    'dispatch: loop {
        match pc {
            0x832B63D0 => {
    //   block [0x832B63D0..0x832B63DC)
	// 832B63D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63D4: 386B5AD4  addi r3, r11, 0x5ad4
	ctx.r[3].s64 = ctx.r[11].s64 + 23252;
	// 832B63D8: 4AF5EA00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63E0 size=12
    let mut pc: u32 = 0x832B63E0;
    'dispatch: loop {
        match pc {
            0x832B63E0 => {
    //   block [0x832B63E0..0x832B63EC)
	// 832B63E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63E4: 386B5AD8  addi r3, r11, 0x5ad8
	ctx.r[3].s64 = ctx.r[11].s64 + 23256;
	// 832B63E8: 4AF5E9F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B63F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B63F0 size=12
    let mut pc: u32 = 0x832B63F0;
    'dispatch: loop {
        match pc {
            0x832B63F0 => {
    //   block [0x832B63F0..0x832B63FC)
	// 832B63F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B63F4: 386B5ADC  addi r3, r11, 0x5adc
	ctx.r[3].s64 = ctx.r[11].s64 + 23260;
	// 832B63F8: 4AF5E9E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6400 size=12
    let mut pc: u32 = 0x832B6400;
    'dispatch: loop {
        match pc {
            0x832B6400 => {
    //   block [0x832B6400..0x832B640C)
	// 832B6400: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6404: 386B5AE0  addi r3, r11, 0x5ae0
	ctx.r[3].s64 = ctx.r[11].s64 + 23264;
	// 832B6408: 4AF5E9D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6410 size=12
    let mut pc: u32 = 0x832B6410;
    'dispatch: loop {
        match pc {
            0x832B6410 => {
    //   block [0x832B6410..0x832B641C)
	// 832B6410: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6414: 386B5AE4  addi r3, r11, 0x5ae4
	ctx.r[3].s64 = ctx.r[11].s64 + 23268;
	// 832B6418: 4AF5E9C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6420 size=12
    let mut pc: u32 = 0x832B6420;
    'dispatch: loop {
        match pc {
            0x832B6420 => {
    //   block [0x832B6420..0x832B642C)
	// 832B6420: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6424: 386B5D2C  addi r3, r11, 0x5d2c
	ctx.r[3].s64 = ctx.r[11].s64 + 23852;
	// 832B6428: 4AF5E9B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6430 size=12
    let mut pc: u32 = 0x832B6430;
    'dispatch: loop {
        match pc {
            0x832B6430 => {
    //   block [0x832B6430..0x832B643C)
	// 832B6430: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6434: 386B5D30  addi r3, r11, 0x5d30
	ctx.r[3].s64 = ctx.r[11].s64 + 23856;
	// 832B6438: 4AF5E9A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6440 size=12
    let mut pc: u32 = 0x832B6440;
    'dispatch: loop {
        match pc {
            0x832B6440 => {
    //   block [0x832B6440..0x832B644C)
	// 832B6440: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6444: 386B5D34  addi r3, r11, 0x5d34
	ctx.r[3].s64 = ctx.r[11].s64 + 23860;
	// 832B6448: 4AF5E990  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6450 size=12
    let mut pc: u32 = 0x832B6450;
    'dispatch: loop {
        match pc {
            0x832B6450 => {
    //   block [0x832B6450..0x832B645C)
	// 832B6450: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6454: 386B5D38  addi r3, r11, 0x5d38
	ctx.r[3].s64 = ctx.r[11].s64 + 23864;
	// 832B6458: 4AF5E980  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6460 size=12
    let mut pc: u32 = 0x832B6460;
    'dispatch: loop {
        match pc {
            0x832B6460 => {
    //   block [0x832B6460..0x832B646C)
	// 832B6460: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6464: 386B5D3C  addi r3, r11, 0x5d3c
	ctx.r[3].s64 = ctx.r[11].s64 + 23868;
	// 832B6468: 4AF5E970  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6470 size=12
    let mut pc: u32 = 0x832B6470;
    'dispatch: loop {
        match pc {
            0x832B6470 => {
    //   block [0x832B6470..0x832B647C)
	// 832B6470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6474: 386B5D40  addi r3, r11, 0x5d40
	ctx.r[3].s64 = ctx.r[11].s64 + 23872;
	// 832B6478: 4AF5E960  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6480 size=12
    let mut pc: u32 = 0x832B6480;
    'dispatch: loop {
        match pc {
            0x832B6480 => {
    //   block [0x832B6480..0x832B648C)
	// 832B6480: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6484: 386B5D44  addi r3, r11, 0x5d44
	ctx.r[3].s64 = ctx.r[11].s64 + 23876;
	// 832B6488: 4AF5E950  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6490 size=12
    let mut pc: u32 = 0x832B6490;
    'dispatch: loop {
        match pc {
            0x832B6490 => {
    //   block [0x832B6490..0x832B649C)
	// 832B6490: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6494: 386B5D48  addi r3, r11, 0x5d48
	ctx.r[3].s64 = ctx.r[11].s64 + 23880;
	// 832B6498: 4AF5E940  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B64A0 size=84
    let mut pc: u32 = 0x832B64A0;
    'dispatch: loop {
        match pc {
            0x832B64A0 => {
    //   block [0x832B64A0..0x832B64F4)
	// 832B64A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B64A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B64A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B64AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B64B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B64B4: 3BEB5D4C  addi r31, r11, 0x5d4c
	ctx.r[31].s64 = ctx.r[11].s64 + 23884;
	// 832B64B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B64BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B64C0: 419A0008  beq cr6, 0x832b64c8
	if ctx.cr[6].eq {
	pc = 0x832B64C8; continue 'dispatch;
	}
	// 832B64C4: 4AF65875  bl 0x8221bd38
	ctx.lr = 0x832B64C8;
	sub_8221BD38(ctx, base);
	// 832B64C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B64CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B64D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B64D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B64D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B64DC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B64E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B64E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B64E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B64EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B64F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B64F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B64F8 size=12
    let mut pc: u32 = 0x832B64F8;
    'dispatch: loop {
        match pc {
            0x832B64F8 => {
    //   block [0x832B64F8..0x832B6504)
	// 832B64F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B64FC: 386B5D5C  addi r3, r11, 0x5d5c
	ctx.r[3].s64 = ctx.r[11].s64 + 23900;
	// 832B6500: 4AF5E8D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6508 size=12
    let mut pc: u32 = 0x832B6508;
    'dispatch: loop {
        match pc {
            0x832B6508 => {
    //   block [0x832B6508..0x832B6514)
	// 832B6508: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B650C: 386B5D60  addi r3, r11, 0x5d60
	ctx.r[3].s64 = ctx.r[11].s64 + 23904;
	// 832B6510: 4AF5E8C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6518 size=12
    let mut pc: u32 = 0x832B6518;
    'dispatch: loop {
        match pc {
            0x832B6518 => {
    //   block [0x832B6518..0x832B6524)
	// 832B6518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B651C: 386B5D64  addi r3, r11, 0x5d64
	ctx.r[3].s64 = ctx.r[11].s64 + 23908;
	// 832B6520: 4AF5E8B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6528 size=12
    let mut pc: u32 = 0x832B6528;
    'dispatch: loop {
        match pc {
            0x832B6528 => {
    //   block [0x832B6528..0x832B6534)
	// 832B6528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B652C: 386B5D68  addi r3, r11, 0x5d68
	ctx.r[3].s64 = ctx.r[11].s64 + 23912;
	// 832B6530: 4AF5E8A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6538 size=12
    let mut pc: u32 = 0x832B6538;
    'dispatch: loop {
        match pc {
            0x832B6538 => {
    //   block [0x832B6538..0x832B6544)
	// 832B6538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B653C: 386B5D6C  addi r3, r11, 0x5d6c
	ctx.r[3].s64 = ctx.r[11].s64 + 23916;
	// 832B6540: 4AF5E898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6548 size=12
    let mut pc: u32 = 0x832B6548;
    'dispatch: loop {
        match pc {
            0x832B6548 => {
    //   block [0x832B6548..0x832B6554)
	// 832B6548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B654C: 386B5D70  addi r3, r11, 0x5d70
	ctx.r[3].s64 = ctx.r[11].s64 + 23920;
	// 832B6550: 4AF5E888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6558 size=12
    let mut pc: u32 = 0x832B6558;
    'dispatch: loop {
        match pc {
            0x832B6558 => {
    //   block [0x832B6558..0x832B6564)
	// 832B6558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B655C: 386B5D74  addi r3, r11, 0x5d74
	ctx.r[3].s64 = ctx.r[11].s64 + 23924;
	// 832B6560: 4AF5E878  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6568 size=12
    let mut pc: u32 = 0x832B6568;
    'dispatch: loop {
        match pc {
            0x832B6568 => {
    //   block [0x832B6568..0x832B6574)
	// 832B6568: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B656C: 386B5D78  addi r3, r11, 0x5d78
	ctx.r[3].s64 = ctx.r[11].s64 + 23928;
	// 832B6570: 4AF5E868  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6578 size=12
    let mut pc: u32 = 0x832B6578;
    'dispatch: loop {
        match pc {
            0x832B6578 => {
    //   block [0x832B6578..0x832B6584)
	// 832B6578: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B657C: 386B5D7C  addi r3, r11, 0x5d7c
	ctx.r[3].s64 = ctx.r[11].s64 + 23932;
	// 832B6580: 4AF5E858  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6588 size=12
    let mut pc: u32 = 0x832B6588;
    'dispatch: loop {
        match pc {
            0x832B6588 => {
    //   block [0x832B6588..0x832B6594)
	// 832B6588: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B658C: 386B5D80  addi r3, r11, 0x5d80
	ctx.r[3].s64 = ctx.r[11].s64 + 23936;
	// 832B6590: 4AF5E848  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6598 size=12
    let mut pc: u32 = 0x832B6598;
    'dispatch: loop {
        match pc {
            0x832B6598 => {
    //   block [0x832B6598..0x832B65A4)
	// 832B6598: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B659C: 386B5D84  addi r3, r11, 0x5d84
	ctx.r[3].s64 = ctx.r[11].s64 + 23940;
	// 832B65A0: 4AF5E838  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65A8 size=12
    let mut pc: u32 = 0x832B65A8;
    'dispatch: loop {
        match pc {
            0x832B65A8 => {
    //   block [0x832B65A8..0x832B65B4)
	// 832B65A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B65AC: 386B5D88  addi r3, r11, 0x5d88
	ctx.r[3].s64 = ctx.r[11].s64 + 23944;
	// 832B65B0: 4AF5E828  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65B8 size=12
    let mut pc: u32 = 0x832B65B8;
    'dispatch: loop {
        match pc {
            0x832B65B8 => {
    //   block [0x832B65B8..0x832B65C4)
	// 832B65B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B65BC: 386B5DA0  addi r3, r11, 0x5da0
	ctx.r[3].s64 = ctx.r[11].s64 + 23968;
	// 832B65C0: 4AF5E818  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65C8 size=12
    let mut pc: u32 = 0x832B65C8;
    'dispatch: loop {
        match pc {
            0x832B65C8 => {
    //   block [0x832B65C8..0x832B65D4)
	// 832B65C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B65CC: 386B5DA4  addi r3, r11, 0x5da4
	ctx.r[3].s64 = ctx.r[11].s64 + 23972;
	// 832B65D0: 4AF5E808  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65D8 size=16
    let mut pc: u32 = 0x832B65D8;
    'dispatch: loop {
        match pc {
            0x832B65D8 => {
    //   block [0x832B65D8..0x832B65E8)
	// 832B65D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B65DC: 806B5DA8  lwz r3, 0x5da8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23976 as u32) ) } as u64;
	// 832B65E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B65E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65E8 size=8
    let mut pc: u32 = 0x832B65E8;
    'dispatch: loop {
        match pc {
            0x832B65E8 => {
    //   block [0x832B65E8..0x832B65F0)
	// 832B65E8: 4AF45D38  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
	// 832B65EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B65F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B65F0 size=12
    let mut pc: u32 = 0x832B65F0;
    'dispatch: loop {
        match pc {
            0x832B65F0 => {
    //   block [0x832B65F0..0x832B65FC)
	// 832B65F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B65F4: 386B5DBC  addi r3, r11, 0x5dbc
	ctx.r[3].s64 = ctx.r[11].s64 + 23996;
	// 832B65F8: 4AF5E7E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6600 size=12
    let mut pc: u32 = 0x832B6600;
    'dispatch: loop {
        match pc {
            0x832B6600 => {
    //   block [0x832B6600..0x832B660C)
	// 832B6600: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6604: 386B5DC0  addi r3, r11, 0x5dc0
	ctx.r[3].s64 = ctx.r[11].s64 + 24000;
	// 832B6608: 4AF5E7D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6610 size=12
    let mut pc: u32 = 0x832B6610;
    'dispatch: loop {
        match pc {
            0x832B6610 => {
    //   block [0x832B6610..0x832B661C)
	// 832B6610: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6614: 386B5E34  addi r3, r11, 0x5e34
	ctx.r[3].s64 = ctx.r[11].s64 + 24116;
	// 832B6618: 4AF01500  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6620 size=4
    let mut pc: u32 = 0x832B6620;
    'dispatch: loop {
        match pc {
            0x832B6620 => {
    //   block [0x832B6620..0x832B6624)
	// 832B6620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6628 size=12
    let mut pc: u32 = 0x832B6628;
    'dispatch: loop {
        match pc {
            0x832B6628 => {
    //   block [0x832B6628..0x832B6634)
	// 832B6628: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B662C: 386B5E60  addi r3, r11, 0x5e60
	ctx.r[3].s64 = ctx.r[11].s64 + 24160;
	// 832B6630: 4B761730  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6638 size=12
    let mut pc: u32 = 0x832B6638;
    'dispatch: loop {
        match pc {
            0x832B6638 => {
    //   block [0x832B6638..0x832B6644)
	// 832B6638: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B663C: 386B5E6C  addi r3, r11, 0x5e6c
	ctx.r[3].s64 = ctx.r[11].s64 + 24172;
	// 832B6640: 4B761720  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6648 size=12
    let mut pc: u32 = 0x832B6648;
    'dispatch: loop {
        match pc {
            0x832B6648 => {
    //   block [0x832B6648..0x832B6654)
	// 832B6648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B664C: 386B5E78  addi r3, r11, 0x5e78
	ctx.r[3].s64 = ctx.r[11].s64 + 24184;
	// 832B6650: 4AF5E788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6658 size=12
    let mut pc: u32 = 0x832B6658;
    'dispatch: loop {
        match pc {
            0x832B6658 => {
    //   block [0x832B6658..0x832B6664)
	// 832B6658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B665C: 386B5E7C  addi r3, r11, 0x5e7c
	ctx.r[3].s64 = ctx.r[11].s64 + 24188;
	// 832B6660: 4AF10198  b 0x821c67f8
	sub_821C67F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6668 size=12
    let mut pc: u32 = 0x832B6668;
    'dispatch: loop {
        match pc {
            0x832B6668 => {
    //   block [0x832B6668..0x832B6674)
	// 832B6668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B666C: 386B5E80  addi r3, r11, 0x5e80
	ctx.r[3].s64 = ctx.r[11].s64 + 24192;
	// 832B6670: 4AF5E768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6678 size=12
    let mut pc: u32 = 0x832B6678;
    'dispatch: loop {
        match pc {
            0x832B6678 => {
    //   block [0x832B6678..0x832B6684)
	// 832B6678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B667C: 386B5E84  addi r3, r11, 0x5e84
	ctx.r[3].s64 = ctx.r[11].s64 + 24196;
	// 832B6680: 4B88D6E8  b 0x82b43d68
	sub_82B43D68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6688 size=4
    let mut pc: u32 = 0x832B6688;
    'dispatch: loop {
        match pc {
            0x832B6688 => {
    //   block [0x832B6688..0x832B668C)
	// 832B6688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6690 size=16
    let mut pc: u32 = 0x832B6690;
    'dispatch: loop {
        match pc {
            0x832B6690 => {
    //   block [0x832B6690..0x832B66A0)
	// 832B6690: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6694: 806B19C0  lwz r3, 0x19c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6592 as u32) ) } as u64;
	// 832B6698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B669C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66A0 size=16
    let mut pc: u32 = 0x832B66A0;
    'dispatch: loop {
        match pc {
            0x832B66A0 => {
    //   block [0x832B66A0..0x832B66B0)
	// 832B66A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B66A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B66A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B66AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66B0 size=4
    let mut pc: u32 = 0x832B66B0;
    'dispatch: loop {
        match pc {
            0x832B66B0 => {
    //   block [0x832B66B0..0x832B66B4)
	// 832B66B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B66B8 size=60
    let mut pc: u32 = 0x832B66B8;
    'dispatch: loop {
        match pc {
            0x832B66B8 => {
    //   block [0x832B66B8..0x832B66F4)
	// 832B66B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B66BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B66C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B66C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B66C8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B66CC: 3BEB3880  addi r31, r11, 0x3880
	ctx.r[31].s64 = ctx.r[11].s64 + 14464;
	// 832B66D0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832B66D4: 4B041625  bl 0x822f7cf8
	ctx.lr = 0x832B66D8;
	sub_822F7CF8(ctx, base);
	// 832B66D8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832B66DC: 4B05D0C5  bl 0x823137a0
	ctx.lr = 0x832B66E0;
	sub_823137A0(ctx, base);
	// 832B66E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B66E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B66E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B66EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B66F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66F8 size=12
    let mut pc: u32 = 0x832B66F8;
    'dispatch: loop {
        match pc {
            0x832B66F8 => {
    //   block [0x832B66F8..0x832B6704)
	// 832B66F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B66FC: 386B5E9C  addi r3, r11, 0x5e9c
	ctx.r[3].s64 = ctx.r[11].s64 + 24220;
	// 832B6700: 4B8AAFA8  b 0x82b616a8
	sub_82B616A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6708 size=12
    let mut pc: u32 = 0x832B6708;
    'dispatch: loop {
        match pc {
            0x832B6708 => {
    //   block [0x832B6708..0x832B6714)
	// 832B6708: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B670C: 386B5F20  addi r3, r11, 0x5f20
	ctx.r[3].s64 = ctx.r[11].s64 + 24352;
	// 832B6710: 4B1088F8  b 0x823bf008
	sub_823BF008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6718 size=4
    let mut pc: u32 = 0x832B6718;
    'dispatch: loop {
        match pc {
            0x832B6718 => {
    //   block [0x832B6718..0x832B671C)
	// 832B6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6720 size=12
    let mut pc: u32 = 0x832B6720;
    'dispatch: loop {
        match pc {
            0x832B6720 => {
    //   block [0x832B6720..0x832B672C)
	// 832B6720: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6724: 386B5F48  addi r3, r11, 0x5f48
	ctx.r[3].s64 = ctx.r[11].s64 + 24392;
	// 832B6728: 4B8AC8A8  b 0x82b62fd0
	sub_82B62FD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6730 size=12
    let mut pc: u32 = 0x832B6730;
    'dispatch: loop {
        match pc {
            0x832B6730 => {
    //   block [0x832B6730..0x832B673C)
	// 832B6730: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6734: 386B5F54  addi r3, r11, 0x5f54
	ctx.r[3].s64 = ctx.r[11].s64 + 24404;
	// 832B6738: 4B8ACFE0  b 0x82b63718
	sub_82B63718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6740 size=12
    let mut pc: u32 = 0x832B6740;
    'dispatch: loop {
        match pc {
            0x832B6740 => {
    //   block [0x832B6740..0x832B674C)
	// 832B6740: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6744: 386BB088  addi r3, r11, -0x4f78
	ctx.r[3].s64 = ctx.r[11].s64 + -20344;
	// 832B6748: 4B76D068  b 0x82a237b0
	sub_82A237B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6750 size=12
    let mut pc: u32 = 0x832B6750;
    'dispatch: loop {
        match pc {
            0x832B6750 => {
    //   block [0x832B6750..0x832B675C)
	// 832B6750: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6754: 386B5F60  addi r3, r11, 0x5f60
	ctx.r[3].s64 = ctx.r[11].s64 + 24416;
	// 832B6758: 4B8AD270  b 0x82b639c8
	sub_82B639C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6760 size=12
    let mut pc: u32 = 0x832B6760;
    'dispatch: loop {
        match pc {
            0x832B6760 => {
    //   block [0x832B6760..0x832B676C)
	// 832B6760: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6764: 386B5F6C  addi r3, r11, 0x5f6c
	ctx.r[3].s64 = ctx.r[11].s64 + 24428;
	// 832B6768: 4AF5E670  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6770 size=92
    let mut pc: u32 = 0x832B6770;
    'dispatch: loop {
        match pc {
            0x832B6770 => {
    //   block [0x832B6770..0x832B67CC)
	// 832B6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B677C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6784: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B6788: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B678C: 3BDFB0BC  addi r30, r31, -0x4f44
	ctx.r[30].s64 = ctx.r[31].s64 + -20292;
	// 832B6790: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B6794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6798: 917FB0BC  stw r11, -0x4f44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20292 as u32), ctx.r[11].u32 ) };
	// 832B679C: 4AF4586D  bl 0x821fc008
	ctx.lr = 0x832B67A0;
	sub_821FC008(ctx, base);
	// 832B67A0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B67A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B67A8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B67AC: 917FB0BC  stw r11, -0x4f44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20292 as u32), ctx.r[11].u32 ) };
	// 832B67B0: 4AF45859  bl 0x821fc008
	ctx.lr = 0x832B67B4;
	sub_821FC008(ctx, base);
	// 832B67B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B67B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B67BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B67C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B67C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B67C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B67D0 size=92
    let mut pc: u32 = 0x832B67D0;
    'dispatch: loop {
        match pc {
            0x832B67D0 => {
    //   block [0x832B67D0..0x832B682C)
	// 832B67D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B67D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B67D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B67DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B67E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B67E4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B67E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832B67EC: 3BDFB0C4  addi r30, r31, -0x4f3c
	ctx.r[30].s64 = ctx.r[31].s64 + -20284;
	// 832B67F0: 396B1460  addi r11, r11, 0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + 5216;
	// 832B67F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B67F8: 917FB0C4  stw r11, -0x4f3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20284 as u32), ctx.r[11].u32 ) };
	// 832B67FC: 4AF4580D  bl 0x821fc008
	ctx.lr = 0x832B6800;
	sub_821FC008(ctx, base);
	// 832B6800: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6808: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B680C: 917FB0C4  stw r11, -0x4f3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20284 as u32), ctx.r[11].u32 ) };
	// 832B6810: 4AF457F9  bl 0x821fc008
	ctx.lr = 0x832B6814;
	sub_821FC008(ctx, base);
	// 832B6814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B681C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6830 size=92
    let mut pc: u32 = 0x832B6830;
    'dispatch: loop {
        match pc {
            0x832B6830 => {
    //   block [0x832B6830..0x832B688C)
	// 832B6830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B683C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6844: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B6848: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832B684C: 3BDFB0CC  addi r30, r31, -0x4f34
	ctx.r[30].s64 = ctx.r[31].s64 + -20276;
	// 832B6850: 396BADE4  addi r11, r11, -0x521c
	ctx.r[11].s64 = ctx.r[11].s64 + -21020;
	// 832B6854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6858: 917FB0CC  stw r11, -0x4f34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20276 as u32), ctx.r[11].u32 ) };
	// 832B685C: 4AF457AD  bl 0x821fc008
	ctx.lr = 0x832B6860;
	sub_821FC008(ctx, base);
	// 832B6860: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6868: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B686C: 917FB0CC  stw r11, -0x4f34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20276 as u32), ctx.r[11].u32 ) };
	// 832B6870: 4AF45799  bl 0x821fc008
	ctx.lr = 0x832B6874;
	sub_821FC008(ctx, base);
	// 832B6874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B687C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6890 size=12
    let mut pc: u32 = 0x832B6890;
    'dispatch: loop {
        match pc {
            0x832B6890 => {
    //   block [0x832B6890..0x832B689C)
	// 832B6890: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6894: 386B5F74  addi r3, r11, 0x5f74
	ctx.r[3].s64 = ctx.r[11].s64 + 24436;
	// 832B6898: 4B8B4150  b 0x82b6a9e8
	sub_82B6A9E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68A0 size=4
    let mut pc: u32 = 0x832B68A0;
    'dispatch: loop {
        match pc {
            0x832B68A0 => {
    //   block [0x832B68A0..0x832B68A4)
	// 832B68A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68A8 size=12
    let mut pc: u32 = 0x832B68A8;
    'dispatch: loop {
        match pc {
            0x832B68A8 => {
    //   block [0x832B68A8..0x832B68B4)
	// 832B68A8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B68AC: 386BB0D4  addi r3, r11, -0x4f2c
	ctx.r[3].s64 = ctx.r[11].s64 + -20268;
	// 832B68B0: 4B934D88  b 0x82beb638
	sub_82BEB638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68B8 size=12
    let mut pc: u32 = 0x832B68B8;
    'dispatch: loop {
        match pc {
            0x832B68B8 => {
    //   block [0x832B68B8..0x832B68C4)
	// 832B68B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68BC: 386B5F98  addi r3, r11, 0x5f98
	ctx.r[3].s64 = ctx.r[11].s64 + 24472;
	// 832B68C0: 4B8B7650  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68C8 size=12
    let mut pc: u32 = 0x832B68C8;
    'dispatch: loop {
        match pc {
            0x832B68C8 => {
    //   block [0x832B68C8..0x832B68D4)
	// 832B68C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68CC: 386B5FA4  addi r3, r11, 0x5fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 24484;
	// 832B68D0: 4B8B7640  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68D8 size=12
    let mut pc: u32 = 0x832B68D8;
    'dispatch: loop {
        match pc {
            0x832B68D8 => {
    //   block [0x832B68D8..0x832B68E4)
	// 832B68D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68DC: 386B5FB0  addi r3, r11, 0x5fb0
	ctx.r[3].s64 = ctx.r[11].s64 + 24496;
	// 832B68E0: 4B8B7630  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68E8 size=12
    let mut pc: u32 = 0x832B68E8;
    'dispatch: loop {
        match pc {
            0x832B68E8 => {
    //   block [0x832B68E8..0x832B68F4)
	// 832B68E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68EC: 386B5FBC  addi r3, r11, 0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 24508;
	// 832B68F0: 4B8B7620  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B68F8 size=88
    let mut pc: u32 = 0x832B68F8;
    'dispatch: loop {
        match pc {
            0x832B68F8 => {
    //   block [0x832B68F8..0x832B6950)
	// 832B68F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B68FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B690C: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 832B6910: 3BFEB0E8  addi r31, r30, -0x4f18
	ctx.r[31].s64 = ctx.r[30].s64 + -20248;
	// 832B6914: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832B6918: 4B8AABF9  bl 0x82b61510
	ctx.lr = 0x832B691C;
	sub_82B61510(ctx, base);
	// 832B691C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832B6920: 4AF65419  bl 0x8221bd38
	ctx.lr = 0x832B6924;
	sub_8221BD38(ctx, base);
	// 832B6924: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832B6928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B692C: 394A0FD0  addi r10, r10, 0xfd0
	ctx.r[10].s64 = ctx.r[10].s64 + 4048;
	// 832B6930: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832B6934: 915EB0E8  stw r10, -0x4f18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-20248 as u32), ctx.r[10].u32 ) };
	// 832B6938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B693C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B694C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6950 size=4
    let mut pc: u32 = 0x832B6950;
    'dispatch: loop {
        match pc {
            0x832B6950 => {
    //   block [0x832B6950..0x832B6954)
	// 832B6950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6958 size=4
    let mut pc: u32 = 0x832B6958;
    'dispatch: loop {
        match pc {
            0x832B6958 => {
    //   block [0x832B6958..0x832B695C)
	// 832B6958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6960 size=4
    let mut pc: u32 = 0x832B6960;
    'dispatch: loop {
        match pc {
            0x832B6960 => {
    //   block [0x832B6960..0x832B6964)
	// 832B6960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6968 size=12
    let mut pc: u32 = 0x832B6968;
    'dispatch: loop {
        match pc {
            0x832B6968 => {
    //   block [0x832B6968..0x832B6974)
	// 832B6968: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B696C: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 832B6970: 4B51C600  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6978 size=12
    let mut pc: u32 = 0x832B6978;
    'dispatch: loop {
        match pc {
            0x832B6978 => {
    //   block [0x832B6978..0x832B6984)
	// 832B6978: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B697C: 386B604C  addi r3, r11, 0x604c
	ctx.r[3].s64 = ctx.r[11].s64 + 24652;
	// 832B6980: 4B51C5F0  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6988 size=12
    let mut pc: u32 = 0x832B6988;
    'dispatch: loop {
        match pc {
            0x832B6988 => {
    //   block [0x832B6988..0x832B6994)
	// 832B6988: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B698C: 386B6058  addi r3, r11, 0x6058
	ctx.r[3].s64 = ctx.r[11].s64 + 24664;
	// 832B6990: 4B8B93D0  b 0x82b6fd60
	sub_82B6FD60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6998 size=4
    let mut pc: u32 = 0x832B6998;
    'dispatch: loop {
        match pc {
            0x832B6998 => {
    //   block [0x832B6998..0x832B699C)
	// 832B6998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B69A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B69A0 size=88
    let mut pc: u32 = 0x832B69A0;
    'dispatch: loop {
        match pc {
            0x832B69A0 => {
    //   block [0x832B69A0..0x832B69F8)
	// 832B69A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B69A4: 4B9F2A65  bl 0x82ca9408
	ctx.lr = 0x832B69A8;
	sub_82CA93D0(ctx, base);
	// 832B69A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B69AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B69B0: 3B800003  li r28, 3
	ctx.r[28].s64 = 3;
	// 832B69B4: 396B6088  addi r11, r11, 0x6088
	ctx.r[11].s64 = ctx.r[11].s64 + 24712;
	// 832B69B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832B69BC: 3BEB0034  addi r31, r11, 0x34
	ctx.r[31].s64 = ctx.r[11].s64 + 52;
	// 832B69C0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B69C4: 3BAB2A2C  addi r29, r11, 0x2a2c
	ctx.r[29].s64 = ctx.r[11].s64 + 10796;
	// 832B69C8: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832B69CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B69D0: 93BFFFFC  stw r29, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 832B69D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B69D8: 419A0010  beq cr6, 0x832b69e8
	if ctx.cr[6].eq {
	pc = 0x832B69E8; continue 'dispatch;
	}
	// 832B69DC: 4AF45945  bl 0x821fc320
	ctx.lr = 0x832B69E0;
	sub_821FC320(ctx, base);
	// 832B69E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832B69E4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 832B69E8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832B69EC: 4080FFDC  bge 0x832b69c8
	if !ctx.cr[0].lt {
	pc = 0x832B69C8; continue 'dispatch;
	}
	// 832B69F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B69F4: 4B9F2A64  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B69F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B69F8 size=88
    let mut pc: u32 = 0x832B69F8;
    'dispatch: loop {
        match pc {
            0x832B69F8 => {
    //   block [0x832B69F8..0x832B6A50)
	// 832B69F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B69FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6A08: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6A0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B6A10: 3BEBB12C  addi r31, r11, -0x4ed4
	ctx.r[31].s64 = ctx.r[11].s64 + -20180;
	// 832B6A14: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B6A18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6A1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B6A20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6A24: 419A0018  beq cr6, 0x832b6a3c
	if ctx.cr[6].eq {
	pc = 0x832B6A3C; continue 'dispatch;
	}
	// 832B6A28: 4AF458F9  bl 0x821fc320
	ctx.lr = 0x832B6A2C;
	sub_821FC320(ctx, base);
	// 832B6A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6A34: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6A38: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6A50 size=80
    let mut pc: u32 = 0x832B6A50;
    'dispatch: loop {
        match pc {
            0x832B6A50 => {
    //   block [0x832B6A50..0x832B6AA0)
	// 832B6A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6A60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6A64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6A68: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 832B6A6C: 396B60C8  addi r11, r11, 0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + 24776;
	// 832B6A70: 3BEB0260  addi r31, r11, 0x260
	ctx.r[31].s64 = ctx.r[11].s64 + 608;
	// 832B6A74: 3BFFFFB4  addi r31, r31, -0x4c
	ctx.r[31].s64 = ctx.r[31].s64 + -76;
	// 832B6A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B6A7C: 4B8B7F15  bl 0x82b6e990
	ctx.lr = 0x832B6A80;
	sub_82B6E990(ctx, base);
	// 832B6A80: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B6A84: 4080FFF0  bge 0x832b6a74
	if !ctx.cr[0].lt {
	pc = 0x832B6A74; continue 'dispatch;
	}
	// 832B6A88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6A94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6A98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


