pub fn sub_83033E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83033E20 size=124
    let mut pc: u32 = 0x83033E20;
    'dispatch: loop {
        match pc {
            0x83033E20 => {
    //   block [0x83033E20..0x83033E9C)
	// 83033E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83033E24: 48174341  bl 0x831a8164
	ctx.lr = 0x83033E28;
	sub_831A8130(ctx, base);
	// 83033E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83033E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83033E30: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83033E34: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 83033E38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033E3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83033E40: 40990050  ble cr6, 0x83033e90
	if !ctx.cr[6].gt {
	pc = 0x83033E90; continue 'dispatch;
	}
	// 83033E44: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 83033E48: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83033E4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033E50: 41820024  beq 0x83033e74
	if ctx.cr[0].eq {
	pc = 0x83033E74; continue 'dispatch;
	}
	// 83033E54: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033E58: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83033E5C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033E60: 41820014  beq 0x83033e74
	if ctx.cr[0].eq {
	pc = 0x83033E74; continue 'dispatch;
	}
	// 83033E64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83033E68: 4BFC6C89  bl 0x82ffaaf0
	ctx.lr = 0x83033E6C;
	sub_82FFAAF0(ctx, base);
	// 83033E6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83033E70: 4BFA4471  bl 0x82fd82e0
	ctx.lr = 0x83033E74;
	sub_82FD82E0(ctx, base);
	// 83033E74: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033E78: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83033E7C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 83033E80: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83033E84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033E88: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83033E8C: 4198FFBC  blt cr6, 0x83033e48
	if ctx.cr[6].lt {
	pc = 0x83033E48; continue 'dispatch;
	}
	// 83033E90: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83033E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83033E98: 4817431C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83033EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83033EA0 size=260
    let mut pc: u32 = 0x83033EA0;
    'dispatch: loop {
        match pc {
            0x83033EA0 => {
    //   block [0x83033EA0..0x83033FA4)
	// 83033EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83033EA4: 481742C9  bl 0x831a816c
	ctx.lr = 0x83033EA8;
	sub_831A8130(ctx, base);
	// 83033EA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83033EAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83033EB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83033EB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033EB8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83033EBC: 41980030  blt cr6, 0x83033eec
	if ctx.cr[6].lt {
	pc = 0x83033EEC; continue 'dispatch;
	}
	// 83033EC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83033EC4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83033EC8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 83033ECC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 83033ED0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 83033ED4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83033ED8: 4BF9CA81  bl 0x82fd0958
	ctx.lr = 0x83033EDC;
	sub_82FD0958(ctx, base);
	// 83033EDC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83033EE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83033EE4: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83033EE8: 4817CD41  bl 0x831b0c28
	ctx.lr = 0x83033EEC;
	sub_831B0C28(ctx, base);
	// 83033EEC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83033EF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033EF4: 41820028  beq 0x83033f1c
	if ctx.cr[0].eq {
	pc = 0x83033F1C; continue 'dispatch;
	}
	// 83033EF8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033EFC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83033F00: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83033F04: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033F08: 41820014  beq 0x83033f1c
	if ctx.cr[0].eq {
	pc = 0x83033F1C; continue 'dispatch;
	}
	// 83033F0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83033F10: 4BFC6BE1  bl 0x82ffaaf0
	ctx.lr = 0x83033F14;
	sub_82FFAAF0(ctx, base);
	// 83033F14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83033F18: 4BFA43C9  bl 0x82fd82e0
	ctx.lr = 0x83033F1C;
	sub_82FD82E0(ctx, base);
	// 83033F1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033F20: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83033F24: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83033F28: 409A0018  bne cr6, 0x83033f40
	if !ctx.cr[6].eq {
	pc = 0x83033F40; continue 'dispatch;
	}
	// 83033F2C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033F30: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83033F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83033F38: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 83033F3C: 48000054  b 0x83033f90
	pc = 0x83033F90; continue 'dispatch;
	// 83033F40: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83033F44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83033F48: 40980030  bge cr6, 0x83033f78
	if !ctx.cr[6].lt {
	pc = 0x83033F78; continue 'dispatch;
	}
	// 83033F4C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83033F50: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033F54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83033F58: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83033F5C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83033F60: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83033F64: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83033F68: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033F6C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83033F70: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83033F74: 4198FFDC  blt cr6, 0x83033f50
	if ctx.cr[6].lt {
	pc = 0x83033F50; continue 'dispatch;
	}
	// 83033F78: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83033F80: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033F84: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83033F88: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83033F8C: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 83033F90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033F94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83033F98: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83033F9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83033FA0: 4817421C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83033FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83033FA8 size=104
    let mut pc: u32 = 0x83033FA8;
    'dispatch: loop {
        match pc {
            0x83033FA8 => {
    //   block [0x83033FA8..0x83034010)
	// 83033FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83033FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83033FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83033FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83033FB8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83033FBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033FC0: 4182003C  beq 0x83033ffc
	if ctx.cr[0].eq {
	pc = 0x83033FFC; continue 'dispatch;
	}
	// 83033FC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83033FC8: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83033FCC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033FD0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83033FD4: 41820028  beq 0x83033ffc
	if ctx.cr[0].eq {
	pc = 0x83033FFC; continue 'dispatch;
	}
	// 83033FD8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83033FDC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83033FE0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83033FE4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83033FE8: 41820014  beq 0x83033ffc
	if ctx.cr[0].eq {
	pc = 0x83033FFC; continue 'dispatch;
	}
	// 83033FEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83033FF0: 4BFC6B01  bl 0x82ffaaf0
	ctx.lr = 0x83033FF4;
	sub_82FFAAF0(ctx, base);
	// 83033FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83033FF8: 4BFA42E9  bl 0x82fd82e0
	ctx.lr = 0x83033FFC;
	sub_82FD82E0(ctx, base);
	// 83033FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034010 size=132
    let mut pc: u32 = 0x83034010;
    'dispatch: loop {
        match pc {
            0x83034010 => {
    //   block [0x83034010..0x83034094)
	// 83034010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034014: 48174155  bl 0x831a8168
	ctx.lr = 0x83034018;
	sub_831A8130(ctx, base);
	// 83034018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303401C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83034020: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034024: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034028: 4182004C  beq 0x83034074
	if ctx.cr[0].eq {
	pc = 0x83034074; continue 'dispatch;
	}
	// 8303402C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034030: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83034034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83034038: 4099003C  ble cr6, 0x83034074
	if !ctx.cr[6].gt {
	pc = 0x83034074; continue 'dispatch;
	}
	// 8303403C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83034040: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83034044: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83034048: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303404C: 41820014  beq 0x83034060
	if ctx.cr[0].eq {
	pc = 0x83034060; continue 'dispatch;
	}
	// 83034050: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83034054: 4BFC6A9D  bl 0x82ffaaf0
	ctx.lr = 0x83034058;
	sub_82FFAAF0(ctx, base);
	// 83034058: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303405C: 4BFA4285  bl 0x82fd82e0
	ctx.lr = 0x83034060;
	sub_82FD82E0(ctx, base);
	// 83034060: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034064: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83034068: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8303406C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83034070: 4198FFD0  blt cr6, 0x83034040
	if ctx.cr[6].lt {
	pc = 0x83034040; continue 'dispatch;
	}
	// 83034074: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034078: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303407C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034080: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034088: 4E800421  bctrl
	ctx.lr = 0x8303408C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303408C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83034090: 48174128  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034098 size=76
    let mut pc: u32 = 0x83034098;
    'dispatch: loop {
        match pc {
            0x83034098 => {
    //   block [0x83034098..0x830340E4)
	// 83034098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303409C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830340A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830340A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830340A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830340AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830340B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830340B4: 4BFFFC05  bl 0x83033cb8
	ctx.lr = 0x830340B8;
	sub_83033CB8(ctx, base);
	// 830340B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830340BC: 4182000C  beq 0x830340c8
	if ctx.cr[0].eq {
	pc = 0x830340C8; continue 'dispatch;
	}
	// 830340C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830340C4: 4BFA421D  bl 0x82fd82e0
	ctx.lr = 0x830340C8;
	sub_82FD82E0(ctx, base);
	// 830340C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830340CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830340D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830340D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830340D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830340DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830340E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830340E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830340E8 size=8
    let mut pc: u32 = 0x830340E8;
    'dispatch: loop {
        match pc {
            0x830340E8 => {
    //   block [0x830340E8..0x830340F0)
	// 830340E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830340EC: 82148A38  lwz r16, -0x75c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-30152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830340F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830340F0 size=264
    let mut pc: u32 = 0x830340F0;
    'dispatch: loop {
        match pc {
            0x830340F0 => {
    //   block [0x830340F0..0x830341F8)
	// 830340F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830340F4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830340F8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830340FC: 4817406D  bl 0x831a8168
	ctx.lr = 0x83034100;
	sub_831A8130(ctx, base);
	// 83034100: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83034104: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034108: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303410C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83034110: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83034114: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83034118: 93BF009C  stw r29, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 8303411C: 939F00A4  stw r28, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 83034120: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83034124: 4BFFD0DD  bl 0x83031200
	ctx.lr = 0x83034128;
	sub_83031200(ctx, base);
	// 83034128: 38604048  li r3, 0x4048
	ctx.r[3].s64 = 16456;
	// 8303412C: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83034130: 4BFA4169  bl 0x82fd8298
	ctx.lr = 0x83034134;
	sub_82FD8298(ctx, base);
	// 83034134: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034138: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303413C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034140: 4182002C  beq 0x8303416c
	if ctx.cr[0].eq {
	pc = 0x8303416C; continue 'dispatch;
	}
	// 83034144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83034148: 813E0034  lwz r9, 0x34(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303414C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83034150: 80BE0018  lwz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83034154: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83034158: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303415C: 4804AC5D  bl 0x8307edb8
	ctx.lr = 0x83034160;
	sub_8307EDB8(ctx, base);
	// 83034160: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83034164: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034168: 48000008  b 0x83034170
	pc = 0x83034170; continue 'dispatch;
	// 8303416C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83034170: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83034174: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83034178: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303417C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83034180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83034184: 4BFFE395  bl 0x83032518
	ctx.lr = 0x83034188;
	sub_83032518(ctx, base);
	// 83034188: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8303418C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034190: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034194: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034198: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303419C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830341A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830341A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830341A8: 4E800421  bctrl
	ctx.lr = 0x830341AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830341AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341BC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830341C0: 4BFFE2F9  bl 0x830324b8
	ctx.lr = 0x830341C4;
	sub_830324B8(ctx, base);
	// 830341C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830341D4: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830341D8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830341DC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830341E0: 48000010  b 0x830341f0
	pc = 0x830341F0; continue 'dispatch;
	// 830341E4: 48000008  b 0x830341ec
	pc = 0x830341EC; continue 'dispatch;
	// 830341E8: 48000004  b 0x830341ec
	pc = 0x830341EC; continue 'dispatch;
	// 830341EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830341F0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830341F4: 48173FC4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830341F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830341F8 size=8
    let mut pc: u32 = 0x830341F8;
    'dispatch: loop {
        match pc {
            0x830341F8 => {
    //   block [0x830341F8..0x83034200)
	// 830341F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830341FC: 82148A38  lwz r16, -0x75c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-30152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034200 size=64
    let mut pc: u32 = 0x83034200;
    'dispatch: loop {
        match pc {
            0x83034200 => {
    //   block [0x83034200..0x83034240)
	// 83034200: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303420C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034210: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83034214: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83034218: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303421C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83034220: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83034224: 4BFFD3A5  bl 0x830315c8
	ctx.lr = 0x83034228;
	sub_830315C8(ctx, base);
	// 83034228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303422C: 3C608303  lis r3, -0x7cfd
	ctx.r[3].s64 = -2096955392;
	// 83034230: 386341EC  addi r3, r3, 0x41ec
	ctx.r[3].s64 = ctx.r[3].s64 + 16876;
	// 83034234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303423C: 48173F7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034240 size=8
    let mut pc: u32 = 0x83034240;
    'dispatch: loop {
        match pc {
            0x83034240 => {
    //   block [0x83034240..0x83034248)
	// 83034240: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83034244: 82148A38  lwz r16, -0x75c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-30152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034248 size=60
    let mut pc: u32 = 0x83034248;
    'dispatch: loop {
        match pc {
            0x83034248 => {
    //   block [0x83034248..0x83034284)
	// 83034248: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303424C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034250: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034258: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8303425C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034260: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034268: 4E800421  bctrl
	ctx.lr = 0x8303426C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303426C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034270: 3C608303  lis r3, -0x7cfd
	ctx.r[3].s64 = -2096955392;
	// 83034274: 386341E4  addi r3, r3, 0x41e4
	ctx.r[3].s64 = ctx.r[3].s64 + 16868;
	// 83034278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303427C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034280: 48173F38  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034284 size=8
    let mut pc: u32 = 0x83034284;
    'dispatch: loop {
        match pc {
            0x83034284 => {
    //   block [0x83034284..0x8303428C)
	// 83034284: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83034288: 82148A38  lwz r16, -0x75c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-30152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303428C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303428C size=60
    let mut pc: u32 = 0x8303428C;
    'dispatch: loop {
        match pc {
            0x8303428C => {
    //   block [0x8303428C..0x830342C8)
	// 8303428C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303429C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 830342A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830342A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830342A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830342AC: 4E800421  bctrl
	ctx.lr = 0x830342B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830342B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830342B4: 3C608303  lis r3, -0x7cfd
	ctx.r[3].s64 = -2096955392;
	// 830342B8: 386341E8  addi r3, r3, 0x41e8
	ctx.r[3].s64 = ctx.r[3].s64 + 16872;
	// 830342BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830342C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830342C4: 48173EF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830342C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830342C8 size=8
    let mut pc: u32 = 0x830342C8;
    'dispatch: loop {
        match pc {
            0x830342C8 => {
    //   block [0x830342C8..0x830342D0)
	// 830342C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830342CC: 82148A38  lwz r16, -0x75c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-30152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830342D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830342D0 size=24
    let mut pc: u32 = 0x830342D0;
    'dispatch: loop {
        match pc {
            0x830342D0 => {
    //   block [0x830342D0..0x830342E8)
	// 830342D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830342D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830342D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830342DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830342E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830342E4: 4817C945  bl 0x831b0c28
	ctx.lr = 0x830342E8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830342F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830342F0 size=48
    let mut pc: u32 = 0x830342F0;
    'dispatch: loop {
        match pc {
            0x830342F0 => {
    //   block [0x830342F0..0x83034320)
	// 830342F0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830342F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830342F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830342FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034300: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 83034304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034308: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303430C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034310: 4E800421  bctrl
	ctx.lr = 0x83034314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034314: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83034318: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303431C: 4817C90D  bl 0x831b0c28
	ctx.lr = 0x83034320;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034320 size=48
    let mut pc: u32 = 0x83034320;
    'dispatch: loop {
        match pc {
            0x83034320 => {
    //   block [0x83034320..0x83034350)
	// 83034320: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034324: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034328: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303432C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034330: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83034334: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83034338: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303433C: 4BFA3FA5  bl 0x82fd82e0
	ctx.lr = 0x83034340;
	sub_82FD82E0(ctx, base);
	// 83034340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303434C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034350 size=40
    let mut pc: u32 = 0x83034350;
    'dispatch: loop {
        match pc {
            0x83034350 => {
    //   block [0x83034350..0x83034378)
	// 83034350: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034354: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034358: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303435C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034360: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83034364: 4BFFE155  bl 0x830324b8
	ctx.lr = 0x83034368;
	sub_830324B8(ctx, base);
	// 83034368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303436C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034378 size=120
    let mut pc: u32 = 0x83034378;
    'dispatch: loop {
        match pc {
            0x83034378 => {
    //   block [0x83034378..0x830343F0)
	// 83034378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83034384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303438C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034390: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034394: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034398: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303439C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830343A0: 4E800421  bctrl
	ctx.lr = 0x830343A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830343A4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830343A8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830343AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830343B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830343B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830343B8: 4E800421  bctrl
	ctx.lr = 0x830343BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830343BC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830343C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830343C4: 41820018  beq 0x830343dc
	if ctx.cr[0].eq {
	pc = 0x830343DC; continue 'dispatch;
	}
	// 830343C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830343CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830343D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830343D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830343D8: 4E800421  bctrl
	ctx.lr = 0x830343DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830343DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830343E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830343E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830343E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830343EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830343F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830343F0 size=144
    let mut pc: u32 = 0x830343F0;
    'dispatch: loop {
        match pc {
            0x830343F0 => {
    //   block [0x830343F0..0x83034480)
	// 830343F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830343F4: 48173D71  bl 0x831a8164
	ctx.lr = 0x830343F8;
	sub_831A8130(ctx, base);
	// 830343F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830343FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83034400: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034404: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83034408: 4082000C  bne 0x83034414
	if !ctx.cr[0].eq {
	pc = 0x83034414; continue 'dispatch;
	}
	// 8303440C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034410: 48000068  b 0x83034478
	pc = 0x83034478; continue 'dispatch;
	// 83034414: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034418: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303441C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83034420: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83034424: 40980050  bge cr6, 0x83034474
	if !ctx.cr[6].lt {
	pc = 0x83034474; continue 'dispatch;
	}
	// 83034428: 57FD083C  slwi r29, r31, 1
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8303442C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034430: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83034434: 7C9D5A2E  lhzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83034438: 4BF9D979  bl 0x82fd1db0
	ctx.lr = 0x8303443C;
	sub_82FD1DB0(ctx, base);
	// 8303443C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83034440: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034444: 419A0010  beq cr6, 0x83034454
	if ctx.cr[6].eq {
	pc = 0x83034454; continue 'dispatch;
	}
	// 83034448: 41820018  beq 0x83034460
	if ctx.cr[0].eq {
	pc = 0x83034460; continue 'dispatch;
	}
	// 8303444C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83034450: 48000010  b 0x83034460
	pc = 0x83034460; continue 'dispatch;
	// 83034454: 4082000C  bne 0x83034460
	if !ctx.cr[0].eq {
	pc = 0x83034460; continue 'dispatch;
	}
	// 83034458: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8303445C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83034460: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034464: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83034468: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 8303446C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83034470: 4198FFBC  blt cr6, 0x8303442c
	if ctx.cr[6].lt {
	pc = 0x8303442C; continue 'dispatch;
	}
	// 83034474: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83034478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303447C: 48173D38  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034480 size=4
    let mut pc: u32 = 0x83034480;
    'dispatch: loop {
        match pc {
            0x83034480 => {
    //   block [0x83034480..0x83034484)
	// 83034480: 4BFFFEF8  b 0x83034378
	sub_83034378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034488 size=48
    let mut pc: u32 = 0x83034488;
    'dispatch: loop {
        match pc {
            0x83034488 => {
    //   block [0x83034488..0x830344B8)
	// 83034488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034494: 4BFFFF5D  bl 0x830343f0
	ctx.lr = 0x83034498;
	sub_830343F0(ctx, base);
	// 83034498: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303449C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830344A0: 41810008  bgt 0x830344a8
	if ctx.cr[0].gt {
	pc = 0x830344A8; continue 'dispatch;
	}
	// 830344A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830344A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830344AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830344B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830344B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830344B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830344B8 size=8
    let mut pc: u32 = 0x830344B8;
    'dispatch: loop {
        match pc {
            0x830344B8 => {
    //   block [0x830344B8..0x830344C0)
	// 830344B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830344BC: 82148B84  lwz r16, -0x747c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29820 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830344C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830344C0 size=276
    let mut pc: u32 = 0x830344C0;
    'dispatch: loop {
        match pc {
            0x830344C0 => {
    //   block [0x830344C0..0x830345D4)
	// 830344C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830344C4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830344C8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830344CC: 48173C9D  bl 0x831a8168
	ctx.lr = 0x830344D0;
	sub_831A8130(ctx, base);
	// 830344D0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830344D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830344D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830344DC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830344E0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830344E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830344E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830344EC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830344F0: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830344F4: 419A0034  beq cr6, 0x83034528
	if ctx.cr[6].eq {
	pc = 0x83034528; continue 'dispatch;
	}
	// 830344F8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830344FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034500: 41820028  beq 0x83034528
	if ctx.cr[0].eq {
	pc = 0x83034528; continue 'dispatch;
	}
	// 83034504: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 83034508: 48000008  b 0x83034510
	pc = 0x83034510; continue 'dispatch;
	// 8303450C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83034510: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034514: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034518: 4082FFF4  bne 0x8303450c
	if !ctx.cr[0].eq {
	pc = 0x8303450C; continue 'dispatch;
	}
	// 8303451C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83034520: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83034524: 48000008  b 0x8303452c
	pc = 0x8303452C; continue 'dispatch;
	// 83034528: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8303452C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83034530: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83034534: 4BF9C64D  bl 0x82fd0b80
	ctx.lr = 0x83034538;
	sub_82FD0B80(ctx, base);
	// 83034538: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303453C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83034540: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83034544: 386B8B28  addi r3, r11, -0x74d8
	ctx.r[3].s64 = ctx.r[11].s64 + -29912;
	// 83034548: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8303454C: 4BF9C635  bl 0x82fd0b80
	ctx.lr = 0x83034550;
	sub_82FD0B80(ctx, base);
	// 83034550: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83034554: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 83034558: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8303455C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034560: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034564: 4099005C  ble cr6, 0x830345c0
	if !ctx.cr[6].gt {
	pc = 0x830345C0; continue 'dispatch;
	}
	// 83034568: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303456C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83034570: 4BFA3D29  bl 0x82fd8298
	ctx.lr = 0x83034574;
	sub_82FD8298(ctx, base);
	// 83034574: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034578: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8303457C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83034580: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83034584: 41820034  beq 0x830345b8
	if ctx.cr[0].eq {
	pc = 0x830345B8; continue 'dispatch;
	}
	// 83034588: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303458C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034590: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83034594: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83034598: 4BF9E4A9  bl 0x82fd2a40
	ctx.lr = 0x8303459C;
	sub_82FD2A40(ctx, base);
	// 8303459C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830345A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830345A4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830345A8: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 830345AC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830345B0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830345B4: 48000008  b 0x830345bc
	pc = 0x830345BC; continue 'dispatch;
	// 830345B8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830345BC: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830345C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830345C4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830345C8: 48173BF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830345CC: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830345D0: 4BFFFFF0  b 0x830345c0
	pc = 0x830345C0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830345D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830345D4 size=8
    let mut pc: u32 = 0x830345D4;
    'dispatch: loop {
        match pc {
            0x830345D4 => {
    //   block [0x830345D4..0x830345DC)
	// 830345D4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830345D8: 82148B84  lwz r16, -0x747c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29820 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830345DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830345DC size=24
    let mut pc: u32 = 0x830345DC;
    'dispatch: loop {
        match pc {
            0x830345DC => {
    //   block [0x830345DC..0x830345F4)
	// 830345DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830345E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830345E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830345E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830345EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830345F0: 4817C639  bl 0x831b0c28
	ctx.lr = 0x830345F4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830345FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830345FC size=48
    let mut pc: u32 = 0x830345FC;
    'dispatch: loop {
        match pc {
            0x830345FC => {
    //   block [0x830345FC..0x8303462C)
	// 830345FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303460C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83034610: 4BFFFD69  bl 0x83034378
	ctx.lr = 0x83034614;
	sub_83034378(ctx, base);
	// 83034614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034618: 3C608303  lis r3, -0x7cfd
	ctx.r[3].s64 = -2096955392;
	// 8303461C: 386345CC  addi r3, r3, 0x45cc
	ctx.r[3].s64 = ctx.r[3].s64 + 17868;
	// 83034620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034628: 48173B90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303462C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303462C size=48
    let mut pc: u32 = 0x8303462C;
    'dispatch: loop {
        match pc {
            0x8303462C => {
    //   block [0x8303462C..0x8303465C)
	// 8303462C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83034630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303463C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83034640: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034644: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83034648: 4BFA3C99  bl 0x82fd82e0
	ctx.lr = 0x8303464C;
	sub_82FD82E0(ctx, base);
	// 8303464C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034660 size=8
    let mut pc: u32 = 0x83034660;
    'dispatch: loop {
        match pc {
            0x83034660 => {
    //   block [0x83034660..0x83034668)
	// 83034660: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83034664: 82148C44  lwz r16, -0x73bc(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29628 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034668 size=276
    let mut pc: u32 = 0x83034668;
    'dispatch: loop {
        match pc {
            0x83034668 => {
    //   block [0x83034668..0x8303477C)
	// 83034668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303466C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83034670: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83034674: 48173AF1  bl 0x831a8164
	ctx.lr = 0x83034678;
	sub_831A8130(ctx, base);
	// 83034678: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303467C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034680: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83034684: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83034688: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8303468C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83034690: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83034694: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83034698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8303469C: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 830346A0: 419A0034  beq cr6, 0x830346d4
	if ctx.cr[6].eq {
	pc = 0x830346D4; continue 'dispatch;
	}
	// 830346A4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830346A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830346AC: 41820028  beq 0x830346d4
	if ctx.cr[0].eq {
	pc = 0x830346D4; continue 'dispatch;
	}
	// 830346B0: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830346B4: 48000008  b 0x830346bc
	pc = 0x830346BC; continue 'dispatch;
	// 830346B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830346BC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830346C0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830346C4: 4082FFF4  bne 0x830346b8
	if !ctx.cr[0].eq {
	pc = 0x830346B8; continue 'dispatch;
	}
	// 830346C8: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830346CC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830346D0: 48000008  b 0x830346d8
	pc = 0x830346D8; continue 'dispatch;
	// 830346D4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830346D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830346DC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830346E0: 4BF9C4A1  bl 0x82fd0b80
	ctx.lr = 0x830346E4;
	sub_82FD0B80(ctx, base);
	// 830346E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830346E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830346EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830346F0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830346F4: 4BF9C48D  bl 0x82fd0b80
	ctx.lr = 0x830346F8;
	sub_82FD0B80(ctx, base);
	// 830346F8: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830346FC: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83034700: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83034704: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303470C: 4099005C  ble cr6, 0x83034768
	if !ctx.cr[6].gt {
	pc = 0x83034768; continue 'dispatch;
	}
	// 83034710: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83034714: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83034718: 4BFA3B81  bl 0x82fd8298
	ctx.lr = 0x8303471C;
	sub_82FD8298(ctx, base);
	// 8303471C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034720: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034724: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83034728: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303472C: 41820034  beq 0x83034760
	if ctx.cr[0].eq {
	pc = 0x83034760; continue 'dispatch;
	}
	// 83034730: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83034734: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034738: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8303473C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83034740: 4BF9E301  bl 0x82fd2a40
	ctx.lr = 0x83034744;
	sub_82FD2A40(ctx, base);
	// 83034744: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83034748: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8303474C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83034750: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 83034754: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83034758: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303475C: 48000008  b 0x83034764
	pc = 0x83034764; continue 'dispatch;
	// 83034760: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 83034764: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83034768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303476C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83034770: 48173A44  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83034774: 83DF00A4  lwz r30, 0xa4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83034778: 4BFFFFF0  b 0x83034768
	pc = 0x83034768; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303477C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303477C size=8
    let mut pc: u32 = 0x8303477C;
    'dispatch: loop {
        match pc {
            0x8303477C => {
    //   block [0x8303477C..0x83034784)
	// 8303477C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83034780: 82148C44  lwz r16, -0x73bc(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29628 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034784 size=24
    let mut pc: u32 = 0x83034784;
    'dispatch: loop {
        match pc {
            0x83034784 => {
    //   block [0x83034784..0x8303479C)
	// 83034784: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034788: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303478C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034790: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83034794: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034798: 4817C491  bl 0x831b0c28
	ctx.lr = 0x8303479C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830347A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830347A4 size=48
    let mut pc: u32 = 0x830347A4;
    'dispatch: loop {
        match pc {
            0x830347A4 => {
    //   block [0x830347A4..0x830347D4)
	// 830347A4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830347A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830347AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830347B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830347B4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830347B8: 4BFFFBC1  bl 0x83034378
	ctx.lr = 0x830347BC;
	sub_83034378(ctx, base);
	// 830347BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830347C0: 3C608303  lis r3, -0x7cfd
	ctx.r[3].s64 = -2096955392;
	// 830347C4: 38634774  addi r3, r3, 0x4774
	ctx.r[3].s64 = ctx.r[3].s64 + 18292;
	// 830347C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830347CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830347D0: 481739E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830347D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830347D4 size=48
    let mut pc: u32 = 0x830347D4;
    'dispatch: loop {
        match pc {
            0x830347D4 => {
    //   block [0x830347D4..0x83034804)
	// 830347D4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830347D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830347DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830347E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830347E4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830347E8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830347EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830347F0: 4BFA3AF1  bl 0x82fd82e0
	ctx.lr = 0x830347F4;
	sub_82FD82E0(ctx, base);
	// 830347F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830347F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830347FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034808 size=208
    let mut pc: u32 = 0x83034808;
    'dispatch: loop {
        match pc {
            0x83034808 => {
    //   block [0x83034808..0x830348D8)
	// 83034808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303480C: 48173959  bl 0x831a8164
	ctx.lr = 0x83034810;
	sub_831A8130(ctx, base);
	// 83034810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83034818: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303481C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034820: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83034824: 409800A8  bge cr6, 0x830348cc
	if !ctx.cr[6].lt {
	pc = 0x830348CC; continue 'dispatch;
	}
	// 83034828: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303482C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 83034830: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83034834: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034838: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303483C: 7C9D5A2E  lhzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83034840: 4BF9D571  bl 0x82fd1db0
	ctx.lr = 0x83034844;
	sub_82FD1DB0(ctx, base);
	// 83034844: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83034848: 419A0014  beq cr6, 0x8303485c
	if ctx.cr[6].eq {
	pc = 0x8303485C; continue 'dispatch;
	}
	// 8303484C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034850: 40820024  bne 0x83034874
	if !ctx.cr[0].eq {
	pc = 0x83034874; continue 'dispatch;
	}
	// 83034854: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83034858: 48000008  b 0x83034860
	pc = 0x83034860; continue 'dispatch;
	// 8303485C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83034860: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034864: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83034868: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 8303486C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83034870: 4198FFC4  blt cr6, 0x83034834
	if ctx.cr[6].lt {
	pc = 0x83034834; continue 'dispatch;
	}
	// 83034874: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034878: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8303487C: 41820050  beq 0x830348cc
	if ctx.cr[0].eq {
	pc = 0x830348CC; continue 'dispatch;
	}
	// 83034880: 7D7CF050  subf r11, r28, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 83034884: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83034888: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303488C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83034890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034894: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303489C: 4E800421  bctrl
	ctx.lr = 0x830348A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830348A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830348A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830348A8: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830348AC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830348B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830348B4: 4BF9D7D5  bl 0x82fd2088
	ctx.lr = 0x830348B8;
	sub_82FD2088(ctx, base);
	// 830348B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830348BC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830348C0: 48006891  bl 0x8303b150
	ctx.lr = 0x830348C4;
	sub_8303B150(ctx, base);
	// 830348C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830348C8: 48000008  b 0x830348d0
	pc = 0x830348D0; continue 'dispatch;
	// 830348CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830348D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830348D4: 481738E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830348D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830348D8 size=44
    let mut pc: u32 = 0x830348D8;
    'dispatch: loop {
        match pc {
            0x830348D8 => {
    //   block [0x830348D8..0x83034904)
	// 830348D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830348DC: A1440008  lhz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830348E0: A16BA6A4  lhz r11, -0x595c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 830348E4: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830348E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830348EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830348F0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830348F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830348F8: 4182000C  beq 0x83034904
	if ctx.cr[0].eq {
		sub_83034904(ctx, base);
		return;
	}
	// 830348FC: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034904 size=8
    let mut pc: u32 = 0x83034904;
    'dispatch: loop {
        match pc {
            0x83034904 => {
    //   block [0x83034904..0x8303490C)
	// 83034904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034910 size=44
    let mut pc: u32 = 0x83034910;
    'dispatch: loop {
        match pc {
            0x83034910 => {
    //   block [0x83034910..0x8303493C)
	// 83034910: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83034914: A1440008  lhz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034918: A16BA6A8  lhz r11, -0x5958(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 8303491C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83034920: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83034924: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83034928: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8303492C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034930: 4182000C  beq 0x8303493c
	if ctx.cr[0].eq {
		sub_8303493C(ctx, base);
		return;
	}
	// 83034934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303493C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303493C size=8
    let mut pc: u32 = 0x8303493C;
    'dispatch: loop {
        match pc {
            0x8303493C => {
    //   block [0x8303493C..0x83034944)
	// 8303493C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034948 size=16
    let mut pc: u32 = 0x83034948;
    'dispatch: loop {
        match pc {
            0x83034948 => {
    //   block [0x83034948..0x83034958)
	// 83034948: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 8303494C: 396B8CB8  addi r11, r11, -0x7348
	ctx.r[11].s64 = ctx.r[11].s64 + -29512;
	// 83034950: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83034954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034958 size=68
    let mut pc: u32 = 0x83034958;
    'dispatch: loop {
        match pc {
            0x83034958 => {
    //   block [0x83034958..0x8303499C)
	// 83034958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303495C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83034964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034968: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 8303496C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83034970: 396B8CB8  addi r11, r11, -0x7348
	ctx.r[11].s64 = ctx.r[11].s64 + -29512;
	// 83034974: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83034978: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303497C: 41820008  beq 0x83034984
	if ctx.cr[0].eq {
	pc = 0x83034984; continue 'dispatch;
	}
	// 83034980: 4B28B8E9  bl 0x822c0268
	ctx.lr = 0x83034984;
	sub_822C0268(ctx, base);
	// 83034984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83034988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303498C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83034998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830349A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830349A0 size=80
    let mut pc: u32 = 0x830349A0;
    'dispatch: loop {
        match pc {
            0x830349A0 => {
    //   block [0x830349A0..0x830349F0)
	// 830349A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830349A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830349A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830349AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830349B0: 3D408215  lis r10, -0x7deb
	ctx.r[10].s64 = -2112552960;
	// 830349B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830349B8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830349BC: 394A8CDC  addi r10, r10, -0x7324
	ctx.r[10].s64 = ctx.r[10].s64 + -29476;
	// 830349C0: 38A00304  li r5, 0x304
	ctx.r[5].s64 = 772;
	// 830349C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830349C8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830349CC: 917F0308  stw r11, 0x308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[11].u32 ) };
	// 830349D0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830349D4: 4817380D  bl 0x831a81e0
	ctx.lr = 0x830349D8;
	sub_831A81E0(ctx, base);
	// 830349D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830349DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830349E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830349E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830349E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830349EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830349F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830349F0 size=8
    let mut pc: u32 = 0x830349F0;
    'dispatch: loop {
        match pc {
            0x830349F0 => {
    //   block [0x830349F0..0x830349F8)
	// 830349F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830349F4: 82148D10  lwz r16, -0x72f0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830349F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830349F8 size=360
    let mut pc: u32 = 0x830349F8;
    'dispatch: loop {
        match pc {
            0x830349F8 => {
    //   block [0x830349F8..0x83034B60)
	// 830349F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830349FC: 4817374D  bl 0x831a8148
	ctx.lr = 0x83034A00;
	sub_831A8130(ctx, base);
	// 83034A00: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83034A04: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034A08: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83034A0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83034A10: 38790004  addi r3, r25, 4
	ctx.r[3].s64 = ctx.r[25].s64 + 4;
	// 83034A14: 4BFAA815  bl 0x82fdf228
	ctx.lr = 0x83034A18;
	sub_82FDF228(ctx, base);
	// 83034A18: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 83034A1C: 3880030C  li r4, 0x30c
	ctx.r[4].s64 = 780;
	// 83034A20: 92DF0050  stw r22, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[22].u32 ) };
	// 83034A24: 4BFACFCD  bl 0x82fe19f0
	ctx.lr = 0x83034A28;
	sub_82FE19F0(ctx, base);
	// 83034A28: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83034A2C: 4182002C  beq 0x83034a58
	if ctx.cr[0].eq {
	pc = 0x83034A58; continue 'dispatch;
	}
	// 83034A30: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83034A34: 933E0308  stw r25, 0x308(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(776 as u32), ctx.r[25].u32 ) };
	// 83034A38: 38A00304  li r5, 0x304
	ctx.r[5].s64 = 772;
	// 83034A3C: 396B8CDC  addi r11, r11, -0x7324
	ctx.r[11].s64 = ctx.r[11].s64 + -29476;
	// 83034A40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83034A44: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83034A48: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83034A4C: 48173795  bl 0x831a81e0
	ctx.lr = 0x83034A50;
	sub_831A81E0(ctx, base);
	// 83034A50: 7FD4F378  mr r20, r30
	ctx.r[20].u64 = ctx.r[30].u64;
	// 83034A54: 48000008  b 0x83034a5c
	pc = 0x83034A5C; continue 'dispatch;
	// 83034A58: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 83034A5C: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 83034A60: 7F5DA050  subf r26, r29, r20
	ctx.r[26].s64 = ctx.r[20].s64 - ctx.r[29].s64;
	// 83034A64: 3AA000C1  li r21, 0xc1
	ctx.r[21].s64 = 193;
	// 83034A68: 3F008214  lis r24, -0x7dec
	ctx.r[24].s64 = -2112618496;
	// 83034A6C: 3EE08214  lis r23, -0x7dec
	ctx.r[23].s64 = -2112618496;
	// 83034A70: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034A74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034A78: 418200D0  beq 0x83034b48
	if ctx.cr[0].eq {
	pc = 0x83034B48; continue 'dispatch;
	}
	// 83034A7C: 4811FB85  bl 0x83154600
	ctx.lr = 0x83034A80;
	sub_83154600(ctx, base);
	// 83034A80: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83034A84: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83034A88: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83034A8C: 4BFACF65  bl 0x82fe19f0
	ctx.lr = 0x83034A90;
	sub_82FE19F0(ctx, base);
	// 83034A90: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83034A94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034A98: 41820014  beq 0x83034aac
	if ctx.cr[0].eq {
	pc = 0x83034AAC; continue 'dispatch;
	}
	// 83034A9C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83034AA0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83034AA4: 4804AE4D  bl 0x8307f8f0
	ctx.lr = 0x83034AA8;
	sub_8307F8F0(ctx, base);
	// 83034AA8: 48000008  b 0x83034ab0
	pc = 0x83034AB0; continue 'dispatch;
	// 83034AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034AB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83034AB4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83034AB8: 7C7AE12E  stwx r3, r26, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 83034ABC: 419A008C  beq cr6, 0x83034b48
	if ctx.cr[6].eq {
	pc = 0x83034B48; continue 'dispatch;
	}
	// 83034AC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83034AC4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034AC8: 4804ACF9  bl 0x8307f7c0
	ctx.lr = 0x83034ACC;
	sub_8307F7C0(ctx, base);
	// 83034ACC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83034AD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83034AD4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034AD8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83034ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034AE0: 4E800421  bctrl
	ctx.lr = 0x83034AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034AE4: A157A6AC  lhz r10, -0x5954(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 83034AE8: A13D0008  lhz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034AEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83034AF0: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83034AF4: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 83034AF8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83034AFC: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83034B00: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 83034B04: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034B08: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034B0C: 4182000C  beq 0x83034b18
	if ctx.cr[0].eq {
	pc = 0x83034B18; continue 'dispatch;
	}
	// 83034B10: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83034B14: 48000008  b 0x83034b1c
	pc = 0x83034B1C; continue 'dispatch;
	// 83034B18: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83034B1C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83034B20: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 83034B24: A158A6A4  lhz r10, -0x595c(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83034B28: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034B2C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83034B30: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83034B34: 7C7AE02E  lwzx r3, r26, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83034B38: 4804ADE9  bl 0x8307f920
	ctx.lr = 0x83034B3C;
	sub_8307F920(ctx, base);
	// 83034B3C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83034B40: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83034B44: 4198FF7C  blt cr6, 0x83034ac0
	if ctx.cr[6].lt {
	pc = 0x83034AC0; continue 'dispatch;
	}
	// 83034B48: 36B5FFFF  addic. r21, r21, -1
	ctx.xer.ca = (ctx.r[21].u32 > (!(-1 as u32)));
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 83034B4C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83034B50: 4082FF20  bne 0x83034a70
	if !ctx.cr[0].eq {
	pc = 0x83034A70; continue 'dispatch;
	}
	// 83034B54: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 83034B58: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83034B5C: 4817363C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034B60 size=44
    let mut pc: u32 = 0x83034B60;
    'dispatch: loop {
        match pc {
            0x83034B60 => {
    //   block [0x83034B60..0x83034B8C)
	// 83034B60: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83034B64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034B68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83034B6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034B70: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83034B74: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83034B78: 48092F69  bl 0x830c7ae0
	ctx.lr = 0x83034B7C;
	sub_830C7AE0(ctx, base);
	// 83034B7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83034B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83034B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83034B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034B90 size=68
    let mut pc: u32 = 0x83034B90;
    'dispatch: loop {
        match pc {
            0x83034B90 => {
    //   block [0x83034B90..0x83034BD4)
	// 83034B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034B94: 481735D9  bl 0x831a816c
	ctx.lr = 0x83034B98;
	sub_831A8130(ctx, base);
	// 83034B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034B9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83034BA0: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 83034BA4: 3BE000C1  li r31, 0xc1
	ctx.r[31].s64 = 193;
	// 83034BA8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034BAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034BB0: 41820008  beq 0x83034bb8
	if ctx.cr[0].eq {
	pc = 0x83034BB8; continue 'dispatch;
	}
	// 83034BB4: 4811FA4D  bl 0x83154600
	ctx.lr = 0x83034BB8;
	sub_83154600(ctx, base);
	// 83034BB8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83034BBC: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83034BC0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83034BC4: 4082FFE4  bne 0x83034ba8
	if !ctx.cr[0].eq {
	pc = 0x83034BA8; continue 'dispatch;
	}
	// 83034BC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83034BCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83034BD0: 481735EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034BD8 size=124
    let mut pc: u32 = 0x83034BD8;
    'dispatch: loop {
        match pc {
            0x83034BD8 => {
    //   block [0x83034BD8..0x83034C54)
	// 83034BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034BDC: 48173589  bl 0x831a8164
	ctx.lr = 0x83034BE0;
	sub_831A8130(ctx, base);
	// 83034BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034BE4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83034BE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83034BEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83034BF0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83034BF4: 3B9B0004  addi r28, r27, 4
	ctx.r[28].s64 = ctx.r[27].s64 + 4;
	// 83034BF8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034BFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034C00: 41820020  beq 0x83034c20
	if ctx.cr[0].eq {
	pc = 0x83034C20; continue 'dispatch;
	}
	// 83034C04: 4811F9FD  bl 0x83154600
	ctx.lr = 0x83034C08;
	sub_83154600(ctx, base);
	// 83034C08: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83034C0C: 41980010  blt cr6, 0x83034c1c
	if ctx.cr[6].lt {
	pc = 0x83034C1C; continue 'dispatch;
	}
	// 83034C10: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83034C14: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83034C18: 41980024  blt cr6, 0x83034c3c
	if ctx.cr[6].lt {
	pc = 0x83034C3C; continue 'dispatch;
	}
	// 83034C1C: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83034C20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83034C24: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83034C28: 2B1D00C1  cmplwi cr6, r29, 0xc1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 193 as u32, &mut ctx.xer);
	// 83034C2C: 4198FFCC  blt cr6, 0x83034bf8
	if ctx.cr[6].lt {
	pc = 0x83034BF8; continue 'dispatch;
	}
	// 83034C30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83034C38: 4817357C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83034C3C: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 83034C40: 7C9FF050  subf r4, r31, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 83034C44: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83034C48: 7C6BD82E  lwzx r3, r11, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83034C4C: 4804AB75  bl 0x8307f7c0
	ctx.lr = 0x83034C50;
	sub_8307F7C0(ctx, base);
	// 83034C50: 4BFFFFE4  b 0x83034c34
	pc = 0x83034C34; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034C58 size=160
    let mut pc: u32 = 0x83034C58;
    'dispatch: loop {
        match pc {
            0x83034C58 => {
    //   block [0x83034C58..0x83034CF8)
	// 83034C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034C5C: 48173505  bl 0x831a8160
	ctx.lr = 0x83034C60;
	sub_831A8130(ctx, base);
	// 83034C60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034C64: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83034C68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83034C6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83034C70: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 83034C74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83034C78: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034C7C: 4BF9CF85  bl 0x82fd1c00
	ctx.lr = 0x83034C80;
	sub_82FD1C00(ctx, base);
	// 83034C80: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 83034C84: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83034C88: 7C7BE82E  lwzx r3, r27, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83034C8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034C90: 41820054  beq 0x83034ce4
	if ctx.cr[0].eq {
	pc = 0x83034CE4; continue 'dispatch;
	}
	// 83034C94: 4811F96D  bl 0x83154600
	ctx.lr = 0x83034C98;
	sub_83154600(ctx, base);
	// 83034C98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83034C9C: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83034CA0: 40810044  ble 0x83034ce4
	if !ctx.cr[0].gt {
	pc = 0x83034CE4; continue 'dispatch;
	}
	// 83034CA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83034CA8: 7C7BE82E  lwzx r3, r27, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83034CAC: 4804AB15  bl 0x8307f7c0
	ctx.lr = 0x83034CB0;
	sub_8307F7C0(ctx, base);
	// 83034CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83034CB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034CB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034CBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034CC0: 4E800421  bctrl
	ctx.lr = 0x83034CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034CC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83034CC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83034CCC: 4BF9EF75  bl 0x82fd3c40
	ctx.lr = 0x83034CD0;
	sub_82FD3C40(ctx, base);
	// 83034CD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034CD4: 4082001C  bne 0x83034cf0
	if !ctx.cr[0].eq {
	pc = 0x83034CF0; continue 'dispatch;
	}
	// 83034CD8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83034CDC: 7F1ED000  cmpw cr6, r30, r26
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[26].s32, &mut ctx.xer);
	// 83034CE0: 4198FFC4  blt cr6, 0x83034ca4
	if ctx.cr[6].lt {
	pc = 0x83034CA4; continue 'dispatch;
	}
	// 83034CE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83034CE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83034CEC: 481734C4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83034CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83034CF4: 4BFFFFF4  b 0x83034ce8
	pc = 0x83034CE8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034CF8 size=592
    let mut pc: u32 = 0x83034CF8;
    'dispatch: loop {
        match pc {
            0x83034CF8 => {
    //   block [0x83034CF8..0x83034F48)
	// 83034CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034CFC: 48173461  bl 0x831a815c
	ctx.lr = 0x83034D00;
	sub_831A8130(ctx, base);
	// 83034D00: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034D04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83034D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83034D0C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83034D10: A14BA698  lhz r10, -0x5968(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83034D14: 817F0308  lwz r11, 0x308(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034D18: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83034D1C: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83034D20: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83034D24: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83034D28: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83034D2C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034D30: 41820070  beq 0x83034da0
	if ctx.cr[0].eq {
	pc = 0x83034DA0; continue 'dispatch;
	}
	// 83034D34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83034D38: 419A003C  beq cr6, 0x83034d74
	if ctx.cr[6].eq {
	pc = 0x83034D74; continue 'dispatch;
	}
	// 83034D3C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83034D40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034D44: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034D4C: 4E800421  bctrl
	ctx.lr = 0x83034D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034D50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034D54: 41820020  beq 0x83034d74
	if ctx.cr[0].eq {
	pc = 0x83034D74; continue 'dispatch;
	}
	// 83034D58: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034D60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034D68: 4E800421  bctrl
	ctx.lr = 0x83034D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034D6C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83034D70: 4800000C  b 0x83034d7c
	pc = 0x83034D7C; continue 'dispatch;
	// 83034D74: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83034D78: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034D7C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83034D80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83034D84: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83034D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034D8C: 4BFC5145  bl 0x82ff9ed0
	ctx.lr = 0x83034D90;
	sub_82FF9ED0(ctx, base);
	// 83034D90: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83034D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034D98: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83034D9C: 4817BE8D  bl 0x831b0c28
	ctx.lr = 0x83034DA0;
	sub_831B0C28(ctx, base);
	// 83034DA0: 3F208339  lis r25, -0x7cc7
	ctx.r[25].s64 = -2093416448;
	// 83034DA4: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 83034DA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83034DAC: 80B9B7E8  lwz r5, -0x4818(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034DB0: 4BF9CE51  bl 0x82fd1c00
	ctx.lr = 0x83034DB4;
	sub_82FD1C00(ctx, base);
	// 83034DB4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 83034DB8: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83034DBC: 7C7DF82E  lwzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83034DC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034DC4: 40820070  bne 0x83034e34
	if !ctx.cr[0].eq {
	pc = 0x83034E34; continue 'dispatch;
	}
	// 83034DC8: 817F0308  lwz r11, 0x308(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83034DD0: 419A003C  beq cr6, 0x83034e0c
	if ctx.cr[6].eq {
	pc = 0x83034E0C; continue 'dispatch;
	}
	// 83034DD4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83034DD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034DDC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034DE4: 4E800421  bctrl
	ctx.lr = 0x83034DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034DE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034DEC: 41820020  beq 0x83034e0c
	if ctx.cr[0].eq {
	pc = 0x83034E0C; continue 'dispatch;
	}
	// 83034DF0: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034DF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034DF8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034E00: 4E800421  bctrl
	ctx.lr = 0x83034E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034E04: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83034E08: 48000008  b 0x83034e10
	pc = 0x83034E10; continue 'dispatch;
	// 83034E0C: 8179B7E8  lwz r11, -0x4818(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034E10: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83034E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83034E18: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83034E1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034E20: 4BFC50B1  bl 0x82ff9ed0
	ctx.lr = 0x83034E24;
	sub_82FF9ED0(ctx, base);
	// 83034E24: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83034E28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034E2C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83034E30: 4817BDF9  bl 0x831b0c28
	ctx.lr = 0x83034E34;
	sub_831B0C28(ctx, base);
	// 83034E34: 4811F7CD  bl 0x83154600
	ctx.lr = 0x83034E38;
	sub_83154600(ctx, base);
	// 83034E38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83034E3C: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83034E40: 40810044  ble 0x83034e84
	if !ctx.cr[0].gt {
	pc = 0x83034E84; continue 'dispatch;
	}
	// 83034E44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83034E48: 7C7DF82E  lwzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83034E4C: 4804A975  bl 0x8307f7c0
	ctx.lr = 0x83034E50;
	sub_8307F7C0(ctx, base);
	// 83034E50: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83034E54: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034E58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034E5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034E60: 4E800421  bctrl
	ctx.lr = 0x83034E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034E64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83034E68: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83034E6C: 4BF9EDD5  bl 0x82fd3c40
	ctx.lr = 0x83034E70;
	sub_82FD3C40(ctx, base);
	// 83034E70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83034E74: 40820054  bne 0x83034ec8
	if !ctx.cr[0].eq {
	pc = 0x83034EC8; continue 'dispatch;
	}
	// 83034E78: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83034E7C: 7F1ED000  cmpw cr6, r30, r26
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[26].s32, &mut ctx.xer);
	// 83034E80: 4198FFC4  blt cr6, 0x83034e44
	if ctx.cr[6].lt {
	pc = 0x83034E44; continue 'dispatch;
	}
	// 83034E84: 817F0308  lwz r11, 0x308(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83034E8C: 419A0094  beq cr6, 0x83034f20
	if ctx.cr[6].eq {
	pc = 0x83034F20; continue 'dispatch;
	}
	// 83034E90: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83034E94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034E98: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034EA0: 4E800421  bctrl
	ctx.lr = 0x83034EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034EA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034EA8: 41820078  beq 0x83034f20
	if ctx.cr[0].eq {
	pc = 0x83034F20; continue 'dispatch;
	}
	// 83034EAC: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034EB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034EBC: 4E800421  bctrl
	ctx.lr = 0x83034EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034EC0: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83034EC4: 48000060  b 0x83034f24
	pc = 0x83034F24; continue 'dispatch;
	// 83034EC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83034ECC: 7C7DF82E  lwzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83034ED0: 4804A989  bl 0x8307f858
	ctx.lr = 0x83034ED4;
	sub_8307F858(ctx, base);
	// 83034ED4: 807F0308  lwz r3, 0x308(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034ED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034EDC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034EE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034EE4: 4E800421  bctrl
	ctx.lr = 0x83034EE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034EE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034EEC: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 83034EF0: 40820008  bne 0x83034ef8
	if !ctx.cr[0].eq {
	pc = 0x83034EF8; continue 'dispatch;
	}
	// 83034EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83034EF8: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 83034EFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83034F00: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83034F04: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83034F08: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83034F0C: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83034F10: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83034F14: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83034F18: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83034F1C: 48173290  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83034F20: 8179B7E8  lwz r11, -0x4818(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034F24: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83034F28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83034F2C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83034F30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034F34: 4BFC4F9D  bl 0x82ff9ed0
	ctx.lr = 0x83034F38;
	sub_82FF9ED0(ctx, base);
	// 83034F38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83034F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83034F40: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83034F44: 4817BCE5  bl 0x831b0c28
	ctx.lr = 0x83034F48;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83034F48 size=8
    let mut pc: u32 = 0x83034F48;
    'dispatch: loop {
        match pc {
            0x83034F48 => {
    //   block [0x83034F48..0x83034F50)
	// 83034F48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83034F4C: 82148D58  lwz r16, -0x72a8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-29352 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83034F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83034F50 size=844
    let mut pc: u32 = 0x83034F50;
    'dispatch: loop {
        match pc {
            0x83034F50 => {
    //   block [0x83034F50..0x8303529C)
	// 83034F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83034F54: 48173201  bl 0x831a8154
	ctx.lr = 0x83034F58;
	sub_831A8130(ctx, base);
	// 83034F58: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 83034F5C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83034F60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83034F64: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83034F68: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034F70: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034F78: 4E800421  bctrl
	ctx.lr = 0x83034F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034F7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83034F80: 3BBA0004  addi r29, r26, 4
	ctx.r[29].s64 = ctx.r[26].s64 + 4;
	// 83034F84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83034F88: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83034F8C: 4BFAA29D  bl 0x82fdf228
	ctx.lr = 0x83034F90;
	sub_82FDF228(ctx, base);
	// 83034F90: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034F94: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83034F98: 419A0070  beq cr6, 0x83035008
	if ctx.cr[6].eq {
	pc = 0x83035008; continue 'dispatch;
	}
	// 83034F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83034FA0: 419A003C  beq cr6, 0x83034fdc
	if ctx.cr[6].eq {
	pc = 0x83034FDC; continue 'dispatch;
	}
	// 83034FA4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83034FA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034FAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034FB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034FB4: 4E800421  bctrl
	ctx.lr = 0x83034FB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034FB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83034FBC: 41820020  beq 0x83034fdc
	if ctx.cr[0].eq {
	pc = 0x83034FDC; continue 'dispatch;
	}
	// 83034FC0: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83034FC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83034FC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83034FCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83034FD0: 4E800421  bctrl
	ctx.lr = 0x83034FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83034FD4: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83034FD8: 4800000C  b 0x83034fe4
	pc = 0x83034FE4; continue 'dispatch;
	// 83034FDC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83034FE0: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83034FE4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83034FE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83034FEC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83034FF0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83034FF4: 4BFC4EDD  bl 0x82ff9ed0
	ctx.lr = 0x83034FF8;
	sub_82FF9ED0(ctx, base);
	// 83034FF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83034FFC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83035000: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035004: 4817BC25  bl 0x831b0c28
	ctx.lr = 0x83035008;
	sub_831B0C28(ctx, base);
	// 83035008: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303500C: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83035010: A14AA698  lhz r10, -0x5968(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83035014: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83035018: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303501C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83035020: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83035024: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035028: 41820070  beq 0x83035098
	if ctx.cr[0].eq {
	pc = 0x83035098; continue 'dispatch;
	}
	// 8303502C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83035030: 419A003C  beq cr6, 0x8303506c
	if ctx.cr[6].eq {
	pc = 0x8303506C; continue 'dispatch;
	}
	// 83035034: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303503C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035044: 4E800421  bctrl
	ctx.lr = 0x83035048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035048: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303504C: 41820020  beq 0x8303506c
	if ctx.cr[0].eq {
	pc = 0x8303506C; continue 'dispatch;
	}
	// 83035050: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035054: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035058: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303505C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035060: 4E800421  bctrl
	ctx.lr = 0x83035064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035064: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83035068: 4800000C  b 0x83035074
	pc = 0x83035074; continue 'dispatch;
	// 8303506C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83035070: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83035074: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83035078: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303507C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83035080: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83035084: 4BFC4E4D  bl 0x82ff9ed0
	ctx.lr = 0x83035088;
	sub_82FF9ED0(ctx, base);
	// 83035088: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303508C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83035090: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035094: 4817BB95  bl 0x831b0c28
	ctx.lr = 0x83035098;
	sub_831B0C28(ctx, base);
	// 83035098: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303509C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830350A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830350A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830350A8: 4E800421  bctrl
	ctx.lr = 0x830350AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830350AC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830350B0: 3EE08214  lis r23, -0x7dec
	ctx.r[23].s64 = -2112618496;
	// 830350B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830350B8: 409A00A0  bne cr6, 0x83035158
	if !ctx.cr[6].eq {
	pc = 0x83035158; continue 'dispatch;
	}
	// 830350BC: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830350C0: A177A6A4  lhz r11, -0x595c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 830350C4: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830350C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830350CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830350D0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830350D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830350D8: 41820080  beq 0x83035158
	if ctx.cr[0].eq {
	pc = 0x83035158; continue 'dispatch;
	}
	// 830350DC: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 830350E0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830350E4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830350E8: 419A0070  beq cr6, 0x83035158
	if ctx.cr[6].eq {
	pc = 0x83035158; continue 'dispatch;
	}
	// 830350EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830350F0: 419A003C  beq cr6, 0x8303512c
	if ctx.cr[6].eq {
	pc = 0x8303512C; continue 'dispatch;
	}
	// 830350F4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830350F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830350FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035104: 4E800421  bctrl
	ctx.lr = 0x83035108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303510C: 41820020  beq 0x8303512c
	if ctx.cr[0].eq {
	pc = 0x8303512C; continue 'dispatch;
	}
	// 83035110: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035118: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303511C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035120: 4E800421  bctrl
	ctx.lr = 0x83035124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035124: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83035128: 4800000C  b 0x83035134
	pc = 0x83035134; continue 'dispatch;
	// 8303512C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83035130: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83035134: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83035138: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303513C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83035140: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83035144: 4BFC4D8D  bl 0x82ff9ed0
	ctx.lr = 0x83035148;
	sub_82FF9ED0(ctx, base);
	// 83035148: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303514C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83035150: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035154: 4817BAD5  bl 0x831b0c28
	ctx.lr = 0x83035158;
	sub_831B0C28(ctx, base);
	// 83035158: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 8303515C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83035160: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83035164: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83035168: A177A6A4  lhz r11, -0x595c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303516C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83035170: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83035174: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035178: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303517C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035180: 4E800421  bctrl
	ctx.lr = 0x83035184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035184: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83035188: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 8303518C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83035190: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83035194: 4BF9CA6D  bl 0x82fd1c00
	ctx.lr = 0x83035198;
	sub_82FD1C00(ctx, base);
	// 83035198: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8303519C: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 830351A0: 7D7DF02E  lwzx r11, r29, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830351A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830351A8: 409A0034  bne cr6, 0x830351dc
	if !ctx.cr[6].eq {
	pc = 0x830351DC; continue 'dispatch;
	}
	// 830351AC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 830351B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830351B4: 4BFAC83D  bl 0x82fe19f0
	ctx.lr = 0x830351B8;
	sub_82FE19F0(ctx, base);
	// 830351B8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830351BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830351C0: 41820014  beq 0x830351d4
	if ctx.cr[0].eq {
	pc = 0x830351D4; continue 'dispatch;
	}
	// 830351C4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 830351C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830351CC: 4804A725  bl 0x8307f8f0
	ctx.lr = 0x830351D0;
	sub_8307F8F0(ctx, base);
	// 830351D0: 48000008  b 0x830351d8
	pc = 0x830351D8; continue 'dispatch;
	// 830351D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830351D8: 7C7DF12E  stwx r3, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 830351DC: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830351E0: 4811F421  bl 0x83154600
	ctx.lr = 0x830351E4;
	sub_83154600(ctx, base);
	// 830351E4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830351E8: 7C781B79  or. r24, r3, r3
	ctx.r[24].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 830351EC: 40810044  ble 0x83035230
	if !ctx.cr[0].gt {
	pc = 0x83035230; continue 'dispatch;
	}
	// 830351F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830351F4: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830351F8: 4804A5C9  bl 0x8307f7c0
	ctx.lr = 0x830351FC;
	sub_8307F7C0(ctx, base);
	// 830351FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83035200: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035204: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83035208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303520C: 4E800421  bctrl
	ctx.lr = 0x83035210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035210: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83035214: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83035218: 4BF9EA29  bl 0x82fd3c40
	ctx.lr = 0x8303521C;
	sub_82FD3C40(ctx, base);
	// 8303521C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83035220: 40820028  bne 0x83035248
	if !ctx.cr[0].eq {
	pc = 0x83035248; continue 'dispatch;
	}
	// 83035224: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83035228: 7F1CC000  cmpw cr6, r28, r24
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8303522C: 4198FFC4  blt cr6, 0x830351f0
	if ctx.cr[6].lt {
	pc = 0x830351F0; continue 'dispatch;
	}
	// 83035230: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83035234: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83035238: 4804A6E9  bl 0x8307f920
	ctx.lr = 0x8303523C;
	sub_8307F920(ctx, base);
	// 8303523C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83035240: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 83035244: 48172F60  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 83035248: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303524C: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83035250: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83035254: 4804A655  bl 0x8307f8a8
	ctx.lr = 0x83035258;
	sub_8307F8A8(ctx, base);
	// 83035258: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 8303525C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035260: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035268: 4E800421  bctrl
	ctx.lr = 0x8303526C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303526C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035270: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 83035274: 40820008  bne 0x8303527c
	if !ctx.cr[0].eq {
	pc = 0x8303527C; continue 'dispatch;
	}
	// 83035278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303527C: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 83035280: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83035284: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83035288: A157A6A4  lhz r10, -0x595c(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303528C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83035290: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83035294: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83035298: 4BFFFFA8  b 0x83035240
	pc = 0x83035240; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303529C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303529C size=44
    let mut pc: u32 = 0x8303529C;
    'dispatch: loop {
        match pc {
            0x8303529C => {
    //   block [0x8303529C..0x830352C8)
	// 8303529C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830352A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830352A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830352A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830352AC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830352B0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830352B4: 4809282D  bl 0x830c7ae0
	ctx.lr = 0x830352B8;
	sub_830C7AE0(ctx, base);
	// 830352B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830352BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830352C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830352C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830352C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830352C8 size=124
    let mut pc: u32 = 0x830352C8;
    'dispatch: loop {
        match pc {
            0x830352C8 => {
    //   block [0x830352C8..0x83035344)
	// 830352C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830352CC: 48172E95  bl 0x831a8160
	ctx.lr = 0x830352D0;
	sub_831A8130(ctx, base);
	// 830352D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830352D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830352D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830352DC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830352E0: 4182005C  beq 0x8303533c
	if ctx.cr[0].eq {
	pc = 0x8303533C; continue 'dispatch;
	}
	// 830352E4: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 830352E8: 3B4000C1  li r26, 0xc1
	ctx.r[26].s64 = 193;
	// 830352EC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830352F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830352F4: 4182003C  beq 0x83035330
	if ctx.cr[0].eq {
	pc = 0x83035330; continue 'dispatch;
	}
	// 830352F8: 4811F309  bl 0x83154600
	ctx.lr = 0x830352FC;
	sub_83154600(ctx, base);
	// 830352FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83035300: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83035304: 4081002C  ble 0x83035330
	if !ctx.cr[0].gt {
	pc = 0x83035330; continue 'dispatch;
	}
	// 83035308: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303530C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035310: 4804A4B1  bl 0x8307f7c0
	ctx.lr = 0x83035314;
	sub_8307F7C0(ctx, base);
	// 83035314: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83035318: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303531C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83035320: 4BFAA071  bl 0x82fdf390
	ctx.lr = 0x83035324;
	sub_82FDF390(ctx, base);
	// 83035324: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83035328: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8303532C: 4198FFDC  blt cr6, 0x83035308
	if ctx.cr[6].lt {
	pc = 0x83035308; continue 'dispatch;
	}
	// 83035330: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83035334: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83035338: 4082FFB4  bne 0x830352ec
	if !ctx.cr[0].eq {
	pc = 0x830352EC; continue 'dispatch;
	}
	// 8303533C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83035340: 48172E70  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035348 size=252
    let mut pc: u32 = 0x83035348;
    'dispatch: loop {
        match pc {
            0x83035348 => {
    //   block [0x83035348..0x83035444)
	// 83035348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303534C: 48172E09  bl 0x831a8154
	ctx.lr = 0x83035350;
	sub_831A8130(ctx, base);
	// 83035350: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035354: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83035358: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8303535C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83035360: 3B630004  addi r27, r3, 4
	ctx.r[27].s64 = ctx.r[3].s64 + 4;
	// 83035364: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035368: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303536C: 418200B4  beq 0x83035420
	if ctx.cr[0].eq {
	pc = 0x83035420; continue 'dispatch;
	}
	// 83035370: 4811F291  bl 0x83154600
	ctx.lr = 0x83035374;
	sub_83154600(ctx, base);
	// 83035374: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83035378: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8303537C: 408100A4  ble 0x83035420
	if !ctx.cr[0].gt {
	pc = 0x83035420; continue 'dispatch;
	}
	// 83035380: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83035384: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035388: 4804A439  bl 0x8307f7c0
	ctx.lr = 0x8303538C;
	sub_8307F7C0(ctx, base);
	// 8303538C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83035390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035394: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83035398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303539C: 4E800421  bctrl
	ctx.lr = 0x830353A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830353A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830353A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830353A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830353AC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830353B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830353B4: 4E800421  bctrl
	ctx.lr = 0x830353B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830353B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830353BC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830353C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830353C4: 4BF9E87D  bl 0x82fd3c40
	ctx.lr = 0x830353C8;
	sub_82FD3C40(ctx, base);
	// 830353C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830353CC: 41820048  beq 0x83035414
	if ctx.cr[0].eq {
	pc = 0x83035414; continue 'dispatch;
	}
	// 830353D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830353D4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830353D8: 4BF9E869  bl 0x82fd3c40
	ctx.lr = 0x830353DC;
	sub_82FD3C40(ctx, base);
	// 830353DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830353E0: 4082005C  bne 0x8303543c
	if !ctx.cr[0].eq {
	pc = 0x8303543C; continue 'dispatch;
	}
	// 830353E4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830353E8: 409A002C  bne cr6, 0x83035414
	if !ctx.cr[6].eq {
	pc = 0x83035414; continue 'dispatch;
	}
	// 830353EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830353F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830353F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830353F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830353FC: 4E800421  bctrl
	ctx.lr = 0x83035400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035400: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83035404: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83035408: 4BF9E839  bl 0x82fd3c40
	ctx.lr = 0x8303540C;
	sub_82FD3C40(ctx, base);
	// 8303540C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83035410: 4082002C  bne 0x8303543c
	if !ctx.cr[0].eq {
	pc = 0x8303543C; continue 'dispatch;
	}
	// 83035414: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83035418: 7F1DD000  cmpw cr6, r29, r26
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8303541C: 4198FF64  blt cr6, 0x83035380
	if ctx.cr[6].lt {
	pc = 0x83035380; continue 'dispatch;
	}
	// 83035420: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83035424: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 83035428: 2F1800C1  cmpwi cr6, r24, 0xc1
	ctx.cr[6].compare_i32(ctx.r[24].s32, 193, &mut ctx.xer);
	// 8303542C: 4198FF38  blt cr6, 0x83035364
	if ctx.cr[6].lt {
	pc = 0x83035364; continue 'dispatch;
	}
	// 83035430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83035434: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83035438: 48172D6C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8303543C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035440: 4BFFFFF4  b 0x83035434
	pc = 0x83035434; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035448 size=864
    let mut pc: u32 = 0x83035448;
    'dispatch: loop {
        match pc {
            0x83035448 => {
    //   block [0x83035448..0x830357A8)
	// 83035448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303544C: 48172CFD  bl 0x831a8148
	ctx.lr = 0x83035450;
	sub_831A8130(ctx, base);
	// 83035450: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83035458: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8303545C: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035464: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303546C: 4E800421  bctrl
	ctx.lr = 0x83035470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035470: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 83035474: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83035478: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303547C: 4BFA9DAD  bl 0x82fdf228
	ctx.lr = 0x83035480;
	sub_82FDF228(ctx, base);
	// 83035480: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035484: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83035488: 419A0070  beq cr6, 0x830354f8
	if ctx.cr[6].eq {
	pc = 0x830354F8; continue 'dispatch;
	}
	// 8303548C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83035490: 419A003C  beq cr6, 0x830354cc
	if ctx.cr[6].eq {
	pc = 0x830354CC; continue 'dispatch;
	}
	// 83035494: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83035498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303549C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830354A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830354A4: 4E800421  bctrl
	ctx.lr = 0x830354A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830354A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830354AC: 41820020  beq 0x830354cc
	if ctx.cr[0].eq {
	pc = 0x830354CC; continue 'dispatch;
	}
	// 830354B0: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 830354B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830354B8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830354BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830354C0: 4E800421  bctrl
	ctx.lr = 0x830354C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830354C4: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830354C8: 4800000C  b 0x830354d4
	pc = 0x830354D4; continue 'dispatch;
	// 830354CC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830354D0: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830354D4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830354D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830354DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830354E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830354E4: 4BFC49ED  bl 0x82ff9ed0
	ctx.lr = 0x830354E8;
	sub_82FF9ED0(ctx, base);
	// 830354E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830354EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830354F0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830354F4: 4817B735  bl 0x831b0c28
	ctx.lr = 0x830354F8;
	sub_831B0C28(ctx, base);
	// 830354F8: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 830354FC: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83035500: A14AA698  lhz r10, -0x5968(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83035504: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83035508: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303550C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83035510: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83035514: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035518: 41820070  beq 0x83035588
	if ctx.cr[0].eq {
	pc = 0x83035588; continue 'dispatch;
	}
	// 8303551C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83035520: 419A003C  beq cr6, 0x8303555c
	if ctx.cr[6].eq {
	pc = 0x8303555C; continue 'dispatch;
	}
	// 83035524: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035528: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303552C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035534: 4E800421  bctrl
	ctx.lr = 0x83035538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035538: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303553C: 41820020  beq 0x8303555c
	if ctx.cr[0].eq {
	pc = 0x8303555C; continue 'dispatch;
	}
	// 83035540: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035544: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035548: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303554C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035550: 4E800421  bctrl
	ctx.lr = 0x83035554;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035554: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83035558: 4800000C  b 0x83035564
	pc = 0x83035564; continue 'dispatch;
	// 8303555C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83035560: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83035564: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83035568: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303556C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83035570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035574: 4BFC495D  bl 0x82ff9ed0
	ctx.lr = 0x83035578;
	sub_82FF9ED0(ctx, base);
	// 83035578: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303557C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035580: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035584: 4817B6A5  bl 0x831b0c28
	ctx.lr = 0x83035588;
	sub_831B0C28(ctx, base);
	// 83035588: 3E808214  lis r20, -0x7dec
	ctx.r[20].s64 = -2112618496;
	// 8303558C: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83035590: A134A6A4  lhz r9, -0x595c(r20)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83035594: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 83035598: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8303559C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 830355A0: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 830355A4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830355A8: 41820070  beq 0x83035618
	if ctx.cr[0].eq {
	pc = 0x83035618; continue 'dispatch;
	}
	// 830355AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830355B0: 419A003C  beq cr6, 0x830355ec
	if ctx.cr[6].eq {
	pc = 0x830355EC; continue 'dispatch;
	}
	// 830355B4: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 830355B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830355BC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830355C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830355C4: 4E800421  bctrl
	ctx.lr = 0x830355C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830355C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830355CC: 41820020  beq 0x830355ec
	if ctx.cr[0].eq {
	pc = 0x830355EC; continue 'dispatch;
	}
	// 830355D0: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 830355D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830355D8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830355DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830355E0: 4E800421  bctrl
	ctx.lr = 0x830355E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830355E4: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830355E8: 4800000C  b 0x830355f4
	pc = 0x830355F4; continue 'dispatch;
	// 830355EC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830355F0: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830355F4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830355F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830355FC: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83035600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035604: 4BFC48CD  bl 0x82ff9ed0
	ctx.lr = 0x83035608;
	sub_82FF9ED0(ctx, base);
	// 83035608: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303560C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035610: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035614: 4817B615  bl 0x831b0c28
	ctx.lr = 0x83035618;
	sub_831B0C28(ctx, base);
	// 83035618: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303561C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035620: A174A6A4  lhz r11, -0x595c(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83035624: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83035628: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8303562C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035630: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83035634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035638: 4E800421  bctrl
	ctx.lr = 0x8303563C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303563C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035640: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83035644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035648: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303564C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035650: 4E800421  bctrl
	ctx.lr = 0x83035654;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035654: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 83035658: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303565C: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 83035660: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035664: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035668: 418200B4  beq 0x8303571c
	if ctx.cr[0].eq {
	pc = 0x8303571C; continue 'dispatch;
	}
	// 8303566C: 4811EF95  bl 0x83154600
	ctx.lr = 0x83035670;
	sub_83154600(ctx, base);
	// 83035670: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83035674: 7C761B79  or. r22, r3, r3
	ctx.r[22].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 83035678: 408100A4  ble 0x8303571c
	if !ctx.cr[0].gt {
	pc = 0x8303571C; continue 'dispatch;
	}
	// 8303567C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83035680: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035684: 4804A13D  bl 0x8307f7c0
	ctx.lr = 0x83035688;
	sub_8307F7C0(ctx, base);
	// 83035688: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303568C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035690: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83035694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035698: 4E800421  bctrl
	ctx.lr = 0x8303569C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303569C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830356A0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830356A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830356A8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830356AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830356B0: 4E800421  bctrl
	ctx.lr = 0x830356B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830356B4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830356B8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830356BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830356C0: 4BF9E581  bl 0x82fd3c40
	ctx.lr = 0x830356C4;
	sub_82FD3C40(ctx, base);
	// 830356C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830356C8: 41820048  beq 0x83035710
	if ctx.cr[0].eq {
	pc = 0x83035710; continue 'dispatch;
	}
	// 830356CC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830356D0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830356D4: 4BF9E56D  bl 0x82fd3c40
	ctx.lr = 0x830356D8;
	sub_82FD3C40(ctx, base);
	// 830356D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830356DC: 40820070  bne 0x8303574c
	if !ctx.cr[0].eq {
	pc = 0x8303574C; continue 'dispatch;
	}
	// 830356E0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830356E4: 409A002C  bne cr6, 0x83035710
	if !ctx.cr[6].eq {
	pc = 0x83035710; continue 'dispatch;
	}
	// 830356E8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830356EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830356F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830356F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830356F8: 4E800421  bctrl
	ctx.lr = 0x830356FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830356FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83035700: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83035704: 4BF9E53D  bl 0x82fd3c40
	ctx.lr = 0x83035708;
	sub_82FD3C40(ctx, base);
	// 83035708: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303570C: 40820040  bne 0x8303574c
	if !ctx.cr[0].eq {
	pc = 0x8303574C; continue 'dispatch;
	}
	// 83035710: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83035714: 7F1CB000  cmpw cr6, r28, r22
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[22].s32, &mut ctx.xer);
	// 83035718: 4198FF64  blt cr6, 0x8303567c
	if ctx.cr[6].lt {
	pc = 0x8303567C; continue 'dispatch;
	}
	// 8303571C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83035720: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 83035724: 2F1800C1  cmpwi cr6, r24, 0xc1
	ctx.cr[6].compare_i32(ctx.r[24].s32, 193, &mut ctx.xer);
	// 83035728: 4198FF38  blt cr6, 0x83035660
	if ctx.cr[6].lt {
	pc = 0x83035660; continue 'dispatch;
	}
	// 8303572C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035730: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83035734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83035738: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303573C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035740: 4E800421  bctrl
	ctx.lr = 0x83035744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035744: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83035748: 48172A50  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 8303574C: 39780001  addi r11, r24, 1
	ctx.r[11].s64 = ctx.r[24].s64 + 1;
	// 83035750: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83035754: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83035758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303575C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83035760: 4804A149  bl 0x8307f8a8
	ctx.lr = 0x83035764;
	sub_8307F8A8(ctx, base);
	// 83035764: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303576C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035774: 4E800421  bctrl
	ctx.lr = 0x83035778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035778: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303577C: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 83035780: 40820008  bne 0x83035788
	if !ctx.cr[0].eq {
	pc = 0x83035788; continue 'dispatch;
	}
	// 83035784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83035788: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8303578C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83035790: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83035794: A154A6A4  lhz r10, -0x595c(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[20].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83035798: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303579C: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 830357A0: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830357A4: 4BFFFFA0  b 0x83035744
	pc = 0x83035744; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830357A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830357A8 size=592
    let mut pc: u32 = 0x830357A8;
    'dispatch: loop {
        match pc {
            0x830357A8 => {
    //   block [0x830357A8..0x830359F8)
	// 830357A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830357AC: 481729A5  bl 0x831a8150
	ctx.lr = 0x830357B0;
	sub_831A8130(ctx, base);
	// 830357B0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830357B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830357B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830357BC: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 830357C0: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 830357C4: A14BA698  lhz r10, -0x5968(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830357C8: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 830357CC: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830357D0: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 830357D4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830357D8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 830357DC: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 830357E0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830357E4: 41820070  beq 0x83035854
	if ctx.cr[0].eq {
	pc = 0x83035854; continue 'dispatch;
	}
	// 830357E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830357EC: 419A003C  beq cr6, 0x83035828
	if ctx.cr[6].eq {
	pc = 0x83035828; continue 'dispatch;
	}
	// 830357F0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830357F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830357F8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830357FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035800: 4E800421  bctrl
	ctx.lr = 0x83035804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035804: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035808: 41820020  beq 0x83035828
	if ctx.cr[0].eq {
	pc = 0x83035828; continue 'dispatch;
	}
	// 8303580C: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035810: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035814: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303581C: 4E800421  bctrl
	ctx.lr = 0x83035820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035820: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83035824: 4800000C  b 0x83035830
	pc = 0x83035830; continue 'dispatch;
	// 83035828: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303582C: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83035830: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83035834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83035838: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303583C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035840: 4BFC4691  bl 0x82ff9ed0
	ctx.lr = 0x83035844;
	sub_82FF9ED0(ctx, base);
	// 83035844: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83035848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303584C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83035850: 4817B3D9  bl 0x831b0c28
	ctx.lr = 0x83035854;
	sub_831B0C28(ctx, base);
	// 83035854: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83035858: 3B5E0004  addi r26, r30, 4
	ctx.r[26].s64 = ctx.r[30].s64 + 4;
	// 8303585C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035860: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035864: 418200B4  beq 0x83035918
	if ctx.cr[0].eq {
	pc = 0x83035918; continue 'dispatch;
	}
	// 83035868: 4811ED99  bl 0x83154600
	ctx.lr = 0x8303586C;
	sub_83154600(ctx, base);
	// 8303586C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83035870: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83035874: 408100A4  ble 0x83035918
	if !ctx.cr[0].gt {
	pc = 0x83035918; continue 'dispatch;
	}
	// 83035878: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303587C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035880: 48049F41  bl 0x8307f7c0
	ctx.lr = 0x83035884;
	sub_8307F7C0(ctx, base);
	// 83035884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83035888: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303588C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83035890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035894: 4E800421  bctrl
	ctx.lr = 0x83035898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035898: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303589C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830358A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830358A4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830358A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830358AC: 4E800421  bctrl
	ctx.lr = 0x830358B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830358B0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830358B4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830358B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830358BC: 4BF9E385  bl 0x82fd3c40
	ctx.lr = 0x830358C0;
	sub_82FD3C40(ctx, base);
	// 830358C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830358C4: 41820048  beq 0x8303590c
	if ctx.cr[0].eq {
	pc = 0x8303590C; continue 'dispatch;
	}
	// 830358C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830358CC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830358D0: 4BF9E371  bl 0x82fd3c40
	ctx.lr = 0x830358D4;
	sub_82FD3C40(ctx, base);
	// 830358D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830358D8: 40820094  bne 0x8303596c
	if !ctx.cr[0].eq {
	pc = 0x8303596C; continue 'dispatch;
	}
	// 830358DC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830358E0: 409A002C  bne cr6, 0x8303590c
	if !ctx.cr[6].eq {
	pc = 0x8303590C; continue 'dispatch;
	}
	// 830358E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830358E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830358EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830358F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830358F4: 4E800421  bctrl
	ctx.lr = 0x830358F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830358F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830358FC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83035900: 4BF9E341  bl 0x82fd3c40
	ctx.lr = 0x83035904;
	sub_82FD3C40(ctx, base);
	// 83035904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83035908: 40820064  bne 0x8303596c
	if !ctx.cr[0].eq {
	pc = 0x8303596C; continue 'dispatch;
	}
	// 8303590C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83035910: 7F1DC800  cmpw cr6, r29, r25
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[25].s32, &mut ctx.xer);
	// 83035914: 4198FF64  blt cr6, 0x83035878
	if ctx.cr[6].lt {
	pc = 0x83035878; continue 'dispatch;
	}
	// 83035918: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8303591C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83035920: 2F1800C1  cmpwi cr6, r24, 0xc1
	ctx.cr[6].compare_i32(ctx.r[24].s32, 193, &mut ctx.xer);
	// 83035924: 4198FF38  blt cr6, 0x8303585c
	if ctx.cr[6].lt {
	pc = 0x8303585C; continue 'dispatch;
	}
	// 83035928: 817E0308  lwz r11, 0x308(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 8303592C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83035930: 419A009C  beq cr6, 0x830359cc
	if ctx.cr[6].eq {
	pc = 0x830359CC; continue 'dispatch;
	}
	// 83035934: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83035938: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303593C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83035940: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035944: 4E800421  bctrl
	ctx.lr = 0x83035948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303594C: 41820080  beq 0x830359cc
	if ctx.cr[0].eq {
	pc = 0x830359CC; continue 'dispatch;
	}
	// 83035950: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035958: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303595C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035960: 4E800421  bctrl
	ctx.lr = 0x83035964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035964: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83035968: 4800006C  b 0x830359d4
	pc = 0x830359D4; continue 'dispatch;
	// 8303596C: 39780001  addi r11, r24, 1
	ctx.r[11].s64 = ctx.r[24].s64 + 1;
	// 83035970: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83035974: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83035978: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8303597C: 48049EDD  bl 0x8307f858
	ctx.lr = 0x83035980;
	sub_8307F858(ctx, base);
	// 83035980: 807E0308  lwz r3, 0x308(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 83035984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035988: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303598C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83035990: 4E800421  bctrl
	ctx.lr = 0x83035994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83035994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035998: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8303599C: 40820008  bne 0x830359a4
	if !ctx.cr[0].eq {
	pc = 0x830359A4; continue 'dispatch;
	}
	// 830359A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830359A4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830359A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830359AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830359B0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 830359B4: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830359B8: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 830359BC: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 830359C0: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830359C4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 830359C8: 481727D8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 830359CC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830359D0: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830359D4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830359D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830359DC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830359E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830359E4: 4BFC44ED  bl 0x82ff9ed0
	ctx.lr = 0x830359E8;
	sub_82FF9ED0(ctx, base);
	// 830359E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830359EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830359F0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830359F4: 4817B235  bl 0x831b0c28
	ctx.lr = 0x830359F8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830359F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830359F8 size=116
    let mut pc: u32 = 0x830359F8;
    'dispatch: loop {
        match pc {
            0x830359F8 => {
    //   block [0x830359F8..0x83035A6C)
	// 830359F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830359FC: 2B040006  cmplwi cr6, r4, 6
	ctx.cr[6].compare_u32(ctx.r[4].u32, 6 as u32, &mut ctx.xer);
	// 83035A00: 4198003C  blt cr6, 0x83035a3c
	if ctx.cr[6].lt {
	pc = 0x83035A3C; continue 'dispatch;
	}
	// 83035A04: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83035A08: 3D208215  lis r9, -0x7deb
	ctx.r[9].s64 = -2112552960;
	// 83035A0C: 390A0006  addi r8, r10, 6
	ctx.r[8].s64 = ctx.r[10].s64 + 6;
	// 83035A10: 39298DA0  addi r9, r9, -0x7260
	ctx.r[9].s64 = ctx.r[9].s64 + -29280;
	// 83035A14: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035A18: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035A1C: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83035A20: 40820014  bne 0x83035a34
	if !ctx.cr[0].eq {
	pc = 0x83035A34; continue 'dispatch;
	}
	// 83035A24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035A28: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83035A2C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83035A30: 409AFFE4  bne cr6, 0x83035a14
	if !ctx.cr[6].eq {
	pc = 0x83035A14; continue 'dispatch;
	}
	// 83035A34: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035A38: 41820224  beq 0x83035c5c
	if ctx.cr[0].eq {
		sub_83035C5C(ctx, base);
		return;
	}
	// 83035A3C: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 83035A40: 4198021C  blt cr6, 0x83035c5c
	if ctx.cr[6].lt {
		sub_83035C5C(ctx, base);
		return;
	}
	// 83035A44: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035A48: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 83035A4C: 40980038  bge cr6, 0x83035a84
	if !ctx.cr[6].lt {
		sub_83035A84(ctx, base);
		return;
	}
	// 83035A50: 2B0A00FE  cmplwi cr6, r10, 0xfe
	ctx.cr[6].compare_u32(ctx.r[10].u32, 254 as u32, &mut ctx.xer);
	// 83035A54: 409A0018  bne cr6, 0x83035a6c
	if !ctx.cr[6].eq {
		sub_83035A6C(ctx, base);
		return;
	}
	// 83035A58: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035A5C: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 83035A60: 409A000C  bne cr6, 0x83035a6c
	if !ctx.cr[6].eq {
		sub_83035A6C(ctx, base);
		return;
	}
	// 83035A64: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83035A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035A6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035A6C size=24
    let mut pc: u32 = 0x83035A6C;
    'dispatch: loop {
        match pc {
            0x83035A6C => {
    //   block [0x83035A6C..0x83035A84)
	// 83035A6C: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 83035A70: 409A01EC  bne cr6, 0x83035c5c
	if !ctx.cr[6].eq {
		sub_83035C5C(ctx, base);
		return;
	}
	// 83035A74: 896B0001  lbz r11, 1(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035A78: 2B0B00FE  cmplwi cr6, r11, 0xfe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 254 as u32, &mut ctx.xer);
	// 83035A7C: 409A01E0  bne cr6, 0x83035c5c
	if !ctx.cr[6].eq {
		sub_83035C5C(ctx, base);
		return;
	}
	// 83035A80: 48000094  b 0x83035b14
	sub_83035AEC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035A84 size=52
    let mut pc: u32 = 0x83035A84;
    'dispatch: loop {
        match pc {
            0x83035A84 => {
    //   block [0x83035A84..0x83035AB8)
	// 83035A84: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035A88: 40820030  bne 0x83035ab8
	if !ctx.cr[0].eq {
		sub_83035AB8(ctx, base);
		return;
	}
	// 83035A8C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035A90: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035A94: 40820024  bne 0x83035ab8
	if !ctx.cr[0].eq {
		sub_83035AB8(ctx, base);
		return;
	}
	// 83035A98: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83035A9C: 2B0900FE  cmplwi cr6, r9, 0xfe
	ctx.cr[6].compare_u32(ctx.r[9].u32, 254 as u32, &mut ctx.xer);
	// 83035AA0: 409A0018  bne cr6, 0x83035ab8
	if !ctx.cr[6].eq {
		sub_83035AB8(ctx, base);
		return;
	}
	// 83035AA4: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83035AA8: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 83035AAC: 409A000C  bne cr6, 0x83035ab8
	if !ctx.cr[6].eq {
		sub_83035AB8(ctx, base);
		return;
	}
	// 83035AB0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83035AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035AB8 size=52
    let mut pc: u32 = 0x83035AB8;
    'dispatch: loop {
        match pc {
            0x83035AB8 => {
    //   block [0x83035AB8..0x83035AEC)
	// 83035AB8: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 83035ABC: 409A0030  bne cr6, 0x83035aec
	if !ctx.cr[6].eq {
		sub_83035AEC(ctx, base);
		return;
	}
	// 83035AC0: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035AC4: 2B0900FE  cmplwi cr6, r9, 0xfe
	ctx.cr[6].compare_u32(ctx.r[9].u32, 254 as u32, &mut ctx.xer);
	// 83035AC8: 409A0024  bne cr6, 0x83035aec
	if !ctx.cr[6].eq {
		sub_83035AEC(ctx, base);
		return;
	}
	// 83035ACC: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83035AD0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035AD4: 40820018  bne 0x83035aec
	if !ctx.cr[0].eq {
		sub_83035AEC(ctx, base);
		return;
	}
	// 83035AD8: 892B0003  lbz r9, 3(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83035ADC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83035AE0: 4082000C  bne 0x83035aec
	if !ctx.cr[0].eq {
		sub_83035AEC(ctx, base);
		return;
	}
	// 83035AE4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83035AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035AEC size=48
    let mut pc: u32 = 0x83035AEC;
    'dispatch: loop {
        match pc {
            0x83035AEC => {
    //   block [0x83035AEC..0x83035B1C)
	// 83035AEC: 2B0A00FE  cmplwi cr6, r10, 0xfe
	ctx.cr[6].compare_u32(ctx.r[10].u32, 254 as u32, &mut ctx.xer);
	// 83035AF0: 409A0010  bne cr6, 0x83035b00
	if !ctx.cr[6].eq {
	pc = 0x83035B00; continue 'dispatch;
	}
	// 83035AF4: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035AF8: 2B0900FF  cmplwi cr6, r9, 0xff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 255 as u32, &mut ctx.xer);
	// 83035AFC: 419AFF68  beq cr6, 0x83035a64
	if ctx.cr[6].eq {
		sub_830359F8(ctx, base);
		return;
	}
	// 83035B00: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 83035B04: 409A0018  bne cr6, 0x83035b1c
	if !ctx.cr[6].eq {
		sub_83035B1C(ctx, base);
		return;
	}
	// 83035B08: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83035B0C: 2B0900FE  cmplwi cr6, r9, 0xfe
	ctx.cr[6].compare_u32(ctx.r[9].u32, 254 as u32, &mut ctx.xer);
	// 83035B10: 409A000C  bne cr6, 0x83035b1c
	if !ctx.cr[6].eq {
		sub_83035B1C(ctx, base);
		return;
	}
	// 83035B14: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83035B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035B1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035B1C size=320
    let mut pc: u32 = 0x83035B1C;
    'dispatch: loop {
        match pc {
            0x83035B1C => {
    //   block [0x83035B1C..0x83035C5C)
	// 83035B1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83035B20: 419A000C  beq cr6, 0x83035b2c
	if ctx.cr[6].eq {
	pc = 0x83035B2C; continue 'dispatch;
	}
	// 83035B24: 2B0A003C  cmplwi cr6, r10, 0x3c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 60 as u32, &mut ctx.xer);
	// 83035B28: 409A00F4  bne cr6, 0x83035c1c
	if !ctx.cr[6].eq {
	pc = 0x83035C1C; continue 'dispatch;
	}
	// 83035B2C: 2B040018  cmplwi cr6, r4, 0x18
	ctx.cr[6].compare_u32(ctx.r[4].u32, 24 as u32, &mut ctx.xer);
	// 83035B30: 41980074  blt cr6, 0x83035ba4
	if ctx.cr[6].lt {
	pc = 0x83035BA4; continue 'dispatch;
	}
	// 83035B34: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83035B38: 3D208215  lis r9, -0x7deb
	ctx.r[9].s64 = -2112552960;
	// 83035B3C: 390A0018  addi r8, r10, 0x18
	ctx.r[8].s64 = ctx.r[10].s64 + 24;
	// 83035B40: 39298DD4  addi r9, r9, -0x722c
	ctx.r[9].s64 = ctx.r[9].s64 + -29228;
	// 83035B44: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035B48: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035B4C: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83035B50: 40820014  bne 0x83035b64
	if !ctx.cr[0].eq {
	pc = 0x83035B64; continue 'dispatch;
	}
	// 83035B54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035B58: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83035B5C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83035B60: 409AFFE4  bne cr6, 0x83035b44
	if !ctx.cr[6].eq {
	pc = 0x83035B44; continue 'dispatch;
	}
	// 83035B64: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035B68: 4182FF48  beq 0x83035ab0
	if ctx.cr[0].eq {
		sub_83035A84(ctx, base);
		return;
	}
	// 83035B6C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83035B70: 3D208215  lis r9, -0x7deb
	ctx.r[9].s64 = -2112552960;
	// 83035B74: 390A0018  addi r8, r10, 0x18
	ctx.r[8].s64 = ctx.r[10].s64 + 24;
	// 83035B78: 39298DEC  addi r9, r9, -0x7214
	ctx.r[9].s64 = ctx.r[9].s64 + -29204;
	// 83035B7C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035B80: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035B84: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83035B88: 40820014  bne 0x83035b9c
	if !ctx.cr[0].eq {
	pc = 0x83035B9C; continue 'dispatch;
	}
	// 83035B8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035B90: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83035B94: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83035B98: 409AFFE4  bne cr6, 0x83035b7c
	if !ctx.cr[6].eq {
	pc = 0x83035B7C; continue 'dispatch;
	}
	// 83035B9C: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035BA0: 4182FF44  beq 0x83035ae4
	if ctx.cr[0].eq {
		sub_83035AB8(ctx, base);
		return;
	}
	// 83035BA4: 2B04000C  cmplwi cr6, r4, 0xc
	ctx.cr[6].compare_u32(ctx.r[4].u32, 12 as u32, &mut ctx.xer);
	// 83035BA8: 41980074  blt cr6, 0x83035c1c
	if ctx.cr[6].lt {
	pc = 0x83035C1C; continue 'dispatch;
	}
	// 83035BAC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83035BB0: 3D208215  lis r9, -0x7deb
	ctx.r[9].s64 = -2112552960;
	// 83035BB4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 83035BB8: 39298DB8  addi r9, r9, -0x7248
	ctx.r[9].s64 = ctx.r[9].s64 + -29256;
	// 83035BBC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035BC0: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035BC4: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83035BC8: 40820014  bne 0x83035bdc
	if !ctx.cr[0].eq {
	pc = 0x83035BDC; continue 'dispatch;
	}
	// 83035BCC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035BD0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83035BD4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83035BD8: 409AFFE4  bne cr6, 0x83035bbc
	if !ctx.cr[6].eq {
	pc = 0x83035BBC; continue 'dispatch;
	}
	// 83035BDC: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035BE0: 4182FE84  beq 0x83035a64
	if ctx.cr[0].eq {
		sub_830359F8(ctx, base);
		return;
	}
	// 83035BE4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83035BE8: 3D208215  lis r9, -0x7deb
	ctx.r[9].s64 = -2112552960;
	// 83035BEC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 83035BF0: 39298DC4  addi r9, r9, -0x723c
	ctx.r[9].s64 = ctx.r[9].s64 + -29244;
	// 83035BF4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035BF8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035BFC: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83035C00: 40820014  bne 0x83035c14
	if !ctx.cr[0].eq {
	pc = 0x83035C14; continue 'dispatch;
	}
	// 83035C04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035C08: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83035C0C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83035C10: 409AFFE4  bne cr6, 0x83035bf4
	if !ctx.cr[6].eq {
	pc = 0x83035BF4; continue 'dispatch;
	}
	// 83035C14: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035C18: 4182FEFC  beq 0x83035b14
	if ctx.cr[0].eq {
		sub_83035AEC(ctx, base);
		return;
	}
	// 83035C1C: 2B040006  cmplwi cr6, r4, 6
	ctx.cr[6].compare_u32(ctx.r[4].u32, 6 as u32, &mut ctx.xer);
	// 83035C20: 4099003C  ble cr6, 0x83035c5c
	if !ctx.cr[6].gt {
		sub_83035C5C(ctx, base);
		return;
	}
	// 83035C24: 3D408215  lis r10, -0x7deb
	ctx.r[10].s64 = -2112552960;
	// 83035C28: 392B0006  addi r9, r11, 6
	ctx.r[9].s64 = ctx.r[11].s64 + 6;
	// 83035C2C: 394A8DAC  addi r10, r10, -0x7254
	ctx.r[10].s64 = ctx.r[10].s64 + -29268;
	// 83035C30: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035C34: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83035C38: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83035C3C: 40820014  bne 0x83035c50
	if !ctx.cr[0].eq {
	pc = 0x83035C50; continue 'dispatch;
	}
	// 83035C40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83035C44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83035C48: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83035C4C: 409AFFE4  bne cr6, 0x83035c30
	if !ctx.cr[6].eq {
	pc = 0x83035C30; continue 'dispatch;
	}
	// 83035C50: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035C54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83035C58: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035C5C size=8
    let mut pc: u32 = 0x83035C5C;
    'dispatch: loop {
        match pc {
            0x83035C5C => {
    //   block [0x83035C5C..0x83035C64)
	// 83035C5C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83035C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035C68 size=464
    let mut pc: u32 = 0x83035C68;
    'dispatch: loop {
        match pc {
            0x83035C68 => {
    //   block [0x83035C68..0x83035E38)
	// 83035C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83035C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83035C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83035C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035C78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83035C80: 388B7F08  addi r4, r11, 0x7f08
	ctx.r[4].s64 = ctx.r[11].s64 + 32520;
	// 83035C84: 7F1F2040  cmplw cr6, r31, r4
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83035C88: 419A0198  beq cr6, 0x83035e20
	if ctx.cr[6].eq {
	pc = 0x83035E20; continue 'dispatch;
	}
	// 83035C8C: 4BF9BD3D  bl 0x82fd19c8
	ctx.lr = 0x83035C90;
	sub_82FD19C8(ctx, base);
	// 83035C90: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035C94: 4182018C  beq 0x83035e20
	if ctx.cr[0].eq {
	pc = 0x83035E20; continue 'dispatch;
	}
	// 83035C98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035CA0: 388B7DC8  addi r4, r11, 0x7dc8
	ctx.r[4].s64 = ctx.r[11].s64 + 32200;
	// 83035CA4: 4BF9BD25  bl 0x82fd19c8
	ctx.lr = 0x83035CA8;
	sub_82FD19C8(ctx, base);
	// 83035CA8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035CAC: 4182016C  beq 0x83035e18
	if ctx.cr[0].eq {
	pc = 0x83035E18; continue 'dispatch;
	}
	// 83035CB0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035CB8: 388B7DD4  addi r4, r11, 0x7dd4
	ctx.r[4].s64 = ctx.r[11].s64 + 32212;
	// 83035CBC: 4BF9BD0D  bl 0x82fd19c8
	ctx.lr = 0x83035CC0;
	sub_82FD19C8(ctx, base);
	// 83035CC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035CC4: 41820154  beq 0x83035e18
	if ctx.cr[0].eq {
	pc = 0x83035E18; continue 'dispatch;
	}
	// 83035CC8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035CD0: 388B7D84  addi r4, r11, 0x7d84
	ctx.r[4].s64 = ctx.r[11].s64 + 32132;
	// 83035CD4: 4BF9BCF5  bl 0x82fd19c8
	ctx.lr = 0x83035CD8;
	sub_82FD19C8(ctx, base);
	// 83035CD8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035CDC: 41820134  beq 0x83035e10
	if ctx.cr[0].eq {
	pc = 0x83035E10; continue 'dispatch;
	}
	// 83035CE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035CE8: 388B7D98  addi r4, r11, 0x7d98
	ctx.r[4].s64 = ctx.r[11].s64 + 32152;
	// 83035CEC: 4BF9BCDD  bl 0x82fd19c8
	ctx.lr = 0x83035CF0;
	sub_82FD19C8(ctx, base);
	// 83035CF0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035CF4: 4182011C  beq 0x83035e10
	if ctx.cr[0].eq {
	pc = 0x83035E10; continue 'dispatch;
	}
	// 83035CF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D00: 388B7DA8  addi r4, r11, 0x7da8
	ctx.r[4].s64 = ctx.r[11].s64 + 32168;
	// 83035D04: 4BF9BCC5  bl 0x82fd19c8
	ctx.lr = 0x83035D08;
	sub_82FD19C8(ctx, base);
	// 83035D08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D0C: 41820104  beq 0x83035e10
	if ctx.cr[0].eq {
	pc = 0x83035E10; continue 'dispatch;
	}
	// 83035D10: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D18: 388B7DB4  addi r4, r11, 0x7db4
	ctx.r[4].s64 = ctx.r[11].s64 + 32180;
	// 83035D1C: 4BF9BCAD  bl 0x82fd19c8
	ctx.lr = 0x83035D20;
	sub_82FD19C8(ctx, base);
	// 83035D20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D24: 418200EC  beq 0x83035e10
	if ctx.cr[0].eq {
	pc = 0x83035E10; continue 'dispatch;
	}
	// 83035D28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D30: 388B7E58  addi r4, r11, 0x7e58
	ctx.r[4].s64 = ctx.r[11].s64 + 32344;
	// 83035D34: 4BF9BC95  bl 0x82fd19c8
	ctx.lr = 0x83035D38;
	sub_82FD19C8(ctx, base);
	// 83035D38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D3C: 418200CC  beq 0x83035e08
	if ctx.cr[0].eq {
	pc = 0x83035E08; continue 'dispatch;
	}
	// 83035D40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D48: 388B7E70  addi r4, r11, 0x7e70
	ctx.r[4].s64 = ctx.r[11].s64 + 32368;
	// 83035D4C: 4BF9BC7D  bl 0x82fd19c8
	ctx.lr = 0x83035D50;
	sub_82FD19C8(ctx, base);
	// 83035D50: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D54: 418200B4  beq 0x83035e08
	if ctx.cr[0].eq {
	pc = 0x83035E08; continue 'dispatch;
	}
	// 83035D58: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D60: 388B7E2C  addi r4, r11, 0x7e2c
	ctx.r[4].s64 = ctx.r[11].s64 + 32300;
	// 83035D64: 4BF9BC65  bl 0x82fd19c8
	ctx.lr = 0x83035D68;
	sub_82FD19C8(ctx, base);
	// 83035D68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D6C: 41820094  beq 0x83035e00
	if ctx.cr[0].eq {
	pc = 0x83035E00; continue 'dispatch;
	}
	// 83035D70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D78: 388B7E44  addi r4, r11, 0x7e44
	ctx.r[4].s64 = ctx.r[11].s64 + 32324;
	// 83035D7C: 4BF9BC4D  bl 0x82fd19c8
	ctx.lr = 0x83035D80;
	sub_82FD19C8(ctx, base);
	// 83035D80: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D84: 4182007C  beq 0x83035e00
	if ctx.cr[0].eq {
	pc = 0x83035E00; continue 'dispatch;
	}
	// 83035D88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035D90: 388B7D5C  addi r4, r11, 0x7d5c
	ctx.r[4].s64 = ctx.r[11].s64 + 32092;
	// 83035D94: 4BF9BC35  bl 0x82fd19c8
	ctx.lr = 0x83035D98;
	sub_82FD19C8(ctx, base);
	// 83035D98: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035D9C: 4182005C  beq 0x83035df8
	if ctx.cr[0].eq {
	pc = 0x83035DF8; continue 'dispatch;
	}
	// 83035DA0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035DA8: 388B7D74  addi r4, r11, 0x7d74
	ctx.r[4].s64 = ctx.r[11].s64 + 32116;
	// 83035DAC: 4BF9BC1D  bl 0x82fd19c8
	ctx.lr = 0x83035DB0;
	sub_82FD19C8(ctx, base);
	// 83035DB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035DB4: 41820044  beq 0x83035df8
	if ctx.cr[0].eq {
	pc = 0x83035DF8; continue 'dispatch;
	}
	// 83035DB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035DC0: 388B7D34  addi r4, r11, 0x7d34
	ctx.r[4].s64 = ctx.r[11].s64 + 32052;
	// 83035DC4: 4BF9BC05  bl 0x82fd19c8
	ctx.lr = 0x83035DC8;
	sub_82FD19C8(ctx, base);
	// 83035DC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035DCC: 41820024  beq 0x83035df0
	if ctx.cr[0].eq {
	pc = 0x83035DF0; continue 'dispatch;
	}
	// 83035DD0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83035DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035DD8: 388B7D4C  addi r4, r11, 0x7d4c
	ctx.r[4].s64 = ctx.r[11].s64 + 32076;
	// 83035DDC: 4BF9BBED  bl 0x82fd19c8
	ctx.lr = 0x83035DE0;
	sub_82FD19C8(ctx, base);
	// 83035DE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83035DE4: 4182000C  beq 0x83035df0
	if ctx.cr[0].eq {
	pc = 0x83035DF0; continue 'dispatch;
	}
	// 83035DE8: 386003E7  li r3, 0x3e7
	ctx.r[3].s64 = 999;
	// 83035DEC: 48000038  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035DF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83035DF4: 48000030  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035DF8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83035DFC: 48000028  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035E00: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83035E04: 48000020  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035E08: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83035E0C: 48000018  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035E10: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83035E14: 48000010  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035E18: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83035E1C: 48000008  b 0x83035e24
	pc = 0x83035E24; continue 'dispatch;
	// 83035E20: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 83035E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83035E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83035E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83035E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83035E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035E38 size=96
    let mut pc: u32 = 0x83035E38;
    'dispatch: loop {
        match pc {
            0x83035E38 => {
    //   block [0x83035E38..0x83035E98)
	// 83035E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83035E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83035E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035E44: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 83035E48: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 83035E4C: 4198002C  blt cr6, 0x83035e78
	if ctx.cr[6].lt {
	pc = 0x83035E78; continue 'dispatch;
	}
	// 83035E50: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035E54: 38C00077  li r6, 0x77
	ctx.r[6].s64 = 119;
	// 83035E58: 388B8E10  addi r4, r11, -0x71f0
	ctx.r[4].s64 = ctx.r[11].s64 + -29168;
	// 83035E5C: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 83035E60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035E64: 4BF9B1F5  bl 0x82fd1058
	ctx.lr = 0x83035E68;
	sub_82FD1058(ctx, base);
	// 83035E68: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83035E6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83035E70: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83035E74: 4817ADB5  bl 0x831b0c28
	ctx.lr = 0x83035E78;
	sub_831B0C28(ctx, base);
	// 83035E78: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83035E7C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83035E80: 396B3274  addi r11, r11, 0x3274
	ctx.r[11].s64 = ctx.r[11].s64 + 12916;
	// 83035E84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83035E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83035E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83035E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83035E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035E98 size=20
    let mut pc: u32 = 0x83035E98;
    'dispatch: loop {
        match pc {
            0x83035E98 => {
    //   block [0x83035E98..0x83035EAC)
	// 83035E98: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035E9C: 546A0DFC  rlwinm r10, r3, 1, 0x17, 0x1e
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83035EA0: 396B8E48  addi r11, r11, -0x71b8
	ctx.r[11].s64 = ctx.r[11].s64 + -29112;
	// 83035EA4: 7C6A5A2E  lhzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83035EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035EB0 size=84
    let mut pc: u32 = 0x83035EB0;
    'dispatch: loop {
        match pc {
            0x83035EB0 => {
    //   block [0x83035EB0..0x83035F04)
	// 83035EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83035EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83035EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83035EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035EC0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035EC4: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 83035EC8: 396B8E48  addi r11, r11, -0x71b8
	ctx.r[11].s64 = ctx.r[11].s64 + -29112;
	// 83035ECC: 39000160  li r8, 0x160
	ctx.r[8].s64 = 352;
	// 83035ED0: 38EB0200  addi r7, r11, 0x200
	ctx.r[7].s64 = ctx.r[11].s64 + 512;
	// 83035ED4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83035ED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83035EDC: 48049B8D  bl 0x8307fa68
	ctx.lr = 0x83035EE0;
	sub_8307FA68(ctx, base);
	// 83035EE0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035EE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035EE8: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 83035EEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83035EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83035EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83035EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83035EFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83035F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83035F08 size=88
    let mut pc: u32 = 0x83035F08;
    'dispatch: loop {
        match pc {
            0x83035F08 => {
    //   block [0x83035F08..0x83035F60)
	// 83035F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83035F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83035F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83035F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83035F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83035F1C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83035F24: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 83035F28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83035F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83035F30: 48049A49  bl 0x8307f978
	ctx.lr = 0x83035F34;
	sub_8307F978(ctx, base);
	// 83035F34: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83035F38: 4182000C  beq 0x83035f44
	if ctx.cr[0].eq {
	pc = 0x83035F44; continue 'dispatch;
	}
	// 83035F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035F40: 4BFA23A1  bl 0x82fd82e0
	ctx.lr = 0x83035F44;
	sub_82FD82E0(ctx, base);
	// 83035F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83035F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83035F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83035F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83035F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83035F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83035F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035F60 size=56
    let mut pc: u32 = 0x83035F60;
    'dispatch: loop {
        match pc {
            0x83035F60 => {
    //   block [0x83035F60..0x83035F98)
	// 83035F60: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83035F64: 4198007C  blt cr6, 0x83035fe0
	if ctx.cr[6].lt {
		sub_83035FE0(ctx, base);
		return;
	}
	// 83035F68: 419A006C  beq cr6, 0x83035fd4
	if ctx.cr[6].eq {
		sub_83035FD4(ctx, base);
		return;
	}
	// 83035F6C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83035F70: 41980058  blt cr6, 0x83035fc8
	if ctx.cr[6].lt {
		sub_83035FC8(ctx, base);
		return;
	}
	// 83035F74: 419A0048  beq cr6, 0x83035fbc
	if ctx.cr[6].eq {
		sub_83035FBC(ctx, base);
		return;
	}
	// 83035F78: 2B030005  cmplwi cr6, r3, 5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 5 as u32, &mut ctx.xer);
	// 83035F7C: 41980034  blt cr6, 0x83035fb0
	if ctx.cr[6].lt {
		sub_83035FB0(ctx, base);
		return;
	}
	// 83035F80: 419A0024  beq cr6, 0x83035fa4
	if ctx.cr[6].eq {
		sub_83035FA4(ctx, base);
		return;
	}
	// 83035F84: 2B030007  cmplwi cr6, r3, 7
	ctx.cr[6].compare_u32(ctx.r[3].u32, 7 as u32, &mut ctx.xer);
	// 83035F88: 41980010  blt cr6, 0x83035f98
	if ctx.cr[6].lt {
		sub_83035F98(ctx, base);
		return;
	}
	// 83035F8C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035F90: 386B96CC  addi r3, r11, -0x6934
	ctx.r[3].s64 = ctx.r[11].s64 + -26932;
	// 83035F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035F98 size=12
    let mut pc: u32 = 0x83035F98;
    'dispatch: loop {
        match pc {
            0x83035F98 => {
    //   block [0x83035F98..0x83035FA4)
	// 83035F98: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035F9C: 386B96A4  addi r3, r11, -0x695c
	ctx.r[3].s64 = ctx.r[11].s64 + -26972;
	// 83035FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FA4 size=12
    let mut pc: u32 = 0x83035FA4;
    'dispatch: loop {
        match pc {
            0x83035FA4 => {
    //   block [0x83035FA4..0x83035FB0)
	// 83035FA4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FA8: 386B9680  addi r3, r11, -0x6980
	ctx.r[3].s64 = ctx.r[11].s64 + -27008;
	// 83035FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FB0 size=12
    let mut pc: u32 = 0x83035FB0;
    'dispatch: loop {
        match pc {
            0x83035FB0 => {
    //   block [0x83035FB0..0x83035FBC)
	// 83035FB0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FB4: 386B9664  addi r3, r11, -0x699c
	ctx.r[3].s64 = ctx.r[11].s64 + -27036;
	// 83035FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FBC size=12
    let mut pc: u32 = 0x83035FBC;
    'dispatch: loop {
        match pc {
            0x83035FBC => {
    //   block [0x83035FBC..0x83035FC8)
	// 83035FBC: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FC0: 386B964C  addi r3, r11, -0x69b4
	ctx.r[3].s64 = ctx.r[11].s64 + -27060;
	// 83035FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FC8 size=12
    let mut pc: u32 = 0x83035FC8;
    'dispatch: loop {
        match pc {
            0x83035FC8 => {
    //   block [0x83035FC8..0x83035FD4)
	// 83035FC8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FCC: 386B962C  addi r3, r11, -0x69d4
	ctx.r[3].s64 = ctx.r[11].s64 + -27092;
	// 83035FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FD4 size=12
    let mut pc: u32 = 0x83035FD4;
    'dispatch: loop {
        match pc {
            0x83035FD4 => {
    //   block [0x83035FD4..0x83035FE0)
	// 83035FD4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FD8: 386B9600  addi r3, r11, -0x6a00
	ctx.r[3].s64 = ctx.r[11].s64 + -27136;
	// 83035FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FE0 size=12
    let mut pc: u32 = 0x83035FE0;
    'dispatch: loop {
        match pc {
            0x83035FE0 => {
    //   block [0x83035FE0..0x83035FEC)
	// 83035FE0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FE4: 386B95D8  addi r3, r11, -0x6a28
	ctx.r[3].s64 = ctx.r[11].s64 + -27176;
	// 83035FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83035FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83035FF0 size=16
    let mut pc: u32 = 0x83035FF0;
    'dispatch: loop {
        match pc {
            0x83035FF0 => {
    //   block [0x83035FF0..0x83036000)
	// 83035FF0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83035FF4: 396B96DC  addi r11, r11, -0x6924
	ctx.r[11].s64 = ctx.r[11].s64 + -26916;
	// 83035FF8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83035FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036000 size=68
    let mut pc: u32 = 0x83036000;
    'dispatch: loop {
        match pc {
            0x83036000 => {
    //   block [0x83036000..0x83036044)
	// 83036000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303600C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036010: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036018: 396B96DC  addi r11, r11, -0x6924
	ctx.r[11].s64 = ctx.r[11].s64 + -26916;
	// 8303601C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83036020: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036024: 41820008  beq 0x8303602c
	if ctx.cr[0].eq {
	pc = 0x8303602C; continue 'dispatch;
	}
	// 83036028: 4BFA22B9  bl 0x82fd82e0
	ctx.lr = 0x8303602C;
	sub_82FD82E0(ctx, base);
	// 8303602C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303603C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036048 size=8
    let mut pc: u32 = 0x83036048;
    'dispatch: loop {
        match pc {
            0x83036048 => {
    //   block [0x83036048..0x83036050)
	// 83036048: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303604C: 82149718  lwz r16, -0x68e8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26856 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036050 size=268
    let mut pc: u32 = 0x83036050;
    'dispatch: loop {
        match pc {
            0x83036050 => {
    //   block [0x83036050..0x8303615C)
	// 83036050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036054: 48172115  bl 0x831a8168
	ctx.lr = 0x83036058;
	sub_831A8130(ctx, base);
	// 83036058: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303605C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036060: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83036064: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83036068: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303606C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036070: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83036074: 396B96F8  addi r11, r11, -0x6908
	ctx.r[11].s64 = ctx.r[11].s64 + -26888;
	// 83036078: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8303607C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036080: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83036084: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83036088: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8303608C: 816BB984  lwz r11, -0x467c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18044 as u32) ) } as u64;
	// 83036090: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83036094: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83036098: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303609C: 409A0014  bne cr6, 0x830360b0
	if !ctx.cr[6].eq {
	pc = 0x830360B0; continue 'dispatch;
	}
	// 830360A0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830360A4: 386B96F0  addi r3, r11, -0x6910
	ctx.r[3].s64 = ctx.r[11].s64 + -26896;
	// 830360A8: 4818F801  bl 0x831c58a8
	ctx.lr = 0x830360AC;
	sub_831C58A8(ctx, base);
	// 830360AC: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830360B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830360B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830360B8: 808BB7E8  lwz r4, -0x4818(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830360BC: 4BF9AAC5  bl 0x82fd0b80
	ctx.lr = 0x830360C0;
	sub_82FD0B80(ctx, base);
	// 830360C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830360C4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830360C8: 388B8070  addi r4, r11, -0x7f90
	ctx.r[4].s64 = ctx.r[11].s64 + -32656;
	// 830360CC: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830360D0: 4BF9DB71  bl 0x82fd3c40
	ctx.lr = 0x830360D4;
	sub_82FD3C40(ctx, base);
	// 830360D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830360D8: 4182000C  beq 0x830360e4
	if ctx.cr[0].eq {
	pc = 0x830360E4; continue 'dispatch;
	}
	// 830360DC: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830360E0: 48000070  b 0x83036150
	pc = 0x83036150; continue 'dispatch;
	// 830360E4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830360E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830360EC: 388B79B8  addi r4, r11, 0x79b8
	ctx.r[4].s64 = ctx.r[11].s64 + 31160;
	// 830360F0: 4BF9DB51  bl 0x82fd3c40
	ctx.lr = 0x830360F4;
	sub_82FD3C40(ctx, base);
	// 830360F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830360F8: 4182000C  beq 0x83036104
	if ctx.cr[0].eq {
	pc = 0x83036104; continue 'dispatch;
	}
	// 830360FC: 39602000  li r11, 0x2000
	ctx.r[11].s64 = 8192;
	// 83036100: 48000040  b 0x83036140
	pc = 0x83036140; continue 'dispatch;
	// 83036104: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83036108: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303610C: 388B7EB0  addi r4, r11, 0x7eb0
	ctx.r[4].s64 = ctx.r[11].s64 + 32432;
	// 83036110: 4BF9DB31  bl 0x82fd3c40
	ctx.lr = 0x83036114;
	sub_82FD3C40(ctx, base);
	// 83036114: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036118: 4182000C  beq 0x83036124
	if ctx.cr[0].eq {
	pc = 0x83036124; continue 'dispatch;
	}
	// 8303611C: 39604000  li r11, 0x4000
	ctx.r[11].s64 = 16384;
	// 83036120: 48000020  b 0x83036140
	pc = 0x83036140; continue 'dispatch;
	// 83036124: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83036128: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303612C: 388B9490  addi r4, r11, -0x6b70
	ctx.r[4].s64 = ctx.r[11].s64 + -27504;
	// 83036130: 4BF9DB11  bl 0x82fd3c40
	ctx.lr = 0x83036134;
	sub_82FD3C40(ctx, base);
	// 83036134: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036138: 41820010  beq 0x83036148
	if ctx.cr[0].eq {
	pc = 0x83036148; continue 'dispatch;
	}
	// 8303613C: 39606000  li r11, 0x6000
	ctx.r[11].s64 = 24576;
	// 83036140: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83036144: 4800000C  b 0x83036150
	pc = 0x83036150; continue 'dispatch;
	// 83036148: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8303614C: 4BFBE8FD  bl 0x82ff4a48
	ctx.lr = 0x83036150;
	sub_82FF4A48(ctx, base);
	// 83036150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036154: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83036158: 48172060  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303615C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303615C size=40
    let mut pc: u32 = 0x8303615C;
    'dispatch: loop {
        match pc {
            0x8303615C => {
    //   block [0x8303615C..0x83036184)
	// 8303615C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83036160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303616C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83036170: 4BFFFE81  bl 0x83035ff0
	ctx.lr = 0x83036174;
	sub_83035FF0(ctx, base);
	// 83036174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303617C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036188 size=8
    let mut pc: u32 = 0x83036188;
    'dispatch: loop {
        match pc {
            0x83036188 => {
    //   block [0x83036188..0x83036190)
	// 83036188: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303618C: 82149750  lwz r16, -0x68b0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036190 size=108
    let mut pc: u32 = 0x83036190;
    'dispatch: loop {
        match pc {
            0x83036190 => {
    //   block [0x83036190..0x830361FC)
	// 83036190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303619C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830361A0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830361A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830361A8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830361AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830361B0: 396B96F8  addi r11, r11, -0x6908
	ctx.r[11].s64 = ctx.r[11].s64 + -26888;
	// 830361B4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830361B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830361BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830361C0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830361C4: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830361C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830361CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830361D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830361D4: 4E800421  bctrl
	ctx.lr = 0x830361D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830361D8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830361DC: 396B96DC  addi r11, r11, -0x6924
	ctx.r[11].s64 = ctx.r[11].s64 + -26916;
	// 830361E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830361E4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830361E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830361EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830361F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830361F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830361F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830361FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830361FC size=40
    let mut pc: u32 = 0x830361FC;
    'dispatch: loop {
        match pc {
            0x830361FC => {
    //   block [0x830361FC..0x83036224)
	// 830361FC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83036200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303620C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83036210: 4BFFFDE1  bl 0x83035ff0
	ctx.lr = 0x83036214;
	sub_83035FF0(ctx, base);
	// 83036214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303621C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036228 size=112
    let mut pc: u32 = 0x83036228;
    'dispatch: loop {
        match pc {
            0x83036228 => {
    //   block [0x83036228..0x83036298)
	// 83036228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303622C: 48171F35  bl 0x831a8160
	ctx.lr = 0x83036230;
	sub_831A8130(ctx, base);
	// 83036230: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036234: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036238: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8303623C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83036240: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83036244: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 83036248: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 8303624C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83036250: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 83036254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036258: 4E800421  bctrl
	ctx.lr = 0x8303625C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303625C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036260: 4082000C  bne 0x8303626c
	if !ctx.cr[0].eq {
	pc = 0x8303626C; continue 'dispatch;
	}
	// 83036264: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036268: 48000028  b 0x83036290
	pc = 0x83036290; continue 'dispatch;
	// 8303626C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 83036270: 812100E4  lwz r9, 0xe4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 83036274: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83036278: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303627C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83036280: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83036284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036288: 4BF9CFE1  bl 0x82fd3268
	ctx.lr = 0x8303628C;
	sub_82FD3268(ctx, base);
	// 8303628C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83036290: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83036294: 48171F1C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036298 size=352
    let mut pc: u32 = 0x83036298;
    'dispatch: loop {
        match pc {
            0x83036298 => {
    //   block [0x83036298..0x830363F8)
	// 83036298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303629C: 48171EAD  bl 0x831a8148
	ctx.lr = 0x830362A0;
	sub_831A8130(ctx, base);
	// 830362A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830362A4: 83E10114  lwz r31, 0x114(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 830362A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830362AC: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 830362B0: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 830362B4: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 830362B8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830362BC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 830362C0: 7D545378  mr r20, r10
	ctx.r[20].u64 = ctx.r[10].u64;
	// 830362C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830362C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830362CC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830362D0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 830362D4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 830362D8: 419A0014  beq cr6, 0x830362ec
	if ctx.cr[6].eq {
	pc = 0x830362EC; continue 'dispatch;
	}
	// 830362DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830362E0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 830362E4: 4BF9B12D  bl 0x82fd1410
	ctx.lr = 0x830362E8;
	sub_82FD1410(ctx, base);
	// 830362E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830362EC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830362F0: 419A0014  beq cr6, 0x83036304
	if ctx.cr[6].eq {
	pc = 0x83036304; continue 'dispatch;
	}
	// 830362F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830362F8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830362FC: 4BF9B115  bl 0x82fd1410
	ctx.lr = 0x83036300;
	sub_82FD1410(ctx, base);
	// 83036300: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83036304: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83036308: 419A0014  beq cr6, 0x8303631c
	if ctx.cr[6].eq {
	pc = 0x8303631C; continue 'dispatch;
	}
	// 8303630C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83036310: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83036314: 4BF9B0FD  bl 0x82fd1410
	ctx.lr = 0x83036318;
	sub_82FD1410(ctx, base);
	// 83036318: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8303631C: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 83036320: 419A0014  beq cr6, 0x83036334
	if ctx.cr[6].eq {
	pc = 0x83036334; continue 'dispatch;
	}
	// 83036324: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83036328: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 8303632C: 4BF9B0E5  bl 0x82fd1410
	ctx.lr = 0x83036330;
	sub_82FD1410(ctx, base);
	// 83036330: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83036334: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036338: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8303633C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83036340: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 83036344: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83036348: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8303634C: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 83036350: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036354: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83036358: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8303635C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83036360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036364: 4E800421  bctrl
	ctx.lr = 0x83036368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036368: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303636C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83036370: 419A001C  beq cr6, 0x8303638c
	if ctx.cr[6].eq {
	pc = 0x8303638C; continue 'dispatch;
	}
	// 83036374: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036378: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303637C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036380: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036388: 4E800421  bctrl
	ctx.lr = 0x8303638C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303638C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83036390: 419A001C  beq cr6, 0x830363ac
	if ctx.cr[6].eq {
	pc = 0x830363AC; continue 'dispatch;
	}
	// 83036394: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036398: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303639C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830363A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830363A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830363A8: 4E800421  bctrl
	ctx.lr = 0x830363AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830363AC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830363B0: 419A001C  beq cr6, 0x830363cc
	if ctx.cr[6].eq {
	pc = 0x830363CC; continue 'dispatch;
	}
	// 830363B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830363B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830363BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830363C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830363C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830363C8: 4E800421  bctrl
	ctx.lr = 0x830363CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830363CC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830363D0: 419A001C  beq cr6, 0x830363ec
	if ctx.cr[6].eq {
	pc = 0x830363EC; continue 'dispatch;
	}
	// 830363D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830363D8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830363DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830363E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830363E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830363E8: 4E800421  bctrl
	ctx.lr = 0x830363EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830363EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830363F0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 830363F4: 48171DA4  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830363F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830363F8 size=76
    let mut pc: u32 = 0x830363F8;
    'dispatch: loop {
        match pc {
            0x830363F8 => {
    //   block [0x830363F8..0x83036444)
	// 830363F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830363FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303640C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036410: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83036414: 4BFFFD7D  bl 0x83036190
	ctx.lr = 0x83036418;
	sub_83036190(ctx, base);
	// 83036418: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303641C: 4182000C  beq 0x83036428
	if ctx.cr[0].eq {
	pc = 0x83036428; continue 'dispatch;
	}
	// 83036420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036424: 4BFA1EBD  bl 0x82fd82e0
	ctx.lr = 0x83036428;
	sub_82FD82E0(ctx, base);
	// 83036428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303642C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036438: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303643C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036448 size=12
    let mut pc: u32 = 0x83036448;
    'dispatch: loop {
        match pc {
            0x83036448 => {
    //   block [0x83036448..0x83036454)
	// 83036448: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8303644C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83036450: 4817B668  b 0x831b1ab8
	sub_831B1AB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036458 size=16
    let mut pc: u32 = 0x83036458;
    'dispatch: loop {
        match pc {
            0x83036458 => {
    //   block [0x83036458..0x83036468)
	// 83036458: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8303645C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83036460: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83036464: 4817B71C  b 0x831b1b80
	sub_831B1B80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036468 size=12
    let mut pc: u32 = 0x83036468;
    'dispatch: loop {
        match pc {
            0x83036468 => {
    //   block [0x83036468..0x83036474)
	// 83036468: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 8303646C: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 83036470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036478 size=52
    let mut pc: u32 = 0x83036478;
    'dispatch: loop {
        match pc {
            0x83036478 => {
    //   block [0x83036478..0x830364AC)
	// 83036478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036484: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83036488: 4817B7F1  bl 0x831b1c78
	ctx.lr = 0x8303648C;
	sub_831B1C78(ctx, base);
	// 8303648C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83036490: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83036494: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83036498: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8303649C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830364A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830364A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830364A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364B0 size=8
    let mut pc: u32 = 0x830364B0;
    'dispatch: loop {
        match pc {
            0x830364B0 => {
    //   block [0x830364B0..0x830364B8)
	// 830364B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830364B4: 4817B574  b 0x831b1a28
	sub_831B1A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364B8 size=8
    let mut pc: u32 = 0x830364B8;
    'dispatch: loop {
        match pc {
            0x830364B8 => {
    //   block [0x830364B8..0x830364C0)
	// 830364B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830364BC: 4817B884  b 0x831b1d40
	sub_831B1D40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364C0 size=16
    let mut pc: u32 = 0x830364C0;
    'dispatch: loop {
        match pc {
            0x830364C0 => {
    //   block [0x830364C0..0x830364D0)
	// 830364C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830364C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830364C8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830364CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364D0 size=16
    let mut pc: u32 = 0x830364D0;
    'dispatch: loop {
        match pc {
            0x830364D0 => {
    //   block [0x830364D0..0x830364E0)
	// 830364D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830364D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830364D8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830364DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364E0 size=20
    let mut pc: u32 = 0x830364E0;
    'dispatch: loop {
        match pc {
            0x830364E0 => {
    //   block [0x830364E0..0x830364F4)
	// 830364E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830364E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830364E8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 830364EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830364F0: 4818E958  b 0x831c4e48
	sub_831C4E48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364F4 size=4
    let mut pc: u32 = 0x830364F4;
    'dispatch: loop {
        match pc {
            0x830364F4 => {
    //   block [0x830364F4..0x830364F8)
	// 830364F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830364F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830364F8 size=16
    let mut pc: u32 = 0x830364F8;
    'dispatch: loop {
        match pc {
            0x830364F8 => {
    //   block [0x830364F8..0x83036508)
	// 830364F8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830364FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036500: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83036504: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036508 size=28
    let mut pc: u32 = 0x83036508;
    'dispatch: loop {
        match pc {
            0x83036508 => {
    //   block [0x83036508..0x83036524)
	// 83036508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303650C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83036510: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83036514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83036518: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8303651C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83036520: 4818E698  b 0x831c4bb8
	sub_831C4BB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036524(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036524 size=4
    let mut pc: u32 = 0x83036524;
    'dispatch: loop {
        match pc {
            0x83036524 => {
    //   block [0x83036524..0x83036528)
	// 83036524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036528 size=172
    let mut pc: u32 = 0x83036528;
    'dispatch: loop {
        match pc {
            0x83036528 => {
    //   block [0x83036528..0x830365D4)
	// 83036528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303652C: 48171C3D  bl 0x831a8168
	ctx.lr = 0x83036530;
	sub_831A8130(ctx, base);
	// 83036530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036534: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83036538: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303653C: 409A000C  bne cr6, 0x83036548
	if !ctx.cr[6].eq {
	pc = 0x83036548; continue 'dispatch;
	}
	// 83036540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036544: 48000088  b 0x830365cc
	pc = 0x830365CC; continue 'dispatch;
	// 83036548: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303654C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036550: 41820064  beq 0x830365b4
	if ctx.cr[0].eq {
	pc = 0x830365B4; continue 'dispatch;
	}
	// 83036554: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83036558: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303655C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83036560: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83036564: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303656C: 4E800421  bctrl
	ctx.lr = 0x83036570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036570: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83036574: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 83036578: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303657C: 4B28A3BD  bl 0x822c0938
	ctx.lr = 0x83036580;
	sub_822C0938(ctx, base);
	// 83036580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83036588: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303658C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83036590: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036594: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83036598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303659C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830365A0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 830365A4: 4818E615  bl 0x831c4bb8
	ctx.lr = 0x830365A8;
	sub_831C4BB8(ctx, base);
	// 830365A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830365AC: 7D7EF9AE  stbx r11, r30, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 830365B0: 48000018  b 0x830365c8
	pc = 0x830365C8; continue 'dispatch;
	// 830365B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830365B8: 4B28A381  bl 0x822c0938
	ctx.lr = 0x830365BC;
	sub_822C0938(ctx, base);
	// 830365BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830365C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830365C4: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830365C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830365CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830365D0: 48171BE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830365D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830365D8 size=204
    let mut pc: u32 = 0x830365D8;
    'dispatch: loop {
        match pc {
            0x830365D8 => {
    //   block [0x830365D8..0x830366A4)
	// 830365D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830365DC: 48171B8D  bl 0x831a8168
	ctx.lr = 0x830365E0;
	sub_831A8130(ctx, base);
	// 830365E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830365E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830365E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830365EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830365F0: 409A000C  bne cr6, 0x830365fc
	if !ctx.cr[6].eq {
	pc = 0x830365FC; continue 'dispatch;
	}
	// 830365F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830365F8: 480000A4  b 0x8303669c
	pc = 0x8303669C; continue 'dispatch;
	// 830365FC: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036600: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036604: 41820070  beq 0x83036674
	if ctx.cr[0].eq {
	pc = 0x83036674; continue 'dispatch;
	}
	// 83036608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303660C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83036610: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83036614: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303661C: 4E800421  bctrl
	ctx.lr = 0x83036620;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036624: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83036628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303662C: 3B9D0001  addi r28, r29, 1
	ctx.r[28].s64 = ctx.r[29].s64 + 1;
	// 83036630: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036634: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83036638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303663C: 4E800421  bctrl
	ctx.lr = 0x83036640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036640: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83036648: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303664C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83036650: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036654: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83036658: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303665C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036660: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83036664: 4818E555  bl 0x831c4bb8
	ctx.lr = 0x83036668;
	sub_831C4BB8(ctx, base);
	// 83036668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303666C: 7D7DF9AE  stbx r11, r29, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 83036670: 48000028  b 0x83036698
	pc = 0x83036698; continue 'dispatch;
	// 83036674: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036678: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303667C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036680: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036688: 4E800421  bctrl
	ctx.lr = 0x8303668C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303668C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83036694: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83036698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303669C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830366A0: 48171B18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830366A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830366A8 size=212
    let mut pc: u32 = 0x830366A8;
    'dispatch: loop {
        match pc {
            0x830366A8 => {
    //   block [0x830366A8..0x8303677C)
	// 830366A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830366AC: 48171ABD  bl 0x831a8168
	ctx.lr = 0x830366B0;
	sub_831A8130(ctx, base);
	// 830366B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830366B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830366B8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830366BC: 409A000C  bne cr6, 0x830366c8
	if !ctx.cr[6].eq {
	pc = 0x830366C8; continue 'dispatch;
	}
	// 830366C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830366C4: 480000B0  b 0x83036774
	pc = 0x83036774; continue 'dispatch;
	// 830366C8: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830366CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830366D0: 419A008C  beq cr6, 0x8303675c
	if ctx.cr[6].eq {
	pc = 0x8303675C; continue 'dispatch;
	}
	// 830366D4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830366D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830366DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830366E0: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830366E4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830366E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830366EC: 4E800421  bctrl
	ctx.lr = 0x830366F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830366F0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830366F4: 40820018  bne 0x8303670c
	if !ctx.cr[0].eq {
	pc = 0x8303670C; continue 'dispatch;
	}
	// 830366F8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830366FC: 4B28A23D  bl 0x822c0938
	ctx.lr = 0x83036700;
	sub_822C0938(ctx, base);
	// 83036700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83036704: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83036708: 4800006C  b 0x83036774
	pc = 0x83036774; continue 'dispatch;
	// 8303670C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83036710: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 83036714: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83036718: 57A3083C  slwi r3, r29, 1
	ctx.r[3].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303671C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83036720: 40990008  ble cr6, 0x83036728
	if !ctx.cr[6].gt {
	pc = 0x83036728; continue 'dispatch;
	}
	// 83036724: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83036728: 4B28A211  bl 0x822c0938
	ctx.lr = 0x8303672C;
	sub_822C0938(ctx, base);
	// 8303672C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036730: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83036734: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036738: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303673C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83036740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036744: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83036748: 4818E701  bl 0x831c4e48
	ctx.lr = 0x8303674C;
	sub_831C4E48(ctx, base);
	// 8303674C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83036750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83036754: 7D4BFB2E  sthx r10, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u16) };
	// 83036758: 48000018  b 0x83036770
	pc = 0x83036770; continue 'dispatch;
	// 8303675C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83036760: 4B28A1D9  bl 0x822c0938
	ctx.lr = 0x83036764;
	sub_822C0938(ctx, base);
	// 83036764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303676C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83036770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83036778: 48171A40  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036780 size=224
    let mut pc: u32 = 0x83036780;
    'dispatch: loop {
        match pc {
            0x83036780 => {
    //   block [0x83036780..0x83036860)
	// 83036780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036784: 481719E5  bl 0x831a8168
	ctx.lr = 0x83036788;
	sub_831A8130(ctx, base);
	// 83036788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303678C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83036790: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83036794: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83036798: 409A000C  bne cr6, 0x830367a4
	if !ctx.cr[6].eq {
	pc = 0x830367A4; continue 'dispatch;
	}
	// 8303679C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830367A0: 480000B8  b 0x83036858
	pc = 0x83036858; continue 'dispatch;
	// 830367A4: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830367A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830367AC: 419A0084  beq cr6, 0x83036830
	if ctx.cr[6].eq {
	pc = 0x83036830; continue 'dispatch;
	}
	// 830367B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830367B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830367B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830367BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830367C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830367C4: 4E800421  bctrl
	ctx.lr = 0x830367C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830367C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830367CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830367D0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830367D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830367D8: 4082001C  bne 0x830367f4
	if !ctx.cr[0].eq {
	pc = 0x830367F4; continue 'dispatch;
	}
	// 830367DC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830367E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830367E4: 4E800421  bctrl
	ctx.lr = 0x830367E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830367E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830367EC: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830367F0: 48000068  b 0x83036858
	pc = 0x83036858; continue 'dispatch;
	// 830367F4: 57DC083C  slwi r28, r30, 1
	ctx.r[28].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 830367F8: 389C0002  addi r4, r28, 2
	ctx.r[4].s64 = ctx.r[28].s64 + 2;
	// 830367FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036800: 4E800421  bctrl
	ctx.lr = 0x83036804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036808: 391E0001  addi r8, r30, 1
	ctx.r[8].s64 = ctx.r[30].s64 + 1;
	// 8303680C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036810: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83036814: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83036818: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303681C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83036820: 4818E629  bl 0x831c4e48
	ctx.lr = 0x83036824;
	sub_831C4E48(ctx, base);
	// 83036824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83036828: 7D7CFB2E  sthx r11, r28, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u16) };
	// 8303682C: 48000028  b 0x83036854
	pc = 0x83036854; continue 'dispatch;
	// 83036830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036834: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83036838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303683C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036844: 4E800421  bctrl
	ctx.lr = 0x83036848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303684C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83036850: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83036854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303685C: 4817195C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036860 size=116
    let mut pc: u32 = 0x83036860;
    'dispatch: loop {
        match pc {
            0x83036860 => {
    //   block [0x83036860..0x830368D4)
	// 83036860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303686C: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83036870: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83036874: 419A0044  beq cr6, 0x830368b8
	if ctx.cr[6].eq {
	pc = 0x830368B8; continue 'dispatch;
	}
	// 83036878: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8303687C: 419A003C  beq cr6, 0x830368b8
	if ctx.cr[6].eq {
	pc = 0x830368B8; continue 'dispatch;
	}
	// 83036880: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83036888: 419A0030  beq cr6, 0x830368b8
	if ctx.cr[6].eq {
	pc = 0x830368B8; continue 'dispatch;
	}
	// 8303688C: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 83036890: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83036894: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036898: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303689C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830368A0: 4818E5A9  bl 0x831c4e48
	ctx.lr = 0x830368A4;
	sub_831C4E48(ctx, base);
	// 830368A4: 39630000  addi r11, r3, 0
	ctx.r[11].s64 = ctx.r[3].s64 + 0;
	// 830368A8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830368AC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830368B0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830368B4: 48000010  b 0x830368c4
	pc = 0x830368C4; continue 'dispatch;
	// 830368B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830368BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830368C0: B1670000  sth r11, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830368C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830368C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830368CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830368D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830368D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830368D8 size=148
    let mut pc: u32 = 0x830368D8;
    'dispatch: loop {
        match pc {
            0x830368D8 => {
    //   block [0x830368D8..0x8303696C)
	// 830368D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830368DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830368E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830368E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830368E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830368EC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830368F0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830368F4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830368F8: 419A0050  beq cr6, 0x83036948
	if ctx.cr[6].eq {
	pc = 0x83036948; continue 'dispatch;
	}
	// 830368FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83036900: 419A0048  beq cr6, 0x83036948
	if ctx.cr[6].eq {
	pc = 0x83036948; continue 'dispatch;
	}
	// 83036904: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036908: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303690C: 4182003C  beq 0x83036948
	if ctx.cr[0].eq {
	pc = 0x83036948; continue 'dispatch;
	}
	// 83036910: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83036914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83036918: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303691C: 391E0001  addi r8, r30, 1
	ctx.r[8].s64 = ctx.r[30].s64 + 1;
	// 83036920: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83036924: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83036928: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303692C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036930: 4818E289  bl 0x831c4bb8
	ctx.lr = 0x83036934;
	sub_831C4BB8(ctx, base);
	// 83036934: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83036938: 4182001C  beq 0x83036954
	if ctx.cr[0].eq {
	pc = 0x83036954; continue 'dispatch;
	}
	// 8303693C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83036940: 7D7FF1AE  stbx r11, r31, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u8) };
	// 83036944: 4800000C  b 0x83036950
	pc = 0x83036950; continue 'dispatch;
	// 83036948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303694C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83036950: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83036954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303695C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036970 size=100
    let mut pc: u32 = 0x83036970;
    'dispatch: loop {
        match pc {
            0x83036970 => {
    //   block [0x83036970..0x830369D4)
	// 83036970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303697C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036984: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83036988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303698C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83036990: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83036994: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036998: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303699C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830369A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830369A4: 4E800421  bctrl
	ctx.lr = 0x830369A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830369A8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830369AC: 4182000C  beq 0x830369b8
	if ctx.cr[0].eq {
	pc = 0x830369B8; continue 'dispatch;
	}
	// 830369B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830369B4: 4BFA192D  bl 0x82fd82e0
	ctx.lr = 0x830369B8;
	sub_82FD82E0(ctx, base);
	// 830369B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830369BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830369C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830369C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830369C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830369CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830369D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830369D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830369D8 size=8
    let mut pc: u32 = 0x830369D8;
    'dispatch: loop {
        match pc {
            0x830369D8 => {
    //   block [0x830369D8..0x830369E0)
	// 830369D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830369DC: 821497B8  lwz r16, -0x6848(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830369E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830369E0 size=100
    let mut pc: u32 = 0x830369E0;
    'dispatch: loop {
        match pc {
            0x830369E0 => {
    //   block [0x830369E0..0x83036A44)
	// 830369E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830369E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830369E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830369EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830369F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830369F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830369F8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830369FC: 4BFA184D  bl 0x82fd8248
	ctx.lr = 0x83036A00;
	sub_82FD8248(ctx, base);
	// 83036A00: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83036A04: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83036A08: 41820020  beq 0x83036a28
	if ctx.cr[0].eq {
	pc = 0x83036A28; continue 'dispatch;
	}
	// 83036A0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036A10: 4BFBEFB9  bl 0x82ff59c8
	ctx.lr = 0x83036A14;
	sub_82FF59C8(ctx, base);
	// 83036A14: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036A18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036A1C: 396B978C  addi r11, r11, -0x6874
	ctx.r[11].s64 = ctx.r[11].s64 + -26740;
	// 83036A20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036A24: 48000008  b 0x83036a2c
	pc = 0x83036A2C; continue 'dispatch;
	// 83036A28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036A2C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83036A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036A38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036A44 size=40
    let mut pc: u32 = 0x83036A44;
    'dispatch: loop {
        match pc {
            0x83036A44 => {
    //   block [0x83036A44..0x83036A6C)
	// 83036A44: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83036A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036A54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83036A58: 4BFA1889  bl 0x82fd82e0
	ctx.lr = 0x83036A5C;
	sub_82FD82E0(ctx, base);
	// 83036A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036A70 size=88
    let mut pc: u32 = 0x83036A70;
    'dispatch: loop {
        match pc {
            0x83036A70 => {
    //   block [0x83036A70..0x83036AC8)
	// 83036A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036A78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036A7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036A80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036A84: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036A8C: 396B978C  addi r11, r11, -0x6874
	ctx.r[11].s64 = ctx.r[11].s64 + -26740;
	// 83036A90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83036A94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036A98: 4BFBEF31  bl 0x82ff59c8
	ctx.lr = 0x83036A9C;
	sub_82FF59C8(ctx, base);
	// 83036A9C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036AA0: 4182000C  beq 0x83036aac
	if ctx.cr[0].eq {
	pc = 0x83036AAC; continue 'dispatch;
	}
	// 83036AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036AA8: 4BFA1839  bl 0x82fd82e0
	ctx.lr = 0x83036AAC;
	sub_82FD82E0(ctx, base);
	// 83036AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036AC8 size=152
    let mut pc: u32 = 0x83036AC8;
    'dispatch: loop {
        match pc {
            0x83036AC8 => {
    //   block [0x83036AC8..0x83036B60)
	// 83036AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036ACC: 48171695  bl 0x831a8160
	ctx.lr = 0x83036AD0;
	sub_831A8130(ctx, base);
	// 83036AD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036AD8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83036ADC: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 83036AE0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83036AE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83036AE8: 4099006C  ble cr6, 0x83036b54
	if !ctx.cr[6].gt {
	pc = 0x83036B54; continue 'dispatch;
	}
	// 83036AEC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 83036AF0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036AF4: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83036AF8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036AFC: 4182003C  beq 0x83036b38
	if ctx.cr[0].eq {
	pc = 0x83036B38; continue 'dispatch;
	}
	// 83036B00: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036B04: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036B08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036B0C: 41820018  beq 0x83036b24
	if ctx.cr[0].eq {
	pc = 0x83036B24; continue 'dispatch;
	}
	// 83036B10: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036B14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036B18: 4182000C  beq 0x83036b24
	if ctx.cr[0].eq {
	pc = 0x83036B24; continue 'dispatch;
	}
	// 83036B1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83036B20: 4BFFFE51  bl 0x83036970
	ctx.lr = 0x83036B24;
	sub_83036970(ctx, base);
	// 83036B24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036B28: 4BFA17B9  bl 0x82fd82e0
	ctx.lr = 0x83036B2C;
	sub_82FD82E0(ctx, base);
	// 83036B2C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83036B30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83036B34: 409AFFCC  bne cr6, 0x83036b00
	if !ctx.cr[6].eq {
	pc = 0x83036B00; continue 'dispatch;
	}
	// 83036B38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036B3C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83036B40: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 83036B44: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83036B48: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83036B4C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83036B50: 4198FFA0  blt cr6, 0x83036af0
	if ctx.cr[6].lt {
	pc = 0x83036AF0; continue 'dispatch;
	}
	// 83036B54: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 83036B58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83036B5C: 48171654  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036B60 size=8
    let mut pc: u32 = 0x83036B60;
    'dispatch: loop {
        match pc {
            0x83036B60 => {
    //   block [0x83036B60..0x83036B68)
	// 83036B60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83036B64: 82149828  lwz r16, -0x67d8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036B68 size=124
    let mut pc: u32 = 0x83036B68;
    'dispatch: loop {
        match pc {
            0x83036B68 => {
    //   block [0x83036B68..0x83036BE4)
	// 83036B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036B78: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83036B7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036B80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83036B84: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83036B88: 4BFC0E09  bl 0x82ff7990
	ctx.lr = 0x83036B8C;
	sub_82FF7990(ctx, base);
	// 83036B8C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036B90: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83036B94: 396B97E8  addi r11, r11, -0x6818
	ctx.r[11].s64 = ctx.r[11].s64 + -26648;
	// 83036B98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036B9C: 4BFA16AD  bl 0x82fd8248
	ctx.lr = 0x83036BA0;
	sub_82FD8248(ctx, base);
	// 83036BA0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83036BA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036BA8: 41820018  beq 0x83036bc0
	if ctx.cr[0].eq {
	pc = 0x83036BC0; continue 'dispatch;
	}
	// 83036BAC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83036BB0: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83036BB4: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83036BB8: 48033A51  bl 0x8306a608
	ctx.lr = 0x83036BBC;
	sub_8306A608(ctx, base);
	// 83036BBC: 48000008  b 0x83036bc4
	pc = 0x83036BC4; continue 'dispatch;
	// 83036BC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83036BC4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83036BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036BCC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83036BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036BD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036BE4 size=40
    let mut pc: u32 = 0x83036BE4;
    'dispatch: loop {
        match pc {
            0x83036BE4 => {
    //   block [0x83036BE4..0x83036C0C)
	// 83036BE4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83036BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036BF4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83036BF8: 4BFBECC9  bl 0x82ff58c0
	ctx.lr = 0x83036BFC;
	sub_82FF58C0(ctx, base);
	// 83036BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036C0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036C0C size=40
    let mut pc: u32 = 0x83036C0C;
    'dispatch: loop {
        match pc {
            0x83036C0C => {
    //   block [0x83036C0C..0x83036C34)
	// 83036C0C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83036C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036C1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83036C20: 4BFA16C1  bl 0x82fd82e0
	ctx.lr = 0x83036C24;
	sub_82FD82E0(ctx, base);
	// 83036C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036C38 size=100
    let mut pc: u32 = 0x83036C38;
    'dispatch: loop {
        match pc {
            0x83036C38 => {
    //   block [0x83036C38..0x83036C9C)
	// 83036C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036C4C: 4BFFFE7D  bl 0x83036ac8
	ctx.lr = 0x83036C50;
	sub_83036AC8(ctx, base);
	// 83036C50: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036C54: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036C58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036C5C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036C60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036C64: 4E800421  bctrl
	ctx.lr = 0x83036C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036C68: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83036C6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036C70: 41820018  beq 0x83036c88
	if ctx.cr[0].eq {
	pc = 0x83036C88; continue 'dispatch;
	}
	// 83036C74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036C78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83036C7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83036C84: 4E800421  bctrl
	ctx.lr = 0x83036C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83036C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83036CA0 size=8
    let mut pc: u32 = 0x83036CA0;
    'dispatch: loop {
        match pc {
            0x83036CA0 => {
    //   block [0x83036CA0..0x83036CA8)
	// 83036CA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83036CA4: 82149868  lwz r16, -0x6798(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036CA8 size=80
    let mut pc: u32 = 0x83036CA8;
    'dispatch: loop {
        match pc {
            0x83036CA8 => {
    //   block [0x83036CA8..0x83036CF8)
	// 83036CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036CAC: 481714C1  bl 0x831a816c
	ctx.lr = 0x83036CB0;
	sub_831A8130(ctx, base);
	// 83036CB0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83036CB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036CB8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036CBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83036CC0: 396B97E8  addi r11, r11, -0x6818
	ctx.r[11].s64 = ctx.r[11].s64 + -26648;
	// 83036CC4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83036CC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036CCC: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83036CD0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83036CD4: 41820014  beq 0x83036ce8
	if ctx.cr[0].eq {
	pc = 0x83036CE8; continue 'dispatch;
	}
	// 83036CD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83036CDC: 4BFFFF5D  bl 0x83036c38
	ctx.lr = 0x83036CE0;
	sub_83036C38(ctx, base);
	// 83036CE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83036CE4: 4BFA15FD  bl 0x82fd82e0
	ctx.lr = 0x83036CE8;
	sub_82FD82E0(ctx, base);
	// 83036CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83036CEC: 4BFBEBD5  bl 0x82ff58c0
	ctx.lr = 0x83036CF0;
	sub_82FF58C0(ctx, base);
	// 83036CF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83036CF4: 481714C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036CF8 size=40
    let mut pc: u32 = 0x83036CF8;
    'dispatch: loop {
        match pc {
            0x83036CF8 => {
    //   block [0x83036CF8..0x83036D20)
	// 83036CF8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83036CFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036D00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036D04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036D08: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83036D0C: 4BFBEBB5  bl 0x82ff58c0
	ctx.lr = 0x83036D10;
	sub_82FF58C0(ctx, base);
	// 83036D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036D20 size=76
    let mut pc: u32 = 0x83036D20;
    'dispatch: loop {
        match pc {
            0x83036D20 => {
    //   block [0x83036D20..0x83036D6C)
	// 83036D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036D28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036D2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036D38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83036D3C: 4BFFFF6D  bl 0x83036ca8
	ctx.lr = 0x83036D40;
	sub_83036CA8(ctx, base);
	// 83036D40: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036D44: 4182000C  beq 0x83036d50
	if ctx.cr[0].eq {
	pc = 0x83036D50; continue 'dispatch;
	}
	// 83036D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036D4C: 4BFA1595  bl 0x82fd82e0
	ctx.lr = 0x83036D50;
	sub_82FD82E0(ctx, base);
	// 83036D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036D54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036D60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036D64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036D70 size=60
    let mut pc: u32 = 0x83036D70;
    'dispatch: loop {
        match pc {
            0x83036D70 => {
    //   block [0x83036D70..0x83036DAC)
	// 83036D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036D84: 4BFBEBE5  bl 0x82ff5968
	ctx.lr = 0x83036D88;
	sub_82FF5968(ctx, base);
	// 83036D88: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036D90: 396B98A8  addi r11, r11, -0x6758
	ctx.r[11].s64 = ctx.r[11].s64 + -26456;
	// 83036D94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036DB0 size=84
    let mut pc: u32 = 0x83036DB0;
    'dispatch: loop {
        match pc {
            0x83036DB0 => {
    //   block [0x83036DB0..0x83036E04)
	// 83036DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036DB4: 481713B5  bl 0x831a8168
	ctx.lr = 0x83036DB8;
	sub_831A8130(ctx, base);
	// 83036DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036DBC: 54BFF87E  srwi r31, r5, 1
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shr(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83036DC0: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83036DC4: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 83036DC8: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83036DCC: 41980008  blt cr6, 0x83036dd4
	if ctx.cr[6].lt {
	pc = 0x83036DD4; continue 'dispatch;
	}
	// 83036DD0: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83036DD4: 57FE083C  slwi r30, r31, 1
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83036DD8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83036DDC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83036DE0: 48171731  bl 0x831a8510
	ctx.lr = 0x83036DE4;
	sub_831A8510(ctx, base);
	// 83036DE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83036DE8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83036DEC: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83036DF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83036DF4: 481713ED  bl 0x831a81e0
	ctx.lr = 0x83036DF8;
	sub_831A81E0(ctx, base);
	// 83036DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83036E00: 481713B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036E08 size=64
    let mut pc: u32 = 0x83036E08;
    'dispatch: loop {
        match pc {
            0x83036E08 => {
    //   block [0x83036E08..0x83036E48)
	// 83036E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036E0C: 48171361  bl 0x831a816c
	ctx.lr = 0x83036E10;
	sub_831A8130(ctx, base);
	// 83036E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036E14: 54FEF87E  srwi r30, r7, 1
	ctx.r[30].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83036E18: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83036E1C: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 83036E20: 41980008  blt cr6, 0x83036e28
	if ctx.cr[6].lt {
	pc = 0x83036E28; continue 'dispatch;
	}
	// 83036E24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83036E28: 57DF083C  slwi r31, r30, 1
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83036E2C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83036E30: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83036E34: 481716DD  bl 0x831a8510
	ctx.lr = 0x83036E38;
	sub_831A8510(ctx, base);
	// 83036E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036E3C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83036E40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036E44: 48171378  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036E48 size=88
    let mut pc: u32 = 0x83036E48;
    'dispatch: loop {
        match pc {
            0x83036E48 => {
    //   block [0x83036E48..0x83036EA0)
	// 83036E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036E5C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036E64: 396B98A8  addi r11, r11, -0x6758
	ctx.r[11].s64 = ctx.r[11].s64 + -26456;
	// 83036E68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83036E6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036E70: 4BFBEA61  bl 0x82ff58d0
	ctx.lr = 0x83036E74;
	sub_82FF58D0(ctx, base);
	// 83036E74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83036E78: 4182000C  beq 0x83036e84
	if ctx.cr[0].eq {
	pc = 0x83036E84; continue 'dispatch;
	}
	// 83036E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036E80: 4BFA1461  bl 0x82fd82e0
	ctx.lr = 0x83036E84;
	sub_82FD82E0(ctx, base);
	// 83036E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83036E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036E94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036EA0 size=60
    let mut pc: u32 = 0x83036EA0;
    'dispatch: loop {
        match pc {
            0x83036EA0 => {
    //   block [0x83036EA0..0x83036EDC)
	// 83036EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036EB4: 4BFBEAB5  bl 0x82ff5968
	ctx.lr = 0x83036EB8;
	sub_82FF5968(ctx, base);
	// 83036EB8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036EC0: 396B98B8  addi r11, r11, -0x6748
	ctx.r[11].s64 = ctx.r[11].s64 + -26440;
	// 83036EC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83036EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83036ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036EE0 size=240
    let mut pc: u32 = 0x83036EE0;
    'dispatch: loop {
        match pc {
            0x83036EE0 => {
    //   block [0x83036EE0..0x83036FD0)
	// 83036EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83036EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036EF0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036EF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83036EF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83036EFC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83036F00: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83036F04: 40980008  bge cr6, 0x83036f0c
	if !ctx.cr[6].lt {
	pc = 0x83036F0C; continue 'dispatch;
	}
	// 83036F08: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83036F0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83036F10: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83036F14: 419A002C  beq cr6, 0x83036f40
	if ctx.cr[6].eq {
	pc = 0x83036F40; continue 'dispatch;
	}
	// 83036F18: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036F1C: 2B060080  cmplwi cr6, r6, 0x80
	ctx.cr[6].compare_u32(ctx.r[6].u32, 128 as u32, &mut ctx.xer);
	// 83036F20: 40980050  bge cr6, 0x83036f70
	if !ctx.cr[6].lt {
	pc = 0x83036F70; continue 'dispatch;
	}
	// 83036F24: 54C6063E  clrlwi r6, r6, 0x18
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 83036F28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83036F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83036F30: 7F1F3840  cmplw cr6, r31, r7
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83036F34: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83036F38: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83036F3C: 4198FFDC  blt cr6, 0x83036f18
	if ctx.cr[6].lt {
	pc = 0x83036F18; continue 'dispatch;
	}
	// 83036F40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83036F44: 93E80000  stw r31, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83036F48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83036F4C: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83036F50: 48171291  bl 0x831a81e0
	ctx.lr = 0x83036F54;
	sub_831A81E0(ctx, base);
	// 83036F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83036F58: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83036F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83036F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83036F64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83036F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83036F6C: 4E800020  blr
	return;
	// 83036F70: 2B1F0020  cmplwi cr6, r31, 0x20
	ctx.cr[6].compare_u32(ctx.r[31].u32, 32 as u32, &mut ctx.xer);
	// 83036F74: 4199FFCC  bgt cr6, 0x83036f40
	if ctx.cr[6].gt {
	pc = 0x83036F40; continue 'dispatch;
	}
	// 83036F78: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83036F7C: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83036F80: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83036F84: 80FE000C  lwz r7, 0xc(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83036F88: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83036F8C: 4BF9A8DD  bl 0x82fd1868
	ctx.lr = 0x83036F90;
	sub_82FD1868(ctx, base);
	// 83036F90: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83036F94: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83036F98: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83036F9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83036FA0: 388B98C8  addi r4, r11, -0x6738
	ctx.r[4].s64 = ctx.r[11].s64 + -26424;
	// 83036FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83036FA8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83036FAC: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 83036FB0: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 83036FB4: 38A00065  li r5, 0x65
	ctx.r[5].s64 = 101;
	// 83036FB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83036FBC: 4BFB5CED  bl 0x82fecca8
	ctx.lr = 0x83036FC0;
	sub_82FECCA8(ctx, base);
	// 83036FC0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83036FC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83036FC8: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 83036FCC: 48179C5D  bl 0x831b0c28
	ctx.lr = 0x83036FD0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83036FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83036FD0 size=216
    let mut pc: u32 = 0x83036FD0;
    'dispatch: loop {
        match pc {
            0x83036FD0 => {
    //   block [0x83036FD0..0x830370A8)
	// 83036FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83036FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83036FD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83036FDC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83036FE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83036FE4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83036FE8: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83036FEC: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83036FF0: 40980008  bge cr6, 0x83036ff8
	if !ctx.cr[6].lt {
	pc = 0x83036FF8; continue 'dispatch;
	}
	// 83036FF4: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83036FF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83036FFC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83037000: 419A0034  beq cr6, 0x83037034
	if ctx.cr[6].eq {
	pc = 0x83037034; continue 'dispatch;
	}
	// 83037004: A0CB0000  lhz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037008: 2B060080  cmplwi cr6, r6, 0x80
	ctx.cr[6].compare_u32(ctx.r[6].u32, 128 as u32, &mut ctx.xer);
	// 8303700C: 41980010  blt cr6, 0x8303701c
	if ctx.cr[6].lt {
	pc = 0x8303701C; continue 'dispatch;
	}
	// 83037010: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83037014: 419A003C  beq cr6, 0x83037050
	if ctx.cr[6].eq {
	pc = 0x83037050; continue 'dispatch;
	}
	// 83037018: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8303701C: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 83037020: 98CA0000  stb r6, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 83037024: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83037028: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8303702C: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037030: 4198FFD4  blt cr6, 0x83037004
	if ctx.cr[6].lt {
	pc = 0x83037004; continue 'dispatch;
	}
	// 83037034: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83037038: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8303703C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83037040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303704C: 4E800020  blr
	return;
	// 83037050: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83037054: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037058: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8303705C: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037060: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83037064: 4BF9A805  bl 0x82fd1868
	ctx.lr = 0x83037068;
	sub_82FD1868(ctx, base);
	// 83037068: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303706C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037070: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83037074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037078: 388B98C8  addi r4, r11, -0x6738
	ctx.r[4].s64 = ctx.r[11].s64 + -26424;
	// 8303707C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037080: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83037084: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 83037088: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 8303708C: 38A000A0  li r5, 0xa0
	ctx.r[5].s64 = 160;
	// 83037090: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83037094: 4BFB5C15  bl 0x82fecca8
	ctx.lr = 0x83037098;
	sub_82FECCA8(ctx, base);
	// 83037098: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303709C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830370A0: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 830370A4: 48179B85  bl 0x831b0c28
	ctx.lr = 0x830370A8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830370A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830370A8 size=20
    let mut pc: u32 = 0x830370A8;
    'dispatch: loop {
        match pc {
            0x830370A8 => {
    //   block [0x830370A8..0x830370BC)
	// 830370A8: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 830370AC: 7D6B2010  subfc r11, r11, r4
	ctx.xer.ca = ctx.r[4].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 830370B0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 830370B4: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830370B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830370C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830370C0 size=88
    let mut pc: u32 = 0x830370C0;
    'dispatch: loop {
        match pc {
            0x830370C0 => {
    //   block [0x830370C0..0x83037118)
	// 830370C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830370C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830370C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830370CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830370D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830370D4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830370D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830370DC: 396B98B8  addi r11, r11, -0x6748
	ctx.r[11].s64 = ctx.r[11].s64 + -26440;
	// 830370E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830370E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830370E8: 4BFBE7E9  bl 0x82ff58d0
	ctx.lr = 0x830370EC;
	sub_82FF58D0(ctx, base);
	// 830370EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830370F0: 4182000C  beq 0x830370fc
	if ctx.cr[0].eq {
	pc = 0x830370FC; continue 'dispatch;
	}
	// 830370F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830370F8: 4BFA11E9  bl 0x82fd82e0
	ctx.lr = 0x830370FC;
	sub_82FD82E0(ctx, base);
	// 830370FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303710C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83037118 size=8
    let mut pc: u32 = 0x83037118;
    'dispatch: loop {
        match pc {
            0x83037118 => {
    //   block [0x83037118..0x83037120)
	// 83037118: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303711C: 82149A48  lwz r16, -0x65b8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-26040 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037120 size=104
    let mut pc: u32 = 0x83037120;
    'dispatch: loop {
        match pc {
            0x83037120 => {
    //   block [0x83037120..0x83037188)
	// 83037120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037124: 48171039  bl 0x831a815c
	ctx.lr = 0x83037128;
	sub_831A8130(ctx, base);
	// 83037128: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303712C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037130: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83037134: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83037138: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8303713C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83037140: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83037144: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83037148: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8303714C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83037150: 4BFA1DE1  bl 0x82fd8f30
	ctx.lr = 0x83037154;
	sub_82FD8F30(ctx, base);
	// 83037154: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037158: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8303715C: 396B9A30  addi r11, r11, -0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26064;
	// 83037160: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83037164: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83037168: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303716C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83037170: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83037174: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037178: 4BFA2259  bl 0x82fd93d0
	ctx.lr = 0x8303717C;
	sub_82FD93D0(ctx, base);
	// 8303717C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83037180: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83037184: 48171028  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037188 size=40
    let mut pc: u32 = 0x83037188;
    'dispatch: loop {
        match pc {
            0x83037188 => {
    //   block [0x83037188..0x830371B0)
	// 83037188: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303718C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037190: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037198: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303719C: 4BFA1CDD  bl 0x82fd8e78
	ctx.lr = 0x830371A0;
	sub_82FD8E78(ctx, base);
	// 830371A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830371A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830371A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830371AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830371B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830371B0 size=60
    let mut pc: u32 = 0x830371B0;
    'dispatch: loop {
        match pc {
            0x830371B0 => {
    //   block [0x830371B0..0x830371EC)
	// 830371B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830371B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830371B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830371BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830371C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830371C4: 4BFA1DE5  bl 0x82fd8fa8
	ctx.lr = 0x830371C8;
	sub_82FD8FA8(ctx, base);
	// 830371C8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830371CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830371D0: 396B9A30  addi r11, r11, -0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26064;
	// 830371D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830371D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830371DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830371E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830371E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830371E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830371F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830371F0 size=16
    let mut pc: u32 = 0x830371F0;
    'dispatch: loop {
        match pc {
            0x830371F0 => {
    //   block [0x830371F0..0x83037200)
	// 830371F0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830371F4: 396B9A30  addi r11, r11, -0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26064;
	// 830371F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830371FC: 4BFA1C7C  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83037200 size=8
    let mut pc: u32 = 0x83037200;
    'dispatch: loop {
        match pc {
            0x83037200 => {
    //   block [0x83037200..0x83037208)
	// 83037200: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83037204: 82149A80  lwz r16, -0x6580(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-25984 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037208 size=92
    let mut pc: u32 = 0x83037208;
    'dispatch: loop {
        match pc {
            0x83037208 => {
    //   block [0x83037208..0x83037264)
	// 83037208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303720C: 48170F61  bl 0x831a816c
	ctx.lr = 0x83037210;
	sub_831A8130(ctx, base);
	// 83037210: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83037214: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037218: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303721C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83037220: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83037224: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83037228: 4BFA1071  bl 0x82fd8298
	ctx.lr = 0x8303722C;
	sub_82FD8298(ctx, base);
	// 8303722C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83037230: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83037234: 41820024  beq 0x83037258
	if ctx.cr[0].eq {
	pc = 0x83037258; continue 'dispatch;
	}
	// 83037238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303723C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83037240: 4BFA1D69  bl 0x82fd8fa8
	ctx.lr = 0x83037244;
	sub_82FD8FA8(ctx, base);
	// 83037244: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303724C: 396B9A30  addi r11, r11, -0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26064;
	// 83037250: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037254: 48000008  b 0x8303725c
	pc = 0x8303725C; continue 'dispatch;
	// 83037258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303725C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83037260: 48170F5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037264 size=48
    let mut pc: u32 = 0x83037264;
    'dispatch: loop {
        match pc {
            0x83037264 => {
    //   block [0x83037264..0x83037294)
	// 83037264: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83037268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037274: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83037278: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303727C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83037280: 4BFA1061  bl 0x82fd82e0
	ctx.lr = 0x83037284;
	sub_82FD82E0(ctx, base);
	// 83037284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83037288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303728C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83037298 size=12
    let mut pc: u32 = 0x83037298;
    'dispatch: loop {
        match pc {
            0x83037298 => {
    //   block [0x83037298..0x830372A4)
	// 83037298: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303729C: 386B8464  addi r3, r11, -0x7b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -31644;
	// 830372A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830372A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830372A8 size=88
    let mut pc: u32 = 0x830372A8;
    'dispatch: loop {
        match pc {
            0x830372A8 => {
    //   block [0x830372A8..0x83037300)
	// 830372A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830372AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830372B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830372B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830372B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830372BC: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830372C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830372C4: 396B9A30  addi r11, r11, -0x65d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26064;
	// 830372C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830372CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830372D0: 4BFA1BA9  bl 0x82fd8e78
	ctx.lr = 0x830372D4;
	sub_82FD8E78(ctx, base);
	// 830372D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830372D8: 4182000C  beq 0x830372e4
	if ctx.cr[0].eq {
	pc = 0x830372E4; continue 'dispatch;
	}
	// 830372DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830372E0: 4BFA1001  bl 0x82fd82e0
	ctx.lr = 0x830372E4;
	sub_82FD82E0(ctx, base);
	// 830372E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830372E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830372EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830372F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830372F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830372F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830372FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037300 size=160
    let mut pc: u32 = 0x83037300;
    'dispatch: loop {
        match pc {
            0x83037300 => {
    //   block [0x83037300..0x830373A0)
	// 83037300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303730C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037310: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037314: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83037318: 57EB0632  rlwinm r11, r31, 0, 0x18, 0x19
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 8303731C: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 83037320: 419A0068  beq cr6, 0x83037388
	if ctx.cr[6].eq {
	pc = 0x83037388; continue 'dispatch;
	}
	// 83037324: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037328: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303732C: 3BC60031  addi r30, r6, 0x31
	ctx.r[30].s64 = ctx.r[6].s64 + 49;
	// 83037330: 9BE10064  stb r31, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u8 ) };
	// 83037334: 388B9AB0  addi r4, r11, -0x6550
	ctx.r[4].s64 = ctx.r[11].s64 + -25936;
	// 83037338: 39650031  addi r11, r5, 0x31
	ctx.r[11].s64 = ctx.r[5].s64 + 49;
	// 8303733C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83037344: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 83037348: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 8303734C: 9BC10062  stb r30, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[30].u8 ) };
	// 83037350: 38E10062  addi r7, r1, 0x62
	ctx.r[7].s64 = ctx.r[1].s64 + 98;
	// 83037354: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 83037358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303735C: 38C0006E  li r6, 0x6e
	ctx.r[6].s64 = 110;
	// 83037360: 38A0006A  li r5, 0x6a
	ctx.r[5].s64 = 106;
	// 83037364: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83037368: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8303736C: 99610063  stb r11, 0x63(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(99 as u32), ctx.r[11].u8 ) };
	// 83037370: 99610065  stb r11, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[11].u8 ) };
	// 83037374: 4BFFFDAD  bl 0x83037120
	ctx.lr = 0x83037378;
	sub_83037120(ctx, base);
	// 83037378: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303737C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83037380: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 83037384: 481798A5  bl 0x831b0c28
	ctx.lr = 0x83037388;
	sub_831B0C28(ctx, base);
	// 83037388: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8303738C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830373A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830373A0 size=60
    let mut pc: u32 = 0x830373A0;
    'dispatch: loop {
        match pc {
            0x830373A0 => {
    //   block [0x830373A0..0x830373DC)
	// 830373A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830373A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830373A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830373AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830373B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830373B4: 4BFBE5B5  bl 0x82ff5968
	ctx.lr = 0x830373B8;
	sub_82FF5968(ctx, base);
	// 830373B8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830373BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830373C0: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 830373C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830373C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830373CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830373D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830373D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830373D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830373E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830373E0 size=1228
    let mut pc: u32 = 0x830373E0;
    'dispatch: loop {
        match pc {
            0x830373E0 => {
    //   block [0x830373E0..0x830378AC)
	// 830373E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830373E4: 48170D5D  bl 0x831a8140
	ctx.lr = 0x830373E8;
	sub_831A8130(ctx, base);
	// 830373E8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830373EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830373F0: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 830373F4: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 830373F8: 7D124378  mr r18, r8
	ctx.r[18].u64 = ctx.r[8].u64;
	// 830373FC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83037400: 419A04A0  beq cr6, 0x830378a0
	if ctx.cr[6].eq {
	pc = 0x830378A0; continue 'dispatch;
	}
	// 83037404: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83037408: 419A0498  beq cr6, 0x830378a0
	if ctx.cr[6].eq {
	pc = 0x830378A0; continue 'dispatch;
	}
	// 8303740C: 7EB72A14  add r21, r23, r5
	ctx.r[21].u64 = ctx.r[23].u64 + ctx.r[5].u64;
	// 83037410: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037414: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 83037418: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 8303741C: 7E6BB214  add r19, r11, r22
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 83037420: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 83037424: 7F17A840  cmplw cr6, r23, r21
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[21].u32, &mut ctx.xer);
	// 83037428: 40980264  bge cr6, 0x8303768c
	if !ctx.cr[6].lt {
	pc = 0x8303768C; continue 'dispatch;
	}
	// 8303742C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037430: 3B160002  addi r24, r22, 2
	ctx.r[24].s64 = ctx.r[22].s64 + 2;
	// 83037434: 3A8B9900  addi r20, r11, -0x6700
	ctx.r[20].s64 = ctx.r[11].s64 + -26368;
	// 83037438: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303743C: 7F1B9840  cmplw cr6, r27, r19
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[19].u32, &mut ctx.xer);
	// 83037440: 4098024C  bge cr6, 0x8303768c
	if !ctx.cr[6].lt {
	pc = 0x8303768C; continue 'dispatch;
	}
	// 83037444: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037448: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8303744C: 2B0B007F  cmplwi cr6, r11, 0x7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 127 as u32, &mut ctx.xer);
	// 83037450: 41990028  bgt cr6, 0x83037478
	if ctx.cr[6].gt {
	pc = 0x83037478; continue 'dispatch;
	}
	// 83037454: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83037458: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8303745C: 3B180002  addi r24, r24, 2
	ctx.r[24].s64 = ctx.r[24].s64 + 2;
	// 83037460: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83037464: B17B0000  sth r11, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83037468: 3B7B0002  addi r27, r27, 2
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	// 8303746C: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83037470: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 83037474: 48000210  b 0x83037684
	pc = 0x83037684; continue 'dispatch;
	// 83037478: 7F4BA0AE  lbzx r26, r11, r20
	ctx.r[26].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 8303747C: 7D5AFA14  add r10, r26, r31
	ctx.r[10].u64 = ctx.r[26].u64 + ctx.r[31].u64;
	// 83037480: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 83037484: 40980208  bge cr6, 0x8303768c
	if !ctx.cr[6].lt {
	pc = 0x8303768C; continue 'dispatch;
	}
	// 83037488: 39540108  addi r10, r20, 0x108
	ctx.r[10].s64 = ctx.r[20].s64 + 264;
	// 8303748C: 39340100  addi r9, r20, 0x100
	ctx.r[9].s64 = ctx.r[20].s64 + 256;
	// 83037490: 7D5A50AE  lbzx r10, r26, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83037494: 7D3A48AE  lbzx r9, r26, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83037498: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8303749C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830374A0: 409A0200  bne cr6, 0x830376a0
	if !ctx.cr[6].eq {
	pc = 0x830376A0; continue 'dispatch;
	}
	// 830374A4: 2B1A0001  cmplwi cr6, r26, 1
	ctx.cr[6].compare_u32(ctx.r[26].u32, 1 as u32, &mut ctx.xer);
	// 830374A8: 419A011C  beq cr6, 0x830375c4
	if ctx.cr[6].eq {
	pc = 0x830375C4; continue 'dispatch;
	}
	// 830374AC: 2B1A0002  cmplwi cr6, r26, 2
	ctx.cr[6].compare_u32(ctx.r[26].u32, 2 as u32, &mut ctx.xer);
	// 830374B0: 419A0098  beq cr6, 0x83037548
	if ctx.cr[6].eq {
	pc = 0x83037548; continue 'dispatch;
	}
	// 830374B4: 2B1A0003  cmplwi cr6, r26, 3
	ctx.cr[6].compare_u32(ctx.r[26].u32, 3 as u32, &mut ctx.xer);
	// 830374B8: 409A024C  bne cr6, 0x83037704
	if !ctx.cr[6].eq {
	pc = 0x83037704; continue 'dispatch;
	}
	// 830374BC: 2B0B00F0  cmplwi cr6, r11, 0xf0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 240 as u32, &mut ctx.xer);
	// 830374C0: 409A0010  bne cr6, 0x830374d0
	if !ctx.cr[6].eq {
	pc = 0x830374D0; continue 'dispatch;
	}
	// 830374C4: 895F0001  lbz r10, 1(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 830374C8: 2B0A0090  cmplwi cr6, r10, 0x90
	ctx.cr[6].compare_u32(ctx.r[10].u32, 144 as u32, &mut ctx.xer);
	// 830374CC: 41980290  blt cr6, 0x8303775c
	if ctx.cr[6].lt {
	pc = 0x8303775C; continue 'dispatch;
	}
	// 830374D0: 2B0B00F4  cmplwi cr6, r11, 0xf4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 244 as u32, &mut ctx.xer);
	// 830374D4: 409A0010  bne cr6, 0x830374e4
	if !ctx.cr[6].eq {
	pc = 0x830374E4; continue 'dispatch;
	}
	// 830374D8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 830374DC: 2B0B008F  cmplwi cr6, r11, 0x8f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 143 as u32, &mut ctx.xer);
	// 830374E0: 4199027C  bgt cr6, 0x8303775c
	if ctx.cr[6].gt {
	pc = 0x8303775C; continue 'dispatch;
	}
	// 830374E4: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 830374E8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830374EC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 830374F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830374F4: 889E0000  lbz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830374F8: 4BFFFE09  bl 0x83037300
	ctx.lr = 0x830374FC;
	sub_83037300(ctx, base);
	// 830374FC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83037500: 889F0002  lbz r4, 2(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83037504: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83037508: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303750C: 4BFFFDF5  bl 0x83037300
	ctx.lr = 0x83037510;
	sub_83037300(ctx, base);
	// 83037510: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83037514: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83037518: 889F0003  lbz r4, 3(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8303751C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83037520: 4BFFFDE1  bl 0x83037300
	ctx.lr = 0x83037524;
	sub_83037300(ctx, base);
	// 83037524: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037528: 893E0000  lbz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303752C: 556A303E  rotlwi r10, r11, 6
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 83037530: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 83037534: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83037538: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8303753C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037540: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83037544: 48000068  b 0x830375ac
	pc = 0x830375AC; continue 'dispatch;
	// 83037548: 2B0B00E0  cmplwi cr6, r11, 0xe0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 224 as u32, &mut ctx.xer);
	// 8303754C: 409A0010  bne cr6, 0x8303755c
	if !ctx.cr[6].eq {
	pc = 0x8303755C; continue 'dispatch;
	}
	// 83037550: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83037554: 2B0B00A0  cmplwi cr6, r11, 0xa0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 160 as u32, &mut ctx.xer);
	// 83037558: 4198025C  blt cr6, 0x830377b4
	if ctx.cr[6].lt {
	pc = 0x830377B4; continue 'dispatch;
	}
	// 8303755C: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 83037560: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83037564: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83037568: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303756C: 889E0000  lbz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037570: 4BFFFD91  bl 0x83037300
	ctx.lr = 0x83037574;
	sub_83037300(ctx, base);
	// 83037574: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83037578: 889F0002  lbz r4, 2(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8303757C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83037580: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83037584: 4BFFFD7D  bl 0x83037300
	ctx.lr = 0x83037588;
	sub_83037300(ctx, base);
	// 83037588: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303758C: 2B0B00ED  cmplwi cr6, r11, 0xed
	ctx.cr[6].compare_u32(ctx.r[11].u32, 237 as u32, &mut ctx.xer);
	// 83037590: 409A0010  bne cr6, 0x830375a0
	if !ctx.cr[6].eq {
	pc = 0x830375A0; continue 'dispatch;
	}
	// 83037594: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037598: 2B0A00A0  cmplwi cr6, r10, 0xa0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 160 as u32, &mut ctx.xer);
	// 8303759C: 40980270  bge cr6, 0x8303780c
	if !ctx.cr[6].lt {
	pc = 0x8303780C; continue 'dispatch;
	}
	// 830375A0: 556A3032  slwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830375A4: 893E0000  lbz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830375A8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830375AC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830375B0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830375B4: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 830375B8: 554B3032  slwi r11, r10, 6
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830375BC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830375C0: 48000030  b 0x830375f0
	pc = 0x830375F0; continue 'dispatch;
	// 830375C4: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 830375C8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830375CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830375D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830375D4: 889E0000  lbz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830375D8: 4BFFFD29  bl 0x83037300
	ctx.lr = 0x830375DC;
	sub_83037300(ctx, base);
	// 830375DC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830375E0: 3BFE0001  addi r31, r30, 1
	ctx.r[31].s64 = ctx.r[30].s64 + 1;
	// 830375E4: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830375E8: 556B303E  rotlwi r11, r11, 6
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 830375EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830375F0: 39540110  addi r10, r20, 0x110
	ctx.r[10].s64 = ctx.r[20].s64 + 272;
	// 830375F4: 5749103A  slwi r9, r26, 2
	ctx.r[9].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830375F8: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830375FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83037600: 556A001F  rlwinm. r10, r11, 0, 0, 0xf
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83037604: 40820020  bne 0x83037624
	if !ctx.cr[0].eq {
	pc = 0x83037624; continue 'dispatch;
	}
	// 83037608: 395A0001  addi r10, r26, 1
	ctx.r[10].s64 = ctx.r[26].s64 + 1;
	// 8303760C: 3B180002  addi r24, r24, 2
	ctx.r[24].s64 = ctx.r[24].s64 + 2;
	// 83037610: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83037614: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 83037618: B17B0000  sth r11, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8303761C: 3B7B0002  addi r27, r27, 2
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	// 83037620: 48000064  b 0x83037684
	pc = 0x83037684; continue 'dispatch;
	// 83037624: 3D400010  lis r10, 0x10
	ctx.r[10].s64 = 1048576;
	// 83037628: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8303762C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83037630: 41990234  bgt cr6, 0x83037864
	if ctx.cr[6].gt {
	pc = 0x83037864; continue 'dispatch;
	}
	// 83037634: 7F189840  cmplw cr6, r24, r19
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[19].u32, &mut ctx.xer);
	// 83037638: 40980054  bge cr6, 0x8303768c
	if !ctx.cr[6].lt {
	pc = 0x8303768C; continue 'dispatch;
	}
	// 8303763C: 3D6BFFFF  addis r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -65536;
	// 83037640: 395A0001  addi r10, r26, 1
	ctx.r[10].s64 = ctx.r[26].s64 + 1;
	// 83037644: 5568B2BE  srwi r8, r11, 0xa
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83037648: 556B05BE  clrlwi r11, r11, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 8303764C: 3D080001  addis r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 65536;
	// 83037650: 3CEB0001  addis r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 65536;
	// 83037654: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83037658: 3908D800  addi r8, r8, -0x2800
	ctx.r[8].s64 = ctx.r[8].s64 + -10240;
	// 8303765C: 39590001  addi r10, r25, 1
	ctx.r[10].s64 = ctx.r[25].s64 + 1;
	// 83037660: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 83037664: 38E7DC00  addi r7, r7, -0x2400
	ctx.r[7].s64 = ctx.r[7].s64 + -9216;
	// 83037668: 39380002  addi r9, r24, 2
	ctx.r[9].s64 = ctx.r[24].s64 + 2;
	// 8303766C: B11B0000  sth r8, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83037670: 3B2A0001  addi r25, r10, 1
	ctx.r[25].s64 = ctx.r[10].s64 + 1;
	// 83037674: 9B8A0000  stb r28, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 83037678: 3B090002  addi r24, r9, 2
	ctx.r[24].s64 = ctx.r[9].s64 + 2;
	// 8303767C: 3B6B0002  addi r27, r11, 2
	ctx.r[27].s64 = ctx.r[11].s64 + 2;
	// 83037680: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83037684: 7F1FA840  cmplw cr6, r31, r21
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[21].u32, &mut ctx.xer);
	// 83037688: 4198FDB4  blt cr6, 0x8303743c
	if ctx.cr[6].lt {
	pc = 0x8303743C; continue 'dispatch;
	}
	// 8303768C: 7D77F850  subf r11, r23, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[23].s64;
	// 83037690: 7D56D850  subf r10, r22, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[22].s64;
	// 83037694: 7D430E70  srawi r3, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83037698: 91720000  stw r11, 0(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303769C: 48000208  b 0x830378a4
	pc = 0x830378A4; continue 'dispatch;
	// 830376A0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830376A4: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830376A8: 80DD000C  lwz r6, 0xc(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830376AC: 3BE00031  li r31, 0x31
	ctx.r[31].s64 = 49;
	// 830376B0: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 830376B4: 9B810061  stb r28, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[28].u8 ) };
	// 830376B8: 397A0031  addi r11, r26, 0x31
	ctx.r[11].s64 = ctx.r[26].s64 + 49;
	// 830376BC: 9B810063  stb r28, 0x63(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(99 as u32), ctx.r[28].u8 ) };
	// 830376C0: 39210062  addi r9, r1, 0x62
	ctx.r[9].s64 = ctx.r[1].s64 + 98;
	// 830376C4: 9B810065  stb r28, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[28].u8 ) };
	// 830376C8: 99410064  stb r10, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u8 ) };
	// 830376CC: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 830376D0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830376D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830376D8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 830376DC: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 830376E0: 38C0006E  li r6, 0x6e
	ctx.r[6].s64 = 110;
	// 830376E4: 99610062  stb r11, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[11].u8 ) };
	// 830376E8: 38A000B4  li r5, 0xb4
	ctx.r[5].s64 = 180;
	// 830376EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830376F0: 4BFFFA31  bl 0x83037120
	ctx.lr = 0x830376F4;
	sub_83037120(ctx, base);
	// 830376F4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830376F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830376FC: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 83037700: 48179529  bl 0x831b0c28
	ctx.lr = 0x83037704;
	sub_831B0C28(ctx, base);
	// 83037704: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037708: 80DD000C  lwz r6, 0xc(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303770C: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037710: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037714: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 83037718: 9B810065  stb r28, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[28].u8 ) };
	// 8303771C: 397A0031  addi r11, r26, 0x31
	ctx.r[11].s64 = ctx.r[26].s64 + 49;
	// 83037720: 9B810063  stb r28, 0x63(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(99 as u32), ctx.r[28].u8 ) };
	// 83037724: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 83037728: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8303772C: 38E10062  addi r7, r1, 0x62
	ctx.r[7].s64 = ctx.r[1].s64 + 98;
	// 83037730: 99410062  stb r10, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[10].u8 ) };
	// 83037734: 38C00073  li r6, 0x73
	ctx.r[6].s64 = 115;
	// 83037738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303773C: 38A0015F  li r5, 0x15f
	ctx.r[5].s64 = 351;
	// 83037740: 99610064  stb r11, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u8 ) };
	// 83037744: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83037748: 4BFFF9D9  bl 0x83037120
	ctx.lr = 0x8303774C;
	sub_83037120(ctx, base);
	// 8303774C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037750: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83037754: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 83037758: 481794D1  bl 0x831b0c28
	ctx.lr = 0x8303775C;
	sub_831B0C28(ctx, base);
	// 8303775C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037760: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037764: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037768: 9B810061  stb r28, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[28].u8 ) };
	// 8303776C: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 83037770: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037778: 9B810067  stb r28, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[28].u8 ) };
	// 8303777C: 39010066  addi r8, r1, 0x66
	ctx.r[8].s64 = ctx.r[1].s64 + 102;
	// 83037780: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83037784: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 83037788: 38C00072  li r6, 0x72
	ctx.r[6].s64 = 114;
	// 8303778C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 83037790: 38A0013E  li r5, 0x13e
	ctx.r[5].s64 = 318;
	// 83037794: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83037798: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8303779C: 99610066  stb r11, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u8 ) };
	// 830377A0: 4BFFF981  bl 0x83037120
	ctx.lr = 0x830377A4;
	sub_83037120(ctx, base);
	// 830377A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830377A8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 830377AC: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 830377B0: 48179479  bl 0x831b0c28
	ctx.lr = 0x830377B4;
	sub_831B0C28(ctx, base);
	// 830377B4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830377B8: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830377BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830377C0: 9B810067  stb r28, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[28].u8 ) };
	// 830377C4: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 830377C8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830377CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830377D0: 9B810061  stb r28, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[28].u8 ) };
	// 830377D4: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 830377D8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830377DC: 38E10066  addi r7, r1, 0x66
	ctx.r[7].s64 = ctx.r[1].s64 + 102;
	// 830377E0: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 830377E4: 99610066  stb r11, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u8 ) };
	// 830377E8: 38A000FE  li r5, 0xfe
	ctx.r[5].s64 = 254;
	// 830377EC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 830377F0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 830377F4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 830377F8: 4BFFF929  bl 0x83037120
	ctx.lr = 0x830377FC;
	sub_83037120(ctx, base);
	// 830377FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037800: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 83037804: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 83037808: 48179421  bl 0x831b0c28
	ctx.lr = 0x8303780C;
	sub_831B0C28(ctx, base);
	// 8303780C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037810: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037818: 9B810067  stb r28, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[28].u8 ) };
	// 8303781C: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 83037820: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037828: 9B810061  stb r28, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[28].u8 ) };
	// 8303782C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 83037830: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83037834: 38E10066  addi r7, r1, 0x66
	ctx.r[7].s64 = ctx.r[1].s64 + 102;
	// 83037838: 38C00071  li r6, 0x71
	ctx.r[6].s64 = 113;
	// 8303783C: 99610066  stb r11, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u8 ) };
	// 83037840: 38A00124  li r5, 0x124
	ctx.r[5].s64 = 292;
	// 83037844: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83037848: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8303784C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 83037850: 4BFFF8D1  bl 0x83037120
	ctx.lr = 0x83037854;
	sub_83037120(ctx, base);
	// 83037854: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037858: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8303785C: 388BC954  addi r4, r11, -0x36ac
	ctx.r[4].s64 = ctx.r[11].s64 + -13996;
	// 83037860: 481793C9  bl 0x831b0c28
	ctx.lr = 0x83037864;
	sub_831B0C28(ctx, base);
	// 83037864: 7D76D850  subf r11, r22, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[22].s64;
	// 83037868: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8303786C: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 83037870: 4199FE1C  bgt cr6, 0x8303768c
	if ctx.cr[6].gt {
	pc = 0x8303768C; continue 'dispatch;
	}
	// 83037874: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037878: 80FD000C  lwz r7, 0xc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303787C: 38C0005F  li r6, 0x5f
	ctx.r[6].s64 = 95;
	// 83037880: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 83037884: 38A00181  li r5, 0x181
	ctx.r[5].s64 = 385;
	// 83037888: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 8303788C: 4BFBA7C5  bl 0x82ff2050
	ctx.lr = 0x83037890;
	sub_82FF2050(ctx, base);
	// 83037890: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037894: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 83037898: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 8303789C: 4817938D  bl 0x831b0c28
	ctx.lr = 0x830378A0;
	sub_831B0C28(ctx, base);
	// 830378A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830378A4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 830378A8: 481708E8  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830378B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830378B0 size=544
    let mut pc: u32 = 0x830378B0;
    'dispatch: loop {
        match pc {
            0x830378B0 => {
    //   block [0x830378B0..0x83037AD0)
	// 830378B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830378B4: 481708AD  bl 0x831a8160
	ctx.lr = 0x830378B8;
	sub_831A8130(ctx, base);
	// 830378B8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830378BC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830378C0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830378C4: 419A0200  beq cr6, 0x83037ac4
	if ctx.cr[6].eq {
	pc = 0x83037AC4; continue 'dispatch;
	}
	// 830378C8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 830378CC: 419A01F8  beq cr6, 0x83037ac4
	if ctx.cr[6].eq {
	pc = 0x83037AC4; continue 'dispatch;
	}
	// 830378D0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830378D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830378D8: 7FAB2214  add r29, r11, r4
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 830378DC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830378E0: 7F863A14  add r28, r6, r7
	ctx.r[28].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 830378E4: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830378E8: 40980174  bge cr6, 0x83037a5c
	if !ctx.cr[6].lt {
	pc = 0x83037A5C; continue 'dispatch;
	}
	// 830378EC: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830378F0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 830378F4: 3B6B9A28  addi r27, r11, -0x65d8
	ctx.r[27].s64 = ctx.r[11].s64 + -26072;
	// 830378F8: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830378FC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83037900: 3D63FFFF  addis r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -65536;
	// 83037904: 396B2800  addi r11, r11, 0x2800
	ctx.r[11].s64 = ctx.r[11].s64 + 10240;
	// 83037908: 2B0B03FF  cmplwi cr6, r11, 0x3ff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1023 as u32, &mut ctx.xer);
	// 8303790C: 41990028  bgt cr6, 0x83037934
	if ctx.cr[6].gt {
	pc = 0x83037934; continue 'dispatch;
	}
	// 83037910: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 83037914: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83037918: 40980144  bge cr6, 0x83037a5c
	if !ctx.cr[6].lt {
	pc = 0x83037A5C; continue 'dispatch;
	}
	// 8303791C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037920: 3D63FFFF  addis r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -65536;
	// 83037924: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 83037928: 396B2809  addi r11, r11, 0x2809
	ctx.r[11].s64 = ctx.r[11].s64 + 10249;
	// 8303792C: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037930: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83037934: 2B030080  cmplwi cr6, r3, 0x80
	ctx.cr[6].compare_u32(ctx.r[3].u32, 128 as u32, &mut ctx.xer);
	// 83037938: 4098000C  bge cr6, 0x83037944
	if !ctx.cr[6].lt {
	pc = 0x83037944; continue 'dispatch;
	}
	// 8303793C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83037940: 48000038  b 0x83037978
	pc = 0x83037978; continue 'dispatch;
	// 83037944: 2B030800  cmplwi cr6, r3, 0x800
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2048 as u32, &mut ctx.xer);
	// 83037948: 4098000C  bge cr6, 0x83037954
	if !ctx.cr[6].lt {
	pc = 0x83037954; continue 'dispatch;
	}
	// 8303794C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83037950: 48000028  b 0x83037978
	pc = 0x83037978; continue 'dispatch;
	// 83037954: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 83037958: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303795C: 4098000C  bge cr6, 0x83037968
	if !ctx.cr[6].lt {
	pc = 0x83037968; continue 'dispatch;
	}
	// 83037960: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83037964: 48000014  b 0x83037978
	pc = 0x83037978; continue 'dispatch;
	// 83037968: 3D600011  lis r11, 0x11
	ctx.r[11].s64 = 1114112;
	// 8303796C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83037970: 409800C8  bge cr6, 0x83037a38
	if !ctx.cr[6].lt {
	pc = 0x83037A38; continue 'dispatch;
	}
	// 83037974: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83037978: 7D6A2A14  add r11, r10, r5
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 8303797C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83037980: 419900DC  bgt cr6, 0x83037a5c
	if ctx.cr[6].gt {
	pc = 0x83037A5C; continue 'dispatch;
	}
	// 83037984: 54E7083C  slwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83037988: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8303798C: 7FE7FA14  add r31, r7, r31
	ctx.r[31].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 83037990: 419A0090  beq cr6, 0x83037a20
	if ctx.cr[6].eq {
	pc = 0x83037A20; continue 'dispatch;
	}
	// 83037994: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 83037998: 419A0074  beq cr6, 0x83037a0c
	if ctx.cr[6].eq {
	pc = 0x83037A0C; continue 'dispatch;
	}
	// 8303799C: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 830379A0: 419A0058  beq cr6, 0x830379f8
	if ctx.cr[6].eq {
	pc = 0x830379F8; continue 'dispatch;
	}
	// 830379A4: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 830379A8: 419A003C  beq cr6, 0x830379e4
	if ctx.cr[6].eq {
	pc = 0x830379E4; continue 'dispatch;
	}
	// 830379AC: 2B0A0005  cmplwi cr6, r10, 5
	ctx.cr[6].compare_u32(ctx.r[10].u32, 5 as u32, &mut ctx.xer);
	// 830379B0: 419A0020  beq cr6, 0x830379d0
	if ctx.cr[6].eq {
	pc = 0x830379D0; continue 'dispatch;
	}
	// 830379B4: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 830379B8: 409A0078  bne cr6, 0x83037a30
	if !ctx.cr[6].eq {
	pc = 0x83037A30; continue 'dispatch;
	}
	// 830379BC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830379C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830379C4: 53C73832  rlwimi r7, r30, 7, 0, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[7].u64 & 0xFFFFFFFF0000003F);
	// 830379C8: 5463D1BE  srwi r3, r3, 6
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830379CC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 830379D0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830379D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830379D8: 53C73832  rlwimi r7, r30, 7, 0, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[7].u64 & 0xFFFFFFFF0000003F);
	// 830379DC: 5463D1BE  srwi r3, r3, 6
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830379E0: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 830379E4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830379E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830379EC: 53C73832  rlwimi r7, r30, 7, 0, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[7].u64 & 0xFFFFFFFF0000003F);
	// 830379F0: 5463D1BE  srwi r3, r3, 6
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830379F4: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 830379F8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830379FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83037A00: 53C73832  rlwimi r7, r30, 7, 0, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[7].u64 & 0xFFFFFFFF0000003F);
	// 83037A04: 5463D1BE  srwi r3, r3, 6
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83037A08: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 83037A0C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83037A10: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83037A14: 53C73832  rlwimi r7, r30, 7, 0, 0x19
	ctx.r[7].u64 = (((ctx.r[30].u32).rotate_left(7) as u64) & 0x00000000FFFFFFC0) | (ctx.r[7].u64 & 0xFFFFFFFF0000003F);
	// 83037A18: 5463D1BE  srwi r3, r3, 6
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(6);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83037A1C: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 83037A20: 7CEAD8AE  lbzx r7, r10, r27
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83037A24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83037A28: 7CE71B78  or r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[3].u64;
	// 83037A2C: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 83037A30: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83037A34: 48000020  b 0x83037a54
	pc = 0x83037A54; continue 'dispatch;
	// 83037A38: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83037A3C: 419A0034  beq cr6, 0x83037a70
	if ctx.cr[6].eq {
	pc = 0x83037A70; continue 'dispatch;
	}
	// 83037A40: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 83037A44: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037A48: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83037A4C: 99450000  stb r10, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83037A50: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 83037A54: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83037A58: 4198FEA0  blt cr6, 0x830378f8
	if ctx.cr[6].lt {
	pc = 0x830378F8; continue 'dispatch;
	}
	// 83037A5C: 7D64F850  subf r11, r4, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[4].s64;
	// 83037A60: 7C662850  subf r3, r6, r5
	ctx.r[3].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 83037A64: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83037A68: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037A6C: 4800005C  b 0x83037ac8
	pc = 0x83037AC8; continue 'dispatch;
	// 83037A70: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83037A74: 80FA000C  lwz r7, 0xc(r26)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037A78: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83037A7C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83037A80: 4BF99DE9  bl 0x82fd1868
	ctx.lr = 0x83037A84;
	sub_82FD1868(ctx, base);
	// 83037A84: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037A88: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037A8C: 811A0008  lwz r8, 8(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 83037A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037A94: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 83037A98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037A9C: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83037AA0: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 83037AA4: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 83037AA8: 38A001ED  li r5, 0x1ed
	ctx.r[5].s64 = 493;
	// 83037AAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83037AB0: 4BFB51F9  bl 0x82fecca8
	ctx.lr = 0x83037AB4;
	sub_82FECCA8(ctx, base);
	// 83037AB4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037AB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83037ABC: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 83037AC0: 48179169  bl 0x831b0c28
	ctx.lr = 0x83037AC4;
	sub_831B0C28(ctx, base);
	// 83037AC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83037AC8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83037ACC: 481706E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83037AD0 size=28
    let mut pc: u32 = 0x83037AD0;
    'dispatch: loop {
        match pc {
            0x83037AD0 => {
    //   block [0x83037AD0..0x83037AEC)
	// 83037AD0: 3D600010  lis r11, 0x10
	ctx.r[11].s64 = 1048576;
	// 83037AD4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83037AD8: 7D645810  subfc r11, r4, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[4].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 83037ADC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83037AE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83037AE4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83037AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037AF0 size=88
    let mut pc: u32 = 0x83037AF0;
    'dispatch: loop {
        match pc {
            0x83037AF0 => {
    //   block [0x83037AF0..0x83037B48)
	// 83037AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037B04: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037B08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037B0C: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 83037B10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83037B14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037B18: 4BFBDDB9  bl 0x82ff58d0
	ctx.lr = 0x83037B1C;
	sub_82FF58D0(ctx, base);
	// 83037B1C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83037B20: 4182000C  beq 0x83037b2c
	if ctx.cr[0].eq {
	pc = 0x83037B2C; continue 'dispatch;
	}
	// 83037B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037B28: 4BFA07B9  bl 0x82fd82e0
	ctx.lr = 0x83037B2C;
	sub_82FD82E0(ctx, base);
	// 83037B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037B3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037B48 size=60
    let mut pc: u32 = 0x83037B48;
    'dispatch: loop {
        match pc {
            0x83037B48 => {
    //   block [0x83037B48..0x83037B84)
	// 83037B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037B5C: 4BFBDE0D  bl 0x82ff5968
	ctx.lr = 0x83037B60;
	sub_82FF5968(ctx, base);
	// 83037B60: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037B68: 396B9B30  addi r11, r11, -0x64d0
	ctx.r[11].s64 = ctx.r[11].s64 + -25808;
	// 83037B6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83037B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037B88 size=120
    let mut pc: u32 = 0x83037B88;
    'dispatch: loop {
        match pc {
            0x83037B88 => {
    //   block [0x83037B88..0x83037C00)
	// 83037B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037B98: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83037B9C: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037BA0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83037BA4: 41980008  blt cr6, 0x83037bac
	if ctx.cr[6].lt {
	pc = 0x83037BAC; continue 'dispatch;
	}
	// 83037BA8: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83037BAC: 7CFF2214  add r7, r31, r4
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 83037BB0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83037BB4: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037BB8: 4098001C  bge cr6, 0x83037bd4
	if !ctx.cr[6].lt {
	pc = 0x83037BD4; continue 'dispatch;
	}
	// 83037BBC: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037BC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83037BC4: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037BC8: B0CA0000  sth r6, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 83037BCC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83037BD0: 4198FFEC  blt cr6, 0x83037bbc
	if ctx.cr[6].lt {
	pc = 0x83037BBC; continue 'dispatch;
	}
	// 83037BD4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83037BD8: 93E80000  stw r31, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83037BDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83037BE0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83037BE4: 481705FD  bl 0x831a81e0
	ctx.lr = 0x83037BE8;
	sub_831A81E0(ctx, base);
	// 83037BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037BEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83037BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037BF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037C00 size=220
    let mut pc: u32 = 0x83037C00;
    'dispatch: loop {
        match pc {
            0x83037C00 => {
    //   block [0x83037C00..0x83037CDC)
	// 83037C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037C0C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037C14: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83037C18: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83037C1C: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037C20: 40980008  bge cr6, 0x83037c28
	if !ctx.cr[6].lt {
	pc = 0x83037C28; continue 'dispatch;
	}
	// 83037C24: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83037C28: 54E4083C  slwi r4, r7, 1
	ctx.r[4].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83037C2C: 7CC45A14  add r6, r4, r11
	ctx.r[6].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 83037C30: 48000030  b 0x83037c60
	pc = 0x83037C60; continue 'dispatch;
	// 83037C34: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037C38: 2B040100  cmplwi cr6, r4, 0x100
	ctx.cr[6].compare_u32(ctx.r[4].u32, 256 as u32, &mut ctx.xer);
	// 83037C3C: 4098000C  bge cr6, 0x83037c48
	if !ctx.cr[6].lt {
	pc = 0x83037C48; continue 'dispatch;
	}
	// 83037C40: 988A0000  stb r4, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83037C44: 48000014  b 0x83037c58
	pc = 0x83037C58; continue 'dispatch;
	// 83037C48: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83037C4C: 419A0038  beq cr6, 0x83037c84
	if ctx.cr[6].eq {
	pc = 0x83037C84; continue 'dispatch;
	}
	// 83037C50: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 83037C54: 98AA0000  stb r5, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83037C58: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83037C5C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83037C60: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 83037C64: 4198FFD0  blt cr6, 0x83037c34
	if ctx.cr[6].lt {
	pc = 0x83037C34; continue 'dispatch;
	}
	// 83037C68: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83037C6C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83037C70: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83037C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037C80: 4E800020  blr
	return;
	// 83037C84: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83037C88: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037C8C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83037C90: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037C94: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83037C98: 4BF99BD1  bl 0x82fd1868
	ctx.lr = 0x83037C9C;
	sub_82FD1868(ctx, base);
	// 83037C9C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83037CA0: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037CA4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83037CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83037CAC: 388B9B40  addi r4, r11, -0x64c0
	ctx.r[4].s64 = ctx.r[11].s64 + -25792;
	// 83037CB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83037CB4: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83037CB8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 83037CBC: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 83037CC0: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 83037CC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83037CC8: 4BFB4FE1  bl 0x82fecca8
	ctx.lr = 0x83037CCC;
	sub_82FECCA8(ctx, base);
	// 83037CCC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83037CD0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83037CD4: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 83037CD8: 48178F51  bl 0x831b0c28
	ctx.lr = 0x83037CDC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83037CE0 size=20
    let mut pc: u32 = 0x83037CE0;
    'dispatch: loop {
        match pc {
            0x83037CE0 => {
    //   block [0x83037CE0..0x83037CF4)
	// 83037CE0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 83037CE4: 7D6B2010  subfc r11, r11, r4
	ctx.xer.ca = ctx.r[4].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83037CE8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83037CEC: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83037CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037CF8 size=88
    let mut pc: u32 = 0x83037CF8;
    'dispatch: loop {
        match pc {
            0x83037CF8 => {
    //   block [0x83037CF8..0x83037D50)
	// 83037CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037D0C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037D10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037D14: 396B9B30  addi r11, r11, -0x64d0
	ctx.r[11].s64 = ctx.r[11].s64 + -25808;
	// 83037D18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83037D1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037D20: 4BFBDBB1  bl 0x82ff58d0
	ctx.lr = 0x83037D24;
	sub_82FF58D0(ctx, base);
	// 83037D24: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83037D28: 4182000C  beq 0x83037d34
	if ctx.cr[0].eq {
	pc = 0x83037D34; continue 'dispatch;
	}
	// 83037D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037D30: 4BFA05B1  bl 0x82fd82e0
	ctx.lr = 0x83037D34;
	sub_82FD82E0(ctx, base);
	// 83037D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037D44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037D50 size=80
    let mut pc: u32 = 0x83037D50;
    'dispatch: loop {
        match pc {
            0x83037D50 => {
    //   block [0x83037D50..0x83037DA0)
	// 83037D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037D58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037D5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037D60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037D64: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83037D68: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83037D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037D70: 4BFBDBF9  bl 0x82ff5968
	ctx.lr = 0x83037D74;
	sub_82FF5968(ctx, base);
	// 83037D74: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037D7C: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 83037D80: 396B9B78  addi r11, r11, -0x6488
	ctx.r[11].s64 = ctx.r[11].s64 + -25736;
	// 83037D84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037D88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037D94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037DA0 size=160
    let mut pc: u32 = 0x83037DA0;
    'dispatch: loop {
        match pc {
            0x83037DA0 => {
    //   block [0x83037DA0..0x83037E40)
	// 83037DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037DA4: 481703C9  bl 0x831a816c
	ctx.lr = 0x83037DA8;
	sub_831A8130(ctx, base);
	// 83037DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037DAC: 54ABF87E  srwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037DB0: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83037DB4: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 83037DB8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83037DBC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83037DC0: 41980008  blt cr6, 0x83037dc8
	if ctx.cr[6].lt {
	pc = 0x83037DC8; continue 'dispatch;
	}
	// 83037DC4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83037DC8: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83037DCC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 83037DD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83037DD4: 4182003C  beq 0x83037e10
	if ctx.cr[0].eq {
	pc = 0x83037E10; continue 'dispatch;
	}
	// 83037DD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83037DDC: 419A0040  beq cr6, 0x83037e1c
	if ctx.cr[6].eq {
	pc = 0x83037E1C; continue 'dispatch;
	}
	// 83037DE0: 7D043050  subf r8, r4, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[4].s64;
	// 83037DE4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83037DE8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037DEC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83037DF0: 5527403E  rotlwi r7, r9, 8
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83037DF4: 5529C23E  srwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83037DF8: 54E7043E  clrlwi r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 83037DFC: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 83037E00: 7D28532E  sthx r9, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 83037E04: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83037E08: 4082FFE0  bne 0x83037de8
	if !ctx.cr[0].eq {
	pc = 0x83037DE8; continue 'dispatch;
	}
	// 83037E0C: 48000010  b 0x83037e1c
	pc = 0x83037E1C; continue 'dispatch;
	// 83037E10: 57E5083C  slwi r5, r31, 1
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83037E14: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83037E18: 481706F9  bl 0x831a8510
	ctx.lr = 0x83037E1C;
	sub_831A8510(ctx, base);
	// 83037E1C: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037E20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83037E24: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83037E28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83037E2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037E30: 481703B1  bl 0x831a81e0
	ctx.lr = 0x83037E34;
	sub_831A81E0(ctx, base);
	// 83037E34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037E38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037E3C: 48170380  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037E40 size=160
    let mut pc: u32 = 0x83037E40;
    'dispatch: loop {
        match pc {
            0x83037E40 => {
    //   block [0x83037E40..0x83037EE0)
	// 83037E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037E54: 54EBF87E  srwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83037E58: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83037E5C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83037E60: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83037E64: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83037E68: 41980008  blt cr6, 0x83037e70
	if ctx.cr[6].lt {
	pc = 0x83037E70; continue 'dispatch;
	}
	// 83037E6C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83037E70: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83037E74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83037E78: 4182003C  beq 0x83037eb4
	if ctx.cr[0].eq {
	pc = 0x83037EB4; continue 'dispatch;
	}
	// 83037E7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83037E80: 419A0040  beq cr6, 0x83037ec0
	if ctx.cr[6].eq {
	pc = 0x83037EC0; continue 'dispatch;
	}
	// 83037E84: 7D0A2050  subf r8, r10, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 83037E88: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83037E8C: 7D28522E  lhzx r9, r8, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83037E90: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83037E94: 5527403E  rotlwi r7, r9, 8
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83037E98: 5529C23E  srwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83037E9C: 54E7043E  clrlwi r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 83037EA0: 7CE94B78  or r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 | ctx.r[9].u64;
	// 83037EA4: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83037EA8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83037EAC: 4082FFE0  bne 0x83037e8c
	if !ctx.cr[0].eq {
	pc = 0x83037E8C; continue 'dispatch;
	}
	// 83037EB0: 48000010  b 0x83037ec0
	pc = 0x83037EC0; continue 'dispatch;
	// 83037EB4: 57E5083C  slwi r5, r31, 1
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83037EB8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83037EBC: 48170655  bl 0x831a8510
	ctx.lr = 0x83037EC0;
	sub_831A8510(ctx, base);
	// 83037EC0: 57E3083C  slwi r3, r31, 1
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83037EC4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83037EC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037ED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037EE0 size=88
    let mut pc: u32 = 0x83037EE0;
    'dispatch: loop {
        match pc {
            0x83037EE0 => {
    //   block [0x83037EE0..0x83037F38)
	// 83037EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037EF4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037EF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037EFC: 396B9B78  addi r11, r11, -0x6488
	ctx.r[11].s64 = ctx.r[11].s64 + -25736;
	// 83037F00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83037F04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037F08: 4BFBD9C9  bl 0x82ff58d0
	ctx.lr = 0x83037F0C;
	sub_82FF58D0(ctx, base);
	// 83037F0C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83037F10: 4182000C  beq 0x83037f1c
	if ctx.cr[0].eq {
	pc = 0x83037F1C; continue 'dispatch;
	}
	// 83037F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037F18: 4BFA03C9  bl 0x82fd82e0
	ctx.lr = 0x83037F1C;
	sub_82FD82E0(ctx, base);
	// 83037F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037F20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037F2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83037F38 size=80
    let mut pc: u32 = 0x83037F38;
    'dispatch: loop {
        match pc {
            0x83037F38 => {
    //   block [0x83037F38..0x83037F88)
	// 83037F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83037F40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83037F44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83037F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83037F4C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83037F50: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83037F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83037F58: 4BFBDA11  bl 0x82ff5968
	ctx.lr = 0x83037F5C;
	sub_82FF5968(ctx, base);
	// 83037F5C: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83037F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83037F64: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 83037F68: 396B9B88  addi r11, r11, -0x6478
	ctx.r[11].s64 = ctx.r[11].s64 + -25720;
	// 83037F6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83037F70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83037F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83037F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83037F7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83037F80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83037F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83037F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83037F88 size=232
    let mut pc: u32 = 0x83037F88;
    'dispatch: loop {
        match pc {
            0x83037F88 => {
    //   block [0x83037F88..0x83038070)
	// 83037F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83037F8C: 481701D9  bl 0x831a8164
	ctx.lr = 0x83037F90;
	sub_831A8130(ctx, base);
	// 83037F90: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83037F94: 54E9083C  slwi r9, r7, 1
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83037F98: 54AB003A  rlwinm r11, r5, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 83037F9C: 7FE93214  add r31, r9, r6
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 83037FA0: 7FAB2214  add r29, r11, r4
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83037FA4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83037FA8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 83037FAC: 7F06F840  cmplw cr6, r6, r31
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83037FB0: 409800AC  bge cr6, 0x8303805c
	if !ctx.cr[6].lt {
	pc = 0x8303805C; continue 'dispatch;
	}
	// 83037FB4: 38E60002  addi r7, r6, 2
	ctx.r[7].s64 = ctx.r[6].s64 + 2;
	// 83037FB8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83037FBC: 7F05E840  cmplw cr6, r5, r29
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83037FC0: 4098009C  bge cr6, 0x8303805c
	if !ctx.cr[6].lt {
	pc = 0x8303805C; continue 'dispatch;
	}
	// 83037FC4: 8B830010  lbz r28, 0x10(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83037FC8: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83037FCC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83037FD0: 41820020  beq 0x83037ff0
	if ctx.cr[0].eq {
	pc = 0x83037FF0; continue 'dispatch;
	}
	// 83037FD4: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 83037FD8: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83037FDC: 513C843E  rlwimi r28, r9, 0x10, 0x10, 0x1f
	ctx.r[28].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[28].u64 & 0xFFFFFFFFFFFF0000);
	// 83037FE0: 513B801E  rlwimi r27, r9, 0x10, 0, 0xf
	ctx.r[27].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[27].u64 & 0xFFFFFFFF0000FFFF);
	// 83037FE4: 5789C43E  rlwinm r9, r28, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 83037FE8: 577C401E  rlwinm r28, r27, 8, 0, 0xf
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x00FFFFFFu64;
	// 83037FEC: 7D29E378  or r9, r9, r28
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[28].u64;
	// 83037FF0: 553C001F  rlwinm. r28, r9, 0, 0, 0xf
	ctx.r[28].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83037FF4: 41820048  beq 0x8303803c
	if ctx.cr[0].eq {
	pc = 0x8303803C; continue 'dispatch;
	}
	// 83037FF8: 7F07F840  cmplw cr6, r7, r31
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83037FFC: 419A0060  beq cr6, 0x8303805c
	if ctx.cr[6].eq {
	pc = 0x8303805C; continue 'dispatch;
	}
	// 83038000: 3F89FFFF  addis r28, r9, -1
	ctx.r[28].s64 = ctx.r[9].s64 + -65536;
	// 83038004: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 83038008: 552905BE  clrlwi r9, r9, 0x16
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000003FFu64;
	// 8303800C: 579CB2BE  srwi r28, r28, 0xa
	ctx.r[28].u32 = ctx.r[28].u32.wrapping_shr(10);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83038010: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83038014: 3F9C0001  addis r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 65536;
	// 83038018: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303801C: 3B9CD800  addi r28, r28, -0x2800
	ctx.r[28].s64 = ctx.r[28].s64 + -10240;
	// 83038020: 3D290001  addis r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 65536;
	// 83038024: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 83038028: 3929DC00  addi r9, r9, -0x2400
	ctx.r[9].s64 = ctx.r[9].s64 + -9216;
	// 8303802C: B38B0000  sth r28, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 83038030: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83038034: 9B6A0000  stb r27, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 83038038: 48000008  b 0x83038040
	pc = 0x83038040; continue 'dispatch;
	// 8303803C: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 83038040: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83038044: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83038048: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8303804C: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 83038050: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 83038054: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83038058: 4198FF64  blt cr6, 0x83037fbc
	if ctx.cr[6].lt {
	pc = 0x83037FBC; continue 'dispatch;
	}
	// 8303805C: 7D442850  subf r10, r4, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 83038060: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 83038064: 7D630E70  srawi r3, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83038068: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303806C: 48170148  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038070 size=288
    let mut pc: u32 = 0x83038070;
    'dispatch: loop {
        match pc {
            0x83038070 => {
    //   block [0x83038070..0x83038190)
	// 83038070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038074: 481700F9  bl 0x831a816c
	ctx.lr = 0x83038078;
	sub_831A8130(ctx, base);
	// 83038078: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303807C: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 83038080: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83038084: 7FCA3214  add r30, r10, r6
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83038088: 7FAB2214  add r29, r11, r4
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8303808C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 83038090: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 83038094: 7F06F040  cmplw cr6, r6, r30
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83038098: 409800B4  bge cr6, 0x8303814c
	if !ctx.cr[6].lt {
	pc = 0x8303814C; continue 'dispatch;
	}
	// 8303809C: 3BE40002  addi r31, r4, 2
	ctx.r[31].s64 = ctx.r[4].s64 + 2;
	// 830380A0: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830380A4: 409800A8  bge cr6, 0x8303814c
	if !ctx.cr[6].lt {
	pc = 0x8303814C; continue 'dispatch;
	}
	// 830380A8: A0AA0000  lhz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830380AC: 2B05D800  cmplwi cr6, r5, 0xd800
	ctx.cr[6].compare_u32(ctx.r[5].u32, 55296 as u32, &mut ctx.xer);
	// 830380B0: 41980054  blt cr6, 0x83038104
	if ctx.cr[6].lt {
	pc = 0x83038104; continue 'dispatch;
	}
	// 830380B4: 2B05DBFF  cmplwi cr6, r5, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[5].u32, 56319 as u32, &mut ctx.xer);
	// 830380B8: 4199004C  bgt cr6, 0x83038104
	if ctx.cr[6].gt {
	pc = 0x83038104; continue 'dispatch;
	}
	// 830380BC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830380C0: 419A008C  beq cr6, 0x8303814c
	if ctx.cr[6].eq {
	pc = 0x8303814C; continue 'dispatch;
	}
	// 830380C4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830380C8: 393F0002  addi r9, r31, 2
	ctx.r[9].s64 = ctx.r[31].s64 + 2;
	// 830380CC: 3BE90002  addi r31, r9, 2
	ctx.r[31].s64 = ctx.r[9].s64 + 2;
	// 830380D0: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830380D4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830380D8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 830380DC: 2B09DC00  cmplwi cr6, r9, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[9].u32, 56320 as u32, &mut ctx.xer);
	// 830380E0: 41980084  blt cr6, 0x83038164
	if ctx.cr[6].lt {
	pc = 0x83038164; continue 'dispatch;
	}
	// 830380E4: 2B09DFFF  cmplwi cr6, r9, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 57343 as u32, &mut ctx.xer);
	// 830380E8: 4199007C  bgt cr6, 0x83038164
	if ctx.cr[6].gt {
	pc = 0x83038164; continue 'dispatch;
	}
	// 830380EC: 3CE5FFFF  addis r7, r5, -1
	ctx.r[7].s64 = ctx.r[5].s64 + -65536;
	// 830380F0: 38E72809  addi r7, r7, 0x2809
	ctx.r[7].s64 = ctx.r[7].s64 + 10249;
	// 830380F4: 54E7502A  slwi r7, r7, 0xa
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(10);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 830380F8: 7D274A14  add r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 830380FC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83038100: 48000040  b 0x83038140
	pc = 0x83038140; continue 'dispatch;
	// 83038104: 89230010  lbz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83038108: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303810C: 41820028  beq 0x83038134
	if ctx.cr[0].eq {
	pc = 0x83038134; continue 'dispatch;
	}
	// 83038110: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 83038114: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83038118: 50A9843E  rlwimi r9, r5, 0x10, 0x10, 0x1f
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF0000);
	// 8303811C: 50A7801E  rlwimi r7, r5, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 83038120: 5529C43E  rlwinm r9, r9, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 83038124: 54E7401E  rlwinm r7, r7, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 83038128: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8303812C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83038130: 48000008  b 0x83038138
	pc = 0x83038138; continue 'dispatch;
	// 83038134: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83038138: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8303813C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 83038140: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83038144: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83038148: 4198FF58  blt cr6, 0x830380a0
	if ctx.cr[6].lt {
	pc = 0x830380A0; continue 'dispatch;
	}
	// 8303814C: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 83038150: 7C665850  subf r3, r6, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 83038154: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83038158: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303815C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83038160: 4817005C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83038164: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83038168: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303816C: 38C00061  li r6, 0x61
	ctx.r[6].s64 = 97;
	// 83038170: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 83038174: 38A000D2  li r5, 0xd2
	ctx.r[5].s64 = 210;
	// 83038178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303817C: 4BFB9ED5  bl 0x82ff2050
	ctx.lr = 0x83038180;
	sub_82FF2050(ctx, base);
	// 83038180: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83038184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83038188: 388BC73C  addi r4, r11, -0x38c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14532;
	// 8303818C: 48178A9D  bl 0x831b0c28
	ctx.lr = 0x83038190;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038190 size=88
    let mut pc: u32 = 0x83038190;
    'dispatch: loop {
        match pc {
            0x83038190 => {
    //   block [0x83038190..0x830381E8)
	// 83038190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303819C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830381A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830381A4: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830381A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830381AC: 396B9B88  addi r11, r11, -0x6478
	ctx.r[11].s64 = ctx.r[11].s64 + -25720;
	// 830381B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830381B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830381B8: 4BFBD719  bl 0x82ff58d0
	ctx.lr = 0x830381BC;
	sub_82FF58D0(ctx, base);
	// 830381BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830381C0: 4182000C  beq 0x830381cc
	if ctx.cr[0].eq {
	pc = 0x830381CC; continue 'dispatch;
	}
	// 830381C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830381C8: 4BFA0119  bl 0x82fd82e0
	ctx.lr = 0x830381CC;
	sub_82FD82E0(ctx, base);
	// 830381CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830381D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830381D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830381D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830381DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830381E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830381E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830381E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830381E8 size=84
    let mut pc: u32 = 0x830381E8;
    'dispatch: loop {
        match pc {
            0x830381E8 => {
    //   block [0x830381E8..0x8303823C)
	// 830381E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830381EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830381F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830381F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830381F8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830381FC: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 83038200: 396B9BD0  addi r11, r11, -0x6430
	ctx.r[11].s64 = ctx.r[11].s64 + -25648;
	// 83038204: 39000160  li r8, 0x160
	ctx.r[8].s64 = 352;
	// 83038208: 38EB0200  addi r7, r11, 0x200
	ctx.r[7].s64 = ctx.r[11].s64 + 512;
	// 8303820C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83038210: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038214: 48047855  bl 0x8307fa68
	ctx.lr = 0x83038218;
	sub_8307FA68(ctx, base);
	// 83038218: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 8303821C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038220: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 83038224: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303822C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038240 size=84
    let mut pc: u32 = 0x83038240;
    'dispatch: loop {
        match pc {
            0x83038240 => {
    //   block [0x83038240..0x83038294)
	// 83038240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303824C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038250: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83038254: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 83038258: 396BA350  addi r11, r11, -0x5cb0
	ctx.r[11].s64 = ctx.r[11].s64 + -23728;
	// 8303825C: 3900015F  li r8, 0x15f
	ctx.r[8].s64 = 351;
	// 83038260: 38EB0200  addi r7, r11, 0x200
	ctx.r[7].s64 = ctx.r[11].s64 + 512;
	// 83038264: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83038268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303826C: 480477FD  bl 0x8307fa68
	ctx.lr = 0x83038270;
	sub_8307FA68(ctx, base);
	// 83038270: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83038274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038278: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 8303827C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303828C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038298 size=84
    let mut pc: u32 = 0x83038298;
    'dispatch: loop {
        match pc {
            0x83038298 => {
    //   block [0x83038298..0x830382EC)
	// 83038298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830382A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830382A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830382A8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830382AC: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 830382B0: 396BAAD0  addi r11, r11, -0x5530
	ctx.r[11].s64 = ctx.r[11].s64 + -21808;
	// 830382B4: 3900015E  li r8, 0x15e
	ctx.r[8].s64 = 350;
	// 830382B8: 38EB0200  addi r7, r11, 0x200
	ctx.r[7].s64 = ctx.r[11].s64 + 512;
	// 830382BC: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830382C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830382C4: 480477A5  bl 0x8307fa68
	ctx.lr = 0x830382C8;
	sub_8307FA68(ctx, base);
	// 830382C8: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 830382CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830382D0: 396B95C8  addi r11, r11, -0x6a38
	ctx.r[11].s64 = ctx.r[11].s64 + -27192;
	// 830382D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830382D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830382DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830382E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830382E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830382E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830382F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830382F0 size=112
    let mut pc: u32 = 0x830382F0;
    'dispatch: loop {
        match pc {
            0x830382F0 => {
    //   block [0x830382F0..0x83038360)
	// 830382F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830382F4: 4816FE75  bl 0x831a8168
	ctx.lr = 0x830382F8;
	sub_831A8130(ctx, base);
	// 830382F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830382FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83038300: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83038304: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83038308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303830C: 4099004C  ble cr6, 0x83038358
	if !ctx.cr[6].gt {
	pc = 0x83038358; continue 'dispatch;
	}
	// 83038310: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83038314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038318: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8303831C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038320: 41820018  beq 0x83038338
	if ctx.cr[0].eq {
	pc = 0x83038338; continue 'dispatch;
	}
	// 83038324: 83830004  lwz r28, 4(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038328: 4BF9FFB9  bl 0x82fd82e0
	ctx.lr = 0x8303832C;
	sub_82FD82E0(ctx, base);
	// 8303832C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83038330: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83038334: 409AFFF0  bne cr6, 0x83038324
	if !ctx.cr[6].eq {
	pc = 0x83038324; continue 'dispatch;
	}
	// 83038338: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303833C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83038340: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83038344: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 83038348: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8303834C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83038350: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83038354: 4198FFC0  blt cr6, 0x83038314
	if ctx.cr[6].lt {
	pc = 0x83038314; continue 'dispatch;
	}
	// 83038358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303835C: 4816FE5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038360 size=196
    let mut pc: u32 = 0x83038360;
    'dispatch: loop {
        match pc {
            0x83038360 => {
    //   block [0x83038360..0x83038424)
	// 83038360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038364: 4816FE09  bl 0x831a816c
	ctx.lr = 0x83038368;
	sub_831A8130(ctx, base);
	// 83038368: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303836C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038370: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83038374: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83038378: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303837C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038380: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83038384: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038388: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303838C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83038390: 4E800421  bctrl
	ctx.lr = 0x83038394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83038394: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83038398: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303839C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830383A0: 40990030  ble cr6, 0x830383d0
	if !ctx.cr[6].gt {
	pc = 0x830383D0; continue 'dispatch;
	}
	// 830383A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830383A8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830383AC: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 830383B0: 388B6190  addi r4, r11, 0x6190
	ctx.r[4].s64 = ctx.r[11].s64 + 24976;
	// 830383B4: 38A000F3  li r5, 0xf3
	ctx.r[5].s64 = 243;
	// 830383B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830383BC: 4BF98C9D  bl 0x82fd1058
	ctx.lr = 0x830383C0;
	sub_82FD1058(ctx, base);
	// 830383C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830383C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830383C8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830383CC: 4817885D  bl 0x831b0c28
	ctx.lr = 0x830383D0;
	sub_831B0C28(ctx, base);
	// 830383D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830383D4: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830383D8: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830383DC: 4800002C  b 0x83038408
	pc = 0x83038408; continue 'dispatch;
	// 830383E0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830383E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830383E8: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830383EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830383F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830383F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830383F8: 4E800421  bctrl
	ctx.lr = 0x830383FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830383FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83038400: 4082001C  bne 0x8303841c
	if !ctx.cr[0].eq {
	pc = 0x8303841C; continue 'dispatch;
	}
	// 83038404: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038408: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303840C: 4082FFD4  bne 0x830383e0
	if !ctx.cr[0].eq {
	pc = 0x830383E0; continue 'dispatch;
	}
	// 83038410: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83038414: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83038418: 4816FDA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8303841C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83038420: 4BFFFFF4  b 0x83038414
	pc = 0x83038414; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038428 size=84
    let mut pc: u32 = 0x83038428;
    'dispatch: loop {
        match pc {
            0x83038428 => {
    //   block [0x83038428..0x8303847C)
	// 83038428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303842C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038430: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038434: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038438: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303843C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83038440: 83FEB988  lwz r31, -0x4678(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18040 as u32) ) } as u64;
	// 83038444: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83038448: 419A0014  beq cr6, 0x8303845c
	if ctx.cr[6].eq {
	pc = 0x8303845C; continue 'dispatch;
	}
	// 8303844C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038450: 4BFBD339  bl 0x82ff5788
	ctx.lr = 0x83038454;
	sub_82FF5788(ctx, base);
	// 83038454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038458: 4BF9FE89  bl 0x82fd82e0
	ctx.lr = 0x8303845C;
	sub_82FD82E0(ctx, base);
	// 8303845C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83038460: 917EB988  stw r11, -0x4678(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18040 as u32), ctx.r[11].u32 ) };
	// 83038464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303846C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038480 size=8
    let mut pc: u32 = 0x83038480;
    'dispatch: loop {
        match pc {
            0x83038480 => {
    //   block [0x83038480..0x83038488)
	// 83038480: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83038484: 8215CEA0  lwz r16, -0x3160(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12640 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038488 size=164
    let mut pc: u32 = 0x83038488;
    'dispatch: loop {
        match pc {
            0x83038488 => {
    //   block [0x83038488..0x8303852C)
	// 83038488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303848C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038498: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8303849C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830384A0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830384A4: 807EB988  lwz r3, -0x4678(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18040 as u32) ) } as u64;
	// 830384A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830384AC: 409A0068  bne cr6, 0x83038514
	if !ctx.cr[6].eq {
	pc = 0x83038514; continue 'dispatch;
	}
	// 830384B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830384B4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830384B8: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830384BC: 4BFBD31D  bl 0x82ff57d8
	ctx.lr = 0x830384C0;
	sub_82FF57D8(ctx, base);
	// 830384C0: 817EB988  lwz r11, -0x4678(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18040 as u32) ) } as u64;
	// 830384C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830384C8: 409A0040  bne cr6, 0x83038508
	if !ctx.cr[6].eq {
	pc = 0x83038508; continue 'dispatch;
	}
	// 830384CC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830384D0: 4BF9FD79  bl 0x82fd8248
	ctx.lr = 0x830384D4;
	sub_82FD8248(ctx, base);
	// 830384D4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830384D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830384DC: 41820010  beq 0x830384ec
	if ctx.cr[0].eq {
	pc = 0x830384EC; continue 'dispatch;
	}
	// 830384E0: 4BFBD269  bl 0x82ff5748
	ctx.lr = 0x830384E4;
	sub_82FF5748(ctx, base);
	// 830384E4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830384E8: 48000008  b 0x830384f0
	pc = 0x830384F0; continue 'dispatch;
	// 830384EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830384F0: 3D608304  lis r11, -0x7cfc
	ctx.r[11].s64 = -2096889856;
	// 830384F4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830384F8: 913EB988  stw r9, -0x4678(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18040 as u32), ctx.r[9].u32 ) };
	// 830384FC: 388B8428  addi r4, r11, -0x7bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31704;
	// 83038500: 386AB990  addi r3, r10, -0x4670
	ctx.r[3].s64 = ctx.r[10].s64 + -18032;
	// 83038504: 4BFBF635  bl 0x82ff7b38
	ctx.lr = 0x83038508;
	sub_82FF7B38(ctx, base);
	// 83038508: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303850C: 4BFBD305  bl 0x82ff5810
	ctx.lr = 0x83038510;
	sub_82FF5810(ctx, base);
	// 83038510: 807EB988  lwz r3, -0x4678(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18040 as u32) ) } as u64;
	// 83038514: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83038518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303851C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038520: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303852C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303852C size=40
    let mut pc: u32 = 0x8303852C;
    'dispatch: loop {
        match pc {
            0x8303852C => {
    //   block [0x8303852C..0x83038554)
	// 8303852C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83038530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303853C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83038540: 4BFBD2D1  bl 0x82ff5810
	ctx.lr = 0x83038544;
	sub_82FF5810(ctx, base);
	// 83038544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303854C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038554(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038554 size=40
    let mut pc: u32 = 0x83038554;
    'dispatch: loop {
        match pc {
            0x83038554 => {
    //   block [0x83038554..0x8303857C)
	// 83038554: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83038558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303855C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038564: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83038568: 4BF9FD79  bl 0x82fd82e0
	ctx.lr = 0x8303856C;
	sub_82FD82E0(ctx, base);
	// 8303856C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038580 size=100
    let mut pc: u32 = 0x83038580;
    'dispatch: loop {
        match pc {
            0x83038580 => {
    //   block [0x83038580..0x830385E4)
	// 83038580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303858C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038594: 4BFFFD5D  bl 0x830382f0
	ctx.lr = 0x83038598;
	sub_830382F0(ctx, base);
	// 83038598: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303859C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830385A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830385A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830385A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830385AC: 4E800421  bctrl
	ctx.lr = 0x830385B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830385B0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830385B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830385B8: 41820018  beq 0x830385d0
	if ctx.cr[0].eq {
	pc = 0x830385D0; continue 'dispatch;
	}
	// 830385BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830385C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830385C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830385C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830385CC: 4E800421  bctrl
	ctx.lr = 0x830385D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830385D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830385D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830385D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830385DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830385E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830385E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830385E8 size=140
    let mut pc: u32 = 0x830385E8;
    'dispatch: loop {
        match pc {
            0x830385E8 => {
    //   block [0x830385E8..0x83038674)
	// 830385E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830385EC: 4816FB81  bl 0x831a816c
	ctx.lr = 0x830385F0;
	sub_831A8130(ctx, base);
	// 830385F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830385F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830385F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830385FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038600: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038604: 4BFED39D  bl 0x830259a0
	ctx.lr = 0x83038608;
	sub_830259A0(ctx, base);
	// 83038608: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303860C: 41820014  beq 0x83038620
	if ctx.cr[0].eq {
	pc = 0x83038620; continue 'dispatch;
	}
	// 83038610: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038614: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83038618: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8303861C: 48000050  b 0x8303866c
	pc = 0x8303866C; continue 'dispatch;
	// 83038620: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83038624: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038628: 4BF9FC71  bl 0x82fd8298
	ctx.lr = 0x8303862C;
	sub_82FD8298(ctx, base);
	// 8303862C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83038630: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038634: 41820028  beq 0x8303865c
	if ctx.cr[0].eq {
	pc = 0x8303865C; continue 'dispatch;
	}
	// 83038638: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303863C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83038640: 88FD0000  lbz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038644: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83038648: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8303864C: 98E30000  stb r7, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 83038650: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83038654: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83038658: 48000008  b 0x83038660
	pc = 0x83038660; continue 'dispatch;
	// 8303865C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83038660: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038664: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83038668: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8303866C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83038670: 4816FB4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038678 size=64
    let mut pc: u32 = 0x83038678;
    'dispatch: loop {
        match pc {
            0x83038678 => {
    //   block [0x83038678..0x830386B8)
	// 83038678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303867C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038684: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83038688: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303868C: 4BFFFCD5  bl 0x83038360
	ctx.lr = 0x83038690;
	sub_83038360(ctx, base);
	// 83038690: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83038694: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83038698: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8303869C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830386A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830386A4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830386A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830386AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830386B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830386B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830386B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830386B8 size=8
    let mut pc: u32 = 0x830386B8;
    'dispatch: loop {
        match pc {
            0x830386B8 => {
    //   block [0x830386B8..0x830386C0)
	// 830386B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830386BC: 8215CEF0  lwz r16, -0x3110(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830386C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830386C0 size=124
    let mut pc: u32 = 0x830386C0;
    'dispatch: loop {
        match pc {
            0x830386C0 => {
    //   block [0x830386C0..0x8303873C)
	// 830386C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830386C4: 4816FAA5  bl 0x831a8168
	ctx.lr = 0x830386C8;
	sub_831A8130(ctx, base);
	// 830386C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830386CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830386D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830386D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830386D8: 4BF9FB71  bl 0x82fd8248
	ctx.lr = 0x830386DC;
	sub_82FD8248(ctx, base);
	// 830386DC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830386E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830386E4: 41820018  beq 0x830386fc
	if ctx.cr[0].eq {
	pc = 0x830386FC; continue 'dispatch;
	}
	// 830386E8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830386EC: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 830386F0: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830386F4: 4BFEDC6D  bl 0x83026360
	ctx.lr = 0x830386F8;
	sub_83026360(ctx, base);
	// 830386F8: 48000008  b 0x83038700
	pc = 0x83038700; continue 'dispatch;
	// 830386FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83038700: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 83038704: 3BC00317  li r30, 0x317
	ctx.r[30].s64 = 791;
	// 83038708: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8303870C: 3BABB248  addi r29, r11, -0x4db8
	ctx.r[29].s64 = ctx.r[11].s64 + -19896;
	// 83038710: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83038714: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 83038718: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8303871C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038720: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83038724: 4BFFFEC5  bl 0x830385e8
	ctx.lr = 0x83038728;
	sub_830385E8(ctx, base);
	// 83038728: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303872C: 3BBD005C  addi r29, r29, 0x5c
	ctx.r[29].s64 = ctx.r[29].s64 + 92;
	// 83038730: 4082FFE8  bne 0x83038718
	if !ctx.cr[0].eq {
	pc = 0x83038718; continue 'dispatch;
	}
	// 83038734: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83038738: 4816FA80  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303873C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303873C size=40
    let mut pc: u32 = 0x8303873C;
    'dispatch: loop {
        match pc {
            0x8303873C => {
    //   block [0x8303873C..0x83038764)
	// 8303873C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83038740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303874C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83038750: 4BF9FB91  bl 0x82fd82e0
	ctx.lr = 0x83038754;
	sub_82FD82E0(ctx, base);
	// 83038754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303875C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038768 size=84
    let mut pc: u32 = 0x83038768;
    'dispatch: loop {
        match pc {
            0x83038768 => {
    //   block [0x83038768..0x830387BC)
	// 83038768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303877C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83038780: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038784: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038788: 41820014  beq 0x8303879c
	if ctx.cr[0].eq {
	pc = 0x8303879C; continue 'dispatch;
	}
	// 8303878C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038790: 4BFFFDF1  bl 0x83038580
	ctx.lr = 0x83038794;
	sub_83038580(ctx, base);
	// 83038794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038798: 4BF9FB49  bl 0x82fd82e0
	ctx.lr = 0x8303879C;
	sub_82FD82E0(ctx, base);
	// 8303879C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830387A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830387A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830387A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830387AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830387B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830387B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830387B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830387C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830387C0 size=84
    let mut pc: u32 = 0x830387C0;
    'dispatch: loop {
        match pc {
            0x830387C0 => {
    //   block [0x830387C0..0x83038814)
	// 830387C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830387C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830387C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830387CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830387D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830387D4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830387D8: 83FEB98C  lwz r31, -0x4674(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 830387DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830387E0: 419A0014  beq cr6, 0x830387f4
	if ctx.cr[6].eq {
	pc = 0x830387F4; continue 'dispatch;
	}
	// 830387E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830387E8: 4BFFFF81  bl 0x83038768
	ctx.lr = 0x830387EC;
	sub_83038768(ctx, base);
	// 830387EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830387F0: 4B287A79  bl 0x822c0268
	ctx.lr = 0x830387F4;
	sub_822C0268(ctx, base);
	// 830387F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830387F8: 917EB98C  stw r11, -0x4674(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18036 as u32), ctx.r[11].u32 ) };
	// 830387FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038808: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303880C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038818 size=8
    let mut pc: u32 = 0x83038818;
    'dispatch: loop {
        match pc {
            0x83038818 => {
    //   block [0x83038818..0x83038820)
	// 83038818: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303881C: 8215CF40  lwz r16, -0x30c0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038820 size=152
    let mut pc: u32 = 0x83038820;
    'dispatch: loop {
        match pc {
            0x83038820 => {
    //   block [0x83038820..0x830388B8)
	// 83038820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038824: 4816F949  bl 0x831a816c
	ctx.lr = 0x83038828;
	sub_831A8130(ctx, base);
	// 83038828: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303882C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038830: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83038834: 807EB98C  lwz r3, -0x4674(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 83038838: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8303883C: 409A0074  bne cr6, 0x830388b0
	if !ctx.cr[6].eq {
	pc = 0x830388B0; continue 'dispatch;
	}
	// 83038840: 4BFFFC49  bl 0x83038488
	ctx.lr = 0x83038844;
	sub_83038488(ctx, base);
	// 83038844: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83038848: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303884C: 4BFBCF8D  bl 0x82ff57d8
	ctx.lr = 0x83038850;
	sub_82FF57D8(ctx, base);
	// 83038850: 817EB98C  lwz r11, -0x4674(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 83038854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83038858: 409A004C  bne cr6, 0x830388a4
	if !ctx.cr[6].eq {
	pc = 0x830388A4; continue 'dispatch;
	}
	// 8303885C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83038860: 4B2880D9  bl 0x822c0938
	ctx.lr = 0x83038864;
	sub_822C0938(ctx, base);
	// 83038864: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83038868: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8303886C: 4182001C  beq 0x83038888
	if ctx.cr[0].eq {
	pc = 0x83038888; continue 'dispatch;
	}
	// 83038870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83038874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83038878: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303887C: 4BFFFE45  bl 0x830386c0
	ctx.lr = 0x83038880;
	sub_830386C0(ctx, base);
	// 83038880: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 83038884: 48000008  b 0x8303888c
	pc = 0x8303888C; continue 'dispatch;
	// 83038888: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303888C: 3D608304  lis r11, -0x7cfc
	ctx.r[11].s64 = -2096889856;
	// 83038890: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83038894: 913EB98C  stw r9, -0x4674(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18036 as u32), ctx.r[9].u32 ) };
	// 83038898: 388B87C0  addi r4, r11, -0x7840
	ctx.r[4].s64 = ctx.r[11].s64 + -30784;
	// 8303889C: 386AB99C  addi r3, r10, -0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + -18020;
	// 830388A0: 4BFBF299  bl 0x82ff7b38
	ctx.lr = 0x830388A4;
	sub_82FF7B38(ctx, base);
	// 830388A4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830388A8: 4BFBCF69  bl 0x82ff5810
	ctx.lr = 0x830388AC;
	sub_82FF5810(ctx, base);
	// 830388AC: 807EB98C  lwz r3, -0x4674(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 830388B0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830388B4: 4816F908  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830388B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830388B8 size=40
    let mut pc: u32 = 0x830388B8;
    'dispatch: loop {
        match pc {
            0x830388B8 => {
    //   block [0x830388B8..0x830388E0)
	// 830388B8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830388BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830388C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830388C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830388C8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830388CC: 4BFBCF45  bl 0x82ff5810
	ctx.lr = 0x830388D0;
	sub_82FF5810(ctx, base);
	// 830388D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830388D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830388D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830388DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830388E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830388E0 size=40
    let mut pc: u32 = 0x830388E0;
    'dispatch: loop {
        match pc {
            0x830388E0 => {
    //   block [0x830388E0..0x83038908)
	// 830388E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830388E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830388E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830388EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830388F0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830388F4: 4B287975  bl 0x822c0268
	ctx.lr = 0x830388F8;
	sub_822C0268(ctx, base);
	// 830388F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830388FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038908 size=88
    let mut pc: u32 = 0x83038908;
    'dispatch: loop {
        match pc {
            0x83038908 => {
    //   block [0x83038908..0x83038960)
	// 83038908: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8303890C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83038910: 419A0058  beq cr6, 0x83038968
	if ctx.cr[6].eq {
		sub_83038960(ctx, base);
		return;
	}
	// 83038914: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038918: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8303891C: 419A0044  beq cr6, 0x83038960
	if ctx.cr[6].eq {
		sub_83038960(ctx, base);
		return;
	}
	// 83038920: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038924: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83038928: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8303892C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83038930: 409A004C  bne cr6, 0x8303897c
	if !ctx.cr[6].eq {
		sub_83038960(ctx, base);
		return;
	}
	// 83038934: 7D432050  subf r10, r3, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 83038938: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303893C: 419A0048  beq cr6, 0x83038984
	if ctx.cr[6].eq {
		sub_83038984(ctx, base);
		return;
	}
	// 83038940: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83038944: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038948: 7D0A48AE  lbzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8303894C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83038950: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 83038954: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83038958: 419AFFE0  beq cr6, 0x83038938
	if ctx.cr[6].eq {
	pc = 0x83038938; continue 'dispatch;
	}
	// 8303895C: 48000020  b 0x8303897c
	sub_83038960(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038960 size=36
    let mut pc: u32 = 0x83038960;
    'dispatch: loop {
        match pc {
            0x83038960 => {
    //   block [0x83038960..0x83038984)
	// 83038960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83038964: 409A0018  bne cr6, 0x8303897c
	if !ctx.cr[6].eq {
	pc = 0x8303897C; continue 'dispatch;
	}
	// 83038968: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8303896C: 419A0018  beq cr6, 0x83038984
	if ctx.cr[6].eq {
		sub_83038984(ctx, base);
		return;
	}
	// 83038970: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83038978: 419A000C  beq cr6, 0x83038984
	if ctx.cr[6].eq {
		sub_83038984(ctx, base);
		return;
	}
	// 8303897C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83038980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038984 size=8
    let mut pc: u32 = 0x83038984;
    'dispatch: loop {
        match pc {
            0x83038984 => {
    //   block [0x83038984..0x8303898C)
	// 83038984: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83038988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038990 size=68
    let mut pc: u32 = 0x83038990;
    'dispatch: loop {
        match pc {
            0x83038990 => {
    //   block [0x83038990..0x830389D4)
	// 83038990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038994: 4816F7D9  bl 0x831a816c
	ctx.lr = 0x83038998;
	sub_831A8130(ctx, base);
	// 83038998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303899C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830389A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830389A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830389A8: 4BF989D9  bl 0x82fd1380
	ctx.lr = 0x830389AC;
	sub_82FD1380(ctx, base);
	// 830389AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830389B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830389B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830389B8: 4BFC0941  bl 0x82ff92f8
	ctx.lr = 0x830389BC;
	sub_82FF92F8(ctx, base);
	// 830389BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830389C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830389C4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830389C8: 4BFC05F9  bl 0x82ff8fc0
	ctx.lr = 0x830389CC;
	sub_82FF8FC0(ctx, base);
	// 830389CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830389D0: 4816F7EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830389D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830389D8 size=380
    let mut pc: u32 = 0x830389D8;
    'dispatch: loop {
        match pc {
            0x830389D8 => {
    //   block [0x830389D8..0x83038B54)
	// 830389D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830389DC: 4816F789  bl 0x831a8164
	ctx.lr = 0x830389E0;
	sub_831A8130(ctx, base);
	// 830389E0: 9421F9E0  stwu r1, -0x620(r1)
	ea = ctx.r[1].u32.wrapping_add(-1568 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830389E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830389E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830389EC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830389F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830389F4: 409A0030  bne cr6, 0x83038a24
	if !ctx.cr[6].eq {
	pc = 0x83038A24; continue 'dispatch;
	}
	// 830389F8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830389FC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83038A00: 388BCF90  addi r4, r11, -0x3070
	ctx.r[4].s64 = ctx.r[11].s64 + -12400;
	// 83038A04: 38C00179  li r6, 0x179
	ctx.r[6].s64 = 377;
	// 83038A08: 38A00049  li r5, 0x49
	ctx.r[5].s64 = 73;
	// 83038A0C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038A10: 4BFBF431  bl 0x82ff7e40
	ctx.lr = 0x83038A14;
	sub_82FF7E40(ctx, base);
	// 83038A14: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83038A18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038A1C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83038A20: 48178209  bl 0x831b0c28
	ctx.lr = 0x83038A24;
	sub_831B0C28(ctx, base);
	// 83038A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83038A28: 4BF98959  bl 0x82fd1380
	ctx.lr = 0x83038A2C;
	sub_82FD1380(ctx, base);
	// 83038A2C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83038A30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83038A34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83038A38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83038A3C: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 83038A40: 4BFC0B39  bl 0x82ff9578
	ctx.lr = 0x83038A44;
	sub_82FF9578(ctx, base);
	// 83038A44: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83038A48: 7F05E800  cmpw cr6, r5, r29
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83038A4C: 419A0070  beq cr6, 0x83038abc
	if ctx.cr[6].eq {
	pc = 0x83038ABC; continue 'dispatch;
	}
	// 83038A50: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83038A54: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83038A58: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83038A5C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 83038A60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83038A64: 4BF98E0D  bl 0x82fd1870
	ctx.lr = 0x83038A68;
	sub_82FD1870(ctx, base);
	// 83038A68: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83038A6C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83038A70: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83038A74: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83038A78: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 83038A7C: 4BF98DF5  bl 0x82fd1870
	ctx.lr = 0x83038A80;
	sub_82FD1870(ctx, base);
	// 83038A80: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83038A88: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83038A8C: 388BCF90  addi r4, r11, -0x3070
	ctx.r[4].s64 = ctx.r[11].s64 + -12400;
	// 83038A90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83038A94: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 83038A98: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 83038A9C: 38C0017A  li r6, 0x17a
	ctx.r[6].s64 = 378;
	// 83038AA0: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 83038AA4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038AA8: 4BFBF451  bl 0x82ff7ef8
	ctx.lr = 0x83038AAC;
	sub_82FF7EF8(ctx, base);
	// 83038AAC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83038AB0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038AB4: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83038AB8: 48178171  bl 0x831b0c28
	ctx.lr = 0x83038ABC;
	sub_831B0C28(ctx, base);
	// 83038ABC: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 83038AC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83038AC4: 4BFC05DD  bl 0x82ff90a0
	ctx.lr = 0x83038AC8;
	sub_82FF90A0(ctx, base);
	// 83038AC8: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83038ACC: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 83038AD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83038AD4: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 83038AD8: 7F6A59AE  stbx r27, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 83038ADC: 4BFFFE2D  bl 0x83038908
	ctx.lr = 0x83038AE0;
	sub_83038908(ctx, base);
	// 83038AE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83038AE4: 40820068  bne 0x83038b4c
	if !ctx.cr[0].eq {
	pc = 0x83038B4C; continue 'dispatch;
	}
	// 83038AE8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83038AEC: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83038AF0: 388103F0  addi r4, r1, 0x3f0
	ctx.r[4].s64 = ctx.r[1].s64 + 1008;
	// 83038AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83038AF8: 4BF98941  bl 0x82fd1438
	ctx.lr = 0x83038AFC;
	sub_82FD1438(ctx, base);
	// 83038AFC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83038B00: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 83038B04: 388101F0  addi r4, r1, 0x1f0
	ctx.r[4].s64 = ctx.r[1].s64 + 496;
	// 83038B08: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 83038B0C: 4BF9892D  bl 0x82fd1438
	ctx.lr = 0x83038B10;
	sub_82FD1438(ctx, base);
	// 83038B10: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038B14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83038B18: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83038B1C: 388BCF90  addi r4, r11, -0x3070
	ctx.r[4].s64 = ctx.r[11].s64 + -12400;
	// 83038B20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83038B24: 390101F0  addi r8, r1, 0x1f0
	ctx.r[8].s64 = ctx.r[1].s64 + 496;
	// 83038B28: 38E103F0  addi r7, r1, 0x3f0
	ctx.r[7].s64 = ctx.r[1].s64 + 1008;
	// 83038B2C: 38C0017B  li r6, 0x17b
	ctx.r[6].s64 = 379;
	// 83038B30: 38A00070  li r5, 0x70
	ctx.r[5].s64 = 112;
	// 83038B34: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038B38: 4BFBF3C1  bl 0x82ff7ef8
	ctx.lr = 0x83038B3C;
	sub_82FF7EF8(ctx, base);
	// 83038B3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83038B40: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83038B44: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83038B48: 481780E1  bl 0x831b0c28
	ctx.lr = 0x83038B4C;
	sub_831B0C28(ctx, base);
	// 83038B4C: 38210620  addi r1, r1, 0x620
	ctx.r[1].s64 = ctx.r[1].s64 + 1568;
	// 83038B50: 4816F664  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038B58 size=16
    let mut pc: u32 = 0x83038B58;
    'dispatch: loop {
        match pc {
            0x83038B58 => {
    //   block [0x83038B58..0x83038B68)
	// 83038B58: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038B5C: 396BCFC4  addi r11, r11, -0x303c
	ctx.r[11].s64 = ctx.r[11].s64 + -12348;
	// 83038B60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038B68 size=68
    let mut pc: u32 = 0x83038B68;
    'dispatch: loop {
        match pc {
            0x83038B68 => {
    //   block [0x83038B68..0x83038BAC)
	// 83038B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038B74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038B78: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038B80: 396BCFC4  addi r11, r11, -0x303c
	ctx.r[11].s64 = ctx.r[11].s64 + -12348;
	// 83038B84: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83038B88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038B8C: 41820008  beq 0x83038b94
	if ctx.cr[0].eq {
	pc = 0x83038B94; continue 'dispatch;
	}
	// 83038B90: 4B2876D9  bl 0x822c0268
	ctx.lr = 0x83038B94;
	sub_822C0268(ctx, base);
	// 83038B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038BB0 size=8
    let mut pc: u32 = 0x83038BB0;
    'dispatch: loop {
        match pc {
            0x83038BB0 => {
    //   block [0x83038BB0..0x83038BB8)
	// 83038BB0: A0630006  lhz r3, 6(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 83038BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038BB8 size=8
    let mut pc: u32 = 0x83038BB8;
    'dispatch: loop {
        match pc {
            0x83038BB8 => {
    //   block [0x83038BB8..0x83038BC0)
	// 83038BB8: B0830006  sth r4, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 83038BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038BC0 size=44
    let mut pc: u32 = 0x83038BC0;
    'dispatch: loop {
        match pc {
            0x83038BC0 => {
    //   block [0x83038BC0..0x83038BEC)
	// 83038BC0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038BC4: B0830006  sth r4, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 83038BC8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 83038BCC: 394BCFF8  addi r10, r11, -0x3008
	ctx.r[10].s64 = ctx.r[11].s64 + -12296;
	// 83038BD0: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83038BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83038BD8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83038BDC: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 83038BE0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83038BE4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83038BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038BF0 size=44
    let mut pc: u32 = 0x83038BF0;
    'dispatch: loop {
        match pc {
            0x83038BF0 => {
    //   block [0x83038BF0..0x83038C1C)
	// 83038BF0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038BF4: B0830006  sth r4, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 83038BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83038BFC: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83038C00: 396BCFF8  addi r11, r11, -0x3008
	ctx.r[11].s64 = ctx.r[11].s64 + -12296;
	// 83038C04: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 83038C08: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83038C0C: 99430004  stb r10, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 83038C10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038C14: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83038C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038C20 size=8
    let mut pc: u32 = 0x83038C20;
    'dispatch: loop {
        match pc {
            0x83038C20 => {
    //   block [0x83038C20..0x83038C28)
	// 83038C20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83038C24: 8215D038  lwz r16, -0x2fc8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12232 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038C28 size=124
    let mut pc: u32 = 0x83038C28;
    'dispatch: loop {
        match pc {
            0x83038C28 => {
    //   block [0x83038C28..0x83038CA4)
	// 83038C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038C38: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83038C3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038C40: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038C44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83038C48: 396BCFF8  addi r11, r11, -0x3008
	ctx.r[11].s64 = ctx.r[11].s64 + -12296;
	// 83038C4C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83038C50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038C54: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038C58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038C5C: 41820024  beq 0x83038c80
	if ctx.cr[0].eq {
	pc = 0x83038C80; continue 'dispatch;
	}
	// 83038C60: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83038C64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038C68: 41820018  beq 0x83038c80
	if ctx.cr[0].eq {
	pc = 0x83038C80; continue 'dispatch;
	}
	// 83038C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83038C74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83038C7C: 4E800421  bctrl
	ctx.lr = 0x83038C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83038C80: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038C84: 396BCFC4  addi r11, r11, -0x303c
	ctx.r[11].s64 = ctx.r[11].s64 + -12348;
	// 83038C88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038C8C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83038C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038C98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038CA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038CA4 size=40
    let mut pc: u32 = 0x83038CA4;
    'dispatch: loop {
        match pc {
            0x83038CA4 => {
    //   block [0x83038CA4..0x83038CCC)
	// 83038CA4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83038CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038CB4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83038CB8: 4BFFFEA1  bl 0x83038b58
	ctx.lr = 0x83038CBC;
	sub_83038B58(ctx, base);
	// 83038CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038CD0 size=100
    let mut pc: u32 = 0x83038CD0;
    'dispatch: loop {
        match pc {
            0x83038CD0 => {
    //   block [0x83038CD0..0x83038D34)
	// 83038CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038CD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038CDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038CE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038CE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038CEC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038CF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038CF4: 41820024  beq 0x83038d18
	if ctx.cr[0].eq {
	pc = 0x83038D18; continue 'dispatch;
	}
	// 83038CF8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83038CFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038D00: 41820018  beq 0x83038d18
	if ctx.cr[0].eq {
	pc = 0x83038D18; continue 'dispatch;
	}
	// 83038D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038D08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83038D0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83038D14: 4E800421  bctrl
	ctx.lr = 0x83038D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83038D18: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83038D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038D28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038D38 size=76
    let mut pc: u32 = 0x83038D38;
    'dispatch: loop {
        match pc {
            0x83038D38 => {
    //   block [0x83038D38..0x83038D84)
	// 83038D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038D54: 4BFFFED5  bl 0x83038c28
	ctx.lr = 0x83038D58;
	sub_83038C28(ctx, base);
	// 83038D58: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83038D5C: 4182000C  beq 0x83038d68
	if ctx.cr[0].eq {
	pc = 0x83038D68; continue 'dispatch;
	}
	// 83038D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038D64: 4B287505  bl 0x822c0268
	ctx.lr = 0x83038D68;
	sub_822C0268(ctx, base);
	// 83038D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038D6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038D78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038D7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038D88 size=80
    let mut pc: u32 = 0x83038D88;
    'dispatch: loop {
        match pc {
            0x83038D88 => {
    //   block [0x83038D88..0x83038DD8)
	// 83038D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038D9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038DA4: 7FC40734  extsh r4, r30
	ctx.r[4].s64 = ctx.r[30].s16 as i64;
	// 83038DA8: 4BFC1129  bl 0x82ff9ed0
	ctx.lr = 0x83038DAC;
	sub_82FF9ED0(ctx, base);
	// 83038DAC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038DB4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83038DB8: 396BD068  addi r11, r11, -0x2f98
	ctx.r[11].s64 = ctx.r[11].s64 + -12184;
	// 83038DBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038DC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038DCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038DD8 size=80
    let mut pc: u32 = 0x83038DD8;
    'dispatch: loop {
        match pc {
            0x83038DD8 => {
    //   block [0x83038DD8..0x83038E28)
	// 83038DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038DF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038DF4: 4BFC1175  bl 0x82ff9f68
	ctx.lr = 0x83038DF8;
	sub_82FF9F68(ctx, base);
	// 83038DF8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038E00: 396BD068  addi r11, r11, -0x2f98
	ctx.r[11].s64 = ctx.r[11].s64 + -12184;
	// 83038E04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038E08: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83038E0C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83038E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038E1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038E28 size=16
    let mut pc: u32 = 0x83038E28;
    'dispatch: loop {
        match pc {
            0x83038E28 => {
    //   block [0x83038E28..0x83038E38)
	// 83038E28: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038E2C: 396BD068  addi r11, r11, -0x2f98
	ctx.r[11].s64 = ctx.r[11].s64 + -12184;
	// 83038E30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038E34: 4BFC0FDC  b 0x82ff9e10
	sub_82FF9E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038E38 size=88
    let mut pc: u32 = 0x83038E38;
    'dispatch: loop {
        match pc {
            0x83038E38 => {
    //   block [0x83038E38..0x83038E90)
	// 83038E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83038E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83038E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038E4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83038E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83038E54: 396BD068  addi r11, r11, -0x2f98
	ctx.r[11].s64 = ctx.r[11].s64 + -12184;
	// 83038E58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83038E5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038E60: 4BFC0FB1  bl 0x82ff9e10
	ctx.lr = 0x83038E64;
	sub_82FF9E10(ctx, base);
	// 83038E64: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83038E68: 4182000C  beq 0x83038e74
	if ctx.cr[0].eq {
	pc = 0x83038E74; continue 'dispatch;
	}
	// 83038E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038E70: 4B2873F9  bl 0x822c0268
	ctx.lr = 0x83038E74;
	sub_822C0268(ctx, base);
	// 83038E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83038E78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83038E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038E84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83038E88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83038E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038E90 size=8
    let mut pc: u32 = 0x83038E90;
    'dispatch: loop {
        match pc {
            0x83038E90 => {
    //   block [0x83038E90..0x83038E98)
	// 83038E90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83038E94: 8215D078  lwz r16, -0x2f88(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038E98 size=136
    let mut pc: u32 = 0x83038E98;
    'dispatch: loop {
        match pc {
            0x83038E98 => {
    //   block [0x83038E98..0x83038F20)
	// 83038E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038E9C: 4816F2D1  bl 0x831a816c
	ctx.lr = 0x83038EA0;
	sub_831A8130(ctx, base);
	// 83038EA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83038EA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038EA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83038EAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83038EB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83038EB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83038EB8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83038EBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83038EC0: 909E0004  stw r4, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83038EC4: 4BFAB9CD  bl 0x82fe4890
	ctx.lr = 0x83038EC8;
	sub_82FE4890(ctx, base);
	// 83038EC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038ECC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83038ED0: 40820038  bne 0x83038f08
	if !ctx.cr[0].eq {
	pc = 0x83038F08; continue 'dispatch;
	}
	// 83038ED4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83038ED8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038EDC: 4BFA8B15  bl 0x82fe19f0
	ctx.lr = 0x83038EE0;
	sub_82FE19F0(ctx, base);
	// 83038EE0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83038EE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038EE8: 41820014  beq 0x83038efc
	if ctx.cr[0].eq {
	pc = 0x83038EFC; continue 'dispatch;
	}
	// 83038EEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83038EF0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038EF4: 4BFC4695  bl 0x82ffd588
	ctx.lr = 0x83038EF8;
	sub_82FFD588(ctx, base);
	// 83038EF8: 48000008  b 0x83038f00
	pc = 0x83038F00; continue 'dispatch;
	// 83038EFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83038F00: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83038F04: 48000010  b 0x83038f14
	pc = 0x83038F14; continue 'dispatch;
	// 83038F08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83038F0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83038F10: 4BFC4859  bl 0x82ffd768
	ctx.lr = 0x83038F14;
	sub_82FFD768(ctx, base);
	// 83038F14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83038F18: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83038F1C: 4816F2A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038F20 size=48
    let mut pc: u32 = 0x83038F20;
    'dispatch: loop {
        match pc {
            0x83038F20 => {
    //   block [0x83038F20..0x83038F50)
	// 83038F20: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83038F24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038F28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83038F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038F30: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83038F34: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038F38: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83038F3C: 4808EBA5  bl 0x830c7ae0
	ctx.lr = 0x83038F40;
	sub_830C7AE0(ctx, base);
	// 83038F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83038F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83038F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83038F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83038F50 size=8
    let mut pc: u32 = 0x83038F50;
    'dispatch: loop {
        match pc {
            0x83038F50 => {
    //   block [0x83038F50..0x83038F58)
	// 83038F50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83038F54: 8215D0C0  lwz r16, -0x2f40(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-12096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83038F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83038F58 size=184
    let mut pc: u32 = 0x83038F58;
    'dispatch: loop {
        match pc {
            0x83038F58 => {
    //   block [0x83038F58..0x83039010)
	// 83038F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83038F5C: 4816F20D  bl 0x831a8168
	ctx.lr = 0x83038F60;
	sub_831A8130(ctx, base);
	// 83038F60: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83038F64: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83038F68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83038F6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83038F70: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83038F74: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83038F78: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83038F7C: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83038F80: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038F84: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83038F88: 4BFAB909  bl 0x82fe4890
	ctx.lr = 0x83038F8C;
	sub_82FE4890(ctx, base);
	// 83038F8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038F90: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83038F94: 4082004C  bne 0x83038fe0
	if !ctx.cr[0].eq {
	pc = 0x83038FE0; continue 'dispatch;
	}
	// 83038F98: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83038F9C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038FA0: 4BFA8A51  bl 0x82fe19f0
	ctx.lr = 0x83038FA4;
	sub_82FE19F0(ctx, base);
	// 83038FA4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83038FA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83038FAC: 41820028  beq 0x83038fd4
	if ctx.cr[0].eq {
	pc = 0x83038FD4; continue 'dispatch;
	}
	// 83038FB0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FB4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038FB8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FBC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83038FC0: 7F8A4B2E  sthx r28, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[28].u16) };
	// 83038FC4: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FC8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038FCC: 4BFC45BD  bl 0x82ffd588
	ctx.lr = 0x83038FD0;
	sub_82FFD588(ctx, base);
	// 83038FD0: 48000008  b 0x83038fd8
	pc = 0x83038FD8; continue 'dispatch;
	// 83038FD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83038FD8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83038FDC: 48000028  b 0x83039004
	pc = 0x83039004; continue 'dispatch;
	// 83038FE0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83038FE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83038FEC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FF0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83038FF4: 7F8A4B2E  sthx r28, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[28].u16) };
	// 83038FF8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83038FFC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039000: 4BFC4769  bl 0x82ffd768
	ctx.lr = 0x83039004;
	sub_82FFD768(ctx, base);
	// 83039004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039008: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8303900C: 4816F1AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039010 size=48
    let mut pc: u32 = 0x83039010;
    'dispatch: loop {
        match pc {
            0x83039010 => {
    //   block [0x83039010..0x83039040)
	// 83039010: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83039014: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039018: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303901C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039020: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83039024: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039028: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303902C: 4808EAB5  bl 0x830c7ae0
	ctx.lr = 0x83039030;
	sub_830C7AE0(ctx, base);
	// 83039030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83039034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83039038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303903C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039040 size=144
    let mut pc: u32 = 0x83039040;
    'dispatch: loop {
        match pc {
            0x83039040 => {
    //   block [0x83039040..0x830390D0)
	// 83039040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83039048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303904C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83039050: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83039054: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039058: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303905C: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83039060: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83039064: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039068: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8303906C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83039070: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039074: 41820040  beq 0x830390b4
	if ctx.cr[0].eq {
	pc = 0x830390B4; continue 'dispatch;
	}
	// 83039078: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303907C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039080: 4182000C  beq 0x8303908c
	if ctx.cr[0].eq {
	pc = 0x8303908C; continue 'dispatch;
	}
	// 83039084: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039088: 4800000C  b 0x83039094
	pc = 0x83039094; continue 'dispatch;
	// 8303908C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039090: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039098: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303909C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830390A0: 4BFC0E31  bl 0x82ff9ed0
	ctx.lr = 0x830390A4;
	sub_82FF9ED0(ctx, base);
	// 830390A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830390A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830390AC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830390B0: 48177B79  bl 0x831b0c28
	ctx.lr = 0x830390B4;
	sub_831B0C28(ctx, base);
	// 830390B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830390B8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830390BC: 4BFC45ED  bl 0x82ffd6a8
	ctx.lr = 0x830390C0;
	sub_82FFD6A8(ctx, base);
	// 830390C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830390C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830390C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830390CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830390D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830390D0 size=32
    let mut pc: u32 = 0x830390D0;
    'dispatch: loop {
        match pc {
            0x830390D0 => {
    //   block [0x830390D0..0x830390F0)
	// 830390D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830390D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830390D8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830390DC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830390E0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830390E4: 7D49432E  sthx r10, r9, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u16) };
	// 830390E8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830390EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830390F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830390F0 size=12
    let mut pc: u32 = 0x830390F0;
    'dispatch: loop {
        match pc {
            0x830390F0 => {
    //   block [0x830390F0..0x830390FC)
	// 830390F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830390F4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830390F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039100 size=328
    let mut pc: u32 = 0x83039100;
    'dispatch: loop {
        match pc {
            0x83039100 => {
    //   block [0x83039100..0x83039248)
	// 83039100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039104: 4816F05D  bl 0x831a8160
	ctx.lr = 0x83039108;
	sub_831A8130(ctx, base);
	// 83039108: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8303910C: 9421E010  stwu r1, -0x1ff0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039110: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83039114: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83039118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303911C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83039120: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039124: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039128: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8303912C: 419900E0  bgt cr6, 0x8303920c
	if ctx.cr[6].gt {
	pc = 0x8303920C; continue 'dispatch;
	}
	// 83039130: 2B1D0F9F  cmplwi cr6, r29, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3999 as u32, &mut ctx.xer);
	// 83039134: 4198003C  blt cr6, 0x83039170
	if ctx.cr[6].lt {
	pc = 0x83039170; continue 'dispatch;
	}
	// 83039138: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303913C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039140: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039148: 4E800421  bctrl
	ctx.lr = 0x8303914C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303914C: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039150: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 83039154: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83039158: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303915C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039164: 4E800421  bctrl
	ctx.lr = 0x83039168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303916C: 48000008  b 0x83039174
	pc = 0x83039174; continue 'dispatch;
	// 83039170: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 83039174: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039178: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303917C: 574A083C  slwi r10, r26, 1
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83039180: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83039184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83039188: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303918C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039190: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83039194: 7F89432E  sthx r28, r9, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[28].u16) };
	// 83039198: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303919C: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830391A0: 4BF98A09  bl 0x82fd1ba8
	ctx.lr = 0x830391A4;
	sub_82FD1BA8(ctx, base);
	// 830391A4: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830391A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830391AC: 7F8BFB2E  sthx r28, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[28].u16) };
	// 830391B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830391B4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830391B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830391BC: 4E800421  bctrl
	ctx.lr = 0x830391C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830391C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830391C4: 4BFA8815  bl 0x82fe19d8
	ctx.lr = 0x830391C8;
	sub_82FE19D8(ctx, base);
	// 830391C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830391CC: 2B1D0F9F  cmplwi cr6, r29, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3999 as u32, &mut ctx.xer);
	// 830391D0: 41980030  blt cr6, 0x83039200
	if ctx.cr[6].lt {
	pc = 0x83039200; continue 'dispatch;
	}
	// 830391D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830391D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830391DC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830391E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830391E4: 4E800421  bctrl
	ctx.lr = 0x830391E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830391E8: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830391EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830391F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830391F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830391F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830391FC: 4E800421  bctrl
	ctx.lr = 0x83039200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039200: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039204: 38211FF0  addi r1, r1, 0x1ff0
	ctx.r[1].s64 = ctx.r[1].s64 + 8176;
	// 83039208: 4816EFA8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8303920C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039210: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039214: 4182000C  beq 0x83039220
	if ctx.cr[0].eq {
	pc = 0x83039220; continue 'dispatch;
	}
	// 83039218: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303921C: 4800000C  b 0x83039228
	pc = 0x83039228; continue 'dispatch;
	// 83039220: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039224: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039228: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303922C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83039230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039234: 4BFC0C9D  bl 0x82ff9ed0
	ctx.lr = 0x83039238;
	sub_82FF9ED0(ctx, base);
	// 83039238: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303923C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039240: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039244: 481779E5  bl 0x831b0c28
	ctx.lr = 0x83039248;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83039248 size=12
    let mut pc: u32 = 0x83039248;
    'dispatch: loop {
        match pc {
            0x83039248 => {
    //   block [0x83039248..0x83039254)
	// 83039248: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303924C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039250: 4BFAC5E0  b 0x82fe5830
	sub_82FE5830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039258 size=264
    let mut pc: u32 = 0x83039258;
    'dispatch: loop {
        match pc {
            0x83039258 => {
    //   block [0x83039258..0x83039360)
	// 83039258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303925C: 4816EF0D  bl 0x831a8168
	ctx.lr = 0x83039260;
	sub_831A8130(ctx, base);
	// 83039260: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039264: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83039268: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303926C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83039270: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039274: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83039278: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8303927C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039280: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83039284: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83039288: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303928C: 41820040  beq 0x830392cc
	if ctx.cr[0].eq {
	pc = 0x830392CC; continue 'dispatch;
	}
	// 83039290: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039294: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039298: 4182000C  beq 0x830392a4
	if ctx.cr[0].eq {
	pc = 0x830392A4; continue 'dispatch;
	}
	// 8303929C: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830392A0: 4800000C  b 0x830392ac
	pc = 0x830392AC; continue 'dispatch;
	// 830392A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830392A8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830392AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830392B0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830392B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830392B8: 4BFC0C19  bl 0x82ff9ed0
	ctx.lr = 0x830392BC;
	sub_82FF9ED0(ctx, base);
	// 830392BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830392C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830392C4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830392C8: 48177961  bl 0x831b0c28
	ctx.lr = 0x830392CC;
	sub_831B0C28(ctx, base);
	// 830392CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830392D0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830392D4: 4BFC4495  bl 0x82ffd768
	ctx.lr = 0x830392D8;
	sub_82FFD768(ctx, base);
	// 830392D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830392DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830392E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830392E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830392E8: 4E800421  bctrl
	ctx.lr = 0x830392EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830392EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830392F0: 41820068  beq 0x83039358
	if ctx.cr[0].eq {
	pc = 0x83039358; continue 'dispatch;
	}
	// 830392F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830392F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830392FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039304: 4E800421  bctrl
	ctx.lr = 0x83039308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303930C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83039310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039314: 4E800421  bctrl
	ctx.lr = 0x83039318;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039318: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303931C: 4182003C  beq 0x83039358
	if ctx.cr[0].eq {
	pc = 0x83039358; continue 'dispatch;
	}
	// 83039320: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039324: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039328: 41820030  beq 0x83039358
	if ctx.cr[0].eq {
	pc = 0x83039358; continue 'dispatch;
	}
	// 8303932C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83039330: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83039334: 419A0024  beq cr6, 0x83039358
	if ctx.cr[6].eq {
	pc = 0x83039358; continue 'dispatch;
	}
	// 83039338: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303933C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039340: 4BFF3531  bl 0x8302c870
	ctx.lr = 0x83039344;
	sub_8302C870(ctx, base);
	// 83039344: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039348: 4BFC7619  bl 0x83000960
	ctx.lr = 0x8303934C;
	sub_83000960(ctx, base);
	// 8303934C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83039350: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83039354: 4198FFE4  blt cr6, 0x83039338
	if ctx.cr[6].lt {
	pc = 0x83039338; continue 'dispatch;
	}
	// 83039358: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8303935C: 4816EE5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039360 size=568
    let mut pc: u32 = 0x83039360;
    'dispatch: loop {
        match pc {
            0x83039360 => {
    //   block [0x83039360..0x83039598)
	// 83039360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039364: 4816EDF5  bl 0x831a8158
	ctx.lr = 0x83039368;
	sub_831A8130(ctx, base);
	// 83039368: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8303936C: 9421E000  stwu r1, -0x2000(r1)
	ea = ctx.r[1].u32.wrapping_add(-8192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039370: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83039374: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83039378: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303937C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83039380: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83039384: A1580008  lhz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039388: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8303938C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83039390: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039394: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83039398: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8303939C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830393A0: 41820040  beq 0x830393e0
	if ctx.cr[0].eq {
	pc = 0x830393E0; continue 'dispatch;
	}
	// 830393A4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830393A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830393AC: 4182000C  beq 0x830393b8
	if ctx.cr[0].eq {
	pc = 0x830393B8; continue 'dispatch;
	}
	// 830393B0: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830393B4: 4800000C  b 0x830393c0
	pc = 0x830393C0; continue 'dispatch;
	// 830393B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830393BC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830393C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830393C4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830393C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830393CC: 4BFC0B05  bl 0x82ff9ed0
	ctx.lr = 0x830393D0;
	sub_82FF9ED0(ctx, base);
	// 830393D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830393D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830393D8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830393DC: 4817784D  bl 0x831b0c28
	ctx.lr = 0x830393E0;
	sub_831B0C28(ctx, base);
	// 830393E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830393E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830393E8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830393EC: 41990170  bgt cr6, 0x8303955c
	if ctx.cr[6].gt {
	pc = 0x8303955C; continue 'dispatch;
	}
	// 830393F0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830393F4: 40990008  ble cr6, 0x830393fc
	if !ctx.cr[6].gt {
	pc = 0x830393FC; continue 'dispatch;
	}
	// 830393F8: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 830393FC: 7D5EDA14  add r10, r30, r27
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 83039400: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83039404: 41980008  blt cr6, 0x8303940c
	if ctx.cr[6].lt {
	pc = 0x8303940C; continue 'dispatch;
	}
	// 83039408: 7F7E5850  subf r27, r30, r11
	ctx.r[27].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8303940C: 7F9B5850  subf r28, r27, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83039410: 3F208339  lis r25, -0x7cc7
	ctx.r[25].s64 = -2093416448;
	// 83039414: 2B1C0F9F  cmplwi cr6, r28, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3999 as u32, &mut ctx.xer);
	// 83039418: 41980028  blt cr6, 0x83039440
	if ctx.cr[6].lt {
	pc = 0x83039440; continue 'dispatch;
	}
	// 8303941C: 8079B7E8  lwz r3, -0x4818(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039420: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 83039424: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83039428: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303942C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039434: 4E800421  bctrl
	ctx.lr = 0x83039438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303943C: 48000008  b 0x83039444
	pc = 0x83039444; continue 'dispatch;
	// 83039440: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 83039444: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039448: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8303944C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83039450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83039454: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039458: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303945C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83039460: 7F4A4B2E  sthx r26, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[26].u16) };
	// 83039464: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039468: 4BF98741  bl 0x82fd1ba8
	ctx.lr = 0x8303946C;
	sub_82FD1BA8(ctx, base);
	// 8303946C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039470: 57C9083C  slwi r9, r30, 1
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83039474: 7D5EDA14  add r10, r30, r27
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 83039478: 7C69FA14  add r3, r9, r31
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 8303947C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83039480: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039484: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039488: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8303948C: 7F49432E  sthx r26, r9, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[26].u16) };
	// 83039490: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039494: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83039498: 4BF986D1  bl 0x82fd1b68
	ctx.lr = 0x8303949C;
	sub_82FD1B68(ctx, base);
	// 8303949C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830394A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830394A4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830394A8: 4BFC42C1  bl 0x82ffd768
	ctx.lr = 0x830394AC;
	sub_82FFD768(ctx, base);
	// 830394AC: 2B1C0F9F  cmplwi cr6, r28, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3999 as u32, &mut ctx.xer);
	// 830394B0: 4198001C  blt cr6, 0x830394cc
	if ctx.cr[6].lt {
	pc = 0x830394CC; continue 'dispatch;
	}
	// 830394B4: 8079B7E8  lwz r3, -0x4818(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830394B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830394BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830394C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830394C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830394C8: 4E800421  bctrl
	ctx.lr = 0x830394CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830394CC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830394D0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830394D4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830394D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830394DC: 4E800421  bctrl
	ctx.lr = 0x830394E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830394E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830394E4: 41820070  beq 0x83039554
	if ctx.cr[0].eq {
	pc = 0x83039554; continue 'dispatch;
	}
	// 830394E8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830394EC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830394F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830394F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830394F8: 4E800421  bctrl
	ctx.lr = 0x830394FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830394FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039500: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83039504: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039508: 4E800421  bctrl
	ctx.lr = 0x8303950C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303950C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83039510: 41820044  beq 0x83039554
	if ctx.cr[0].eq {
	pc = 0x83039554; continue 'dispatch;
	}
	// 83039514: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039518: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303951C: 41820038  beq 0x83039554
	if ctx.cr[0].eq {
	pc = 0x83039554; continue 'dispatch;
	}
	// 83039520: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 83039524: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83039528: 419A002C  beq cr6, 0x83039554
	if ctx.cr[6].eq {
	pc = 0x83039554; continue 'dispatch;
	}
	// 8303952C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039530: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039534: 4BFF333D  bl 0x8302c870
	ctx.lr = 0x83039538;
	sub_8302C870(ctx, base);
	// 83039538: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303953C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83039540: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83039544: 4BFC74D5  bl 0x83000a18
	ctx.lr = 0x83039548;
	sub_83000A18(ctx, base);
	// 83039548: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8303954C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83039550: 4198FFDC  blt cr6, 0x8303952c
	if ctx.cr[6].lt {
	pc = 0x8303952C; continue 'dispatch;
	}
	// 83039554: 38212000  addi r1, r1, 0x2000
	ctx.r[1].s64 = ctx.r[1].s64 + 8192;
	// 83039558: 4816EC50  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8303955C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039560: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039564: 4182000C  beq 0x83039570
	if ctx.cr[0].eq {
	pc = 0x83039570; continue 'dispatch;
	}
	// 83039568: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303956C: 4800000C  b 0x83039578
	pc = 0x83039578; continue 'dispatch;
	// 83039570: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039574: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039578: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303957C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83039580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039584: 4BFC094D  bl 0x82ff9ed0
	ctx.lr = 0x83039588;
	sub_82FF9ED0(ctx, base);
	// 83039588: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303958C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039590: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039594: 48177695  bl 0x831b0c28
	ctx.lr = 0x83039598;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039598 size=620
    let mut pc: u32 = 0x83039598;
    'dispatch: loop {
        match pc {
            0x83039598 => {
    //   block [0x83039598..0x83039804)
	// 83039598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303959C: 4816EBB5  bl 0x831a8150
	ctx.lr = 0x830395A0;
	sub_831A8130(ctx, base);
	// 830395A0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 830395A4: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 830395A8: 9421DFF0  stwu r1, -0x2010(r1)
	ea = ctx.r[1].u32.wrapping_add(-8208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830395AC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 830395B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830395B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830395B8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830395BC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830395C0: A1560008  lhz r10, 8(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 830395C4: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 830395C8: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830395CC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830395D0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830395D4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830395D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830395DC: 41820040  beq 0x8303961c
	if ctx.cr[0].eq {
	pc = 0x8303961C; continue 'dispatch;
	}
	// 830395E0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830395E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830395E8: 4182000C  beq 0x830395f4
	if ctx.cr[0].eq {
	pc = 0x830395F4; continue 'dispatch;
	}
	// 830395EC: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830395F0: 4800000C  b 0x830395fc
	pc = 0x830395FC; continue 'dispatch;
	// 830395F4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830395F8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830395FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039600: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83039604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039608: 4BFC08C9  bl 0x82ff9ed0
	ctx.lr = 0x8303960C;
	sub_82FF9ED0(ctx, base);
	// 8303960C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039610: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039614: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039618: 48177611  bl 0x831b0c28
	ctx.lr = 0x8303961C;
	sub_831B0C28(ctx, base);
	// 8303961C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039620: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039624: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83039628: 419901A0  bgt cr6, 0x830397c8
	if ctx.cr[6].gt {
	pc = 0x830397C8; continue 'dispatch;
	}
	// 8303962C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83039630: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83039634: 419A0034  beq cr6, 0x83039668
	if ctx.cr[6].eq {
	pc = 0x83039668; continue 'dispatch;
	}
	// 83039638: A1790000  lhz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303963C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039640: 41820028  beq 0x83039668
	if ctx.cr[0].eq {
	pc = 0x83039668; continue 'dispatch;
	}
	// 83039644: 39790002  addi r11, r25, 2
	ctx.r[11].s64 = ctx.r[25].s64 + 2;
	// 83039648: 48000008  b 0x83039650
	pc = 0x83039650; continue 'dispatch;
	// 8303964C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83039650: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039654: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039658: 4082FFF4  bne 0x8303964c
	if !ctx.cr[0].eq {
	pc = 0x8303964C; continue 'dispatch;
	}
	// 8303965C: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 83039660: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83039664: 48000008  b 0x8303966c
	pc = 0x8303966C; continue 'dispatch;
	// 83039668: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 8303966C: 7F9B5214  add r28, r27, r10
	ctx.r[28].u64 = ctx.r[27].u64 + ctx.r[10].u64;
	// 83039670: 3EE08339  lis r23, -0x7cc7
	ctx.r[23].s64 = -2093416448;
	// 83039674: 2B1C0F9F  cmplwi cr6, r28, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3999 as u32, &mut ctx.xer);
	// 83039678: 41980028  blt cr6, 0x830396a0
	if ctx.cr[6].lt {
	pc = 0x830396A0; continue 'dispatch;
	}
	// 8303967C: 8077B7E8  lwz r3, -0x4818(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039680: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 83039684: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83039688: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303968C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039694: 4E800421  bctrl
	ctx.lr = 0x83039698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303969C: 48000008  b 0x830396a4
	pc = 0x830396A4; continue 'dispatch;
	// 830396A0: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 830396A4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830396A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830396AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830396B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830396B4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830396B8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830396BC: 7F0A4B2E  sthx r24, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[24].u16) };
	// 830396C0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830396C4: 4BF984E5  bl 0x82fd1ba8
	ctx.lr = 0x830396C8;
	sub_82FD1BA8(ctx, base);
	// 830396C8: 575E083C  slwi r30, r26, 1
	ctx.r[30].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 830396CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830396D0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830396D4: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 830396D8: 4BF984D1  bl 0x82fd1ba8
	ctx.lr = 0x830396DC;
	sub_82FD1BA8(ctx, base);
	// 830396DC: 7D5BD214  add r10, r27, r26
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 830396E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830396E4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830396E8: 7C6AFA14  add r3, r10, r31
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 830396EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830396F0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830396F4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830396F8: 7F0A4B2E  sthx r24, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[24].u16) };
	// 830396FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039700: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83039704: 4BF98465  bl 0x82fd1b68
	ctx.lr = 0x83039708;
	sub_82FD1B68(ctx, base);
	// 83039708: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303970C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039710: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039714: 4BFC4055  bl 0x82ffd768
	ctx.lr = 0x83039718;
	sub_82FFD768(ctx, base);
	// 83039718: 2B1C0F9F  cmplwi cr6, r28, 0xf9f
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3999 as u32, &mut ctx.xer);
	// 8303971C: 4198001C  blt cr6, 0x83039738
	if ctx.cr[6].lt {
	pc = 0x83039738; continue 'dispatch;
	}
	// 83039720: 8077B7E8  lwz r3, -0x4818(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039724: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303972C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039734: 4E800421  bctrl
	ctx.lr = 0x83039738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039738: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303973C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83039740: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039748: 4E800421  bctrl
	ctx.lr = 0x8303974C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303974C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039750: 41820070  beq 0x830397c0
	if ctx.cr[0].eq {
	pc = 0x830397C0; continue 'dispatch;
	}
	// 83039754: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039758: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8303975C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039764: 4E800421  bctrl
	ctx.lr = 0x83039768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303976C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83039770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039774: 4E800421  bctrl
	ctx.lr = 0x83039778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039778: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303977C: 41820044  beq 0x830397c0
	if ctx.cr[0].eq {
	pc = 0x830397C0; continue 'dispatch;
	}
	// 83039780: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039784: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039788: 41820038  beq 0x830397c0
	if ctx.cr[0].eq {
	pc = 0x830397C0; continue 'dispatch;
	}
	// 8303978C: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 83039790: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83039794: 419A002C  beq cr6, 0x830397c0
	if ctx.cr[6].eq {
	pc = 0x830397C0; continue 'dispatch;
	}
	// 83039798: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303979C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830397A0: 4BFF30D1  bl 0x8302c870
	ctx.lr = 0x830397A4;
	sub_8302C870(ctx, base);
	// 830397A4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830397A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830397AC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830397B0: 4BFC7371  bl 0x83000b20
	ctx.lr = 0x830397B4;
	sub_83000B20(ctx, base);
	// 830397B4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830397B8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830397BC: 4198FFDC  blt cr6, 0x83039798
	if ctx.cr[6].lt {
	pc = 0x83039798; continue 'dispatch;
	}
	// 830397C0: 38212010  addi r1, r1, 0x2010
	ctx.r[1].s64 = ctx.r[1].s64 + 8208;
	// 830397C4: 4816E9DC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 830397C8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830397CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830397D0: 4182000C  beq 0x830397dc
	if ctx.cr[0].eq {
	pc = 0x830397DC; continue 'dispatch;
	}
	// 830397D4: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830397D8: 4800000C  b 0x830397e4
	pc = 0x830397E4; continue 'dispatch;
	// 830397DC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830397E0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830397E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830397E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830397EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830397F0: 4BFC06E1  bl 0x82ff9ed0
	ctx.lr = 0x830397F4;
	sub_82FF9ED0(ctx, base);
	// 830397F4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830397F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830397FC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039800: 48177429  bl 0x831b0c28
	ctx.lr = 0x83039804;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039808 size=168
    let mut pc: u32 = 0x83039808;
    'dispatch: loop {
        match pc {
            0x83039808 => {
    //   block [0x83039808..0x830398B0)
	// 83039808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303980C: 4816E95D  bl 0x831a8168
	ctx.lr = 0x83039810;
	sub_831A8130(ctx, base);
	// 83039810: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039814: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83039818: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303981C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83039820: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83039824: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83039828: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303982C: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83039830: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83039834: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039838: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8303983C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83039840: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039844: 41820040  beq 0x83039884
	if ctx.cr[0].eq {
	pc = 0x83039884; continue 'dispatch;
	}
	// 83039848: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303984C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039850: 4182000C  beq 0x8303985c
	if ctx.cr[0].eq {
	pc = 0x8303985C; continue 'dispatch;
	}
	// 83039854: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039858: 4800000C  b 0x83039864
	pc = 0x83039864; continue 'dispatch;
	// 8303985C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039860: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039868: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303986C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83039870: 4BFC0661  bl 0x82ff9ed0
	ctx.lr = 0x83039874;
	sub_82FF9ED0(ctx, base);
	// 83039874: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303987C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039880: 481773A9  bl 0x831b0c28
	ctx.lr = 0x83039884;
	sub_831B0C28(ctx, base);
	// 83039884: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83039888: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303988C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039890: 4BFFFAD1  bl 0x83039360
	ctx.lr = 0x83039894;
	sub_83039360(ctx, base);
	// 83039894: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83039898: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303989C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830398A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830398A4: 4BFFFCF5  bl 0x83039598
	ctx.lr = 0x830398A8;
	sub_83039598(ctx, base);
	// 830398A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830398AC: 4816E90C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830398B0 size=4
    let mut pc: u32 = 0x830398B0;
    'dispatch: loop {
        match pc {
            0x830398B0 => {
    //   block [0x830398B0..0x830398B4)
	// 830398B0: 4BFFF9A8  b 0x83039258
	sub_83039258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830398B8 size=8
    let mut pc: u32 = 0x830398B8;
    'dispatch: loop {
        match pc {
            0x830398B8 => {
    //   block [0x830398B8..0x830398C0)
	// 830398B8: 9883000C  stb r4, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 830398BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830398C0 size=32
    let mut pc: u32 = 0x830398C0;
    'dispatch: loop {
        match pc {
            0x830398C0 => {
    //   block [0x830398C0..0x830398E0)
	// 830398C0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830398C4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 830398C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830398CC: 396BD100  addi r11, r11, -0x2f00
	ctx.r[11].s64 = ctx.r[11].s64 + -12032;
	// 830398D0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830398D4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830398D8: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 830398DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830398E0 size=8
    let mut pc: u32 = 0x830398E0;
    'dispatch: loop {
        match pc {
            0x830398E0 => {
    //   block [0x830398E0..0x830398E8)
	// 830398E0: 8863000C  lbz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830398E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830398E8 size=8
    let mut pc: u32 = 0x830398E8;
    'dispatch: loop {
        match pc {
            0x830398E8 => {
    //   block [0x830398E8..0x830398F0)
	// 830398E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830398EC: 8215D150  lwz r16, -0x2eb0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830398F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830398F0 size=336
    let mut pc: u32 = 0x830398F0;
    'dispatch: loop {
        match pc {
            0x830398F0 => {
    //   block [0x830398F0..0x83039A40)
	// 830398F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830398F4: 4816E869  bl 0x831a815c
	ctx.lr = 0x830398F8;
	sub_831A8130(ctx, base);
	// 830398F8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830398FC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039900: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83039904: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83039908: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303990C: 419A012C  beq cr6, 0x83039a38
	if ctx.cr[6].eq {
	pc = 0x83039A38; continue 'dispatch;
	}
	// 83039910: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039914: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039918: 41820120  beq 0x83039a38
	if ctx.cr[0].eq {
	pc = 0x83039A38; continue 'dispatch;
	}
	// 8303991C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039920: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039924: 41820010  beq 0x83039934
	if ctx.cr[0].eq {
	pc = 0x83039934; continue 'dispatch;
	}
	// 83039928: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8303992C: 4802EE15  bl 0x83068740
	ctx.lr = 0x83039930;
	sub_83068740(ctx, base);
	// 83039930: 48000058  b 0x83039988
	pc = 0x83039988; continue 'dispatch;
	// 83039934: 4811ACCD  bl 0x83154600
	ctx.lr = 0x83039938;
	sub_83154600(ctx, base);
	// 83039938: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303993C: 4182004C  beq 0x83039988
	if ctx.cr[0].eq {
	pc = 0x83039988; continue 'dispatch;
	}
	// 83039940: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039948: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303994C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039950: 4E800421  bctrl
	ctx.lr = 0x83039954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039954: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83039958: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8303995C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83039960: 4BFA8091  bl 0x82fe19f0
	ctx.lr = 0x83039964;
	sub_82FE19F0(ctx, base);
	// 83039964: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83039968: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303996C: 41820014  beq 0x83039980
	if ctx.cr[0].eq {
	pc = 0x83039980; continue 'dispatch;
	}
	// 83039970: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83039974: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83039978: 48045F79  bl 0x8307f8f0
	ctx.lr = 0x8303997C;
	sub_8307F8F0(ctx, base);
	// 8303997C: 48000008  b 0x83039984
	pc = 0x83039984; continue 'dispatch;
	// 83039980: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83039984: 907B0004  stw r3, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83039988: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303998C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83039990: 4811AC71  bl 0x83154600
	ctx.lr = 0x83039994;
	sub_83154600(ctx, base);
	// 83039994: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039998: 418200A0  beq 0x83039a38
	if ctx.cr[0].eq {
	pc = 0x83039A38; continue 'dispatch;
	}
	// 8303999C: 3F208214  lis r25, -0x7dec
	ctx.r[25].s64 = -2112618496;
	// 830399A0: 3F408214  lis r26, -0x7dec
	ctx.r[26].s64 = -2112618496;
	// 830399A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830399A8: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830399AC: 48045E15  bl 0x8307f7c0
	ctx.lr = 0x830399B0;
	sub_8307F7C0(ctx, base);
	// 830399B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830399B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830399B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830399BC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830399C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830399C4: 4E800421  bctrl
	ctx.lr = 0x830399C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830399C8: A15AA6AC  lhz r10, -0x5954(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(-22868 as u32) ) } as u64;
	// 830399CC: A13E0008  lhz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830399D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830399D4: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 830399D8: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 830399DC: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 830399E0: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 830399E4: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 830399E8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830399EC: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830399F0: 4182000C  beq 0x830399fc
	if ctx.cr[0].eq {
	pc = 0x830399FC; continue 'dispatch;
	}
	// 830399F4: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 830399F8: 48000008  b 0x83039a00
	pc = 0x83039A00; continue 'dispatch;
	// 830399FC: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83039A00: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83039A04: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039A08: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83039A0C: A159A6A4  lhz r10, -0x595c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83039A10: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039A14: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83039A18: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83039A1C: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039A20: 48045F01  bl 0x8307f920
	ctx.lr = 0x83039A24;
	sub_8307F920(ctx, base);
	// 83039A24: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039A28: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83039A2C: 4811ABD5  bl 0x83154600
	ctx.lr = 0x83039A30;
	sub_83154600(ctx, base);
	// 83039A30: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83039A34: 4198FF70  blt cr6, 0x830399a4
	if ctx.cr[6].lt {
	pc = 0x830399A4; continue 'dispatch;
	}
	// 83039A38: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83039A3C: 4816E770  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039A40 size=44
    let mut pc: u32 = 0x83039A40;
    'dispatch: loop {
        match pc {
            0x83039A40 => {
    //   block [0x83039A40..0x83039A6C)
	// 83039A40: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83039A44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039A48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83039A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039A50: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83039A54: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83039A58: 4808E089  bl 0x830c7ae0
	ctx.lr = 0x83039A5C;
	sub_830C7AE0(ctx, base);
	// 83039A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83039A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83039A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83039A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039A70 size=120
    let mut pc: u32 = 0x83039A70;
    'dispatch: loop {
        match pc {
            0x83039A70 => {
    //   block [0x83039A70..0x83039AE8)
	// 83039A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039A74: 4816E6F9  bl 0x831a816c
	ctx.lr = 0x83039A78;
	sub_831A8130(ctx, base);
	// 83039A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039A7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83039A80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83039A84: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83039A88: 4BFA57A1  bl 0x82fdf228
	ctx.lr = 0x83039A8C;
	sub_82FDF228(ctx, base);
	// 83039A8C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83039A90: 4BFA7F61  bl 0x82fe19f0
	ctx.lr = 0x83039A94;
	sub_82FE19F0(ctx, base);
	// 83039A94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039A98: 41820028  beq 0x83039ac0
	if ctx.cr[0].eq {
	pc = 0x83039AC0; continue 'dispatch;
	}
	// 83039A9C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83039AA0: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83039AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83039AA8: 396BD100  addi r11, r11, -0x2f00
	ctx.r[11].s64 = ctx.r[11].s64 + -12032;
	// 83039AAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83039AB0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83039AB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83039AB8: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 83039ABC: 48000008  b 0x83039ac4
	pc = 0x83039AC4; continue 'dispatch;
	// 83039AC0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83039AC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039AC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83039ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039AD0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83039AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039AD8: 4E800421  bctrl
	ctx.lr = 0x83039ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039ADC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039AE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83039AE4: 4816E6D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039AE8 size=108
    let mut pc: u32 = 0x83039AE8;
    'dispatch: loop {
        match pc {
            0x83039AE8 => {
    //   block [0x83039AE8..0x83039B54)
	// 83039AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039AEC: 4816E679  bl 0x831a8164
	ctx.lr = 0x83039AF0;
	sub_831A8130(ctx, base);
	// 83039AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039AF4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83039AF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83039AFC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83039B00: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83039B04: 41820048  beq 0x83039b4c
	if ctx.cr[0].eq {
	pc = 0x83039B4C; continue 'dispatch;
	}
	// 83039B08: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039B0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039B10: 4182003C  beq 0x83039b4c
	if ctx.cr[0].eq {
	pc = 0x83039B4C; continue 'dispatch;
	}
	// 83039B14: 4811AAED  bl 0x83154600
	ctx.lr = 0x83039B18;
	sub_83154600(ctx, base);
	// 83039B18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83039B1C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83039B20: 4081002C  ble 0x83039b4c
	if !ctx.cr[0].gt {
	pc = 0x83039B4C; continue 'dispatch;
	}
	// 83039B24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039B28: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039B2C: 48045C95  bl 0x8307f7c0
	ctx.lr = 0x83039B30;
	sub_8307F7C0(ctx, base);
	// 83039B30: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83039B34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83039B38: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83039B3C: 4BFA5855  bl 0x82fdf390
	ctx.lr = 0x83039B40;
	sub_82FDF390(ctx, base);
	// 83039B40: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83039B44: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83039B48: 4198FFDC  blt cr6, 0x83039b24
	if ctx.cr[6].lt {
	pc = 0x83039B24; continue 'dispatch;
	}
	// 83039B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83039B50: 4816E664  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83039B58 size=172
    let mut pc: u32 = 0x83039B58;
    'dispatch: loop {
        match pc {
            0x83039B58 => {
    //   block [0x83039B58..0x83039C04)
	// 83039B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039B5C: 4816E609  bl 0x831a8164
	ctx.lr = 0x83039B60;
	sub_831A8130(ctx, base);
	// 83039B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039B64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83039B68: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83039B6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83039B70: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039B74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039B78: 41820078  beq 0x83039bf0
	if ctx.cr[0].eq {
	pc = 0x83039BF0; continue 'dispatch;
	}
	// 83039B7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83039B80: 4811AA81  bl 0x83154600
	ctx.lr = 0x83039B84;
	sub_83154600(ctx, base);
	// 83039B84: 37C3FFFF  addic. r30, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83039B88: 41800068  blt 0x83039bf0
	if ctx.cr[0].lt {
	pc = 0x83039BF0; continue 'dispatch;
	}
	// 83039B8C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 83039B90: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039B94: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83039B98: 7FEB0194  addze r31, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[31].s64 = tmp.s64;
	// 83039B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83039BA0: 48045C21  bl 0x8307f7c0
	ctx.lr = 0x83039BA4;
	sub_8307F7C0(ctx, base);
	// 83039BA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039BA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039BB0: 4E800421  bctrl
	ctx.lr = 0x83039BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039BB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83039BB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83039BBC: 4BF97E0D  bl 0x82fd19c8
	ctx.lr = 0x83039BC0;
	sub_82FD19C8(ctx, base);
	// 83039BC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83039BC4: 41820038  beq 0x83039bfc
	if ctx.cr[0].eq {
	pc = 0x83039BFC; continue 'dispatch;
	}
	// 83039BC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83039BCC: 4098000C  bge cr6, 0x83039bd8
	if !ctx.cr[6].lt {
	pc = 0x83039BD8; continue 'dispatch;
	}
	// 83039BD0: 3BDFFFFF  addi r30, r31, -1
	ctx.r[30].s64 = ctx.r[31].s64 + -1;
	// 83039BD4: 48000008  b 0x83039bdc
	pc = 0x83039BDC; continue 'dispatch;
	// 83039BD8: 3BBF0001  addi r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 1;
	// 83039BDC: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83039BE0: 4099FFAC  ble cr6, 0x83039b8c
	if !ctx.cr[6].gt {
	pc = 0x83039B8C; continue 'dispatch;
	}
	// 83039BE4: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83039BE8: 40990008  ble cr6, 0x83039bf0
	if !ctx.cr[6].gt {
	pc = 0x83039BF0; continue 'dispatch;
	}
	// 83039BEC: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83039BF0: 207FFFFF  subfic r3, r31, -1
	ctx.xer.ca = ctx.r[31].u32 <= -1 as u32;
	ctx.r[3].s64 = (-1 as i64) - ctx.r[31].s64;
	// 83039BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83039BF8: 4816E5BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83039BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83039C00: 4BFFFFF4  b 0x83039bf4
	pc = 0x83039BF4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039C08 size=80
    let mut pc: u32 = 0x83039C08;
    'dispatch: loop {
        match pc {
            0x83039C08 => {
    //   block [0x83039C08..0x83039C58)
	// 83039C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83039C10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83039C14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83039C1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039C20: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83039C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039C28: 4E800421  bctrl
	ctx.lr = 0x83039C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039C2C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83039C30: 4080000C  bge 0x83039c3c
	if !ctx.cr[0].lt {
	pc = 0x83039C3C; continue 'dispatch;
	}
	// 83039C34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83039C38: 4800000C  b 0x83039c44
	pc = 0x83039C44; continue 'dispatch;
	// 83039C3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039C40: 48045B81  bl 0x8307f7c0
	ctx.lr = 0x83039C44;
	sub_8307F7C0(ctx, base);
	// 83039C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83039C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83039C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83039C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83039C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83039C58 size=8
    let mut pc: u32 = 0x83039C58;
    'dispatch: loop {
        match pc {
            0x83039C58 => {
    //   block [0x83039C58..0x83039C60)
	// 83039C58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83039C5C: 8215D198  lwz r16, -0x2e68(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039C60 size=916
    let mut pc: u32 = 0x83039C60;
    'dispatch: loop {
        match pc {
            0x83039C60 => {
    //   block [0x83039C60..0x83039FF4)
	// 83039C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039C64: 4816E4F9  bl 0x831a815c
	ctx.lr = 0x83039C68;
	sub_831A8130(ctx, base);
	// 83039C68: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83039C6C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83039C70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83039C74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83039C78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039C7C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039C80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83039C84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039C88: 4E800421  bctrl
	ctx.lr = 0x83039C8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039C8C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83039C90: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039C94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83039C98: 419A006C  beq cr6, 0x83039d04
	if ctx.cr[6].eq {
	pc = 0x83039D04; continue 'dispatch;
	}
	// 83039C9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039CA0: 41820038  beq 0x83039cd8
	if ctx.cr[0].eq {
	pc = 0x83039CD8; continue 'dispatch;
	}
	// 83039CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039CA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039CAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039CB0: 4E800421  bctrl
	ctx.lr = 0x83039CB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039CB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039CB8: 41820020  beq 0x83039cd8
	if ctx.cr[0].eq {
	pc = 0x83039CD8; continue 'dispatch;
	}
	// 83039CBC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039CC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039CC4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039CC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039CCC: 4E800421  bctrl
	ctx.lr = 0x83039CD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039CD0: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039CD4: 4800000C  b 0x83039ce0
	pc = 0x83039CE0; continue 'dispatch;
	// 83039CD8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039CDC: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039CE0: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83039CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039CE8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83039CEC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039CF0: 4BFC01E1  bl 0x82ff9ed0
	ctx.lr = 0x83039CF4;
	sub_82FF9ED0(ctx, base);
	// 83039CF4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039CF8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039CFC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039D00: 48176F29  bl 0x831b0c28
	ctx.lr = 0x83039D04;
	sub_831B0C28(ctx, base);
	// 83039D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039D08: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039D0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039D10: 4E800421  bctrl
	ctx.lr = 0x83039D14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039D14: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83039D18: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 83039D1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83039D20: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 83039D24: 4BFA5505  bl 0x82fdf228
	ctx.lr = 0x83039D28;
	sub_82FDF228(ctx, base);
	// 83039D28: 7F03D040  cmplw cr6, r3, r26
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[26].u32, &mut ctx.xer);
	// 83039D2C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039D30: 419A006C  beq cr6, 0x83039d9c
	if ctx.cr[6].eq {
	pc = 0x83039D9C; continue 'dispatch;
	}
	// 83039D34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039D38: 41820038  beq 0x83039d70
	if ctx.cr[0].eq {
	pc = 0x83039D70; continue 'dispatch;
	}
	// 83039D3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039D40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039D48: 4E800421  bctrl
	ctx.lr = 0x83039D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039D4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039D50: 41820020  beq 0x83039d70
	if ctx.cr[0].eq {
	pc = 0x83039D70; continue 'dispatch;
	}
	// 83039D54: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039D58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039D5C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039D60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039D64: 4E800421  bctrl
	ctx.lr = 0x83039D68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039D68: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039D6C: 4800000C  b 0x83039d78
	pc = 0x83039D78; continue 'dispatch;
	// 83039D70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039D74: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039D78: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83039D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039D80: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83039D84: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039D88: 4BFC0149  bl 0x82ff9ed0
	ctx.lr = 0x83039D8C;
	sub_82FF9ED0(ctx, base);
	// 83039D8C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039D90: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039D94: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039D98: 48176E91  bl 0x831b0c28
	ctx.lr = 0x83039D9C;
	sub_831B0C28(ctx, base);
	// 83039D9C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83039DA0: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039DA4: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 83039DA8: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83039DAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039DB0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83039DB4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83039DB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039DBC: 4182006C  beq 0x83039e28
	if ctx.cr[0].eq {
	pc = 0x83039E28; continue 'dispatch;
	}
	// 83039DC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83039DC4: 419A0038  beq cr6, 0x83039dfc
	if ctx.cr[6].eq {
	pc = 0x83039DFC; continue 'dispatch;
	}
	// 83039DC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039DCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039DD4: 4E800421  bctrl
	ctx.lr = 0x83039DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039DD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039DDC: 41820020  beq 0x83039dfc
	if ctx.cr[0].eq {
	pc = 0x83039DFC; continue 'dispatch;
	}
	// 83039DE0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039DE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039DE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039DF0: 4E800421  bctrl
	ctx.lr = 0x83039DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039DF4: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039DF8: 4800000C  b 0x83039e04
	pc = 0x83039E04; continue 'dispatch;
	// 83039DFC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039E00: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039E04: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83039E08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039E0C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83039E10: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039E14: 4BFC00BD  bl 0x82ff9ed0
	ctx.lr = 0x83039E18;
	sub_82FF9ED0(ctx, base);
	// 83039E18: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039E1C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039E20: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039E24: 48176E05  bl 0x831b0c28
	ctx.lr = 0x83039E28;
	sub_831B0C28(ctx, base);
	// 83039E28: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039E2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039E30: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83039E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039E38: 4E800421  bctrl
	ctx.lr = 0x83039E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039E3C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83039E40: 3F208214  lis r25, -0x7dec
	ctx.r[25].s64 = -2112618496;
	// 83039E44: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83039E48: 409A009C  bne cr6, 0x83039ee4
	if !ctx.cr[6].eq {
	pc = 0x83039EE4; continue 'dispatch;
	}
	// 83039E4C: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039E50: A179A6A4  lhz r11, -0x595c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83039E54: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83039E58: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83039E5C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83039E60: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83039E64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039E68: 4182007C  beq 0x83039ee4
	if ctx.cr[0].eq {
	pc = 0x83039EE4; continue 'dispatch;
	}
	// 83039E6C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039E70: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039E74: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83039E78: 419A006C  beq cr6, 0x83039ee4
	if ctx.cr[6].eq {
	pc = 0x83039EE4; continue 'dispatch;
	}
	// 83039E7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83039E80: 419A0038  beq cr6, 0x83039eb8
	if ctx.cr[6].eq {
	pc = 0x83039EB8; continue 'dispatch;
	}
	// 83039E84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039E88: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039E90: 4E800421  bctrl
	ctx.lr = 0x83039E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039E94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039E98: 41820020  beq 0x83039eb8
	if ctx.cr[0].eq {
	pc = 0x83039EB8; continue 'dispatch;
	}
	// 83039E9C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039EA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039EA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039EAC: 4E800421  bctrl
	ctx.lr = 0x83039EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039EB0: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83039EB4: 4800000C  b 0x83039ec0
	pc = 0x83039EC0; continue 'dispatch;
	// 83039EB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83039EBC: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83039EC0: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 83039EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83039EC8: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83039ECC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039ED0: 4BFC0001  bl 0x82ff9ed0
	ctx.lr = 0x83039ED4;
	sub_82FF9ED0(ctx, base);
	// 83039ED4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83039ED8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83039EDC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83039EE0: 48176D49  bl 0x831b0c28
	ctx.lr = 0x83039EE4;
	sub_831B0C28(ctx, base);
	// 83039EE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039EE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83039EEC: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039EF0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83039EF4: A179A6A4  lhz r11, -0x595c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83039EF8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83039EFC: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83039F00: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039F04: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039F08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039F10: 4E800421  bctrl
	ctx.lr = 0x83039F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039F14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83039F18: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83039F1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83039F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039F24: 4E800421  bctrl
	ctx.lr = 0x83039F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039F28: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83039F2C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83039F30: 41800028  blt 0x83039f58
	if ctx.cr[0].lt {
	pc = 0x83039F58; continue 'dispatch;
	}
	// 83039F34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83039F38: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039F3C: 48045885  bl 0x8307f7c0
	ctx.lr = 0x83039F40;
	sub_8307F7C0(ctx, base);
	// 83039F40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83039F44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83039F48: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83039F4C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039F50: 48045959  bl 0x8307f8a8
	ctx.lr = 0x83039F54;
	sub_8307F8A8(ctx, base);
	// 83039F54: 48000050  b 0x83039fa4
	pc = 0x83039FA4; continue 'dispatch;
	// 83039F58: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039F5C: 23BDFFFF  subfic r29, r29, -1
	ctx.xer.ca = ctx.r[29].u32 <= -1 as u32;
	ctx.r[29].s64 = (-1 as i64) - ctx.r[29].s64;
	// 83039F60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83039F64: 409A0030  bne cr6, 0x83039f94
	if !ctx.cr[6].eq {
	pc = 0x83039F94; continue 'dispatch;
	}
	// 83039F68: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83039F6C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83039F70: 4BFA7A81  bl 0x82fe19f0
	ctx.lr = 0x83039F74;
	sub_82FE19F0(ctx, base);
	// 83039F74: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83039F78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039F7C: 41820010  beq 0x83039f8c
	if ctx.cr[0].eq {
	pc = 0x83039F8C; continue 'dispatch;
	}
	// 83039F80: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83039F84: 48045935  bl 0x8307f8b8
	ctx.lr = 0x83039F88;
	sub_8307F8B8(ctx, base);
	// 83039F88: 48000008  b 0x83039f90
	pc = 0x83039F90; continue 'dispatch;
	// 83039F8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83039F90: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83039F94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83039F98: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039F9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83039FA0: 48045849  bl 0x8307f7e8
	ctx.lr = 0x83039FA4;
	sub_8307F7E8(ctx, base);
	// 83039FA4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83039FA8: 419A0040  beq cr6, 0x83039fe8
	if ctx.cr[6].eq {
	pc = 0x83039FE8; continue 'dispatch;
	}
	// 83039FAC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83039FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83039FB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83039FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83039FBC: 4E800421  bctrl
	ctx.lr = 0x83039FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83039FC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83039FC4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 83039FC8: 40820008  bne 0x83039fd0
	if !ctx.cr[0].eq {
	pc = 0x83039FD0; continue 'dispatch;
	}
	// 83039FCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83039FD0: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 83039FD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83039FD8: A159A6A4  lhz r10, -0x595c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83039FDC: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83039FE0: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 83039FE4: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83039FE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83039FEC: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83039FF0: 4816E1BC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83039FF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83039FF4 size=44
    let mut pc: u32 = 0x83039FF4;
    'dispatch: loop {
        match pc {
            0x83039FF4 => {
    //   block [0x83039FF4..0x8303A020)
	// 83039FF4: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83039FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83039FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303A000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A004: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303A008: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303A00C: 4808DAD5  bl 0x830c7ae0
	ctx.lr = 0x8303A010;
	sub_830C7AE0(ctx, base);
	// 8303A010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303A014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303A018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303A01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A020 size=232
    let mut pc: u32 = 0x8303A020;
    'dispatch: loop {
        match pc {
            0x8303A020 => {
    //   block [0x8303A020..0x8303A108)
	// 8303A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A024: 4816E135  bl 0x831a8158
	ctx.lr = 0x8303A028;
	sub_831A8130(ctx, base);
	// 8303A028: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A02C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303A030: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8303A034: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8303A038: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A03C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A040: 418200B4  beq 0x8303a0f4
	if ctx.cr[0].eq {
	pc = 0x8303A0F4; continue 'dispatch;
	}
	// 8303A044: 4811A5BD  bl 0x83154600
	ctx.lr = 0x8303A048;
	sub_83154600(ctx, base);
	// 8303A048: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303A04C: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303A050: 408100A4  ble 0x8303a0f4
	if !ctx.cr[0].gt {
	pc = 0x8303A0F4; continue 'dispatch;
	}
	// 8303A054: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A058: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A05C: 48045765  bl 0x8307f7c0
	ctx.lr = 0x8303A060;
	sub_8307F7C0(ctx, base);
	// 8303A060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303A064: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A068: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303A06C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A070: 4E800421  bctrl
	ctx.lr = 0x8303A074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A074: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A078: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303A07C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A080: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303A084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A088: 4E800421  bctrl
	ctx.lr = 0x8303A08C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A08C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303A090: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8303A094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303A098: 4BF99BA9  bl 0x82fd3c40
	ctx.lr = 0x8303A09C;
	sub_82FD3C40(ctx, base);
	// 8303A09C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303A0A0: 41820048  beq 0x8303a0e8
	if ctx.cr[0].eq {
	pc = 0x8303A0E8; continue 'dispatch;
	}
	// 8303A0A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303A0A8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303A0AC: 4BF99B95  bl 0x82fd3c40
	ctx.lr = 0x8303A0B0;
	sub_82FD3C40(ctx, base);
	// 8303A0B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303A0B4: 4082004C  bne 0x8303a100
	if !ctx.cr[0].eq {
	pc = 0x8303A100; continue 'dispatch;
	}
	// 8303A0B8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303A0BC: 409A002C  bne cr6, 0x8303a0e8
	if !ctx.cr[6].eq {
	pc = 0x8303A0E8; continue 'dispatch;
	}
	// 8303A0C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A0C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A0D0: 4E800421  bctrl
	ctx.lr = 0x8303A0D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A0D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303A0D8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303A0DC: 4BF99B65  bl 0x82fd3c40
	ctx.lr = 0x8303A0E0;
	sub_82FD3C40(ctx, base);
	// 8303A0E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303A0E4: 4082001C  bne 0x8303a100
	if !ctx.cr[0].eq {
	pc = 0x8303A100; continue 'dispatch;
	}
	// 8303A0E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8303A0EC: 7F1DC800  cmpw cr6, r29, r25
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8303A0F0: 4198FF64  blt cr6, 0x8303a054
	if ctx.cr[6].lt {
	pc = 0x8303A054; continue 'dispatch;
	}
	// 8303A0F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8303A0F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8303A0FC: 4816E0AC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8303A100: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303A104: 4BFFFFF4  b 0x8303a0f8
	pc = 0x8303A0F8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A108 size=80
    let mut pc: u32 = 0x8303A108;
    'dispatch: loop {
        match pc {
            0x8303A108 => {
    //   block [0x8303A108..0x8303A158)
	// 8303A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303A110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303A114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303A11C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A120: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303A124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A128: 4E800421  bctrl
	ctx.lr = 0x8303A12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A12C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8303A130: 4080000C  bge 0x8303a13c
	if !ctx.cr[0].lt {
	pc = 0x8303A13C; continue 'dispatch;
	}
	// 8303A134: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303A138: 4800000C  b 0x8303a144
	pc = 0x8303A144; continue 'dispatch;
	// 8303A13C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A140: 48045681  bl 0x8307f7c0
	ctx.lr = 0x8303A144;
	sub_8307F7C0(ctx, base);
	// 8303A144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303A148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303A14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303A150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303A154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303A158 size=8
    let mut pc: u32 = 0x8303A158;
    'dispatch: loop {
        match pc {
            0x8303A158 => {
    //   block [0x8303A158..0x8303A160)
	// 8303A158: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303A15C: 8215D1E0  lwz r16, -0x2e20(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A160 size=952
    let mut pc: u32 = 0x8303A160;
    'dispatch: loop {
        match pc {
            0x8303A160 => {
    //   block [0x8303A160..0x8303A518)
	// 8303A160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A164: 4816DFF9  bl 0x831a815c
	ctx.lr = 0x8303A168;
	sub_831A8130(ctx, base);
	// 8303A168: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 8303A16C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A170: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303A174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303A178: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303A17C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A180: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303A184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A188: 4E800421  bctrl
	ctx.lr = 0x8303A18C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A18C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8303A190: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A194: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303A198: 419A006C  beq cr6, 0x8303a204
	if ctx.cr[6].eq {
	pc = 0x8303A204; continue 'dispatch;
	}
	// 8303A19C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A1A0: 41820038  beq 0x8303a1d8
	if ctx.cr[0].eq {
	pc = 0x8303A1D8; continue 'dispatch;
	}
	// 8303A1A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A1A8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A1AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A1B0: 4E800421  bctrl
	ctx.lr = 0x8303A1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A1B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A1B8: 41820020  beq 0x8303a1d8
	if ctx.cr[0].eq {
	pc = 0x8303A1D8; continue 'dispatch;
	}
	// 8303A1BC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A1C4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A1C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A1CC: 4E800421  bctrl
	ctx.lr = 0x8303A1D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A1D0: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A1D4: 4800000C  b 0x8303a1e0
	pc = 0x8303A1E0; continue 'dispatch;
	// 8303A1D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A1DC: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A1E0: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A1E8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8303A1EC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A1F0: 4BFBFCE1  bl 0x82ff9ed0
	ctx.lr = 0x8303A1F4;
	sub_82FF9ED0(ctx, base);
	// 8303A1F4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A1F8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A1FC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A200: 48176A29  bl 0x831b0c28
	ctx.lr = 0x8303A204;
	sub_831B0C28(ctx, base);
	// 8303A204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A208: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A20C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A210: 4E800421  bctrl
	ctx.lr = 0x8303A214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A214: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8303A218: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 8303A21C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303A220: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8303A224: 4BFA5005  bl 0x82fdf228
	ctx.lr = 0x8303A228;
	sub_82FDF228(ctx, base);
	// 8303A228: 7F03C840  cmplw cr6, r3, r25
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8303A22C: 419A0070  beq cr6, 0x8303a29c
	if ctx.cr[6].eq {
	pc = 0x8303A29C; continue 'dispatch;
	}
	// 8303A230: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A238: 41820038  beq 0x8303a270
	if ctx.cr[0].eq {
	pc = 0x8303A270; continue 'dispatch;
	}
	// 8303A23C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A240: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A248: 4E800421  bctrl
	ctx.lr = 0x8303A24C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A24C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A250: 41820020  beq 0x8303a270
	if ctx.cr[0].eq {
	pc = 0x8303A270; continue 'dispatch;
	}
	// 8303A254: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A258: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A25C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A264: 4E800421  bctrl
	ctx.lr = 0x8303A268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A268: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A26C: 4800000C  b 0x8303a278
	pc = 0x8303A278; continue 'dispatch;
	// 8303A270: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A274: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A278: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A280: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8303A284: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A288: 4BFBFC49  bl 0x82ff9ed0
	ctx.lr = 0x8303A28C;
	sub_82FF9ED0(ctx, base);
	// 8303A28C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A290: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A294: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A298: 48176991  bl 0x831b0c28
	ctx.lr = 0x8303A29C;
	sub_831B0C28(ctx, base);
	// 8303A29C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A2A0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303A2A4: A14AA698  lhz r10, -0x5968(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8303A2A8: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A2AC: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8303A2B0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303A2B4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8303A2B8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8303A2BC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A2C0: 41820070  beq 0x8303a330
	if ctx.cr[0].eq {
	pc = 0x8303A330; continue 'dispatch;
	}
	// 8303A2C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A2C8: 419A003C  beq cr6, 0x8303a304
	if ctx.cr[6].eq {
	pc = 0x8303A304; continue 'dispatch;
	}
	// 8303A2CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A2D0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8303A2D4: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A2DC: 4E800421  bctrl
	ctx.lr = 0x8303A2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A2E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A2E4: 41820020  beq 0x8303a304
	if ctx.cr[0].eq {
	pc = 0x8303A304; continue 'dispatch;
	}
	// 8303A2E8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A2EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A2F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A2F8: 4E800421  bctrl
	ctx.lr = 0x8303A2FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A2FC: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A300: 4800000C  b 0x8303a30c
	pc = 0x8303A30C; continue 'dispatch;
	// 8303A304: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A308: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A30C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A314: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303A318: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A31C: 4BFBFBB5  bl 0x82ff9ed0
	ctx.lr = 0x8303A320;
	sub_82FF9ED0(ctx, base);
	// 8303A320: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A324: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A328: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A32C: 481768FD  bl 0x831b0c28
	ctx.lr = 0x8303A330;
	sub_831B0C28(ctx, base);
	// 8303A330: 3F408214  lis r26, -0x7dec
	ctx.r[26].s64 = -2112618496;
	// 8303A334: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A338: A13AA6A4  lhz r9, -0x595c(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303A33C: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8303A340: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8303A344: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8303A348: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 8303A34C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A350: 41820070  beq 0x8303a3c0
	if ctx.cr[0].eq {
	pc = 0x8303A3C0; continue 'dispatch;
	}
	// 8303A354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A358: 419A003C  beq cr6, 0x8303a394
	if ctx.cr[6].eq {
	pc = 0x8303A394; continue 'dispatch;
	}
	// 8303A35C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A360: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8303A364: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A36C: 4E800421  bctrl
	ctx.lr = 0x8303A370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A370: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A374: 41820020  beq 0x8303a394
	if ctx.cr[0].eq {
	pc = 0x8303A394; continue 'dispatch;
	}
	// 8303A378: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A37C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A380: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A388: 4E800421  bctrl
	ctx.lr = 0x8303A38C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A38C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A390: 4800000C  b 0x8303a39c
	pc = 0x8303A39C; continue 'dispatch;
	// 8303A394: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A398: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A39C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A3A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A3A4: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8303A3A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A3AC: 4BFBFB25  bl 0x82ff9ed0
	ctx.lr = 0x8303A3B0;
	sub_82FF9ED0(ctx, base);
	// 8303A3B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A3B4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303A3B8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A3BC: 4817686D  bl 0x831b0c28
	ctx.lr = 0x8303A3C0;
	sub_831B0C28(ctx, base);
	// 8303A3C0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303A3C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303A3C8: A17AA6A4  lhz r11, -0x595c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303A3CC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8303A3D0: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8303A3D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A3D8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A3DC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303A3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A3E4: 4E800421  bctrl
	ctx.lr = 0x8303A3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A3E8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A3EC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303A3F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303A3F4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303A3F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A3FC: 4E800421  bctrl
	ctx.lr = 0x8303A400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A400: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303A404: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303A408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303A40C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303A410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A414: 4E800421  bctrl
	ctx.lr = 0x8303A418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A418: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303A41C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303A420: 41800028  blt 0x8303a448
	if ctx.cr[0].lt {
	pc = 0x8303A448; continue 'dispatch;
	}
	// 8303A424: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303A428: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A42C: 48045395  bl 0x8307f7c0
	ctx.lr = 0x8303A430;
	sub_8307F7C0(ctx, base);
	// 8303A430: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303A434: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A438: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303A43C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A440: 48045469  bl 0x8307f8a8
	ctx.lr = 0x8303A444;
	sub_8307F8A8(ctx, base);
	// 8303A444: 48000084  b 0x8303a4c8
	pc = 0x8303A4C8; continue 'dispatch;
	// 8303A448: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A44C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303A450: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A454: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A45C: 4E800421  bctrl
	ctx.lr = 0x8303A460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A460: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303A464: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303A468: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303A46C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A470: 4E800421  bctrl
	ctx.lr = 0x8303A474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A474: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303A478: 40800008  bge 0x8303a480
	if !ctx.cr[0].lt {
	pc = 0x8303A480; continue 'dispatch;
	}
	// 8303A47C: 239CFFFF  subfic r28, r28, -1
	ctx.xer.ca = ctx.r[28].u32 <= -1 as u32;
	ctx.r[28].s64 = (-1 as i64) - ctx.r[28].s64;
	// 8303A480: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A484: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A488: 409A0030  bne cr6, 0x8303a4b8
	if !ctx.cr[6].eq {
	pc = 0x8303A4B8; continue 'dispatch;
	}
	// 8303A48C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8303A490: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303A494: 4BFA755D  bl 0x82fe19f0
	ctx.lr = 0x8303A498;
	sub_82FE19F0(ctx, base);
	// 8303A498: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8303A49C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A4A0: 41820010  beq 0x8303a4b0
	if ctx.cr[0].eq {
	pc = 0x8303A4B0; continue 'dispatch;
	}
	// 8303A4A4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303A4A8: 48045411  bl 0x8307f8b8
	ctx.lr = 0x8303A4AC;
	sub_8307F8B8(ctx, base);
	// 8303A4AC: 48000008  b 0x8303a4b4
	pc = 0x8303A4B4; continue 'dispatch;
	// 8303A4B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303A4B4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8303A4B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303A4BC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A4C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A4C4: 48045325  bl 0x8307f7e8
	ctx.lr = 0x8303A4C8;
	sub_8307F7E8(ctx, base);
	// 8303A4C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8303A4CC: 419A0040  beq cr6, 0x8303a50c
	if ctx.cr[6].eq {
	pc = 0x8303A50C; continue 'dispatch;
	}
	// 8303A4D0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A4D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A4D8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A4E0: 4E800421  bctrl
	ctx.lr = 0x8303A4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A4E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A4E8: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8303A4EC: 40820008  bne 0x8303a4f4
	if !ctx.cr[0].eq {
	pc = 0x8303A4F4; continue 'dispatch;
	}
	// 8303A4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303A4F4: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 8303A4F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303A4FC: A15AA6A4  lhz r10, -0x595c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303A500: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A504: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8303A508: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8303A50C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303A510: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 8303A514: 4816DC98  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A518 size=44
    let mut pc: u32 = 0x8303A518;
    'dispatch: loop {
        match pc {
            0x8303A518 => {
    //   block [0x8303A518..0x8303A544)
	// 8303A518: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8303A51C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A520: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303A524: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A528: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303A52C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303A530: 4808D5B1  bl 0x830c7ae0
	ctx.lr = 0x8303A534;
	sub_830C7AE0(ctx, base);
	// 8303A534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303A538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303A53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303A540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A548 size=536
    let mut pc: u32 = 0x8303A548;
    'dispatch: loop {
        match pc {
            0x8303A548 => {
    //   block [0x8303A548..0x8303A760)
	// 8303A548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A54C: 4816DC21  bl 0x831a816c
	ctx.lr = 0x8303A550;
	sub_831A8130(ctx, base);
	// 8303A550: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A554: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303A558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303A55C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303A560: A14BA698  lhz r10, -0x5968(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8303A564: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A568: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A56C: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8303A570: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303A574: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8303A578: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8303A57C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A580: 41820070  beq 0x8303a5f0
	if ctx.cr[0].eq {
	pc = 0x8303A5F0; continue 'dispatch;
	}
	// 8303A584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A588: 419A003C  beq cr6, 0x8303a5c4
	if ctx.cr[6].eq {
	pc = 0x8303A5C4; continue 'dispatch;
	}
	// 8303A58C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303A590: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A594: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A59C: 4E800421  bctrl
	ctx.lr = 0x8303A5A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A5A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A5A4: 41820020  beq 0x8303a5c4
	if ctx.cr[0].eq {
	pc = 0x8303A5C4; continue 'dispatch;
	}
	// 8303A5A8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A5AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A5B0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A5B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A5B8: 4E800421  bctrl
	ctx.lr = 0x8303A5BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A5BC: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A5C0: 4800000C  b 0x8303a5cc
	pc = 0x8303A5CC; continue 'dispatch;
	// 8303A5C4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A5C8: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A5CC: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A5D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A5D4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303A5D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A5DC: 4BFBF8F5  bl 0x82ff9ed0
	ctx.lr = 0x8303A5E0;
	sub_82FF9ED0(ctx, base);
	// 8303A5E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A5E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A5E8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A5EC: 4817663D  bl 0x831b0c28
	ctx.lr = 0x8303A5F0;
	sub_831B0C28(ctx, base);
	// 8303A5F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A5F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A5FC: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303A600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A604: 4E800421  bctrl
	ctx.lr = 0x8303A608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A608: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303A60C: 40800074  bge 0x8303a680
	if !ctx.cr[0].lt {
	pc = 0x8303A680; continue 'dispatch;
	}
	// 8303A610: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A618: 419A003C  beq cr6, 0x8303a654
	if ctx.cr[6].eq {
	pc = 0x8303A654; continue 'dispatch;
	}
	// 8303A61C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303A620: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A624: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A62C: 4E800421  bctrl
	ctx.lr = 0x8303A630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A630: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A634: 41820020  beq 0x8303a654
	if ctx.cr[0].eq {
	pc = 0x8303A654; continue 'dispatch;
	}
	// 8303A638: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A63C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A640: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A648: 4E800421  bctrl
	ctx.lr = 0x8303A64C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A64C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A650: 4800000C  b 0x8303a65c
	pc = 0x8303A65C; continue 'dispatch;
	// 8303A654: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A658: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A65C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A660: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A664: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8303A668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A66C: 4BFBF865  bl 0x82ff9ed0
	ctx.lr = 0x8303A670;
	sub_82FF9ED0(ctx, base);
	// 8303A670: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A678: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A67C: 481765AD  bl 0x831b0c28
	ctx.lr = 0x8303A680;
	sub_831B0C28(ctx, base);
	// 8303A680: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303A684: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A688: 48045139  bl 0x8307f7c0
	ctx.lr = 0x8303A68C;
	sub_8307F7C0(ctx, base);
	// 8303A68C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303A690: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303A694: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A698: 480451C1  bl 0x8307f858
	ctx.lr = 0x8303A69C;
	sub_8307F858(ctx, base);
	// 8303A69C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A6A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A6A4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A6A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A6AC: 4E800421  bctrl
	ctx.lr = 0x8303A6B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A6B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A6B4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8303A6B8: 40820008  bne 0x8303a6c0
	if !ctx.cr[0].eq {
	pc = 0x8303A6C0; continue 'dispatch;
	}
	// 8303A6BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303A6C0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8303A6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A6C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303A6CC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303A6D0: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A6D4: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303A6D8: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8303A6DC: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8303A6E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A6E4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303A6E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A6EC: 4E800421  bctrl
	ctx.lr = 0x8303A6F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A6F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303A6F4: 41820060  beq 0x8303a754
	if ctx.cr[0].eq {
	pc = 0x8303A754; continue 'dispatch;
	}
	// 8303A6F8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A6FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A700: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 8303A704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A708: 4E800421  bctrl
	ctx.lr = 0x8303A70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A70C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A710: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A714: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303A718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A71C: 4E800421  bctrl
	ctx.lr = 0x8303A720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A720: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A724: 41820030  beq 0x8303a754
	if ctx.cr[0].eq {
	pc = 0x8303A754; continue 'dispatch;
	}
	// 8303A728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A72C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303A730: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303A734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A738: 4E800421  bctrl
	ctx.lr = 0x8303A73C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A73C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A740: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303A744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A748: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A74C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A750: 4E800421  bctrl
	ctx.lr = 0x8303A754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303A758: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8303A75C: 4816DA60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A760 size=548
    let mut pc: u32 = 0x8303A760;
    'dispatch: loop {
        match pc {
            0x8303A760 => {
    //   block [0x8303A760..0x8303A984)
	// 8303A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A764: 4816DA05  bl 0x831a8168
	ctx.lr = 0x8303A768;
	sub_831A8130(ctx, base);
	// 8303A768: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A76C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303A770: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303A774: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303A778: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303A77C: A14BA698  lhz r10, -0x5968(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8303A780: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A784: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A788: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8303A78C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303A790: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8303A794: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8303A798: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A79C: 41820070  beq 0x8303a80c
	if ctx.cr[0].eq {
	pc = 0x8303A80C; continue 'dispatch;
	}
	// 8303A7A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A7A4: 419A003C  beq cr6, 0x8303a7e0
	if ctx.cr[6].eq {
	pc = 0x8303A7E0; continue 'dispatch;
	}
	// 8303A7A8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303A7AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A7B0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A7B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A7B8: 4E800421  bctrl
	ctx.lr = 0x8303A7BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A7BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A7C0: 41820020  beq 0x8303a7e0
	if ctx.cr[0].eq {
	pc = 0x8303A7E0; continue 'dispatch;
	}
	// 8303A7C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A7C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A7CC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A7D4: 4E800421  bctrl
	ctx.lr = 0x8303A7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A7D8: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A7DC: 4800000C  b 0x8303a7e8
	pc = 0x8303A7E8; continue 'dispatch;
	// 8303A7E0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A7E4: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A7E8: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A7F0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303A7F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A7F8: 4BFBF6D9  bl 0x82ff9ed0
	ctx.lr = 0x8303A7FC;
	sub_82FF9ED0(ctx, base);
	// 8303A7FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A804: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A808: 48176421  bl 0x831b0c28
	ctx.lr = 0x8303A80C;
	sub_831B0C28(ctx, base);
	// 8303A80C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A810: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303A814: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A81C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303A820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A824: 4E800421  bctrl
	ctx.lr = 0x8303A828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A828: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303A82C: 40800074  bge 0x8303a8a0
	if !ctx.cr[0].lt {
	pc = 0x8303A8A0; continue 'dispatch;
	}
	// 8303A830: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A838: 419A003C  beq cr6, 0x8303a874
	if ctx.cr[6].eq {
	pc = 0x8303A874; continue 'dispatch;
	}
	// 8303A83C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303A840: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A844: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A84C: 4E800421  bctrl
	ctx.lr = 0x8303A850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A850: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A854: 41820020  beq 0x8303a874
	if ctx.cr[0].eq {
	pc = 0x8303A874; continue 'dispatch;
	}
	// 8303A858: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A85C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A860: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A868: 4E800421  bctrl
	ctx.lr = 0x8303A86C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A86C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303A870: 4800000C  b 0x8303a87c
	pc = 0x8303A87C; continue 'dispatch;
	// 8303A874: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303A878: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303A87C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303A880: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303A884: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8303A888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A88C: 4BFBF645  bl 0x82ff9ed0
	ctx.lr = 0x8303A890;
	sub_82FF9ED0(ctx, base);
	// 8303A890: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303A894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303A898: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303A89C: 4817638D  bl 0x831b0c28
	ctx.lr = 0x8303A8A0;
	sub_831B0C28(ctx, base);
	// 8303A8A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303A8A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A8A8: 48044F19  bl 0x8307f7c0
	ctx.lr = 0x8303A8AC;
	sub_8307F7C0(ctx, base);
	// 8303A8AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303A8B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303A8B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A8B8: 48044FA1  bl 0x8307f858
	ctx.lr = 0x8303A8BC;
	sub_8307F858(ctx, base);
	// 8303A8BC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A8C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A8C4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A8C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A8CC: 4E800421  bctrl
	ctx.lr = 0x8303A8D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A8D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A8D4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8303A8D8: 40820008  bne 0x8303a8e0
	if !ctx.cr[0].eq {
	pc = 0x8303A8E0; continue 'dispatch;
	}
	// 8303A8DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303A8E0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8303A8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A8E8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303A8EC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303A8F0: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303A8F4: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303A8F8: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8303A8FC: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8303A900: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A904: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303A908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A90C: 4E800421  bctrl
	ctx.lr = 0x8303A910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A910: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303A914: 41820064  beq 0x8303a978
	if ctx.cr[0].eq {
	pc = 0x8303A978; continue 'dispatch;
	}
	// 8303A918: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A91C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A920: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 8303A924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A928: 4E800421  bctrl
	ctx.lr = 0x8303A92C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A92C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A930: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303A934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303A938: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303A93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A940: 4E800421  bctrl
	ctx.lr = 0x8303A944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A948: 41820030  beq 0x8303a978
	if ctx.cr[0].eq {
	pc = 0x8303A978; continue 'dispatch;
	}
	// 8303A94C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A950: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303A954: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303A958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A95C: 4E800421  bctrl
	ctx.lr = 0x8303A960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A960: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A964: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303A968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303A96C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303A970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A974: 4E800421  bctrl
	ctx.lr = 0x8303A978;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303A97C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8303A980: 4816D838  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303A988 size=620
    let mut pc: u32 = 0x8303A988;
    'dispatch: loop {
        match pc {
            0x8303A988 => {
    //   block [0x8303A988..0x8303ABF4)
	// 8303A988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303A98C: 4816D7D9  bl 0x831a8164
	ctx.lr = 0x8303A990;
	sub_831A8130(ctx, base);
	// 8303A990: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303A994: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303A998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303A99C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303A9A0: A14BA698  lhz r10, -0x5968(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8303A9A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A9A8: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A9AC: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8303A9B0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8303A9B4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8303A9B8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8303A9BC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A9C0: 41820070  beq 0x8303aa30
	if ctx.cr[0].eq {
	pc = 0x8303AA30; continue 'dispatch;
	}
	// 8303A9C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303A9C8: 419A003C  beq cr6, 0x8303aa04
	if ctx.cr[6].eq {
	pc = 0x8303AA04; continue 'dispatch;
	}
	// 8303A9CC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303A9D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A9D4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A9D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A9DC: 4E800421  bctrl
	ctx.lr = 0x8303A9E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A9E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303A9E4: 41820020  beq 0x8303aa04
	if ctx.cr[0].eq {
	pc = 0x8303AA04; continue 'dispatch;
	}
	// 8303A9E8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303A9EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303A9F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303A9F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303A9F8: 4E800421  bctrl
	ctx.lr = 0x8303A9FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303A9FC: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303AA00: 4800000C  b 0x8303aa0c
	pc = 0x8303AA0C; continue 'dispatch;
	// 8303AA04: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303AA08: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303AA0C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303AA10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303AA14: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8303AA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303AA1C: 4BFBF4B5  bl 0x82ff9ed0
	ctx.lr = 0x8303AA20;
	sub_82FF9ED0(ctx, base);
	// 8303AA20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303AA24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303AA28: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303AA2C: 481761FD  bl 0x831b0c28
	ctx.lr = 0x8303AA30;
	sub_831B0C28(ctx, base);
	// 8303AA30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AA34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303AA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AA3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AA40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AA44: 4E800421  bctrl
	ctx.lr = 0x8303AA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AA48: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303AA4C: 40820074  bne 0x8303aac0
	if !ctx.cr[0].eq {
	pc = 0x8303AAC0; continue 'dispatch;
	}
	// 8303AA50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AA54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303AA58: 419A003C  beq cr6, 0x8303aa94
	if ctx.cr[6].eq {
	pc = 0x8303AA94; continue 'dispatch;
	}
	// 8303AA5C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8303AA60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AA64: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303AA68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AA6C: 4E800421  bctrl
	ctx.lr = 0x8303AA70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AA70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303AA74: 41820020  beq 0x8303aa94
	if ctx.cr[0].eq {
	pc = 0x8303AA94; continue 'dispatch;
	}
	// 8303AA78: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AA7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AA80: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303AA84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AA88: 4E800421  bctrl
	ctx.lr = 0x8303AA8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AA8C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303AA90: 4800000C  b 0x8303aa9c
	pc = 0x8303AA9C; continue 'dispatch;
	// 8303AA94: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303AA98: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303AA9C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8303AAA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303AAA4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8303AAA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303AAAC: 4BFBF425  bl 0x82ff9ed0
	ctx.lr = 0x8303AAB0;
	sub_82FF9ED0(ctx, base);
	// 8303AAB0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303AAB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8303AAB8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8303AABC: 4817616D  bl 0x831b0c28
	ctx.lr = 0x8303AAC0;
	sub_831B0C28(ctx, base);
	// 8303AAC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303AAC4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AAC8: 48044D91  bl 0x8307f858
	ctx.lr = 0x8303AACC;
	sub_8307F858(ctx, base);
	// 8303AACC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AAD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AAD4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303AAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AADC: 4E800421  bctrl
	ctx.lr = 0x8303AAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AAE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303AAE4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8303AAE8: 40820008  bne 0x8303aaf0
	if !ctx.cr[0].eq {
	pc = 0x8303AAF0; continue 'dispatch;
	}
	// 8303AAEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303AAF0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8303AAF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AAF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303AAFC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303AB00: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AB04: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8303AB08: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8303AB0C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8303AB10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB14: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303AB18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB1C: 4E800421  bctrl
	ctx.lr = 0x8303AB20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AB20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303AB24: 418200C4  beq 0x8303abe8
	if ctx.cr[0].eq {
	pc = 0x8303ABE8; continue 'dispatch;
	}
	// 8303AB28: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AB2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB30: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 8303AB34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB38: 4E800421  bctrl
	ctx.lr = 0x8303AB3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AB3C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303AB44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303AB48: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303AB4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB50: 4E800421  bctrl
	ctx.lr = 0x8303AB54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AB54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB58: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303AB5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303AB60: 41820030  beq 0x8303ab90
	if ctx.cr[0].eq {
	pc = 0x8303AB90; continue 'dispatch;
	}
	// 8303AB64: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB68: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303AB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB70: 4E800421  bctrl
	ctx.lr = 0x8303AB74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AB74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303AB78: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303AB7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303AB80: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303AB84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB88: 4E800421  bctrl
	ctx.lr = 0x8303AB8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AB8C: 48000028  b 0x8303abb4
	pc = 0x8303ABB4; continue 'dispatch;
	// 8303AB90: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AB94: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8303AB98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AB9C: 4E800421  bctrl
	ctx.lr = 0x8303ABA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ABA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303ABA4: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303ABA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303ABAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ABB0: 4E800421  bctrl
	ctx.lr = 0x8303ABB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ABB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8303ABB8: 419A0030  beq cr6, 0x8303abe8
	if ctx.cr[6].eq {
	pc = 0x8303ABE8; continue 'dispatch;
	}
	// 8303ABBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ABC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303ABC4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303ABC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ABCC: 4E800421  bctrl
	ctx.lr = 0x8303ABD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ABD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ABD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303ABD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303ABDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303ABE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ABE4: 4E800421  bctrl
	ctx.lr = 0x8303ABE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ABE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303ABEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8303ABF0: 4816D5C4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303ABF8 size=368
    let mut pc: u32 = 0x8303ABF8;
    'dispatch: loop {
        match pc {
            0x8303ABF8 => {
    //   block [0x8303ABF8..0x8303AD68)
	// 8303ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303ABFC: 4816D569  bl 0x831a8164
	ctx.lr = 0x8303AC00;
	sub_831A8130(ctx, base);
	// 8303AC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303AC08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8303AC0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC10: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303AC14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AC18: 4E800421  bctrl
	ctx.lr = 0x8303AC1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AC1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303AC20: 37DDFFFF  addic. r30, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303AC24: 41800054  blt 0x8303ac78
	if ctx.cr[0].lt {
	pc = 0x8303AC78; continue 'dispatch;
	}
	// 8303AC28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303AC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AC34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AC38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AC3C: 4E800421  bctrl
	ctx.lr = 0x8303AC40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AC40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC44: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8303AC48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AC4C: 4E800421  bctrl
	ctx.lr = 0x8303AC50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AC50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303AC54: 4082001C  bne 0x8303ac70
	if !ctx.cr[0].eq {
	pc = 0x8303AC70; continue 'dispatch;
	}
	// 8303AC58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303AC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AC64: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303AC68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AC6C: 4E800421  bctrl
	ctx.lr = 0x8303AC70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AC70: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303AC74: 4080FFB4  bge 0x8303ac28
	if !ctx.cr[0].lt {
	pc = 0x8303AC28; continue 'dispatch;
	}
	// 8303AC78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303AC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AC84: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303AC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AC8C: 4E800421  bctrl
	ctx.lr = 0x8303AC90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AC90: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303AC94: 419A00CC  beq cr6, 0x8303ad60
	if ctx.cr[6].eq {
	pc = 0x8303AD60; continue 'dispatch;
	}
	// 8303AC98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AC9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303ACA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303ACA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303ACA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ACAC: 4E800421  bctrl
	ctx.lr = 0x8303ACB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ACB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8303ACB4: 409A0020  bne cr6, 0x8303acd4
	if !ctx.cr[6].eq {
	pc = 0x8303ACD4; continue 'dispatch;
	}
	// 8303ACB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ACBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303ACC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303ACC4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303ACC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ACCC: 4E800421  bctrl
	ctx.lr = 0x8303ACD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ACD0: 48000090  b 0x8303ad60
	pc = 0x8303AD60; continue 'dispatch;
	// 8303ACD4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ACD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303ACDC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303ACE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ACE4: 4E800421  bctrl
	ctx.lr = 0x8303ACE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ACE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303ACEC: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8303ACF0: 41820070  beq 0x8303ad60
	if ctx.cr[0].eq {
	pc = 0x8303AD60; continue 'dispatch;
	}
	// 8303ACF4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ACF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303ACFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303AD00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AD04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AD08: 4E800421  bctrl
	ctx.lr = 0x8303AD0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AD0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AD10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303AD14: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303AD18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AD1C: 4E800421  bctrl
	ctx.lr = 0x8303AD20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AD20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AD24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303AD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AD2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303AD30: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303AD34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AD38: 4E800421  bctrl
	ctx.lr = 0x8303AD3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AD3C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AD40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303AD44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303AD48: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 8303AD4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AD50: 4E800421  bctrl
	ctx.lr = 0x8303AD54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AD54: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8303AD58: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8303AD5C: 4198FF98  blt cr6, 0x8303acf4
	if ctx.cr[6].lt {
	pc = 0x8303ACF4; continue 'dispatch;
	}
	// 8303AD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303AD64: 4816D450  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303AD68 size=200
    let mut pc: u32 = 0x8303AD68;
    'dispatch: loop {
        match pc {
            0x8303AD68 => {
    //   block [0x8303AD68..0x8303AE30)
	// 8303AD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303AD6C: 4816D3FD  bl 0x831a8168
	ctx.lr = 0x8303AD70;
	sub_831A8130(ctx, base);
	// 8303AD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AD74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8303AD78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303AD7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303AD80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AD84: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303AD88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AD8C: 4E800421  bctrl
	ctx.lr = 0x8303AD90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AD90: 37A3FFFF  addic. r29, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303AD94: 41800094  blt 0x8303ae28
	if ctx.cr[0].lt {
	pc = 0x8303AE28; continue 'dispatch;
	}
	// 8303AD98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AD9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303ADA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303ADA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303ADA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ADAC: 4E800421  bctrl
	ctx.lr = 0x8303ADB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ADB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303ADB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ADB8: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8303ADBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ADC0: 4E800421  bctrl
	ctx.lr = 0x8303ADC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ADC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303ADC8: 4182001C  beq 0x8303ade4
	if ctx.cr[0].eq {
	pc = 0x8303ADE4; continue 'dispatch;
	}
	// 8303ADCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ADD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303ADD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303ADD8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303ADDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ADE0: 4E800421  bctrl
	ctx.lr = 0x8303ADE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ADE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303ADE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303ADEC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303ADF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303ADF4: 4E800421  bctrl
	ctx.lr = 0x8303ADF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303ADF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303ADFC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AE00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303AE04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303AE08: 4182000C  beq 0x8303ae14
	if ctx.cr[0].eq {
	pc = 0x8303AE14; continue 'dispatch;
	}
	// 8303AE0C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303AE10: 48000008  b 0x8303ae18
	pc = 0x8303AE18; continue 'dispatch;
	// 8303AE14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AE18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AE1C: 4E800421  bctrl
	ctx.lr = 0x8303AE20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AE20: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303AE24: 4080FF74  bge 0x8303ad98
	if !ctx.cr[0].lt {
	pc = 0x8303AD98; continue 'dispatch;
	}
	// 8303AE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303AE2C: 4816D38C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303AE30 size=16
    let mut pc: u32 = 0x8303AE30;
    'dispatch: loop {
        match pc {
            0x8303AE30 => {
    //   block [0x8303AE30..0x8303AE40)
	// 8303AE30: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AE34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303AE38: 41820008  beq 0x8303ae40
	if ctx.cr[0].eq {
		sub_8303AE40(ctx, base);
		return;
	}
	// 8303AE3C: 481197C4  b 0x83154600
	sub_83154600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303AE40 size=8
    let mut pc: u32 = 0x8303AE40;
    'dispatch: loop {
        match pc {
            0x8303AE40 => {
    //   block [0x8303AE40..0x8303AE48)
	// 8303AE40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303AE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303AE48 size=96
    let mut pc: u32 = 0x8303AE48;
    'dispatch: loop {
        match pc {
            0x8303AE48 => {
    //   block [0x8303AE48..0x8303AEA8)
	// 8303AE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303AE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303AE50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303AE54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303AE58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AE5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303AE60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8303AE64: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AE68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303AE6C: 41820020  beq 0x8303ae8c
	if ctx.cr[0].eq {
	pc = 0x8303AE8C; continue 'dispatch;
	}
	// 8303AE70: 48119791  bl 0x83154600
	ctx.lr = 0x8303AE74;
	sub_83154600(ctx, base);
	// 8303AE74: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8303AE78: 40980014  bge cr6, 0x8303ae8c
	if !ctx.cr[6].lt {
	pc = 0x8303AE8C; continue 'dispatch;
	}
	// 8303AE7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303AE80: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AE84: 4804493D  bl 0x8307f7c0
	ctx.lr = 0x8303AE88;
	sub_8307F7C0(ctx, base);
	// 8303AE88: 48000008  b 0x8303ae90
	pc = 0x8303AE90; continue 'dispatch;
	// 8303AE8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303AE90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303AE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303AE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303AE9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303AEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303AEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303AEA8 size=8
    let mut pc: u32 = 0x8303AEA8;
    'dispatch: loop {
        match pc {
            0x8303AEA8 => {
    //   block [0x8303AEA8..0x8303AEB0)
	// 8303AEA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303AEAC: 8215D228  lwz r16, -0x2dd8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303AEB0 size=124
    let mut pc: u32 = 0x8303AEB0;
    'dispatch: loop {
        match pc {
            0x8303AEB0 => {
    //   block [0x8303AEB0..0x8303AF2C)
	// 8303AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303AEB4: 4816D2B9  bl 0x831a816c
	ctx.lr = 0x8303AEB8;
	sub_831A8130(ctx, base);
	// 8303AEB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8303AEBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AEC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303AEC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8303AEC8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8303AECC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303AED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303AED4: 909E0008  stw r4, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8303AED8: 396BD100  addi r11, r11, -0x2f00
	ctx.r[11].s64 = ctx.r[11].s64 + -12032;
	// 8303AEDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8303AEE0: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8303AEE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303AEE8: 995E000C  stb r10, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 8303AEEC: 419A0034  beq cr6, 0x8303af20
	if ctx.cr[6].eq {
	pc = 0x8303AF20; continue 'dispatch;
	}
	// 8303AEF0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AEF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303AEF8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303AEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AF00: 4E800421  bctrl
	ctx.lr = 0x8303AF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AF04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303AF08: 41820018  beq 0x8303af20
	if ctx.cr[0].eq {
	pc = 0x8303AF20; continue 'dispatch;
	}
	// 8303AF0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8303AF10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303AF14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303AF18: 997E000C  stb r11, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 8303AF1C: 4BFFE9D5  bl 0x830398f0
	ctx.lr = 0x8303AF20;
	sub_830398F0(ctx, base);
	// 8303AF20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303AF24: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8303AF28: 4816D294  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AF2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303AF2C size=40
    let mut pc: u32 = 0x8303AF2C;
    'dispatch: loop {
        match pc {
            0x8303AF2C => {
    //   block [0x8303AF2C..0x8303AF54)
	// 8303AF2C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8303AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303AF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303AF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AF3C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8303AF40: 4BFF9A09  bl 0x83034948
	ctx.lr = 0x8303AF44;
	sub_83034948(ctx, base);
	// 8303AF44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303AF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303AF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303AF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303AF58 size=36
    let mut pc: u32 = 0x8303AF58;
    'dispatch: loop {
        match pc {
            0x8303AF58 => {
    //   block [0x8303AF58..0x8303AF7C)
	// 8303AF58: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8303AF5C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8303AF60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303AF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303AF68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AF6C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303AF70: 4082000C  bne 0x8303af7c
	if !ctx.cr[0].eq {
		sub_8303AF7C(ctx, base);
		return;
	}
	// 8303AF74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303AF78: 48000018  b 0x8303af90
	sub_8303AF7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AF7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303AF7C size=48
    let mut pc: u32 = 0x8303AF7C;
    'dispatch: loop {
        match pc {
            0x8303AF7C => {
    //   block [0x8303AF7C..0x8303AFAC)
	// 8303AF7C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303AF80: 419A000C  beq cr6, 0x8303af8c
	if ctx.cr[6].eq {
	pc = 0x8303AF8C; continue 'dispatch;
	}
	// 8303AF84: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8303AF88: 409A0008  bne cr6, 0x8303af90
	if !ctx.cr[6].eq {
	pc = 0x8303AF90; continue 'dispatch;
	}
	// 8303AF8C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8303AF90: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303AF94: 419A000C  beq cr6, 0x8303afa0
	if ctx.cr[6].eq {
	pc = 0x8303AFA0; continue 'dispatch;
	}
	// 8303AF98: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8303AF9C: 409A0008  bne cr6, 0x8303afa4
	if !ctx.cr[6].eq {
	pc = 0x8303AFA4; continue 'dispatch;
	}
	// 8303AFA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303AFA4: 80CA0018  lwz r6, 0x18(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303AFA8: 48044D00  b 0x8307fca8
	sub_8307FCA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303AFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303AFB0 size=216
    let mut pc: u32 = 0x8303AFB0;
    'dispatch: loop {
        match pc {
            0x8303AFB0 => {
    //   block [0x8303AFB0..0x8303B088)
	// 8303AFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303AFB4: 4816D1B9  bl 0x831a816c
	ctx.lr = 0x8303AFB8;
	sub_831A8130(ctx, base);
	// 8303AFB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303AFBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303AFC0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303AFC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303AFC8: 7FC45214  add r30, r4, r10
	ctx.r[30].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 8303AFCC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303AFD0: 419800B0  blt cr6, 0x8303b080
	if ctx.cr[6].lt {
	pc = 0x8303B080; continue 'dispatch;
	}
	// 8303AFD4: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8303AFD8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303AFDC: 40980008  bge cr6, 0x8303afe4
	if !ctx.cr[6].lt {
	pc = 0x8303AFE4; continue 'dispatch;
	}
	// 8303AFE0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8303AFE4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303AFE8: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8303AFEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303AFF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303AFF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303AFF8: 4E800421  bctrl
	ctx.lr = 0x8303AFFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303AFFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B000: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303B008: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8303B00C: 40990028  ble cr6, 0x8303b034
	if !ctx.cr[6].gt {
	pc = 0x8303B034; continue 'dispatch;
	}
	// 8303B010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303B014: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B018: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303B01C: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8303B020: 7D2AE92E  stwx r9, r10, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 8303B024: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8303B028: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B02C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8303B030: 4198FFE4  blt cr6, 0x8303b014
	if ctx.cr[6].lt {
	pc = 0x8303B014; continue 'dispatch;
	}
	// 8303B034: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8303B038: 40980028  bge cr6, 0x8303b060
	if !ctx.cr[6].lt {
	pc = 0x8303B060; continue 'dispatch;
	}
	// 8303B03C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8303B040: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303B044: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 8303B048: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303B04C: 41820014  beq 0x8303b060
	if ctx.cr[0].eq {
	pc = 0x8303B060; continue 'dispatch;
	}
	// 8303B050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303B054: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8303B058: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8303B05C: 4200FFF8  bdnz 0x8303b054
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8303B054; continue 'dispatch;
	}
	// 8303B060: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303B064: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B068: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B06C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B070: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303B074: 4E800421  bctrl
	ctx.lr = 0x8303B078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303B078: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8303B07C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303B080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303B084: 4816D138  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B088 size=20
    let mut pc: u32 = 0x8303B088;
    'dispatch: loop {
        match pc {
            0x8303B088 => {
    //   block [0x8303B088..0x8303B09C)
	// 8303B088: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8303B08C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B090: 4082000C  bne 0x8303b09c
	if !ctx.cr[0].eq {
		sub_8303B09C(ctx, base);
		return;
	}
	// 8303B094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303B098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


