pub fn sub_82645DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645DE8 size=112
    let mut pc: u32 = 0x82645DE8;
    'dispatch: loop {
        match pc {
            0x82645DE8 => {
    //   block [0x82645DE8..0x82645E58)
	// 82645DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645DF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645DFC: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645E04: 390BE5C0  addi r8, r11, -0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + -6720;
	// 82645E08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82645E0C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82645E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645E14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645E20: 386AFB74  addi r3, r10, -0x48c
	ctx.r[3].s64 = ctx.r[10].s64 + -1164;
	// 82645E24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645E44: 4BE20FDD  bl 0x82466e20
	ctx.lr = 0x82645E48;
	sub_82466E20(ctx, base);
	// 82645E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645E58 size=112
    let mut pc: u32 = 0x82645E58;
    'dispatch: loop {
        match pc {
            0x82645E58 => {
    //   block [0x82645E58..0x82645EC8)
	// 82645E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645E6C: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645E74: 390BE5D8  addi r8, r11, -0x1a28
	ctx.r[8].s64 = ctx.r[11].s64 + -6696;
	// 82645E78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82645E7C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82645E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645E90: 386AFBA4  addi r3, r10, -0x45c
	ctx.r[3].s64 = ctx.r[10].s64 + -1116;
	// 82645E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645EB4: 4BE20F6D  bl 0x82466e20
	ctx.lr = 0x82645EB8;
	sub_82466E20(ctx, base);
	// 82645EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645EC8 size=112
    let mut pc: u32 = 0x82645EC8;
    'dispatch: loop {
        match pc {
            0x82645EC8 => {
    //   block [0x82645EC8..0x82645F38)
	// 82645EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645ED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645ED8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645EDC: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645EE4: 390BE608  addi r8, r11, -0x19f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6648;
	// 82645EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82645EEC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82645EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645EF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645F00: 386AFBD4  addi r3, r10, -0x42c
	ctx.r[3].s64 = ctx.r[10].s64 + -1068;
	// 82645F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645F24: 4BE20EFD  bl 0x82466e20
	ctx.lr = 0x82645F28;
	sub_82466E20(ctx, base);
	// 82645F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82645F38 size=24
    let mut pc: u32 = 0x82645F38;
    'dispatch: loop {
        match pc {
            0x82645F38 => {
    //   block [0x82645F38..0x82645F50)
	// 82645F38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645F3C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82645F40: 394A0C88  addi r10, r10, 0xc88
	ctx.r[10].s64 = ctx.r[10].s64 + 3208;
	// 82645F44: 816BDFD4  lwz r11, -0x202c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8236 as u32) ) } as u64;
	// 82645F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82645F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645F50 size=112
    let mut pc: u32 = 0x82645F50;
    'dispatch: loop {
        match pc {
            0x82645F50 => {
    //   block [0x82645F50..0x82645FC0)
	// 82645F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645F5C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82645F60: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645F64: 392A87A4  addi r9, r10, -0x785c
	ctx.r[9].s64 = ctx.r[10].s64 + -30812;
	// 82645F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645F6C: 390B0C88  addi r8, r11, 0xc88
	ctx.r[8].s64 = ctx.r[11].s64 + 3208;
	// 82645F70: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82645F74: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82645F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645F7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645F88: 386AFC04  addi r3, r10, -0x3fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1020;
	// 82645F8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82645F90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82645F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645FAC: 4BE20E75  bl 0x82466e20
	ctx.lr = 0x82645FB0;
	sub_82466E20(ctx, base);
	// 82645FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645FC0 size=108
    let mut pc: u32 = 0x82645FC0;
    'dispatch: loop {
        match pc {
            0x82645FC0 => {
    //   block [0x82645FC0..0x8264602C)
	// 82645FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645FCC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645FD4: 38EBE620  addi r7, r11, -0x19e0
	ctx.r[7].s64 = ctx.r[11].s64 + -6624;
	// 82645FD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82645FDC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82645FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645FF0: 386AFC34  addi r3, r10, -0x3cc
	ctx.r[3].s64 = ctx.r[10].s64 + -972;
	// 82645FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264600C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646018: 4BE20E09  bl 0x82466e20
	ctx.lr = 0x8264601C;
	sub_82466E20(ctx, base);
	// 8264601C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646030 size=108
    let mut pc: u32 = 0x82646030;
    'dispatch: loop {
        match pc {
            0x82646030 => {
    //   block [0x82646030..0x8264609C)
	// 82646030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264603C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646044: 38EBE638  addi r7, r11, -0x19c8
	ctx.r[7].s64 = ctx.r[11].s64 + -6600;
	// 82646048: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264604C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82646050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264605C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646060: 386AFC64  addi r3, r10, -0x39c
	ctx.r[3].s64 = ctx.r[10].s64 + -924;
	// 82646064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264606C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264607C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646088: 4BE20D99  bl 0x82466e20
	ctx.lr = 0x8264608C;
	sub_82466E20(ctx, base);
	// 8264608C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826460A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826460A0 size=112
    let mut pc: u32 = 0x826460A0;
    'dispatch: loop {
        match pc {
            0x826460A0 => {
    //   block [0x826460A0..0x82646110)
	// 826460A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826460A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826460A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826460AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826460B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826460B4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826460B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826460BC: 390BE684  addi r8, r11, -0x197c
	ctx.r[8].s64 = ctx.r[11].s64 + -6524;
	// 826460C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826460C4: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826460C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826460CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826460D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826460D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826460D8: 386AFC94  addi r3, r10, -0x36c
	ctx.r[3].s64 = ctx.r[10].s64 + -876;
	// 826460DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826460E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826460E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826460E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826460EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826460F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826460F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826460F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826460FC: 4BE20D25  bl 0x82466e20
	ctx.lr = 0x82646100;
	sub_82466E20(ctx, base);
	// 82646100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264610C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646110 size=108
    let mut pc: u32 = 0x82646110;
    'dispatch: loop {
        match pc {
            0x82646110 => {
    //   block [0x82646110..0x8264617C)
	// 82646110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264611C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646124: 38EBE6A0  addi r7, r11, -0x1960
	ctx.r[7].s64 = ctx.r[11].s64 + -6496;
	// 82646128: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264612C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82646130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264613C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646140: 386AFCC4  addi r3, r10, -0x33c
	ctx.r[3].s64 = ctx.r[10].s64 + -828;
	// 82646144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264614C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264615C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646168: 4BE20CB9  bl 0x82466e20
	ctx.lr = 0x8264616C;
	sub_82466E20(ctx, base);
	// 8264616C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82646180 size=24
    let mut pc: u32 = 0x82646180;
    'dispatch: loop {
        match pc {
            0x82646180 => {
    //   block [0x82646180..0x82646198)
	// 82646180: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646184: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82646188: 394A0CD0  addi r10, r10, 0xcd0
	ctx.r[10].s64 = ctx.r[10].s64 + 3280;
	// 8264618C: 816BE69C  lwz r11, -0x1964(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6500 as u32) ) } as u64;
	// 82646190: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82646194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646198 size=116
    let mut pc: u32 = 0x82646198;
    'dispatch: loop {
        match pc {
            0x82646198 => {
    //   block [0x82646198..0x8264620C)
	// 82646198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264619C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826461A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826461A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826461A8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826461AC: 390B0CD0  addi r8, r11, 0xcd0
	ctx.r[8].s64 = ctx.r[11].s64 + 3280;
	// 826461B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826461B4: 392A886C  addi r9, r10, -0x7794
	ctx.r[9].s64 = ctx.r[10].s64 + -30612;
	// 826461B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826461BC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826461C0: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826461C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826461C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826461CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826461D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826461D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826461D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826461DC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826461E0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826461E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826461E8: 386BFCF4  addi r3, r11, -0x30c
	ctx.r[3].s64 = ctx.r[11].s64 + -780;
	// 826461EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826461F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826461F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826461F8: 4BE20C29  bl 0x82466e20
	ctx.lr = 0x826461FC;
	sub_82466E20(ctx, base);
	// 826461FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646210 size=112
    let mut pc: u32 = 0x82646210;
    'dispatch: loop {
        match pc {
            0x82646210 => {
    //   block [0x82646210..0x82646280)
	// 82646210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264621C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646220: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646224: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264622C: 390BE700  addi r8, r11, -0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + -6400;
	// 82646230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82646234: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82646238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264623C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646248: 386AFD24  addi r3, r10, -0x2dc
	ctx.r[3].s64 = ctx.r[10].s64 + -732;
	// 8264624C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264625C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264626C: 4BE20BB5  bl 0x82466e20
	ctx.lr = 0x82646270;
	sub_82466E20(ctx, base);
	// 82646270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264627C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646280 size=108
    let mut pc: u32 = 0x82646280;
    'dispatch: loop {
        match pc {
            0x82646280 => {
    //   block [0x82646280..0x826462EC)
	// 82646280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264628C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646294: 38EBE730  addi r7, r11, -0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6352;
	// 82646298: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264629C: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826462A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826462A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826462A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826462AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826462B0: 386AFD54  addi r3, r10, -0x2ac
	ctx.r[3].s64 = ctx.r[10].s64 + -684;
	// 826462B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826462B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826462BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826462C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826462C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826462C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826462CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826462D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826462D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826462D8: 4BE20B49  bl 0x82466e20
	ctx.lr = 0x826462DC;
	sub_82466E20(ctx, base);
	// 826462DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826462E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826462E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826462E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826462F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826462F0 size=108
    let mut pc: u32 = 0x826462F0;
    'dispatch: loop {
        match pc {
            0x826462F0 => {
    //   block [0x826462F0..0x8264635C)
	// 826462F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826462F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826462F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826462FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646304: 38EBE778  addi r7, r11, -0x1888
	ctx.r[7].s64 = ctx.r[11].s64 + -6280;
	// 82646308: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264630C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82646310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646314: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264631C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646320: 386AFD84  addi r3, r10, -0x27c
	ctx.r[3].s64 = ctx.r[10].s64 + -636;
	// 82646324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264632C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264633C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646348: 4BE20AD9  bl 0x82466e20
	ctx.lr = 0x8264634C;
	sub_82466E20(ctx, base);
	// 8264634C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646360 size=112
    let mut pc: u32 = 0x82646360;
    'dispatch: loop {
        match pc {
            0x82646360 => {
    //   block [0x82646360..0x826463D0)
	// 82646360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264636C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646370: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646374: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264637C: 390BE7A8  addi r8, r11, -0x1858
	ctx.r[8].s64 = ctx.r[11].s64 + -6232;
	// 82646380: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82646384: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82646388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264638C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646398: 386AFDB4  addi r3, r10, -0x24c
	ctx.r[3].s64 = ctx.r[10].s64 + -588;
	// 8264639C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826463A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826463A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826463A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826463AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826463B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826463B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826463B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826463BC: 4BE20A65  bl 0x82466e20
	ctx.lr = 0x826463C0;
	sub_82466E20(ctx, base);
	// 826463C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826463C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826463C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826463CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826463D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826463D0 size=108
    let mut pc: u32 = 0x826463D0;
    'dispatch: loop {
        match pc {
            0x826463D0 => {
    //   block [0x826463D0..0x8264643C)
	// 826463D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826463D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826463D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826463DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826463E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826463E4: 38EBE7D8  addi r7, r11, -0x1828
	ctx.r[7].s64 = ctx.r[11].s64 + -6184;
	// 826463E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826463EC: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826463F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826463F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826463F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826463FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646400: 386AFDE4  addi r3, r10, -0x21c
	ctx.r[3].s64 = ctx.r[10].s64 + -540;
	// 82646404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264640C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264641C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646428: 4BE209F9  bl 0x82466e20
	ctx.lr = 0x8264642C;
	sub_82466E20(ctx, base);
	// 8264642C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646440 size=108
    let mut pc: u32 = 0x82646440;
    'dispatch: loop {
        match pc {
            0x82646440 => {
    //   block [0x82646440..0x826464AC)
	// 82646440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264644C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646454: 38EBE838  addi r7, r11, -0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + -6088;
	// 82646458: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264645C: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82646460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264646C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646470: 386AFE14  addi r3, r10, -0x1ec
	ctx.r[3].s64 = ctx.r[10].s64 + -492;
	// 82646474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264647C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264648C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646498: 4BE20989  bl 0x82466e20
	ctx.lr = 0x8264649C;
	sub_82466E20(ctx, base);
	// 8264649C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826464A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826464A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826464A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826464B0 size=112
    let mut pc: u32 = 0x826464B0;
    'dispatch: loop {
        match pc {
            0x826464B0 => {
    //   block [0x826464B0..0x82646520)
	// 826464B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826464B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826464B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826464BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826464C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826464C4: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826464C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826464CC: 390BE880  addi r8, r11, -0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + -6016;
	// 826464D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826464D4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826464D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826464DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826464E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826464E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826464E8: 386AFE44  addi r3, r10, -0x1bc
	ctx.r[3].s64 = ctx.r[10].s64 + -444;
	// 826464EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826464F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826464F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826464F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826464FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264650C: 4BE20915  bl 0x82466e20
	ctx.lr = 0x82646510;
	sub_82466E20(ctx, base);
	// 82646510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646520 size=112
    let mut pc: u32 = 0x82646520;
    'dispatch: loop {
        match pc {
            0x82646520 => {
    //   block [0x82646520..0x82646590)
	// 82646520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264652C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646530: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646534: 38AAFEA4  addi r5, r10, -0x15c
	ctx.r[5].s64 = ctx.r[10].s64 + -348;
	// 82646538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264653C: 390BE8F8  addi r8, r11, -0x1708
	ctx.r[8].s64 = ctx.r[11].s64 + -5896;
	// 82646540: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82646544: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82646548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264654C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646558: 386AFE74  addi r3, r10, -0x18c
	ctx.r[3].s64 = ctx.r[10].s64 + -396;
	// 8264655C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264656C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264657C: 4BE208A5  bl 0x82466e20
	ctx.lr = 0x82646580;
	sub_82466E20(ctx, base);
	// 82646580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264658C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646590 size=100
    let mut pc: u32 = 0x82646590;
    'dispatch: loop {
        match pc {
            0x82646590 => {
    //   block [0x82646590..0x826465F4)
	// 82646590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264659C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826465A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826465A4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826465A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826465AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826465B0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826465B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826465B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826465BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826465C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826465C4: 386AFEA4  addi r3, r10, -0x15c
	ctx.r[3].s64 = ctx.r[10].s64 + -348;
	// 826465C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826465CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826465D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826465D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826465D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826465DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826465E0: 4BE20841  bl 0x82466e20
	ctx.lr = 0x826465E4;
	sub_82466E20(ctx, base);
	// 826465E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826465E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826465EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826465F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826465F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826465F8 size=24
    let mut pc: u32 = 0x826465F8;
    'dispatch: loop {
        match pc {
            0x826465F8 => {
    //   block [0x826465F8..0x82646610)
	// 826465F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826465FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82646600: 394A0D90  addi r10, r10, 0xd90
	ctx.r[10].s64 = ctx.r[10].s64 + 3472;
	// 82646604: 816BE970  lwz r11, -0x1690(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5776 as u32) ) } as u64;
	// 82646608: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8264660C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646610 size=116
    let mut pc: u32 = 0x82646610;
    'dispatch: loop {
        match pc {
            0x82646610 => {
    //   block [0x82646610..0x82646684)
	// 82646610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264661C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646620: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82646624: 390B0D90  addi r8, r11, 0xd90
	ctx.r[8].s64 = ctx.r[11].s64 + 3472;
	// 82646628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264662C: 392A88A8  addi r9, r10, -0x7758
	ctx.r[9].s64 = ctx.r[10].s64 + -30552;
	// 82646630: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646634: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82646638: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 8264663C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646644: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264664C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646654: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82646658: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8264665C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82646660: 386BFED4  addi r3, r11, -0x12c
	ctx.r[3].s64 = ctx.r[11].s64 + -300;
	// 82646664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82646668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264666C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646670: 4BE207B1  bl 0x82466e20
	ctx.lr = 0x82646674;
	sub_82466E20(ctx, base);
	// 82646674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264667C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646688 size=112
    let mut pc: u32 = 0x82646688;
    'dispatch: loop {
        match pc {
            0x82646688 => {
    //   block [0x82646688..0x826466F8)
	// 82646688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646698: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264669C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826466A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826466A4: 390BE978  addi r8, r11, -0x1688
	ctx.r[8].s64 = ctx.r[11].s64 + -5768;
	// 826466A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826466AC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826466B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826466B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826466B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826466BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826466C0: 386AFF04  addi r3, r10, -0xfc
	ctx.r[3].s64 = ctx.r[10].s64 + -252;
	// 826466C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826466C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826466CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826466D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826466D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826466D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826466DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826466E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826466E4: 4BE2073D  bl 0x82466e20
	ctx.lr = 0x826466E8;
	sub_82466E20(ctx, base);
	// 826466E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826466EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826466F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826466F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826466F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826466F8 size=112
    let mut pc: u32 = 0x826466F8;
    'dispatch: loop {
        match pc {
            0x826466F8 => {
    //   block [0x826466F8..0x82646768)
	// 826466F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826466FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646708: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264670C: 38AAFE44  addi r5, r10, -0x1bc
	ctx.r[5].s64 = ctx.r[10].s64 + -444;
	// 82646710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646714: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 82646718: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264671C: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82646720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264672C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646730: 386AFF34  addi r3, r10, -0xcc
	ctx.r[3].s64 = ctx.r[10].s64 + -204;
	// 82646734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264673C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264674C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646754: 4BE206CD  bl 0x82466e20
	ctx.lr = 0x82646758;
	sub_82466E20(ctx, base);
	// 82646758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646768 size=108
    let mut pc: u32 = 0x82646768;
    'dispatch: loop {
        match pc {
            0x82646768 => {
    //   block [0x82646768..0x826467D4)
	// 82646768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264676C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646774: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264677C: 38EBEA20  addi r7, r11, -0x15e0
	ctx.r[7].s64 = ctx.r[11].s64 + -5600;
	// 82646780: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82646784: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82646788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264678C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646798: 386AFF64  addi r3, r10, -0x9c
	ctx.r[3].s64 = ctx.r[10].s64 + -156;
	// 8264679C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826467A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826467A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826467A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826467AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826467B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826467B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826467B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826467BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826467C0: 4BE20661  bl 0x82466e20
	ctx.lr = 0x826467C4;
	sub_82466E20(ctx, base);
	// 826467C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826467C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826467CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826467D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826467D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826467D8 size=108
    let mut pc: u32 = 0x826467D8;
    'dispatch: loop {
        match pc {
            0x826467D8 => {
    //   block [0x826467D8..0x82646844)
	// 826467D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826467DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826467E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826467E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826467E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826467EC: 38EBEA68  addi r7, r11, -0x1598
	ctx.r[7].s64 = ctx.r[11].s64 + -5528;
	// 826467F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826467F4: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 826467F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826467FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646808: 386AFF94  addi r3, r10, -0x6c
	ctx.r[3].s64 = ctx.r[10].s64 + -108;
	// 8264680C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264681C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264682C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646830: 4BE205F1  bl 0x82466e20
	ctx.lr = 0x82646834;
	sub_82466E20(ctx, base);
	// 82646834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646848 size=112
    let mut pc: u32 = 0x82646848;
    'dispatch: loop {
        match pc {
            0x82646848 => {
    //   block [0x82646848..0x826468B8)
	// 82646848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646854: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646858: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264685C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646864: 390BEAB0  addi r8, r11, -0x1550
	ctx.r[8].s64 = ctx.r[11].s64 + -5456;
	// 82646868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264686C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82646870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646874: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264687C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646880: 386AFFC4  addi r3, r10, -0x3c
	ctx.r[3].s64 = ctx.r[10].s64 + -60;
	// 82646884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264689C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826468A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826468A4: 4BE2057D  bl 0x82466e20
	ctx.lr = 0x826468A8;
	sub_82466E20(ctx, base);
	// 826468A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826468AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826468B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826468B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826468B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826468B8 size=112
    let mut pc: u32 = 0x826468B8;
    'dispatch: loop {
        match pc {
            0x826468B8 => {
    //   block [0x826468B8..0x82646928)
	// 826468B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826468BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826468C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826468C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826468C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826468CC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826468D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826468D4: 390BEB58  addi r8, r11, -0x14a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5288;
	// 826468D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826468DC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826468E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826468E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826468E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826468EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826468F0: 386AFFF4  addi r3, r10, -0xc
	ctx.r[3].s64 = ctx.r[10].s64 + -12;
	// 826468F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826468F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826468FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646914: 4BE2050D  bl 0x82466e20
	ctx.lr = 0x82646918;
	sub_82466E20(ctx, base);
	// 82646918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646928 size=108
    let mut pc: u32 = 0x82646928;
    'dispatch: loop {
        match pc {
            0x82646928 => {
    //   block [0x82646928..0x82646994)
	// 82646928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646934: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264693C: 38EBEBA0  addi r7, r11, -0x1460
	ctx.r[7].s64 = ctx.r[11].s64 + -5216;
	// 82646940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82646944: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82646948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264694C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646958: 386A0024  addi r3, r10, 0x24
	ctx.r[3].s64 = ctx.r[10].s64 + 36;
	// 8264695C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646980: 4BE204A1  bl 0x82466e20
	ctx.lr = 0x82646984;
	sub_82466E20(ctx, base);
	// 82646984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264698C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646998 size=108
    let mut pc: u32 = 0x82646998;
    'dispatch: loop {
        match pc {
            0x82646998 => {
    //   block [0x82646998..0x82646A04)
	// 82646998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826469A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826469A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826469A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826469AC: 38EBEBD0  addi r7, r11, -0x1430
	ctx.r[7].s64 = ctx.r[11].s64 + -5168;
	// 826469B0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826469B4: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826469B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826469BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826469C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826469C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826469C8: 386A0054  addi r3, r10, 0x54
	ctx.r[3].s64 = ctx.r[10].s64 + 84;
	// 826469CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826469D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826469D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826469D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826469DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826469E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826469E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826469E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826469EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826469F0: 4BE20431  bl 0x82466e20
	ctx.lr = 0x826469F4;
	sub_82466E20(ctx, base);
	// 826469F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826469F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826469FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646A08 size=112
    let mut pc: u32 = 0x82646A08;
    'dispatch: loop {
        match pc {
            0x82646A08 => {
    //   block [0x82646A08..0x82646A78)
	// 82646A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646A1C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646A24: 390BEC60  addi r8, r11, -0x13a0
	ctx.r[8].s64 = ctx.r[11].s64 + -5024;
	// 82646A28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82646A2C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82646A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646A40: 386A0084  addi r3, r10, 0x84
	ctx.r[3].s64 = ctx.r[10].s64 + 132;
	// 82646A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646A64: 4BE203BD  bl 0x82466e20
	ctx.lr = 0x82646A68;
	sub_82466E20(ctx, base);
	// 82646A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646A78 size=112
    let mut pc: u32 = 0x82646A78;
    'dispatch: loop {
        match pc {
            0x82646A78 => {
    //   block [0x82646A78..0x82646AE8)
	// 82646A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646A8C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646A94: 390BECF0  addi r8, r11, -0x1310
	ctx.r[8].s64 = ctx.r[11].s64 + -4880;
	// 82646A98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82646A9C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82646AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646AB0: 386A00B4  addi r3, r10, 0xb4
	ctx.r[3].s64 = ctx.r[10].s64 + 180;
	// 82646AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646AD4: 4BE2034D  bl 0x82466e20
	ctx.lr = 0x82646AD8;
	sub_82466E20(ctx, base);
	// 82646AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646AE8 size=112
    let mut pc: u32 = 0x82646AE8;
    'dispatch: loop {
        match pc {
            0x82646AE8 => {
    //   block [0x82646AE8..0x82646B58)
	// 82646AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646AFC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646B04: 390BED98  addi r8, r11, -0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + -4712;
	// 82646B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82646B0C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82646B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646B20: 386A00E4  addi r3, r10, 0xe4
	ctx.r[3].s64 = ctx.r[10].s64 + 228;
	// 82646B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646B44: 4BE202DD  bl 0x82466e20
	ctx.lr = 0x82646B48;
	sub_82466E20(ctx, base);
	// 82646B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646B58 size=108
    let mut pc: u32 = 0x82646B58;
    'dispatch: loop {
        match pc {
            0x82646B58 => {
    //   block [0x82646B58..0x82646BC4)
	// 82646B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646B64: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646B6C: 38EBEDB0  addi r7, r11, -0x1250
	ctx.r[7].s64 = ctx.r[11].s64 + -4688;
	// 82646B70: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82646B74: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82646B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646B88: 386A0114  addi r3, r10, 0x114
	ctx.r[3].s64 = ctx.r[10].s64 + 276;
	// 82646B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646BB0: 4BE20271  bl 0x82466e20
	ctx.lr = 0x82646BB4;
	sub_82466E20(ctx, base);
	// 82646BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646BC8 size=112
    let mut pc: u32 = 0x82646BC8;
    'dispatch: loop {
        match pc {
            0x82646BC8 => {
    //   block [0x82646BC8..0x82646C38)
	// 82646BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646BD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646BD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646BDC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646BE4: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 82646BE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82646BEC: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82646BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646BF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646C00: 386A0144  addi r3, r10, 0x144
	ctx.r[3].s64 = ctx.r[10].s64 + 324;
	// 82646C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646C24: 4BE201FD  bl 0x82466e20
	ctx.lr = 0x82646C28;
	sub_82466E20(ctx, base);
	// 82646C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646C38 size=100
    let mut pc: u32 = 0x82646C38;
    'dispatch: loop {
        match pc {
            0x82646C38 => {
    //   block [0x82646C38..0x82646C9C)
	// 82646C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646C4C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82646C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646C58: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82646C5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646C6C: 386A0174  addi r3, r10, 0x174
	ctx.r[3].s64 = ctx.r[10].s64 + 372;
	// 82646C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646C74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646C88: 4BE20199  bl 0x82466e20
	ctx.lr = 0x82646C8C;
	sub_82466E20(ctx, base);
	// 82646C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646CA0 size=112
    let mut pc: u32 = 0x82646CA0;
    'dispatch: loop {
        match pc {
            0x82646CA0 => {
    //   block [0x82646CA0..0x82646D10)
	// 82646CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646CAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646CB0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646CB4: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646CBC: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 82646CC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82646CC4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82646CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646CCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646CD8: 386A01A4  addi r3, r10, 0x1a4
	ctx.r[3].s64 = ctx.r[10].s64 + 420;
	// 82646CDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646CFC: 4BE20125  bl 0x82466e20
	ctx.lr = 0x82646D00;
	sub_82466E20(ctx, base);
	// 82646D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646D10 size=112
    let mut pc: u32 = 0x82646D10;
    'dispatch: loop {
        match pc {
            0x82646D10 => {
    //   block [0x82646D10..0x82646D80)
	// 82646D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646D1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646D24: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646D2C: 390BEEE8  addi r8, r11, -0x1118
	ctx.r[8].s64 = ctx.r[11].s64 + -4376;
	// 82646D30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82646D34: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82646D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646D48: 386A01D4  addi r3, r10, 0x1d4
	ctx.r[3].s64 = ctx.r[10].s64 + 468;
	// 82646D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646D6C: 4BE200B5  bl 0x82466e20
	ctx.lr = 0x82646D70;
	sub_82466E20(ctx, base);
	// 82646D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646D80 size=112
    let mut pc: u32 = 0x82646D80;
    'dispatch: loop {
        match pc {
            0x82646D80 => {
    //   block [0x82646D80..0x82646DF0)
	// 82646D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D90: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646D94: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646D9C: 390BEF48  addi r8, r11, -0x10b8
	ctx.r[8].s64 = ctx.r[11].s64 + -4280;
	// 82646DA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82646DA4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82646DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646DAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646DB8: 386A0204  addi r3, r10, 0x204
	ctx.r[3].s64 = ctx.r[10].s64 + 516;
	// 82646DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646DDC: 4BE20045  bl 0x82466e20
	ctx.lr = 0x82646DE0;
	sub_82466E20(ctx, base);
	// 82646DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646DF0 size=112
    let mut pc: u32 = 0x82646DF0;
    'dispatch: loop {
        match pc {
            0x82646DF0 => {
    //   block [0x82646DF0..0x82646E60)
	// 82646DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E00: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646E04: 38AA0654  addi r5, r10, 0x654
	ctx.r[5].s64 = ctx.r[10].s64 + 1620;
	// 82646E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646E0C: 390BEFA8  addi r8, r11, -0x1058
	ctx.r[8].s64 = ctx.r[11].s64 + -4184;
	// 82646E10: 39200011  li r9, 0x11
	ctx.r[9].s64 = 17;
	// 82646E14: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82646E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646E28: 386A0234  addi r3, r10, 0x234
	ctx.r[3].s64 = ctx.r[10].s64 + 564;
	// 82646E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646E4C: 4BE1FFD5  bl 0x82466e20
	ctx.lr = 0x82646E50;
	sub_82466E20(ctx, base);
	// 82646E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646E60 size=100
    let mut pc: u32 = 0x82646E60;
    'dispatch: loop {
        match pc {
            0x82646E60 => {
    //   block [0x82646E60..0x82646EC4)
	// 82646E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646E74: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82646E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646E80: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82646E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646E94: 386A0264  addi r3, r10, 0x264
	ctx.r[3].s64 = ctx.r[10].s64 + 612;
	// 82646E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646EB0: 4BE1FF71  bl 0x82466e20
	ctx.lr = 0x82646EB4;
	sub_82466E20(ctx, base);
	// 82646EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646EC8 size=100
    let mut pc: u32 = 0x82646EC8;
    'dispatch: loop {
        match pc {
            0x82646EC8 => {
    //   block [0x82646EC8..0x82646F2C)
	// 82646EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646ED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646EDC: 38AA02F4  addi r5, r10, 0x2f4
	ctx.r[5].s64 = ctx.r[10].s64 + 756;
	// 82646EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646EE8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82646EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646EFC: 386A0294  addi r3, r10, 0x294
	ctx.r[3].s64 = ctx.r[10].s64 + 660;
	// 82646F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646F04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646F08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646F10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646F18: 4BE1FF09  bl 0x82466e20
	ctx.lr = 0x82646F1C;
	sub_82466E20(ctx, base);
	// 82646F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646F30 size=100
    let mut pc: u32 = 0x82646F30;
    'dispatch: loop {
        match pc {
            0x82646F30 => {
    //   block [0x82646F30..0x82646F94)
	// 82646F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646F44: 38AA0234  addi r5, r10, 0x234
	ctx.r[5].s64 = ctx.r[10].s64 + 564;
	// 82646F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646F50: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82646F54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646F64: 386A02C4  addi r3, r10, 0x2c4
	ctx.r[3].s64 = ctx.r[10].s64 + 708;
	// 82646F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646F80: 4BE1FEA1  bl 0x82466e20
	ctx.lr = 0x82646F84;
	sub_82466E20(ctx, base);
	// 82646F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646F98 size=104
    let mut pc: u32 = 0x82646F98;
    'dispatch: loop {
        match pc {
            0x82646F98 => {
    //   block [0x82646F98..0x82647000)
	// 82646F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646FA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82646FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646FAC: 392A88DC  addi r9, r10, -0x7724
	ctx.r[9].s64 = ctx.r[10].s64 + -30500;
	// 82646FB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646FB8: 38AA0264  addi r5, r10, 0x264
	ctx.r[5].s64 = ctx.r[10].s64 + 612;
	// 82646FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646FCC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82646FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646FD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646FE4: 386A02F4  addi r3, r10, 0x2f4
	ctx.r[3].s64 = ctx.r[10].s64 + 756;
	// 82646FE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82646FEC: 4BE1FE35  bl 0x82466e20
	ctx.lr = 0x82646FF0;
	sub_82466E20(ctx, base);
	// 82646FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647000 size=108
    let mut pc: u32 = 0x82647000;
    'dispatch: loop {
        match pc {
            0x82647000 => {
    //   block [0x82647000..0x8264706C)
	// 82647000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264700C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647014: 38EBF140  addi r7, r11, -0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + -3776;
	// 82647018: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264701C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82647020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264702C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647030: 386A0324  addi r3, r10, 0x324
	ctx.r[3].s64 = ctx.r[10].s64 + 804;
	// 82647034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264703C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647058: 4BE1FDC9  bl 0x82466e20
	ctx.lr = 0x8264705C;
	sub_82466E20(ctx, base);
	// 8264705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647070 size=112
    let mut pc: u32 = 0x82647070;
    'dispatch: loop {
        match pc {
            0x82647070 => {
    //   block [0x82647070..0x826470E0)
	// 82647070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264707C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647084: 38AA02F4  addi r5, r10, 0x2f4
	ctx.r[5].s64 = ctx.r[10].s64 + 756;
	// 82647088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264708C: 390BF170  addi r8, r11, -0xe90
	ctx.r[8].s64 = ctx.r[11].s64 + -3728;
	// 82647090: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82647094: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82647098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264709C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826470A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826470A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826470A8: 386A0354  addi r3, r10, 0x354
	ctx.r[3].s64 = ctx.r[10].s64 + 852;
	// 826470AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826470B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826470B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826470B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826470BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826470C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826470C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826470C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826470CC: 4BE1FD55  bl 0x82466e20
	ctx.lr = 0x826470D0;
	sub_82466E20(ctx, base);
	// 826470D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826470D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826470D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826470DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826470E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826470E0 size=116
    let mut pc: u32 = 0x826470E0;
    'dispatch: loop {
        match pc {
            0x826470E0 => {
    //   block [0x826470E0..0x82647154)
	// 826470E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826470E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826470E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826470EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826470F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826470F4: 390BF21C  addi r8, r11, -0xde4
	ctx.r[8].s64 = ctx.r[11].s64 + -3556;
	// 826470F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826470FC: 392A8938  addi r9, r10, -0x76c8
	ctx.r[9].s64 = ctx.r[10].s64 + -30408;
	// 82647100: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647104: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82647108: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 8264710C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264711C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647124: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82647128: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8264712C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82647130: 386B0384  addi r3, r11, 0x384
	ctx.r[3].s64 = ctx.r[11].s64 + 900;
	// 82647134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82647138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647140: 4BE1FCE1  bl 0x82466e20
	ctx.lr = 0x82647144;
	sub_82466E20(ctx, base);
	// 82647144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647158 size=112
    let mut pc: u32 = 0x82647158;
    'dispatch: loop {
        match pc {
            0x82647158 => {
    //   block [0x82647158..0x826471C8)
	// 82647158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647168: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264716C: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647174: 390BF234  addi r8, r11, -0xdcc
	ctx.r[8].s64 = ctx.r[11].s64 + -3532;
	// 82647178: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264717C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82647180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647190: 386A03B4  addi r3, r10, 0x3b4
	ctx.r[3].s64 = ctx.r[10].s64 + 948;
	// 82647194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826471A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826471A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826471A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826471AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826471B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826471B4: 4BE1FC6D  bl 0x82466e20
	ctx.lr = 0x826471B8;
	sub_82466E20(ctx, base);
	// 826471B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826471BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826471C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826471C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826471C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826471C8 size=100
    let mut pc: u32 = 0x826471C8;
    'dispatch: loop {
        match pc {
            0x826471C8 => {
    //   block [0x826471C8..0x8264722C)
	// 826471C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826471CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826471D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826471D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826471D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826471DC: 38AA0414  addi r5, r10, 0x414
	ctx.r[5].s64 = ctx.r[10].s64 + 1044;
	// 826471E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826471E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826471E8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826471EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826471F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826471F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826471F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826471FC: 386A03E4  addi r3, r10, 0x3e4
	ctx.r[3].s64 = ctx.r[10].s64 + 996;
	// 82647200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264720C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82647214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647218: 4BE1FC09  bl 0x82466e20
	ctx.lr = 0x8264721C;
	sub_82466E20(ctx, base);
	// 8264721C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647230 size=112
    let mut pc: u32 = 0x82647230;
    'dispatch: loop {
        match pc {
            0x82647230 => {
    //   block [0x82647230..0x826472A0)
	// 82647230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264723C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647240: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647244: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264724C: 390BF24C  addi r8, r11, -0xdb4
	ctx.r[8].s64 = ctx.r[11].s64 + -3508;
	// 82647250: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82647254: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82647258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264725C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647268: 386A0414  addi r3, r10, 0x414
	ctx.r[3].s64 = ctx.r[10].s64 + 1044;
	// 8264726C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264727C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264728C: 4BE1FB95  bl 0x82466e20
	ctx.lr = 0x82647290;
	sub_82466E20(ctx, base);
	// 82647290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264729C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826472A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826472A0 size=112
    let mut pc: u32 = 0x826472A0;
    'dispatch: loop {
        match pc {
            0x826472A0 => {
    //   block [0x826472A0..0x82647310)
	// 826472A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826472A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826472A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826472AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826472B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826472B4: 38AA0384  addi r5, r10, 0x384
	ctx.r[5].s64 = ctx.r[10].s64 + 900;
	// 826472B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826472BC: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 826472C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826472C4: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 826472C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826472CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826472D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826472D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826472D8: 386A0444  addi r3, r10, 0x444
	ctx.r[3].s64 = ctx.r[10].s64 + 1092;
	// 826472DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826472E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826472E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826472E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826472EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826472F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826472F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826472F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826472FC: 4BE1FB25  bl 0x82466e20
	ctx.lr = 0x82647300;
	sub_82466E20(ctx, base);
	// 82647300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647310 size=100
    let mut pc: u32 = 0x82647310;
    'dispatch: loop {
        match pc {
            0x82647310 => {
    //   block [0x82647310..0x82647374)
	// 82647310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264731C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647324: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264732C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647330: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82647334: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264733C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647344: 386A0474  addi r3, r10, 0x474
	ctx.r[3].s64 = ctx.r[10].s64 + 1140;
	// 82647348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264734C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82647354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264735C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647360: 4BE1FAC1  bl 0x82466e20
	ctx.lr = 0x82647364;
	sub_82466E20(ctx, base);
	// 82647364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647378 size=100
    let mut pc: u32 = 0x82647378;
    'dispatch: loop {
        match pc {
            0x82647378 => {
    //   block [0x82647378..0x826473DC)
	// 82647378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264738C: 38AA03B4  addi r5, r10, 0x3b4
	ctx.r[5].s64 = ctx.r[10].s64 + 948;
	// 82647390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647398: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8264739C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826473A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826473A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826473A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826473AC: 386A04A4  addi r3, r10, 0x4a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1188;
	// 826473B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826473B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826473B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826473BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826473C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826473C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826473C8: 4BE1FA59  bl 0x82466e20
	ctx.lr = 0x826473CC;
	sub_82466E20(ctx, base);
	// 826473CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826473D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826473D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826473D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826473E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826473E0 size=100
    let mut pc: u32 = 0x826473E0;
    'dispatch: loop {
        match pc {
            0x826473E0 => {
    //   block [0x826473E0..0x82647444)
	// 826473E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826473E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826473E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826473EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826473F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826473F4: 38AA0474  addi r5, r10, 0x474
	ctx.r[5].s64 = ctx.r[10].s64 + 1140;
	// 826473F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826473FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647400: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82647404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264740C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647414: 386A04D4  addi r3, r10, 0x4d4
	ctx.r[3].s64 = ctx.r[10].s64 + 1236;
	// 82647418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264741C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82647424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647430: 4BE1F9F1  bl 0x82466e20
	ctx.lr = 0x82647434;
	sub_82466E20(ctx, base);
	// 82647434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647448 size=112
    let mut pc: u32 = 0x82647448;
    'dispatch: loop {
        match pc {
            0x82647448 => {
    //   block [0x82647448..0x826474B8)
	// 82647448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264745C: 38AA0564  addi r5, r10, 0x564
	ctx.r[5].s64 = ctx.r[10].s64 + 1380;
	// 82647460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647464: 390BF328  addi r8, r11, -0xcd8
	ctx.r[8].s64 = ctx.r[11].s64 + -3288;
	// 82647468: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264746C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82647470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264747C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647480: 386A0504  addi r3, r10, 0x504
	ctx.r[3].s64 = ctx.r[10].s64 + 1284;
	// 82647484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264748C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826474A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826474A4: 4BE1F97D  bl 0x82466e20
	ctx.lr = 0x826474A8;
	sub_82466E20(ctx, base);
	// 826474A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826474AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826474B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826474B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826474B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826474B8 size=112
    let mut pc: u32 = 0x826474B8;
    'dispatch: loop {
        match pc {
            0x826474B8 => {
    //   block [0x826474B8..0x82647528)
	// 826474B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826474BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826474C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826474C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826474C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826474CC: 38AA0594  addi r5, r10, 0x594
	ctx.r[5].s64 = ctx.r[10].s64 + 1428;
	// 826474D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826474D4: 390BF358  addi r8, r11, -0xca8
	ctx.r[8].s64 = ctx.r[11].s64 + -3240;
	// 826474D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826474DC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826474E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826474E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826474E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826474EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826474F0: 386A0534  addi r3, r10, 0x534
	ctx.r[3].s64 = ctx.r[10].s64 + 1332;
	// 826474F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826474F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826474FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264750C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647514: 4BE1F90D  bl 0x82466e20
	ctx.lr = 0x82647518;
	sub_82466E20(ctx, base);
	// 82647518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264751C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647528 size=112
    let mut pc: u32 = 0x82647528;
    'dispatch: loop {
        match pc {
            0x82647528 => {
    //   block [0x82647528..0x82647598)
	// 82647528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647538: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264753C: 38AA0654  addi r5, r10, 0x654
	ctx.r[5].s64 = ctx.r[10].s64 + 1620;
	// 82647540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647544: 390BF370  addi r8, r11, -0xc90
	ctx.r[8].s64 = ctx.r[11].s64 + -3216;
	// 82647548: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264754C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82647550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264755C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647560: 386A0564  addi r3, r10, 0x564
	ctx.r[3].s64 = ctx.r[10].s64 + 1380;
	// 82647564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264756C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264757C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647584: 4BE1F89D  bl 0x82466e20
	ctx.lr = 0x82647588;
	sub_82466E20(ctx, base);
	// 82647588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264758C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647598 size=112
    let mut pc: u32 = 0x82647598;
    'dispatch: loop {
        match pc {
            0x82647598 => {
    //   block [0x82647598..0x82647608)
	// 82647598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826475A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826475A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826475A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826475AC: 38AA0564  addi r5, r10, 0x564
	ctx.r[5].s64 = ctx.r[10].s64 + 1380;
	// 826475B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826475B4: 390BF3A0  addi r8, r11, -0xc60
	ctx.r[8].s64 = ctx.r[11].s64 + -3168;
	// 826475B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826475BC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826475C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826475C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826475C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826475CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826475D0: 386A0594  addi r3, r10, 0x594
	ctx.r[3].s64 = ctx.r[10].s64 + 1428;
	// 826475D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826475D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826475DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826475E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826475E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826475E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826475EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826475F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826475F4: 4BE1F82D  bl 0x82466e20
	ctx.lr = 0x826475F8;
	sub_82466E20(ctx, base);
	// 826475F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826475FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647608 size=112
    let mut pc: u32 = 0x82647608;
    'dispatch: loop {
        match pc {
            0x82647608 => {
    //   block [0x82647608..0x82647678)
	// 82647608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647618: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264761C: 38AA0594  addi r5, r10, 0x594
	ctx.r[5].s64 = ctx.r[10].s64 + 1428;
	// 82647620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647624: 390BF3B8  addi r8, r11, -0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + -3144;
	// 82647628: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264762C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82647630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647634: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264763C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647640: 386A05C4  addi r3, r10, 0x5c4
	ctx.r[3].s64 = ctx.r[10].s64 + 1476;
	// 82647644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264764C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264765C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647664: 4BE1F7BD  bl 0x82466e20
	ctx.lr = 0x82647668;
	sub_82466E20(ctx, base);
	// 82647668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264766C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647678 size=112
    let mut pc: u32 = 0x82647678;
    'dispatch: loop {
        match pc {
            0x82647678 => {
    //   block [0x82647678..0x826476E8)
	// 82647678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647684: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264768C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647694: 390BF3D0  addi r8, r11, -0xc30
	ctx.r[8].s64 = ctx.r[11].s64 + -3120;
	// 82647698: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264769C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826476A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826476A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826476A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826476AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826476B0: 386A05F4  addi r3, r10, 0x5f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1524;
	// 826476B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826476B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826476BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826476C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826476C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826476C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826476CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826476D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826476D4: 4BE1F74D  bl 0x82466e20
	ctx.lr = 0x826476D8;
	sub_82466E20(ctx, base);
	// 826476D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826476DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826476E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826476E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826476E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826476E8 size=48
    let mut pc: u32 = 0x826476E8;
    'dispatch: loop {
        match pc {
            0x826476E8 => {
    //   block [0x826476E8..0x82647718)
	// 826476E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826476EC: 814BF468  lwz r10, -0xb98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) } as u64;
	// 826476F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826476F4: 396B0DF0  addi r11, r11, 0xdf0
	ctx.r[11].s64 = ctx.r[11].s64 + 3568;
	// 826476F8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826476FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82647700: 814AF464  lwz r10, -0xb9c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2972 as u32) ) } as u64;
	// 82647704: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82647708: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264770C: 814AF460  lwz r10, -0xba0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2976 as u32) ) } as u64;
	// 82647710: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 82647714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647718 size=116
    let mut pc: u32 = 0x82647718;
    'dispatch: loop {
        match pc {
            0x82647718 => {
    //   block [0x82647718..0x8264778C)
	// 82647718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264771C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647724: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82647728: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264772C: 392B8A38  addi r9, r11, -0x75c8
	ctx.r[9].s64 = ctx.r[11].s64 + -30152;
	// 82647730: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647734: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647738: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8264773C: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82647740: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647744: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82647748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264774C: 396B0DF0  addi r11, r11, 0xdf0
	ctx.r[11].s64 = ctx.r[11].s64 + 3568;
	// 82647750: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82647754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647758: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264775C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647760: 386A0624  addi r3, r10, 0x624
	ctx.r[3].s64 = ctx.r[10].s64 + 1572;
	// 82647764: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82647768: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264776C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647770: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82647774: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82647778: 4BE1F6A9  bl 0x82466e20
	ctx.lr = 0x8264777C;
	sub_82466E20(ctx, base);
	// 8264777C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647790 size=116
    let mut pc: u32 = 0x82647790;
    'dispatch: loop {
        match pc {
            0x82647790 => {
    //   block [0x82647790..0x82647804)
	// 82647790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264779C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826477A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826477A4: 390BF470  addi r8, r11, -0xb90
	ctx.r[8].s64 = ctx.r[11].s64 + -2960;
	// 826477A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826477AC: 392A894C  addi r9, r10, -0x76b4
	ctx.r[9].s64 = ctx.r[10].s64 + -30388;
	// 826477B0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826477B4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826477B8: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826477BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826477C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826477C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826477C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826477CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826477D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826477D4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826477D8: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826477DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826477E0: 386B0654  addi r3, r11, 0x654
	ctx.r[3].s64 = ctx.r[11].s64 + 1620;
	// 826477E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826477E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826477EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826477F0: 4BE1F631  bl 0x82466e20
	ctx.lr = 0x826477F4;
	sub_82466E20(ctx, base);
	// 826477F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826477F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826477FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647808 size=112
    let mut pc: u32 = 0x82647808;
    'dispatch: loop {
        match pc {
            0x82647808 => {
    //   block [0x82647808..0x82647878)
	// 82647808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264780C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647818: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264781C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647824: 390BF4E8  addi r8, r11, -0xb18
	ctx.r[8].s64 = ctx.r[11].s64 + -2840;
	// 82647828: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264782C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82647830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264783C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647840: 386A0684  addi r3, r10, 0x684
	ctx.r[3].s64 = ctx.r[10].s64 + 1668;
	// 82647844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264784C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264785C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647864: 4BE1F5BD  bl 0x82466e20
	ctx.lr = 0x82647868;
	sub_82466E20(ctx, base);
	// 82647868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264786C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647878 size=112
    let mut pc: u32 = 0x82647878;
    'dispatch: loop {
        match pc {
            0x82647878 => {
    //   block [0x82647878..0x826478E8)
	// 82647878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264787C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647888: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264788C: 38AAF0F4  addi r5, r10, -0xf0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3852;
	// 82647890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647894: 390BF500  addi r8, r11, -0xb00
	ctx.r[8].s64 = ctx.r[11].s64 + -2816;
	// 82647898: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264789C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826478A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826478A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826478A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826478AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826478B0: 386A06B4  addi r3, r10, 0x6b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1716;
	// 826478B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826478B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826478BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826478C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826478C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826478C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826478CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826478D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826478D4: 4BE1F54D  bl 0x82466e20
	ctx.lr = 0x826478D8;
	sub_82466E20(ctx, base);
	// 826478D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826478DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826478E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826478E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826478E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826478E8 size=108
    let mut pc: u32 = 0x826478E8;
    'dispatch: loop {
        match pc {
            0x826478E8 => {
    //   block [0x826478E8..0x82647954)
	// 826478E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826478EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826478F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826478F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826478F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826478FC: 38EBF518  addi r7, r11, -0xae8
	ctx.r[7].s64 = ctx.r[11].s64 + -2792;
	// 82647900: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82647904: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82647908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264790C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647918: 386A06E4  addi r3, r10, 0x6e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1764;
	// 8264791C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264792C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264793C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647940: 4BE1F4E1  bl 0x82466e20
	ctx.lr = 0x82647944;
	sub_82466E20(ctx, base);
	// 82647944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264794C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647958 size=112
    let mut pc: u32 = 0x82647958;
    'dispatch: loop {
        match pc {
            0x82647958 => {
    //   block [0x82647958..0x826479C8)
	// 82647958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647968: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264796C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647974: 390BF530  addi r8, r11, -0xad0
	ctx.r[8].s64 = ctx.r[11].s64 + -2768;
	// 82647978: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264797C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82647980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264798C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647990: 386A0714  addi r3, r10, 0x714
	ctx.r[3].s64 = ctx.r[10].s64 + 1812;
	// 82647994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264799C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826479A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826479A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826479A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826479AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826479B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826479B4: 4BE1F46D  bl 0x82466e20
	ctx.lr = 0x826479B8;
	sub_82466E20(ctx, base);
	// 826479B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826479BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826479C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826479C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826479C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826479C8 size=108
    let mut pc: u32 = 0x826479C8;
    'dispatch: loop {
        match pc {
            0x826479C8 => {
    //   block [0x826479C8..0x82647A34)
	// 826479C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826479CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826479D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826479D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826479D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826479DC: 38EBF578  addi r7, r11, -0xa88
	ctx.r[7].s64 = ctx.r[11].s64 + -2696;
	// 826479E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826479E4: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826479E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826479EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826479F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826479F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826479F8: 386A0744  addi r3, r10, 0x744
	ctx.r[3].s64 = ctx.r[10].s64 + 1860;
	// 826479FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647A20: 4BE1F401  bl 0x82466e20
	ctx.lr = 0x82647A24;
	sub_82466E20(ctx, base);
	// 82647A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647A38 size=112
    let mut pc: u32 = 0x82647A38;
    'dispatch: loop {
        match pc {
            0x82647A38 => {
    //   block [0x82647A38..0x82647AA8)
	// 82647A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647A48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647A4C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647A54: 390BF590  addi r8, r11, -0xa70
	ctx.r[8].s64 = ctx.r[11].s64 + -2672;
	// 82647A58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82647A5C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82647A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647A64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647A70: 386A0774  addi r3, r10, 0x774
	ctx.r[3].s64 = ctx.r[10].s64 + 1908;
	// 82647A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647A94: 4BE1F38D  bl 0x82466e20
	ctx.lr = 0x82647A98;
	sub_82466E20(ctx, base);
	// 82647A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647AA8 size=112
    let mut pc: u32 = 0x82647AA8;
    'dispatch: loop {
        match pc {
            0x82647AA8 => {
    //   block [0x82647AA8..0x82647B18)
	// 82647AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647AB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647ABC: 38AA0834  addi r5, r10, 0x834
	ctx.r[5].s64 = ctx.r[10].s64 + 2100;
	// 82647AC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647AC4: 390BF5C0  addi r8, r11, -0xa40
	ctx.r[8].s64 = ctx.r[11].s64 + -2624;
	// 82647AC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82647ACC: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82647AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647AD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647AE0: 386A07A4  addi r3, r10, 0x7a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1956;
	// 82647AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647B04: 4BE1F31D  bl 0x82466e20
	ctx.lr = 0x82647B08;
	sub_82466E20(ctx, base);
	// 82647B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647B18 size=108
    let mut pc: u32 = 0x82647B18;
    'dispatch: loop {
        match pc {
            0x82647B18 => {
    //   block [0x82647B18..0x82647B84)
	// 82647B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647B24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647B2C: 38EBF638  addi r7, r11, -0x9c8
	ctx.r[7].s64 = ctx.r[11].s64 + -2504;
	// 82647B30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82647B34: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82647B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647B40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647B48: 386A07D4  addi r3, r10, 0x7d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2004;
	// 82647B4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647B70: 4BE1F2B1  bl 0x82466e20
	ctx.lr = 0x82647B74;
	sub_82466E20(ctx, base);
	// 82647B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647B88 size=108
    let mut pc: u32 = 0x82647B88;
    'dispatch: loop {
        match pc {
            0x82647B88 => {
    //   block [0x82647B88..0x82647BF4)
	// 82647B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647B94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647B9C: 38EBF680  addi r7, r11, -0x980
	ctx.r[7].s64 = ctx.r[11].s64 + -2432;
	// 82647BA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82647BA4: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82647BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647BB8: 386A0804  addi r3, r10, 0x804
	ctx.r[3].s64 = ctx.r[10].s64 + 2052;
	// 82647BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647BE0: 4BE1F241  bl 0x82466e20
	ctx.lr = 0x82647BE4;
	sub_82466E20(ctx, base);
	// 82647BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647BF8 size=112
    let mut pc: u32 = 0x82647BF8;
    'dispatch: loop {
        match pc {
            0x82647BF8 => {
    //   block [0x82647BF8..0x82647C68)
	// 82647BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647C0C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82647C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647C14: 390BF6C8  addi r8, r11, -0x938
	ctx.r[8].s64 = ctx.r[11].s64 + -2360;
	// 82647C18: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82647C1C: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82647C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647C30: 386A0834  addi r3, r10, 0x834
	ctx.r[3].s64 = ctx.r[10].s64 + 2100;
	// 82647C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647C54: 4BE1F1CD  bl 0x82466e20
	ctx.lr = 0x82647C58;
	sub_82466E20(ctx, base);
	// 82647C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647C68 size=112
    let mut pc: u32 = 0x82647C68;
    'dispatch: loop {
        match pc {
            0x82647C68 => {
    //   block [0x82647C68..0x82647CD8)
	// 82647C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647C7C: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647C84: 390BF7A0  addi r8, r11, -0x860
	ctx.r[8].s64 = ctx.r[11].s64 + -2144;
	// 82647C88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82647C8C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82647C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647CA0: 386A0864  addi r3, r10, 0x864
	ctx.r[3].s64 = ctx.r[10].s64 + 2148;
	// 82647CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647CC4: 4BE1F15D  bl 0x82466e20
	ctx.lr = 0x82647CC8;
	sub_82466E20(ctx, base);
	// 82647CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647CD8 size=112
    let mut pc: u32 = 0x82647CD8;
    'dispatch: loop {
        match pc {
            0x82647CD8 => {
    //   block [0x82647CD8..0x82647D48)
	// 82647CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647CE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647CEC: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647CF4: 390BF7E8  addi r8, r11, -0x818
	ctx.r[8].s64 = ctx.r[11].s64 + -2072;
	// 82647CF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647CFC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82647D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647D04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647D10: 386A0894  addi r3, r10, 0x894
	ctx.r[3].s64 = ctx.r[10].s64 + 2196;
	// 82647D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647D34: 4BE1F0ED  bl 0x82466e20
	ctx.lr = 0x82647D38;
	sub_82466E20(ctx, base);
	// 82647D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647D48 size=112
    let mut pc: u32 = 0x82647D48;
    'dispatch: loop {
        match pc {
            0x82647D48 => {
    //   block [0x82647D48..0x82647DB8)
	// 82647D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647D5C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82647D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647D64: 390BF848  addi r8, r11, -0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + -1976;
	// 82647D68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647D6C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82647D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647D74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647D80: 386A08C4  addi r3, r10, 0x8c4
	ctx.r[3].s64 = ctx.r[10].s64 + 2244;
	// 82647D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647DA4: 4BE1F07D  bl 0x82466e20
	ctx.lr = 0x82647DA8;
	sub_82466E20(ctx, base);
	// 82647DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647DB8 size=112
    let mut pc: u32 = 0x82647DB8;
    'dispatch: loop {
        match pc {
            0x82647DB8 => {
    //   block [0x82647DB8..0x82647E28)
	// 82647DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647DC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647DCC: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82647DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647DD4: 390BF8A8  addi r8, r11, -0x758
	ctx.r[8].s64 = ctx.r[11].s64 + -1880;
	// 82647DD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647DDC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82647DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647DF0: 386A08F4  addi r3, r10, 0x8f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2292;
	// 82647DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647E14: 4BE1F00D  bl 0x82466e20
	ctx.lr = 0x82647E18;
	sub_82466E20(ctx, base);
	// 82647E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647E28 size=112
    let mut pc: u32 = 0x82647E28;
    'dispatch: loop {
        match pc {
            0x82647E28 => {
    //   block [0x82647E28..0x82647E98)
	// 82647E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647E34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647E38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647E3C: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647E44: 390BF908  addi r8, r11, -0x6f8
	ctx.r[8].s64 = ctx.r[11].s64 + -1784;
	// 82647E48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82647E4C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82647E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647E60: 386A0924  addi r3, r10, 0x924
	ctx.r[3].s64 = ctx.r[10].s64 + 2340;
	// 82647E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647E84: 4BE1EF9D  bl 0x82466e20
	ctx.lr = 0x82647E88;
	sub_82466E20(ctx, base);
	// 82647E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647E98 size=112
    let mut pc: u32 = 0x82647E98;
    'dispatch: loop {
        match pc {
            0x82647E98 => {
    //   block [0x82647E98..0x82647F08)
	// 82647E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647EA4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82647EA8: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82647EAC: 38EAF9C8  addi r7, r10, -0x638
	ctx.r[7].s64 = ctx.r[10].s64 + -1592;
	// 82647EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647EB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82647EB8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82647EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647EC0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647EC4: 396B8B10  addi r11, r11, -0x74f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29936;
	// 82647EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647ED4: 386A0954  addi r3, r10, 0x954
	ctx.r[3].s64 = ctx.r[10].s64 + 2388;
	// 82647ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647EDC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82647EE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647EE4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82647EE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647EEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647EF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647EF4: 4BE1EF2D  bl 0x82466e20
	ctx.lr = 0x82647EF8;
	sub_82466E20(ctx, base);
	// 82647EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647F08 size=112
    let mut pc: u32 = 0x82647F08;
    'dispatch: loop {
        match pc {
            0x82647F08 => {
    //   block [0x82647F08..0x82647F78)
	// 82647F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647F1C: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82647F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647F24: 390BFB60  addi r8, r11, -0x4a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1184;
	// 82647F28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82647F2C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82647F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647F40: 386A0984  addi r3, r10, 0x984
	ctx.r[3].s64 = ctx.r[10].s64 + 2436;
	// 82647F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647F64: 4BE1EEBD  bl 0x82466e20
	ctx.lr = 0x82647F68;
	sub_82466E20(ctx, base);
	// 82647F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647F78 size=112
    let mut pc: u32 = 0x82647F78;
    'dispatch: loop {
        match pc {
            0x82647F78 => {
    //   block [0x82647F78..0x82647FE8)
	// 82647F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647F8C: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82647F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647F94: 390BFB78  addi r8, r11, -0x488
	ctx.r[8].s64 = ctx.r[11].s64 + -1160;
	// 82647F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82647F9C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82647FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647FB0: 386A09B4  addi r3, r10, 0x9b4
	ctx.r[3].s64 = ctx.r[10].s64 + 2484;
	// 82647FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647FC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82647FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647FD4: 4BE1EE4D  bl 0x82466e20
	ctx.lr = 0x82647FD8;
	sub_82466E20(ctx, base);
	// 82647FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647FE8 size=112
    let mut pc: u32 = 0x82647FE8;
    'dispatch: loop {
        match pc {
            0x82647FE8 => {
    //   block [0x82647FE8..0x82648058)
	// 82647FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647FF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647FFC: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82648000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648004: 390BFB90  addi r8, r11, -0x470
	ctx.r[8].s64 = ctx.r[11].s64 + -1136;
	// 82648008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264800C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82648010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648020: 386A09E4  addi r3, r10, 0x9e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2532;
	// 82648024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264802C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264803C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648044: 4BE1EDDD  bl 0x82466e20
	ctx.lr = 0x82648048;
	sub_82466E20(ctx, base);
	// 82648048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648058 size=108
    let mut pc: u32 = 0x82648058;
    'dispatch: loop {
        match pc {
            0x82648058 => {
    //   block [0x82648058..0x826480C4)
	// 82648058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648064: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264806C: 38EBFBC0  addi r7, r11, -0x440
	ctx.r[7].s64 = ctx.r[11].s64 + -1088;
	// 82648070: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648074: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82648078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264807C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648088: 386A0A14  addi r3, r10, 0xa14
	ctx.r[3].s64 = ctx.r[10].s64 + 2580;
	// 8264808C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264809C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826480A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826480A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826480A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826480AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826480B0: 4BE1ED71  bl 0x82466e20
	ctx.lr = 0x826480B4;
	sub_82466E20(ctx, base);
	// 826480B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826480B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826480BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826480C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826480C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826480C8 size=112
    let mut pc: u32 = 0x826480C8;
    'dispatch: loop {
        match pc {
            0x826480C8 => {
    //   block [0x826480C8..0x82648138)
	// 826480C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826480CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826480D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826480D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826480D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826480DC: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 826480E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826480E4: 390BFBF0  addi r8, r11, -0x410
	ctx.r[8].s64 = ctx.r[11].s64 + -1040;
	// 826480E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826480EC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826480F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826480F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826480F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826480FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648100: 386A0A44  addi r3, r10, 0xa44
	ctx.r[3].s64 = ctx.r[10].s64 + 2628;
	// 82648104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82648118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264811C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648124: 4BE1ECFD  bl 0x82466e20
	ctx.lr = 0x82648128;
	sub_82466E20(ctx, base);
	// 82648128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264812C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648138 size=112
    let mut pc: u32 = 0x82648138;
    'dispatch: loop {
        match pc {
            0x82648138 => {
    //   block [0x82648138..0x826481A8)
	// 82648138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264813C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648148: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264814C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82648150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648154: 390BFC08  addi r8, r11, -0x3f8
	ctx.r[8].s64 = ctx.r[11].s64 + -1016;
	// 82648158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264815C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82648160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264816C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648170: 386A0A74  addi r3, r10, 0xa74
	ctx.r[3].s64 = ctx.r[10].s64 + 2676;
	// 82648174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264817C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264818C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648194: 4BE1EC8D  bl 0x82466e20
	ctx.lr = 0x82648198;
	sub_82466E20(ctx, base);
	// 82648198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264819C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826481A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826481A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826481A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826481A8 size=112
    let mut pc: u32 = 0x826481A8;
    'dispatch: loop {
        match pc {
            0x826481A8 => {
    //   block [0x826481A8..0x82648218)
	// 826481A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826481AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826481B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826481B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826481B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826481BC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826481C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826481C4: 390BFC98  addi r8, r11, -0x368
	ctx.r[8].s64 = ctx.r[11].s64 + -872;
	// 826481C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826481CC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826481D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826481D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826481D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826481DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826481E0: 386A0AA4  addi r3, r10, 0xaa4
	ctx.r[3].s64 = ctx.r[10].s64 + 2724;
	// 826481E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826481E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826481EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826481F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826481F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826481F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826481FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648204: 4BE1EC1D  bl 0x82466e20
	ctx.lr = 0x82648208;
	sub_82466E20(ctx, base);
	// 82648208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264820C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648218 size=112
    let mut pc: u32 = 0x82648218;
    'dispatch: loop {
        match pc {
            0x82648218 => {
    //   block [0x82648218..0x82648288)
	// 82648218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264821C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648228: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264822C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648234: 390BFCC8  addi r8, r11, -0x338
	ctx.r[8].s64 = ctx.r[11].s64 + -824;
	// 82648238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264823C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82648240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264824C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648250: 386A0AD4  addi r3, r10, 0xad4
	ctx.r[3].s64 = ctx.r[10].s64 + 2772;
	// 82648254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264825C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264826C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648274: 4BE1EBAD  bl 0x82466e20
	ctx.lr = 0x82648278;
	sub_82466E20(ctx, base);
	// 82648278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264827C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648288 size=112
    let mut pc: u32 = 0x82648288;
    'dispatch: loop {
        match pc {
            0x82648288 => {
    //   block [0x82648288..0x826482F8)
	// 82648288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264828C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648294: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648298: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264829C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826482A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826482A4: 390BFCE0  addi r8, r11, -0x320
	ctx.r[8].s64 = ctx.r[11].s64 + -800;
	// 826482A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826482AC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826482B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826482B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826482B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826482BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826482C0: 386A0B04  addi r3, r10, 0xb04
	ctx.r[3].s64 = ctx.r[10].s64 + 2820;
	// 826482C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826482C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826482CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826482D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826482D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826482D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826482DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826482E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826482E4: 4BE1EB3D  bl 0x82466e20
	ctx.lr = 0x826482E8;
	sub_82466E20(ctx, base);
	// 826482E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826482EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826482F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826482F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826482F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826482F8 size=108
    let mut pc: u32 = 0x826482F8;
    'dispatch: loop {
        match pc {
            0x826482F8 => {
    //   block [0x826482F8..0x82648364)
	// 826482F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826482FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648304: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264830C: 38EBFCF8  addi r7, r11, -0x308
	ctx.r[7].s64 = ctx.r[11].s64 + -776;
	// 82648310: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648314: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82648318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264831C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648328: 386A0B34  addi r3, r10, 0xb34
	ctx.r[3].s64 = ctx.r[10].s64 + 2868;
	// 8264832C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264833C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264834C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648350: 4BE1EAD1  bl 0x82466e20
	ctx.lr = 0x82648354;
	sub_82466E20(ctx, base);
	// 82648354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264835C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648368 size=112
    let mut pc: u32 = 0x82648368;
    'dispatch: loop {
        match pc {
            0x82648368 => {
    //   block [0x82648368..0x826483D8)
	// 82648368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648378: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264837C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648384: 390BFD28  addi r8, r11, -0x2d8
	ctx.r[8].s64 = ctx.r[11].s64 + -728;
	// 82648388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264838C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82648390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264839C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826483A0: 386A0B64  addi r3, r10, 0xb64
	ctx.r[3].s64 = ctx.r[10].s64 + 2916;
	// 826483A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826483A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826483AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826483B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826483B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826483B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826483BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826483C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826483C4: 4BE1EA5D  bl 0x82466e20
	ctx.lr = 0x826483C8;
	sub_82466E20(ctx, base);
	// 826483C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826483CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826483D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826483D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826483D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826483D8 size=108
    let mut pc: u32 = 0x826483D8;
    'dispatch: loop {
        match pc {
            0x826483D8 => {
    //   block [0x826483D8..0x82648444)
	// 826483D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826483DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826483E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826483E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826483E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826483EC: 38EBFD40  addi r7, r11, -0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + -704;
	// 826483F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826483F4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826483F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826483FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648408: 386A0B94  addi r3, r10, 0xb94
	ctx.r[3].s64 = ctx.r[10].s64 + 2964;
	// 8264840C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264841C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264842C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648430: 4BE1E9F1  bl 0x82466e20
	ctx.lr = 0x82648434;
	sub_82466E20(ctx, base);
	// 82648434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264843C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648448 size=112
    let mut pc: u32 = 0x82648448;
    'dispatch: loop {
        match pc {
            0x82648448 => {
    //   block [0x82648448..0x826484B8)
	// 82648448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264845C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648464: 390BFE18  addi r8, r11, -0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + -488;
	// 82648468: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8264846C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82648470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264847C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648480: 386A0BC4  addi r3, r10, 0xbc4
	ctx.r[3].s64 = ctx.r[10].s64 + 3012;
	// 82648484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264848C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264849C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826484A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826484A4: 4BE1E97D  bl 0x82466e20
	ctx.lr = 0x826484A8;
	sub_82466E20(ctx, base);
	// 826484A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826484AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826484B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826484B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826484B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826484B8 size=108
    let mut pc: u32 = 0x826484B8;
    'dispatch: loop {
        match pc {
            0x826484B8 => {
    //   block [0x826484B8..0x82648524)
	// 826484B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826484BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826484C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826484C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826484C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826484CC: 38EBFFC8  addi r7, r11, -0x38
	ctx.r[7].s64 = ctx.r[11].s64 + -56;
	// 826484D0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826484D4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826484D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826484DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826484E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826484E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826484E8: 386A0BF4  addi r3, r10, 0xbf4
	ctx.r[3].s64 = ctx.r[10].s64 + 3060;
	// 826484EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826484F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826484F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826484F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826484FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264850C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648510: 4BE1E911  bl 0x82466e20
	ctx.lr = 0x82648514;
	sub_82466E20(ctx, base);
	// 82648514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264851C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648528 size=112
    let mut pc: u32 = 0x82648528;
    'dispatch: loop {
        match pc {
            0x82648528 => {
    //   block [0x82648528..0x82648598)
	// 82648528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264852C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648538: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264853C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82648540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648544: 390B0160  addi r8, r11, 0x160
	ctx.r[8].s64 = ctx.r[11].s64 + 352;
	// 82648548: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8264854C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82648550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264855C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648560: 386A0C24  addi r3, r10, 0xc24
	ctx.r[3].s64 = ctx.r[10].s64 + 3108;
	// 82648564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264856C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264857C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648584: 4BE1E89D  bl 0x82466e20
	ctx.lr = 0x82648588;
	sub_82466E20(ctx, base);
	// 82648588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264858C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648598 size=100
    let mut pc: u32 = 0x82648598;
    'dispatch: loop {
        match pc {
            0x82648598 => {
    //   block [0x82648598..0x826485FC)
	// 82648598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826485A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826485A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826485A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826485AC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826485B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826485B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826485B8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826485BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826485C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826485C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826485C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826485CC: 386A0C54  addi r3, r10, 0xc54
	ctx.r[3].s64 = ctx.r[10].s64 + 3156;
	// 826485D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826485D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826485D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826485DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826485E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826485E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826485E8: 4BE1E839  bl 0x82466e20
	ctx.lr = 0x826485EC;
	sub_82466E20(ctx, base);
	// 826485EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826485F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826485F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826485F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648600 size=112
    let mut pc: u32 = 0x82648600;
    'dispatch: loop {
        match pc {
            0x82648600 => {
    //   block [0x82648600..0x82648670)
	// 82648600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264860C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648610: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648614: 38AA0C54  addi r5, r10, 0xc54
	ctx.r[5].s64 = ctx.r[10].s64 + 3156;
	// 82648618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264861C: 390B03D0  addi r8, r11, 0x3d0
	ctx.r[8].s64 = ctx.r[11].s64 + 976;
	// 82648620: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82648624: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82648628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264862C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648638: 386A0C84  addi r3, r10, 0xc84
	ctx.r[3].s64 = ctx.r[10].s64 + 3204;
	// 8264863C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264864C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264865C: 4BE1E7C5  bl 0x82466e20
	ctx.lr = 0x82648660;
	sub_82466E20(ctx, base);
	// 82648660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264866C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648670 size=100
    let mut pc: u32 = 0x82648670;
    'dispatch: loop {
        match pc {
            0x82648670 => {
    //   block [0x82648670..0x826486D4)
	// 82648670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264867C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648684: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264868C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648690: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82648694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264869C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826486A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826486A4: 386A0CB4  addi r3, r10, 0xcb4
	ctx.r[3].s64 = ctx.r[10].s64 + 3252;
	// 826486A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826486AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826486B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826486B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826486B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826486BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826486C0: 4BE1E761  bl 0x82466e20
	ctx.lr = 0x826486C4;
	sub_82466E20(ctx, base);
	// 826486C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826486C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826486CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826486D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826486D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826486D8 size=108
    let mut pc: u32 = 0x826486D8;
    'dispatch: loop {
        match pc {
            0x826486D8 => {
    //   block [0x826486D8..0x82648744)
	// 826486D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826486DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826486E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826486E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826486E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826486EC: 38EB0448  addi r7, r11, 0x448
	ctx.r[7].s64 = ctx.r[11].s64 + 1096;
	// 826486F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826486F4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826486F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826486FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648708: 386A0CE4  addi r3, r10, 0xce4
	ctx.r[3].s64 = ctx.r[10].s64 + 3300;
	// 8264870C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264872C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648730: 4BE1E6F1  bl 0x82466e20
	ctx.lr = 0x82648734;
	sub_82466E20(ctx, base);
	// 82648734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264873C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648748 size=112
    let mut pc: u32 = 0x82648748;
    'dispatch: loop {
        match pc {
            0x82648748 => {
    //   block [0x82648748..0x826487B8)
	// 82648748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264874C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648758: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264875C: 38AA0CB4  addi r5, r10, 0xcb4
	ctx.r[5].s64 = ctx.r[10].s64 + 3252;
	// 82648760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648764: 390B0490  addi r8, r11, 0x490
	ctx.r[8].s64 = ctx.r[11].s64 + 1168;
	// 82648768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264876C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82648770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648774: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264877C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648780: 386A0D14  addi r3, r10, 0xd14
	ctx.r[3].s64 = ctx.r[10].s64 + 3348;
	// 82648784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264878C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264879C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826487A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826487A4: 4BE1E67D  bl 0x82466e20
	ctx.lr = 0x826487A8;
	sub_82466E20(ctx, base);
	// 826487A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826487AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826487B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826487B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826487B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826487B8 size=100
    let mut pc: u32 = 0x826487B8;
    'dispatch: loop {
        match pc {
            0x826487B8 => {
    //   block [0x826487B8..0x8264881C)
	// 826487B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826487BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826487C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826487C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826487C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826487CC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826487D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826487D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826487D8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826487DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826487E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826487E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826487E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826487EC: 386A0D44  addi r3, r10, 0xd44
	ctx.r[3].s64 = ctx.r[10].s64 + 3396;
	// 826487F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826487F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826487F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826487FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648808: 4BE1E619  bl 0x82466e20
	ctx.lr = 0x8264880C;
	sub_82466E20(ctx, base);
	// 8264880C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648820 size=100
    let mut pc: u32 = 0x82648820;
    'dispatch: loop {
        match pc {
            0x82648820 => {
    //   block [0x82648820..0x82648884)
	// 82648820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264882C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648834: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264883C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648840: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82648844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648854: 386A0D74  addi r3, r10, 0xd74
	ctx.r[3].s64 = ctx.r[10].s64 + 3444;
	// 82648858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264885C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648860: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648868: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264886C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648870: 4BE1E5B1  bl 0x82466e20
	ctx.lr = 0x82648874;
	sub_82466E20(ctx, base);
	// 82648874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264887C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648888 size=112
    let mut pc: u32 = 0x82648888;
    'dispatch: loop {
        match pc {
            0x82648888 => {
    //   block [0x82648888..0x826488F8)
	// 82648888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648898: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264889C: 38AA0D44  addi r5, r10, 0xd44
	ctx.r[5].s64 = ctx.r[10].s64 + 3396;
	// 826488A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826488A4: 390B04C0  addi r8, r11, 0x4c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1216;
	// 826488A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826488AC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826488B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826488B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826488B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826488BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826488C0: 386A0DA4  addi r3, r10, 0xda4
	ctx.r[3].s64 = ctx.r[10].s64 + 3492;
	// 826488C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826488C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826488CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826488D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826488D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826488D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826488DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826488E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826488E4: 4BE1E53D  bl 0x82466e20
	ctx.lr = 0x826488E8;
	sub_82466E20(ctx, base);
	// 826488E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826488EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826488F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826488F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826488F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826488F8 size=112
    let mut pc: u32 = 0x826488F8;
    'dispatch: loop {
        match pc {
            0x826488F8 => {
    //   block [0x826488F8..0x82648968)
	// 826488F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826488FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648908: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264890C: 38AA0D74  addi r5, r10, 0xd74
	ctx.r[5].s64 = ctx.r[10].s64 + 3444;
	// 82648910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648914: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 82648918: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264891C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82648920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264892C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648930: 386A0DD4  addi r3, r10, 0xdd4
	ctx.r[3].s64 = ctx.r[10].s64 + 3540;
	// 82648934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264893C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264894C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648954: 4BE1E4CD  bl 0x82466e20
	ctx.lr = 0x82648958;
	sub_82466E20(ctx, base);
	// 82648958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264895C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648968 size=100
    let mut pc: u32 = 0x82648968;
    'dispatch: loop {
        match pc {
            0x82648968 => {
    //   block [0x82648968..0x826489CC)
	// 82648968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264897C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648988: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8264898C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264899C: 386A0E04  addi r3, r10, 0xe04
	ctx.r[3].s64 = ctx.r[10].s64 + 3588;
	// 826489A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826489A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826489A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826489AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826489B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826489B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826489B8: 4BE1E469  bl 0x82466e20
	ctx.lr = 0x826489BC;
	sub_82466E20(ctx, base);
	// 826489BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826489C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826489C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826489C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826489D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826489D0 size=112
    let mut pc: u32 = 0x826489D0;
    'dispatch: loop {
        match pc {
            0x826489D0 => {
    //   block [0x826489D0..0x82648A40)
	// 826489D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826489D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826489D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826489DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826489E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826489E4: 38AA0E04  addi r5, r10, 0xe04
	ctx.r[5].s64 = ctx.r[10].s64 + 3588;
	// 826489E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826489EC: 390B0580  addi r8, r11, 0x580
	ctx.r[8].s64 = ctx.r[11].s64 + 1408;
	// 826489F0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826489F4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826489F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826489FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648A08: 386A0E34  addi r3, r10, 0xe34
	ctx.r[3].s64 = ctx.r[10].s64 + 3636;
	// 82648A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648A2C: 4BE1E3F5  bl 0x82466e20
	ctx.lr = 0x82648A30;
	sub_82466E20(ctx, base);
	// 82648A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648A40 size=100
    let mut pc: u32 = 0x82648A40;
    'dispatch: loop {
        match pc {
            0x82648A40 => {
    //   block [0x82648A40..0x82648AA4)
	// 82648A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648A4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648A54: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648A60: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82648A64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648A74: 386A0E64  addi r3, r10, 0xe64
	ctx.r[3].s64 = ctx.r[10].s64 + 3684;
	// 82648A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648A80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648A88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648A90: 4BE1E391  bl 0x82466e20
	ctx.lr = 0x82648A94;
	sub_82466E20(ctx, base);
	// 82648A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648AA8 size=112
    let mut pc: u32 = 0x82648AA8;
    'dispatch: loop {
        match pc {
            0x82648AA8 => {
    //   block [0x82648AA8..0x82648B18)
	// 82648AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648AB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648ABC: 38AA0E64  addi r5, r10, 0xe64
	ctx.r[5].s64 = ctx.r[10].s64 + 3684;
	// 82648AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648AC4: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 82648AC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648ACC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82648AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648AD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648AE0: 386A0E94  addi r3, r10, 0xe94
	ctx.r[3].s64 = ctx.r[10].s64 + 3732;
	// 82648AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648B04: 4BE1E31D  bl 0x82466e20
	ctx.lr = 0x82648B08;
	sub_82466E20(ctx, base);
	// 82648B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648B18 size=108
    let mut pc: u32 = 0x82648B18;
    'dispatch: loop {
        match pc {
            0x82648B18 => {
    //   block [0x82648B18..0x82648B84)
	// 82648B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648B24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648B2C: 38EB06B8  addi r7, r11, 0x6b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1720;
	// 82648B30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82648B34: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82648B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648B40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648B48: 386A0EC4  addi r3, r10, 0xec4
	ctx.r[3].s64 = ctx.r[10].s64 + 3780;
	// 82648B4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648B70: 4BE1E2B1  bl 0x82466e20
	ctx.lr = 0x82648B74;
	sub_82466E20(ctx, base);
	// 82648B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648B88 size=112
    let mut pc: u32 = 0x82648B88;
    'dispatch: loop {
        match pc {
            0x82648B88 => {
    //   block [0x82648B88..0x82648BF8)
	// 82648B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648B98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648B9C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648BA4: 390B0700  addi r8, r11, 0x700
	ctx.r[8].s64 = ctx.r[11].s64 + 1792;
	// 82648BA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648BAC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82648BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648BB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648BC0: 386A0EF4  addi r3, r10, 0xef4
	ctx.r[3].s64 = ctx.r[10].s64 + 3828;
	// 82648BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648BE4: 4BE1E23D  bl 0x82466e20
	ctx.lr = 0x82648BE8;
	sub_82466E20(ctx, base);
	// 82648BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648BF8 size=108
    let mut pc: u32 = 0x82648BF8;
    'dispatch: loop {
        match pc {
            0x82648BF8 => {
    //   block [0x82648BF8..0x82648C64)
	// 82648BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648C04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648C0C: 38EB0718  addi r7, r11, 0x718
	ctx.r[7].s64 = ctx.r[11].s64 + 1816;
	// 82648C10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82648C14: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82648C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648C1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648C28: 386A0F24  addi r3, r10, 0xf24
	ctx.r[3].s64 = ctx.r[10].s64 + 3876;
	// 82648C2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648C50: 4BE1E1D1  bl 0x82466e20
	ctx.lr = 0x82648C54;
	sub_82466E20(ctx, base);
	// 82648C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648C68 size=112
    let mut pc: u32 = 0x82648C68;
    'dispatch: loop {
        match pc {
            0x82648C68 => {
    //   block [0x82648C68..0x82648CD8)
	// 82648C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648C7C: 38AA0EF4  addi r5, r10, 0xef4
	ctx.r[5].s64 = ctx.r[10].s64 + 3828;
	// 82648C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648C84: 390B0760  addi r8, r11, 0x760
	ctx.r[8].s64 = ctx.r[11].s64 + 1888;
	// 82648C88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648C8C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82648C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648CA0: 386A0F54  addi r3, r10, 0xf54
	ctx.r[3].s64 = ctx.r[10].s64 + 3924;
	// 82648CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648CC4: 4BE1E15D  bl 0x82466e20
	ctx.lr = 0x82648CC8;
	sub_82466E20(ctx, base);
	// 82648CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648CD8 size=100
    let mut pc: u32 = 0x82648CD8;
    'dispatch: loop {
        match pc {
            0x82648CD8 => {
    //   block [0x82648CD8..0x82648D3C)
	// 82648CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648CEC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648CF8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82648CFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648D0C: 386A0F84  addi r3, r10, 0xf84
	ctx.r[3].s64 = ctx.r[10].s64 + 3972;
	// 82648D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648D28: 4BE1E0F9  bl 0x82466e20
	ctx.lr = 0x82648D2C;
	sub_82466E20(ctx, base);
	// 82648D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648D40 size=112
    let mut pc: u32 = 0x82648D40;
    'dispatch: loop {
        match pc {
            0x82648D40 => {
    //   block [0x82648D40..0x82648DB0)
	// 82648D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648D4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648D54: 38AA0F84  addi r5, r10, 0xf84
	ctx.r[5].s64 = ctx.r[10].s64 + 3972;
	// 82648D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648D5C: 390B0778  addi r8, r11, 0x778
	ctx.r[8].s64 = ctx.r[11].s64 + 1912;
	// 82648D60: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82648D64: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82648D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648D78: 386A0FB4  addi r3, r10, 0xfb4
	ctx.r[3].s64 = ctx.r[10].s64 + 4020;
	// 82648D7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648D80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648D9C: 4BE1E085  bl 0x82466e20
	ctx.lr = 0x82648DA0;
	sub_82466E20(ctx, base);
	// 82648DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648DB0 size=108
    let mut pc: u32 = 0x82648DB0;
    'dispatch: loop {
        match pc {
            0x82648DB0 => {
    //   block [0x82648DB0..0x82648E1C)
	// 82648DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648DBC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648DC4: 38EB0820  addi r7, r11, 0x820
	ctx.r[7].s64 = ctx.r[11].s64 + 2080;
	// 82648DC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648DCC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82648DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648DD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648DD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648DE0: 386A0FE4  addi r3, r10, 0xfe4
	ctx.r[3].s64 = ctx.r[10].s64 + 4068;
	// 82648DE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648E08: 4BE1E019  bl 0x82466e20
	ctx.lr = 0x82648E0C;
	sub_82466E20(ctx, base);
	// 82648E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648E20 size=112
    let mut pc: u32 = 0x82648E20;
    'dispatch: loop {
        match pc {
            0x82648E20 => {
    //   block [0x82648E20..0x82648E90)
	// 82648E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648E30: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648E34: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648E3C: 390B0850  addi r8, r11, 0x850
	ctx.r[8].s64 = ctx.r[11].s64 + 2128;
	// 82648E40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648E44: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82648E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648E4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648E58: 386A1014  addi r3, r10, 0x1014
	ctx.r[3].s64 = ctx.r[10].s64 + 4116;
	// 82648E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648E7C: 4BE1DFA5  bl 0x82466e20
	ctx.lr = 0x82648E80;
	sub_82466E20(ctx, base);
	// 82648E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648E90 size=112
    let mut pc: u32 = 0x82648E90;
    'dispatch: loop {
        match pc {
            0x82648E90 => {
    //   block [0x82648E90..0x82648F00)
	// 82648E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648E9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648EA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648EA4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648EAC: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 82648EB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648EB4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82648EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648EC8: 386A1044  addi r3, r10, 0x1044
	ctx.r[3].s64 = ctx.r[10].s64 + 4164;
	// 82648ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648EEC: 4BE1DF35  bl 0x82466e20
	ctx.lr = 0x82648EF0;
	sub_82466E20(ctx, base);
	// 82648EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648F00 size=100
    let mut pc: u32 = 0x82648F00;
    'dispatch: loop {
        match pc {
            0x82648F00 => {
    //   block [0x82648F00..0x82648F64)
	// 82648F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648F0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648F14: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648F20: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82648F24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648F34: 386A1074  addi r3, r10, 0x1074
	ctx.r[3].s64 = ctx.r[10].s64 + 4212;
	// 82648F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648F50: 4BE1DED1  bl 0x82466e20
	ctx.lr = 0x82648F54;
	sub_82466E20(ctx, base);
	// 82648F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648F68 size=112
    let mut pc: u32 = 0x82648F68;
    'dispatch: loop {
        match pc {
            0x82648F68 => {
    //   block [0x82648F68..0x82648FD8)
	// 82648F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648F74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648F7C: 38AA1074  addi r5, r10, 0x1074
	ctx.r[5].s64 = ctx.r[10].s64 + 4212;
	// 82648F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648F84: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 82648F88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648F8C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82648F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648F94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648FA0: 386A10A4  addi r3, r10, 0x10a4
	ctx.r[3].s64 = ctx.r[10].s64 + 4260;
	// 82648FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648FC4: 4BE1DE5D  bl 0x82466e20
	ctx.lr = 0x82648FC8;
	sub_82466E20(ctx, base);
	// 82648FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648FD8 size=112
    let mut pc: u32 = 0x82648FD8;
    'dispatch: loop {
        match pc {
            0x82648FD8 => {
    //   block [0x82648FD8..0x82649048)
	// 82648FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648FE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648FEC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648FF4: 390B0928  addi r8, r11, 0x928
	ctx.r[8].s64 = ctx.r[11].s64 + 2344;
	// 82648FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648FFC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82649000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649004: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649010: 386A10D4  addi r3, r10, 0x10d4
	ctx.r[3].s64 = ctx.r[10].s64 + 4308;
	// 82649014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264901C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264902C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649034: 4BE1DDED  bl 0x82466e20
	ctx.lr = 0x82649038;
	sub_82466E20(ctx, base);
	// 82649038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264903C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649048 size=112
    let mut pc: u32 = 0x82649048;
    'dispatch: loop {
        match pc {
            0x82649048 => {
    //   block [0x82649048..0x826490B8)
	// 82649048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264904C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649058: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264905C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82649060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649064: 390B0940  addi r8, r11, 0x940
	ctx.r[8].s64 = ctx.r[11].s64 + 2368;
	// 82649068: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264906C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82649070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649074: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264907C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649080: 386A1104  addi r3, r10, 0x1104
	ctx.r[3].s64 = ctx.r[10].s64 + 4356;
	// 82649084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264908C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649094: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82649098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264909C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826490A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826490A4: 4BE1DD7D  bl 0x82466e20
	ctx.lr = 0x826490A8;
	sub_82466E20(ctx, base);
	// 826490A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826490AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826490B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826490B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826490B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826490B8 size=112
    let mut pc: u32 = 0x826490B8;
    'dispatch: loop {
        match pc {
            0x826490B8 => {
    //   block [0x826490B8..0x82649128)
	// 826490B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826490BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826490C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826490C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826490C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826490CC: 38AA10D4  addi r5, r10, 0x10d4
	ctx.r[5].s64 = ctx.r[10].s64 + 4308;
	// 826490D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826490D4: 390B0958  addi r8, r11, 0x958
	ctx.r[8].s64 = ctx.r[11].s64 + 2392;
	// 826490D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826490DC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826490E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826490E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826490E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826490EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826490F0: 386A1134  addi r3, r10, 0x1134
	ctx.r[3].s64 = ctx.r[10].s64 + 4404;
	// 826490F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826490F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826490FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649114: 4BE1DD0D  bl 0x82466e20
	ctx.lr = 0x82649118;
	sub_82466E20(ctx, base);
	// 82649118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649128 size=72
    let mut pc: u32 = 0x82649128;
    'dispatch: loop {
        match pc {
            0x82649128 => {
    //   block [0x82649128..0x82649170)
	// 82649128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649134: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649138: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8264913C: 38CB8448  addi r6, r11, -0x7bb8
	ctx.r[6].s64 = ctx.r[11].s64 + -31672;
	// 82649140: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649144: 388B8B68  addi r4, r11, -0x7498
	ctx.r[4].s64 = ctx.r[11].s64 + -29848;
	// 82649148: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264914C: 386B1164  addi r3, r11, 0x1164
	ctx.r[3].s64 = ctx.r[11].s64 + 4452;
	// 82649150: 4BE32939  bl 0x8247ba88
	ctx.lr = 0x82649154;
	sub_8247BA88(ctx, base);
	// 82649154: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82649158: 386BCDB0  addi r3, r11, -0x3250
	ctx.r[3].s64 = ctx.r[11].s64 + -12880;
	// 8264915C: 4BEE99DD  bl 0x82532b38
	ctx.lr = 0x82649160;
	sub_82532B38(ctx, base);
	// 82649160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82649164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264916C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649170 size=108
    let mut pc: u32 = 0x82649170;
    'dispatch: loop {
        match pc {
            0x82649170 => {
    //   block [0x82649170..0x826491DC)
	// 82649170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264917C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649184: 38EB10A8  addi r7, r11, 0x10a8
	ctx.r[7].s64 = ctx.r[11].s64 + 4264;
	// 82649188: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8264918C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82649190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264919C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826491A0: 386A1180  addi r3, r10, 0x1180
	ctx.r[3].s64 = ctx.r[10].s64 + 4480;
	// 826491A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826491A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826491AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826491B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826491B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826491B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826491BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826491C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826491C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826491C8: 4BE1DC59  bl 0x82466e20
	ctx.lr = 0x826491CC;
	sub_82466E20(ctx, base);
	// 826491CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826491D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826491D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826491D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826491E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826491E0 size=24
    let mut pc: u32 = 0x826491E0;
    'dispatch: loop {
        match pc {
            0x826491E0 => {
    //   block [0x826491E0..0x826491F8)
	// 826491E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826491E4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826491E8: 394A76C0  addi r10, r10, 0x76c0
	ctx.r[10].s64 = ctx.r[10].s64 + 30400;
	// 826491EC: 816B1120  lwz r11, 0x1120(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4384 as u32) ) } as u64;
	// 826491F0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826491F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826491F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826491F8 size=112
    let mut pc: u32 = 0x826491F8;
    'dispatch: loop {
        match pc {
            0x826491F8 => {
    //   block [0x826491F8..0x82649268)
	// 826491F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826491FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649204: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264920C: 392B9504  addi r9, r11, -0x6afc
	ctx.r[9].s64 = ctx.r[11].s64 + -27388;
	// 82649210: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82649214: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82649218: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264921C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82649220: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649224: 396B76C0  addi r11, r11, 0x76c0
	ctx.r[11].s64 = ctx.r[11].s64 + 30400;
	// 82649228: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264922C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649230: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82649234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649238: 386A11B0  addi r3, r10, 0x11b0
	ctx.r[3].s64 = ctx.r[10].s64 + 4528;
	// 8264923C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649240: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82649244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649248: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264924C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649254: 4BE1DBCD  bl 0x82466e20
	ctx.lr = 0x82649258;
	sub_82466E20(ctx, base);
	// 82649258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264925C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649268 size=108
    let mut pc: u32 = 0x82649268;
    'dispatch: loop {
        match pc {
            0x82649268 => {
    //   block [0x82649268..0x826492D4)
	// 82649268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649274: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264927C: 38EB1124  addi r7, r11, 0x1124
	ctx.r[7].s64 = ctx.r[11].s64 + 4388;
	// 82649280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82649284: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82649288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264928C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649298: 386A11E0  addi r3, r10, 0x11e0
	ctx.r[3].s64 = ctx.r[10].s64 + 4576;
	// 8264929C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826492A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826492A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826492A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826492AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826492B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826492B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826492B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826492BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826492C0: 4BE1DB61  bl 0x82466e20
	ctx.lr = 0x826492C4;
	sub_82466E20(ctx, base);
	// 826492C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826492C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826492CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826492D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826492D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826492D8 size=108
    let mut pc: u32 = 0x826492D8;
    'dispatch: loop {
        match pc {
            0x826492D8 => {
    //   block [0x826492D8..0x82649344)
	// 826492D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826492DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826492E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826492E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826492E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826492EC: 38EB1154  addi r7, r11, 0x1154
	ctx.r[7].s64 = ctx.r[11].s64 + 4436;
	// 826492F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826492F4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826492F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826492FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649308: 386A1210  addi r3, r10, 0x1210
	ctx.r[3].s64 = ctx.r[10].s64 + 4624;
	// 8264930C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264931C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264932C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649330: 4BE1DAF1  bl 0x82466e20
	ctx.lr = 0x82649334;
	sub_82466E20(ctx, base);
	// 82649334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264933C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82649348 size=24
    let mut pc: u32 = 0x82649348;
    'dispatch: loop {
        match pc {
            0x82649348 => {
    //   block [0x82649348..0x82649360)
	// 82649348: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264934C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649350: 394A7708  addi r10, r10, 0x7708
	ctx.r[10].s64 = ctx.r[10].s64 + 30472;
	// 82649354: 816B1184  lwz r11, 0x1184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4484 as u32) ) } as u64;
	// 82649358: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649360 size=116
    let mut pc: u32 = 0x82649360;
    'dispatch: loop {
        match pc {
            0x82649360 => {
    //   block [0x82649360..0x826493D4)
	// 82649360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264936C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649370: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649374: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 82649378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264937C: 392A9548  addi r9, r10, -0x6ab8
	ctx.r[9].s64 = ctx.r[10].s64 + -27320;
	// 82649380: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649384: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82649388: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264938C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649394: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264939C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826493A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826493A4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826493A8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826493AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826493B0: 386B1240  addi r3, r11, 0x1240
	ctx.r[3].s64 = ctx.r[11].s64 + 4672;
	// 826493B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826493B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826493BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826493C0: 4BE1DA61  bl 0x82466e20
	ctx.lr = 0x826493C4;
	sub_82466E20(ctx, base);
	// 826493C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826493C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826493CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826493D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826493D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826493D8 size=108
    let mut pc: u32 = 0x826493D8;
    'dispatch: loop {
        match pc {
            0x826493D8 => {
    //   block [0x826493D8..0x82649444)
	// 826493D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826493DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826493E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826493E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826493E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826493EC: 38EB1188  addi r7, r11, 0x1188
	ctx.r[7].s64 = ctx.r[11].s64 + 4488;
	// 826493F0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826493F4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826493F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826493FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649408: 386A1270  addi r3, r10, 0x1270
	ctx.r[3].s64 = ctx.r[10].s64 + 4720;
	// 8264940C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264942C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649430: 4BE1D9F1  bl 0x82466e20
	ctx.lr = 0x82649434;
	sub_82466E20(ctx, base);
	// 82649434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649448 size=112
    let mut pc: u32 = 0x82649448;
    'dispatch: loop {
        match pc {
            0x82649448 => {
    //   block [0x82649448..0x826494B8)
	// 82649448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264945C: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 82649460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649464: 390B1218  addi r8, r11, 0x1218
	ctx.r[8].s64 = ctx.r[11].s64 + 4632;
	// 82649468: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8264946C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82649470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264947C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649480: 386A12A0  addi r3, r10, 0x12a0
	ctx.r[3].s64 = ctx.r[10].s64 + 4768;
	// 82649484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264948C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264949C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826494A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826494A4: 4BE1D97D  bl 0x82466e20
	ctx.lr = 0x826494A8;
	sub_82466E20(ctx, base);
	// 826494A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826494AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826494B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826494B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826494B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826494B8 size=112
    let mut pc: u32 = 0x826494B8;
    'dispatch: loop {
        match pc {
            0x826494B8 => {
    //   block [0x826494B8..0x82649528)
	// 826494B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826494BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826494C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826494C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826494C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826494CC: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 826494D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826494D4: 390B1338  addi r8, r11, 0x1338
	ctx.r[8].s64 = ctx.r[11].s64 + 4920;
	// 826494D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826494DC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826494E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826494E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826494E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826494EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826494F0: 386A12D0  addi r3, r10, 0x12d0
	ctx.r[3].s64 = ctx.r[10].s64 + 4816;
	// 826494F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826494F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826494FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264950C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649514: 4BE1D90D  bl 0x82466e20
	ctx.lr = 0x82649518;
	sub_82466E20(ctx, base);
	// 82649518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649528 size=108
    let mut pc: u32 = 0x82649528;
    'dispatch: loop {
        match pc {
            0x82649528 => {
    //   block [0x82649528..0x82649594)
	// 82649528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649534: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264953C: 38EB1350  addi r7, r11, 0x1350
	ctx.r[7].s64 = ctx.r[11].s64 + 4944;
	// 82649540: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82649544: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82649548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264954C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649558: 386A1300  addi r3, r10, 0x1300
	ctx.r[3].s64 = ctx.r[10].s64 + 4864;
	// 8264955C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264956C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264957C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649580: 4BE1D8A1  bl 0x82466e20
	ctx.lr = 0x82649584;
	sub_82466E20(ctx, base);
	// 82649584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649598 size=112
    let mut pc: u32 = 0x82649598;
    'dispatch: loop {
        match pc {
            0x82649598 => {
    //   block [0x82649598..0x82649608)
	// 82649598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264959C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826495A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826495A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826495A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826495AC: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 826495B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826495B4: 390B13E0  addi r8, r11, 0x13e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5088;
	// 826495B8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826495BC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826495C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826495C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826495C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826495CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826495D0: 386A1330  addi r3, r10, 0x1330
	ctx.r[3].s64 = ctx.r[10].s64 + 4912;
	// 826495D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826495D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826495DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826495E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826495E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826495E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826495EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826495F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826495F4: 4BE1D82D  bl 0x82466e20
	ctx.lr = 0x826495F8;
	sub_82466E20(ctx, base);
	// 826495F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826495FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649608 size=108
    let mut pc: u32 = 0x82649608;
    'dispatch: loop {
        match pc {
            0x82649608 => {
    //   block [0x82649608..0x82649674)
	// 82649608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264960C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649614: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264961C: 38EB14D0  addi r7, r11, 0x14d0
	ctx.r[7].s64 = ctx.r[11].s64 + 5328;
	// 82649620: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82649624: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82649628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264962C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649638: 386A1360  addi r3, r10, 0x1360
	ctx.r[3].s64 = ctx.r[10].s64 + 4960;
	// 8264963C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264964C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264965C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649660: 4BE1D7C1  bl 0x82466e20
	ctx.lr = 0x82649664;
	sub_82466E20(ctx, base);
	// 82649664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264966C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649678 size=108
    let mut pc: u32 = 0x82649678;
    'dispatch: loop {
        match pc {
            0x82649678 => {
    //   block [0x82649678..0x826496E4)
	// 82649678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649684: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264968C: 38EB14E8  addi r7, r11, 0x14e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5352;
	// 82649690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649694: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82649698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264969C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826496A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826496A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826496A8: 386A1390  addi r3, r10, 0x1390
	ctx.r[3].s64 = ctx.r[10].s64 + 5008;
	// 826496AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826496B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826496B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826496B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826496BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826496C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826496C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826496C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826496CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826496D0: 4BE1D751  bl 0x82466e20
	ctx.lr = 0x826496D4;
	sub_82466E20(ctx, base);
	// 826496D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826496D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826496DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826496E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826496E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826496E8 size=116
    let mut pc: u32 = 0x826496E8;
    'dispatch: loop {
        match pc {
            0x826496E8 => {
    //   block [0x826496E8..0x8264975C)
	// 826496E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826496EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826496F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826496F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826496F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826496FC: 390B154C  addi r8, r11, 0x154c
	ctx.r[8].s64 = ctx.r[11].s64 + 5452;
	// 82649700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649704: 392A9574  addi r9, r10, -0x6a8c
	ctx.r[9].s64 = ctx.r[10].s64 + -27276;
	// 82649708: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264970C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82649710: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82649714: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264971C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264972C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82649730: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82649734: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649738: 386B13C0  addi r3, r11, 0x13c0
	ctx.r[3].s64 = ctx.r[11].s64 + 5056;
	// 8264973C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649740: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649748: 4BE1D6D9  bl 0x82466e20
	ctx.lr = 0x8264974C;
	sub_82466E20(ctx, base);
	// 8264974C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649760 size=108
    let mut pc: u32 = 0x82649760;
    'dispatch: loop {
        match pc {
            0x82649760 => {
    //   block [0x82649760..0x826497CC)
	// 82649760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264976C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649774: 38EB1568  addi r7, r11, 0x1568
	ctx.r[7].s64 = ctx.r[11].s64 + 5480;
	// 82649778: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264977C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82649780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264978C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649790: 386A13F0  addi r3, r10, 0x13f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5104;
	// 82649794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264979C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826497A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826497A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826497A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826497AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826497B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826497B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826497B8: 4BE1D669  bl 0x82466e20
	ctx.lr = 0x826497BC;
	sub_82466E20(ctx, base);
	// 826497BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826497C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826497C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826497C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826497D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826497D0 size=108
    let mut pc: u32 = 0x826497D0;
    'dispatch: loop {
        match pc {
            0x826497D0 => {
    //   block [0x826497D0..0x8264983C)
	// 826497D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826497D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826497D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826497DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826497E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826497E4: 38EB15B0  addi r7, r11, 0x15b0
	ctx.r[7].s64 = ctx.r[11].s64 + 5552;
	// 826497E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826497EC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826497F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826497F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826497F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826497FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649800: 386A1420  addi r3, r10, 0x1420
	ctx.r[3].s64 = ctx.r[10].s64 + 5152;
	// 82649804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264980C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264981C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649828: 4BE1D5F9  bl 0x82466e20
	ctx.lr = 0x8264982C;
	sub_82466E20(ctx, base);
	// 8264982C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649840 size=108
    let mut pc: u32 = 0x82649840;
    'dispatch: loop {
        match pc {
            0x82649840 => {
    //   block [0x82649840..0x826498AC)
	// 82649840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264984C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649854: 38EB1640  addi r7, r11, 0x1640
	ctx.r[7].s64 = ctx.r[11].s64 + 5696;
	// 82649858: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264985C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82649860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264986C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649870: 386A1450  addi r3, r10, 0x1450
	ctx.r[3].s64 = ctx.r[10].s64 + 5200;
	// 82649874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264987C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264988C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649898: 4BE1D589  bl 0x82466e20
	ctx.lr = 0x8264989C;
	sub_82466E20(ctx, base);
	// 8264989C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826498A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826498A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826498A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826498B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826498B0 size=100
    let mut pc: u32 = 0x826498B0;
    'dispatch: loop {
        match pc {
            0x826498B0 => {
    //   block [0x826498B0..0x82649914)
	// 826498B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826498B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826498B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826498BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826498C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826498C4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826498C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826498CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826498D0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826498D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826498D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826498DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826498E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826498E4: 386A1480  addi r3, r10, 0x1480
	ctx.r[3].s64 = ctx.r[10].s64 + 5248;
	// 826498E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826498EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826498F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826498F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826498F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826498FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649900: 4BE1D521  bl 0x82466e20
	ctx.lr = 0x82649904;
	sub_82466E20(ctx, base);
	// 82649904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649918 size=112
    let mut pc: u32 = 0x82649918;
    'dispatch: loop {
        match pc {
            0x82649918 => {
    //   block [0x82649918..0x82649988)
	// 82649918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264991C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649928: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264992C: 38AA1480  addi r5, r10, 0x1480
	ctx.r[5].s64 = ctx.r[10].s64 + 5248;
	// 82649930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649934: 390B16D0  addi r8, r11, 0x16d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5840;
	// 82649938: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264993C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82649940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264994C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649950: 386A14B0  addi r3, r10, 0x14b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5296;
	// 82649954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264995C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264996C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649974: 4BE1D4AD  bl 0x82466e20
	ctx.lr = 0x82649978;
	sub_82466E20(ctx, base);
	// 82649978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264997C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649988 size=108
    let mut pc: u32 = 0x82649988;
    'dispatch: loop {
        match pc {
            0x82649988 => {
    //   block [0x82649988..0x826499F4)
	// 82649988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649994: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264999C: 38EB1730  addi r7, r11, 0x1730
	ctx.r[7].s64 = ctx.r[11].s64 + 5936;
	// 826499A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826499A4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826499A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826499AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826499B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826499B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826499B8: 386A14E0  addi r3, r10, 0x14e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5344;
	// 826499BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826499C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826499C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826499C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826499CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826499D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826499D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826499D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826499DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826499E0: 4BE1D441  bl 0x82466e20
	ctx.lr = 0x826499E4;
	sub_82466E20(ctx, base);
	// 826499E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826499E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826499EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826499F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826499F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826499F8 size=108
    let mut pc: u32 = 0x826499F8;
    'dispatch: loop {
        match pc {
            0x826499F8 => {
    //   block [0x826499F8..0x82649A64)
	// 826499F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826499FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649A04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649A0C: 38EB1760  addi r7, r11, 0x1760
	ctx.r[7].s64 = ctx.r[11].s64 + 5984;
	// 82649A10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649A14: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82649A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649A28: 386A1510  addi r3, r10, 0x1510
	ctx.r[3].s64 = ctx.r[10].s64 + 5392;
	// 82649A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649A50: 4BE1D3D1  bl 0x82466e20
	ctx.lr = 0x82649A54;
	sub_82466E20(ctx, base);
	// 82649A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649A68 size=108
    let mut pc: u32 = 0x82649A68;
    'dispatch: loop {
        match pc {
            0x82649A68 => {
    //   block [0x82649A68..0x82649AD4)
	// 82649A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649A74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649A7C: 38EB17C0  addi r7, r11, 0x17c0
	ctx.r[7].s64 = ctx.r[11].s64 + 6080;
	// 82649A80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649A84: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82649A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649A8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649A98: 386A1540  addi r3, r10, 0x1540
	ctx.r[3].s64 = ctx.r[10].s64 + 5440;
	// 82649A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649AC0: 4BE1D361  bl 0x82466e20
	ctx.lr = 0x82649AC4;
	sub_82466E20(ctx, base);
	// 82649AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649AD8 size=108
    let mut pc: u32 = 0x82649AD8;
    'dispatch: loop {
        match pc {
            0x82649AD8 => {
    //   block [0x82649AD8..0x82649B44)
	// 82649AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649AE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649AEC: 38EB1820  addi r7, r11, 0x1820
	ctx.r[7].s64 = ctx.r[11].s64 + 6176;
	// 82649AF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82649AF4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82649AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649AFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649B08: 386A1570  addi r3, r10, 0x1570
	ctx.r[3].s64 = ctx.r[10].s64 + 5488;
	// 82649B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649B30: 4BE1D2F1  bl 0x82466e20
	ctx.lr = 0x82649B34;
	sub_82466E20(ctx, base);
	// 82649B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649B48 size=112
    let mut pc: u32 = 0x82649B48;
    'dispatch: loop {
        match pc {
            0x82649B48 => {
    //   block [0x82649B48..0x82649BB8)
	// 82649B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649B54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649B58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649B5C: 392A95A8  addi r9, r10, -0x6a58
	ctx.r[9].s64 = ctx.r[10].s64 + -27224;
	// 82649B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649B64: 390B1898  addi r8, r11, 0x1898
	ctx.r[8].s64 = ctx.r[11].s64 + 6296;
	// 82649B68: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82649B6C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82649B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649B74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649B80: 386A15A0  addi r3, r10, 0x15a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5536;
	// 82649B84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649B88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649BA4: 4BE1D27D  bl 0x82466e20
	ctx.lr = 0x82649BA8;
	sub_82466E20(ctx, base);
	// 82649BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649BB8 size=112
    let mut pc: u32 = 0x82649BB8;
    'dispatch: loop {
        match pc {
            0x82649BB8 => {
    //   block [0x82649BB8..0x82649C28)
	// 82649BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649BC4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649BC8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82649BCC: 38EA19A0  addi r7, r10, 0x19a0
	ctx.r[7].s64 = ctx.r[10].s64 + 6560;
	// 82649BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649BD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649BD8: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82649BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649BE0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649BE4: 396B95BC  addi r11, r11, -0x6a44
	ctx.r[11].s64 = ctx.r[11].s64 + -27204;
	// 82649BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649BEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649BF4: 386A15D0  addi r3, r10, 0x15d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5584;
	// 82649BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649BFC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649C04: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649C0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649C14: 4BE1D20D  bl 0x82466e20
	ctx.lr = 0x82649C18;
	sub_82466E20(ctx, base);
	// 82649C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649C28 size=112
    let mut pc: u32 = 0x82649C28;
    'dispatch: loop {
        match pc {
            0x82649C28 => {
    //   block [0x82649C28..0x82649C98)
	// 82649C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649C34: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649C38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649C3C: 392A9600  addi r9, r10, -0x6a00
	ctx.r[9].s64 = ctx.r[10].s64 + -27136;
	// 82649C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649C44: 390B1AAC  addi r8, r11, 0x1aac
	ctx.r[8].s64 = ctx.r[11].s64 + 6828;
	// 82649C48: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82649C4C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82649C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649C54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649C60: 386A1600  addi r3, r10, 0x1600
	ctx.r[3].s64 = ctx.r[10].s64 + 5632;
	// 82649C64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649C68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649C84: 4BE1D19D  bl 0x82466e20
	ctx.lr = 0x82649C88;
	sub_82466E20(ctx, base);
	// 82649C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649C98 size=100
    let mut pc: u32 = 0x82649C98;
    'dispatch: loop {
        match pc {
            0x82649C98 => {
    //   block [0x82649C98..0x82649CFC)
	// 82649C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649CA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649CAC: 38AA1BD0  addi r5, r10, 0x1bd0
	ctx.r[5].s64 = ctx.r[10].s64 + 7120;
	// 82649CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649CB8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82649CBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649CCC: 386A1630  addi r3, r10, 0x1630
	ctx.r[3].s64 = ctx.r[10].s64 + 5680;
	// 82649CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82649CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649CE8: 4BE1D139  bl 0x82466e20
	ctx.lr = 0x82649CEC;
	sub_82466E20(ctx, base);
	// 82649CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649D00 size=116
    let mut pc: u32 = 0x82649D00;
    'dispatch: loop {
        match pc {
            0x82649D00 => {
    //   block [0x82649D00..0x82649D74)
	// 82649D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649D0C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649D10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82649D14: 390A1AE0  addi r8, r10, 0x1ae0
	ctx.r[8].s64 = ctx.r[10].s64 + 6880;
	// 82649D18: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649D20: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649D28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649D34: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82649D38: 396B9614  addi r11, r11, -0x69ec
	ctx.r[11].s64 = ctx.r[11].s64 + -27116;
	// 82649D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649D44: 386A1660  addi r3, r10, 0x1660
	ctx.r[3].s64 = ctx.r[10].s64 + 5728;
	// 82649D48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649D50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649D60: 4BE1D0C1  bl 0x82466e20
	ctx.lr = 0x82649D64;
	sub_82466E20(ctx, base);
	// 82649D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649D78 size=100
    let mut pc: u32 = 0x82649D78;
    'dispatch: loop {
        match pc {
            0x82649D78 => {
    //   block [0x82649D78..0x82649DDC)
	// 82649D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649D8C: 38AA1660  addi r5, r10, 0x1660
	ctx.r[5].s64 = ctx.r[10].s64 + 5728;
	// 82649D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649D98: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82649D9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649DAC: 386A1690  addi r3, r10, 0x1690
	ctx.r[3].s64 = ctx.r[10].s64 + 5776;
	// 82649DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82649DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649DC8: 4BE1D059  bl 0x82466e20
	ctx.lr = 0x82649DCC;
	sub_82466E20(ctx, base);
	// 82649DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649DE0 size=112
    let mut pc: u32 = 0x82649DE0;
    'dispatch: loop {
        match pc {
            0x82649DE0 => {
    //   block [0x82649DE0..0x82649E50)
	// 82649DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649DEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649DF0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649DF4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82649DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649DFC: 390B1B88  addi r8, r11, 0x1b88
	ctx.r[8].s64 = ctx.r[11].s64 + 7048;
	// 82649E00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82649E04: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82649E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649E0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649E18: 386A16C0  addi r3, r10, 0x16c0
	ctx.r[3].s64 = ctx.r[10].s64 + 5824;
	// 82649E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649E3C: 4BE1CFE5  bl 0x82466e20
	ctx.lr = 0x82649E40;
	sub_82466E20(ctx, base);
	// 82649E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649E50 size=116
    let mut pc: u32 = 0x82649E50;
    'dispatch: loop {
        match pc {
            0x82649E50 => {
    //   block [0x82649E50..0x82649EC4)
	// 82649E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649E5C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649E60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82649E64: 390A1BD0  addi r8, r10, 0x1bd0
	ctx.r[8].s64 = ctx.r[10].s64 + 7120;
	// 82649E68: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E6C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649E70: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649E74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649E78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649E84: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82649E88: 396B9640  addi r11, r11, -0x69c0
	ctx.r[11].s64 = ctx.r[11].s64 + -27072;
	// 82649E8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649E94: 386A16F0  addi r3, r10, 0x16f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5872;
	// 82649E98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649EA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649EB0: 4BE1CF71  bl 0x82466e20
	ctx.lr = 0x82649EB4;
	sub_82466E20(ctx, base);
	// 82649EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649EC8 size=108
    let mut pc: u32 = 0x82649EC8;
    'dispatch: loop {
        match pc {
            0x82649EC8 => {
    //   block [0x82649EC8..0x82649F34)
	// 82649EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649ED4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649EDC: 38EB1C90  addi r7, r11, 0x1c90
	ctx.r[7].s64 = ctx.r[11].s64 + 7312;
	// 82649EE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82649EE4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82649EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649EF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649EF8: 386A1720  addi r3, r10, 0x1720
	ctx.r[3].s64 = ctx.r[10].s64 + 5920;
	// 82649EFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649F20: 4BE1CF01  bl 0x82466e20
	ctx.lr = 0x82649F24;
	sub_82466E20(ctx, base);
	// 82649F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82649F38 size=24
    let mut pc: u32 = 0x82649F38;
    'dispatch: loop {
        match pc {
            0x82649F38 => {
    //   block [0x82649F38..0x82649F50)
	// 82649F38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649F3C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649F40: 394A7780  addi r10, r10, 0x7780
	ctx.r[10].s64 = ctx.r[10].s64 + 30592;
	// 82649F44: 816B1ADC  lwz r11, 0x1adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6876 as u32) ) } as u64;
	// 82649F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82649F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649F50 size=116
    let mut pc: u32 = 0x82649F50;
    'dispatch: loop {
        match pc {
            0x82649F50 => {
    //   block [0x82649F50..0x82649FC4)
	// 82649F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649F5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649F60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649F64: 392B9688  addi r9, r11, -0x6978
	ctx.r[9].s64 = ctx.r[11].s64 + -27000;
	// 82649F68: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649F6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649F70: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82649F74: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82649F78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649F7C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82649F80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649F84: 396B7780  addi r11, r11, 0x7780
	ctx.r[11].s64 = ctx.r[11].s64 + 30592;
	// 82649F88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82649F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649F90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82649F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649F98: 386A1750  addi r3, r10, 0x1750
	ctx.r[3].s64 = ctx.r[10].s64 + 5968;
	// 82649F9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649FA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82649FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649FA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82649FAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649FB0: 4BE1CE71  bl 0x82466e20
	ctx.lr = 0x82649FB4;
	sub_82466E20(ctx, base);
	// 82649FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649FC8 size=112
    let mut pc: u32 = 0x82649FC8;
    'dispatch: loop {
        match pc {
            0x82649FC8 => {
    //   block [0x82649FC8..0x8264A038)
	// 82649FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649FD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649FD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649FDC: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649FE4: 390B1CD8  addi r8, r11, 0x1cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 7384;
	// 82649FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82649FEC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82649FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A000: 386A1780  addi r3, r10, 0x1780
	ctx.r[3].s64 = ctx.r[10].s64 + 6016;
	// 8264A004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A024: 4BE1CDFD  bl 0x82466e20
	ctx.lr = 0x8264A028;
	sub_82466E20(ctx, base);
	// 8264A028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A038 size=112
    let mut pc: u32 = 0x8264A038;
    'dispatch: loop {
        match pc {
            0x8264A038 => {
    //   block [0x8264A038..0x8264A0A8)
	// 8264A038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A048: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A04C: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A054: 390B1D08  addi r8, r11, 0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + 7432;
	// 8264A058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A05C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8264A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A070: 386A17B0  addi r3, r10, 0x17b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6064;
	// 8264A074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A094: 4BE1CD8D  bl 0x82466e20
	ctx.lr = 0x8264A098;
	sub_82466E20(ctx, base);
	// 8264A098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A0A8 size=100
    let mut pc: u32 = 0x8264A0A8;
    'dispatch: loop {
        match pc {
            0x8264A0A8 => {
    //   block [0x8264A0A8..0x8264A10C)
	// 8264A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A0B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A0BC: 38AA1BD0  addi r5, r10, 0x1bd0
	ctx.r[5].s64 = ctx.r[10].s64 + 7120;
	// 8264A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A0C8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8264A0CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A0DC: 386A17E0  addi r3, r10, 0x17e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6112;
	// 8264A0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A0E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A0E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264A0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A0F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A0F8: 4BE1CD29  bl 0x82466e20
	ctx.lr = 0x8264A0FC;
	sub_82466E20(ctx, base);
	// 8264A0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A110 size=112
    let mut pc: u32 = 0x8264A110;
    'dispatch: loop {
        match pc {
            0x8264A110 => {
    //   block [0x8264A110..0x8264A180)
	// 8264A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A120: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A124: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A12C: 390B1D20  addi r8, r11, 0x1d20
	ctx.r[8].s64 = ctx.r[11].s64 + 7456;
	// 8264A130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A134: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 8264A138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A13C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A148: 386A1810  addi r3, r10, 0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + 6160;
	// 8264A14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A16C: 4BE1CCB5  bl 0x82466e20
	ctx.lr = 0x8264A170;
	sub_82466E20(ctx, base);
	// 8264A170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A180 size=108
    let mut pc: u32 = 0x8264A180;
    'dispatch: loop {
        match pc {
            0x8264A180 => {
    //   block [0x8264A180..0x8264A1EC)
	// 8264A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A18C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A194: 38EB1D50  addi r7, r11, 0x1d50
	ctx.r[7].s64 = ctx.r[11].s64 + 7504;
	// 8264A198: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264A19C: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8264A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A1A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A1B0: 386A1840  addi r3, r10, 0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + 6208;
	// 8264A1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A1D8: 4BE1CC49  bl 0x82466e20
	ctx.lr = 0x8264A1DC;
	sub_82466E20(ctx, base);
	// 8264A1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A1F0 size=108
    let mut pc: u32 = 0x8264A1F0;
    'dispatch: loop {
        match pc {
            0x8264A1F0 => {
    //   block [0x8264A1F0..0x8264A25C)
	// 8264A1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A1FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A204: 38EB1DB0  addi r7, r11, 0x1db0
	ctx.r[7].s64 = ctx.r[11].s64 + 7600;
	// 8264A208: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264A20C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 8264A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A214: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A220: 386A1870  addi r3, r10, 0x1870
	ctx.r[3].s64 = ctx.r[10].s64 + 6256;
	// 8264A224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A248: 4BE1CBD9  bl 0x82466e20
	ctx.lr = 0x8264A24C;
	sub_82466E20(ctx, base);
	// 8264A24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A260 size=116
    let mut pc: u32 = 0x8264A260;
    'dispatch: loop {
        match pc {
            0x8264A260 => {
    //   block [0x8264A260..0x8264A2D4)
	// 8264A260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A26C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A270: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8264A274: 390A1DE0  addi r8, r10, 0x1de0
	ctx.r[8].s64 = ctx.r[10].s64 + 7648;
	// 8264A278: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A27C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A280: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A284: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A288: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A294: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8264A298: 396B96E0  addi r11, r11, -0x6920
	ctx.r[11].s64 = ctx.r[11].s64 + -26912;
	// 8264A29C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A2A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A2A4: 386A18A0  addi r3, r10, 0x18a0
	ctx.r[3].s64 = ctx.r[10].s64 + 6304;
	// 8264A2A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A2AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A2B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A2C0: 4BE1CB61  bl 0x82466e20
	ctx.lr = 0x8264A2C4;
	sub_82466E20(ctx, base);
	// 8264A2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A2D8 size=112
    let mut pc: u32 = 0x8264A2D8;
    'dispatch: loop {
        match pc {
            0x8264A2D8 => {
    //   block [0x8264A2D8..0x8264A348)
	// 8264A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A2E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A2E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A2EC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A2F4: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 8264A2F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A2FC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 8264A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A310: 386A18D0  addi r3, r10, 0x18d0
	ctx.r[3].s64 = ctx.r[10].s64 + 6352;
	// 8264A314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A334: 4BE1CAED  bl 0x82466e20
	ctx.lr = 0x8264A338;
	sub_82466E20(ctx, base);
	// 8264A338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A348 size=116
    let mut pc: u32 = 0x8264A348;
    'dispatch: loop {
        match pc {
            0x8264A348 => {
    //   block [0x8264A348..0x8264A3BC)
	// 8264A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A354: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A358: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264A35C: 390A1F78  addi r8, r10, 0x1f78
	ctx.r[8].s64 = ctx.r[10].s64 + 8056;
	// 8264A360: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A364: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A368: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A36C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A37C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8264A380: 396B972C  addi r11, r11, -0x68d4
	ctx.r[11].s64 = ctx.r[11].s64 + -26836;
	// 8264A384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A38C: 386A1900  addi r3, r10, 0x1900
	ctx.r[3].s64 = ctx.r[10].s64 + 6400;
	// 8264A390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A3A8: 4BE1CA79  bl 0x82466e20
	ctx.lr = 0x8264A3AC;
	sub_82466E20(ctx, base);
	// 8264A3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A3C0 size=112
    let mut pc: u32 = 0x8264A3C0;
    'dispatch: loop {
        match pc {
            0x8264A3C0 => {
    //   block [0x8264A3C0..0x8264A430)
	// 8264A3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A3CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A3D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A3D4: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A3DC: 390B1FD8  addi r8, r11, 0x1fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 8152;
	// 8264A3E0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8264A3E4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8264A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A3EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A3F8: 386A1930  addi r3, r10, 0x1930
	ctx.r[3].s64 = ctx.r[10].s64 + 6448;
	// 8264A3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A41C: 4BE1CA05  bl 0x82466e20
	ctx.lr = 0x8264A420;
	sub_82466E20(ctx, base);
	// 8264A420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A430 size=112
    let mut pc: u32 = 0x8264A430;
    'dispatch: loop {
        match pc {
            0x8264A430 => {
    //   block [0x8264A430..0x8264A4A0)
	// 8264A430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A43C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A440: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A444: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A44C: 390B20B0  addi r8, r11, 0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8368;
	// 8264A450: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8264A454: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8264A458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A45C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A468: 386A1960  addi r3, r10, 0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + 6496;
	// 8264A46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A48C: 4BE1C995  bl 0x82466e20
	ctx.lr = 0x8264A490;
	sub_82466E20(ctx, base);
	// 8264A490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A4A0 size=112
    let mut pc: u32 = 0x8264A4A0;
    'dispatch: loop {
        match pc {
            0x8264A4A0 => {
    //   block [0x8264A4A0..0x8264A510)
	// 8264A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A4AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A4B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A4B4: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A4B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264A4BC: 390B21B8  addi r8, r11, 0x21b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8632;
	// 8264A4C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A4C4: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 8264A4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A4CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A4D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A4D8: 386A1990  addi r3, r10, 0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + 6544;
	// 8264A4DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A4FC: 4BE1C925  bl 0x82466e20
	ctx.lr = 0x8264A500;
	sub_82466E20(ctx, base);
	// 8264A500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264A510 size=24
    let mut pc: u32 = 0x8264A510;
    'dispatch: loop {
        match pc {
            0x8264A510 => {
    //   block [0x8264A510..0x8264A528)
	// 8264A510: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A514: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A518: 394A78E8  addi r10, r10, 0x78e8
	ctx.r[10].s64 = ctx.r[10].s64 + 30952;
	// 8264A51C: 816B21E8  lwz r11, 0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8680 as u32) ) } as u64;
	// 8264A520: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264A524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A528 size=116
    let mut pc: u32 = 0x8264A528;
    'dispatch: loop {
        match pc {
            0x8264A528 => {
    //   block [0x8264A528..0x8264A59C)
	// 8264A528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A534: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A538: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A53C: 392B975C  addi r9, r11, -0x68a4
	ctx.r[9].s64 = ctx.r[11].s64 + -26788;
	// 8264A540: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A544: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264A548: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264A54C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8264A550: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A554: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 8264A558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A55C: 396B78E8  addi r11, r11, 0x78e8
	ctx.r[11].s64 = ctx.r[11].s64 + 30952;
	// 8264A560: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264A564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A568: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264A56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A570: 386A19C0  addi r3, r10, 0x19c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6592;
	// 8264A574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264A578: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264A57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A580: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264A584: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A588: 4BE1C899  bl 0x82466e20
	ctx.lr = 0x8264A58C;
	sub_82466E20(ctx, base);
	// 8264A58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A5A0 size=116
    let mut pc: u32 = 0x8264A5A0;
    'dispatch: loop {
        match pc {
            0x8264A5A0 => {
    //   block [0x8264A5A0..0x8264A614)
	// 8264A5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A5AC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A5B0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8264A5B4: 390A21EC  addi r8, r10, 0x21ec
	ctx.r[8].s64 = ctx.r[10].s64 + 8684;
	// 8264A5B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A5BC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A5C0: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264A5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A5D4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8264A5D8: 396B97CC  addi r11, r11, -0x6834
	ctx.r[11].s64 = ctx.r[11].s64 + -26676;
	// 8264A5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A5E4: 386A19F0  addi r3, r10, 0x19f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6640;
	// 8264A5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A600: 4BE1C821  bl 0x82466e20
	ctx.lr = 0x8264A604;
	sub_82466E20(ctx, base);
	// 8264A604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A618 size=108
    let mut pc: u32 = 0x8264A618;
    'dispatch: loop {
        match pc {
            0x8264A618 => {
    //   block [0x8264A618..0x8264A684)
	// 8264A618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A624: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A62C: 38EB2220  addi r7, r11, 0x2220
	ctx.r[7].s64 = ctx.r[11].s64 + 8736;
	// 8264A630: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264A634: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 8264A638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A63C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A648: 386A1A20  addi r3, r10, 0x1a20
	ctx.r[3].s64 = ctx.r[10].s64 + 6688;
	// 8264A64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A670: 4BE1C7B1  bl 0x82466e20
	ctx.lr = 0x8264A674;
	sub_82466E20(ctx, base);
	// 8264A674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264A688 size=24
    let mut pc: u32 = 0x8264A688;
    'dispatch: loop {
        match pc {
            0x8264A688 => {
    //   block [0x8264A688..0x8264A6A0)
	// 8264A688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A68C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A690: 394A7A80  addi r10, r10, 0x7a80
	ctx.r[10].s64 = ctx.r[10].s64 + 31360;
	// 8264A694: 816B221C  lwz r11, 0x221c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8732 as u32) ) } as u64;
	// 8264A698: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264A69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A6A0 size=116
    let mut pc: u32 = 0x8264A6A0;
    'dispatch: loop {
        match pc {
            0x8264A6A0 => {
    //   block [0x8264A6A0..0x8264A714)
	// 8264A6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A6AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A6B0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A6B4: 392B97F0  addi r9, r11, -0x6810
	ctx.r[9].s64 = ctx.r[11].s64 + -26640;
	// 8264A6B8: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A6BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A6C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264A6C4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8264A6C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A6CC: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8264A6D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A6D4: 396B7A80  addi r11, r11, 0x7a80
	ctx.r[11].s64 = ctx.r[11].s64 + 31360;
	// 8264A6D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264A6DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A6E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264A6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A6E8: 386A1A50  addi r3, r10, 0x1a50
	ctx.r[3].s64 = ctx.r[10].s64 + 6736;
	// 8264A6EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264A6F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264A6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A6F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264A6FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A700: 4BE1C721  bl 0x82466e20
	ctx.lr = 0x8264A704;
	sub_82466E20(ctx, base);
	// 8264A704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A718 size=112
    let mut pc: u32 = 0x8264A718;
    'dispatch: loop {
        match pc {
            0x8264A718 => {
    //   block [0x8264A718..0x8264A788)
	// 8264A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A728: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A72C: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A734: 390B22B0  addi r8, r11, 0x22b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8880;
	// 8264A738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A73C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 8264A740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A750: 386A1A80  addi r3, r10, 0x1a80
	ctx.r[3].s64 = ctx.r[10].s64 + 6784;
	// 8264A754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A774: 4BE1C6AD  bl 0x82466e20
	ctx.lr = 0x8264A778;
	sub_82466E20(ctx, base);
	// 8264A778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A788 size=116
    let mut pc: u32 = 0x8264A788;
    'dispatch: loop {
        match pc {
            0x8264A788 => {
    //   block [0x8264A788..0x8264A7FC)
	// 8264A788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A794: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A798: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264A79C: 390A22E0  addi r8, r10, 0x22e0
	ctx.r[8].s64 = ctx.r[10].s64 + 8928;
	// 8264A7A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A7A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A7A8: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A7AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A7B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A7B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A7BC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 8264A7C0: 396B9838  addi r11, r11, -0x67c8
	ctx.r[11].s64 = ctx.r[11].s64 + -26568;
	// 8264A7C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A7C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A7CC: 386A1AB0  addi r3, r10, 0x1ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 6832;
	// 8264A7D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A7D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A7E8: 4BE1C639  bl 0x82466e20
	ctx.lr = 0x8264A7EC;
	sub_82466E20(ctx, base);
	// 8264A7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A800 size=108
    let mut pc: u32 = 0x8264A800;
    'dispatch: loop {
        match pc {
            0x8264A800 => {
    //   block [0x8264A800..0x8264A86C)
	// 8264A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A80C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A814: 38EB2370  addi r7, r11, 0x2370
	ctx.r[7].s64 = ctx.r[11].s64 + 9072;
	// 8264A818: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8264A81C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8264A820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A830: 386A1AE0  addi r3, r10, 0x1ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 6880;
	// 8264A834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A858: 4BE1C5C9  bl 0x82466e20
	ctx.lr = 0x8264A85C;
	sub_82466E20(ctx, base);
	// 8264A85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A870 size=116
    let mut pc: u32 = 0x8264A870;
    'dispatch: loop {
        match pc {
            0x8264A870 => {
    //   block [0x8264A870..0x8264A8E4)
	// 8264A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A87C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A880: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264A884: 390A24C0  addi r8, r10, 0x24c0
	ctx.r[8].s64 = ctx.r[10].s64 + 9408;
	// 8264A888: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A88C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A890: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A894: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A898: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A8A4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8264A8A8: 396B985C  addi r11, r11, -0x67a4
	ctx.r[11].s64 = ctx.r[11].s64 + -26532;
	// 8264A8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A8B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A8B4: 386A1B10  addi r3, r10, 0x1b10
	ctx.r[3].s64 = ctx.r[10].s64 + 6928;
	// 8264A8B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A8BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A8C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A8D0: 4BE1C551  bl 0x82466e20
	ctx.lr = 0x8264A8D4;
	sub_82466E20(ctx, base);
	// 8264A8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A8E8 size=112
    let mut pc: u32 = 0x8264A8E8;
    'dispatch: loop {
        match pc {
            0x8264A8E8 => {
    //   block [0x8264A8E8..0x8264A958)
	// 8264A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A8F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A8F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A8FC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A904: 390B2580  addi r8, r11, 0x2580
	ctx.r[8].s64 = ctx.r[11].s64 + 9600;
	// 8264A908: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A90C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 8264A910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A920: 386A1B40  addi r3, r10, 0x1b40
	ctx.r[3].s64 = ctx.r[10].s64 + 6976;
	// 8264A924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A944: 4BE1C4DD  bl 0x82466e20
	ctx.lr = 0x8264A948;
	sub_82466E20(ctx, base);
	// 8264A948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A958 size=112
    let mut pc: u32 = 0x8264A958;
    'dispatch: loop {
        match pc {
            0x8264A958 => {
    //   block [0x8264A958..0x8264A9C8)
	// 8264A958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A968: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A96C: 38AA1930  addi r5, r10, 0x1930
	ctx.r[5].s64 = ctx.r[10].s64 + 6448;
	// 8264A970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A974: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 8264A978: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264A97C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8264A980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A990: 386A1B70  addi r3, r10, 0x1b70
	ctx.r[3].s64 = ctx.r[10].s64 + 7024;
	// 8264A994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A9B4: 4BE1C46D  bl 0x82466e20
	ctx.lr = 0x8264A9B8;
	sub_82466E20(ctx, base);
	// 8264A9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A9C8 size=112
    let mut pc: u32 = 0x8264A9C8;
    'dispatch: loop {
        match pc {
            0x8264A9C8 => {
    //   block [0x8264A9C8..0x8264AA38)
	// 8264A9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A9D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A9D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A9DC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A9E4: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 8264A9E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A9EC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 8264A9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A9F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AA00: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 8264AA04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264AA08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AA24: 4BE1C3FD  bl 0x82466e20
	ctx.lr = 0x8264AA28;
	sub_82466E20(ctx, base);
	// 8264AA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AA38 size=116
    let mut pc: u32 = 0x8264AA38;
    'dispatch: loop {
        match pc {
            0x8264AA38 => {
    //   block [0x8264AA38..0x8264AAAC)
	// 8264AA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AA44: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AA48: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264AA4C: 390B2644  addi r8, r11, 0x2644
	ctx.r[8].s64 = ctx.r[11].s64 + 9796;
	// 8264AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AA54: 392A98A4  addi r9, r10, -0x675c
	ctx.r[9].s64 = ctx.r[10].s64 + -26460;
	// 8264AA58: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AA5C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264AA60: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AA64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AA6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AA7C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264AA80: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8264AA84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AA88: 386B1BD0  addi r3, r11, 0x1bd0
	ctx.r[3].s64 = ctx.r[11].s64 + 7120;
	// 8264AA8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AA90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AA98: 4BE1C389  bl 0x82466e20
	ctx.lr = 0x8264AA9C;
	sub_82466E20(ctx, base);
	// 8264AA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AAB0 size=100
    let mut pc: u32 = 0x8264AAB0;
    'dispatch: loop {
        match pc {
            0x8264AAB0 => {
    //   block [0x8264AAB0..0x8264AB14)
	// 8264AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AAC4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AAC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AAD0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8264AAD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AAE4: 386A1C00  addi r3, r10, 0x1c00
	ctx.r[3].s64 = ctx.r[10].s64 + 7168;
	// 8264AAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AAEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AAF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AAF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AB00: 4BE1C321  bl 0x82466e20
	ctx.lr = 0x8264AB04;
	sub_82466E20(ctx, base);
	// 8264AB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AB18 size=112
    let mut pc: u32 = 0x8264AB18;
    'dispatch: loop {
        match pc {
            0x8264AB18 => {
    //   block [0x8264AB18..0x8264AB88)
	// 8264AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AB24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AB28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AB2C: 38AA1C00  addi r5, r10, 0x1c00
	ctx.r[5].s64 = ctx.r[10].s64 + 7168;
	// 8264AB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AB34: 390B265C  addi r8, r11, 0x265c
	ctx.r[8].s64 = ctx.r[11].s64 + 9820;
	// 8264AB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264AB3C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 8264AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AB44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AB50: 386A1C30  addi r3, r10, 0x1c30
	ctx.r[3].s64 = ctx.r[10].s64 + 7216;
	// 8264AB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264AB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AB74: 4BE1C2AD  bl 0x82466e20
	ctx.lr = 0x8264AB78;
	sub_82466E20(ctx, base);
	// 8264AB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AB88 size=108
    let mut pc: u32 = 0x8264AB88;
    'dispatch: loop {
        match pc {
            0x8264AB88 => {
    //   block [0x8264AB88..0x8264ABF4)
	// 8264AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AB94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AB98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264AB9C: 38EB2678  addi r7, r11, 0x2678
	ctx.r[7].s64 = ctx.r[11].s64 + 9848;
	// 8264ABA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264ABA4: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 8264ABA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264ABAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ABB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264ABB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264ABB8: 386A1C60  addi r3, r10, 0x1c60
	ctx.r[3].s64 = ctx.r[10].s64 + 7264;
	// 8264ABBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264ABC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ABC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ABC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ABD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ABD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ABD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ABDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264ABE0: 4BE1C241  bl 0x82466e20
	ctx.lr = 0x8264ABE4;
	sub_82466E20(ctx, base);
	// 8264ABE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ABE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ABEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ABF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ABF8 size=112
    let mut pc: u32 = 0x8264ABF8;
    'dispatch: loop {
        match pc {
            0x8264ABF8 => {
    //   block [0x8264ABF8..0x8264AC68)
	// 8264ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AC04: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264AC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AC0C: 392B9910  addi r9, r11, -0x66f0
	ctx.r[9].s64 = ctx.r[11].s64 + -26352;
	// 8264AC10: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8264AC14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264AC18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AC1C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8264AC20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AC24: 396B26D8  addi r11, r11, 0x26d8
	ctx.r[11].s64 = ctx.r[11].s64 + 9944;
	// 8264AC28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264AC2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AC30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264AC34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AC38: 386A1C90  addi r3, r10, 0x1c90
	ctx.r[3].s64 = ctx.r[10].s64 + 7312;
	// 8264AC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AC40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AC48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264AC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AC50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AC54: 4BE1C1CD  bl 0x82466e20
	ctx.lr = 0x8264AC58;
	sub_82466E20(ctx, base);
	// 8264AC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AC68 size=108
    let mut pc: u32 = 0x8264AC68;
    'dispatch: loop {
        match pc {
            0x8264AC68 => {
    //   block [0x8264AC68..0x8264ACD4)
	// 8264AC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AC74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AC7C: 38EB2798  addi r7, r11, 0x2798
	ctx.r[7].s64 = ctx.r[11].s64 + 10136;
	// 8264AC80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264AC84: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8264AC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AC8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AC98: 386A1CC0  addi r3, r10, 0x1cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7360;
	// 8264AC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264ACA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ACA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ACA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ACAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ACB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ACB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ACB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ACBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264ACC0: 4BE1C161  bl 0x82466e20
	ctx.lr = 0x8264ACC4;
	sub_82466E20(ctx, base);
	// 8264ACC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ACC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ACCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ACD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ACD8 size=116
    let mut pc: u32 = 0x8264ACD8;
    'dispatch: loop {
        match pc {
            0x8264ACD8 => {
    //   block [0x8264ACD8..0x8264AD4C)
	// 8264ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ACE4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264ACE8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8264ACEC: 390A27E0  addi r8, r10, 0x27e0
	ctx.r[8].s64 = ctx.r[10].s64 + 10208;
	// 8264ACF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ACF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264ACF8: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264ACFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AD00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AD08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AD0C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8264AD10: 396B9948  addi r11, r11, -0x66b8
	ctx.r[11].s64 = ctx.r[11].s64 + -26296;
	// 8264AD14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AD1C: 386A1CF0  addi r3, r10, 0x1cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 7408;
	// 8264AD20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264AD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AD28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264AD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AD38: 4BE1C0E9  bl 0x82466e20
	ctx.lr = 0x8264AD3C;
	sub_82466E20(ctx, base);
	// 8264AD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AD50 size=100
    let mut pc: u32 = 0x8264AD50;
    'dispatch: loop {
        match pc {
            0x8264AD50 => {
    //   block [0x8264AD50..0x8264ADB4)
	// 8264AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AD5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AD64: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264AD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AD70: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 8264AD74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AD84: 386A1D20  addi r3, r10, 0x1d20
	ctx.r[3].s64 = ctx.r[10].s64 + 7456;
	// 8264AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ADA0: 4BE1C081  bl 0x82466e20
	ctx.lr = 0x8264ADA4;
	sub_82466E20(ctx, base);
	// 8264ADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264ADB8 size=24
    let mut pc: u32 = 0x8264ADB8;
    'dispatch: loop {
        match pc {
            0x8264ADB8 => {
    //   block [0x8264ADB8..0x8264ADD0)
	// 8264ADB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ADBC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264ADC0: 394A7B88  addi r10, r10, 0x7b88
	ctx.r[10].s64 = ctx.r[10].s64 + 31624;
	// 8264ADC4: 816B2964  lwz r11, 0x2964(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10596 as u32) ) } as u64;
	// 8264ADC8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264ADCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ADD0 size=116
    let mut pc: u32 = 0x8264ADD0;
    'dispatch: loop {
        match pc {
            0x8264ADD0 => {
    //   block [0x8264ADD0..0x8264AE44)
	// 8264ADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ADD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ADDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264ADE0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ADE4: 392B99C8  addi r9, r11, -0x6638
	ctx.r[9].s64 = ctx.r[11].s64 + -26168;
	// 8264ADE8: 38AA1D20  addi r5, r10, 0x1d20
	ctx.r[5].s64 = ctx.r[10].s64 + 7456;
	// 8264ADEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264ADF0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 8264ADF4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8264ADF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ADFC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8264AE00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AE04: 396B7B88  addi r11, r11, 0x7b88
	ctx.r[11].s64 = ctx.r[11].s64 + 31624;
	// 8264AE08: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264AE0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AE10: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264AE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AE18: 386A1D50  addi r3, r10, 0x1d50
	ctx.r[3].s64 = ctx.r[10].s64 + 7504;
	// 8264AE1C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264AE20: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264AE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AE28: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264AE2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AE30: 4BE1BFF1  bl 0x82466e20
	ctx.lr = 0x8264AE34;
	sub_82466E20(ctx, base);
	// 8264AE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AE48 size=112
    let mut pc: u32 = 0x8264AE48;
    'dispatch: loop {
        match pc {
            0x8264AE48 => {
    //   block [0x8264AE48..0x8264AEB8)
	// 8264AE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AE54: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264AE58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264AE5C: 38EA2970  addi r7, r10, 0x2970
	ctx.r[7].s64 = ctx.r[10].s64 + 10608;
	// 8264AE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AE64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264AE68: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8264AE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AE70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264AE74: 396B9A54  addi r11, r11, -0x65ac
	ctx.r[11].s64 = ctx.r[11].s64 + -26028;
	// 8264AE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AE7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AE80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AE84: 386A1D80  addi r3, r10, 0x1d80
	ctx.r[3].s64 = ctx.r[10].s64 + 7552;
	// 8264AE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AE8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264AE90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AE94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264AE98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AE9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AEA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AEA4: 4BE1BF7D  bl 0x82466e20
	ctx.lr = 0x8264AEA8;
	sub_82466E20(ctx, base);
	// 8264AEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AEB8 size=108
    let mut pc: u32 = 0x8264AEB8;
    'dispatch: loop {
        match pc {
            0x8264AEB8 => {
    //   block [0x8264AEB8..0x8264AF24)
	// 8264AEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AEC4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AECC: 38EB29D0  addi r7, r11, 0x29d0
	ctx.r[7].s64 = ctx.r[11].s64 + 10704;
	// 8264AED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264AED4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8264AED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AEE8: 386A1DB0  addi r3, r10, 0x1db0
	ctx.r[3].s64 = ctx.r[10].s64 + 7600;
	// 8264AEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264AEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AF10: 4BE1BF11  bl 0x82466e20
	ctx.lr = 0x8264AF14;
	sub_82466E20(ctx, base);
	// 8264AF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AF28 size=116
    let mut pc: u32 = 0x8264AF28;
    'dispatch: loop {
        match pc {
            0x8264AF28 => {
    //   block [0x8264AF28..0x8264AF9C)
	// 8264AF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AF34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AF38: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264AF3C: 390B2A00  addi r8, r11, 0x2a00
	ctx.r[8].s64 = ctx.r[11].s64 + 10752;
	// 8264AF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AF44: 392A9A40  addi r9, r10, -0x65c0
	ctx.r[9].s64 = ctx.r[10].s64 + -26048;
	// 8264AF48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AF4C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264AF50: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AF54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AF5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AF6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264AF70: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8264AF74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AF78: 386B1DE0  addi r3, r11, 0x1de0
	ctx.r[3].s64 = ctx.r[11].s64 + 7648;
	// 8264AF7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AF80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AF88: 4BE1BE99  bl 0x82466e20
	ctx.lr = 0x8264AF8C;
	sub_82466E20(ctx, base);
	// 8264AF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AFA0 size=96
    let mut pc: u32 = 0x8264AFA0;
    'dispatch: loop {
        match pc {
            0x8264AFA0 => {
    //   block [0x8264AFA0..0x8264B000)
	// 8264AFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AFAC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264AFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AFB4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8264AFB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AFC0: 386A1E10  addi r3, r10, 0x1e10
	ctx.r[3].s64 = ctx.r[10].s64 + 7696;
	// 8264AFC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AFCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AFE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AFE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AFEC: 4BE1BE35  bl 0x82466e20
	ctx.lr = 0x8264AFF0;
	sub_82466E20(ctx, base);
	// 8264AFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B000 size=112
    let mut pc: u32 = 0x8264B000;
    'dispatch: loop {
        match pc {
            0x8264B000 => {
    //   block [0x8264B000..0x8264B070)
	// 8264B000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B00C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B014: 38AA1E10  addi r5, r10, 0x1e10
	ctx.r[5].s64 = ctx.r[10].s64 + 7696;
	// 8264B018: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B01C: 390B2A18  addi r8, r11, 0x2a18
	ctx.r[8].s64 = ctx.r[11].s64 + 10776;
	// 8264B020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264B024: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8264B028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B02C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B038: 386A1E40  addi r3, r10, 0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + 7744;
	// 8264B03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264B040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B05C: 4BE1BDC5  bl 0x82466e20
	ctx.lr = 0x8264B060;
	sub_82466E20(ctx, base);
	// 8264B060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B070 size=112
    let mut pc: u32 = 0x8264B070;
    'dispatch: loop {
        match pc {
            0x8264B070 => {
    //   block [0x8264B070..0x8264B0E0)
	// 8264B070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B07C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B084: 392A9A70  addi r9, r10, -0x6590
	ctx.r[9].s64 = ctx.r[10].s64 + -26000;
	// 8264B088: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B08C: 390B2A48  addi r8, r11, 0x2a48
	ctx.r[8].s64 = ctx.r[11].s64 + 10824;
	// 8264B090: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8264B094: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8264B098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B09C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B0A8: 386A1E70  addi r3, r10, 0x1e70
	ctx.r[3].s64 = ctx.r[10].s64 + 7792;
	// 8264B0AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B0B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B0CC: 4BE1BD55  bl 0x82466e20
	ctx.lr = 0x8264B0D0;
	sub_82466E20(ctx, base);
	// 8264B0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B0E0 size=108
    let mut pc: u32 = 0x8264B0E0;
    'dispatch: loop {
        match pc {
            0x8264B0E0 => {
    //   block [0x8264B0E0..0x8264B14C)
	// 8264B0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B0EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B0F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B0F4: 38EB2AF0  addi r7, r11, 0x2af0
	ctx.r[7].s64 = ctx.r[11].s64 + 10992;
	// 8264B0F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B0FC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8264B100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B110: 386A1EA0  addi r3, r10, 0x1ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 7840;
	// 8264B114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B138: 4BE1BCE9  bl 0x82466e20
	ctx.lr = 0x8264B13C;
	sub_82466E20(ctx, base);
	// 8264B13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B150 size=108
    let mut pc: u32 = 0x8264B150;
    'dispatch: loop {
        match pc {
            0x8264B150 => {
    //   block [0x8264B150..0x8264B1BC)
	// 8264B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B15C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B160: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B164: 38EB2B20  addi r7, r11, 0x2b20
	ctx.r[7].s64 = ctx.r[11].s64 + 11040;
	// 8264B168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B16C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8264B170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B180: 386A1ED0  addi r3, r10, 0x1ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 7888;
	// 8264B184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B1A8: 4BE1BC79  bl 0x82466e20
	ctx.lr = 0x8264B1AC;
	sub_82466E20(ctx, base);
	// 8264B1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B1C0 size=28
    let mut pc: u32 = 0x8264B1C0;
    'dispatch: loop {
        match pc {
            0x8264B1C0 => {
    //   block [0x8264B1C0..0x8264B1DC)
	// 8264B1C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B1C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B1C8: 394A7CA8  addi r10, r10, 0x7ca8
	ctx.r[10].s64 = ctx.r[10].s64 + 31912;
	// 8264B1CC: 816B2B50  lwz r11, 0x2b50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11088 as u32) ) } as u64;
	// 8264B1D0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264B1D4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264B1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B1E0 size=112
    let mut pc: u32 = 0x8264B1E0;
    'dispatch: loop {
        match pc {
            0x8264B1E0 => {
    //   block [0x8264B1E0..0x8264B250)
	// 8264B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B1EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B1F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B1F4: 392A9BF0  addi r9, r10, -0x6410
	ctx.r[9].s64 = ctx.r[10].s64 + -25616;
	// 8264B1F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B1FC: 390B7CA8  addi r8, r11, 0x7ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 31912;
	// 8264B200: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264B204: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8264B208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B20C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B218: 386A1F00  addi r3, r10, 0x1f00
	ctx.r[3].s64 = ctx.r[10].s64 + 7936;
	// 8264B21C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B220: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8264B224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B23C: 4BE1BBE5  bl 0x82466e20
	ctx.lr = 0x8264B240;
	sub_82466E20(ctx, base);
	// 8264B240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B250 size=108
    let mut pc: u32 = 0x8264B250;
    'dispatch: loop {
        match pc {
            0x8264B250 => {
    //   block [0x8264B250..0x8264B2BC)
	// 8264B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B25C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B260: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B264: 38EB2B5C  addi r7, r11, 0x2b5c
	ctx.r[7].s64 = ctx.r[11].s64 + 11100;
	// 8264B268: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B26C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 8264B270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B280: 386A1F30  addi r3, r10, 0x1f30
	ctx.r[3].s64 = ctx.r[10].s64 + 7984;
	// 8264B284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B2A8: 4BE1BB79  bl 0x82466e20
	ctx.lr = 0x8264B2AC;
	sub_82466E20(ctx, base);
	// 8264B2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B2C0 size=108
    let mut pc: u32 = 0x8264B2C0;
    'dispatch: loop {
        match pc {
            0x8264B2C0 => {
    //   block [0x8264B2C0..0x8264B32C)
	// 8264B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B2CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B2D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B2D4: 38EB2B74  addi r7, r11, 0x2b74
	ctx.r[7].s64 = ctx.r[11].s64 + 11124;
	// 8264B2D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B2DC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8264B2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B2E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B2F0: 386A1F60  addi r3, r10, 0x1f60
	ctx.r[3].s64 = ctx.r[10].s64 + 8032;
	// 8264B2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B318: 4BE1BB09  bl 0x82466e20
	ctx.lr = 0x8264B31C;
	sub_82466E20(ctx, base);
	// 8264B31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B330 size=108
    let mut pc: u32 = 0x8264B330;
    'dispatch: loop {
        match pc {
            0x8264B330 => {
    //   block [0x8264B330..0x8264B39C)
	// 8264B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B340: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B344: 38EB2BA4  addi r7, r11, 0x2ba4
	ctx.r[7].s64 = ctx.r[11].s64 + 11172;
	// 8264B348: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B34C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8264B350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B360: 386A1F90  addi r3, r10, 0x1f90
	ctx.r[3].s64 = ctx.r[10].s64 + 8080;
	// 8264B364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B388: 4BE1BA99  bl 0x82466e20
	ctx.lr = 0x8264B38C;
	sub_82466E20(ctx, base);
	// 8264B38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B3A0 size=24
    let mut pc: u32 = 0x8264B3A0;
    'dispatch: loop {
        match pc {
            0x8264B3A0 => {
    //   block [0x8264B3A0..0x8264B3B8)
	// 8264B3A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B3A4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B3A8: 394A7D68  addi r10, r10, 0x7d68
	ctx.r[10].s64 = ctx.r[10].s64 + 32104;
	// 8264B3AC: 816B2BBC  lwz r11, 0x2bbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11196 as u32) ) } as u64;
	// 8264B3B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264B3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B3B8 size=112
    let mut pc: u32 = 0x8264B3B8;
    'dispatch: loop {
        match pc {
            0x8264B3B8 => {
    //   block [0x8264B3B8..0x8264B428)
	// 8264B3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B3C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B3C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B3CC: 392A9C44  addi r9, r10, -0x63bc
	ctx.r[9].s64 = ctx.r[10].s64 + -25532;
	// 8264B3D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B3D4: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 8264B3D8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264B3DC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8264B3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B3F0: 386A1FC0  addi r3, r10, 0x1fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 8128;
	// 8264B3F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B3F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B40C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B414: 4BE1BA0D  bl 0x82466e20
	ctx.lr = 0x8264B418;
	sub_82466E20(ctx, base);
	// 8264B418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B428 size=112
    let mut pc: u32 = 0x8264B428;
    'dispatch: loop {
        match pc {
            0x8264B428 => {
    //   block [0x8264B428..0x8264B498)
	// 8264B428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B434: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B438: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B43C: 392A9C80  addi r9, r10, -0x6380
	ctx.r[9].s64 = ctx.r[10].s64 + -25472;
	// 8264B440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B444: 390B2BC8  addi r8, r11, 0x2bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 11208;
	// 8264B448: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264B44C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8264B450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B460: 386A1FF0  addi r3, r10, 0x1ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 8176;
	// 8264B464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B468: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264B46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B484: 4BE1B99D  bl 0x82466e20
	ctx.lr = 0x8264B488;
	sub_82466E20(ctx, base);
	// 8264B488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B498 size=108
    let mut pc: u32 = 0x8264B498;
    'dispatch: loop {
        match pc {
            0x8264B498 => {
    //   block [0x8264B498..0x8264B504)
	// 8264B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B4A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B4A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B4AC: 38EB2C10  addi r7, r11, 0x2c10
	ctx.r[7].s64 = ctx.r[11].s64 + 11280;
	// 8264B4B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B4B4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8264B4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B4BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B4C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B4C8: 386A2020  addi r3, r10, 0x2020
	ctx.r[3].s64 = ctx.r[10].s64 + 8224;
	// 8264B4CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B4EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B4F0: 4BE1B931  bl 0x82466e20
	ctx.lr = 0x8264B4F4;
	sub_82466E20(ctx, base);
	// 8264B4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B508 size=108
    let mut pc: u32 = 0x8264B508;
    'dispatch: loop {
        match pc {
            0x8264B508 => {
    //   block [0x8264B508..0x8264B574)
	// 8264B508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B514: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B518: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B51C: 38EB2C40  addi r7, r11, 0x2c40
	ctx.r[7].s64 = ctx.r[11].s64 + 11328;
	// 8264B520: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B524: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8264B528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B52C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B538: 386A2050  addi r3, r10, 0x2050
	ctx.r[3].s64 = ctx.r[10].s64 + 8272;
	// 8264B53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B560: 4BE1B8C1  bl 0x82466e20
	ctx.lr = 0x8264B564;
	sub_82466E20(ctx, base);
	// 8264B564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B578 size=112
    let mut pc: u32 = 0x8264B578;
    'dispatch: loop {
        match pc {
            0x8264B578 => {
    //   block [0x8264B578..0x8264B5E8)
	// 8264B578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B584: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B588: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B58C: 392A9CB8  addi r9, r10, -0x6348
	ctx.r[9].s64 = ctx.r[10].s64 + -25416;
	// 8264B590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B594: 390B2C78  addi r8, r11, 0x2c78
	ctx.r[8].s64 = ctx.r[11].s64 + 11384;
	// 8264B598: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264B59C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8264B5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B5A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B5B0: 386A2080  addi r3, r10, 0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + 8320;
	// 8264B5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B5D4: 4BE1B84D  bl 0x82466e20
	ctx.lr = 0x8264B5D8;
	sub_82466E20(ctx, base);
	// 8264B5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B5E8 size=108
    let mut pc: u32 = 0x8264B5E8;
    'dispatch: loop {
        match pc {
            0x8264B5E8 => {
    //   block [0x8264B5E8..0x8264B654)
	// 8264B5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B5F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B5F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B5FC: 38EB2CD8  addi r7, r11, 0x2cd8
	ctx.r[7].s64 = ctx.r[11].s64 + 11480;
	// 8264B600: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8264B604: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8264B608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B60C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B618: 386A20B0  addi r3, r10, 0x20b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8368;
	// 8264B61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B640: 4BE1B7E1  bl 0x82466e20
	ctx.lr = 0x8264B644;
	sub_82466E20(ctx, base);
	// 8264B644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B658 size=108
    let mut pc: u32 = 0x8264B658;
    'dispatch: loop {
        match pc {
            0x8264B658 => {
    //   block [0x8264B658..0x8264B6C4)
	// 8264B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B664: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B668: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B66C: 38EB2DE0  addi r7, r11, 0x2de0
	ctx.r[7].s64 = ctx.r[11].s64 + 11744;
	// 8264B670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B674: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8264B678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B67C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B688: 386A20E0  addi r3, r10, 0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8416;
	// 8264B68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B6B0: 4BE1B771  bl 0x82466e20
	ctx.lr = 0x8264B6B4;
	sub_82466E20(ctx, base);
	// 8264B6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B6C8 size=108
    let mut pc: u32 = 0x8264B6C8;
    'dispatch: loop {
        match pc {
            0x8264B6C8 => {
    //   block [0x8264B6C8..0x8264B734)
	// 8264B6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B6D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B6DC: 38EB2DF8  addi r7, r11, 0x2df8
	ctx.r[7].s64 = ctx.r[11].s64 + 11768;
	// 8264B6E0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8264B6E4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8264B6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B6EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B6F8: 386A2110  addi r3, r10, 0x2110
	ctx.r[3].s64 = ctx.r[10].s64 + 8464;
	// 8264B6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B720: 4BE1B701  bl 0x82466e20
	ctx.lr = 0x8264B724;
	sub_82466E20(ctx, base);
	// 8264B724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B738 size=24
    let mut pc: u32 = 0x8264B738;
    'dispatch: loop {
        match pc {
            0x8264B738 => {
    //   block [0x8264B738..0x8264B750)
	// 8264B738: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B73C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B740: 394A7DF8  addi r10, r10, 0x7df8
	ctx.r[10].s64 = ctx.r[10].s64 + 32248;
	// 8264B744: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B748: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B750 size=108
    let mut pc: u32 = 0x8264B750;
    'dispatch: loop {
        match pc {
            0x8264B750 => {
    //   block [0x8264B750..0x8264B7BC)
	// 8264B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B75C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B764: 38EB7DF8  addi r7, r11, 0x7df8
	ctx.r[7].s64 = ctx.r[11].s64 + 32248;
	// 8264B768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B76C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8264B770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B774: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B780: 386A2140  addi r3, r10, 0x2140
	ctx.r[3].s64 = ctx.r[10].s64 + 8512;
	// 8264B784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B7A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B7A8: 4BE1B679  bl 0x82466e20
	ctx.lr = 0x8264B7AC;
	sub_82466E20(ctx, base);
	// 8264B7AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B7C0 size=24
    let mut pc: u32 = 0x8264B7C0;
    'dispatch: loop {
        match pc {
            0x8264B7C0 => {
    //   block [0x8264B7C0..0x8264B7D8)
	// 8264B7C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B7C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B7C8: 394A7E28  addi r10, r10, 0x7e28
	ctx.r[10].s64 = ctx.r[10].s64 + 32296;
	// 8264B7CC: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B7D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B7D8 size=108
    let mut pc: u32 = 0x8264B7D8;
    'dispatch: loop {
        match pc {
            0x8264B7D8 => {
    //   block [0x8264B7D8..0x8264B844)
	// 8264B7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B7E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B7EC: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 8264B7F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B7F4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8264B7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B7FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B808: 386A2170  addi r3, r10, 0x2170
	ctx.r[3].s64 = ctx.r[10].s64 + 8560;
	// 8264B80C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B830: 4BE1B5F1  bl 0x82466e20
	ctx.lr = 0x8264B834;
	sub_82466E20(ctx, base);
	// 8264B834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B848 size=108
    let mut pc: u32 = 0x8264B848;
    'dispatch: loop {
        match pc {
            0x8264B848 => {
    //   block [0x8264B848..0x8264B8B4)
	// 8264B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B854: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B85C: 38EB2E70  addi r7, r11, 0x2e70
	ctx.r[7].s64 = ctx.r[11].s64 + 11888;
	// 8264B860: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B864: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8264B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B86C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B878: 386A21A0  addi r3, r10, 0x21a0
	ctx.r[3].s64 = ctx.r[10].s64 + 8608;
	// 8264B87C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B89C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B8A0: 4BE1B581  bl 0x82466e20
	ctx.lr = 0x8264B8A4;
	sub_82466E20(ctx, base);
	// 8264B8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B8B8 size=24
    let mut pc: u32 = 0x8264B8B8;
    'dispatch: loop {
        match pc {
            0x8264B8B8 => {
    //   block [0x8264B8B8..0x8264B8D0)
	// 8264B8B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B8BC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B8C0: 394A7E58  addi r10, r10, 0x7e58
	ctx.r[10].s64 = ctx.r[10].s64 + 32344;
	// 8264B8C4: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B8C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B8D0 size=108
    let mut pc: u32 = 0x8264B8D0;
    'dispatch: loop {
        match pc {
            0x8264B8D0 => {
    //   block [0x8264B8D0..0x8264B93C)
	// 8264B8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B8DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B8E4: 38EB7E58  addi r7, r11, 0x7e58
	ctx.r[7].s64 = ctx.r[11].s64 + 32344;
	// 8264B8E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B8EC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8264B8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B8F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B900: 386A21D0  addi r3, r10, 0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + 8656;
	// 8264B904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B928: 4BE1B4F9  bl 0x82466e20
	ctx.lr = 0x8264B92C;
	sub_82466E20(ctx, base);
	// 8264B92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B940 size=112
    let mut pc: u32 = 0x8264B940;
    'dispatch: loop {
        match pc {
            0x8264B940 => {
    //   block [0x8264B940..0x8264B9B0)
	// 8264B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B94C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B950: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B954: 392A9CFC  addi r9, r10, -0x6304
	ctx.r[9].s64 = ctx.r[10].s64 + -25348;
	// 8264B958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B95C: 390B2E88  addi r8, r11, 0x2e88
	ctx.r[8].s64 = ctx.r[11].s64 + 11912;
	// 8264B960: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8264B964: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8264B968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B96C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B978: 386A2200  addi r3, r10, 0x2200
	ctx.r[3].s64 = ctx.r[10].s64 + 8704;
	// 8264B97C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B980: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B99C: 4BE1B485  bl 0x82466e20
	ctx.lr = 0x8264B9A0;
	sub_82466E20(ctx, base);
	// 8264B9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B9B0 size=108
    let mut pc: u32 = 0x8264B9B0;
    'dispatch: loop {
        match pc {
            0x8264B9B0 => {
    //   block [0x8264B9B0..0x8264BA1C)
	// 8264B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B9BC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B9C4: 38EB2EB8  addi r7, r11, 0x2eb8
	ctx.r[7].s64 = ctx.r[11].s64 + 11960;
	// 8264B9C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B9CC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8264B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B9D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B9E0: 386A2230  addi r3, r10, 0x2230
	ctx.r[3].s64 = ctx.r[10].s64 + 8752;
	// 8264B9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BA08: 4BE1B419  bl 0x82466e20
	ctx.lr = 0x8264BA0C;
	sub_82466E20(ctx, base);
	// 8264BA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BA20 size=108
    let mut pc: u32 = 0x8264BA20;
    'dispatch: loop {
        match pc {
            0x8264BA20 => {
    //   block [0x8264BA20..0x8264BA8C)
	// 8264BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BA2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BA34: 38EB2EE8  addi r7, r11, 0x2ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 12008;
	// 8264BA38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BA3C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8264BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BA44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BA50: 386A2260  addi r3, r10, 0x2260
	ctx.r[3].s64 = ctx.r[10].s64 + 8800;
	// 8264BA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BA78: 4BE1B3A9  bl 0x82466e20
	ctx.lr = 0x8264BA7C;
	sub_82466E20(ctx, base);
	// 8264BA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BA90 size=112
    let mut pc: u32 = 0x8264BA90;
    'dispatch: loop {
        match pc {
            0x8264BA90 => {
    //   block [0x8264BA90..0x8264BB00)
	// 8264BA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BA9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BAA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BAA4: 38AA22C0  addi r5, r10, 0x22c0
	ctx.r[5].s64 = ctx.r[10].s64 + 8896;
	// 8264BAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BAAC: 390B2F18  addi r8, r11, 0x2f18
	ctx.r[8].s64 = ctx.r[11].s64 + 12056;
	// 8264BAB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264BAB4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8264BAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BAC8: 386A2290  addi r3, r10, 0x2290
	ctx.r[3].s64 = ctx.r[10].s64 + 8848;
	// 8264BACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264BAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BAEC: 4BE1B335  bl 0x82466e20
	ctx.lr = 0x8264BAF0;
	sub_82466E20(ctx, base);
	// 8264BAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BB00 size=108
    let mut pc: u32 = 0x8264BB00;
    'dispatch: loop {
        match pc {
            0x8264BB00 => {
    //   block [0x8264BB00..0x8264BB6C)
	// 8264BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BB0C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BB14: 38EB2F30  addi r7, r11, 0x2f30
	ctx.r[7].s64 = ctx.r[11].s64 + 12080;
	// 8264BB18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BB1C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8264BB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BB24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BB30: 386A22C0  addi r3, r10, 0x22c0
	ctx.r[3].s64 = ctx.r[10].s64 + 8896;
	// 8264BB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BB58: 4BE1B2C9  bl 0x82466e20
	ctx.lr = 0x8264BB5C;
	sub_82466E20(ctx, base);
	// 8264BB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BB70 size=108
    let mut pc: u32 = 0x8264BB70;
    'dispatch: loop {
        match pc {
            0x8264BB70 => {
    //   block [0x8264BB70..0x8264BBDC)
	// 8264BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BB7C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BB84: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 8264BB88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264BB8C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8264BB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BB94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BBA0: 386A22F0  addi r3, r10, 0x22f0
	ctx.r[3].s64 = ctx.r[10].s64 + 8944;
	// 8264BBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BBC8: 4BE1B259  bl 0x82466e20
	ctx.lr = 0x8264BBCC;
	sub_82466E20(ctx, base);
	// 8264BBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BBE0 size=108
    let mut pc: u32 = 0x8264BBE0;
    'dispatch: loop {
        match pc {
            0x8264BBE0 => {
    //   block [0x8264BBE0..0x8264BC4C)
	// 8264BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BBEC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BBF4: 38EB2F78  addi r7, r11, 0x2f78
	ctx.r[7].s64 = ctx.r[11].s64 + 12152;
	// 8264BBF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BBFC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8264BC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BC04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BC10: 386A2320  addi r3, r10, 0x2320
	ctx.r[3].s64 = ctx.r[10].s64 + 8992;
	// 8264BC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BC38: 4BE1B1E9  bl 0x82466e20
	ctx.lr = 0x8264BC3C;
	sub_82466E20(ctx, base);
	// 8264BC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BC50 size=108
    let mut pc: u32 = 0x8264BC50;
    'dispatch: loop {
        match pc {
            0x8264BC50 => {
    //   block [0x8264BC50..0x8264BCBC)
	// 8264BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BC5C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BC64: 38EB2FA8  addi r7, r11, 0x2fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 12200;
	// 8264BC68: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8264BC6C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8264BC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BC74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BC78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BC80: 386A2350  addi r3, r10, 0x2350
	ctx.r[3].s64 = ctx.r[10].s64 + 9040;
	// 8264BC84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BCA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BCA8: 4BE1B179  bl 0x82466e20
	ctx.lr = 0x8264BCAC;
	sub_82466E20(ctx, base);
	// 8264BCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BCC0 size=108
    let mut pc: u32 = 0x8264BCC0;
    'dispatch: loop {
        match pc {
            0x8264BCC0 => {
    //   block [0x8264BCC0..0x8264BD2C)
	// 8264BCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BCCC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BCD4: 38EB3050  addi r7, r11, 0x3050
	ctx.r[7].s64 = ctx.r[11].s64 + 12368;
	// 8264BCD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BCDC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8264BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BCE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BCE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BCF0: 386A2380  addi r3, r10, 0x2380
	ctx.r[3].s64 = ctx.r[10].s64 + 9088;
	// 8264BCF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BD18: 4BE1B109  bl 0x82466e20
	ctx.lr = 0x8264BD1C;
	sub_82466E20(ctx, base);
	// 8264BD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BD30 size=108
    let mut pc: u32 = 0x8264BD30;
    'dispatch: loop {
        match pc {
            0x8264BD30 => {
    //   block [0x8264BD30..0x8264BD9C)
	// 8264BD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BD3C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BD44: 38EB3080  addi r7, r11, 0x3080
	ctx.r[7].s64 = ctx.r[11].s64 + 12416;
	// 8264BD48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264BD4C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8264BD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BD54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BD60: 386A23B0  addi r3, r10, 0x23b0
	ctx.r[3].s64 = ctx.r[10].s64 + 9136;
	// 8264BD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BD88: 4BE1B099  bl 0x82466e20
	ctx.lr = 0x8264BD8C;
	sub_82466E20(ctx, base);
	// 8264BD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BDA0 size=108
    let mut pc: u32 = 0x8264BDA0;
    'dispatch: loop {
        match pc {
            0x8264BDA0 => {
    //   block [0x8264BDA0..0x8264BE0C)
	// 8264BDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BDAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BDB4: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 8264BDB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BDBC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8264BDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BDC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BDD0: 386A23E0  addi r3, r10, 0x23e0
	ctx.r[3].s64 = ctx.r[10].s64 + 9184;
	// 8264BDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BDF8: 4BE1B029  bl 0x82466e20
	ctx.lr = 0x8264BDFC;
	sub_82466E20(ctx, base);
	// 8264BDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BE10 size=108
    let mut pc: u32 = 0x8264BE10;
    'dispatch: loop {
        match pc {
            0x8264BE10 => {
    //   block [0x8264BE10..0x8264BE7C)
	// 8264BE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BE1C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BE24: 38EB30C8  addi r7, r11, 0x30c8
	ctx.r[7].s64 = ctx.r[11].s64 + 12488;
	// 8264BE28: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8264BE2C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8264BE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BE34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BE38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BE40: 386A2410  addi r3, r10, 0x2410
	ctx.r[3].s64 = ctx.r[10].s64 + 9232;
	// 8264BE44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BE64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BE68: 4BE1AFB9  bl 0x82466e20
	ctx.lr = 0x8264BE6C;
	sub_82466E20(ctx, base);
	// 8264BE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264BE80 size=24
    let mut pc: u32 = 0x8264BE80;
    'dispatch: loop {
        match pc {
            0x8264BE80 => {
    //   block [0x8264BE80..0x8264BE98)
	// 8264BE80: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BE84: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264BE88: 394A7E88  addi r10, r10, 0x7e88
	ctx.r[10].s64 = ctx.r[10].s64 + 32392;
	// 8264BE8C: 816B3188  lwz r11, 0x3188(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12680 as u32) ) } as u64;
	// 8264BE90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BE98 size=112
    let mut pc: u32 = 0x8264BE98;
    'dispatch: loop {
        match pc {
            0x8264BE98 => {
    //   block [0x8264BE98..0x8264BF08)
	// 8264BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BEA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264BEA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BEAC: 392A9D28  addi r9, r10, -0x62d8
	ctx.r[9].s64 = ctx.r[10].s64 + -25304;
	// 8264BEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BEB4: 390B7E88  addi r8, r11, 0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + 32392;
	// 8264BEB8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264BEBC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8264BEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BEC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BED0: 386A2440  addi r3, r10, 0x2440
	ctx.r[3].s64 = ctx.r[10].s64 + 9280;
	// 8264BED4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264BED8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264BEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BEF4: 4BE1AF2D  bl 0x82466e20
	ctx.lr = 0x8264BEF8;
	sub_82466E20(ctx, base);
	// 8264BEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BF08 size=108
    let mut pc: u32 = 0x8264BF08;
    'dispatch: loop {
        match pc {
            0x8264BF08 => {
    //   block [0x8264BF08..0x8264BF74)
	// 8264BF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BF14: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BF1C: 38EB3190  addi r7, r11, 0x3190
	ctx.r[7].s64 = ctx.r[11].s64 + 12688;
	// 8264BF20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BF24: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8264BF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BF2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BF38: 386A2470  addi r3, r10, 0x2470
	ctx.r[3].s64 = ctx.r[10].s64 + 9328;
	// 8264BF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BF60: 4BE1AEC1  bl 0x82466e20
	ctx.lr = 0x8264BF64;
	sub_82466E20(ctx, base);
	// 8264BF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BF78 size=112
    let mut pc: u32 = 0x8264BF78;
    'dispatch: loop {
        match pc {
            0x8264BF78 => {
    //   block [0x8264BF78..0x8264BFE8)
	// 8264BF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BF84: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264BF88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BF8C: 392A9D6C  addi r9, r10, -0x6294
	ctx.r[9].s64 = ctx.r[10].s64 + -25236;
	// 8264BF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BF94: 390B31C0  addi r8, r11, 0x31c0
	ctx.r[8].s64 = ctx.r[11].s64 + 12736;
	// 8264BF98: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264BF9C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8264BFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BFA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BFB0: 386A24A0  addi r3, r10, 0x24a0
	ctx.r[3].s64 = ctx.r[10].s64 + 9376;
	// 8264BFB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264BFB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264BFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BFD4: 4BE1AE4D  bl 0x82466e20
	ctx.lr = 0x8264BFD8;
	sub_82466E20(ctx, base);
	// 8264BFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264BFE8 size=24
    let mut pc: u32 = 0x8264BFE8;
    'dispatch: loop {
        match pc {
            0x8264BFE8 => {
    //   block [0x8264BFE8..0x8264C000)
	// 8264BFE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BFEC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264BFF0: 394A7F00  addi r10, r10, 0x7f00
	ctx.r[10].s64 = ctx.r[10].s64 + 32512;
	// 8264BFF4: 816B3280  lwz r11, 0x3280(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12928 as u32) ) } as u64;
	// 8264BFF8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264BFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C000 size=112
    let mut pc: u32 = 0x8264C000;
    'dispatch: loop {
        match pc {
            0x8264C000 => {
    //   block [0x8264C000..0x8264C070)
	// 8264C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C00C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264C010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C014: 392A9DA8  addi r9, r10, -0x6258
	ctx.r[9].s64 = ctx.r[10].s64 + -25176;
	// 8264C018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C01C: 390B7F00  addi r8, r11, 0x7f00
	ctx.r[8].s64 = ctx.r[11].s64 + 32512;
	// 8264C020: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264C024: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8264C028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C02C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264C034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C038: 386A24D0  addi r3, r10, 0x24d0
	ctx.r[3].s64 = ctx.r[10].s64 + 9424;
	// 8264C03C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264C040: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264C044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C05C: 4BE1ADC5  bl 0x82466e20
	ctx.lr = 0x8264C060;
	sub_82466E20(ctx, base);
	// 8264C060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C070 size=108
    let mut pc: u32 = 0x8264C070;
    'dispatch: loop {
        match pc {
            0x8264C070 => {
    //   block [0x8264C070..0x8264C0DC)
	// 8264C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C07C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C084: 38EB3284  addi r7, r11, 0x3284
	ctx.r[7].s64 = ctx.r[11].s64 + 12932;
	// 8264C088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C08C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8264C090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C0A0: 386A2500  addi r3, r10, 0x2500
	ctx.r[3].s64 = ctx.r[10].s64 + 9472;
	// 8264C0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C0C8: 4BE1AD59  bl 0x82466e20
	ctx.lr = 0x8264C0CC;
	sub_82466E20(ctx, base);
	// 8264C0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C0E0 size=108
    let mut pc: u32 = 0x8264C0E0;
    'dispatch: loop {
        match pc {
            0x8264C0E0 => {
    //   block [0x8264C0E0..0x8264C14C)
	// 8264C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C0EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C0F4: 38EB329C  addi r7, r11, 0x329c
	ctx.r[7].s64 = ctx.r[11].s64 + 12956;
	// 8264C0F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264C0FC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8264C100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C110: 386A2530  addi r3, r10, 0x2530
	ctx.r[3].s64 = ctx.r[10].s64 + 9520;
	// 8264C114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C138: 4BE1ACE9  bl 0x82466e20
	ctx.lr = 0x8264C13C;
	sub_82466E20(ctx, base);
	// 8264C13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264C150 size=24
    let mut pc: u32 = 0x8264C150;
    'dispatch: loop {
        match pc {
            0x8264C150 => {
    //   block [0x8264C150..0x8264C168)
	// 8264C150: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C154: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264C158: 394A7F48  addi r10, r10, 0x7f48
	ctx.r[10].s64 = ctx.r[10].s64 + 32584;
	// 8264C15C: 816B32CC  lwz r11, 0x32cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13004 as u32) ) } as u64;
	// 8264C160: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264C164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C168 size=112
    let mut pc: u32 = 0x8264C168;
    'dispatch: loop {
        match pc {
            0x8264C168 => {
    //   block [0x8264C168..0x8264C1D8)
	// 8264C168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C174: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264C178: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C17C: 392A9DE4  addi r9, r10, -0x621c
	ctx.r[9].s64 = ctx.r[10].s64 + -25116;
	// 8264C180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C184: 390B7F48  addi r8, r11, 0x7f48
	ctx.r[8].s64 = ctx.r[11].s64 + 32584;
	// 8264C188: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264C18C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8264C190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264C19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C1A0: 386A2560  addi r3, r10, 0x2560
	ctx.r[3].s64 = ctx.r[10].s64 + 9568;
	// 8264C1A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264C1A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264C1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C1C4: 4BE1AC5D  bl 0x82466e20
	ctx.lr = 0x8264C1C8;
	sub_82466E20(ctx, base);
	// 8264C1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C1D8 size=108
    let mut pc: u32 = 0x8264C1D8;
    'dispatch: loop {
        match pc {
            0x8264C1D8 => {
    //   block [0x8264C1D8..0x8264C244)
	// 8264C1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C1E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C1EC: 38EB32D0  addi r7, r11, 0x32d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13008;
	// 8264C1F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C1F4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8264C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C208: 386A2590  addi r3, r10, 0x2590
	ctx.r[3].s64 = ctx.r[10].s64 + 9616;
	// 8264C20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C230: 4BE1ABF1  bl 0x82466e20
	ctx.lr = 0x8264C234;
	sub_82466E20(ctx, base);
	// 8264C234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C248 size=108
    let mut pc: u32 = 0x8264C248;
    'dispatch: loop {
        match pc {
            0x8264C248 => {
    //   block [0x8264C248..0x8264C2B4)
	// 8264C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C254: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C25C: 38EB32E8  addi r7, r11, 0x32e8
	ctx.r[7].s64 = ctx.r[11].s64 + 13032;
	// 8264C260: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264C264: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8264C268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C278: 386A25C0  addi r3, r10, 0x25c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9664;
	// 8264C27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C2A0: 4BE1AB81  bl 0x82466e20
	ctx.lr = 0x8264C2A4;
	sub_82466E20(ctx, base);
	// 8264C2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C2B8 size=108
    let mut pc: u32 = 0x8264C2B8;
    'dispatch: loop {
        match pc {
            0x8264C2B8 => {
    //   block [0x8264C2B8..0x8264C324)
	// 8264C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C2C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C2CC: 38EB3330  addi r7, r11, 0x3330
	ctx.r[7].s64 = ctx.r[11].s64 + 13104;
	// 8264C2D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264C2D4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8264C2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C2DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C2E8: 386A25F0  addi r3, r10, 0x25f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9712;
	// 8264C2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C310: 4BE1AB11  bl 0x82466e20
	ctx.lr = 0x8264C314;
	sub_82466E20(ctx, base);
	// 8264C314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C328 size=108
    let mut pc: u32 = 0x8264C328;
    'dispatch: loop {
        match pc {
            0x8264C328 => {
    //   block [0x8264C328..0x8264C394)
	// 8264C328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C334: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C33C: 38EB3360  addi r7, r11, 0x3360
	ctx.r[7].s64 = ctx.r[11].s64 + 13152;
	// 8264C340: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8264C344: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8264C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C34C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C358: 386A2620  addi r3, r10, 0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + 9760;
	// 8264C35C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C37C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C380: 4BE1AAA1  bl 0x82466e20
	ctx.lr = 0x8264C384;
	sub_82466E20(ctx, base);
	// 8264C384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C398 size=108
    let mut pc: u32 = 0x8264C398;
    'dispatch: loop {
        match pc {
            0x8264C398 => {
    //   block [0x8264C398..0x8264C404)
	// 8264C398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C3A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C3A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C3AC: 38EB3480  addi r7, r11, 0x3480
	ctx.r[7].s64 = ctx.r[11].s64 + 13440;
	// 8264C3B0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264C3B4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8264C3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C3BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C3C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C3C8: 386A2650  addi r3, r10, 0x2650
	ctx.r[3].s64 = ctx.r[10].s64 + 9808;
	// 8264C3CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C3D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C3D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C3E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C3E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C3EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C3F0: 4BE1AA31  bl 0x82466e20
	ctx.lr = 0x8264C3F4;
	sub_82466E20(ctx, base);
	// 8264C3F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C408 size=108
    let mut pc: u32 = 0x8264C408;
    'dispatch: loop {
        match pc {
            0x8264C408 => {
    //   block [0x8264C408..0x8264C474)
	// 8264C408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C414: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C41C: 38EB3510  addi r7, r11, 0x3510
	ctx.r[7].s64 = ctx.r[11].s64 + 13584;
	// 8264C420: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8264C424: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8264C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C42C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C438: 386A2680  addi r3, r10, 0x2680
	ctx.r[3].s64 = ctx.r[10].s64 + 9856;
	// 8264C43C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C460: 4BE1A9C1  bl 0x82466e20
	ctx.lr = 0x8264C464;
	sub_82466E20(ctx, base);
	// 8264C464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C478 size=108
    let mut pc: u32 = 0x8264C478;
    'dispatch: loop {
        match pc {
            0x8264C478 => {
    //   block [0x8264C478..0x8264C4E4)
	// 8264C478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C484: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C48C: 38EB35D0  addi r7, r11, 0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13776;
	// 8264C490: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8264C494: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8264C498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C49C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C4A8: 386A26B0  addi r3, r10, 0x26b0
	ctx.r[3].s64 = ctx.r[10].s64 + 9904;
	// 8264C4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C4D0: 4BE1A951  bl 0x82466e20
	ctx.lr = 0x8264C4D4;
	sub_82466E20(ctx, base);
	// 8264C4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


