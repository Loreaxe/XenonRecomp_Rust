pub fn sub_82FF8ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF8ED0 size=8
    let mut pc: u32 = 0x82FF8ED0;
    'dispatch: loop {
        match pc {
            0x82FF8ED0 => {
    //   block [0x82FF8ED0..0x82FF8ED8)
	// 82FF8ED0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FF8ED4: 8213F7D0  lwz r16, -0x830(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-2096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8ED8 size=188
    let mut pc: u32 = 0x82FF8ED8;
    'dispatch: loop {
        match pc {
            0x82FF8ED8 => {
    //   block [0x82FF8ED8..0x82FF8F94)
	// 82FF8ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8EDC: 481AF28D  bl 0x831a8168
	ctx.lr = 0x82FF8EE0;
	sub_831A8130(ctx, base);
	// 82FF8EE0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FF8EE4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8EE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF8EEC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF8EF0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF8EF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FF8EF8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FF8EFC: 90DE0010  stw r6, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82FF8F00: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FF8F04: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF8F08: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FF8F0C: B15E0000  sth r10, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FF8F10: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FF8F14: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF8F1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF8F24: 4E800421  bctrl
	ctx.lr = 0x82FF8F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF8F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF8F2C: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82FF8F30: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FF8F34: 939E0024  stw r28, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82FF8F38: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FF8F3C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF8F40: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FF8F44: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8F48: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FF8F4C: 4BFDF34D  bl 0x82fd8298
	ctx.lr = 0x82FF8F50;
	sub_82FD8298(ctx, base);
	// 82FF8F50: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FF8F54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF8F58: 4182001C  beq 0x82ff8f74
	if ctx.cr[0].eq {
	pc = 0x82FF8F74; continue 'dispatch;
	}
	// 82FF8F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FF8F60: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF8F64: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 82FF8F68: 4806F839  bl 0x830687a0
	ctx.lr = 0x82FF8F6C;
	sub_830687A0(ctx, base);
	// 82FF8F6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF8F70: 48000008  b 0x82ff8f78
	pc = 0x82FF8F78; continue 'dispatch;
	// 82FF8F74: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FF8F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF8F7C: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FF8F80: 939E002C  stw r28, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82FF8F84: 4BFFFAD5  bl 0x82ff8a58
	ctx.lr = 0x82FF8F88;
	sub_82FF8A58(ctx, base);
	// 82FF8F88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF8F8C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FF8F90: 481AF228  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8F94 size=44
    let mut pc: u32 = 0x82FF8F94;
    'dispatch: loop {
        match pc {
            0x82FF8F94 => {
    //   block [0x82FF8F94..0x82FF8FC0)
	// 82FF8F94: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FF8F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF8FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8FA4: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF8FA8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FF8FAC: 4BFDF335  bl 0x82fd82e0
	ctx.lr = 0x82FF8FB0;
	sub_82FD82E0(ctx, base);
	// 82FF8FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF8FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF8FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF8FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF8FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF8FC0 size=224
    let mut pc: u32 = 0x82FF8FC0;
    'dispatch: loop {
        match pc {
            0x82FF8FC0 => {
    //   block [0x82FF8FC0..0x82FF90A0)
	// 82FF8FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF8FC4: 481AF1A5  bl 0x831a8168
	ctx.lr = 0x82FF8FC8;
	sub_831A8130(ctx, base);
	// 82FF8FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF8FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF8FD0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF8FD4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF8FD8: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FF8FDC: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF8FE0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF8FE4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF8FE8: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF8FEC: 4BFFF145  bl 0x82ff8130
	ctx.lr = 0x82FF8FF0;
	sub_82FF8130(ctx, base);
	// 82FF8FF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF8FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF8FF8: 4BFFF5A1  bl 0x82ff8598
	ctx.lr = 0x82FF8FFC;
	sub_82FF8598(ctx, base);
	// 82FF8FFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9004: 4BFFF655  bl 0x82ff8658
	ctx.lr = 0x82FF9008;
	sub_82FF8658(ctx, base);
	// 82FF9008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF900C: 4BFFF37D  bl 0x82ff8388
	ctx.lr = 0x82FF9010;
	sub_82FF8388(ctx, base);
	// 82FF9010: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF9014: 419A0084  beq cr6, 0x82ff9098
	if ctx.cr[6].eq {
	pc = 0x82FF9098; continue 'dispatch;
	}
	// 82FF9018: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF901C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF9020: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9024: 7FA35850  subf r29, r3, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FF9028: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FF902C: 40990058  ble cr6, 0x82ff9084
	if !ctx.cr[6].gt {
	pc = 0x82FF9084; continue 'dispatch;
	}
	// 82FF9030: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FF9034: 481AF4DD  bl 0x831a8510
	ctx.lr = 0x82FF9038;
	sub_831A8510(ctx, base);
	// 82FF9038: 7F9DE214  add r28, r29, r28
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82FF903C: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82FF9040: 48000020  b 0x82ff9060
	pc = 0x82FF9060; continue 'dispatch;
	// 82FF9044: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF9048: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF904C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9050: 481AF4C1  bl 0x831a8510
	ctx.lr = 0x82FF9054;
	sub_831A8510(ctx, base);
	// 82FF9054: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9058: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FF905C: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82FF9060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9064: 4BFFFBE5  bl 0x82ff8c48
	ctx.lr = 0x82FF9068;
	sub_82FF8C48(ctx, base);
	// 82FF9068: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF906C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF9070: 4098FFD4  bge cr6, 0x82ff9044
	if !ctx.cr[6].lt {
	pc = 0x82FF9044; continue 'dispatch;
	}
	// 82FF9074: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF9078: 419A0020  beq cr6, 0x82ff9098
	if ctx.cr[6].eq {
	pc = 0x82FF9098; continue 'dispatch;
	}
	// 82FF907C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9080: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF9084: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF9088: 481AF489  bl 0x831a8510
	ctx.lr = 0x82FF908C;
	sub_831A8510(ctx, base);
	// 82FF908C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9090: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF9094: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF9098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF909C: 481AF11C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF90A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF90A0 size=244
    let mut pc: u32 = 0x82FF90A0;
    'dispatch: loop {
        match pc {
            0x82FF90A0 => {
    //   block [0x82FF90A0..0x82FF9194)
	// 82FF90A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF90A4: 481AF0C5  bl 0x831a8168
	ctx.lr = 0x82FF90A8;
	sub_831A8130(ctx, base);
	// 82FF90A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF90AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF90B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF90B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FF90B8: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF90BC: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF90C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF90C4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF90C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF90CC: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF90D0: 4BFFF061  bl 0x82ff8130
	ctx.lr = 0x82FF90D4;
	sub_82FF8130(ctx, base);
	// 82FF90D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF90D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF90DC: 4BFFF57D  bl 0x82ff8658
	ctx.lr = 0x82FF90E0;
	sub_82FF8658(ctx, base);
	// 82FF90E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FF90E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF90E8: 4BFFF4B1  bl 0x82ff8598
	ctx.lr = 0x82FF90EC;
	sub_82FF8598(ctx, base);
	// 82FF90EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF90F0: 4BFFF3A1  bl 0x82ff8490
	ctx.lr = 0x82FF90F4;
	sub_82FF8490(ctx, base);
	// 82FF90F4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FF90F8: 419A0094  beq cr6, 0x82ff918c
	if ctx.cr[6].eq {
	pc = 0x82FF918C; continue 'dispatch;
	}
	// 82FF90FC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF9100: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FF9104: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9108: 7FA45850  subf r29, r4, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82FF910C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FF9110: 40990068  ble cr6, 0x82ff9178
	if !ctx.cr[6].gt {
	pc = 0x82FF9178; continue 'dispatch;
	}
	// 82FF9114: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FF9118: 481AF3F9  bl 0x831a8510
	ctx.lr = 0x82FF911C;
	sub_831A8510(ctx, base);
	// 82FF911C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9120: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82FF9124: 7FBDE214  add r29, r29, r28
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82FF9128: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FF912C: 41980034  blt cr6, 0x82ff9160
	if ctx.cr[6].lt {
	pc = 0x82FF9160; continue 'dispatch;
	}
	// 82FF9130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9134: 4BFFF925  bl 0x82ff8a58
	ctx.lr = 0x82FF9138;
	sub_82FF8A58(ctx, base);
	// 82FF9138: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF913C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9140: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9144: 481AF3CD  bl 0x831a8510
	ctx.lr = 0x82FF9148;
	sub_831A8510(ctx, base);
	// 82FF9148: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF914C: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FF9150: 7FCBF050  subf r30, r11, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82FF9154: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FF9158: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF915C: 4098FFD4  bge cr6, 0x82ff9130
	if !ctx.cr[6].lt {
	pc = 0x82FF9130; continue 'dispatch;
	}
	// 82FF9160: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF9164: 419A0028  beq cr6, 0x82ff918c
	if ctx.cr[6].eq {
	pc = 0x82FF918C; continue 'dispatch;
	}
	// 82FF9168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF916C: 4BFFF8ED  bl 0x82ff8a58
	ctx.lr = 0x82FF9170;
	sub_82FF8A58(ctx, base);
	// 82FF9170: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9174: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9178: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FF917C: 481AF395  bl 0x831a8510
	ctx.lr = 0x82FF9180;
	sub_831A8510(ctx, base);
	// 82FF9180: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9184: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FF9188: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF918C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF9190: 481AF028  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9198 size=100
    let mut pc: u32 = 0x82FF9198;
    'dispatch: loop {
        match pc {
            0x82FF9198 => {
    //   block [0x82FF9198..0x82FF91FC)
	// 82FF9198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF91A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF91A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF91A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF91AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF91B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF91B4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF91B8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF91BC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF91C0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF91C4: 41980008  blt cr6, 0x82ff91cc
	if ctx.cr[6].lt {
	pc = 0x82FF91CC; continue 'dispatch;
	}
	// 82FF91C8: 4BFFFA81  bl 0x82ff8c48
	ctx.lr = 0x82FF91CC;
	sub_82FF8C48(ctx, base);
	// 82FF91CC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF91D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF91D4: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FF91D8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF91DC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF91E0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF91E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF91E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF91EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF91F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF91F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF91F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9200 size=100
    let mut pc: u32 = 0x82FF9200;
    'dispatch: loop {
        match pc {
            0x82FF9200 => {
    //   block [0x82FF9200..0x82FF9264)
	// 82FF9200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF920C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF921C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9220: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FF9224: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF9228: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF922C: 41980008  blt cr6, 0x82ff9234
	if ctx.cr[6].lt {
	pc = 0x82FF9234; continue 'dispatch;
	}
	// 82FF9230: 4BFFFA19  bl 0x82ff8c48
	ctx.lr = 0x82FF9234;
	sub_82FF8C48(ctx, base);
	// 82FF9234: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF923C: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FF9240: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9244: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF9248: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF924C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF925C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9268 size=140
    let mut pc: u32 = 0x82FF9268;
    'dispatch: loop {
        match pc {
            0x82FF9268 => {
    //   block [0x82FF9268..0x82FF92F4)
	// 82FF9268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF927C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9284: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9288: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF928C: 4082000C  bne 0x82ff9298
	if !ctx.cr[0].eq {
	pc = 0x82FF9298; continue 'dispatch;
	}
	// 82FF9290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9294: 48000008  b 0x82ff929c
	pc = 0x82FF929C; continue 'dispatch;
	// 82FF9298: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF929C: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 82FF92A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF92A4: 4BFFFA1D  bl 0x82ff8cc0
	ctx.lr = 0x82FF92A8;
	sub_82FF8CC0(ctx, base);
	// 82FF92A8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF92AC: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF92B0: 4082000C  bne 0x82ff92bc
	if !ctx.cr[0].eq {
	pc = 0x82FF92BC; continue 'dispatch;
	}
	// 82FF92B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF92B8: 48000008  b 0x82ff92c0
	pc = 0x82FF92C0; continue 'dispatch;
	// 82FF92BC: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF92C0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF92C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF92C8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF92CC: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FF92D0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF92D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF92D8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF92DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF92E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF92E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF92E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF92EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF92F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF92F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF92F8 size=140
    let mut pc: u32 = 0x82FF92F8;
    'dispatch: loop {
        match pc {
            0x82FF92F8 => {
    //   block [0x82FF92F8..0x82FF9384)
	// 82FF92F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF92FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF930C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9310: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9314: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9318: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF931C: 4082000C  bne 0x82ff9328
	if !ctx.cr[0].eq {
	pc = 0x82FF9328; continue 'dispatch;
	}
	// 82FF9320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9324: 48000008  b 0x82ff932c
	pc = 0x82FF932C; continue 'dispatch;
	// 82FF9328: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF932C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82FF9330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9334: 4BFFF98D  bl 0x82ff8cc0
	ctx.lr = 0x82FF9338;
	sub_82FF8CC0(ctx, base);
	// 82FF9338: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF933C: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9340: 4082000C  bne 0x82ff934c
	if !ctx.cr[0].eq {
	pc = 0x82FF934C; continue 'dispatch;
	}
	// 82FF9344: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9348: 48000008  b 0x82ff9350
	pc = 0x82FF9350; continue 'dispatch;
	// 82FF934C: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF9350: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF9354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9358: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF935C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FF9360: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9364: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FF9368: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF936C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9378: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF937C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9388 size=140
    let mut pc: u32 = 0x82FF9388;
    'dispatch: loop {
        match pc {
            0x82FF9388 => {
    //   block [0x82FF9388..0x82FF9414)
	// 82FF9388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9394: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82FF9398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF939C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF93A0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82FF93A4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF93A8: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF93AC: 4082000C  bne 0x82ff93b8
	if !ctx.cr[0].eq {
	pc = 0x82FF93B8; continue 'dispatch;
	}
	// 82FF93B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF93B4: 48000008  b 0x82ff93bc
	pc = 0x82FF93BC; continue 'dispatch;
	// 82FF93B8: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF93BC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82FF93C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF93C4: 4BFFF8FD  bl 0x82ff8cc0
	ctx.lr = 0x82FF93C8;
	sub_82FF8CC0(ctx, base);
	// 82FF93C8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF93CC: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF93D0: 4082000C  bne 0x82ff93dc
	if !ctx.cr[0].eq {
	pc = 0x82FF93DC; continue 'dispatch;
	}
	// 82FF93D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF93D8: 48000008  b 0x82ff93e0
	pc = 0x82FF93E0; continue 'dispatch;
	// 82FF93DC: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF93E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF93E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF93E8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF93EC: DBEB0000  stfd f31, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[31].u64 ) };
	// 82FF93F0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF93F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FF93F8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF93FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9408: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF940C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9418 size=104
    let mut pc: u32 = 0x82FF9418;
    'dispatch: loop {
        match pc {
            0x82FF9418 => {
    //   block [0x82FF9418..0x82FF9480)
	// 82FF9418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF941C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF942C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9430: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9434: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9438: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF943C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF9440: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF9444: 40990008  ble cr6, 0x82ff944c
	if !ctx.cr[6].gt {
	pc = 0x82FF944C; continue 'dispatch;
	}
	// 82FF9448: 4BFFF611  bl 0x82ff8a58
	ctx.lr = 0x82FF944C;
	sub_82FF8A58(ctx, base);
	// 82FF944C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9454: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9458: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FF945C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9460: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF9464: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF9468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9474: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9480 size=104
    let mut pc: u32 = 0x82FF9480;
    'dispatch: loop {
        match pc {
            0x82FF9480 => {
    //   block [0x82FF9480..0x82FF94E8)
	// 82FF9480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF948C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF949C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF94A0: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FF94A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF94A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FF94AC: 40990008  ble cr6, 0x82ff94b4
	if !ctx.cr[6].gt {
	pc = 0x82FF94B4; continue 'dispatch;
	}
	// 82FF94B0: 4BFFF5A9  bl 0x82ff8a58
	ctx.lr = 0x82FF94B4;
	sub_82FF8A58(ctx, base);
	// 82FF94B4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF94B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF94BC: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF94C0: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FF94C4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF94C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF94CC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF94D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF94D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF94D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF94DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF94E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF94E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF94E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF94E8 size=144
    let mut pc: u32 = 0x82FF94E8;
    'dispatch: loop {
        match pc {
            0x82FF94E8 => {
    //   block [0x82FF94E8..0x82FF9578)
	// 82FF94E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF94EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF94F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF94F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF94F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF94FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9504: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9508: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF950C: 4082000C  bne 0x82ff9518
	if !ctx.cr[0].eq {
	pc = 0x82FF9518; continue 'dispatch;
	}
	// 82FF9510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9514: 48000008  b 0x82ff951c
	pc = 0x82FF951C; continue 'dispatch;
	// 82FF9518: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF951C: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 82FF9520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9524: 4BFFF875  bl 0x82ff8d98
	ctx.lr = 0x82FF9528;
	sub_82FF8D98(ctx, base);
	// 82FF9528: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF952C: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9530: 4082000C  bne 0x82ff953c
	if !ctx.cr[0].eq {
	pc = 0x82FF953C; continue 'dispatch;
	}
	// 82FF9534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9538: 48000008  b 0x82ff9540
	pc = 0x82FF9540; continue 'dispatch;
	// 82FF953C: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF9540: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF9544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9548: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF954C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9550: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FF9554: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9558: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF955C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF9560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF956C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9578 size=144
    let mut pc: u32 = 0x82FF9578;
    'dispatch: loop {
        match pc {
            0x82FF9578 => {
    //   block [0x82FF9578..0x82FF9608)
	// 82FF9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF958C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9590: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9594: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9598: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF959C: 4082000C  bne 0x82ff95a8
	if !ctx.cr[0].eq {
	pc = 0x82FF95A8; continue 'dispatch;
	}
	// 82FF95A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF95A4: 48000008  b 0x82ff95ac
	pc = 0x82FF95AC; continue 'dispatch;
	// 82FF95A8: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF95AC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82FF95B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF95B4: 4BFFF7E5  bl 0x82ff8d98
	ctx.lr = 0x82FF95B8;
	sub_82FF8D98(ctx, base);
	// 82FF95B8: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF95BC: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF95C0: 4082000C  bne 0x82ff95cc
	if !ctx.cr[0].eq {
	pc = 0x82FF95CC; continue 'dispatch;
	}
	// 82FF95C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF95C8: 48000008  b 0x82ff95d0
	pc = 0x82FF95D0; continue 'dispatch;
	// 82FF95CC: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF95D0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF95D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF95D8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF95DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF95E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF95E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF95E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FF95EC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF95F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF95F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF95F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF95FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9608 size=144
    let mut pc: u32 = 0x82FF9608;
    'dispatch: loop {
        match pc {
            0x82FF9608 => {
    //   block [0x82FF9608..0x82FF9698)
	// 82FF9608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF960C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF961C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9620: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9624: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9628: 556B077F  clrlwi. r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF962C: 4082000C  bne 0x82ff9638
	if !ctx.cr[0].eq {
	pc = 0x82FF9638; continue 'dispatch;
	}
	// 82FF9630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9634: 48000008  b 0x82ff963c
	pc = 0x82FF963C; continue 'dispatch;
	// 82FF9638: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF963C: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82FF9640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9644: 4BFFF755  bl 0x82ff8d98
	ctx.lr = 0x82FF9648;
	sub_82FF8D98(ctx, base);
	// 82FF9648: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF964C: 554B077F  clrlwi. r11, r10, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9650: 4082000C  bne 0x82ff965c
	if !ctx.cr[0].eq {
	pc = 0x82FF965C; continue 'dispatch;
	}
	// 82FF9654: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9658: 48000008  b 0x82ff9660
	pc = 0x82FF9660; continue 'dispatch;
	// 82FF965C: 216B0008  subfic r11, r11, 8
	ctx.xer.ca = ctx.r[11].u32 <= 8 as u32;
	ctx.r[11].s64 = (8 as i64) - ctx.r[11].s64;
	// 82FF9660: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FF9664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9668: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF966C: C80B0000  lfd f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82FF9670: D81E0000  stfd f0, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82FF9674: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FF9678: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FF967C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FF9680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF968C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9698 size=148
    let mut pc: u32 = 0x82FF9698;
    'dispatch: loop {
        match pc {
            0x82FF9698 => {
    //   block [0x82FF9698..0x82FF972C)
	// 82FF9698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF969C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF96A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF96A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF96A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF96AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF96B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF96B4: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FF96B8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF96BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF96C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF96C4: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF96C8: 4BFFEA69  bl 0x82ff8130
	ctx.lr = 0x82FF96CC;
	sub_82FF8130(ctx, base);
	// 82FF96CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FF96D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF96D4: 409A0014  bne cr6, 0x82ff96e8
	if !ctx.cr[6].eq {
	pc = 0x82FF96E8; continue 'dispatch;
	}
	// 82FF96D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF96DC: 4BFFFC1D  bl 0x82ff92f8
	ctx.lr = 0x82FF96E0;
	sub_82FF92F8(ctx, base);
	// 82FF96E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF96E4: 48000030  b 0x82ff9714
	pc = 0x82FF9714; continue 'dispatch;
	// 82FF96E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF96EC: 4BFFF1A5  bl 0x82ff8890
	ctx.lr = 0x82FF96F0;
	sub_82FF8890(ctx, base);
	// 82FF96F0: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FF96F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF96F8: 4082FFE4  bne 0x82ff96dc
	if !ctx.cr[0].eq {
	pc = 0x82FF96DC; continue 'dispatch;
	}
	// 82FF96FC: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82FF9700: 4BFFFBF9  bl 0x82ff92f8
	ctx.lr = 0x82FF9704;
	sub_82FF92F8(ctx, base);
	// 82FF9704: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF970C: 4BFFF1CD  bl 0x82ff88d8
	ctx.lr = 0x82FF9710;
	sub_82FF88D8(ctx, base);
	// 82FF9710: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF9714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF971C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9730 size=132
    let mut pc: u32 = 0x82FF9730;
    'dispatch: loop {
        match pc {
            0x82FF9730 => {
    //   block [0x82FF9730..0x82FF97B4)
	// 82FF9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF973C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9748: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF974C: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF9750: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF9754: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF9758: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF975C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF9760: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF9764: 4BFFE9CD  bl 0x82ff8130
	ctx.lr = 0x82FF9768;
	sub_82FF8130(ctx, base);
	// 82FF9768: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF976C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9770: 4BFFFE09  bl 0x82ff9578
	ctx.lr = 0x82FF9774;
	sub_82FF9578(ctx, base);
	// 82FF9774: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF9778: 2F04FFFE  cmpwi cr6, r4, -2
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2, &mut ctx.xer);
	// 82FF977C: 409A000C  bne cr6, 0x82ff9788
	if !ctx.cr[6].eq {
	pc = 0x82FF9788; continue 'dispatch;
	}
	// 82FF9780: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF9784: 48000018  b 0x82ff979c
	pc = 0x82FF979C; continue 'dispatch;
	// 82FF9788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF978C: 4BFFEA0D  bl 0x82ff8198
	ctx.lr = 0x82FF9790;
	sub_82FF8198(ctx, base);
	// 82FF9790: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF9794: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF9798: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF979C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF97A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF97A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF97A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF97AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF97B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF97B8 size=168
    let mut pc: u32 = 0x82FF97B8;
    'dispatch: loop {
        match pc {
            0x82FF97B8 => {
    //   block [0x82FF97B8..0x82FF9860)
	// 82FF97B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF97BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF97C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF97C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF97C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF97CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF97D0: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF97D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF97D8: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF97DC: 41820028  beq 0x82ff9804
	if ctx.cr[0].eq {
	pc = 0x82FF9804; continue 'dispatch;
	}
	// 82FF97E0: 4BFFF469  bl 0x82ff8c48
	ctx.lr = 0x82FF97E4;
	sub_82FF8C48(ctx, base);
	// 82FF97E4: 83FE0024  lwz r31, 0x24(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FF97E8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF97EC: 4182002C  beq 0x82ff9818
	if ctx.cr[0].eq {
	pc = 0x82FF9818; continue 'dispatch;
	}
	// 82FF97F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF97F4: 4808BBAD  bl 0x830853a0
	ctx.lr = 0x82FF97F8;
	sub_830853A0(ctx, base);
	// 82FF97F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF97FC: 4BFDEAE5  bl 0x82fd82e0
	ctx.lr = 0x82FF9800;
	sub_82FD82E0(ctx, base);
	// 82FF9800: 48000018  b 0x82ff9818
	pc = 0x82FF9818; continue 'dispatch;
	// 82FF9804: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF9808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF980C: 4182000C  beq 0x82ff9818
	if ctx.cr[0].eq {
	pc = 0x82FF9818; continue 'dispatch;
	}
	// 82FF9810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FF9814: 480A0F05  bl 0x8309a718
	ctx.lr = 0x82FF9818;
	sub_8309A718(ctx, base);
	// 82FF9818: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF981C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9820: 4182000C  beq 0x82ff982c
	if ctx.cr[0].eq {
	pc = 0x82FF982C; continue 'dispatch;
	}
	// 82FF9824: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9828: 4800000C  b 0x82ff9834
	pc = 0x82FF9834; continue 'dispatch;
	// 82FF982C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF9830: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9834: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9838: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FF983C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9844: 4E800421  bctrl
	ctx.lr = 0x82FF9848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF984C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9854: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF985C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9860 size=160
    let mut pc: u32 = 0x82FF9860;
    'dispatch: loop {
        match pc {
            0x82FF9860 => {
    //   block [0x82FF9860..0x82FF9900)
	// 82FF9860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF986C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9878: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF987C: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FF9880: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF9884: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF9888: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF988C: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF9890: 4BFFE8A1  bl 0x82ff8130
	ctx.lr = 0x82FF9894;
	sub_82FF8130(ctx, base);
	// 82FF9894: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF989C: 4BFFECFD  bl 0x82ff8598
	ctx.lr = 0x82FF98A0;
	sub_82FF8598(ctx, base);
	// 82FF98A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF98A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF98A8: 4BFFEFE9  bl 0x82ff8890
	ctx.lr = 0x82FF98AC;
	sub_82FF8890(ctx, base);
	// 82FF98AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF98B0: 41820014  beq 0x82ff98c4
	if ctx.cr[0].eq {
	pc = 0x82FF98C4; continue 'dispatch;
	}
	// 82FF98B4: 64648000  oris r4, r3, 0x8000
	ctx.r[4].u64 = ctx.r[3].u64 | 2147483648;
	// 82FF98B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF98BC: 4BFFFA3D  bl 0x82ff92f8
	ctx.lr = 0x82FF98C0;
	sub_82FF92F8(ctx, base);
	// 82FF98C0: 48000028  b 0x82ff98e8
	pc = 0x82FF98E8; continue 'dispatch;
	// 82FF98C4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82FF98C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF98CC: 4BFFFA2D  bl 0x82ff92f8
	ctx.lr = 0x82FF98D0;
	sub_82FF92F8(ctx, base);
	// 82FF98D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF98D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF98D8: 4803F0B9  bl 0x83038990
	ctx.lr = 0x82FF98DC;
	sub_83038990(ctx, base);
	// 82FF98DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF98E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF98E4: 4BFFEFF5  bl 0x82ff88d8
	ctx.lr = 0x82FF98E8;
	sub_82FF88D8(ctx, base);
	// 82FF98E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF98EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF98F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF98F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF98F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF98FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9900 size=148
    let mut pc: u32 = 0x82FF9900;
    'dispatch: loop {
        match pc {
            0x82FF9900 => {
    //   block [0x82FF9900..0x82FF9994)
	// 82FF9900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9904: 481AE869  bl 0x831a816c
	ctx.lr = 0x82FF9908;
	sub_831A8130(ctx, base);
	// 82FF9908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF990C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF9910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF9914: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FF9918: 419A0068  beq cr6, 0x82ff9980
	if ctx.cr[6].eq {
	pc = 0x82FF9980; continue 'dispatch;
	}
	// 82FF991C: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9920: 4182000C  beq 0x82ff992c
	if ctx.cr[0].eq {
	pc = 0x82FF992C; continue 'dispatch;
	}
	// 82FF9924: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FF9928: 4BFFF9D1  bl 0x82ff92f8
	ctx.lr = 0x82FF992C;
	sub_82FF92F8(ctx, base);
	// 82FF992C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9934: 41820028  beq 0x82ff995c
	if ctx.cr[0].eq {
	pc = 0x82FF995C; continue 'dispatch;
	}
	// 82FF9938: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FF993C: 48000008  b 0x82ff9944
	pc = 0x82FF9944; continue 'dispatch;
	// 82FF9940: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FF9944: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9948: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF994C: 4082FFF4  bne 0x82ff9940
	if !ctx.cr[0].eq {
	pc = 0x82FF9940; continue 'dispatch;
	}
	// 82FF9950: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FF9954: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FF9958: 48000008  b 0x82ff9960
	pc = 0x82FF9960; continue 'dispatch;
	// 82FF995C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FF9960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9964: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9968: 4BFFF991  bl 0x82ff92f8
	ctx.lr = 0x82FF996C;
	sub_82FF92F8(ctx, base);
	// 82FF996C: 57C5083C  slwi r5, r30, 1
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FF9970: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF9974: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9978: 4BFFF649  bl 0x82ff8fc0
	ctx.lr = 0x82FF997C;
	sub_82FF8FC0(ctx, base);
	// 82FF997C: 48000010  b 0x82ff998c
	pc = 0x82FF998C; continue 'dispatch;
	// 82FF9980: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82FF9984: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9988: 4BFFF971  bl 0x82ff92f8
	ctx.lr = 0x82FF998C;
	sub_82FF92F8(ctx, base);
	// 82FF998C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9990: 481AE82C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9998 size=396
    let mut pc: u32 = 0x82FF9998;
    'dispatch: loop {
        match pc {
            0x82FF9998 => {
    //   block [0x82FF9998..0x82FF9B24)
	// 82FF9998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF999C: 481AE7D1  bl 0x831a816c
	ctx.lr = 0x82FF99A0;
	sub_831A8130(ctx, base);
	// 82FF99A0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF99A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF99A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF99AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FF99B0: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF99B4: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF99B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF99BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF99C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF99C4: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF99C8: 4BFFE769  bl 0x82ff8130
	ctx.lr = 0x82FF99CC;
	sub_82FF8130(ctx, base);
	// 82FF99CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF99D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF99D4: 4BFFEBC5  bl 0x82ff8598
	ctx.lr = 0x82FF99D8;
	sub_82FF8598(ctx, base);
	// 82FF99D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82FF99DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF99E0: 4BFFFB99  bl 0x82ff9578
	ctx.lr = 0x82FF99E4;
	sub_82FF9578(ctx, base);
	// 82FF99E4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FF99E8: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF99EC: 40820014  bne 0x82ff9a00
	if !ctx.cr[0].eq {
	pc = 0x82FF9A00; continue 'dispatch;
	}
	// 82FF99F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FF99F4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF99F8: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82FF99FC: 481AE7C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FF9A00: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FF9A04: 409A003C  bne cr6, 0x82ff9a40
	if !ctx.cr[6].eq {
	pc = 0x82FF9A40; continue 'dispatch;
	}
	// 82FF9A08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9A0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9A10: 4182000C  beq 0x82ff9a1c
	if ctx.cr[0].eq {
	pc = 0x82FF9A1C; continue 'dispatch;
	}
	// 82FF9A14: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9A18: 4800000C  b 0x82ff9a24
	pc = 0x82FF9A24; continue 'dispatch;
	// 82FF9A1C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF9A20: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9A28: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9A2C: 4803EFAD  bl 0x830389d8
	ctx.lr = 0x82FF9A30;
	sub_830389D8(ctx, base);
	// 82FF9A30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9A38: 4BFFEF11  bl 0x82ff8948
	ctx.lr = 0x82FF9A3C;
	sub_82FF8948(ctx, base);
	// 82FF9A3C: 48000034  b 0x82ff9a70
	pc = 0x82FF9A70; continue 'dispatch;
	// 82FF9A40: 5563007F  clrlwi. r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FF9A44: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FF9A48: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9A4C: 4182002C  beq 0x82ff9a78
	if ctx.cr[0].eq {
	pc = 0x82FF9A78; continue 'dispatch;
	}
	// 82FF9A50: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FF9A54: 41990024  bgt cr6, 0x82ff9a78
	if ctx.cr[6].gt {
	pc = 0x82FF9A78; continue 'dispatch;
	}
	// 82FF9A58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF9A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9A60: 4BFFE739  bl 0x82ff8198
	ctx.lr = 0x82FF9A64;
	sub_82FF8198(ctx, base);
	// 82FF9A64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF9A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9A6C: 4BFFEB2D  bl 0x82ff8598
	ctx.lr = 0x82FF9A70;
	sub_82FF8598(ctx, base);
	// 82FF9A70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FF9A74: 4BFFFF84  b 0x82ff99f8
	pc = 0x82FF99F8; continue 'dispatch;
	// 82FF9A78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9A7C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FF9A80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9A84: 4182000C  beq 0x82ff9a90
	if ctx.cr[0].eq {
	pc = 0x82FF9A90; continue 'dispatch;
	}
	// 82FF9A88: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9A8C: 48000008  b 0x82ff9a94
	pc = 0x82FF9A94; continue 'dispatch;
	// 82FF9A90: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9A94: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF9A98: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF9A9C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82FF9AA0: 4BFD7DC9  bl 0x82fd1868
	ctx.lr = 0x82FF9AA4;
	sub_82FD1868(ctx, base);
	// 82FF9AA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9AA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9AAC: 4182000C  beq 0x82ff9ab8
	if ctx.cr[0].eq {
	pc = 0x82FF9AB8; continue 'dispatch;
	}
	// 82FF9AB0: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9AB4: 48000008  b 0x82ff9abc
	pc = 0x82FF9ABC; continue 'dispatch;
	// 82FF9AB8: 80FEB7E8  lwz r7, -0x4818(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9ABC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FF9AC0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FF9AC4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82FF9AC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9ACC: 4BFD7D9D  bl 0x82fd1868
	ctx.lr = 0x82FF9AD0;
	sub_82FD1868(ctx, base);
	// 82FF9AD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9AD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9AD8: 4182000C  beq 0x82ff9ae4
	if ctx.cr[0].eq {
	pc = 0x82FF9AE4; continue 'dispatch;
	}
	// 82FF9ADC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9AE0: 48000008  b 0x82ff9ae8
	pc = 0x82FF9AE8; continue 'dispatch;
	// 82FF9AE4: 815EB7E8  lwz r10, -0x4818(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9AE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9AEC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FF9AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF9AF4: 388BF72C  addi r4, r11, -0x8d4
	ctx.r[4].s64 = ctx.r[11].s64 + -2260;
	// 82FF9AF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF9AFC: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82FF9B00: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82FF9B04: 38C00184  li r6, 0x184
	ctx.r[6].s64 = 388;
	// 82FF9B08: 38A001EE  li r5, 0x1ee
	ctx.r[5].s64 = 494;
	// 82FF9B0C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF9B10: 4BFFE3E9  bl 0x82ff7ef8
	ctx.lr = 0x82FF9B14;
	sub_82FF7EF8(ctx, base);
	// 82FF9B14: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FF9B18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FF9B1C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82FF9B20: 481B7109  bl 0x831b0c28
	ctx.lr = 0x82FF9B24;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9B28 size=216
    let mut pc: u32 = 0x82FF9B28;
    'dispatch: loop {
        match pc {
            0x82FF9B28 => {
    //   block [0x82FF9B28..0x82FF9C00)
	// 82FF9B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9B2C: 481AE639  bl 0x831a8164
	ctx.lr = 0x82FF9B30;
	sub_831A8130(ctx, base);
	// 82FF9B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9B34: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FF9B38: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FF9B3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF9B40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FF9B44: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FF9B48: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FF9B4C: 4BFFFA2D  bl 0x82ff9578
	ctx.lr = 0x82FF9B50;
	sub_82FF9578(ctx, base);
	// 82FF9B50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9B54: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FF9B58: 409A0018  bne cr6, 0x82ff9b70
	if !ctx.cr[6].eq {
	pc = 0x82FF9B70; continue 'dispatch;
	}
	// 82FF9B5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FF9B60: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9B64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9B68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9B6C: 4800008C  b 0x82ff9bf8
	pc = 0x82FF9BF8; continue 'dispatch;
	// 82FF9B70: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF9B74: 41820014  beq 0x82ff9b88
	if ctx.cr[0].eq {
	pc = 0x82FF9B88; continue 'dispatch;
	}
	// 82FF9B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9B80: 4BFFF9F9  bl 0x82ff9578
	ctx.lr = 0x82FF9B84;
	sub_82FF9578(ctx, base);
	// 82FF9B84: 48000014  b 0x82ff9b98
	pc = 0x82FF9B98; continue 'dispatch;
	// 82FF9B88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9B8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9B90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FF9B94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9B98: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9B9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9BA0: 4182000C  beq 0x82ff9bac
	if ctx.cr[0].eq {
	pc = 0x82FF9BAC; continue 'dispatch;
	}
	// 82FF9BA4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9BA8: 4800000C  b 0x82ff9bb4
	pc = 0x82FF9BB4; continue 'dispatch;
	// 82FF9BAC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF9BB0: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9BB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9BB8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9BBC: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FF9BC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9BC8: 4E800421  bctrl
	ctx.lr = 0x82FF9BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9BCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF9BD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FF9BD4: 909C0000  stw r4, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FF9BD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9BDC: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FF9BE0: 4BFFF4C1  bl 0x82ff90a0
	ctx.lr = 0x82FF9BE4;
	sub_82FF90A0(ctx, base);
	// 82FF9BE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9BE8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF9BF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FF9BF4: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 82FF9BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FF9BFC: 481AE5B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9C00 size=192
    let mut pc: u32 = 0x82FF9C00;
    'dispatch: loop {
        match pc {
            0x82FF9C00 => {
    //   block [0x82FF9C00..0x82FF9CC0)
	// 82FF9C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9C14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF9C18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FF9C1C: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FF9C20: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF9C24: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF9C28: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF9C2C: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF9C30: 4BFFE501  bl 0x82ff8130
	ctx.lr = 0x82FF9C34;
	sub_82FF8130(ctx, base);
	// 82FF9C34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FF9C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9C3C: 409A000C  bne cr6, 0x82ff9c48
	if !ctx.cr[6].eq {
	pc = 0x82FF9C48; continue 'dispatch;
	}
	// 82FF9C40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FF9C44: 48000018  b 0x82ff9c5c
	pc = 0x82FF9C5C; continue 'dispatch;
	// 82FF9C48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF9C4C: 4BFFEC45  bl 0x82ff8890
	ctx.lr = 0x82FF9C50;
	sub_82FF8890(ctx, base);
	// 82FF9C50: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FF9C54: 41820010  beq 0x82ff9c64
	if ctx.cr[0].eq {
	pc = 0x82FF9C64; continue 'dispatch;
	}
	// 82FF9C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9C5C: 4BFFF69D  bl 0x82ff92f8
	ctx.lr = 0x82FF9C60;
	sub_82FF92F8(ctx, base);
	// 82FF9C60: 48000048  b 0x82ff9ca8
	pc = 0x82FF9CA8; continue 'dispatch;
	// 82FF9C64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9C6C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9C70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9C74: 4E800421  bctrl
	ctx.lr = 0x82FF9C78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9C78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FF9C7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9C80: 4BFFFBE1  bl 0x82ff9860
	ctx.lr = 0x82FF9C84;
	sub_82FF9860(ctx, base);
	// 82FF9C84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF9C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9C8C: 4BFFEC4D  bl 0x82ff88d8
	ctx.lr = 0x82FF9C90;
	sub_82FF88D8(ctx, base);
	// 82FF9C90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9C94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9C9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9CA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9CA4: 4E800421  bctrl
	ctx.lr = 0x82FF9CA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9CA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9CB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9CB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9CC0 size=244
    let mut pc: u32 = 0x82FF9CC0;
    'dispatch: loop {
        match pc {
            0x82FF9CC0 => {
    //   block [0x82FF9CC0..0x82FF9DB4)
	// 82FF9CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9CD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9CD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9CDC: 38A00182  li r5, 0x182
	ctx.r[5].s64 = 386;
	// 82FF9CE0: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FF9CE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FF9CE8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FF9CEC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF9CF0: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FF9CF4: 4BFFE43D  bl 0x82ff8130
	ctx.lr = 0x82FF9CF8;
	sub_82FF8130(ctx, base);
	// 82FF9CF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9D00: 4BFFE899  bl 0x82ff8598
	ctx.lr = 0x82FF9D04;
	sub_82FF8598(ctx, base);
	// 82FF9D04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FF9D08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9D10: 4BFFFC89  bl 0x82ff9998
	ctx.lr = 0x82FF9D14;
	sub_82FF9998(ctx, base);
	// 82FF9D14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9D18: 40820018  bne 0x82ff9d30
	if !ctx.cr[0].eq {
	pc = 0x82FF9D30; continue 'dispatch;
	}
	// 82FF9D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9D20: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FF9D24: 4BFFE475  bl 0x82ff8198
	ctx.lr = 0x82FF9D28;
	sub_82FF8198(ctx, base);
	// 82FF9D28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF9D2C: 4800006C  b 0x82ff9d98
	pc = 0x82FF9D98; continue 'dispatch;
	// 82FF9D30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9D34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9D38: 4182000C  beq 0x82ff9d44
	if ctx.cr[0].eq {
	pc = 0x82FF9D44; continue 'dispatch;
	}
	// 82FF9D3C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9D40: 4800000C  b 0x82ff9d4c
	pc = 0x82FF9D4C; continue 'dispatch;
	// 82FF9D44: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FF9D48: 806BB7E8  lwz r3, -0x4818(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FF9D4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9D54: 4E800421  bctrl
	ctx.lr = 0x82FF9D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9D58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FF9D5C: 38A0018A  li r5, 0x18a
	ctx.r[5].s64 = 394;
	// 82FF9D60: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82FF9D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9D68: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FF9D6C: 69640001  xori r4, r11, 1
	ctx.r[4].u64 = ctx.r[11].u64 ^ 1;
	// 82FF9D70: 4BFFE3C1  bl 0x82ff8130
	ctx.lr = 0x82FF9D74;
	sub_82FF8130(ctx, base);
	// 82FF9D74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FF9D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9D7C: 4BFFEBCD  bl 0x82ff8948
	ctx.lr = 0x82FF9D80;
	sub_82FF8948(ctx, base);
	// 82FF9D80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9D84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FF9D88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9D8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9D94: 4E800421  bctrl
	ctx.lr = 0x82FF9D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FF9D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9DB8 size=16
    let mut pc: u32 = 0x82FF9DB8;
    'dispatch: loop {
        match pc {
            0x82FF9DB8 => {
    //   block [0x82FF9DB8..0x82FF9DC8)
	// 82FF9DB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9DBC: 396BF810  addi r11, r11, -0x7f0
	ctx.r[11].s64 = ctx.r[11].s64 + -2032;
	// 82FF9DC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9DC8 size=68
    let mut pc: u32 = 0x82FF9DC8;
    'dispatch: loop {
        match pc {
            0x82FF9DC8 => {
    //   block [0x82FF9DC8..0x82FF9E0C)
	// 82FF9DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9DD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9DD8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9DE0: 396BF810  addi r11, r11, -0x7f0
	ctx.r[11].s64 = ctx.r[11].s64 + -2032;
	// 82FF9DE4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FF9DE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9DEC: 41820008  beq 0x82ff9df4
	if ctx.cr[0].eq {
	pc = 0x82FF9DF4; continue 'dispatch;
	}
	// 82FF9DF0: 4BFDE4F1  bl 0x82fd82e0
	ctx.lr = 0x82FF9DF4;
	sub_82FD82E0(ctx, base);
	// 82FF9DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF9DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9E10 size=24
    let mut pc: u32 = 0x82FF9E10;
    'dispatch: loop {
        match pc {
            0x82FF9E10 => {
    //   block [0x82FF9E10..0x82FF9E28)
	// 82FF9E10: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9E14: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9E18: 396BF81C  addi r11, r11, -0x7e4
	ctx.r[11].s64 = ctx.r[11].s64 + -2020;
	// 82FF9E1C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9E20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9E24: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9E28 size=12
    let mut pc: u32 = 0x82FF9E28;
    'dispatch: loop {
        match pc {
            0x82FF9E28 => {
    //   block [0x82FF9E28..0x82FF9E34)
	// 82FF9E28: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9E2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9E30: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9E34 size=20
    let mut pc: u32 = 0x82FF9E34;
    'dispatch: loop {
        match pc {
            0x82FF9E34 => {
    //   block [0x82FF9E34..0x82FF9E48)
	// 82FF9E34: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9E38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9E3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9E44: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9E48 size=4
    let mut pc: u32 = 0x82FF9E48;
    'dispatch: loop {
        match pc {
            0x82FF9E48 => {
    //   block [0x82FF9E48..0x82FF9E4C)
	// 82FF9E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9E50 size=128
    let mut pc: u32 = 0x82FF9E50;
    'dispatch: loop {
        match pc {
            0x82FF9E50 => {
    //   block [0x82FF9E50..0x82FF9ED0)
	// 82FF9E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FF9E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9E68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9E6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FF9E70: 396BF81C  addi r11, r11, -0x7e4
	ctx.r[11].s64 = ctx.r[11].s64 + -2020;
	// 82FF9E74: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9E78: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9E7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9E80: 41820024  beq 0x82ff9ea4
	if ctx.cr[0].eq {
	pc = 0x82FF9EA4; continue 'dispatch;
	}
	// 82FF9E84: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9E88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9E8C: 41820018  beq 0x82ff9ea4
	if ctx.cr[0].eq {
	pc = 0x82FF9EA4; continue 'dispatch;
	}
	// 82FF9E90: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9E94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FF9E98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FF9EA0: 4E800421  bctrl
	ctx.lr = 0x82FF9EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FF9EA4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9EA8: 4182000C  beq 0x82ff9eb4
	if ctx.cr[0].eq {
	pc = 0x82FF9EB4; continue 'dispatch;
	}
	// 82FF9EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9EB0: 4B2C63B9  bl 0x822c0268
	ctx.lr = 0x82FF9EB4;
	sub_822C0268(ctx, base);
	// 82FF9EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FF9EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9EC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FF9EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9ED0 size=148
    let mut pc: u32 = 0x82FF9ED0;
    'dispatch: loop {
        match pc {
            0x82FF9ED0 => {
    //   block [0x82FF9ED0..0x82FF9F64)
	// 82FF9ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9ED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9EDC: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FF9EE0: 9421EFA0  stwu r1, -0x1060(r1)
	ea = ctx.r[1].u32.wrapping_add(-4192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9EE4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9EE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9EEC: 7C830734  extsh r3, r4
	ctx.r[3].s64 = ctx.r[4].s16 as i64;
	// 82FF9EF0: 396BF81C  addi r11, r11, -0x7e4
	ctx.r[11].s64 = ctx.r[11].s64 + -2020;
	// 82FF9EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FF9EF8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FF9EFC: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82FF9F00: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82FF9F04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FF9F08: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82FF9F0C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FF9F10: 409A003C  bne cr6, 0x82ff9f4c
	if !ctx.cr[6].eq {
	pc = 0x82FF9F4C; continue 'dispatch;
	}
	// 82FF9F14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FF9F18: 38A007FF  li r5, 0x7ff
	ctx.r[5].s64 = 2047;
	// 82FF9F1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FF9F20: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82FF9F24: 4BFF5EBD  bl 0x82fefde0
	ctx.lr = 0x82FF9F28;
	sub_82FEFDE0(ctx, base);
	// 82FF9F28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FF9F2C: 4182000C  beq 0x82ff9f38
	if ctx.cr[0].eq {
	pc = 0x82FF9F38; continue 'dispatch;
	}
	// 82FF9F30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FF9F34: 4800000C  b 0x82ff9f40
	pc = 0x82FF9F40; continue 'dispatch;
	// 82FF9F38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9F3C: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 82FF9F40: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9F44: 4BFD6C3D  bl 0x82fd0b80
	ctx.lr = 0x82FF9F48;
	sub_82FD0B80(ctx, base);
	// 82FF9F48: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FF9F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9F50: 38211060  addi r1, r1, 0x1060
	ctx.r[1].s64 = ctx.r[1].s64 + 4192;
	// 82FF9F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9F5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FF9F68 size=132
    let mut pc: u32 = 0x82FF9F68;
    'dispatch: loop {
        match pc {
            0x82FF9F68 => {
    //   block [0x82FF9F68..0x82FF9FEC)
	// 82FF9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FF9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FF9F70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FF9F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FF9F78: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FF9F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FF9F80: 394AF81C  addi r10, r10, -0x7e4
	ctx.r[10].s64 = ctx.r[10].s64 + -2020;
	// 82FF9F84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FF9F88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FF9F8C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FF9F90: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FF9F94: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FF9F98: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FF9F9C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9FA0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FF9FA4: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9FA8: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82FF9FAC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FF9FB0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FF9FB4: 41820018  beq 0x82ff9fcc
	if ctx.cr[0].eq {
	pc = 0x82FF9FCC; continue 'dispatch;
	}
	// 82FF9FB8: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FF9FBC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9FC0: 4BFD6BC1  bl 0x82fd0b80
	ctx.lr = 0x82FF9FC4;
	sub_82FD0B80(ctx, base);
	// 82FF9FC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FF9FC8: 48000008  b 0x82ff9fd0
	pc = 0x82FF9FD0; continue 'dispatch;
	// 82FF9FCC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FF9FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FF9FD4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FF9FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FF9FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FF9FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FF9FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FF9FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FF9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FF9FF0 size=20
    let mut pc: u32 = 0x82FF9FF0;
    'dispatch: loop {
        match pc {
            0x82FF9FF0 => {
    //   block [0x82FF9FF0..0x82FFA004)
	// 82FF9FF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FF9FF4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FF9FF8: 396BF82C  addi r11, r11, -0x7d4
	ctx.r[11].s64 = ctx.r[11].s64 + -2004;
	// 82FF9FFC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFA000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA008 size=16
    let mut pc: u32 = 0x82FFA008;
    'dispatch: loop {
        match pc {
            0x82FFA008 => {
    //   block [0x82FFA008..0x82FFA018)
	// 82FFA008: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA00C: 396BF820  addi r11, r11, -0x7e0
	ctx.r[11].s64 = ctx.r[11].s64 + -2016;
	// 82FFA010: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFA014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA018 size=16
    let mut pc: u32 = 0x82FFA018;
    'dispatch: loop {
        match pc {
            0x82FFA018 => {
    //   block [0x82FFA018..0x82FFA028)
	// 82FFA018: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA01C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFA020: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA024: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA028 size=12
    let mut pc: u32 = 0x82FFA028;
    'dispatch: loop {
        match pc {
            0x82FFA028 => {
    //   block [0x82FFA028..0x82FFA034)
	// 82FFA028: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFA02C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA030: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA034 size=48
    let mut pc: u32 = 0x82FFA034;
    'dispatch: loop {
        match pc {
            0x82FFA034 => {
    //   block [0x82FFA034..0x82FFA064)
	// 82FFA034: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FFA038: A14AA6C0  lhz r10, -0x5940(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFA03C: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA040: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FFA044: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82FFA048: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82FFA04C: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82FFA050: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 82FFA054: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA058: 4182000C  beq 0x82ffa064
	if ctx.cr[0].eq {
		sub_82FFA064(ctx, base);
		return;
	}
	// 82FFA05C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFA060: 48000008  b 0x82ffa068
	sub_82FFA064(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA064(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA064 size=20
    let mut pc: u32 = 0x82FFA064;
    'dispatch: loop {
        match pc {
            0x82FFA064 => {
    //   block [0x82FFA064..0x82FFA078)
	// 82FFA064: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFA068: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA06C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA070: 4082FFCC  bne 0x82ffa03c
	if !ctx.cr[0].eq {
		sub_82FFA034(ctx, base);
		return;
	}
	// 82FFA074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA078 size=28
    let mut pc: u32 = 0x82FFA078;
    'dispatch: loop {
        match pc {
            0x82FFA078 => {
    //   block [0x82FFA078..0x82FFA094)
	// 82FFA078: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA07C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA080: 41820060  beq 0x82ffa0e0
	if ctx.cr[0].eq {
		sub_82FFA0E0(ctx, base);
		return;
	}
	// 82FFA084: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFA088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFA08C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFA090: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA094 size=16
    let mut pc: u32 = 0x82FFA094;
    'dispatch: loop {
        match pc {
            0x82FFA094 => {
    //   block [0x82FFA094..0x82FFA0A4)
	// 82FFA094: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA098: A12BA6C0  lhz r9, -0x5940(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFA09C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFA0A0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA0A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA0A4 size=60
    let mut pc: u32 = 0x82FFA0A4;
    'dispatch: loop {
        match pc {
            0x82FFA0A4 => {
    //   block [0x82FFA0A4..0x82FFA0E0)
	// 82FFA0A4: A1630008  lhz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA0A8: 5528043E  clrlwi r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82FFA0AC: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82FFA0B0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFA0B4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFA0B8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFA0BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA0C0: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82FFA0C4: 40820008  bne 0x82ffa0cc
	if !ctx.cr[0].eq {
	pc = 0x82FFA0CC; continue 'dispatch;
	}
	// 82FFA0C8: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 82FFA0CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFA0D0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA0D4: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FFA0D8: 4198FFC4  blt cr6, 0x82ffa09c
	if ctx.cr[6].lt {
		sub_82FFA094(ctx, base);
		return;
	}
	// 82FFA0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA0E0 size=8
    let mut pc: u32 = 0x82FFA0E0;
    'dispatch: loop {
        match pc {
            0x82FFA0E0 => {
    //   block [0x82FFA0E0..0x82FFA0E8)
	// 82FFA0E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFA0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA0E8 size=68
    let mut pc: u32 = 0x82FFA0E8;
    'dispatch: loop {
        match pc {
            0x82FFA0E8 => {
    //   block [0x82FFA0E8..0x82FFA12C)
	// 82FFA0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA0F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA0F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA0F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA100: 396BF820  addi r11, r11, -0x7e0
	ctx.r[11].s64 = ctx.r[11].s64 + -2016;
	// 82FFA104: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFA108: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFA10C: 41820008  beq 0x82ffa114
	if ctx.cr[0].eq {
	pc = 0x82FFA114; continue 'dispatch;
	}
	// 82FFA110: 4B2C6159  bl 0x822c0268
	ctx.lr = 0x82FFA114;
	sub_822C0268(ctx, base);
	// 82FFA114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFA118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFA128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA130 size=80
    let mut pc: u32 = 0x82FFA130;
    'dispatch: loop {
        match pc {
            0x82FFA130 => {
    //   block [0x82FFA130..0x82FFA180)
	// 82FFA130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA13C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA140: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FFA144: 807FB920  lwz r3, -0x46e0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18144 as u32) ) } as u64;
	// 82FFA148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFA14C: 419A0018  beq cr6, 0x82ffa164
	if ctx.cr[6].eq {
	pc = 0x82FFA164; continue 'dispatch;
	}
	// 82FFA150: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA154: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFA158: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA15C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA160: 4E800421  bctrl
	ctx.lr = 0x82FFA164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA164: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFA168: 917FB920  stw r11, -0x46e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18144 as u32), ctx.r[11].u32 ) };
	// 82FFA16C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFA17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA180 size=268
    //   switch @ 0x82FFA1EC: r11 with 10 label(s)
    //       case  0 → 0x82FFA1FC
    //       case  1 → 0x82FFA204
    //       case  2 → 0x82FFA20C
    //       case  3 → 0x82FFA214
    //       case  4 → 0x82FFA21C
    //       case  5 → 0x82FFA224
    //       case  6 → 0x82FFA22C
    //       case  7 → 0x82FFA234
    //       case  8 → 0x82FFA23C
    //       case  9 → 0x82FFA244
    let mut pc: u32 = 0x82FFA180;
    'dispatch: loop {
        match pc {
            0x82FFA180 => {
    //   block [0x82FFA180..0x82FFA1FC)
	// 82FFA180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA184: 481ADFE9  bl 0x831a816c
	ctx.lr = 0x82FFA188;
	sub_831A8130(ctx, base);
	// 82FFA188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA18C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFA190: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFA194: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 82FFA198: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFA19C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA1A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA1A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA1A8: 4E800421  bctrl
	ctx.lr = 0x82FFA1AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA1AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFA1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFA1B4: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82FFA1B8: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 82FFA1BC: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FFA1C0: 7D7F5396  divwu r11, r31, r10
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[10].u32;
	// 82FFA1C4: 1D6B000A  mulli r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 * 10;
	// 82FFA1C8: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82FFA1CC: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82FFA1D0: 41990080  bgt cr6, 0x82ffa250
	if ctx.cr[6].gt {
	pc = 0x82FFA250; continue 'dispatch;
	}
	// 82FFA1D4: 3D808214  lis r12, -0x7dec
	ctx.r[12].s64 = -2112618496;
	// 82FFA1D8: 398CF838  addi r12, r12, -0x7c8
	ctx.r[12].s64 = ctx.r[12].s64 + -1992;
	// 82FFA1DC: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFA1E0: 3D808300  lis r12, -0x7d00
	ctx.r[12].s64 = -2097152000;
	// 82FFA1E4: 398CA1FC  addi r12, r12, -0x5e04
	ctx.r[12].s64 = ctx.r[12].s64 + -24068;
	// 82FFA1E8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FFA1EC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FFA1F0: 60000000  nop
	// 82FFA1F4: 60000000  nop
	// 82FFA1F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82FFA1FC => {
    //   block [0x82FFA1FC..0x82FFA204)
	// 82FFA1FC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82FFA200: 48000048  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA204 => {
    //   block [0x82FFA204..0x82FFA20C)
	// 82FFA204: 39600031  li r11, 0x31
	ctx.r[11].s64 = 49;
	// 82FFA208: 48000040  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA20C => {
    //   block [0x82FFA20C..0x82FFA214)
	// 82FFA20C: 39600032  li r11, 0x32
	ctx.r[11].s64 = 50;
	// 82FFA210: 48000038  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA214 => {
    //   block [0x82FFA214..0x82FFA21C)
	// 82FFA214: 39600033  li r11, 0x33
	ctx.r[11].s64 = 51;
	// 82FFA218: 48000030  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA21C => {
    //   block [0x82FFA21C..0x82FFA224)
	// 82FFA21C: 39600034  li r11, 0x34
	ctx.r[11].s64 = 52;
	// 82FFA220: 48000028  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA224 => {
    //   block [0x82FFA224..0x82FFA22C)
	// 82FFA224: 39600035  li r11, 0x35
	ctx.r[11].s64 = 53;
	// 82FFA228: 48000020  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA22C => {
    //   block [0x82FFA22C..0x82FFA234)
	// 82FFA22C: 39600036  li r11, 0x36
	ctx.r[11].s64 = 54;
	// 82FFA230: 48000018  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA234 => {
    //   block [0x82FFA234..0x82FFA23C)
	// 82FFA234: 39600037  li r11, 0x37
	ctx.r[11].s64 = 55;
	// 82FFA238: 48000010  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA23C => {
    //   block [0x82FFA23C..0x82FFA244)
	// 82FFA23C: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82FFA240: 48000008  b 0x82ffa248
	pc = 0x82FFA248; continue 'dispatch;
            }
            0x82FFA244 => {
    //   block [0x82FFA244..0x82FFA28C)
	// 82FFA244: 39600039  li r11, 0x39
	ctx.r[11].s64 = 57;
	// 82FFA248: 3884FFFE  addi r4, r4, -2
	ctx.r[4].s64 = ctx.r[4].s64 + -2;
	// 82FFA24C: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FFA250: 7FFF5397  divwu. r31, r31, r10
	ctx.r[31].u32 = ctx.r[31].u32 / ctx.r[10].u32;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFA254: 4082FF6C  bne 0x82ffa1c0
	if !ctx.cr[0].eq {
	pc = 0x82FFA1C0; continue 'dispatch;
	}
	// 82FFA258: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA25C: 4BFE777D  bl 0x82fe19d8
	ctx.lr = 0x82FFA260;
	sub_82FE19D8(ctx, base);
	// 82FFA260: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFA264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA268: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFA26C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFA270: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA274: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA27C: 4E800421  bctrl
	ctx.lr = 0x82FFA280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFA284: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFA288: 481ADF34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA290 size=8
    let mut pc: u32 = 0x82FFA290;
    'dispatch: loop {
        match pc {
            0x82FFA290 => {
    //   block [0x82FFA290..0x82FFA298)
	// 82FFA290: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFA294: 8213F850  lwz r16, -0x7b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA298 size=256
    let mut pc: u32 = 0x82FFA298;
    'dispatch: loop {
        match pc {
            0x82FFA298 => {
    //   block [0x82FFA298..0x82FFA398)
	// 82FFA298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA29C: 481ADEC9  bl 0x831a8164
	ctx.lr = 0x82FFA2A0;
	sub_831A8130(ctx, base);
	// 82FFA2A0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FFA2A4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA2A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFA2AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA2B0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FFA2B4: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 82FFA2B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFA2BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFA2C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FFA2C4: 4BFD997D  bl 0x82fd3c40
	ctx.lr = 0x82FFA2C8;
	sub_82FD3C40(ctx, base);
	// 82FFA2C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFA2CC: 41820030  beq 0x82ffa2fc
	if ctx.cr[0].eq {
	pc = 0x82FFA2FC; continue 'dispatch;
	}
	// 82FFA2D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA2D4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA2D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFA2DC: 38AB8018  addi r5, r11, -0x7fe8
	ctx.r[5].s64 = ctx.r[11].s64 + -32744;
	// 82FFA2E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA2E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA2E8: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 82FFA2EC: 816A00C0  lwz r11, 0xc0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FFA2F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA2F4: 4E800421  bctrl
	ctx.lr = 0x82FFA2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA2F8: 48000098  b 0x82ffa390
	pc = 0x82FFA390; continue 'dispatch;
	// 82FFA2FC: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FFA300: 80BB0014  lwz r5, 0x14(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFA304: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA308: 4BFE4B51  bl 0x82fdee58
	ctx.lr = 0x82FFA30C;
	sub_82FDEE58(ctx, base);
	// 82FFA30C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFA310: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFA318: 388B8018  addi r4, r11, -0x7fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -32744;
	// 82FFA31C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA320: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82FFA324: 4BFDF24D  bl 0x82fd9570
	ctx.lr = 0x82FFA328;
	sub_82FD9570(ctx, base);
	// 82FFA328: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FFA32C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA330: 4BFD67E9  bl 0x82fd0b18
	ctx.lr = 0x82FFA334;
	sub_82FD0B18(ctx, base);
	// 82FFA334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFA338: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFA33C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA340: 4BFDF231  bl 0x82fd9570
	ctx.lr = 0x82FFA344;
	sub_82FD9570(ctx, base);
	// 82FFA344: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFA348: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFA34C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFA350: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA358: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 82FFA35C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFA360: 7F6A5B2E  sthx r27, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u16) };
	// 82FFA364: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA368: 80BF0068  lwz r5, 0x68(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFA36C: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FFA370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA374: 4E800421  bctrl
	ctx.lr = 0x82FFA378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA378: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFA37C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFA380: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA384: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA38C: 4E800421  bctrl
	ctx.lr = 0x82FFA390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA390: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FFA394: 481ADE20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA398 size=40
    let mut pc: u32 = 0x82FFA398;
    'dispatch: loop {
        match pc {
            0x82FFA398 => {
    //   block [0x82FFA398..0x82FFA3C0)
	// 82FFA398: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FFA39C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA3A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA3A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA3A8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA3AC: 4BFE4B2D  bl 0x82fdeed8
	ctx.lr = 0x82FFA3B0;
	sub_82FDEED8(ctx, base);
	// 82FFA3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA3C0 size=16
    let mut pc: u32 = 0x82FFA3C0;
    'dispatch: loop {
        match pc {
            0x82FFA3C0 => {
    //   block [0x82FFA3C0..0x82FFA3D0)
	// 82FFA3C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA3C4: 396BF890  addi r11, r11, -0x770
	ctx.r[11].s64 = ctx.r[11].s64 + -1904;
	// 82FFA3C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFA3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA3D0 size=140
    let mut pc: u32 = 0x82FFA3D0;
    'dispatch: loop {
        match pc {
            0x82FFA3D0 => {
    //   block [0x82FFA3D0..0x82FFA45C)
	// 82FFA3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA3D4: 481ADD8D  bl 0x831a8160
	ctx.lr = 0x82FFA3D8;
	sub_831A8130(ctx, base);
	// 82FFA3D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA3E0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FFA3E4: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FFA3E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA3EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFA3F0: 40990060  ble cr6, 0x82ffa450
	if !ctx.cr[6].gt {
	pc = 0x82FFA450; continue 'dispatch;
	}
	// 82FFA3F4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82FFA3F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA3FC: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FFA400: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA404: 41820030  beq 0x82ffa434
	if ctx.cr[0].eq {
	pc = 0x82FFA434; continue 'dispatch;
	}
	// 82FFA408: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA40C: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA410: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA414: 4182000C  beq 0x82ffa420
	if ctx.cr[0].eq {
	pc = 0x82FFA420; continue 'dispatch;
	}
	// 82FFA418: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA41C: 4B2C5E4D  bl 0x822c0268
	ctx.lr = 0x82FFA420;
	sub_822C0268(ctx, base);
	// 82FFA420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA424: 4BFDDEBD  bl 0x82fd82e0
	ctx.lr = 0x82FFA428;
	sub_82FD82E0(ctx, base);
	// 82FFA428: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FFA42C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFA430: 409AFFD8  bne cr6, 0x82ffa408
	if !ctx.cr[6].eq {
	pc = 0x82FFA408; continue 'dispatch;
	}
	// 82FFA434: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA438: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FFA43C: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 82FFA440: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FFA444: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA448: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFA44C: 4198FFAC  blt cr6, 0x82ffa3f8
	if ctx.cr[6].lt {
	pc = 0x82FFA3F8; continue 'dispatch;
	}
	// 82FFA450: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82FFA454: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFA458: 481ADD58  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA460 size=196
    let mut pc: u32 = 0x82FFA460;
    'dispatch: loop {
        match pc {
            0x82FFA460 => {
    //   block [0x82FFA460..0x82FFA524)
	// 82FFA460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA464: 481ADD09  bl 0x831a816c
	ctx.lr = 0x82FFA468;
	sub_831A8130(ctx, base);
	// 82FFA468: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA46C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA470: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFA474: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFA478: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFA47C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA480: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA488: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA48C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA490: 4E800421  bctrl
	ctx.lr = 0x82FFA494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA494: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FFA498: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA49C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFA4A0: 40990030  ble cr6, 0x82ffa4d0
	if !ctx.cr[6].gt {
	pc = 0x82FFA4D0; continue 'dispatch;
	}
	// 82FFA4A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA4A8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA4AC: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FFA4B0: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FFA4B4: 38A001F6  li r5, 0x1f6
	ctx.r[5].s64 = 502;
	// 82FFA4B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA4BC: 4BFD6B9D  bl 0x82fd1058
	ctx.lr = 0x82FFA4C0;
	sub_82FD1058(ctx, base);
	// 82FFA4C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFA4C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA4C8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FFA4CC: 481B675D  bl 0x831b0c28
	ctx.lr = 0x82FFA4D0;
	sub_831B0C28(ctx, base);
	// 82FFA4D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA4D4: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFA4D8: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFA4DC: 4800002C  b 0x82ffa508
	pc = 0x82FFA508; continue 'dispatch;
	// 82FFA4E0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFA4E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFA4E8: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA4EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA4F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA4F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA4F8: 4E800421  bctrl
	ctx.lr = 0x82FFA4FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA4FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFA500: 4082001C  bne 0x82ffa51c
	if !ctx.cr[0].eq {
	pc = 0x82FFA51C; continue 'dispatch;
	}
	// 82FFA504: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA508: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA50C: 4082FFD4  bne 0x82ffa4e0
	if !ctx.cr[0].eq {
	pc = 0x82FFA4E0; continue 'dispatch;
	}
	// 82FFA510: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFA514: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFA518: 481ADCA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FFA51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA520: 4BFFFFF4  b 0x82ffa514
	pc = 0x82FFA514; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA528 size=316
    let mut pc: u32 = 0x82FFA528;
    'dispatch: loop {
        match pc {
            0x82FFA528 => {
    //   block [0x82FFA528..0x82FFA664)
	// 82FFA528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA52C: 481ADC39  bl 0x831a8164
	ctx.lr = 0x82FFA530;
	sub_831A8130(ctx, base);
	// 82FFA530: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA538: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FFA53C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFA540: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFA544: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA548: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA54C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA550: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA558: 4E800421  bctrl
	ctx.lr = 0x82FFA55C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA55C: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FFA560: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA564: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFA568: 40990030  ble cr6, 0x82ffa598
	if !ctx.cr[6].gt {
	pc = 0x82FFA598; continue 'dispatch;
	}
	// 82FFA56C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA570: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA574: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FFA578: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FFA57C: 38A0021F  li r5, 0x21f
	ctx.r[5].s64 = 543;
	// 82FFA580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA584: 4BFD6AD5  bl 0x82fd1058
	ctx.lr = 0x82FFA588;
	sub_82FD1058(ctx, base);
	// 82FFA588: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFA58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA590: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FFA594: 481B6695  bl 0x831b0c28
	ctx.lr = 0x82FFA598;
	sub_831B0C28(ctx, base);
	// 82FFA598: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA59C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFA5A0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FFA5A4: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FFA5A8: 48000030  b 0x82ffa5d8
	pc = 0x82FFA5D8; continue 'dispatch;
	// 82FFA5AC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFA5B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFA5B4: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA5BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA5C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA5C4: 4E800421  bctrl
	ctx.lr = 0x82FFA5C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA5C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFA5CC: 40820040  bne 0x82ffa60c
	if !ctx.cr[0].eq {
	pc = 0x82FFA60C; continue 'dispatch;
	}
	// 82FFA5D0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82FFA5D4: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA5D8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA5DC: 4082FFD0  bne 0x82ffa5ac
	if !ctx.cr[0].eq {
	pc = 0x82FFA5AC; continue 'dispatch;
	}
	// 82FFA5E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA5E4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA5E8: 38C00032  li r6, 0x32
	ctx.r[6].s64 = 50;
	// 82FFA5EC: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FFA5F0: 38A00249  li r5, 0x249
	ctx.r[5].s64 = 585;
	// 82FFA5F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA5F8: 4BFE6731  bl 0x82fe0d28
	ctx.lr = 0x82FFA5FC;
	sub_82FE0D28(ctx, base);
	// 82FFA5FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFA600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFA604: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FFA608: 481B6621  bl 0x831b0c28
	ctx.lr = 0x82FFA60C;
	sub_831B0C28(ctx, base);
	// 82FFA60C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFA610: 409A001C  bne cr6, 0x82ffa62c
	if !ctx.cr[6].eq {
	pc = 0x82FFA62C; continue 'dispatch;
	}
	// 82FFA614: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA618: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA61C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA620: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFA624: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FFA628: 4800000C  b 0x82ffa634
	pc = 0x82FFA634; continue 'dispatch;
	// 82FFA62C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA630: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFA634: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA638: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA63C: 4182000C  beq 0x82ffa648
	if ctx.cr[0].eq {
	pc = 0x82FFA648; continue 'dispatch;
	}
	// 82FFA640: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA644: 4B2C5C25  bl 0x822c0268
	ctx.lr = 0x82FFA648;
	sub_822C0268(ctx, base);
	// 82FFA648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA64C: 4BFDDC95  bl 0x82fd82e0
	ctx.lr = 0x82FFA650;
	sub_82FD82E0(ctx, base);
	// 82FFA650: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFA654: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFA658: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFA65C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFA660: 481ADB54  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA668 size=128
    let mut pc: u32 = 0x82FFA668;
    'dispatch: loop {
        match pc {
            0x82FFA668 => {
    //   block [0x82FFA668..0x82FFA6E8)
	// 82FFA668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA66C: 481ADB01  bl 0x831a816c
	ctx.lr = 0x82FFA670;
	sub_831A8130(ctx, base);
	// 82FFA670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA674: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA67C: 396BF890  addi r11, r11, -0x770
	ctx.r[11].s64 = ctx.r[11].s64 + -1904;
	// 82FFA680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFA684: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFA688: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFA68C: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FFA690: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82FFA694: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFA698: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FFA69C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFA6A0: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FFA6A4: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FFA6A8: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA6AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA6B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA6B4: 4E800421  bctrl
	ctx.lr = 0x82FFA6B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA6B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFA6BC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FFA6C0: 419A001C  beq cr6, 0x82ffa6dc
	if ctx.cr[6].eq {
	pc = 0x82FFA6DC; continue 'dispatch;
	}
	// 82FFA6C4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FFA6C8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFA6CC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFA6D0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FFA6D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FFA6D8: 4082FFF0  bne 0x82ffa6c8
	if !ctx.cr[0].eq {
	pc = 0x82FFA6C8; continue 'dispatch;
	}
	// 82FFA6DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFA6E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFA6E4: 481ADAD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA6E8 size=32
    let mut pc: u32 = 0x82FFA6E8;
    'dispatch: loop {
        match pc {
            0x82FFA6E8 => {
    //   block [0x82FFA6E8..0x82FFA708)
	// 82FFA6E8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA6EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA6F0: 4182000C  beq 0x82ffa6fc
	if ctx.cr[0].eq {
	pc = 0x82FFA6FC; continue 'dispatch;
	}
	// 82FFA6F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA6F8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFA6FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFA704: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA708 size=28
    let mut pc: u32 = 0x82FFA708;
    'dispatch: loop {
        match pc {
            0x82FFA708 => {
    //   block [0x82FFA708..0x82FFA724)
	// 82FFA708: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA70C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFA710: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFA714: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFA718: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA71C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFA720: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA724 size=12
    let mut pc: u32 = 0x82FFA724;
    'dispatch: loop {
        match pc {
            0x82FFA724 => {
    //   block [0x82FFA724..0x82FFA730)
	// 82FFA724: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA728: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFA72C: 48000028  b 0x82ffa754
	sub_82FFA744(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA730 size=20
    let mut pc: u32 = 0x82FFA730;
    'dispatch: loop {
        match pc {
            0x82FFA730 => {
    //   block [0x82FFA730..0x82FFA744)
	// 82FFA730: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFA734: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFA738: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA73C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFA740: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA744(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA744 size=52
    let mut pc: u32 = 0x82FFA744;
    'dispatch: loop {
        match pc {
            0x82FFA744 => {
    //   block [0x82FFA744..0x82FFA778)
	// 82FFA744: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFA748: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FFA74C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FFA750: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA754: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FFA758: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FFA75C: 419AFFD4  beq cr6, 0x82ffa730
	if ctx.cr[6].eq {
		sub_82FFA730(ctx, base);
		return;
	}
	// 82FFA760: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFA764: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFA768: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFA76C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FFA770: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFA774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA778 size=92
    let mut pc: u32 = 0x82FFA778;
    'dispatch: loop {
        match pc {
            0x82FFA778 => {
    //   block [0x82FFA778..0x82FFA7D4)
	// 82FFA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFA784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA78C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFA790: 3BEBB918  addi r31, r11, -0x46e8
	ctx.r[31].s64 = ctx.r[11].s64 + -18152;
	// 82FFA794: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA798: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFA79C: 419A0014  beq cr6, 0x82ffa7b0
	if ctx.cr[6].eq {
	pc = 0x82FFA7B0; continue 'dispatch;
	}
	// 82FFA7A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA7A4: 4BFFAFE5  bl 0x82ff5788
	ctx.lr = 0x82FFA7A8;
	sub_82FF5788(ctx, base);
	// 82FFA7A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFA7AC: 4BFDDB35  bl 0x82fd82e0
	ctx.lr = 0x82FFA7B0;
	sub_82FD82E0(ctx, base);
	// 82FFA7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFA7B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFA7B8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FFA7BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFA7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA7C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFA7CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFA7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA7D8 size=8
    let mut pc: u32 = 0x82FFA7D8;
    'dispatch: loop {
        match pc {
            0x82FFA7D8 => {
    //   block [0x82FFA7D8..0x82FFA7E0)
	// 82FFA7D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFA7DC: 8213F8B8  lwz r16, -0x748(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA7E0 size=176
    let mut pc: u32 = 0x82FFA7E0;
    'dispatch: loop {
        match pc {
            0x82FFA7E0 => {
    //   block [0x82FFA7E0..0x82FFA890)
	// 82FFA7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFA7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA7F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FFA7F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA7F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFA7FC: 3BCBB918  addi r30, r11, -0x46e8
	ctx.r[30].s64 = ctx.r[11].s64 + -18152;
	// 82FFA800: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA804: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFA808: 409A0070  bne cr6, 0x82ffa878
	if !ctx.cr[6].eq {
	pc = 0x82FFA878; continue 'dispatch;
	}
	// 82FFA80C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFA810: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA814: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FFA818: 4BFFAFC1  bl 0x82ff57d8
	ctx.lr = 0x82FFA81C;
	sub_82FF57D8(ctx, base);
	// 82FFA81C: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA820: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA824: 40820048  bne 0x82ffa86c
	if !ctx.cr[0].eq {
	pc = 0x82FFA86C; continue 'dispatch;
	}
	// 82FFA828: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FFA82C: 4BFDDA1D  bl 0x82fd8248
	ctx.lr = 0x82FFA830;
	sub_82FD8248(ctx, base);
	// 82FFA830: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FFA834: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA838: 41820010  beq 0x82ffa848
	if ctx.cr[0].eq {
	pc = 0x82FFA848; continue 'dispatch;
	}
	// 82FFA83C: 4BFFAF0D  bl 0x82ff5748
	ctx.lr = 0x82FFA840;
	sub_82FF5748(ctx, base);
	// 82FFA840: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FFA844: 48000008  b 0x82ffa84c
	pc = 0x82FFA84C; continue 'dispatch;
	// 82FFA848: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFA84C: 3D608300  lis r11, -0x7d00
	ctx.r[11].s64 = -2097152000;
	// 82FFA850: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FFA854: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FFA858: 388BA778  addi r4, r11, -0x5888
	ctx.r[4].s64 = ctx.r[11].s64 + -22664;
	// 82FFA85C: 386AB924  addi r3, r10, -0x46dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18140;
	// 82FFA860: 4BFFD2D9  bl 0x82ff7b38
	ctx.lr = 0x82FFA864;
	sub_82FF7B38(ctx, base);
	// 82FFA864: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFA868: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FFA86C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA870: 4BFFAFA1  bl 0x82ff5810
	ctx.lr = 0x82FFA874;
	sub_82FF5810(ctx, base);
	// 82FFA874: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFA878: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FFA87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA884: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFA888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFA88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA890 size=40
    let mut pc: u32 = 0x82FFA890;
    'dispatch: loop {
        match pc {
            0x82FFA890 => {
    //   block [0x82FFA890..0x82FFA8B8)
	// 82FFA890: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFA894: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA898: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA89C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA8A0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA8A4: 4BFFAF6D  bl 0x82ff5810
	ctx.lr = 0x82FFA8A8;
	sub_82FF5810(ctx, base);
	// 82FFA8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA8B8 size=40
    let mut pc: u32 = 0x82FFA8B8;
    'dispatch: loop {
        match pc {
            0x82FFA8B8 => {
    //   block [0x82FFA8B8..0x82FFA8E0)
	// 82FFA8B8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFA8BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA8C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA8C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA8C8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFA8CC: 4BFDDA15  bl 0x82fd82e0
	ctx.lr = 0x82FFA8D0;
	sub_82FD82E0(ctx, base);
	// 82FFA8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFA8E0 size=8
    let mut pc: u32 = 0x82FFA8E0;
    'dispatch: loop {
        match pc {
            0x82FFA8E0 => {
    //   block [0x82FFA8E0..0x82FFA8E8)
	// 82FFA8E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFA8E4: 8213F908  lwz r16, -0x6f8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1784 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA8E8 size=156
    let mut pc: u32 = 0x82FFA8E8;
    'dispatch: loop {
        match pc {
            0x82FFA8E8 => {
    //   block [0x82FFA8E8..0x82FFA984)
	// 82FFA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFA8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA8F8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FFA8FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA900: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FFA904: 807EB920  lwz r3, -0x46e0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18144 as u32) ) } as u64;
	// 82FFA908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFA90C: 409A0060  bne cr6, 0x82ffa96c
	if !ctx.cr[6].eq {
	pc = 0x82FFA96C; continue 'dispatch;
	}
	// 82FFA910: 4BFFFED1  bl 0x82ffa7e0
	ctx.lr = 0x82FFA914;
	sub_82FFA7E0(ctx, base);
	// 82FFA914: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFA918: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA91C: 4BFFAEBD  bl 0x82ff57d8
	ctx.lr = 0x82FFA920;
	sub_82FF57D8(ctx, base);
	// 82FFA920: 817EB920  lwz r11, -0x46e0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18144 as u32) ) } as u64;
	// 82FFA924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFA928: 409A0038  bne cr6, 0x82ffa960
	if !ctx.cr[6].eq {
	pc = 0x82FFA960; continue 'dispatch;
	}
	// 82FFA92C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFA930: 386B8070  addi r3, r11, -0x7f90
	ctx.r[3].s64 = ctx.r[11].s64 + -32656;
	// 82FFA934: 4BFDD75D  bl 0x82fd8090
	ctx.lr = 0x82FFA938;
	sub_82FD8090(ctx, base);
	// 82FFA938: 907EB920  stw r3, -0x46e0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18144 as u32), ctx.r[3].u32 ) };
	// 82FFA93C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA940: 4082000C  bne 0x82ffa94c
	if !ctx.cr[0].eq {
	pc = 0x82FFA94C; continue 'dispatch;
	}
	// 82FFA944: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FFA948: 4BFFA101  bl 0x82ff4a48
	ctx.lr = 0x82FFA94C;
	sub_82FF4A48(ctx, base);
	// 82FFA94C: 3D608300  lis r11, -0x7d00
	ctx.r[11].s64 = -2097152000;
	// 82FFA950: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FFA954: 388BA130  addi r4, r11, -0x5ed0
	ctx.r[4].s64 = ctx.r[11].s64 + -24272;
	// 82FFA958: 386AB930  addi r3, r10, -0x46d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18128;
	// 82FFA95C: 4BFFD1DD  bl 0x82ff7b38
	ctx.lr = 0x82FFA960;
	sub_82FF7B38(ctx, base);
	// 82FFA960: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA964: 4BFFAEAD  bl 0x82ff5810
	ctx.lr = 0x82FFA968;
	sub_82FF5810(ctx, base);
	// 82FFA968: 807EB920  lwz r3, -0x46e0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18144 as u32) ) } as u64;
	// 82FFA96C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FFA970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA978: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFA97C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFA980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA984 size=40
    let mut pc: u32 = 0x82FFA984;
    'dispatch: loop {
        match pc {
            0x82FFA984 => {
    //   block [0x82FFA984..0x82FFA9AC)
	// 82FFA984: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFA988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA994: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFA998: 4BFFAE79  bl 0x82ff5810
	ctx.lr = 0x82FFA99C;
	sub_82FF5810(ctx, base);
	// 82FFA99C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFA9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFA9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFA9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFA9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFA9B0 size=104
    let mut pc: u32 = 0x82FFA9B0;
    'dispatch: loop {
        match pc {
            0x82FFA9B0 => {
    //   block [0x82FFA9B0..0x82FFAA18)
	// 82FFA9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFA9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFA9B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFA9BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFA9C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFA9C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFA9C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFA9CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA9D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFA9D4: 41820018  beq 0x82ffa9ec
	if ctx.cr[0].eq {
	pc = 0x82FFA9EC; continue 'dispatch;
	}
	// 82FFA9D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA9DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFA9E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFA9E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFA9E8: 4E800421  bctrl
	ctx.lr = 0x82FFA9EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFA9EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFA9F0: 4182000C  beq 0x82ffa9fc
	if ctx.cr[0].eq {
	pc = 0x82FFA9FC; continue 'dispatch;
	}
	// 82FFA9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFA9F8: 4BFDD8E9  bl 0x82fd82e0
	ctx.lr = 0x82FFA9FC;
	sub_82FD82E0(ctx, base);
	// 82FFA9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFAA00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFAA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAA0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFAA10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFAA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAA18 size=8
    let mut pc: u32 = 0x82FFAA18;
    'dispatch: loop {
        match pc {
            0x82FFAA18 => {
    //   block [0x82FFAA18..0x82FFAA20)
	// 82FFAA18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFAA1C: 8213F948  lwz r16, -0x6b8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAA20 size=168
    let mut pc: u32 = 0x82FFAA20;
    'dispatch: loop {
        match pc {
            0x82FFAA20 => {
    //   block [0x82FFAA20..0x82FFAAC8)
	// 82FFAA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAA24: 481AD745  bl 0x831a8168
	ctx.lr = 0x82FFAA28;
	sub_831A8130(ctx, base);
	// 82FFAA28: 3BE1EF50  addi r31, r1, -0x10b0
	ctx.r[31].s64 = ctx.r[1].s64 + -4272;
	// 82FFAA2C: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FFAA30: 9421EF50  stwu r1, -0x10b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAA34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFAA38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFAA3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFAA40: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAA44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFAA48: 419A0078  beq cr6, 0x82ffaac0
	if ctx.cr[6].eq {
	pc = 0x82FFAAC0; continue 'dispatch;
	}
	// 82FFAA4C: 4BFFFE9D  bl 0x82ffa8e8
	ctx.lr = 0x82FFAA50;
	sub_82FFA8E8(ctx, base);
	// 82FFAA50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAA54: 38C007FF  li r6, 0x7ff
	ctx.r[6].s64 = 2047;
	// 82FFAA58: 38BF0080  addi r5, r31, 0x80
	ctx.r[5].s64 = ctx.r[31].s64 + 128;
	// 82FFAA5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFAA60: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFAA64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFAA68: 4E800421  bctrl
	ctx.lr = 0x82FFAA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFAA6C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FFAA70: 38DF0080  addi r6, r31, 0x80
	ctx.r[6].s64 = ctx.r[31].s64 + 128;
	// 82FFAA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFAA78: 7FC40734  extsh r4, r30
	ctx.r[4].s64 = ctx.r[30].s16 as i64;
	// 82FFAA7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFAA80: 4803E171  bl 0x83038bf0
	ctx.lr = 0x82FFAA84;
	sub_83038BF0(ctx, base);
	// 82FFAA84: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAA88: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82FFAA8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAA90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFAA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFAA98: 4E800421  bctrl
	ctx.lr = 0x82FFAA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFAA9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFAAA0: 40820018  bne 0x82ffaab8
	if !ctx.cr[0].eq {
	pc = 0x82FFAAB8; continue 'dispatch;
	}
	// 82FFAAA4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFAAA8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FFAAAC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFAAB0: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 82FFAAB4: 481B6175  bl 0x831b0c28
	ctx.lr = 0x82FFAAB8;
	sub_831B0C28(ctx, base);
	// 82FFAAB8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFAABC: 4803E16D  bl 0x83038c28
	ctx.lr = 0x82FFAAC0;
	sub_83038C28(ctx, base);
	// 82FFAAC0: 383F10B0  addi r1, r31, 0x10b0
	ctx.r[1].s64 = ctx.r[31].s64 + 4272;
	// 82FFAAC4: 481AD6F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAAC8 size=40
    let mut pc: u32 = 0x82FFAAC8;
    'dispatch: loop {
        match pc {
            0x82FFAAC8 => {
    //   block [0x82FFAAC8..0x82FFAAF0)
	// 82FFAAC8: 3BECEF50  addi r31, r12, -0x10b0
	ctx.r[31].s64 = ctx.r[12].s64 + -4272;
	// 82FFAACC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAAD0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAAD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAAD8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFAADC: 4803E14D  bl 0x83038c28
	ctx.lr = 0x82FFAAE0;
	sub_83038C28(ctx, base);
	// 82FFAAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAAF0 size=100
    let mut pc: u32 = 0x82FFAAF0;
    'dispatch: loop {
        match pc {
            0x82FFAAF0 => {
    //   block [0x82FFAAF0..0x82FFAB54)
	// 82FFAAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFAAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAB00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFAB04: 4BFFF8CD  bl 0x82ffa3d0
	ctx.lr = 0x82FFAB08;
	sub_82FFA3D0(ctx, base);
	// 82FFAB08: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAB0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAB10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAB14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAB18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFAB1C: 4E800421  bctrl
	ctx.lr = 0x82FFAB20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFAB20: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFAB24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAB28: 41820018  beq 0x82ffab40
	if ctx.cr[0].eq {
	pc = 0x82FFAB40; continue 'dispatch;
	}
	// 82FFAB2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAB30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFAB34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAB38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFAB3C: 4E800421  bctrl
	ctx.lr = 0x82FFAB40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFAB40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAB4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFAB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAB58 size=200
    let mut pc: u32 = 0x82FFAB58;
    'dispatch: loop {
        match pc {
            0x82FFAB58 => {
    //   block [0x82FFAB58..0x82FFAC20)
	// 82FFAB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAB5C: 481AD60D  bl 0x831a8168
	ctx.lr = 0x82FFAB60;
	sub_831A8130(ctx, base);
	// 82FFAB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAB64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFAB68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFAB6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFAB70: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFAB74: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFAB78: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 82FFAB7C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFAB80: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFAB84: 41980008  blt cr6, 0x82ffab8c
	if ctx.cr[6].lt {
	pc = 0x82FFAB8C; continue 'dispatch;
	}
	// 82FFAB88: 4806DE61  bl 0x830689e8
	ctx.lr = 0x82FFAB8C;
	sub_830689E8(ctx, base);
	// 82FFAB8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFAB90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFAB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFAB98: 4BFFF8C9  bl 0x82ffa460
	ctx.lr = 0x82FFAB9C;
	sub_82FFA460(ctx, base);
	// 82FFAB9C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFABA0: 41820024  beq 0x82ffabc4
	if ctx.cr[0].eq {
	pc = 0x82FFABC4; continue 'dispatch;
	}
	// 82FFABA4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFABA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFABAC: 4182000C  beq 0x82ffabb8
	if ctx.cr[0].eq {
	pc = 0x82FFABB8; continue 'dispatch;
	}
	// 82FFABB0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFABB4: 4B2C56B5  bl 0x822c0268
	ctx.lr = 0x82FFABB8;
	sub_822C0268(ctx, base);
	// 82FFABB8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FFABBC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFABC0: 48000058  b 0x82ffac18
	pc = 0x82FFAC18; continue 'dispatch;
	// 82FFABC4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FFABC8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFABCC: 4BFDD6CD  bl 0x82fd8298
	ctx.lr = 0x82FFABD0;
	sub_82FD8298(ctx, base);
	// 82FFABD0: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFABD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFABD8: 41820024  beq 0x82ffabfc
	if ctx.cr[0].eq {
	pc = 0x82FFABFC; continue 'dispatch;
	}
	// 82FFABDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFABE0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FFABE4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FFABE8: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFABEC: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FFABF0: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFABF4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFABF8: 48000008  b 0x82ffac00
	pc = 0x82FFAC00; continue 'dispatch;
	// 82FFABFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFAC00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAC04: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFAC08: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82FFAC0C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFAC10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFAC14: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFAC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFAC1C: 481AD59C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAC20 size=8
    let mut pc: u32 = 0x82FFAC20;
    'dispatch: loop {
        match pc {
            0x82FFAC20 => {
    //   block [0x82FFAC20..0x82FFAC28)
	// 82FFAC20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFAC24: 8213F9B8  lwz r16, -0x648(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAC28 size=160
    let mut pc: u32 = 0x82FFAC28;
    'dispatch: loop {
        match pc {
            0x82FFAC28 => {
    //   block [0x82FFAC28..0x82FFACC8)
	// 82FFAC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAC30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFAC34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFAC38: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFAC3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAC40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFAC44: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FFAC48: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FFAC4C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFAC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFAC54: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FFAC58: 396BF9A0  addi r11, r11, -0x660
	ctx.r[11].s64 = ctx.r[11].s64 + -1632;
	// 82FFAC5C: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FFAC60: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FFAC64: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82FFAC68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFAC6C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FFAC70: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFAC74: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FFAC78: 409A002C  bne cr6, 0x82ffaca4
	if !ctx.cr[6].eq {
	pc = 0x82FFACA4; continue 'dispatch;
	}
	// 82FFAC7C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFAC80: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82FFAC84: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FFAC88: 38A00259  li r5, 0x259
	ctx.r[5].s64 = 601;
	// 82FFAC8C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFAC90: 4BFE6261  bl 0x82fe0ef0
	ctx.lr = 0x82FFAC94;
	sub_82FE0EF0(ctx, base);
	// 82FFAC94: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFAC98: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFAC9C: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 82FFACA0: 481B5F89  bl 0x831b0c28
	ctx.lr = 0x82FFACA4;
	sub_831B0C28(ctx, base);
	// 82FFACA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFACA8: 4BFFFA41  bl 0x82ffa6e8
	ctx.lr = 0x82FFACAC;
	sub_82FFA6E8(ctx, base);
	// 82FFACAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFACB0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFACBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFACC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFACC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFACC8 size=40
    let mut pc: u32 = 0x82FFACC8;
    'dispatch: loop {
        match pc {
            0x82FFACC8 => {
    //   block [0x82FFACC8..0x82FFACF0)
	// 82FFACC8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFACCC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFACD0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFACD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFACD8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFACDC: 48051A85  bl 0x8304c760
	ctx.lr = 0x82FFACE0;
	sub_8304C760(ctx, base);
	// 82FFACE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFACE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFACE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFACF0 size=12
    let mut pc: u32 = 0x82FFACF0;
    'dispatch: loop {
        match pc {
            0x82FFACF0 => {
    //   block [0x82FFACF0..0x82FFACFC)
	// 82FFACF0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFACF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFACF8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFACFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFACFC size=8
    let mut pc: u32 = 0x82FFACFC;
    'dispatch: loop {
        match pc {
            0x82FFACFC => {
    //   block [0x82FFACFC..0x82FFAD04)
	// 82FFACFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFAD00: 4BFFFCB0  b 0x82ffa9b0
	sub_82FFA9B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAD04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAD04 size=4
    let mut pc: u32 = 0x82FFAD04;
    'dispatch: loop {
        match pc {
            0x82FFAD04 => {
    //   block [0x82FFAD04..0x82FFAD08)
	// 82FFAD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAD08 size=8
    let mut pc: u32 = 0x82FFAD08;
    'dispatch: loop {
        match pc {
            0x82FFAD08 => {
    //   block [0x82FFAD08..0x82FFAD10)
	// 82FFAD08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFAD0C: 8213F9F0  lwz r16, -0x610(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAD10 size=116
    let mut pc: u32 = 0x82FFAD10;
    'dispatch: loop {
        match pc {
            0x82FFAD10 => {
    //   block [0x82FFAD10..0x82FFAD84)
	// 82FFAD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAD14: 481AD455  bl 0x831a8168
	ctx.lr = 0x82FFAD18;
	sub_831A8130(ctx, base);
	// 82FFAD18: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFAD1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAD20: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFAD24: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFAD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFAD2C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FFAD30: 93BF009C  stw r29, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 82FFAD34: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFAD38: 4BFDD561  bl 0x82fd8298
	ctx.lr = 0x82FFAD3C;
	sub_82FD8298(ctx, base);
	// 82FFAD3C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFAD40: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FFAD44: 4182002C  beq 0x82ffad70
	if ctx.cr[0].eq {
	pc = 0x82FFAD70; continue 'dispatch;
	}
	// 82FFAD48: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FFAD4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FFAD50: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FFAD54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFAD58: 4BFFF911  bl 0x82ffa668
	ctx.lr = 0x82FFAD5C;
	sub_82FFA668(ctx, base);
	// 82FFAD5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFAD60: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FFAD64: 396BF988  addi r11, r11, -0x678
	ctx.r[11].s64 = ctx.r[11].s64 + -1656;
	// 82FFAD68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFAD6C: 48000008  b 0x82ffad74
	pc = 0x82FFAD74; continue 'dispatch;
	// 82FFAD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFAD74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFAD78: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFAD7C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFAD80: 481AD438  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAD84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAD84 size=44
    let mut pc: u32 = 0x82FFAD84;
    'dispatch: loop {
        match pc {
            0x82FFAD84 => {
    //   block [0x82FFAD84..0x82FFADB0)
	// 82FFAD84: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFAD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAD94: 809F009C  lwz r4, 0x9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FFAD98: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFAD9C: 4BFDD545  bl 0x82fd82e0
	ctx.lr = 0x82FFADA0;
	sub_82FD82E0(ctx, base);
	// 82FFADA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFADA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFADA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFADB0 size=96
    let mut pc: u32 = 0x82FFADB0;
    'dispatch: loop {
        match pc {
            0x82FFADB0 => {
    //   block [0x82FFADB0..0x82FFAE10)
	// 82FFADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFADB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFADBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFADC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFADC4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FFADC8: 4BFDD4D1  bl 0x82fd8298
	ctx.lr = 0x82FFADCC;
	sub_82FD8298(ctx, base);
	// 82FFADCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFADD0: 41820020  beq 0x82ffadf0
	if ctx.cr[0].eq {
	pc = 0x82FFADF0; continue 'dispatch;
	}
	// 82FFADD4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFADD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFADDC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFADE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFADE4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFADE8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFADEC: 48000008  b 0x82ffadf4
	pc = 0x82FFADF4; continue 'dispatch;
	// 82FFADF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFADF4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFADF8: 48040359  bl 0x8303b150
	ctx.lr = 0x82FFADFC;
	sub_8303B150(ctx, base);
	// 82FFADFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAE08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFAE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAE10 size=96
    let mut pc: u32 = 0x82FFAE10;
    'dispatch: loop {
        match pc {
            0x82FFAE10 => {
    //   block [0x82FFAE10..0x82FFAE70)
	// 82FFAE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAE18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAE1C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFAE20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE24: 41820020  beq 0x82ffae44
	if ctx.cr[0].eq {
	pc = 0x82FFAE44; continue 'dispatch;
	}
	// 82FFAE28: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFAE2C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFAE30: 4BFFF631  bl 0x82ffa460
	ctx.lr = 0x82FFAE34;
	sub_82FFA460(ctx, base);
	// 82FFAE34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE38: 41820024  beq 0x82ffae5c
	if ctx.cr[0].eq {
	pc = 0x82FFAE5C; continue 'dispatch;
	}
	// 82FFAE3C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAE40: 48000014  b 0x82ffae54
	pc = 0x82FFAE54; continue 'dispatch;
	// 82FFAE44: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAE48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE4C: 41820010  beq 0x82ffae5c
	if ctx.cr[0].eq {
	pc = 0x82FFAE5C; continue 'dispatch;
	}
	// 82FFAE50: 4BFFFFC1  bl 0x82ffae10
	ctx.lr = 0x82FFAE54;
	sub_82FFAE10(ctx, base);
	// 82FFAE54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE58: 409A0008  bne cr6, 0x82ffae60
	if !ctx.cr[6].eq {
	pc = 0x82FFAE60; continue 'dispatch;
	}
	// 82FFAE5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFAE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAE70 size=96
    let mut pc: u32 = 0x82FFAE70;
    'dispatch: loop {
        match pc {
            0x82FFAE70 => {
    //   block [0x82FFAE70..0x82FFAED0)
	// 82FFAE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAE7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFAE80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE84: 41820020  beq 0x82ffaea4
	if ctx.cr[0].eq {
	pc = 0x82FFAEA4; continue 'dispatch;
	}
	// 82FFAE88: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFAE8C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFAE90: 4BFFF5D1  bl 0x82ffa460
	ctx.lr = 0x82FFAE94;
	sub_82FFA460(ctx, base);
	// 82FFAE94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAE98: 41820024  beq 0x82ffaebc
	if ctx.cr[0].eq {
	pc = 0x82FFAEBC; continue 'dispatch;
	}
	// 82FFAE9C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAEA0: 48000014  b 0x82ffaeb4
	pc = 0x82FFAEB4; continue 'dispatch;
	// 82FFAEA4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFAEA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAEAC: 41820010  beq 0x82ffaebc
	if ctx.cr[0].eq {
	pc = 0x82FFAEBC; continue 'dispatch;
	}
	// 82FFAEB0: 4BFFFFC1  bl 0x82ffae70
	ctx.lr = 0x82FFAEB4;
	sub_82FFAE70(ctx, base);
	// 82FFAEB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFAEB8: 409A0008  bne cr6, 0x82ffaec0
	if !ctx.cr[6].eq {
	pc = 0x82FFAEC0; continue 'dispatch;
	}
	// 82FFAEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFAEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAED0 size=8
    let mut pc: u32 = 0x82FFAED0;
    'dispatch: loop {
        match pc {
            0x82FFAED0 => {
    //   block [0x82FFAED0..0x82FFAED8)
	// 82FFAED0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFAED4: 8213FA28  lwz r16, -0x5d8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAED8 size=96
    let mut pc: u32 = 0x82FFAED8;
    'dispatch: loop {
        match pc {
            0x82FFAED8 => {
    //   block [0x82FFAED8..0x82FFAF38)
	// 82FFAED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAEDC: 481AD291  bl 0x831a816c
	ctx.lr = 0x82FFAEE0;
	sub_831A8130(ctx, base);
	// 82FFAEE0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FFAEE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAEE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFAEEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFAEF0: 396BF9A0  addi r11, r11, -0x660
	ctx.r[11].s64 = ctx.r[11].s64 + -1632;
	// 82FFAEF4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FFAEF8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFAEFC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFAF00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAF04: 41820020  beq 0x82ffaf24
	if ctx.cr[0].eq {
	pc = 0x82FFAF24; continue 'dispatch;
	}
	// 82FFAF08: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFAF0C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFAF10: 41820014  beq 0x82ffaf24
	if ctx.cr[0].eq {
	pc = 0x82FFAF24; continue 'dispatch;
	}
	// 82FFAF14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFAF18: 4BFFFBD9  bl 0x82ffaaf0
	ctx.lr = 0x82FFAF1C;
	sub_82FFAAF0(ctx, base);
	// 82FFAF1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFAF20: 4BFDD3C1  bl 0x82fd82e0
	ctx.lr = 0x82FFAF24;
	sub_82FD82E0(ctx, base);
	// 82FFAF24: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFAF28: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FFAF2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFAF30: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FFAF34: 481AD288  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAF38 size=40
    let mut pc: u32 = 0x82FFAF38;
    'dispatch: loop {
        match pc {
            0x82FFAF38 => {
    //   block [0x82FFAF38..0x82FFAF60)
	// 82FFAF38: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFAF3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAF40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAF44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAF48: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FFAF4C: 48051815  bl 0x8304c760
	ctx.lr = 0x82FFAF50;
	sub_8304C760(ctx, base);
	// 82FFAF50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFAF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAF60 size=76
    let mut pc: u32 = 0x82FFAF60;
    'dispatch: loop {
        match pc {
            0x82FFAF60 => {
    //   block [0x82FFAF60..0x82FFAFAC)
	// 82FFAF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFAF68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFAF6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFAF70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAF74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFAF78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFAF7C: 4BFFFF5D  bl 0x82ffaed8
	ctx.lr = 0x82FFAF80;
	sub_82FFAED8(ctx, base);
	// 82FFAF80: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFAF84: 4182000C  beq 0x82ffaf90
	if ctx.cr[0].eq {
	pc = 0x82FFAF90; continue 'dispatch;
	}
	// 82FFAF88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFAF8C: 4BFDD355  bl 0x82fd82e0
	ctx.lr = 0x82FFAF90;
	sub_82FD82E0(ctx, base);
	// 82FFAF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFAF94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFAF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFAF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFAFA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFAFA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFAFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFAFB0 size=8
    let mut pc: u32 = 0x82FFAFB0;
    'dispatch: loop {
        match pc {
            0x82FFAFB0 => {
    //   block [0x82FFAFB0..0x82FFAFB8)
	// 82FFAFB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFAFB4: 8213FA60  lwz r16, -0x5a0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1440 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFAFB8 size=104
    let mut pc: u32 = 0x82FFAFB8;
    'dispatch: loop {
        match pc {
            0x82FFAFB8 => {
    //   block [0x82FFAFB8..0x82FFB020)
	// 82FFAFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFAFBC: 481AD1B1  bl 0x831a816c
	ctx.lr = 0x82FFAFC0;
	sub_831A8130(ctx, base);
	// 82FFAFC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFAFC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFAFC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFAFCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFAFD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFAFD4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FFAFD8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FFAFDC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82FFAFE0: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FFAFE4: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFAFE8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFAFEC: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FFAFF0: 909E0014  stw r4, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82FFAFF4: 4BFDD2A5  bl 0x82fd8298
	ctx.lr = 0x82FFAFF8;
	sub_82FD8298(ctx, base);
	// 82FFAFF8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFAFFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB000: 41820010  beq 0x82ffb010
	if ctx.cr[0].eq {
	pc = 0x82FFB010; continue 'dispatch;
	}
	// 82FFB004: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB008: 4BFFFD09  bl 0x82ffad10
	ctx.lr = 0x82FFB00C;
	sub_82FFAD10(ctx, base);
	// 82FFB00C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFB010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFB014: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FFB018: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFB01C: 481AD1A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB020 size=48
    let mut pc: u32 = 0x82FFB020;
    'dispatch: loop {
        match pc {
            0x82FFB020 => {
    //   block [0x82FFB020..0x82FFB050)
	// 82FFB020: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFB024: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB028: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB030: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFB034: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB038: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFB03C: 4BFDD2A5  bl 0x82fd82e0
	ctx.lr = 0x82FFB040;
	sub_82FD82E0(ctx, base);
	// 82FFB040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB050 size=108
    let mut pc: u32 = 0x82FFB050;
    'dispatch: loop {
        match pc {
            0x82FFB050 => {
    //   block [0x82FFB050..0x82FFB0BC)
	// 82FFB050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFB05C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFB060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB064: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB068: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFB06C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFB070: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB074: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFB078: 480317F9  bl 0x8302c870
	ctx.lr = 0x82FFB07C;
	sub_8302C870(ctx, base);
	// 82FFB07C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFB080: 4BFFFD91  bl 0x82ffae10
	ctx.lr = 0x82FFB084;
	sub_82FFAE10(ctx, base);
	// 82FFB084: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB088: 41820018  beq 0x82ffb0a0
	if ctx.cr[0].eq {
	pc = 0x82FFB0A0; continue 'dispatch;
	}
	// 82FFB08C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFB090: 4BFD8BB1  bl 0x82fd3c40
	ctx.lr = 0x82FFB094;
	sub_82FD3C40(ctx, base);
	// 82FFB094: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFB098: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFB09C: 40820008  bne 0x82ffb0a4
	if !ctx.cr[0].eq {
	pc = 0x82FFB0A4; continue 'dispatch;
	}
	// 82FFB0A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFB0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFB0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB0B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFB0B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFB0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB0C0 size=104
    let mut pc: u32 = 0x82FFB0C0;
    'dispatch: loop {
        match pc {
            0x82FFB0C0 => {
    //   block [0x82FFB0C0..0x82FFB128)
	// 82FFB0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB0C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFB0CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFB0D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB0D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB0D8: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB0DC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB0E0: 41820014  beq 0x82ffb0f4
	if ctx.cr[0].eq {
	pc = 0x82FFB0F4; continue 'dispatch;
	}
	// 82FFB0E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB0E8: 4BFFFA09  bl 0x82ffaaf0
	ctx.lr = 0x82FFB0EC;
	sub_82FFAAF0(ctx, base);
	// 82FFB0EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB0F0: 4BFDD1F1  bl 0x82fd82e0
	ctx.lr = 0x82FFB0F4;
	sub_82FD82E0(ctx, base);
	// 82FFB0F4: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB0F8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB0FC: 41820014  beq 0x82ffb110
	if ctx.cr[0].eq {
	pc = 0x82FFB110; continue 'dispatch;
	}
	// 82FFB100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB104: 4BFFF9ED  bl 0x82ffaaf0
	ctx.lr = 0x82FFB108;
	sub_82FFAAF0(ctx, base);
	// 82FFB108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB10C: 4BFDD1D5  bl 0x82fd82e0
	ctx.lr = 0x82FFB110;
	sub_82FD82E0(ctx, base);
	// 82FFB110: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFB114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB11C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFB120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFB124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFB128 size=8
    let mut pc: u32 = 0x82FFB128;
    'dispatch: loop {
        match pc {
            0x82FFB128 => {
    //   block [0x82FFB128..0x82FFB130)
	// 82FFB128: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFB12C: 8213FAB0  lwz r16, -0x550(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB130 size=540
    let mut pc: u32 = 0x82FFB130;
    'dispatch: loop {
        match pc {
            0x82FFB130 => {
    //   block [0x82FFB130..0x82FFB34C)
	// 82FFB130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB134: 481AD02D  bl 0x831a8160
	ctx.lr = 0x82FFB138;
	sub_831A8130(ctx, base);
	// 82FFB138: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FFB13C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB140: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB144: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FFB148: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FFB14C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FFB150: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB154: 939F00FC  stw r28, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[28].u32 ) };
	// 82FFB158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFB15C: 409A0198  bne cr6, 0x82ffb2f4
	if !ctx.cr[6].eq {
	pc = 0x82FFB2F4; continue 'dispatch;
	}
	// 82FFB160: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFB164: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FFB168: 4BFDD131  bl 0x82fd8298
	ctx.lr = 0x82FFB16C;
	sub_82FD8298(ctx, base);
	// 82FFB16C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFB170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB174: 4182001C  beq 0x82ffb190
	if ctx.cr[0].eq {
	pc = 0x82FFB190; continue 'dispatch;
	}
	// 82FFB178: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFB17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB180: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FFB184: 48051DDD  bl 0x8304cf60
	ctx.lr = 0x82FFB188;
	sub_8304CF60(ctx, base);
	// 82FFB188: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFB18C: 48000008  b 0x82ffb194
	pc = 0x82FFB194; continue 'dispatch;
	// 82FFB190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFB194: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFB198: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FFB19C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFB1A0: 4BFDD0F9  bl 0x82fd8298
	ctx.lr = 0x82FFB1A4;
	sub_82FD8298(ctx, base);
	// 82FFB1A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFB1A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB1AC: 41820018  beq 0x82ffb1c4
	if ctx.cr[0].eq {
	pc = 0x82FFB1C4; continue 'dispatch;
	}
	// 82FFB1B0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFB1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB1B8: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FFB1BC: 48051DA5  bl 0x8304cf60
	ctx.lr = 0x82FFB1C0;
	sub_8304CF60(ctx, base);
	// 82FFB1C0: 48000008  b 0x82ffb1c8
	pc = 0x82FFB1C8; continue 'dispatch;
	// 82FFB1C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFB1C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB1CC: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FFB1D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB1D4: 41820120  beq 0x82ffb2f4
	if ctx.cr[0].eq {
	pc = 0x82FFB2F4; continue 'dispatch;
	}
	// 82FFB1D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFB1DC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB1E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB1E4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFB1E8: 4BFFFA41  bl 0x82ffac28
	ctx.lr = 0x82FFB1EC;
	sub_82FFAC28(ctx, base);
	// 82FFB1EC: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FFB1F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFB1F4: 409A001C  bne cr6, 0x82ffb210
	if !ctx.cr[6].eq {
	pc = 0x82FFB210; continue 'dispatch;
	}
	// 82FFB1F8: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFB1FC: 815F008C  lwz r10, 0x8c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFB200: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFB204: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFB20C: 419A0008  beq cr6, 0x82ffb214
	if ctx.cr[6].eq {
	pc = 0x82FFB214; continue 'dispatch;
	}
	// 82FFB210: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFB214: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFB218: 41820044  beq 0x82ffb25c
	if ctx.cr[0].eq {
	pc = 0x82FFB25C; continue 'dispatch;
	}
	// 82FFB21C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFB220: 48051F41  bl 0x8304d160
	ctx.lr = 0x82FFB224;
	sub_8304D160(ctx, base);
	// 82FFB224: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB228: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFB22C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FFB230: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFB234: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB238: 4BFFF229  bl 0x82ffa460
	ctx.lr = 0x82FFB23C;
	sub_82FFA460(ctx, base);
	// 82FFB23C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB240: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB244: 41820008  beq 0x82ffb24c
	if ctx.cr[0].eq {
	pc = 0x82FFB24C; continue 'dispatch;
	}
	// 82FFB248: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB24C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFB250: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB254: 4BFFF905  bl 0x82ffab58
	ctx.lr = 0x82FFB258;
	sub_82FFAB58(ctx, base);
	// 82FFB258: 4BFFFF94  b 0x82ffb1ec
	pc = 0x82FFB1EC; continue 'dispatch;
	// 82FFB25C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB260: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FFB264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB268: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFB26C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB270: 4BFFF9B9  bl 0x82ffac28
	ctx.lr = 0x82FFB274;
	sub_82FFAC28(ctx, base);
	// 82FFB274: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFB27C: 409A001C  bne cr6, 0x82ffb298
	if !ctx.cr[6].eq {
	pc = 0x82FFB298; continue 'dispatch;
	}
	// 82FFB280: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FFB284: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FFB288: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFB28C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFB294: 419A0008  beq cr6, 0x82ffb29c
	if ctx.cr[6].eq {
	pc = 0x82FFB29C; continue 'dispatch;
	}
	// 82FFB298: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFB29C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFB2A0: 41820044  beq 0x82ffb2e4
	if ctx.cr[0].eq {
	pc = 0x82FFB2E4; continue 'dispatch;
	}
	// 82FFB2A4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFB2A8: 48051EB9  bl 0x8304d160
	ctx.lr = 0x82FFB2AC;
	sub_8304D160(ctx, base);
	// 82FFB2AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB2B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFB2B4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FFB2B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFB2BC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB2C0: 4BFFF1A1  bl 0x82ffa460
	ctx.lr = 0x82FFB2C4;
	sub_82FFA460(ctx, base);
	// 82FFB2C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB2C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB2CC: 41820008  beq 0x82ffb2d4
	if ctx.cr[0].eq {
	pc = 0x82FFB2D4; continue 'dispatch;
	}
	// 82FFB2D0: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB2D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFB2D8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB2DC: 4BFFF87D  bl 0x82ffab58
	ctx.lr = 0x82FFB2E0;
	sub_82FFAB58(ctx, base);
	// 82FFB2E0: 4BFFFF94  b 0x82ffb274
	pc = 0x82FFB274; continue 'dispatch;
	// 82FFB2E4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFB2E8: 4BFFFBF1  bl 0x82ffaed8
	ctx.lr = 0x82FFB2EC;
	sub_82FFAED8(ctx, base);
	// 82FFB2EC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFB2F0: 4BFFFBE9  bl 0x82ffaed8
	ctx.lr = 0x82FFB2F4;
	sub_82FFAED8(ctx, base);
	// 82FFB2F4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FFB2F8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB2FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFB300: 4BFFF161  bl 0x82ffa460
	ctx.lr = 0x82FFB304;
	sub_82FFA460(ctx, base);
	// 82FFB304: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB308: 4182001C  beq 0x82ffb324
	if ctx.cr[0].eq {
	pc = 0x82FFB324; continue 'dispatch;
	}
	// 82FFB30C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB310: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB314: 41820010  beq 0x82ffb324
	if ctx.cr[0].eq {
	pc = 0x82FFB324; continue 'dispatch;
	}
	// 82FFB318: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FFB31C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB320: 4BFFF209  bl 0x82ffa528
	ctx.lr = 0x82FFB324;
	sub_82FFA528(ctx, base);
	// 82FFB324: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FFB328: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB32C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFB330: 4BFFF829  bl 0x82ffab58
	ctx.lr = 0x82FFB334;
	sub_82FFAB58(ctx, base);
	// 82FFB334: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FFB338: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FFB33C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB340: 4BFFF819  bl 0x82ffab58
	ctx.lr = 0x82FFB344;
	sub_82FFAB58(ctx, base);
	// 82FFB344: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FFB348: 481ACE68  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB34C size=44
    let mut pc: u32 = 0x82FFB34C;
    'dispatch: loop {
        match pc {
            0x82FFB34C => {
    //   block [0x82FFB34C..0x82FFB378)
	// 82FFB34C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB35C: 809F00FC  lwz r4, 0xfc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82FFB360: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFB364: 4BFDCF7D  bl 0x82fd82e0
	ctx.lr = 0x82FFB368;
	sub_82FD82E0(ctx, base);
	// 82FFB368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB378 size=44
    let mut pc: u32 = 0x82FFB378;
    'dispatch: loop {
        match pc {
            0x82FFB378 => {
    //   block [0x82FFB378..0x82FFB3A4)
	// 82FFB378: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB37C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB380: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB388: 809F00FC  lwz r4, 0xfc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82FFB38C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFB390: 4BFDCF51  bl 0x82fd82e0
	ctx.lr = 0x82FFB394;
	sub_82FD82E0(ctx, base);
	// 82FFB394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB3A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB3A4 size=40
    let mut pc: u32 = 0x82FFB3A4;
    'dispatch: loop {
        match pc {
            0x82FFB3A4 => {
    //   block [0x82FFB3A4..0x82FFB3CC)
	// 82FFB3A4: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB3B4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFB3B8: 4BFFFB21  bl 0x82ffaed8
	ctx.lr = 0x82FFB3BC;
	sub_82FFAED8(ctx, base);
	// 82FFB3BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB3CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB3CC size=40
    let mut pc: u32 = 0x82FFB3CC;
    'dispatch: loop {
        match pc {
            0x82FFB3CC => {
    //   block [0x82FFB3CC..0x82FFB3F4)
	// 82FFB3CC: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB3DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFB3E0: 4BFFFAF9  bl 0x82ffaed8
	ctx.lr = 0x82FFB3E4;
	sub_82FFAED8(ctx, base);
	// 82FFB3E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFB3F8 size=8
    let mut pc: u32 = 0x82FFB3F8;
    'dispatch: loop {
        match pc {
            0x82FFB3F8 => {
    //   block [0x82FFB3F8..0x82FFB400)
	// 82FFB3F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFB3FC: 8213FB30  lwz r16, -0x4d0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1232 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB400 size=500
    let mut pc: u32 = 0x82FFB400;
    'dispatch: loop {
        match pc {
            0x82FFB400 => {
    //   block [0x82FFB400..0x82FFB5F4)
	// 82FFB400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB404: 481ACD5D  bl 0x831a8160
	ctx.lr = 0x82FFB408;
	sub_831A8130(ctx, base);
	// 82FFB408: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FFB40C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB410: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB414: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FFB418: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FFB41C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB420: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFB424: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB428: 4BFE3A31  bl 0x82fdee58
	ctx.lr = 0x82FFB42C;
	sub_82FDEE58(ctx, base);
	// 82FFB42C: 3880004E  li r4, 0x4e
	ctx.r[4].s64 = 78;
	// 82FFB430: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB434: 4BFD56E5  bl 0x82fd0b18
	ctx.lr = 0x82FFB438;
	sub_82FD0B18(ctx, base);
	// 82FFB438: 38800053  li r4, 0x53
	ctx.r[4].s64 = 83;
	// 82FFB43C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB440: 4BFD56D9  bl 0x82fd0b18
	ctx.lr = 0x82FFB444;
	sub_82FD0B18(ctx, base);
	// 82FFB444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFB448: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB44C: 4BFFED35  bl 0x82ffa180
	ctx.lr = 0x82FFB450;
	sub_82FFA180(ctx, base);
	// 82FFB450: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFB454: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB458: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB45C: 4BFDE115  bl 0x82fd9570
	ctx.lr = 0x82FFB460;
	sub_82FD9570(ctx, base);
	// 82FFB460: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFB464: 48000044  b 0x82ffb4a8
	pc = 0x82FFB4A8; continue 'dispatch;
	// 82FFB468: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB46C: 3880004E  li r4, 0x4e
	ctx.r[4].s64 = 78;
	// 82FFB470: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB474: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FFB478: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FFB47C: 4BFD569D  bl 0x82fd0b18
	ctx.lr = 0x82FFB480;
	sub_82FD0B18(ctx, base);
	// 82FFB480: 38800053  li r4, 0x53
	ctx.r[4].s64 = 83;
	// 82FFB484: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB488: 4BFD5691  bl 0x82fd0b18
	ctx.lr = 0x82FFB48C;
	sub_82FD0B18(ctx, base);
	// 82FFB48C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFB490: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB494: 4BFFECED  bl 0x82ffa180
	ctx.lr = 0x82FFB498;
	sub_82FFA180(ctx, base);
	// 82FFB498: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFB49C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB4A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB4A4: 4BFDE0CD  bl 0x82fd9570
	ctx.lr = 0x82FFB4A8;
	sub_82FD9570(ctx, base);
	// 82FFB4A8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB4AC: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFB4B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFB4B4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB4B8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFB4BC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB4C0: 7FAA5B2E  sthx r29, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u16) };
	// 82FFB4C4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFB4C8: 835F0068  lwz r26, 0x68(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB4CC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB4D0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB4D4: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFB4D8: 48031399  bl 0x8302c870
	ctx.lr = 0x82FFB4DC;
	sub_8302C870(ctx, base);
	// 82FFB4DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FFB4E0: 4BFFF931  bl 0x82ffae10
	ctx.lr = 0x82FFB4E4;
	sub_82FFAE10(ctx, base);
	// 82FFB4E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB4E8: 4082FF80  bne 0x82ffb468
	if !ctx.cr[0].eq {
	pc = 0x82FFB468; continue 'dispatch;
	}
	// 82FFB4EC: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FFB4F0: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB4F4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FFB4F8: 4BFE3961  bl 0x82fdee58
	ctx.lr = 0x82FFB4FC;
	sub_82FDEE58(ctx, base);
	// 82FFB4FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFB500: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB504: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82FFB508: 388B8018  addi r4, r11, -0x7fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -32744;
	// 82FFB50C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FFB510: 4BFDE061  bl 0x82fd9570
	ctx.lr = 0x82FFB514;
	sub_82FD9570(ctx, base);
	// 82FFB514: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FFB518: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FFB51C: 4BFD55FD  bl 0x82fd0b18
	ctx.lr = 0x82FFB520;
	sub_82FD0B18(ctx, base);
	// 82FFB520: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFB524: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB528: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFB52C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFB530: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FFB534: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FFB538: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB53C: 4BFDE035  bl 0x82fd9570
	ctx.lr = 0x82FFB540;
	sub_82FD9570(ctx, base);
	// 82FFB540: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FFB544: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FFB548: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB54C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB550: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFB554: 3BCB8034  addi r30, r11, -0x7fcc
	ctx.r[30].s64 = ctx.r[11].s64 + -32716;
	// 82FFB558: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FFB55C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFB560: 7FAA5B2E  sthx r29, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u16) };
	// 82FFB564: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB568: 80BF0088  lwz r5, 0x88(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FFB56C: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FFB570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB574: 4E800421  bctrl
	ctx.lr = 0x82FFB578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB578: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFB57C: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB580: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB584: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFB588: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFB58C: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 82FFB590: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB594: 80BF0068  lwz r5, 0x68(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB598: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FFB59C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB5A0: 4E800421  bctrl
	ctx.lr = 0x82FFB5A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB5A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB5A8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFB5AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB5B0: 4E800421  bctrl
	ctx.lr = 0x82FFB5B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB5B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB5B8: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FFB5BC: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FFB5C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB5C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB5C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB5CC: 4E800421  bctrl
	ctx.lr = 0x82FFB5D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB5D0: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFB5D4: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFB5D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB5DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB5E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB5E4: 4E800421  bctrl
	ctx.lr = 0x82FFB5E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB5E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFB5EC: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FFB5F0: 481ACBC0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB5F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB5F4 size=40
    let mut pc: u32 = 0x82FFB5F4;
    'dispatch: loop {
        match pc {
            0x82FFB5F4 => {
    //   block [0x82FFB5F4..0x82FFB61C)
	// 82FFB5F4: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB604: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFB608: 4BFE38D1  bl 0x82fdeed8
	ctx.lr = 0x82FFB60C;
	sub_82FDEED8(ctx, base);
	// 82FFB60C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB61C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB61C size=40
    let mut pc: u32 = 0x82FFB61C;
    'dispatch: loop {
        match pc {
            0x82FFB61C => {
    //   block [0x82FFB61C..0x82FFB644)
	// 82FFB61C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FFB620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB62C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FFB630: 4BFE38A9  bl 0x82fdeed8
	ctx.lr = 0x82FFB634;
	sub_82FDEED8(ctx, base);
	// 82FFB634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB648 size=96
    let mut pc: u32 = 0x82FFB648;
    'dispatch: loop {
        match pc {
            0x82FFB648 => {
    //   block [0x82FFB648..0x82FFB6A8)
	// 82FFB648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB64C: 481ACB19  bl 0x831a8164
	ctx.lr = 0x82FFB650;
	sub_831A8130(ctx, base);
	// 82FFB650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFB658: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFB65C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FFB660: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FFB664: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB668: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB66C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB670: 4082000C  bne 0x82ffb67c
	if !ctx.cr[0].eq {
	pc = 0x82FFB67C; continue 'dispatch;
	}
	// 82FFB674: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFB678: 4BFFF739  bl 0x82ffadb0
	ctx.lr = 0x82FFB67C;
	sub_82FFADB0(ctx, base);
	// 82FFB67C: 389EFFFF  addi r4, r30, -1
	ctx.r[4].s64 = ctx.r[30].s64 + -1;
	// 82FFB680: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB684: 480311ED  bl 0x8302c870
	ctx.lr = 0x82FFB688;
	sub_8302C870(ctx, base);
	// 82FFB688: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FFB68C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FFB690: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFB694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB698: 4BFFFA99  bl 0x82ffb130
	ctx.lr = 0x82FFB69C;
	sub_82FFB130(ctx, base);
	// 82FFB69C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FFB6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFB6A4: 481ACB10  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFB6A8 size=8
    let mut pc: u32 = 0x82FFB6A8;
    'dispatch: loop {
        match pc {
            0x82FFB6A8 => {
    //   block [0x82FFB6A8..0x82FFB6B0)
	// 82FFB6A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFB6AC: 8213FB88  lwz r16, -0x478(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-1144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB6B0 size=164
    let mut pc: u32 = 0x82FFB6B0;
    'dispatch: loop {
        match pc {
            0x82FFB6B0 => {
    //   block [0x82FFB6B0..0x82FFB754)
	// 82FFB6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB6B4: 481ACAB1  bl 0x831a8164
	ctx.lr = 0x82FFB6B8;
	sub_831A8130(ctx, base);
	// 82FFB6B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFB6BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB6C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFB6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFB6C8: 396BF988  addi r11, r11, -0x678
	ctx.r[11].s64 = ctx.r[11].s64 + -1656;
	// 82FFB6CC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FFB6D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFB6D4: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB6D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB6DC: 4182004C  beq 0x82ffb728
	if ctx.cr[0].eq {
	pc = 0x82FFB728; continue 'dispatch;
	}
	// 82FFB6E0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB6E4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFB6E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFB6EC: 4099003C  ble cr6, 0x82ffb728
	if !ctx.cr[6].gt {
	pc = 0x82FFB728; continue 'dispatch;
	}
	// 82FFB6F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFB6F4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB6F8: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FFB6FC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB700: 41820014  beq 0x82ffb714
	if ctx.cr[0].eq {
	pc = 0x82FFB714; continue 'dispatch;
	}
	// 82FFB704: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB708: 4BFFF9B9  bl 0x82ffb0c0
	ctx.lr = 0x82FFB70C;
	sub_82FFB0C0(ctx, base);
	// 82FFB70C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB710: 4BFDCBD1  bl 0x82fd82e0
	ctx.lr = 0x82FFB714;
	sub_82FD82E0(ctx, base);
	// 82FFB714: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB718: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FFB71C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FFB720: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB724: 4198FFD0  blt cr6, 0x82ffb6f4
	if ctx.cr[6].lt {
	pc = 0x82FFB6F4; continue 'dispatch;
	}
	// 82FFB728: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB72C: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFB734: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFB73C: 4E800421  bctrl
	ctx.lr = 0x82FFB740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFB740: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFB744: 396BF890  addi r11, r11, -0x770
	ctx.r[11].s64 = ctx.r[11].s64 + -1904;
	// 82FFB748: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFB74C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFB750: 481ACA64  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB754 size=40
    let mut pc: u32 = 0x82FFB754;
    'dispatch: loop {
        match pc {
            0x82FFB754 => {
    //   block [0x82FFB754..0x82FFB77C)
	// 82FFB754: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFB758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB764: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFB768: 4BFFEC59  bl 0x82ffa3c0
	ctx.lr = 0x82FFB76C;
	sub_82FFA3C0(ctx, base);
	// 82FFB76C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFB778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB780 size=148
    let mut pc: u32 = 0x82FFB780;
    'dispatch: loop {
        match pc {
            0x82FFB780 => {
    //   block [0x82FFB780..0x82FFB814)
	// 82FFB780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB784: 481AC9E5  bl 0x831a8168
	ctx.lr = 0x82FFB788;
	sub_831A8130(ctx, base);
	// 82FFB788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFB790: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFB794: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFB798: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB79C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB7A0: 41980030  blt cr6, 0x82ffb7d0
	if ctx.cr[6].lt {
	pc = 0x82FFB7D0; continue 'dispatch;
	}
	// 82FFB7A4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FFB7A8: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB7AC: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FFB7B0: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FFB7B4: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 82FFB7B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFB7BC: 4BFD519D  bl 0x82fd0958
	ctx.lr = 0x82FFB7C0;
	sub_82FD0958(ctx, base);
	// 82FFB7C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFB7C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFB7C8: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FFB7CC: 481B545D  bl 0x831b0c28
	ctx.lr = 0x82FFB7D0;
	sub_831B0C28(ctx, base);
	// 82FFB7D0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB7D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB7D8: 41820028  beq 0x82ffb800
	if ctx.cr[0].eq {
	pc = 0x82FFB800; continue 'dispatch;
	}
	// 82FFB7DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB7E0: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB7E4: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFB7E8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB7EC: 41820014  beq 0x82ffb800
	if ctx.cr[0].eq {
	pc = 0x82FFB800; continue 'dispatch;
	}
	// 82FFB7F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFB7F4: 4BFFF8CD  bl 0x82ffb0c0
	ctx.lr = 0x82FFB7F8;
	sub_82FFB0C0(ctx, base);
	// 82FFB7F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFB7FC: 4BFDCAE5  bl 0x82fd82e0
	ctx.lr = 0x82FFB800;
	sub_82FD82E0(ctx, base);
	// 82FFB800: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB804: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB808: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82FFB80C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFB810: 481AC9A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB818 size=124
    let mut pc: u32 = 0x82FFB818;
    'dispatch: loop {
        match pc {
            0x82FFB818 => {
    //   block [0x82FFB818..0x82FFB894)
	// 82FFB818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB81C: 481AC949  bl 0x831a8164
	ctx.lr = 0x82FFB820;
	sub_831A8130(ctx, base);
	// 82FFB820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFB828: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFB82C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82FFB830: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFB838: 40990050  ble cr6, 0x82ffb888
	if !ctx.cr[6].gt {
	pc = 0x82FFB888; continue 'dispatch;
	}
	// 82FFB83C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82FFB840: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB844: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB848: 41820024  beq 0x82ffb86c
	if ctx.cr[0].eq {
	pc = 0x82FFB86C; continue 'dispatch;
	}
	// 82FFB84C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB850: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FFB854: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB858: 41820014  beq 0x82ffb86c
	if ctx.cr[0].eq {
	pc = 0x82FFB86C; continue 'dispatch;
	}
	// 82FFB85C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB860: 4BFFF861  bl 0x82ffb0c0
	ctx.lr = 0x82FFB864;
	sub_82FFB0C0(ctx, base);
	// 82FFB864: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFB868: 4BFDCA79  bl 0x82fd82e0
	ctx.lr = 0x82FFB86C;
	sub_82FD82E0(ctx, base);
	// 82FFB86C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB870: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FFB874: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 82FFB878: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FFB87C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB880: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB884: 4198FFBC  blt cr6, 0x82ffb840
	if ctx.cr[6].lt {
	pc = 0x82FFB840; continue 'dispatch;
	}
	// 82FFB888: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FFB88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFB890: 481AC924  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB898 size=260
    let mut pc: u32 = 0x82FFB898;
    'dispatch: loop {
        match pc {
            0x82FFB898 => {
    //   block [0x82FFB898..0x82FFB99C)
	// 82FFB898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB89C: 481AC8D1  bl 0x831a816c
	ctx.lr = 0x82FFB8A0;
	sub_831A8130(ctx, base);
	// 82FFB8A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB8A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFB8A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFB8AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB8B0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB8B4: 41980030  blt cr6, 0x82ffb8e4
	if ctx.cr[6].lt {
	pc = 0x82FFB8E4; continue 'dispatch;
	}
	// 82FFB8B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FFB8BC: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFB8C0: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FFB8C4: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FFB8C8: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82FFB8CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFB8D0: 4BFD5089  bl 0x82fd0958
	ctx.lr = 0x82FFB8D4;
	sub_82FD0958(ctx, base);
	// 82FFB8D4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFB8D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFB8DC: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FFB8E0: 481B5349  bl 0x831b0c28
	ctx.lr = 0x82FFB8E4;
	sub_831B0C28(ctx, base);
	// 82FFB8E4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB8E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB8EC: 41820028  beq 0x82ffb914
	if ctx.cr[0].eq {
	pc = 0x82FFB914; continue 'dispatch;
	}
	// 82FFB8F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB8F4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB8F8: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFB8FC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB900: 41820014  beq 0x82ffb914
	if ctx.cr[0].eq {
	pc = 0x82FFB914; continue 'dispatch;
	}
	// 82FFB904: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFB908: 4BFFF7B9  bl 0x82ffb0c0
	ctx.lr = 0x82FFB90C;
	sub_82FFB0C0(ctx, base);
	// 82FFB90C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFB910: 4BFDC9D1  bl 0x82fd82e0
	ctx.lr = 0x82FFB914;
	sub_82FD82E0(ctx, base);
	// 82FFB914: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB918: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFB91C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB920: 409A0018  bne cr6, 0x82ffb938
	if !ctx.cr[6].eq {
	pc = 0x82FFB938; continue 'dispatch;
	}
	// 82FFB924: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB928: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFB930: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FFB934: 48000054  b 0x82ffb988
	pc = 0x82FFB988; continue 'dispatch;
	// 82FFB938: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FFB93C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFB940: 40980030  bge cr6, 0x82ffb970
	if !ctx.cr[6].lt {
	pc = 0x82FFB970; continue 'dispatch;
	}
	// 82FFB944: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFB948: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB94C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FFB950: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FFB954: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FFB958: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB95C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FFB960: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB964: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FFB968: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FFB96C: 4198FFDC  blt cr6, 0x82ffb948
	if ctx.cr[6].lt {
	pc = 0x82FFB948; continue 'dispatch;
	}
	// 82FFB970: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FFB978: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB97C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFB980: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFB984: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82FFB988: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB98C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFB990: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFB994: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFB998: 481AC824  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFB9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFB9A0 size=104
    let mut pc: u32 = 0x82FFB9A0;
    'dispatch: loop {
        match pc {
            0x82FFB9A0 => {
    //   block [0x82FFB9A0..0x82FFBA08)
	// 82FFB9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFB9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFB9A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFB9AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFB9B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFB9B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB9B8: 4182003C  beq 0x82ffb9f4
	if ctx.cr[0].eq {
	pc = 0x82FFB9F4; continue 'dispatch;
	}
	// 82FFB9BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFB9C0: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFB9C4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB9C8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFB9CC: 41820028  beq 0x82ffb9f4
	if ctx.cr[0].eq {
	pc = 0x82FFB9F4; continue 'dispatch;
	}
	// 82FFB9D0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFB9D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFB9D8: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FFB9DC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFB9E0: 41820014  beq 0x82ffb9f4
	if ctx.cr[0].eq {
	pc = 0x82FFB9F4; continue 'dispatch;
	}
	// 82FFB9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB9E8: 4BFFF6D9  bl 0x82ffb0c0
	ctx.lr = 0x82FFB9EC;
	sub_82FFB0C0(ctx, base);
	// 82FFB9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFB9F0: 4BFDC8F1  bl 0x82fd82e0
	ctx.lr = 0x82FFB9F4;
	sub_82FD82E0(ctx, base);
	// 82FFB9F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFB9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFB9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFBA00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFBA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFBA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFBA08 size=132
    let mut pc: u32 = 0x82FFBA08;
    'dispatch: loop {
        match pc {
            0x82FFBA08 => {
    //   block [0x82FFBA08..0x82FFBA8C)
	// 82FFBA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFBA0C: 481AC75D  bl 0x831a8168
	ctx.lr = 0x82FFBA10;
	sub_831A8130(ctx, base);
	// 82FFBA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFBA14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFBA18: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFBA1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFBA20: 4182004C  beq 0x82ffba6c
	if ctx.cr[0].eq {
	pc = 0x82FFBA6C; continue 'dispatch;
	}
	// 82FFBA24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBA28: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FFBA2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFBA30: 4099003C  ble cr6, 0x82ffba6c
	if !ctx.cr[6].gt {
	pc = 0x82FFBA6C; continue 'dispatch;
	}
	// 82FFBA34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFBA38: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFBA3C: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FFBA40: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFBA44: 41820014  beq 0x82ffba58
	if ctx.cr[0].eq {
	pc = 0x82FFBA58; continue 'dispatch;
	}
	// 82FFBA48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFBA4C: 4BFFF675  bl 0x82ffb0c0
	ctx.lr = 0x82FFBA50;
	sub_82FFB0C0(ctx, base);
	// 82FFBA50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFBA54: 4BFDC88D  bl 0x82fd82e0
	ctx.lr = 0x82FFBA58;
	sub_82FD82E0(ctx, base);
	// 82FFBA58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBA5C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FFBA60: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FFBA64: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFBA68: 4198FFD0  blt cr6, 0x82ffba38
	if ctx.cr[6].lt {
	pc = 0x82FFBA38; continue 'dispatch;
	}
	// 82FFBA6C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBA70: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFBA74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBA78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBA7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBA80: 4E800421  bctrl
	ctx.lr = 0x82FFBA84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFBA88: 481AC730  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFBA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFBA90 size=76
    let mut pc: u32 = 0x82FFBA90;
    'dispatch: loop {
        match pc {
            0x82FFBA90 => {
    //   block [0x82FFBA90..0x82FFBADC)
	// 82FFBA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFBA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFBA98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFBA9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFBAA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFBAA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFBAA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFBAAC: 4BFFFC05  bl 0x82ffb6b0
	ctx.lr = 0x82FFBAB0;
	sub_82FFB6B0(ctx, base);
	// 82FFBAB0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBAB4: 4182000C  beq 0x82ffbac0
	if ctx.cr[0].eq {
	pc = 0x82FFBAC0; continue 'dispatch;
	}
	// 82FFBAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBABC: 4BFDC825  bl 0x82fd82e0
	ctx.lr = 0x82FFBAC0;
	sub_82FD82E0(ctx, base);
	// 82FFBAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBAC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFBAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFBACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFBAD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFBAD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFBAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFBAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFBAE0 size=1008
    let mut pc: u32 = 0x82FFBAE0;
    'dispatch: loop {
        match pc {
            0x82FFBAE0 => {
    //   block [0x82FFBAE0..0x82FFBED0)
	// 82FFBAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFBAE4: 481AC669  bl 0x831a814c
	ctx.lr = 0x82FFBAE8;
	sub_831A8130(ctx, base);
	// 82FFBAE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFBAEC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82FFBAF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFBAF4: 82F60024  lwz r23, 0x24(r22)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFBAF8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FFBAFC: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBB00: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFBB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBB08: 4E800421  bctrl
	ctx.lr = 0x82FFBB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBB0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFBB10: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FFBB14: 3B6B8158  addi r27, r11, -0x7ea8
	ctx.r[27].s64 = ctx.r[11].s64 + -32424;
	// 82FFBB18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFBB1C: 3AAB8034  addi r21, r11, -0x7fcc
	ctx.r[21].s64 = ctx.r[11].s64 + -32716;
	// 82FFBB20: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82FFBB24: 4081010C  ble 0x82ffbc30
	if !ctx.cr[0].gt {
	pc = 0x82FFBC30; continue 'dispatch;
	}
	// 82FFBB28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFBB2C: 3B0B8018  addi r24, r11, -0x7fe8
	ctx.r[24].s64 = ctx.r[11].s64 + -32744;
	// 82FFBB30: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBB34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FFBB38: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FFBB3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBB44: 4E800421  bctrl
	ctx.lr = 0x82FFBB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBB48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFBB4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBB50: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FFBB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBB58: 4E800421  bctrl
	ctx.lr = 0x82FFBB5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBB5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBB64: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFBB68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBB6C: 4E800421  bctrl
	ctx.lr = 0x82FFBB70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBB70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBB74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFBB78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBB7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBB80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBB84: 4E800421  bctrl
	ctx.lr = 0x82FFBB88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBB88: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFBB8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFBB90: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FFBB94: 4BFD80AD  bl 0x82fd3c40
	ctx.lr = 0x82FFBB98;
	sub_82FD3C40(ctx, base);
	// 82FFBB98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBB9C: 41820088  beq 0x82ffbc24
	if ctx.cr[0].eq {
	pc = 0x82FFBC24; continue 'dispatch;
	}
	// 82FFBBA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBBA4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FFBBA8: 4BFD8099  bl 0x82fd3c40
	ctx.lr = 0x82FFBBAC;
	sub_82FD3C40(ctx, base);
	// 82FFBBAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBBB0: 41820018  beq 0x82ffbbc8
	if ctx.cr[0].eq {
	pc = 0x82FFBBC8; continue 'dispatch;
	}
	// 82FFBBB4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82FFBBB8: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 82FFBBBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBBC0: 4BFFEE61  bl 0x82ffaa20
	ctx.lr = 0x82FFBBC4;
	sub_82FFAA20(ctx, base);
	// 82FFBBC4: 48000060  b 0x82ffbc24
	pc = 0x82FFBC24; continue 'dispatch;
	// 82FFBBC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBBD0: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FFBBD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBBD8: 4E800421  bctrl
	ctx.lr = 0x82FFBBDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBBDC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FFBBE0: 4BFD8061  bl 0x82fd3c40
	ctx.lr = 0x82FFBBE4;
	sub_82FD3C40(ctx, base);
	// 82FFBBE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBBE8: 41820028  beq 0x82ffbc10
	if ctx.cr[0].eq {
	pc = 0x82FFBC10; continue 'dispatch;
	}
	// 82FFBBEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBBF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBBF4: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBBF8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFBBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBC00: 4E800421  bctrl
	ctx.lr = 0x82FFBC04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBC04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFBC08: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FFBC0C: 4800000C  b 0x82ffbc18
	pc = 0x82FFBC18; continue 'dispatch;
	// 82FFBC10: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBC14: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBC18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FFBC1C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBC20: 4BFFFA29  bl 0x82ffb648
	ctx.lr = 0x82FFBC24;
	sub_82FFB648(ctx, base);
	// 82FFBC24: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FFBC28: 7F1AC800  cmpw cr6, r26, r25
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FFBC2C: 4198FF04  blt cr6, 0x82ffbb30
	if ctx.cr[6].lt {
	pc = 0x82FFBB30; continue 'dispatch;
	}
	// 82FFBC30: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBC34: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FFBC38: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FFBC3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBC40: 4E800421  bctrl
	ctx.lr = 0x82FFBC44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBC44: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FFBC48: 40820008  bne 0x82ffbc50
	if !ctx.cr[0].eq {
	pc = 0x82FFBC50; continue 'dispatch;
	}
	// 82FFBC4C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82FFBC50: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBC54: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FFBC58: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFBC5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBC60: 4E800421  bctrl
	ctx.lr = 0x82FFBC64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBC64: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFBC68: 40820008  bne 0x82ffbc70
	if !ctx.cr[0].eq {
	pc = 0x82FFBC70; continue 'dispatch;
	}
	// 82FFBC6C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82FFBC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBC74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBC78: 4BFD7FC9  bl 0x82fd3c40
	ctx.lr = 0x82FFBC7C;
	sub_82FD3C40(ctx, base);
	// 82FFBC7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBC80: 4082003C  bne 0x82ffbcbc
	if !ctx.cr[0].eq {
	pc = 0x82FFBCBC; continue 'dispatch;
	}
	// 82FFBC84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBC88: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBC8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFBC90: 4BFFF3C1  bl 0x82ffb050
	ctx.lr = 0x82FFBC94;
	sub_82FFB050(ctx, base);
	// 82FFBC94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBC98: 40820094  bne 0x82ffbd2c
	if !ctx.cr[0].eq {
	pc = 0x82FFBD2C; continue 'dispatch;
	}
	// 82FFBC9C: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82FFBCA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBCA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFBCA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBCAC: 4BFFE5ED  bl 0x82ffa298
	ctx.lr = 0x82FFBCB0;
	sub_82FFA298(ctx, base);
	// 82FFBCB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBCB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFBCB8: 48000068  b 0x82ffbd20
	pc = 0x82FFBD20; continue 'dispatch;
	// 82FFBCBC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBCC0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FFBCC4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFBCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBCCC: 4E800421  bctrl
	ctx.lr = 0x82FFBCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBCD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFBCD4: 40820018  bne 0x82ffbcec
	if !ctx.cr[0].eq {
	pc = 0x82FFBCEC; continue 'dispatch;
	}
	// 82FFBCD8: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82FFBCDC: 388000A9  li r4, 0xa9
	ctx.r[4].s64 = 169;
	// 82FFBCE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBCE4: 4BFFED3D  bl 0x82ffaa20
	ctx.lr = 0x82FFBCE8;
	sub_82FFAA20(ctx, base);
	// 82FFBCE8: 48000044  b 0x82ffbd2c
	pc = 0x82FFBD2C; continue 'dispatch;
	// 82FFBCEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FFBCF0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBCF4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBCF8: 4BFFF359  bl 0x82ffb050
	ctx.lr = 0x82FFBCFC;
	sub_82FFB050(ctx, base);
	// 82FFBCFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBD00: 4082002C  bne 0x82ffbd2c
	if !ctx.cr[0].eq {
	pc = 0x82FFBD2C; continue 'dispatch;
	}
	// 82FFBD04: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82FFBD08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FFBD0C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBD10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBD14: 4BFFE585  bl 0x82ffa298
	ctx.lr = 0x82FFBD18;
	sub_82FFA298(ctx, base);
	// 82FFBD18: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FFBD1C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBD20: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBD24: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBD28: 4BFFF921  bl 0x82ffb648
	ctx.lr = 0x82FFBD2C;
	sub_82FFB648(ctx, base);
	// 82FFBD2C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBD30: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FFBD34: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFBD38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBD3C: 4E800421  bctrl
	ctx.lr = 0x82FFBD40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBD40: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFBD44: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FFBD48: 40810180  ble 0x82ffbec8
	if !ctx.cr[0].gt {
	pc = 0x82FFBEC8; continue 'dispatch;
	}
	// 82FFBD4C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBD50: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FFBD54: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FFBD58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBD5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBD60: 4E800421  bctrl
	ctx.lr = 0x82FFBD64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBD64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFBD68: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBD6C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFBD70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBD74: 4E800421  bctrl
	ctx.lr = 0x82FFBD78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBD78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBD7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFBD80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFBD84: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FFBD88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBD8C: 4E800421  bctrl
	ctx.lr = 0x82FFBD90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBD90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFBD94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFBD98: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FFBD9C: 4BFD7EA5  bl 0x82fd3c40
	ctx.lr = 0x82FFBDA0;
	sub_82FD3C40(ctx, base);
	// 82FFBDA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBDA4: 40820118  bne 0x82ffbebc
	if !ctx.cr[0].eq {
	pc = 0x82FFBEBC; continue 'dispatch;
	}
	// 82FFBDA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFBDAC: 419A00E4  beq cr6, 0x82ffbe90
	if ctx.cr[6].eq {
	pc = 0x82FFBE90; continue 'dispatch;
	}
	// 82FFBDB0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFBDB4: 419A001C  beq cr6, 0x82ffbdd0
	if ctx.cr[6].eq {
	pc = 0x82FFBDD0; continue 'dispatch;
	}
	// 82FFBDB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBDBC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBDC0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBDC4: 4BFFF28D  bl 0x82ffb050
	ctx.lr = 0x82FFBDC8;
	sub_82FFB050(ctx, base);
	// 82FFBDC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBDCC: 408200F0  bne 0x82ffbebc
	if !ctx.cr[0].eq {
	pc = 0x82FFBEBC; continue 'dispatch;
	}
	// 82FFBDD0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBDD4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBDD8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBDDC: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFBDE0: 48030A91  bl 0x8302c870
	ctx.lr = 0x82FFBDE4;
	sub_8302C870(ctx, base);
	// 82FFBDE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFBDE8: 4BFFF089  bl 0x82ffae70
	ctx.lr = 0x82FFBDEC;
	sub_82FFAE70(ctx, base);
	// 82FFBDEC: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FFBDF0: 40820088  bne 0x82ffbe78
	if !ctx.cr[0].eq {
	pc = 0x82FFBE78; continue 'dispatch;
	}
	// 82FFBDF4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFBDF8: 419A0054  beq cr6, 0x82ffbe4c
	if ctx.cr[6].eq {
	pc = 0x82FFBE4C; continue 'dispatch;
	}
	// 82FFBDFC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBE00: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBE04: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBE08: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFBE0C: 48030A65  bl 0x8302c870
	ctx.lr = 0x82FFBE10;
	sub_8302C870(ctx, base);
	// 82FFBE10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBE14: 4BFFEFFD  bl 0x82ffae10
	ctx.lr = 0x82FFBE18;
	sub_82FFAE10(ctx, base);
	// 82FFBE18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFBE1C: 40820030  bne 0x82ffbe4c
	if !ctx.cr[0].eq {
	pc = 0x82FFBE4C; continue 'dispatch;
	}
	// 82FFBE20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBE24: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBE28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBE2C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBE30: 4BFFF819  bl 0x82ffb648
	ctx.lr = 0x82FFBE34;
	sub_82FFB648(ctx, base);
	// 82FFBE34: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82FFBE38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBE3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBE40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBE44: 4BFFE455  bl 0x82ffa298
	ctx.lr = 0x82FFBE48;
	sub_82FFA298(ctx, base);
	// 82FFBE48: 48000074  b 0x82ffbebc
	pc = 0x82FFBEBC; continue 'dispatch;
	// 82FFBE4C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82FFBE50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFBE54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBE58: 4BFFF5A9  bl 0x82ffb400
	ctx.lr = 0x82FFBE5C;
	sub_82FFB400(ctx, base);
	// 82FFBE5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFBE60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFBE64: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFBE68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBE6C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBE70: 4BFFF7D9  bl 0x82ffb648
	ctx.lr = 0x82FFBE74;
	sub_82FFB648(ctx, base);
	// 82FFBE74: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFBE78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBE7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFBE80: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FFBE84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBE88: 4E800421  bctrl
	ctx.lr = 0x82FFBE8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBE8C: 48000030  b 0x82ffbebc
	pc = 0x82FFBEBC; continue 'dispatch;
	// 82FFBE90: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBE94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFBE98: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FFBE9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBEA0: 4E800421  bctrl
	ctx.lr = 0x82FFBEA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBEA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFBEA8: 40820014  bne 0x82ffbebc
	if !ctx.cr[0].eq {
	pc = 0x82FFBEBC; continue 'dispatch;
	}
	// 82FFBEAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFBEB0: 388000A9  li r4, 0xa9
	ctx.r[4].s64 = 169;
	// 82FFBEB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBEB8: 4BFFEB69  bl 0x82ffaa20
	ctx.lr = 0x82FFBEBC;
	sub_82FFAA20(ctx, base);
	// 82FFBEBC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FFBEC0: 7F1BD000  cmpw cr6, r27, r26
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FFBEC4: 4198FE88  blt cr6, 0x82ffbd4c
	if ctx.cr[6].lt {
	pc = 0x82FFBD4C; continue 'dispatch;
	}
	// 82FFBEC8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FFBECC: 481AC2D0  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFBED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFBED0 size=108
    let mut pc: u32 = 0x82FFBED0;
    'dispatch: loop {
        match pc {
            0x82FFBED0 => {
    //   block [0x82FFBED0..0x82FFBF3C)
	// 82FFBED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFBED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFBED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFBEDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFBEE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFBEE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBEE8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBEEC: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFBEF0: 48030981  bl 0x8302c870
	ctx.lr = 0x82FFBEF4;
	sub_8302C870(ctx, base);
	// 82FFBEF4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBEF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBEFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFBF00: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFBF04: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFBF08: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FFBF0C: 4809EB4D  bl 0x8309aa58
	ctx.lr = 0x82FFBF10;
	sub_8309AA58(ctx, base);
	// 82FFBF10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFBF14: 41820014  beq 0x82ffbf28
	if ctx.cr[0].eq {
	pc = 0x82FFBF28; continue 'dispatch;
	}
	// 82FFBF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBF1C: 4BFFF1A5  bl 0x82ffb0c0
	ctx.lr = 0x82FFBF20;
	sub_82FFB0C0(ctx, base);
	// 82FFBF20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBF24: 4BFDC3BD  bl 0x82fd82e0
	ctx.lr = 0x82FFBF28;
	sub_82FD82E0(ctx, base);
	// 82FFBF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFBF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFBF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFBF34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFBF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFBF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFBF40 size=1156
    let mut pc: u32 = 0x82FFBF40;
    'dispatch: loop {
        match pc {
            0x82FFBF40 => {
    //   block [0x82FFBF40..0x82FFC3C4)
	// 82FFBF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFBF44: 481AC225  bl 0x831a8168
	ctx.lr = 0x82FFBF48;
	sub_831A8130(ctx, base);
	// 82FFBF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFBF4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFBF50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFBF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBF58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBF5C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBF60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBF64: 4E800421  bctrl
	ctx.lr = 0x82FFBF68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBF68: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFBF6C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFBF70: 419A0334  beq cr6, 0x82ffc2a4
	if ctx.cr[6].eq {
	pc = 0x82FFC2A4; continue 'dispatch;
	}
	// 82FFBF74: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFBF78: 419A0224  beq cr6, 0x82ffc19c
	if ctx.cr[6].eq {
	pc = 0x82FFC19C; continue 'dispatch;
	}
	// 82FFBF7C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82FFBF80: 419A0114  beq cr6, 0x82ffc094
	if ctx.cr[6].eq {
	pc = 0x82FFC094; continue 'dispatch;
	}
	// 82FFBF84: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82FFBF88: 409A0430  bne cr6, 0x82ffc3b8
	if !ctx.cr[6].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFBF8C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFBF90: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFBF94: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFBF98: 40820420  bne 0x82ffc3b8
	if !ctx.cr[0].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFBF9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBFA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBFA4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFBFA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBFAC: 4E800421  bctrl
	ctx.lr = 0x82FFBFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBFB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBFB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFBFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFBFBC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFBFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBFC4: 4E800421  bctrl
	ctx.lr = 0x82FFBFC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBFC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFBFCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFBFD0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBFD4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFBFD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBFDC: 4E800421  bctrl
	ctx.lr = 0x82FFBFE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBFE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFBFE4: 419A03D4  beq cr6, 0x82ffc3b8
	if ctx.cr[6].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFBFE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFBFEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFBFF0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFBFF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFBFF8: 4E800421  bctrl
	ctx.lr = 0x82FFBFFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFBFFC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFC000: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFC004: 409A03B4  bne cr6, 0x82ffc3b8
	if !ctx.cr[6].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFC008: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC00C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC010: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC018: 4E800421  bctrl
	ctx.lr = 0x82FFC01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC01C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFC020: 41820398  beq 0x82ffc3b8
	if ctx.cr[0].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFC024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC02C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC034: 4E800421  bctrl
	ctx.lr = 0x82FFC038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC038: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFC03C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFC040: 409A0378  bne cr6, 0x82ffc3b8
	if !ctx.cr[6].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFC044: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC04C: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC050: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC058: 4E800421  bctrl
	ctx.lr = 0x82FFC05C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC05C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FFC060: 817C00AC  lwz r11, 0xac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFC064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFC068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC06C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC070: 4E800421  bctrl
	ctx.lr = 0x82FFC074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC074: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC07C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFC080: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC088: 4E800421  bctrl
	ctx.lr = 0x82FFC08C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC090: 4800032C  b 0x82ffc3bc
	pc = 0x82FFC3BC; continue 'dispatch;
	// 82FFC094: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC098: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC09C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC0A0: 40820318  bne 0x82ffc3b8
	if !ctx.cr[0].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFC0A4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC0AC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0B0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0B4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC0B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC0BC: 4E800421  bctrl
	ctx.lr = 0x82FFC0C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC0C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFC0C4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0C8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC0D0: 4E800421  bctrl
	ctx.lr = 0x82FFC0D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC0D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFC0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC0E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC0E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC0E8: 4E800421  bctrl
	ctx.lr = 0x82FFC0EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC0EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC0F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFC0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC0F8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFC0FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC100: 4E800421  bctrl
	ctx.lr = 0x82FFC104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC104: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFC10C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FFC110: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFC114: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC118: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FFC11C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC120: 4E800421  bctrl
	ctx.lr = 0x82FFC124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC124: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC128: 419A006C  beq cr6, 0x82ffc194
	if ctx.cr[6].eq {
	pc = 0x82FFC194; continue 'dispatch;
	}
	// 82FFC12C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC134: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC13C: 4E800421  bctrl
	ctx.lr = 0x82FFC140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC140: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFC144: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFC148: 409A004C  bne cr6, 0x82ffc194
	if !ctx.cr[6].eq {
	pc = 0x82FFC194; continue 'dispatch;
	}
	// 82FFC14C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC154: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC158: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC15C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC160: 4E800421  bctrl
	ctx.lr = 0x82FFC164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC164: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FFC168: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FFC16C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFC170: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFC174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC178: 4E800421  bctrl
	ctx.lr = 0x82FFC17C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC17C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC180: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC184: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC188: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFC18C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC190: 4E800421  bctrl
	ctx.lr = 0x82FFC194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC194: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFC198: 48000224  b 0x82ffc3bc
	pc = 0x82FFC3BC; continue 'dispatch;
	// 82FFC19C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC1A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC1A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC1AC: 4E800421  bctrl
	ctx.lr = 0x82FFC1B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC1B0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFC1B4: 4182006C  beq 0x82ffc220
	if ctx.cr[0].eq {
	pc = 0x82FFC220; continue 'dispatch;
	}
	// 82FFC1B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC1BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC1C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC1C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC1C8: 4E800421  bctrl
	ctx.lr = 0x82FFC1CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC1CC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFC1D0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFC1D4: 409A004C  bne cr6, 0x82ffc220
	if !ctx.cr[6].eq {
	pc = 0x82FFC220; continue 'dispatch;
	}
	// 82FFC1D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC1DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC1E0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC1E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC1E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC1EC: 4E800421  bctrl
	ctx.lr = 0x82FFC1F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC1F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFC1F4: 817D00A8  lwz r11, 0xa8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FFC1F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC1FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC200: 4E800421  bctrl
	ctx.lr = 0x82FFC204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC204: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC20C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC214: 4E800421  bctrl
	ctx.lr = 0x82FFC218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC21C: 4BFFFE60  b 0x82ffc07c
	pc = 0x82FFC07C; continue 'dispatch;
	// 82FFC220: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC228: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC22C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC230: 4E800421  bctrl
	ctx.lr = 0x82FFC234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC238: 41820034  beq 0x82ffc26c
	if ctx.cr[0].eq {
	pc = 0x82FFC26C; continue 'dispatch;
	}
	// 82FFC23C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC240: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC244: 41820028  beq 0x82ffc26c
	if ctx.cr[0].eq {
	pc = 0x82FFC26C; continue 'dispatch;
	}
	// 82FFC248: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FFC24C: 48000008  b 0x82ffc254
	pc = 0x82FFC254; continue 'dispatch;
	// 82FFC250: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FFC254: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC258: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC25C: 4082FFF4  bne 0x82ffc250
	if !ctx.cr[0].eq {
	pc = 0x82FFC250; continue 'dispatch;
	}
	// 82FFC260: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FFC264: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FFC268: 48000008  b 0x82ffc270
	pc = 0x82FFC270; continue 'dispatch;
	// 82FFC26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFC270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFC274: 409A0144  bne cr6, 0x82ffc3b8
	if !ctx.cr[6].eq {
	pc = 0x82FFC3B8; continue 'dispatch;
	}
	// 82FFC278: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC280: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC288: 4E800421  bctrl
	ctx.lr = 0x82FFC28C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC28C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFC294: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFC298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC29C: 4E800421  bctrl
	ctx.lr = 0x82FFC2A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC2A0: 48000118  b 0x82ffc3b8
	pc = 0x82FFC3B8; continue 'dispatch;
	// 82FFC2A4: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFC2A8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC2AC: 4BFFEB05  bl 0x82ffadb0
	ctx.lr = 0x82FFC2B0;
	sub_82FFADB0(ctx, base);
	// 82FFC2B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC2B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC2B8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FFC2BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC2C0: 4E800421  bctrl
	ctx.lr = 0x82FFC2C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC2C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC2C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFC2CC: A16B0004  lhz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC2D0: 556B0631  rlwinm. r11, r11, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC2D4: 41820014  beq 0x82ffc2e8
	if ctx.cr[0].eq {
	pc = 0x82FFC2E8; continue 'dispatch;
	}
	// 82FFC2D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFC2DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC2E0: 4BFFF801  bl 0x82ffbae0
	ctx.lr = 0x82FFC2E4;
	sub_82FFBAE0(ctx, base);
	// 82FFC2E4: 48000074  b 0x82ffc358
	pc = 0x82FFC358; continue 'dispatch;
	// 82FFC2E8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC2EC: 419A006C  beq cr6, 0x82ffc358
	if ctx.cr[6].eq {
	pc = 0x82FFC358; continue 'dispatch;
	}
	// 82FFC2F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC2F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FFC2F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC2FC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC304: 4E800421  bctrl
	ctx.lr = 0x82FFC308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC308: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC30C: 4182004C  beq 0x82ffc358
	if ctx.cr[0].eq {
	pc = 0x82FFC358; continue 'dispatch;
	}
	// 82FFC310: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC314: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFC318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC31C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC324: 4E800421  bctrl
	ctx.lr = 0x82FFC328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC328: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC32C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FFC330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC334: 4E800421  bctrl
	ctx.lr = 0x82FFC338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC338: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC33C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FFC340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC344: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC34C: 4E800421  bctrl
	ctx.lr = 0x82FFC350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC350: 7F1C1840  cmplw cr6, r28, r3
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FFC354: 4198FFBC  blt cr6, 0x82ffc310
	if ctx.cr[6].lt {
	pc = 0x82FFC310; continue 'dispatch;
	}
	// 82FFC358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC35C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC360: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFC364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC368: 4E800421  bctrl
	ctx.lr = 0x82FFC36C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC36C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFC370: 41820040  beq 0x82ffc3b0
	if ctx.cr[0].eq {
	pc = 0x82FFC3B0; continue 'dispatch;
	}
	// 82FFC374: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC37C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC384: 4E800421  bctrl
	ctx.lr = 0x82FFC388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC388: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFC38C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFC390: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC394: 4BFFFBAD  bl 0x82ffbf40
	ctx.lr = 0x82FFC398;
	sub_82FFBF40(ctx, base);
	// 82FFC398: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC39C: 41820008  beq 0x82ffc3a4
	if ctx.cr[0].eq {
	pc = 0x82FFC3A4; continue 'dispatch;
	}
	// 82FFC3A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFC3A4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FFC3A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC3AC: 409AFFC8  bne cr6, 0x82ffc374
	if !ctx.cr[6].eq {
	pc = 0x82FFC374; continue 'dispatch;
	}
	// 82FFC3B0: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC3B4: 4BFFFB1D  bl 0x82ffbed0
	ctx.lr = 0x82FFC3B8;
	sub_82FFBED0(ctx, base);
	// 82FFC3B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFC3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFC3C0: 481ABDF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC3C8 size=204
    let mut pc: u32 = 0x82FFC3C8;
    'dispatch: loop {
        match pc {
            0x82FFC3C8 => {
    //   block [0x82FFC3C8..0x82FFC494)
	// 82FFC3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC3CC: 481ABDA1  bl 0x831a816c
	ctx.lr = 0x82FFC3D0;
	sub_831A8130(ctx, base);
	// 82FFC3D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC3D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFC3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFC3DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC3E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFC3E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC3E8: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFC3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC3F0: 4E800421  bctrl
	ctx.lr = 0x82FFC3F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC3F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC3F8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FFC3FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFC400: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC404: 816A008C  lwz r11, 0x8c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FFC408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC40C: 4E800421  bctrl
	ctx.lr = 0x82FFC410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC410: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC414: 41820010  beq 0x82ffc424
	if ctx.cr[0].eq {
	pc = 0x82FFC424; continue 'dispatch;
	}
	// 82FFC418: 481581E9  bl 0x83154600
	ctx.lr = 0x82FFC41C;
	sub_83154600(ctx, base);
	// 82FFC41C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FFC420: 4800000C  b 0x82ffc42c
	pc = 0x82FFC42C; continue 'dispatch;
	// 82FFC424: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFC428: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFC42C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFC430: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FFC434: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFC438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC43C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFC440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC444: 4E800421  bctrl
	ctx.lr = 0x82FFC448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC448: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFC44C: 41820040  beq 0x82ffc48c
	if ctx.cr[0].eq {
	pc = 0x82FFC48C; continue 'dispatch;
	}
	// 82FFC450: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC458: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC45C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC460: 4E800421  bctrl
	ctx.lr = 0x82FFC464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC464: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFC468: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC46C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC470: 4BFFFAD1  bl 0x82ffbf40
	ctx.lr = 0x82FFC474;
	sub_82FFBF40(ctx, base);
	// 82FFC474: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC478: 41820008  beq 0x82ffc480
	if ctx.cr[0].eq {
	pc = 0x82FFC480; continue 'dispatch;
	}
	// 82FFC47C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFC480: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82FFC484: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FFC488: 409AFFC8  bne cr6, 0x82ffc450
	if !ctx.cr[6].eq {
	pc = 0x82FFC450; continue 'dispatch;
	}
	// 82FFC48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFC490: 481ABD2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC498 size=72
    let mut pc: u32 = 0x82FFC498;
    'dispatch: loop {
        match pc {
            0x82FFC498 => {
    //   block [0x82FFC498..0x82FFC4E0)
	// 82FFC498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFC4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFC4A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC4A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFC4AC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FFC4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFC4B4: 389FFFF4  addi r4, r31, -0xc
	ctx.r[4].s64 = ctx.r[31].s64 + -12;
	// 82FFC4B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFC4BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFC4C0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFC4C4: 4BFFDB2D  bl 0x82ff9ff0
	ctx.lr = 0x82FFC4C8;
	sub_82FF9FF0(ctx, base);
	// 82FFC4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC4CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFC4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFC4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFC4D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFC4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC4E0 size=84
    let mut pc: u32 = 0x82FFC4E0;
    'dispatch: loop {
        match pc {
            0x82FFC4E0 => {
    //   block [0x82FFC4E0..0x82FFC534)
	// 82FFC4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFC4E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFC4EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFC4F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC4F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFC4F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFC4FC: 389FFFF4  addi r4, r31, -0xc
	ctx.r[4].s64 = ctx.r[31].s64 + -12;
	// 82FFC500: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82FFC504: 4BFFDAED  bl 0x82ff9ff0
	ctx.lr = 0x82FFC508;
	sub_82FF9FF0(ctx, base);
	// 82FFC508: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC50C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFC510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC514: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFC518: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFC51C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFC520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFC524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFC528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFC52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFC530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC538 size=8
    let mut pc: u32 = 0x82FFC538;
    'dispatch: loop {
        match pc {
            0x82FFC538 => {
    //   block [0x82FFC538..0x82FFC540)
	// 82FFC538: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FFC53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC540 size=20
    let mut pc: u32 = 0x82FFC540;
    'dispatch: loop {
        match pc {
            0x82FFC540 => {
    //   block [0x82FFC540..0x82FFC554)
	// 82FFC540: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC544: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC548: 4082000C  bne 0x82ffc554
	if !ctx.cr[0].eq {
		sub_82FFC554(ctx, base);
		return;
	}
	// 82FFC54C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFC550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC554(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC554 size=44
    let mut pc: u32 = 0x82FFC554;
    'dispatch: loop {
        match pc {
            0x82FFC554 => {
    //   block [0x82FFC554..0x82FFC580)
	// 82FFC554: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FFC558: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC55C: A14AA6C0  lhz r10, -0x5940(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFC560: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82FFC564: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFC568: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFC56C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFC570: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC574: 4182000C  beq 0x82ffc580
	if ctx.cr[0].eq {
		sub_82FFC580(ctx, base);
		return;
	}
	// 82FFC578: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFC57C: 48000008  b 0x82ffc584
	sub_82FFC580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC580 size=12
    let mut pc: u32 = 0x82FFC580;
    'dispatch: loop {
        match pc {
            0x82FFC580 => {
    //   block [0x82FFC580..0x82FFC58C)
	// 82FFC580: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFC584: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC590 size=24
    let mut pc: u32 = 0x82FFC590;
    'dispatch: loop {
        match pc {
            0x82FFC590 => {
    //   block [0x82FFC590..0x82FFC5A8)
	// 82FFC590: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC594: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFC598: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFC59C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFC5A0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FFC5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC5A8 size=224
    let mut pc: u32 = 0x82FFC5A8;
    'dispatch: loop {
        match pc {
            0x82FFC5A8 => {
    //   block [0x82FFC5A8..0x82FFC688)
	// 82FFC5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC5AC: 481ABBC1  bl 0x831a816c
	ctx.lr = 0x82FFC5B0;
	sub_831A8130(ctx, base);
	// 82FFC5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC5B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFC5B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFC5BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC5C0: 419A00BC  beq cr6, 0x82ffc67c
	if ctx.cr[6].eq {
	pc = 0x82FFC67C; continue 'dispatch;
	}
	// 82FFC5C4: 3BFDFFF8  addi r31, r29, -8
	ctx.r[31].s64 = ctx.r[29].s64 + -8;
	// 82FFC5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC5CC: 4BFE31F5  bl 0x82fdf7c0
	ctx.lr = 0x82FFC5D0;
	sub_82FDF7C0(ctx, base);
	// 82FFC5D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC5D4: 4182000C  beq 0x82ffc5e0
	if ctx.cr[0].eq {
	pc = 0x82FFC5E0; continue 'dispatch;
	}
	// 82FFC5D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFC5DC: 480000A4  b 0x82ffc680
	pc = 0x82FFC680; continue 'dispatch;
	// 82FFC5E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC5E8: 4BFE31F1  bl 0x82fdf7d8
	ctx.lr = 0x82FFC5EC;
	sub_82FDF7D8(ctx, base);
	// 82FFC5EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC5F0: 4182008C  beq 0x82ffc67c
	if ctx.cr[0].eq {
	pc = 0x82FFC67C; continue 'dispatch;
	}
	// 82FFC5F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC5F8: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC5FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFC600: 4800004C  b 0x82ffc64c
	pc = 0x82FFC64C; continue 'dispatch;
	// 82FFC604: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC608: 419A0060  beq cr6, 0x82ffc668
	if ctx.cr[6].eq {
	pc = 0x82FFC668; continue 'dispatch;
	}
	// 82FFC60C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC610: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC618: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FFC61C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC620: 4E800421  bctrl
	ctx.lr = 0x82FFC624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC624: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC628: 41820054  beq 0x82ffc67c
	if ctx.cr[0].eq {
	pc = 0x82FFC67C; continue 'dispatch;
	}
	// 82FFC62C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC634: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC63C: 4E800421  bctrl
	ctx.lr = 0x82FFC640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC640: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFC648: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC64C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC654: 4E800421  bctrl
	ctx.lr = 0x82FFC658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC658: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFC65C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFC660: 409AFFA4  bne cr6, 0x82ffc604
	if !ctx.cr[6].eq {
	pc = 0x82FFC604; continue 'dispatch;
	}
	// 82FFC664: 4800000C  b 0x82ffc670
	pc = 0x82FFC670; continue 'dispatch;
	// 82FFC668: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFC66C: 409A0010  bne cr6, 0x82ffc67c
	if !ctx.cr[6].eq {
	pc = 0x82FFC67C; continue 'dispatch;
	}
	// 82FFC670: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC674: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFC678: 419A0008  beq cr6, 0x82ffc680
	if ctx.cr[6].eq {
	pc = 0x82FFC680; continue 'dispatch;
	}
	// 82FFC67C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFC680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFC684: 481ABB38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC688 size=132
    let mut pc: u32 = 0x82FFC688;
    'dispatch: loop {
        match pc {
            0x82FFC688 => {
    //   block [0x82FFC688..0x82FFC70C)
	// 82FFC688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC68C: 481ABAE1  bl 0x831a816c
	ctx.lr = 0x82FFC690;
	sub_831A8130(ctx, base);
	// 82FFC690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC694: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC698: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC69C: 41820068  beq 0x82ffc704
	if ctx.cr[0].eq {
	pc = 0x82FFC704; continue 'dispatch;
	}
	// 82FFC6A0: 3FC08214  lis r30, -0x7dec
	ctx.r[30].s64 = -2112618496;
	// 82FFC6A4: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 82FFC6A8: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFC6AC: A17DA6C0  lhz r11, -0x5940(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFC6B0: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFC6B4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82FFC6B8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFC6BC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFC6C0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFC6C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC6C8: 40820008  bne 0x82ffc6d0
	if !ctx.cr[0].eq {
	pc = 0x82FFC6D0; continue 'dispatch;
	}
	// 82FFC6CC: 3943001C  addi r10, r3, 0x1c
	ctx.r[10].s64 = ctx.r[3].s64 + 28;
	// 82FFC6D0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82FFC6D4: 83EA0004  lwz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC6D8: A15EA6C8  lhz r10, -0x5938(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 82FFC6DC: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFC6E0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FFC6E4: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FFC6E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC6EC: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FFC6F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC6F4: 4E800421  bctrl
	ctx.lr = 0x82FFC6F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC6FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFC700: 409AFFA8  bne cr6, 0x82ffc6a8
	if !ctx.cr[6].eq {
	pc = 0x82FFC6A8; continue 'dispatch;
	}
	// 82FFC704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFC708: 481ABAB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC710 size=20
    let mut pc: u32 = 0x82FFC710;
    'dispatch: loop {
        match pc {
            0x82FFC710 => {
    //   block [0x82FFC710..0x82FFC724)
	// 82FFC710: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC714: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC718: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FFC71C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC720: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC728 size=20
    let mut pc: u32 = 0x82FFC728;
    'dispatch: loop {
        match pc {
            0x82FFC728 => {
    //   block [0x82FFC728..0x82FFC73C)
	// 82FFC728: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC72C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC730: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FFC734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC738: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFC740 size=4
    let mut pc: u32 = 0x82FFC740;
    'dispatch: loop {
        match pc {
            0x82FFC740 => {
    //   block [0x82FFC740..0x82FFC744)
	// 82FFC740: 4BFFFE00  b 0x82ffc540
	sub_82FFC540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFC748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFC748 size=1792
    let mut pc: u32 = 0x82FFC748;
    'dispatch: loop {
        match pc {
            0x82FFC748 => {
    //   block [0x82FFC748..0x82FFCE48)
	// 82FFC748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFC74C: 481ABA19  bl 0x831a8164
	ctx.lr = 0x82FFC750;
	sub_831A8130(ctx, base);
	// 82FFC750: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFC754: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FFC758: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFC75C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFC760: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFC764: A15BFFFC  lhz r10, -4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FFC768: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 82FFC76C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFC770: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFC774: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFC778: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFC77C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC780: 41820040  beq 0x82ffc7c0
	if ctx.cr[0].eq {
	pc = 0x82FFC7C0; continue 'dispatch;
	}
	// 82FFC784: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC788: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC78C: 4182000C  beq 0x82ffc798
	if ctx.cr[0].eq {
	pc = 0x82FFC798; continue 'dispatch;
	}
	// 82FFC790: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFC794: 4800000C  b 0x82ffc7a0
	pc = 0x82FFC7A0; continue 'dispatch;
	// 82FFC798: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFC79C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFC7A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFC7A4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FFC7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC7AC: 4BFFD725  bl 0x82ff9ed0
	ctx.lr = 0x82FFC7B0;
	sub_82FF9ED0(ctx, base);
	// 82FFC7B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFC7B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC7B8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFC7BC: 481B446D  bl 0x831b0c28
	ctx.lr = 0x82FFC7C0;
	sub_831B0C28(ctx, base);
	// 82FFC7C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC7C8: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC7CC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FFC7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC7D4: 4E800421  bctrl
	ctx.lr = 0x82FFC7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC7D8: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FFC7DC: 419A003C  beq cr6, 0x82ffc818
	if ctx.cr[6].eq {
	pc = 0x82FFC818; continue 'dispatch;
	}
	// 82FFC7E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFC7E4: 419A000C  beq cr6, 0x82ffc7f0
	if ctx.cr[6].eq {
	pc = 0x82FFC7F0; continue 'dispatch;
	}
	// 82FFC7E8: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFC7EC: 4800000C  b 0x82ffc7f8
	pc = 0x82FFC7F8; continue 'dispatch;
	// 82FFC7F0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFC7F4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFC7F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFC7FC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FFC800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC804: 4BFFD6CD  bl 0x82ff9ed0
	ctx.lr = 0x82FFC808;
	sub_82FF9ED0(ctx, base);
	// 82FFC808: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFC80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC810: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFC814: 481B4415  bl 0x831b0c28
	ctx.lr = 0x82FFC818;
	sub_831B0C28(ctx, base);
	// 82FFC818: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC81C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC820: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFC824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC828: 4E800421  bctrl
	ctx.lr = 0x82FFC82C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC82C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC830: 41820094  beq 0x82ffc8c4
	if ctx.cr[0].eq {
	pc = 0x82FFC8C4; continue 'dispatch;
	}
	// 82FFC834: 387BFFF4  addi r3, r27, -0xc
	ctx.r[3].s64 = ctx.r[27].s64 + -12;
	// 82FFC838: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FFC83C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC840: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC848: 4E800421  bctrl
	ctx.lr = 0x82FFC84C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC84C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FFC850: 419A0030  beq cr6, 0x82ffc880
	if ctx.cr[6].eq {
	pc = 0x82FFC880; continue 'dispatch;
	}
	// 82FFC854: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82FFC858: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC85C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFC860: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFC864: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC868: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFC86C: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 82FFC870: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FFC874: 4E800421  bctrl
	ctx.lr = 0x82FFC878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC878: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC87C: 4082FFD0  bne 0x82ffc84c
	if !ctx.cr[0].eq {
	pc = 0x82FFC84C; continue 'dispatch;
	}
	// 82FFC880: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC884: 40820040  bne 0x82ffc8c4
	if !ctx.cr[0].eq {
	pc = 0x82FFC8C4; continue 'dispatch;
	}
	// 82FFC888: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC88C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC890: 4182000C  beq 0x82ffc89c
	if ctx.cr[0].eq {
	pc = 0x82FFC89C; continue 'dispatch;
	}
	// 82FFC894: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFC898: 4800000C  b 0x82ffc8a4
	pc = 0x82FFC8A4; continue 'dispatch;
	// 82FFC89C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFC8A0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFC8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFC8A8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FFC8AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC8B0: 4BFFD621  bl 0x82ff9ed0
	ctx.lr = 0x82FFC8B4;
	sub_82FF9ED0(ctx, base);
	// 82FFC8B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFC8B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC8BC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFC8C0: 481B4369  bl 0x831b0c28
	ctx.lr = 0x82FFC8C4;
	sub_831B0C28(ctx, base);
	// 82FFC8C4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFC8C8: 419A0080  beq cr6, 0x82ffc948
	if ctx.cr[6].eq {
	pc = 0x82FFC948; continue 'dispatch;
	}
	// 82FFC8CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC8D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFC8D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFC8D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC8DC: 4E800421  bctrl
	ctx.lr = 0x82FFC8E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC8E0: 397BFFF4  addi r11, r27, -0xc
	ctx.r[11].s64 = ctx.r[27].s64 + -12;
	// 82FFC8E4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFC8E8: 419A0040  beq cr6, 0x82ffc928
	if ctx.cr[6].eq {
	pc = 0x82FFC928; continue 'dispatch;
	}
	// 82FFC8EC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC8F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC8F4: 4182000C  beq 0x82ffc900
	if ctx.cr[0].eq {
	pc = 0x82FFC900; continue 'dispatch;
	}
	// 82FFC8F8: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFC8FC: 4800000C  b 0x82ffc908
	pc = 0x82FFC908; continue 'dispatch;
	// 82FFC900: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFC904: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFC908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFC90C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FFC910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC914: 4BFFD5BD  bl 0x82ff9ed0
	ctx.lr = 0x82FFC918;
	sub_82FF9ED0(ctx, base);
	// 82FFC918: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFC91C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC920: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFC924: 481B4305  bl 0x831b0c28
	ctx.lr = 0x82FFC928;
	sub_831B0C28(ctx, base);
	// 82FFC928: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC92C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFC930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC934: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FFC938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC93C: 4E800421  bctrl
	ctx.lr = 0x82FFC940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC940: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC944: 408204F8  bne 0x82ffce3c
	if !ctx.cr[0].eq {
	pc = 0x82FFCE3C; continue 'dispatch;
	}
	// 82FFC948: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC950: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFC954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC958: 4E800421  bctrl
	ctx.lr = 0x82FFC95C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC95C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFC960: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82FFC964: 409A00D8  bne cr6, 0x82ffca3c
	if !ctx.cr[6].eq {
	pc = 0x82FFCA3C; continue 'dispatch;
	}
	// 82FFC968: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC96C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFC970: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFC974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC978: 4E800421  bctrl
	ctx.lr = 0x82FFC97C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC97C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFC980: 4182009C  beq 0x82ffca1c
	if ctx.cr[0].eq {
	pc = 0x82FFCA1C; continue 'dispatch;
	}
	// 82FFC984: 3BBBFFF4  addi r29, r27, -0xc
	ctx.r[29].s64 = ctx.r[27].s64 + -12;
	// 82FFC988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFC98C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFC990: 4BFE4F41  bl 0x82fe18d0
	ctx.lr = 0x82FFC994;
	sub_82FE18D0(ctx, base);
	// 82FFC994: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFC998: 41820024  beq 0x82ffc9bc
	if ctx.cr[0].eq {
	pc = 0x82FFC9BC; continue 'dispatch;
	}
	// 82FFC99C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC9A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFC9A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFC9A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFC9AC: 4E800421  bctrl
	ctx.lr = 0x82FFC9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFC9B0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFC9B4: 4082FFD4  bne 0x82ffc988
	if !ctx.cr[0].eq {
	pc = 0x82FFC988; continue 'dispatch;
	}
	// 82FFC9B8: 48000064  b 0x82ffca1c
	pc = 0x82FFCA1C; continue 'dispatch;
	// 82FFC9BC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC9C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFC9C4: 4182000C  beq 0x82ffc9d0
	if ctx.cr[0].eq {
	pc = 0x82FFC9D0; continue 'dispatch;
	}
	// 82FFC9C8: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFC9CC: 4800000C  b 0x82ffc9d8
	pc = 0x82FFC9D8; continue 'dispatch;
	// 82FFC9D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFC9D4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFC9D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFC9DC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FFC9E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC9E4: 4BFFD4ED  bl 0x82ff9ed0
	ctx.lr = 0x82FFC9E8;
	sub_82FF9ED0(ctx, base);
	// 82FFC9E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFC9EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFC9F0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFC9F4: 481B4235  bl 0x831b0c28
	ctx.lr = 0x82FFC9F8;
	sub_831B0C28(ctx, base);
	// 82FFC9F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFC9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFCA00: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFCA04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCA08: 4E800421  bctrl
	ctx.lr = 0x82FFCA0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCA0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFCA10: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FFCA14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FFCA18: 4BFFFD31  bl 0x82ffc748
	ctx.lr = 0x82FFCA1C;
	sub_82FFC748(ctx, base);
	// 82FFCA1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCA20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFCA24: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFCA28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCA2C: 4E800421  bctrl
	ctx.lr = 0x82FFCA30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCA30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFCA34: 4082FFC4  bne 0x82ffc9f8
	if !ctx.cr[0].eq {
	pc = 0x82FFC9F8; continue 'dispatch;
	}
	// 82FFCA38: 48000394  b 0x82ffcdcc
	pc = 0x82FFCDCC; continue 'dispatch;
	// 82FFCA3C: 3BDBFFF4  addi r30, r27, -0xc
	ctx.r[30].s64 = ctx.r[27].s64 + -12;
	// 82FFCA40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFCA44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFCA48: 4BFE4E89  bl 0x82fe18d0
	ctx.lr = 0x82FFCA4C;
	sub_82FE18D0(ctx, base);
	// 82FFCA4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFCA50: 40820040  bne 0x82ffca90
	if !ctx.cr[0].eq {
	pc = 0x82FFCA90; continue 'dispatch;
	}
	// 82FFCA54: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCA58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCA5C: 4182000C  beq 0x82ffca68
	if ctx.cr[0].eq {
	pc = 0x82FFCA68; continue 'dispatch;
	}
	// 82FFCA60: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFCA64: 4800000C  b 0x82ffca70
	pc = 0x82FFCA70; continue 'dispatch;
	// 82FFCA68: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFCA6C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFCA70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFCA74: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FFCA78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFCA7C: 4BFFD455  bl 0x82ff9ed0
	ctx.lr = 0x82FFCA80;
	sub_82FF9ED0(ctx, base);
	// 82FFCA80: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFCA84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFCA88: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFCA8C: 481B419D  bl 0x831b0c28
	ctx.lr = 0x82FFCA90;
	sub_831B0C28(ctx, base);
	// 82FFCA90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCA94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFCA98: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFCA9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCAA0: 4E800421  bctrl
	ctx.lr = 0x82FFCAA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCAA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCAA8: 41820018  beq 0x82ffcac0
	if ctx.cr[0].eq {
	pc = 0x82FFCAC0; continue 'dispatch;
	}
	// 82FFCAAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCAB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFCAB4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FFCAB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCABC: 4E800421  bctrl
	ctx.lr = 0x82FFCAC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCAC0: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 82FFCAC4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFCAC8: 93C90000  stw r30, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFCACC: A16BA6A4  lhz r11, -0x595c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 82FFCAD0: A1490004  lhz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCAD4: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82FFCAD8: B1690004  sth r11, 4(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFCADC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCAE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCAE4: 4082004C  bne 0x82ffcb30
	if !ctx.cr[0].eq {
	pc = 0x82FFCB30; continue 'dispatch;
	}
	// 82FFCAE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFCAEC: 93FB0004  stw r31, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FFCAF0: A1490004  lhz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCAF4: A16BA6A8  lhz r11, -0x5958(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 82FFCAF8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82FFCAFC: B1690004  sth r11, 4(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFCB00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFCB04: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCB08: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCB0C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFCB10: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCB14: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCB18: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCB1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCB20: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCB24: 408200CC  bne 0x82ffcbf0
	if !ctx.cr[0].eq {
	pc = 0x82FFCBF0; continue 'dispatch;
	}
	// 82FFCB28: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCB2C: 480000C4  b 0x82ffcbf0
	pc = 0x82FFCBF0; continue 'dispatch;
	// 82FFCB30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFCB34: 409A00C4  bne cr6, 0x82ffcbf8
	if !ctx.cr[6].eq {
	pc = 0x82FFCBF8; continue 'dispatch;
	}
	// 82FFCB38: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FFCB3C: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCB40: A128A6C0  lhz r9, -0x5940(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCB44: 7D4A4838  and r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 82FFCB48: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFCB4C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFCB50: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFCB54: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCB58: 4182000C  beq 0x82ffcb64
	if ctx.cr[0].eq {
	pc = 0x82FFCB64; continue 'dispatch;
	}
	// 82FFCB5C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFCB60: 48000008  b 0x82ffcb68
	pc = 0x82FFCB68; continue 'dispatch;
	// 82FFCB64: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFCB68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCB6C: A16A0008  lhz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCB70: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82FFCB74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCB78: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCB7C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCB80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCB84: 396A000C  addi r11, r10, 0xc
	ctx.r[11].s64 = ctx.r[10].s64 + 12;
	// 82FFCB88: 40820008  bne 0x82ffcb90
	if !ctx.cr[0].eq {
	pc = 0x82FFCB90; continue 'dispatch;
	}
	// 82FFCB8C: 396A001C  addi r11, r10, 0x1c
	ctx.r[11].s64 = ctx.r[10].s64 + 28;
	// 82FFCB90: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FFCB94: A168A6C0  lhz r11, -0x5940(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCB98: A13F0008  lhz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCB9C: 7D2B5838  and r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82FFCBA0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCBA4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCBA8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCBAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCBB0: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCBB4: 40820008  bne 0x82ffcbbc
	if !ctx.cr[0].eq {
	pc = 0x82FFCBBC; continue 'dispatch;
	}
	// 82FFCBB8: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCBBC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFCBC0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCBC4: A148A6C0  lhz r10, -0x5940(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCBC8: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCBCC: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82FFCBD0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFCBD4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFCBD8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFCBDC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCBE0: 4182000C  beq 0x82ffcbec
	if ctx.cr[0].eq {
	pc = 0x82FFCBEC; continue 'dispatch;
	}
	// 82FFCBE4: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFCBE8: 48000008  b 0x82ffcbf0
	pc = 0x82FFCBF0; continue 'dispatch;
	// 82FFCBEC: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFCBF0: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82FFCBF4: 480001D8  b 0x82ffcdcc
	pc = 0x82FFCDCC; continue 'dispatch;
	// 82FFCBF8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFCBFC: 409A00F4  bne cr6, 0x82ffccf0
	if !ctx.cr[6].eq {
	pc = 0x82FFCCF0; continue 'dispatch;
	}
	// 82FFCC00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FFCC04: 3CC08214  lis r6, -0x7dec
	ctx.r[6].s64 = -2112618496;
	// 82FFCC08: A10B0004  lhz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCC0C: A146A6A8  lhz r10, -0x5958(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 82FFCC10: 7D0A5078  andc r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 & !ctx.r[10].u64;
	// 82FFCC14: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FFCC18: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FFCC1C: A11F0008  lhz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCC20: A16AA6C0  lhz r11, -0x5940(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCC24: 7D0B5838  and r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[11].u64;
	// 82FFCC28: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCC2C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCC30: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCC34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCC38: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCC3C: 40820008  bne 0x82ffcc44
	if !ctx.cr[0].eq {
	pc = 0x82FFCC44; continue 'dispatch;
	}
	// 82FFCC40: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCC44: 811B0004  lwz r8, 4(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCC48: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FFCC4C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCC50: A10AA6C0  lhz r8, -0x5940(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCC54: A0EB0008  lhz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCC58: 7CE74038  and r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 82FFCC5C: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82FFCC60: 54E7DFFE  rlwinm r7, r7, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82FFCC64: 68E70001  xori r7, r7, 1
	ctx.r[7].u64 = ctx.r[7].u64 ^ 1;
	// 82FFCC68: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCC6C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82FFCC70: 40820008  bne 0x82ffcc78
	if !ctx.cr[0].eq {
	pc = 0x82FFCC78; continue 'dispatch;
	}
	// 82FFCC74: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 82FFCC78: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCC7C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82FFCC80: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCC84: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCC88: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCC8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCC90: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCC94: 40820008  bne 0x82ffcc9c
	if !ctx.cr[0].eq {
	pc = 0x82FFCC9C; continue 'dispatch;
	}
	// 82FFCC98: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCC9C: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCCA0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FFCCA4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCCA8: A14AA6C0  lhz r10, -0x5940(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCCAC: A10B0008  lhz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCCB0: 7D0A5038  and r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82FFCCB4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFCCB8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFCCBC: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFCCC0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCCC4: 4182000C  beq 0x82ffccd0
	if ctx.cr[0].eq {
	pc = 0x82FFCCD0; continue 'dispatch;
	}
	// 82FFCCC8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFCCCC: 48000008  b 0x82ffccd4
	pc = 0x82FFCCD4; continue 'dispatch;
	// 82FFCCD0: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFCCD4: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82FFCCD8: 93FB0004  stw r31, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FFCCDC: A166A6A8  lhz r11, -0x5958(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 82FFCCE0: A1490004  lhz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCCE4: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82FFCCE8: B1690004  sth r11, 4(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFCCEC: 480000E0  b 0x82ffcdcc
	pc = 0x82FFCDCC; continue 'dispatch;
	// 82FFCCF0: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FFCCF4: A17C0008  lhz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCCF8: A149A6C0  lhz r10, -0x5940(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCCFC: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82FFCD00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCD04: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCD08: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCD0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCD10: 397C000C  addi r11, r28, 0xc
	ctx.r[11].s64 = ctx.r[28].s64 + 12;
	// 82FFCD14: 40820008  bne 0x82ffcd1c
	if !ctx.cr[0].eq {
	pc = 0x82FFCD1C; continue 'dispatch;
	}
	// 82FFCD18: 397C001C  addi r11, r28, 0x1c
	ctx.r[11].s64 = ctx.r[28].s64 + 28;
	// 82FFCD1C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCD20: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCD24: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82FFCD28: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCD2C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCD30: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCD34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCD38: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCD3C: 40820008  bne 0x82ffcd44
	if !ctx.cr[0].eq {
	pc = 0x82FFCD44; continue 'dispatch;
	}
	// 82FFCD40: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCD44: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FFCD48: A169A6C0  lhz r11, -0x5940(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCD4C: A1480008  lhz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCD50: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFCD54: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCD58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCD5C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCD60: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCD64: 3968000C  addi r11, r8, 0xc
	ctx.r[11].s64 = ctx.r[8].s64 + 12;
	// 82FFCD68: 40820008  bne 0x82ffcd70
	if !ctx.cr[0].eq {
	pc = 0x82FFCD70; continue 'dispatch;
	}
	// 82FFCD6C: 3968001C  addi r11, r8, 0x1c
	ctx.r[11].s64 = ctx.r[8].s64 + 28;
	// 82FFCD70: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FFCD74: A169A6C0  lhz r11, -0x5940(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCD78: A15C0008  lhz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCD7C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFCD80: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCD84: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCD88: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCD8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCD90: 397C000C  addi r11, r28, 0xc
	ctx.r[11].s64 = ctx.r[28].s64 + 12;
	// 82FFCD94: 40820008  bne 0x82ffcd9c
	if !ctx.cr[0].eq {
	pc = 0x82FFCD9C; continue 'dispatch;
	}
	// 82FFCD98: 397C001C  addi r11, r28, 0x1c
	ctx.r[11].s64 = ctx.r[28].s64 + 28;
	// 82FFCD9C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82FFCDA0: A169A6C0  lhz r11, -0x5940(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFCDA4: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCDA8: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFCDAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCDB0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCDB4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCDB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCDBC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFCDC0: 40820008  bne 0x82ffcdc8
	if !ctx.cr[0].eq {
	pc = 0x82FFCDC8; continue 'dispatch;
	}
	// 82FFCDC4: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFCDC8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FFCDCC: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCDD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCDD4: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FFCDD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCDDC: 4E800421  bctrl
	ctx.lr = 0x82FFCDE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCDE0: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCDE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFCDE8: 41820054  beq 0x82ffce3c
	if ctx.cr[0].eq {
	pc = 0x82FFCE3C; continue 'dispatch;
	}
	// 82FFCDEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCDF0: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFCDF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCDF8: 4E800421  bctrl
	ctx.lr = 0x82FFCDFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCDFC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FFCE00: 4182003C  beq 0x82ffce3c
	if ctx.cr[0].eq {
	pc = 0x82FFCE3C; continue 'dispatch;
	}
	// 82FFCE04: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCE08: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCE0C: 41820030  beq 0x82ffce3c
	if ctx.cr[0].eq {
	pc = 0x82FFCE3C; continue 'dispatch;
	}
	// 82FFCE10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFCE14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FFCE18: 419A0024  beq cr6, 0x82ffce3c
	if ctx.cr[6].eq {
	pc = 0x82FFCE3C; continue 'dispatch;
	}
	// 82FFCE1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFCE20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFCE24: 4802FA4D  bl 0x8302c870
	ctx.lr = 0x82FFCE28;
	sub_8302C870(ctx, base);
	// 82FFCE28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFCE2C: 48003F5D  bl 0x83000d88
	ctx.lr = 0x82FFCE30;
	sub_83000D88(ctx, base);
	// 82FFCE30: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FFCE34: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FFCE38: 4198FFE4  blt cr6, 0x82ffce1c
	if ctx.cr[6].lt {
	pc = 0x82FFCE1C; continue 'dispatch;
	}
	// 82FFCE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFCE40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFCE44: 481AB370  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFCE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFCE48 size=988
    let mut pc: u32 = 0x82FFCE48;
    'dispatch: loop {
        match pc {
            0x82FFCE48 => {
    //   block [0x82FFCE48..0x82FFD224)
	// 82FFCE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFCE4C: 481AB315  bl 0x831a8160
	ctx.lr = 0x82FFCE50;
	sub_831A8130(ctx, base);
	// 82FFCE50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFCE54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FFCE58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFCE5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFCE60: A15BFFFC  lhz r10, -4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FFCE64: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 82FFCE68: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFCE6C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCE70: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFCE74: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFCE78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCE7C: 41820040  beq 0x82ffcebc
	if ctx.cr[0].eq {
	pc = 0x82FFCEBC; continue 'dispatch;
	}
	// 82FFCE80: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCE84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCE88: 4182000C  beq 0x82ffce94
	if ctx.cr[0].eq {
	pc = 0x82FFCE94; continue 'dispatch;
	}
	// 82FFCE8C: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFCE90: 4800000C  b 0x82ffce9c
	pc = 0x82FFCE9C; continue 'dispatch;
	// 82FFCE94: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFCE98: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFCE9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFCEA0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FFCEA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFCEA8: 4BFFD029  bl 0x82ff9ed0
	ctx.lr = 0x82FFCEAC;
	sub_82FF9ED0(ctx, base);
	// 82FFCEAC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFCEB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFCEB4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFCEB8: 481B3D71  bl 0x831b0c28
	ctx.lr = 0x82FFCEBC;
	sub_831B0C28(ctx, base);
	// 82FFCEBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFCEC0: 419A0328  beq cr6, 0x82ffd1e8
	if ctx.cr[6].eq {
	pc = 0x82FFD1E8; continue 'dispatch;
	}
	// 82FFCEC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFCECC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFCED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCED4: 4E800421  bctrl
	ctx.lr = 0x82FFCED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCED8: 397BFFF4  addi r11, r27, -0xc
	ctx.r[11].s64 = ctx.r[27].s64 + -12;
	// 82FFCEDC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFCEE0: 409A0308  bne cr6, 0x82ffd1e8
	if !ctx.cr[6].eq {
	pc = 0x82FFD1E8; continue 'dispatch;
	}
	// 82FFCEE4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCEE8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FFCEEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FFCEF0: 418200D0  beq 0x82ffcfc0
	if ctx.cr[0].eq {
	pc = 0x82FFCFC0; continue 'dispatch;
	}
	// 82FFCEF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCEF8: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FFCEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCF00: 4E800421  bctrl
	ctx.lr = 0x82FFCF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCF04: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FFCF08: 41820050  beq 0x82ffcf58
	if ctx.cr[0].eq {
	pc = 0x82FFCF58; continue 'dispatch;
	}
	// 82FFCF0C: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCF10: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF14: 41820044  beq 0x82ffcf58
	if ctx.cr[0].eq {
	pc = 0x82FFCF58; continue 'dispatch;
	}
	// 82FFCF18: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82FFCF1C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF20: 419A0038  beq cr6, 0x82ffcf58
	if ctx.cr[6].eq {
	pc = 0x82FFCF58; continue 'dispatch;
	}
	// 82FFCF24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFCF28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFCF2C: 4802F945  bl 0x8302c870
	ctx.lr = 0x82FFCF30;
	sub_8302C870(ctx, base);
	// 82FFCF30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF34: 41820018  beq 0x82ffcf4c
	if ctx.cr[0].eq {
	pc = 0x82FFCF4C; continue 'dispatch;
	}
	// 82FFCF38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFCF3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFCF40: 4802F931  bl 0x8302c870
	ctx.lr = 0x82FFCF44;
	sub_8302C870(ctx, base);
	// 82FFCF44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFCF48: 48002AD1  bl 0x82fffa18
	ctx.lr = 0x82FFCF4C;
	sub_82FFFA18(ctx, base);
	// 82FFCF4C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FFCF50: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFCF54: 4198FFD0  blt cr6, 0x82ffcf24
	if ctx.cr[6].lt {
	pc = 0x82FFCF24; continue 'dispatch;
	}
	// 82FFCF58: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCF5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCF60: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFCF64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFCF68: 4E800421  bctrl
	ctx.lr = 0x82FFCF6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFCF6C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FFCF70: 41820050  beq 0x82ffcfc0
	if ctx.cr[0].eq {
	pc = 0x82FFCFC0; continue 'dispatch;
	}
	// 82FFCF74: 839D0008  lwz r28, 8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCF78: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF7C: 41820044  beq 0x82ffcfc0
	if ctx.cr[0].eq {
	pc = 0x82FFCFC0; continue 'dispatch;
	}
	// 82FFCF80: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82FFCF84: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF88: 419A0038  beq cr6, 0x82ffcfc0
	if ctx.cr[6].eq {
	pc = 0x82FFCFC0; continue 'dispatch;
	}
	// 82FFCF8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFCF90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFCF94: 4802F8DD  bl 0x8302c870
	ctx.lr = 0x82FFCF98;
	sub_8302C870(ctx, base);
	// 82FFCF98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFCF9C: 41820018  beq 0x82ffcfb4
	if ctx.cr[0].eq {
	pc = 0x82FFCFB4; continue 'dispatch;
	}
	// 82FFCFA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFCFA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFCFA8: 4802F8C9  bl 0x8302c870
	ctx.lr = 0x82FFCFAC;
	sub_8302C870(ctx, base);
	// 82FFCFAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFCFB0: 48003C49  bl 0x83000bf8
	ctx.lr = 0x82FFCFB4;
	sub_83000BF8(ctx, base);
	// 82FFCFB4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FFCFB8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FFCFBC: 4198FFD0  blt cr6, 0x82ffcf8c
	if ctx.cr[6].lt {
	pc = 0x82FFCF8C; continue 'dispatch;
	}
	// 82FFCFC0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCFC4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFCFC8: 409A00CC  bne cr6, 0x82ffd094
	if !ctx.cr[6].eq {
	pc = 0x82FFD094; continue 'dispatch;
	}
	// 82FFCFCC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82FFCFD0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FFCFD4: A10B0004  lhz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFCFD8: A12AA6A8  lhz r9, -0x5958(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 82FFCFDC: 7D094878  andc r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & !ctx.r[9].u64;
	// 82FFCFE0: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82FFCFE4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFCFE8: A13F0008  lhz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFCFEC: 38EBA6C0  addi r7, r11, -0x5940
	ctx.r[7].s64 = ctx.r[11].s64 + -22848;
	// 82FFCFF0: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFCFF4: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82FFCFF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFCFFC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD000: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD004: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD008: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFD00C: 40820008  bne 0x82ffd014
	if !ctx.cr[0].eq {
	pc = 0x82FFD014; continue 'dispatch;
	}
	// 82FFD010: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFD014: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD018: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD01C: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFD020: 41820120  beq 0x82ffd140
	if ctx.cr[0].eq {
	pc = 0x82FFD140; continue 'dispatch;
	}
	// 82FFD024: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FFD028: A14AA6A8  lhz r10, -0x5958(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22872 as u32) ) } as u64;
	// 82FFD02C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD030: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82FFD034: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 82FFD038: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FFD03C: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD040: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD044: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFD048: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFD04C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFD050: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFD054: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD058: 40820008  bne 0x82ffd060
	if !ctx.cr[0].eq {
	pc = 0x82FFD060; continue 'dispatch;
	}
	// 82FFD05C: 393F001C  addi r9, r31, 0x1c
	ctx.r[9].s64 = ctx.r[31].s64 + 28;
	// 82FFD060: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD064: A10A0008  lhz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD068: 7D0B5838  and r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[11].u64;
	// 82FFD06C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFD070: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD074: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD078: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD07C: 396A000C  addi r11, r10, 0xc
	ctx.r[11].s64 = ctx.r[10].s64 + 12;
	// 82FFD080: 40820008  bne 0x82ffd088
	if !ctx.cr[0].eq {
	pc = 0x82FFD088; continue 'dispatch;
	}
	// 82FFD084: 396A001C  addi r11, r10, 0x1c
	ctx.r[11].s64 = ctx.r[10].s64 + 28;
	// 82FFD088: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD08C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFD090: 480000B0  b 0x82ffd140
	pc = 0x82FFD140; continue 'dispatch;
	// 82FFD094: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFD098: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82FFD09C: 38EBA6C0  addi r7, r11, -0x5940
	ctx.r[7].s64 = ctx.r[11].s64 + -22848;
	// 82FFD0A0: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD0A4: A1270000  lhz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD0A8: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82FFD0AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFD0B0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD0B4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD0B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD0BC: 40820008  bne 0x82ffd0c4
	if !ctx.cr[0].eq {
	pc = 0x82FFD0C4; continue 'dispatch;
	}
	// 82FFD0C0: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 82FFD0C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFD0C8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD0CC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFD0D0: 409A0008  bne cr6, 0x82ffd0d8
	if !ctx.cr[6].eq {
	pc = 0x82FFD0D8; continue 'dispatch;
	}
	// 82FFD0D4: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFD0D8: A1480008  lhz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD0DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD0E0: 7D4A4838  and r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 82FFD0E4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFD0E8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFD0EC: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFD0F0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD0F4: 3948000C  addi r10, r8, 0xc
	ctx.r[10].s64 = ctx.r[8].s64 + 12;
	// 82FFD0F8: 40820008  bne 0x82ffd100
	if !ctx.cr[0].eq {
	pc = 0x82FFD100; continue 'dispatch;
	}
	// 82FFD0FC: 3948001C  addi r10, r8, 0x1c
	ctx.r[10].s64 = ctx.r[8].s64 + 28;
	// 82FFD100: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFD104: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFD108: A1470000  lhz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD10C: 409A0008  bne cr6, 0x82ffd114
	if !ctx.cr[6].eq {
	pc = 0x82FFD114; continue 'dispatch;
	}
	// 82FFD110: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD114: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD118: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82FFD11C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FFD120: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FFD124: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82FFD128: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD12C: 4182000C  beq 0x82ffd138
	if ctx.cr[0].eq {
	pc = 0x82FFD138; continue 'dispatch;
	}
	// 82FFD130: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FFD134: 48000008  b 0x82ffd13c
	pc = 0x82FFD13C; continue 'dispatch;
	// 82FFD138: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82FFD13C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FFD140: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD144: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD148: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82FFD14C: 40820008  bne 0x82ffd154
	if !ctx.cr[0].eq {
	pc = 0x82FFD154; continue 'dispatch;
	}
	// 82FFD150: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FFD154: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82FFD158: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFD15C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FFD160: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD164: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 82FFD168: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82FFD16C: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FFD170: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD174: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD178: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82FFD17C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFD180: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD184: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD188: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD18C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFD190: 40820008  bne 0x82ffd198
	if !ctx.cr[0].eq {
	pc = 0x82FFD198; continue 'dispatch;
	}
	// 82FFD194: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFD198: 934B0004  stw r26, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82FFD19C: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD1A0: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD1A4: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82FFD1A8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFD1AC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD1B0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD1B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD1B8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFD1BC: 40820008  bne 0x82ffd1c4
	if !ctx.cr[0].eq {
	pc = 0x82FFD1C4; continue 'dispatch;
	}
	// 82FFD1C0: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFD1C4: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82FFD1C8: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD1CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD1D0: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FFD1D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD1D8: 4E800421  bctrl
	ctx.lr = 0x82FFD1DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD1DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD1E0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FFD1E4: 481AAFCC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FFD1E8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD1EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD1F0: 4182000C  beq 0x82ffd1fc
	if ctx.cr[0].eq {
	pc = 0x82FFD1FC; continue 'dispatch;
	}
	// 82FFD1F4: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFD1F8: 4800000C  b 0x82ffd204
	pc = 0x82FFD204; continue 'dispatch;
	// 82FFD1FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFD200: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFD204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFD208: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FFD20C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFD210: 4BFFCCC1  bl 0x82ff9ed0
	ctx.lr = 0x82FFD214;
	sub_82FF9ED0(ctx, base);
	// 82FFD214: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFD218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFD21C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFD220: 481B3A09  bl 0x831b0c28
	ctx.lr = 0x82FFD224;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD228 size=68
    let mut pc: u32 = 0x82FFD228;
    'dispatch: loop {
        match pc {
            0x82FFD228 => {
    //   block [0x82FFD228..0x82FFD26C)
	// 82FFD228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFD234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFD238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD23C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD240: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFD244: 4BFFF505  bl 0x82ffc748
	ctx.lr = 0x82FFD248;
	sub_82FFC748(ctx, base);
	// 82FFD248: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFD24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD250: 4BFFFBF9  bl 0x82ffce48
	ctx.lr = 0x82FFD254;
	sub_82FFCE48(ctx, base);
	// 82FFD254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFD258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD260: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFD264: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFD268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD270 size=284
    let mut pc: u32 = 0x82FFD270;
    'dispatch: loop {
        match pc {
            0x82FFD270 => {
    //   block [0x82FFD270..0x82FFD38C)
	// 82FFD270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD274: 481AAEF1  bl 0x831a8164
	ctx.lr = 0x82FFD278;
	sub_831A8130(ctx, base);
	// 82FFD278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD27C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFD280: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD284: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD288: 418200FC  beq 0x82ffd384
	if ctx.cr[0].eq {
	pc = 0x82FFD384; continue 'dispatch;
	}
	// 82FFD28C: 3F608214  lis r27, -0x7dec
	ctx.r[27].s64 = -2112618496;
	// 82FFD290: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD294: A17BA6C0  lhz r11, -0x5940(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 82FFD298: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82FFD29C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFD2A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFD2A4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFD2A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD2AC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82FFD2B0: 40820008  bne 0x82ffd2b8
	if !ctx.cr[0].eq {
	pc = 0x82FFD2B8; continue 'dispatch;
	}
	// 82FFD2B4: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82FFD2B8: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD2BC: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD2C0: 41820084  beq 0x82ffd344
	if ctx.cr[0].eq {
	pc = 0x82FFD344; continue 'dispatch;
	}
	// 82FFD2C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD2CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD2D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD2D4: 4E800421  bctrl
	ctx.lr = 0x82FFD2D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD2D8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFD2DC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFD2E0: 409A0064  bne cr6, 0x82ffd344
	if !ctx.cr[6].eq {
	pc = 0x82FFD344; continue 'dispatch;
	}
	// 82FFD2E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD2E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFD2EC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD2F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD2F4: 4E800421  bctrl
	ctx.lr = 0x82FFD2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD2F8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFD2FC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFD300: 409A0044  bne cr6, 0x82ffd344
	if !ctx.cr[6].eq {
	pc = 0x82FFD344; continue 'dispatch;
	}
	// 82FFD304: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFD30C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD310: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FFD314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD318: 4E800421  bctrl
	ctx.lr = 0x82FFD31C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD31C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFD320: 817D00A8  lwz r11, 0xa8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FFD324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD32C: 4E800421  bctrl
	ctx.lr = 0x82FFD330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD330: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFD334: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFD338: 4BFFFB11  bl 0x82ffce48
	ctx.lr = 0x82FFD33C;
	sub_82FFCE48(ctx, base);
	// 82FFD33C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82FFD340: 48000038  b 0x82ffd378
	pc = 0x82FFD378; continue 'dispatch;
	// 82FFD344: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD34C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD354: 4E800421  bctrl
	ctx.lr = 0x82FFD358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD358: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFD35C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFD360: 409A0018  bne cr6, 0x82ffd378
	if !ctx.cr[6].eq {
	pc = 0x82FFD378; continue 'dispatch;
	}
	// 82FFD364: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD36C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FFD370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD374: 4E800421  bctrl
	ctx.lr = 0x82FFD378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD378: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FFD37C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFD380: 409AFF10  bne cr6, 0x82ffd290
	if !ctx.cr[6].eq {
	pc = 0x82FFD290; continue 'dispatch;
	}
	// 82FFD384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFD388: 481AAE2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFD390 size=8
    let mut pc: u32 = 0x82FFD390;
    'dispatch: loop {
        match pc {
            0x82FFD390 => {
    //   block [0x82FFD390..0x82FFD398)
	// 82FFD390: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFD394: 4BFFF3B4  b 0x82ffc748
	sub_82FFC748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD398 size=132
    let mut pc: u32 = 0x82FFD398;
    'dispatch: loop {
        match pc {
            0x82FFD398 => {
    //   block [0x82FFD398..0x82FFD41C)
	// 82FFD398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD3A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFD3A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFD3A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD3AC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD3B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFD3B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFD3B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFD3BC: 48000038  b 0x82ffd3f4
	pc = 0x82FFD3F4; continue 'dispatch;
	// 82FFD3C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD3C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFD3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD3CC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FFD3D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD3D4: 4E800421  bctrl
	ctx.lr = 0x82FFD3D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD3D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFD3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFD3E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFD3E4: 4BFFF365  bl 0x82ffc748
	ctx.lr = 0x82FFD3E8;
	sub_82FFC748(ctx, base);
	// 82FFD3E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD3F0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFD3F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD3F8: 4E800421  bctrl
	ctx.lr = 0x82FFD3FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD3FC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFD400: 4082FFC0  bne 0x82ffd3c0
	if !ctx.cr[0].eq {
	pc = 0x82FFD3C0; continue 'dispatch;
	}
	// 82FFD404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFD408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD410: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFD414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFD418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD420 size=124
    let mut pc: u32 = 0x82FFD420;
    'dispatch: loop {
        match pc {
            0x82FFD420 => {
    //   block [0x82FFD420..0x82FFD49C)
	// 82FFD420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFD42C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD434: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FFD438: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FFD43C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFD440: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FFD444: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFD448: 4BFE45A9  bl 0x82fe19f0
	ctx.lr = 0x82FFD44C;
	sub_82FE19F0(ctx, base);
	// 82FFD44C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFD454: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FFD458: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFD45C: 40990028  ble cr6, 0x82ffd484
	if !ctx.cr[6].gt {
	pc = 0x82FFD484; continue 'dispatch;
	}
	// 82FFD460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFD464: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD468: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FFD46C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFD470: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FFD474: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FFD478: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD47C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82FFD480: 4198FFE4  blt cr6, 0x82ffd464
	if ctx.cr[6].lt {
	pc = 0x82FFD464; continue 'dispatch;
	}
	// 82FFD484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFD48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFD498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD4A0 size=228
    let mut pc: u32 = 0x82FFD4A0;
    'dispatch: loop {
        match pc {
            0x82FFD4A0 => {
    //   block [0x82FFD4A0..0x82FFD584)
	// 82FFD4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD4A4: 481AACC5  bl 0x831a8168
	ctx.lr = 0x82FFD4A8;
	sub_831A8130(ctx, base);
	// 82FFD4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD4AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFD4B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFD4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD4B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD4BC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD4C0: 80AB0090  lwz r5, 0x90(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFD4C4: 4BFD473D  bl 0x82fd1c00
	ctx.lr = 0x82FFD4C8;
	sub_82FD1C00(ctx, base);
	// 82FFD4C8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD4CC: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFD4D0: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFD4D4: 48000020  b 0x82ffd4f4
	pc = 0x82FFD4F4; continue 'dispatch;
	// 82FFD4D8: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD4DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFD4E0: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82FFD4E4: 4BFD675D  bl 0x82fd3c40
	ctx.lr = 0x82FFD4E8;
	sub_82FD3C40(ctx, base);
	// 82FFD4E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFD4EC: 40820034  bne 0x82ffd520
	if !ctx.cr[0].eq {
	pc = 0x82FFD520; continue 'dispatch;
	}
	// 82FFD4F0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FFD4F4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD4F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFD4FC: 409AFFDC  bne cr6, 0x82ffd4d8
	if !ctx.cr[6].eq {
	pc = 0x82FFD4D8; continue 'dispatch;
	}
	// 82FFD500: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD504: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFD508: 419A0040  beq cr6, 0x82ffd548
	if ctx.cr[6].eq {
	pc = 0x82FFD548; continue 'dispatch;
	}
	// 82FFD50C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD510: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD514: 41820034  beq 0x82ffd548
	if ctx.cr[0].eq {
	pc = 0x82FFD548; continue 'dispatch;
	}
	// 82FFD518: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FFD51C: 48000014  b 0x82ffd530
	pc = 0x82FFD530; continue 'dispatch;
	// 82FFD520: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD524: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FFD528: 48000054  b 0x82ffd57c
	pc = 0x82FFD57C; continue 'dispatch;
	// 82FFD52C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FFD530: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD534: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD538: 4082FFF4  bne 0x82ffd52c
	if !ctx.cr[0].eq {
	pc = 0x82FFD52C; continue 'dispatch;
	}
	// 82FFD53C: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FFD540: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FFD544: 48000008  b 0x82ffd54c
	pc = 0x82FFD54C; continue 'dispatch;
	// 82FFD548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFD54C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FFD550: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFD554: 4BFE449D  bl 0x82fe19f0
	ctx.lr = 0x82FFD558;
	sub_82FE19F0(ctx, base);
	// 82FFD558: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFD55C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFD560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFD564: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 82FFD568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD56C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFD570: 4BFD45F9  bl 0x82fd1b68
	ctx.lr = 0x82FFD574;
	sub_82FD1B68(ctx, base);
	// 82FFD574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD578: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFD57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFD580: 481AAC38  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD588 size=168
    let mut pc: u32 = 0x82FFD588;
    'dispatch: loop {
        match pc {
            0x82FFD588 => {
    //   block [0x82FFD588..0x82FFD630)
	// 82FFD588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD58C: 481AABD9  bl 0x831a8164
	ctx.lr = 0x82FFD590;
	sub_831A8130(ctx, base);
	// 82FFD590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD598: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFD59C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFD5A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFD5A4: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FFD5A8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FFD5AC: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82FFD5B0: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FFD5B4: 419A0034  beq cr6, 0x82ffd5e8
	if ctx.cr[6].eq {
	pc = 0x82FFD5E8; continue 'dispatch;
	}
	// 82FFD5B8: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD5BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD5C0: 41820028  beq 0x82ffd5e8
	if ctx.cr[0].eq {
	pc = 0x82FFD5E8; continue 'dispatch;
	}
	// 82FFD5C4: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FFD5C8: 48000008  b 0x82ffd5d0
	pc = 0x82FFD5D0; continue 'dispatch;
	// 82FFD5CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FFD5D0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD5D4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD5D8: 4082FFF4  bne 0x82ffd5cc
	if !ctx.cr[0].eq {
	pc = 0x82FFD5CC; continue 'dispatch;
	}
	// 82FFD5DC: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82FFD5E0: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FFD5E4: 48000008  b 0x82ffd5ec
	pc = 0x82FFD5EC; continue 'dispatch;
	// 82FFD5E8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82FFD5EC: 397E000F  addi r11, r30, 0xf
	ctx.r[11].s64 = ctx.r[30].s64 + 15;
	// 82FFD5F0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFD5F4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FFD5F8: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFD5FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFD600: 4BFE43F1  bl 0x82fe19f0
	ctx.lr = 0x82FFD604;
	sub_82FE19F0(ctx, base);
	// 82FFD604: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82FFD608: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFD60C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FFD610: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFD614: 481AAEFD  bl 0x831a8510
	ctx.lr = 0x82FFD618;
	sub_831A8510(ctx, base);
	// 82FFD618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD61C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FFD620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD624: 7F7D5B2E  sthx r27, r29, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u16) };
	// 82FFD628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFD62C: 481AAB88  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FFD630 size=120
    let mut pc: u32 = 0x82FFD630;
    'dispatch: loop {
        match pc {
            0x82FFD630 => {
    //   block [0x82FFD630..0x82FFD6A8)
	// 82FFD630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD634: 481AAB39  bl 0x831a816c
	ctx.lr = 0x82FFD638;
	sub_831A8130(ctx, base);
	// 82FFD638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD63C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD640: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FFD644: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD648: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD64C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FFD650: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82FFD654: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82FFD658: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FFD65C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82FFD660: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FFD664: C80B7390  lfd f0, 0x7390(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(29584 as u32) ) };
	// 82FFD668: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82FFD66C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FFD670: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82FFD674: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFD678: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82FFD67C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFD680: 4BFE4371  bl 0x82fe19f0
	ctx.lr = 0x82FFD684;
	sub_82FE19F0(ctx, base);
	// 82FFD684: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD688: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD68C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFD690: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFD694: 481AAE7D  bl 0x831a8510
	ctx.lr = 0x82FFD698;
	sub_831A8510(ctx, base);
	// 82FFD698: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FFD69C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFD6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFD6A4: 481AAB18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD6A8 size=188
    let mut pc: u32 = 0x82FFD6A8;
    'dispatch: loop {
        match pc {
            0x82FFD6A8 => {
    //   block [0x82FFD6A8..0x82FFD764)
	// 82FFD6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD6AC: 481AAAC1  bl 0x831a816c
	ctx.lr = 0x82FFD6B0;
	sub_831A8130(ctx, base);
	// 82FFD6B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD6B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FFD6B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD6BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFD6C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFD6C4: 409A0040  bne cr6, 0x82ffd704
	if !ctx.cr[6].eq {
	pc = 0x82FFD704; continue 'dispatch;
	}
	// 82FFD6C8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FFD6CC: 419A0034  beq cr6, 0x82ffd700
	if ctx.cr[6].eq {
	pc = 0x82FFD700; continue 'dispatch;
	}
	// 82FFD6D0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD6D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD6D8: 41820028  beq 0x82ffd700
	if ctx.cr[0].eq {
	pc = 0x82FFD700; continue 'dispatch;
	}
	// 82FFD6DC: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FFD6E0: 48000008  b 0x82ffd6e8
	pc = 0x82FFD6E8; continue 'dispatch;
	// 82FFD6E4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FFD6E8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD6EC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD6F0: 4082FFF4  bne 0x82ffd6e4
	if !ctx.cr[0].eq {
	pc = 0x82FFD6E4; continue 'dispatch;
	}
	// 82FFD6F4: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FFD6F8: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FFD6FC: 48000008  b 0x82ffd704
	pc = 0x82FFD704; continue 'dispatch;
	// 82FFD700: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFD704: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD708: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD70C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FFD710: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFD714: 41980010  blt cr6, 0x82ffd724
	if ctx.cr[6].lt {
	pc = 0x82FFD724; continue 'dispatch;
	}
	// 82FFD718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFD71C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD720: 4BFFFF11  bl 0x82ffd630
	ctx.lr = 0x82FFD724;
	sub_82FFD630(ctx, base);
	// 82FFD724: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD728: 57C5083C  slwi r5, r30, 1
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFD72C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD730: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFD734: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFD738: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FFD73C: 481AADD5  bl 0x831a8510
	ctx.lr = 0x82FFD740;
	sub_831A8510(ctx, base);
	// 82FFD740: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD744: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD748: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FFD74C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFD750: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFD754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFD758: 7D69532E  sthx r11, r9, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u16) };
	// 82FFD75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFD760: 481AAA5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD768 size=164
    let mut pc: u32 = 0x82FFD768;
    'dispatch: loop {
        match pc {
            0x82FFD768 => {
    //   block [0x82FFD768..0x82FFD80C)
	// 82FFD768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD76C: 481AA9F9  bl 0x831a8164
	ctx.lr = 0x82FFD770;
	sub_831A8130(ctx, base);
	// 82FFD770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD774: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FFD778: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFD77C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFD780: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FFD784: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFD788: 409A0040  bne cr6, 0x82ffd7c8
	if !ctx.cr[6].eq {
	pc = 0x82FFD7C8; continue 'dispatch;
	}
	// 82FFD78C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFD790: 419A0034  beq cr6, 0x82ffd7c4
	if ctx.cr[6].eq {
	pc = 0x82FFD7C4; continue 'dispatch;
	}
	// 82FFD794: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD798: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD79C: 41820028  beq 0x82ffd7c4
	if ctx.cr[0].eq {
	pc = 0x82FFD7C4; continue 'dispatch;
	}
	// 82FFD7A0: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FFD7A4: 48000008  b 0x82ffd7ac
	pc = 0x82FFD7AC; continue 'dispatch;
	// 82FFD7A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FFD7AC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD7B0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD7B4: 4082FFF4  bne 0x82ffd7a8
	if !ctx.cr[0].eq {
	pc = 0x82FFD7A8; continue 'dispatch;
	}
	// 82FFD7B8: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82FFD7BC: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FFD7C0: 48000008  b 0x82ffd7c8
	pc = 0x82FFD7C8; continue 'dispatch;
	// 82FFD7C4: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82FFD7C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD7CC: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82FFD7D0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFD7D4: 41980010  blt cr6, 0x82ffd7e4
	if ctx.cr[6].lt {
	pc = 0x82FFD7E4; continue 'dispatch;
	}
	// 82FFD7D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFD7DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFD7E0: 4BFFFE51  bl 0x82ffd630
	ctx.lr = 0x82FFD7E4;
	sub_82FFD630(ctx, base);
	// 82FFD7E4: 57FD083C  slwi r29, r31, 1
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82FFD7E8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD7EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFD7F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFD7F4: 481AAD1D  bl 0x831a8510
	ctx.lr = 0x82FFD7F8;
	sub_831A8510(ctx, base);
	// 82FFD7F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD7FC: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FFD800: 7F7D5B2E  sthx r27, r29, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u16) };
	// 82FFD804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFD808: 481AA9AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD810 size=80
    let mut pc: u32 = 0x82FFD810;
    'dispatch: loop {
        match pc {
            0x82FFD810 => {
    //   block [0x82FFD810..0x82FFD860)
	// 82FFD810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFD81C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFD828: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFD82C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FFD830: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FFD834: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82FFD838: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFD83C: 4BFD3345  bl 0x82fd0b80
	ctx.lr = 0x82FFD840;
	sub_82FD0B80(ctx, base);
	// 82FFD840: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFD844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD848: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFD84C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFD850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFD85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFD860 size=24
    let mut pc: u32 = 0x82FFD860;
    'dispatch: loop {
        match pc {
            0x82FFD860 => {
    //   block [0x82FFD860..0x82FFD878)
	// 82FFD860: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFD864: 419A0014  beq cr6, 0x82ffd878
	if ctx.cr[6].eq {
		sub_82FFD878(ctx, base);
		return;
	}
	// 82FFD868: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFD86C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFD870: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFD874: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFD878 size=8
    let mut pc: u32 = 0x82FFD878;
    'dispatch: loop {
        match pc {
            0x82FFD878 => {
    //   block [0x82FFD878..0x82FFD880)
	// 82FFD878: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFD87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD880 size=108
    let mut pc: u32 = 0x82FFD880;
    'dispatch: loop {
        match pc {
            0x82FFD880 => {
    //   block [0x82FFD880..0x82FFD8EC)
	// 82FFD880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD88C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFD890: 419A0030  beq cr6, 0x82ffd8c0
	if ctx.cr[6].eq {
	pc = 0x82FFD8C0; continue 'dispatch;
	}
	// 82FFD894: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFD898: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFD89C: 40980024  bge cr6, 0x82ffd8c0
	if !ctx.cr[6].lt {
	pc = 0x82FFD8C0; continue 'dispatch;
	}
	// 82FFD8A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD8A4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FFD8A8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFD8AC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD8B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFD8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD8BC: 4E800020  blr
	return;
	// 82FFD8C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFD8C4: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD8C8: 38C00059  li r6, 0x59
	ctx.r[6].s64 = 89;
	// 82FFD8CC: 388BFBC8  addi r4, r11, -0x438
	ctx.r[4].s64 = ctx.r[11].s64 + -1080;
	// 82FFD8D0: 38A000C3  li r5, 0xc3
	ctx.r[5].s64 = 195;
	// 82FFD8D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFD8D8: 4BFD33F1  bl 0x82fd0cc8
	ctx.lr = 0x82FFD8DC;
	sub_82FD0CC8(ctx, base);
	// 82FFD8DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFD8E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFD8E4: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 82FFD8E8: 481B3341  bl 0x831b0c28
	ctx.lr = 0x82FFD8EC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFD8F0 size=12
    let mut pc: u32 = 0x82FFD8F0;
    'dispatch: loop {
        match pc {
            0x82FFD8F0 => {
    //   block [0x82FFD8F0..0x82FFD8FC)
	// 82FFD8F0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFD8F4: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82FFD8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFD900 size=12
    let mut pc: u32 = 0x82FFD900;
    'dispatch: loop {
        match pc {
            0x82FFD900 => {
    //   block [0x82FFD900..0x82FFD90C)
	// 82FFD900: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FFD904: 386B31B0  addi r3, r11, 0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + 12720;
	// 82FFD908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD910 size=96
    let mut pc: u32 = 0x82FFD910;
    'dispatch: loop {
        match pc {
            0x82FFD910 => {
    //   block [0x82FFD910..0x82FFD970)
	// 82FFD910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFD918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFD91C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFD920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFD92C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD930: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD938: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFD940: 4E800421  bctrl
	ctx.lr = 0x82FFD944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFD944: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFD948: 4182000C  beq 0x82ffd954
	if ctx.cr[0].eq {
	pc = 0x82FFD954; continue 'dispatch;
	}
	// 82FFD94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD950: 4BFDA991  bl 0x82fd82e0
	ctx.lr = 0x82FFD954;
	sub_82FD82E0(ctx, base);
	// 82FFD954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFD958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFD95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFD960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFD964: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFD968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFD96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFD970 size=152
    let mut pc: u32 = 0x82FFD970;
    'dispatch: loop {
        match pc {
            0x82FFD970 => {
    //   block [0x82FFD970..0x82FFDA08)
	// 82FFD970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFD974: 481AA7ED  bl 0x831a8160
	ctx.lr = 0x82FFD978;
	sub_831A8130(ctx, base);
	// 82FFD978: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFD97C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFD980: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FFD984: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FFD988: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD98C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFD990: 4099006C  ble cr6, 0x82ffd9fc
	if !ctx.cr[6].gt {
	pc = 0x82FFD9FC; continue 'dispatch;
	}
	// 82FFD994: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82FFD998: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD99C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FFD9A0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD9A4: 4182003C  beq 0x82ffd9e0
	if ctx.cr[0].eq {
	pc = 0x82FFD9E0; continue 'dispatch;
	}
	// 82FFD9A8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD9AC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFD9B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD9B4: 41820018  beq 0x82ffd9cc
	if ctx.cr[0].eq {
	pc = 0x82FFD9CC; continue 'dispatch;
	}
	// 82FFD9B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFD9BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFD9C0: 4182000C  beq 0x82ffd9cc
	if ctx.cr[0].eq {
	pc = 0x82FFD9CC; continue 'dispatch;
	}
	// 82FFD9C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFD9C8: 4BFFFF49  bl 0x82ffd910
	ctx.lr = 0x82FFD9CC;
	sub_82FFD910(ctx, base);
	// 82FFD9CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFD9D0: 4BFDA911  bl 0x82fd82e0
	ctx.lr = 0x82FFD9D4;
	sub_82FD82E0(ctx, base);
	// 82FFD9D4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FFD9D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FFD9DC: 409AFFCC  bne cr6, 0x82ffd9a8
	if !ctx.cr[6].eq {
	pc = 0x82FFD9A8; continue 'dispatch;
	}
	// 82FFD9E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFD9E4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FFD9E8: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 82FFD9EC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FFD9F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFD9F4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFD9F8: 4198FFA0  blt cr6, 0x82ffd998
	if ctx.cr[6].lt {
	pc = 0x82FFD998; continue 'dispatch;
	}
	// 82FFD9FC: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82FFDA00: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FFDA04: 481AA7AC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDA08 size=212
    let mut pc: u32 = 0x82FFDA08;
    'dispatch: loop {
        match pc {
            0x82FFDA08 => {
    //   block [0x82FFDA08..0x82FFDADC)
	// 82FFDA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDA0C: 481AA75D  bl 0x831a8168
	ctx.lr = 0x82FFDA10;
	sub_831A8130(ctx, base);
	// 82FFDA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDA14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDA18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFDA1C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFDA20: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDA24: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDA28: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 82FFDA2C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDA30: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDA34: 41980008  blt cr6, 0x82ffda3c
	if ctx.cr[6].lt {
	pc = 0x82FFDA3C; continue 'dispatch;
	}
	// 82FFDA38: 4806AFB1  bl 0x830689e8
	ctx.lr = 0x82FFDA3C;
	sub_830689E8(ctx, base);
	// 82FFDA3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFDA40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFDA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFDA48: 4BFFCA19  bl 0x82ffa460
	ctx.lr = 0x82FFDA4C;
	sub_82FFA460(ctx, base);
	// 82FFDA4C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFDA50: 41820030  beq 0x82ffda80
	if ctx.cr[0].eq {
	pc = 0x82FFDA80; continue 'dispatch;
	}
	// 82FFDA54: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDA58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDA5C: 41820018  beq 0x82ffda74
	if ctx.cr[0].eq {
	pc = 0x82FFDA74; continue 'dispatch;
	}
	// 82FFDA60: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDA64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDA68: 4182000C  beq 0x82ffda74
	if ctx.cr[0].eq {
	pc = 0x82FFDA74; continue 'dispatch;
	}
	// 82FFDA6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFDA70: 4BFFFEA1  bl 0x82ffd910
	ctx.lr = 0x82FFDA74;
	sub_82FFD910(ctx, base);
	// 82FFDA74: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FFDA78: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFDA7C: 48000058  b 0x82ffdad4
	pc = 0x82FFDAD4; continue 'dispatch;
	// 82FFDA80: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FFDA84: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDA88: 4BFDA811  bl 0x82fd8298
	ctx.lr = 0x82FFDA8C;
	sub_82FD8298(ctx, base);
	// 82FFDA8C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDA90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDA94: 41820024  beq 0x82ffdab8
	if ctx.cr[0].eq {
	pc = 0x82FFDAB8; continue 'dispatch;
	}
	// 82FFDA98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDA9C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FFDAA0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FFDAA4: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FFDAA8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82FFDAAC: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFDAB0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FFDAB4: 48000008  b 0x82ffdabc
	pc = 0x82FFDABC; continue 'dispatch;
	// 82FFDAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFDABC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDAC0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FFDAC4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82FFDAC8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDACC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFDAD0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFDAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFDAD8: 481AA6E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDAE0 size=8
    let mut pc: u32 = 0x82FFDAE0;
    'dispatch: loop {
        match pc {
            0x82FFDAE0 => {
    //   block [0x82FFDAE0..0x82FFDAE8)
	// 82FFDAE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFDAE4: 8213FC38  lwz r16, -0x3c8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDAE8 size=184
    let mut pc: u32 = 0x82FFDAE8;
    'dispatch: loop {
        match pc {
            0x82FFDAE8 => {
    //   block [0x82FFDAE8..0x82FFDBA0)
	// 82FFDAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDAEC: 481AA67D  bl 0x831a8168
	ctx.lr = 0x82FFDAF0;
	sub_831A8130(ctx, base);
	// 82FFDAF0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFDAF4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDAF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFDAFC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FFDB00: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FFDB04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFDB08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFDB0C: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FFDB10: 396BFBF8  addi r11, r11, -0x408
	ctx.r[11].s64 = ctx.r[11].s64 + -1032;
	// 82FFDB14: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82FFDB18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFDB1C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FFDB20: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FFDB24: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFDB28: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDB2C: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FFDB30: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FFDB34: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FFDB38: 4BFDA761  bl 0x82fd8298
	ctx.lr = 0x82FFDB3C;
	sub_82FD8298(ctx, base);
	// 82FFDB3C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFDB40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDB44: 41820014  beq 0x82ffdb58
	if ctx.cr[0].eq {
	pc = 0x82FFDB58; continue 'dispatch;
	}
	// 82FFDB48: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFDB4C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDB50: 4806CAB9  bl 0x8306a608
	ctx.lr = 0x82FFDB54;
	sub_8306A608(ctx, base);
	// 82FFDB54: 48000008  b 0x82ffdb5c
	pc = 0x82FFDB5C; continue 'dispatch;
	// 82FFDB58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDB5C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDB60: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDB64: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFDB68: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFDB6C: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFDB70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDB74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDB78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDB7C: 4E800421  bctrl
	ctx.lr = 0x82FFDB80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDB80: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDB84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFDB88: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FFDB8C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFDB90: 481AA651  bl 0x831a81e0
	ctx.lr = 0x82FFDB94;
	sub_831A81E0(ctx, base);
	// 82FFDB94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFDB98: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFDB9C: 481AA61C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDBA0 size=40
    let mut pc: u32 = 0x82FFDBA0;
    'dispatch: loop {
        match pc {
            0x82FFDBA0 => {
    //   block [0x82FFDBA0..0x82FFDBC8)
	// 82FFDBA0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFDBA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDBA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDBAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDBB0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFDBB4: 4804EBAD  bl 0x8304c760
	ctx.lr = 0x82FFDBB8;
	sub_8304C760(ctx, base);
	// 82FFDBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDBC8 size=48
    let mut pc: u32 = 0x82FFDBC8;
    'dispatch: loop {
        match pc {
            0x82FFDBC8 => {
    //   block [0x82FFDBC8..0x82FFDBF8)
	// 82FFDBC8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFDBCC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDBD0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDBD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDBD8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFDBDC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDBE0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDBE4: 4BFDA6FD  bl 0x82fd82e0
	ctx.lr = 0x82FFDBE8;
	sub_82FD82E0(ctx, base);
	// 82FFDBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDBF8 size=56
    let mut pc: u32 = 0x82FFDBF8;
    'dispatch: loop {
        match pc {
            0x82FFDBF8 => {
    //   block [0x82FFDBF8..0x82FFDC30)
	// 82FFDBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDC00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDC04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFDC08: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDC0C: 4BFE8B4D  bl 0x82fe6758
	ctx.lr = 0x82FFDC10;
	sub_82FE6758(ctx, base);
	// 82FFDC10: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FFDC14: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFDC18: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFDC1C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FFDC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDC30 size=20
    let mut pc: u32 = 0x82FFDC30;
    'dispatch: loop {
        match pc {
            0x82FFDC30 => {
    //   block [0x82FFDC30..0x82FFDC44)
	// 82FFDC30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFDC34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FFDC38: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDC3C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82FFDC40: 4BFFFD30  b 0x82ffd970
	sub_82FFD970(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDC48 size=72
    let mut pc: u32 = 0x82FFDC48;
    'dispatch: loop {
        match pc {
            0x82FFDC48 => {
    //   block [0x82FFDC48..0x82FFDC90)
	// 82FFDC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDC50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDC54: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFDC58: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDC5C: 4BFFC805  bl 0x82ffa460
	ctx.lr = 0x82FFDC60;
	sub_82FFA460(ctx, base);
	// 82FFDC60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDC64: 41820018  beq 0x82ffdc7c
	if ctx.cr[0].eq {
	pc = 0x82FFDC7C; continue 'dispatch;
	}
	// 82FFDC68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDC6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDC70: 4182000C  beq 0x82ffdc7c
	if ctx.cr[0].eq {
	pc = 0x82FFDC7C; continue 'dispatch;
	}
	// 82FFDC74: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDC78: 48000008  b 0x82ffdc80
	pc = 0x82FFDC80; continue 'dispatch;
	// 82FFDC7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFDC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDC90 size=8
    let mut pc: u32 = 0x82FFDC90;
    'dispatch: loop {
        match pc {
            0x82FFDC90 => {
    //   block [0x82FFDC90..0x82FFDC98)
	// 82FFDC90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFDC94: 8213FC80  lwz r16, -0x380(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FFDC98 size=292
    let mut pc: u32 = 0x82FFDC98;
    'dispatch: loop {
        match pc {
            0x82FFDC98 => {
    //   block [0x82FFDC98..0x82FFDDBC)
	// 82FFDC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDC9C: 481AA4C5  bl 0x831a8160
	ctx.lr = 0x82FFDCA0;
	sub_831A8130(ctx, base);
	// 82FFDCA0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FFDCA4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDCA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFDCAC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FFDCB0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDCB4: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDCB8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FFDCBC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDCC0: 409A0090  bne cr6, 0x82ffdd50
	if !ctx.cr[6].eq {
	pc = 0x82FFDD50; continue 'dispatch;
	}
	// 82FFDCC4: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82FFDCC8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDCCC: 395F0050  addi r10, r31, 0x50
	ctx.r[10].s64 = ctx.r[31].s64 + 80;
	// 82FFDCD0: F97F0050  std r11, 0x50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82FFDCD4: C81F0050  lfd f0, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	// 82FFDCD8: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82FFDCDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82FFDCE0: C80BA8D0  lfd f0, -0x5730(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) };
	// 82FFDCE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDCE8: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82FFDCEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDCF0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FFDCF4: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82FFDCF8: 837F0050  lwz r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDCFC: 577C103A  slwi r28, r27, 2
	ctx.r[28].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FFDD00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FFDD04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDD08: 4E800421  bctrl
	ctx.lr = 0x82FFDD0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDD0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FFDD10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFDD14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFDD18: 481AA4C9  bl 0x831a81e0
	ctx.lr = 0x82FFDD1C;
	sub_831A81E0(ctx, base);
	// 82FFDD1C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDD20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDD24: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDD28: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFDD2C: 481AA7E5  bl 0x831a8510
	ctx.lr = 0x82FFDD30;
	sub_831A8510(ctx, base);
	// 82FFDD30: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDD34: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDD38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDD3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDD40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDD44: 4E800421  bctrl
	ctx.lr = 0x82FFDD48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDD48: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFDD4C: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82FFDD50: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FFDD54: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDD58: 4BFDA541  bl 0x82fd8298
	ctx.lr = 0x82FFDD5C;
	sub_82FD8298(ctx, base);
	// 82FFDD5C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFDD60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDD64: 4182001C  beq 0x82ffdd80
	if ctx.cr[0].eq {
	pc = 0x82FFDD80; continue 'dispatch;
	}
	// 82FFDD68: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FFDD6C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDD70: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDD74: 4BFFFA9D  bl 0x82ffd810
	ctx.lr = 0x82FFDD78;
	sub_82FFD810(ctx, base);
	// 82FFDD78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFDD7C: 48000008  b 0x82ffdd84
	pc = 0x82FFDD84; continue 'dispatch;
	// 82FFDD80: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFDD84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFDD88: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDD8C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFDD90: 4BFFFC79  bl 0x82ffda08
	ctx.lr = 0x82FFDD94;
	sub_82FFDA08(ctx, base);
	// 82FFDD94: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDD98: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDD9C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FFDDA0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FFDDA4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDDA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FFDDAC: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FFDDB0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDDB4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FFDDB8: 481AA3F8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDDBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDDBC size=48
    let mut pc: u32 = 0x82FFDDBC;
    'dispatch: loop {
        match pc {
            0x82FFDDBC => {
    //   block [0x82FFDDBC..0x82FFDDEC)
	// 82FFDDBC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FFDDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDDCC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FFDDD0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDDD4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDDD8: 4BFDA509  bl 0x82fd82e0
	ctx.lr = 0x82FFDDDC;
	sub_82FD82E0(ctx, base);
	// 82FFDDDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDDF0 size=236
    let mut pc: u32 = 0x82FFDDF0;
    'dispatch: loop {
        match pc {
            0x82FFDDF0 => {
    //   block [0x82FFDDF0..0x82FFDEDC)
	// 82FFDDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDDF4: 481AA379  bl 0x831a816c
	ctx.lr = 0x82FFDDF8;
	sub_831A8130(ctx, base);
	// 82FFDDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDDFC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFDE00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFDE04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDE08: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FFDE0C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFDE10: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFDE14: 4182005C  beq 0x82ffde70
	if ctx.cr[0].eq {
	pc = 0x82FFDE70; continue 'dispatch;
	}
	// 82FFDE18: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDE1C: 4BFFB4DD  bl 0x82ff92f8
	ctx.lr = 0x82FFDE20;
	sub_82FF92F8(ctx, base);
	// 82FFDE20: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDE24: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FFDE28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FFDE2C: 409900A8  ble cr6, 0x82ffded4
	if !ctx.cr[6].gt {
	pc = 0x82FFDED4; continue 'dispatch;
	}
	// 82FFDE30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDE34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFDE38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFDE3C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFDE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDE44: 4E800421  bctrl
	ctx.lr = 0x82FFDE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDE48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFDE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FFDE50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFDE54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDE58: 4BFFBAA9  bl 0x82ff9900
	ctx.lr = 0x82FFDE5C;
	sub_82FF9900(ctx, base);
	// 82FFDE5C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFDE60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FFDE64: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDE68: 4198FFC8  blt cr6, 0x82ffde30
	if ctx.cr[6].lt {
	pc = 0x82FFDE30; continue 'dispatch;
	}
	// 82FFDE6C: 48000068  b 0x82ffded4
	pc = 0x82FFDED4; continue 'dispatch;
	// 82FFDE70: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82FFDE74: 4BFFB705  bl 0x82ff9578
	ctx.lr = 0x82FFDE78;
	sub_82FF9578(ctx, base);
	// 82FFDE78: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFDE7C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FFDE80: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FFDE84: 40990050  ble cr6, 0x82ffded4
	if !ctx.cr[6].gt {
	pc = 0x82FFDED4; continue 'dispatch;
	}
	// 82FFDE88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FFDE8C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82FFDE90: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82FFDE94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FFDE98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDE9C: 4BFFBC8D  bl 0x82ff9b28
	ctx.lr = 0x82FFDEA0;
	sub_82FF9B28(ctx, base);
	// 82FFDEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFDEA4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDEA8: 4BFFFDF1  bl 0x82ffdc98
	ctx.lr = 0x82FFDEAC;
	sub_82FFDC98(ctx, base);
	// 82FFDEAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDEB0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDEB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDEB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFDEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDEC0: 4E800421  bctrl
	ctx.lr = 0x82FFDEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDEC4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFDEC8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FFDECC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFDED0: 4198FFB8  blt cr6, 0x82ffde88
	if ctx.cr[6].lt {
	pc = 0x82FFDE88; continue 'dispatch;
	}
	// 82FFDED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFDED8: 481AA2E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFDEE0 size=8
    let mut pc: u32 = 0x82FFDEE0;
    'dispatch: loop {
        match pc {
            0x82FFDEE0 => {
    //   block [0x82FFDEE0..0x82FFDEE8)
	// 82FFDEE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFDEE4: 8213FCD0  lwz r16, -0x330(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-816 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDEE8 size=176
    let mut pc: u32 = 0x82FFDEE8;
    'dispatch: loop {
        match pc {
            0x82FFDEE8 => {
    //   block [0x82FFDEE8..0x82FFDF98)
	// 82FFDEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDEEC: 481AA281  bl 0x831a816c
	ctx.lr = 0x82FFDEF0;
	sub_831A8130(ctx, base);
	// 82FFDEF0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFDEF4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDEF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFDEFC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FFDF00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFDF04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFDF08: 909E0004  stw r4, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FFDF0C: 396BFBF8  addi r11, r11, -0x408
	ctx.r[11].s64 = ctx.r[11].s64 + -1032;
	// 82FFDF10: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82FFDF14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFDF18: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FFDF1C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FFDF20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFDF24: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FFDF28: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82FFDF2C: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FFDF30: 4BFDA369  bl 0x82fd8298
	ctx.lr = 0x82FFDF34;
	sub_82FD8298(ctx, base);
	// 82FFDF34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFDF38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFDF3C: 41820014  beq 0x82ffdf50
	if ctx.cr[0].eq {
	pc = 0x82FFDF50; continue 'dispatch;
	}
	// 82FFDF40: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 82FFDF44: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDF48: 4806C6C1  bl 0x8306a608
	ctx.lr = 0x82FFDF4C;
	sub_8306A608(ctx, base);
	// 82FFDF4C: 48000008  b 0x82ffdf54
	pc = 0x82FFDF54; continue 'dispatch;
	// 82FFDF50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFDF54: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDF58: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDF5C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFDF60: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FFDF64: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FFDF68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFDF6C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDF70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFDF74: 4E800421  bctrl
	ctx.lr = 0x82FFDF78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFDF78: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFDF7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFDF80: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FFDF84: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FFDF88: 481AA259  bl 0x831a81e0
	ctx.lr = 0x82FFDF8C;
	sub_831A81E0(ctx, base);
	// 82FFDF8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFDF90: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFDF94: 481AA228  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDF98 size=40
    let mut pc: u32 = 0x82FFDF98;
    'dispatch: loop {
        match pc {
            0x82FFDF98 => {
    //   block [0x82FFDF98..0x82FFDFC0)
	// 82FFDF98: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFDF9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDFA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDFA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDFA8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFDFAC: 4804E7B5  bl 0x8304c760
	ctx.lr = 0x82FFDFB0;
	sub_8304C760(ctx, base);
	// 82FFDFB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDFC0 size=48
    let mut pc: u32 = 0x82FFDFC0;
    'dispatch: loop {
        match pc {
            0x82FFDFC0 => {
    //   block [0x82FFDFC0..0x82FFDFF0)
	// 82FFDFC0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFDFC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDFC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDFCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFDFD0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FFDFD4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFDFD8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFDFDC: 4BFDA305  bl 0x82fd82e0
	ctx.lr = 0x82FFDFE0;
	sub_82FD82E0(ctx, base);
	// 82FFDFE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFDFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFDFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFDFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFDFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFDFF0 size=100
    let mut pc: u32 = 0x82FFDFF0;
    'dispatch: loop {
        match pc {
            0x82FFDFF0 => {
    //   block [0x82FFDFF0..0x82FFE054)
	// 82FFDFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFDFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFDFF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFDFFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE004: 4BFFF96D  bl 0x82ffd970
	ctx.lr = 0x82FFE008;
	sub_82FFD970(ctx, base);
	// 82FFE008: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE00C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE010: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE014: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE018: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE01C: 4E800421  bctrl
	ctx.lr = 0x82FFE020;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE020: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE028: 41820018  beq 0x82ffe040
	if ctx.cr[0].eq {
	pc = 0x82FFE040; continue 'dispatch;
	}
	// 82FFE02C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE030: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFE034: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE03C: 4E800421  bctrl
	ctx.lr = 0x82FFE040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE058 size=104
    let mut pc: u32 = 0x82FFE058;
    'dispatch: loop {
        match pc {
            0x82FFE058 => {
    //   block [0x82FFE058..0x82FFE0C0)
	// 82FFE058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE070: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FFE074: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFE078: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE07C: 4BFFC3E5  bl 0x82ffa460
	ctx.lr = 0x82FFE080;
	sub_82FFA460(ctx, base);
	// 82FFE080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE084: 41820018  beq 0x82ffe09c
	if ctx.cr[0].eq {
	pc = 0x82FFE09C; continue 'dispatch;
	}
	// 82FFE088: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE08C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE090: 4182000C  beq 0x82ffe09c
	if ctx.cr[0].eq {
	pc = 0x82FFE09C; continue 'dispatch;
	}
	// 82FFE094: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE098: 48000010  b 0x82ffe0a8
	pc = 0x82FFE0A8; continue 'dispatch;
	// 82FFE09C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE0A4: 4BFFFBF5  bl 0x82ffdc98
	ctx.lr = 0x82FFE0A8;
	sub_82FFDC98(ctx, base);
	// 82FFE0A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE0B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE0B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE0C0 size=8
    let mut pc: u32 = 0x82FFE0C0;
    'dispatch: loop {
        match pc {
            0x82FFE0C0 => {
    //   block [0x82FFE0C0..0x82FFE0C8)
	// 82FFE0C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFE0C4: 8213FD18  lwz r16, -0x2e8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE0C8 size=96
    let mut pc: u32 = 0x82FFE0C8;
    'dispatch: loop {
        match pc {
            0x82FFE0C8 => {
    //   block [0x82FFE0C8..0x82FFE128)
	// 82FFE0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE0D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE0D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE0D8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FFE0DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE0E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE0E4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FFE0E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE0EC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FFE0F0: 4BFDA1A9  bl 0x82fd8298
	ctx.lr = 0x82FFE0F4;
	sub_82FD8298(ctx, base);
	// 82FFE0F4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFE0F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE0FC: 41820010  beq 0x82ffe10c
	if ctx.cr[0].eq {
	pc = 0x82FFE10C; continue 'dispatch;
	}
	// 82FFE100: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE104: 4BFFFDE5  bl 0x82ffdee8
	ctx.lr = 0x82FFE108;
	sub_82FFDEE8(ctx, base);
	// 82FFE108: 48000008  b 0x82ffe110
	pc = 0x82FFE110; continue 'dispatch;
	// 82FFE10C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE110: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FFE114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE11C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE128 size=44
    let mut pc: u32 = 0x82FFE128;
    'dispatch: loop {
        match pc {
            0x82FFE128 => {
    //   block [0x82FFE128..0x82FFE154)
	// 82FFE128: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFE12C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE130: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE138: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FFE13C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFE140: 4BFDA1A1  bl 0x82fd82e0
	ctx.lr = 0x82FFE144;
	sub_82FD82E0(ctx, base);
	// 82FFE144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE158 size=8
    let mut pc: u32 = 0x82FFE158;
    'dispatch: loop {
        match pc {
            0x82FFE158 => {
    //   block [0x82FFE158..0x82FFE160)
	// 82FFE158: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFE15C: 8213FD50  lwz r16, -0x2b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE160 size=108
    let mut pc: u32 = 0x82FFE160;
    'dispatch: loop {
        match pc {
            0x82FFE160 => {
    //   block [0x82FFE160..0x82FFE1CC)
	// 82FFE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE164: 481AA009  bl 0x831a816c
	ctx.lr = 0x82FFE168;
	sub_831A8130(ctx, base);
	// 82FFE168: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FFE16C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE170: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFE174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE178: 396BFBF8  addi r11, r11, -0x408
	ctx.r[11].s64 = ctx.r[11].s64 + -1032;
	// 82FFE17C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FFE180: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE184: 83BE000C  lwz r29, 0xc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE188: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE18C: 41820014  beq 0x82ffe1a0
	if ctx.cr[0].eq {
	pc = 0x82FFE1A0; continue 'dispatch;
	}
	// 82FFE190: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE194: 4BFFFE5D  bl 0x82ffdff0
	ctx.lr = 0x82FFE198;
	sub_82FFDFF0(ctx, base);
	// 82FFE198: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE19C: 4BFDA145  bl 0x82fd82e0
	ctx.lr = 0x82FFE1A0;
	sub_82FD82E0(ctx, base);
	// 82FFE1A0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE1A4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE1A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE1AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE1B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE1B4: 4E800421  bctrl
	ctx.lr = 0x82FFE1B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE1B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFE1BC: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FFE1C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE1C4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FFE1C8: 481A9FF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE1CC size=40
    let mut pc: u32 = 0x82FFE1CC;
    'dispatch: loop {
        match pc {
            0x82FFE1CC => {
    //   block [0x82FFE1CC..0x82FFE1F4)
	// 82FFE1CC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FFE1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE1DC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FFE1E0: 4804E581  bl 0x8304c760
	ctx.lr = 0x82FFE1E4;
	sub_8304C760(ctx, base);
	// 82FFE1E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE1F8 size=76
    let mut pc: u32 = 0x82FFE1F8;
    'dispatch: loop {
        match pc {
            0x82FFE1F8 => {
    //   block [0x82FFE1F8..0x82FFE244)
	// 82FFE1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE20C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE210: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFE214: 4BFFFF4D  bl 0x82ffe160
	ctx.lr = 0x82FFE218;
	sub_82FFE160(ctx, base);
	// 82FFE218: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE21C: 4182000C  beq 0x82ffe228
	if ctx.cr[0].eq {
	pc = 0x82FFE228; continue 'dispatch;
	}
	// 82FFE220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE224: 4BFDA0BD  bl 0x82fd82e0
	ctx.lr = 0x82FFE228;
	sub_82FD82E0(ctx, base);
	// 82FFE228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE22C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE248 size=68
    let mut pc: u32 = 0x82FFE248;
    'dispatch: loop {
        match pc {
            0x82FFE248 => {
    //   block [0x82FFE248..0x82FFE28C)
	// 82FFE248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE258: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFE25C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE260: 396BFD80  addi r11, r11, -0x280
	ctx.r[11].s64 = ctx.r[11].s64 + -640;
	// 82FFE264: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFE268: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE26C: 41820008  beq 0x82ffe274
	if ctx.cr[0].eq {
	pc = 0x82FFE274; continue 'dispatch;
	}
	// 82FFE270: 4B2C1FF9  bl 0x822c0268
	ctx.lr = 0x82FFE274;
	sub_822C0268(ctx, base);
	// 82FFE274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFE290 size=36
    let mut pc: u32 = 0x82FFE290;
    'dispatch: loop {
        match pc {
            0x82FFE290 => {
    //   block [0x82FFE290..0x82FFE2B4)
	// 82FFE290: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFE294: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FFE298: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82FFE29C: 396BFDBC  addi r11, r11, -0x244
	ctx.r[11].s64 = ctx.r[11].s64 + -580;
	// 82FFE2A0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FFE2A4: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FFE2A8: 98E30014  stb r7, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 82FFE2AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFE2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE2B8 size=160
    let mut pc: u32 = 0x82FFE2B8;
    'dispatch: loop {
        match pc {
            0x82FFE2B8 => {
    //   block [0x82FFE2B8..0x82FFE358)
	// 82FFE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE2C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE2C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE2C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE2CC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFE2D0: 409A0070  bne cr6, 0x82ffe340
	if !ctx.cr[6].eq {
	pc = 0x82FFE340; continue 'dispatch;
	}
	// 82FFE2D4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE2D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE2DC: 41820038  beq 0x82ffe314
	if ctx.cr[0].eq {
	pc = 0x82FFE314; continue 'dispatch;
	}
	// 82FFE2E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE2E4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FFE2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE2EC: 4E800421  bctrl
	ctx.lr = 0x82FFE2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE2F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE2F4: 41820020  beq 0x82ffe314
	if ctx.cr[0].eq {
	pc = 0x82FFE314; continue 'dispatch;
	}
	// 82FFE2F8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE2FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE300: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FFE304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE308: 4E800421  bctrl
	ctx.lr = 0x82FFE30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE30C: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFE310: 4800000C  b 0x82ffe31c
	pc = 0x82FFE31C; continue 'dispatch;
	// 82FFE314: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFE318: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFE31C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82FFE320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFE324: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FFE328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFE32C: 4BFFBBA5  bl 0x82ff9ed0
	ctx.lr = 0x82FFE330;
	sub_82FF9ED0(ctx, base);
	// 82FFE330: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFE334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFE338: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFE33C: 481B28ED  bl 0x831b0c28
	ctx.lr = 0x82FFE340;
	sub_831B0C28(ctx, base);
	// 82FFE340: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FFE344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFE348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FFE358 size=216
    let mut pc: u32 = 0x82FFE358;
    'dispatch: loop {
        match pc {
            0x82FFE358 => {
    //   block [0x82FFE358..0x82FFE430)
	// 82FFE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE36C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE370: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE378: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE37C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFE380: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE384: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE38C: 409A0034  bne cr6, 0x82ffe3c0
	if !ctx.cr[6].eq {
	pc = 0x82FFE3C0; continue 'dispatch;
	}
	// 82FFE390: 4E800421  bctrl
	ctx.lr = 0x82FFE394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE394: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE398: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FFE39C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE3A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFE3A4: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFE3A8: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82FFE3AC: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82FFE3B0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FFE3B4: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FFE3B8: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 82FFE3BC: 4800005C  b 0x82ffe418
	pc = 0x82FFE418; continue 'dispatch;
	// 82FFE3C0: 4E800421  bctrl
	ctx.lr = 0x82FFE3C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE3C4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE3C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FFE3CC: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE3D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFE3D4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFE3D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE3DC: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFE3E0: 7D6B4839  and. r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE3E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE3E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFE3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE3F0: 4182000C  beq 0x82ffe3fc
	if ctx.cr[0].eq {
	pc = 0x82FFE3FC; continue 'dispatch;
	}
	// 82FFE3F4: 4E800421  bctrl
	ctx.lr = 0x82FFE3F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE3F8: 48000020  b 0x82ffe418
	pc = 0x82FFE418; continue 'dispatch;
	// 82FFE3FC: 4E800421  bctrl
	ctx.lr = 0x82FFE400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE400: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE404: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82FFE408: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFE40C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFE410: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FFE414: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 82FFE418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE430 size=140
    let mut pc: u32 = 0x82FFE430;
    'dispatch: loop {
        match pc {
            0x82FFE430 => {
    //   block [0x82FFE430..0x82FFE4BC)
	// 82FFE430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE43C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE444: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE448: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFE44C: 419A0054  beq cr6, 0x82ffe4a0
	if ctx.cr[6].eq {
	pc = 0x82FFE4A0; continue 'dispatch;
	}
	// 82FFE450: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE454: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE458: 419A0048  beq cr6, 0x82ffe4a0
	if ctx.cr[6].eq {
	pc = 0x82FFE4A0; continue 'dispatch;
	}
	// 82FFE45C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE460: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFE464: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE46C: 4E800421  bctrl
	ctx.lr = 0x82FFE470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE470: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFE474: 4182002C  beq 0x82ffe4a0
	if ctx.cr[0].eq {
	pc = 0x82FFE4A0; continue 'dispatch;
	}
	// 82FFE478: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE480: 4BFFFED9  bl 0x82ffe358
	ctx.lr = 0x82FFE484;
	sub_82FFE358(ctx, base);
	// 82FFE484: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE488: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE48C: 419A000C  beq cr6, 0x82ffe498
	if ctx.cr[6].eq {
	pc = 0x82FFE498; continue 'dispatch;
	}
	// 82FFE490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE494: 4BFFFFBC  b 0x82ffe450
	pc = 0x82FFE450; continue 'dispatch;
	// 82FFE498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE49C: 48000008  b 0x82ffe4a4
	pc = 0x82FFE4A4; continue 'dispatch;
	// 82FFE4A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE4A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE4B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE4B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE4C0 size=248
    let mut pc: u32 = 0x82FFE4C0;
    'dispatch: loop {
        match pc {
            0x82FFE4C0 => {
    //   block [0x82FFE4C0..0x82FFE5B8)
	// 82FFE4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE4C4: 481A9CA9  bl 0x831a816c
	ctx.lr = 0x82FFE4C8;
	sub_831A8130(ctx, base);
	// 82FFE4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE4CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE4D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFE4D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFE4D8: 419A006C  beq cr6, 0x82ffe544
	if ctx.cr[6].eq {
	pc = 0x82FFE544; continue 'dispatch;
	}
	// 82FFE4DC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE4E0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE4E4: 419A0060  beq cr6, 0x82ffe544
	if ctx.cr[6].eq {
	pc = 0x82FFE544; continue 'dispatch;
	}
	// 82FFE4E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE4F0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFE4F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE4F8: 4E800421  bctrl
	ctx.lr = 0x82FFE4FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE4FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFE500: 40820050  bne 0x82ffe550
	if !ctx.cr[0].eq {
	pc = 0x82FFE550; continue 'dispatch;
	}
	// 82FFE504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE50C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE514: 4E800421  bctrl
	ctx.lr = 0x82FFE518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE518: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFE51C: 41820028  beq 0x82ffe544
	if ctx.cr[0].eq {
	pc = 0x82FFE544; continue 'dispatch;
	}
	// 82FFE520: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE524: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE528: 419A001C  beq cr6, 0x82ffe544
	if ctx.cr[6].eq {
	pc = 0x82FFE544; continue 'dispatch;
	}
	// 82FFE52C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE530: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE534: 4BFFFE25  bl 0x82ffe358
	ctx.lr = 0x82FFE538;
	sub_82FFE358(ctx, base);
	// 82FFE538: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE53C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE540: 419A0060  beq cr6, 0x82ffe5a0
	if ctx.cr[6].eq {
	pc = 0x82FFE5A0; continue 'dispatch;
	}
	// 82FFE544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE548: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE54C: 481A9C70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FFE550: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE558: 4BFFFE01  bl 0x82ffe358
	ctx.lr = 0x82FFE55C;
	sub_82FFE358(ctx, base);
	// 82FFE55C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE560: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE564: 419A0044  beq cr6, 0x82ffe5a8
	if ctx.cr[6].eq {
	pc = 0x82FFE5A8; continue 'dispatch;
	}
	// 82FFE568: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE56C: 409A0034  bne cr6, 0x82ffe5a0
	if !ctx.cr[6].eq {
	pc = 0x82FFE5A0; continue 'dispatch;
	}
	// 82FFE570: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE578: 48000139  bl 0x82ffe6b0
	ctx.lr = 0x82FFE57C;
	sub_82FFE6B0(ctx, base);
	// 82FFE57C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFE580: 40820030  bne 0x82ffe5b0
	if !ctx.cr[0].eq {
	pc = 0x82FFE5B0; continue 'dispatch;
	}
	// 82FFE584: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE58C: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFE590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE594: 4E800421  bctrl
	ctx.lr = 0x82FFE598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE598: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE59C: 40820014  bne 0x82ffe5b0
	if !ctx.cr[0].eq {
	pc = 0x82FFE5B0; continue 'dispatch;
	}
	// 82FFE5A0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FFE5A4: 4BFFFF38  b 0x82ffe4dc
	pc = 0x82FFE4DC; continue 'dispatch;
	// 82FFE5A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE5AC: 4BFFFF9C  b 0x82ffe548
	pc = 0x82FFE548; continue 'dispatch;
	// 82FFE5B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE5B4: 4BFFFF94  b 0x82ffe548
	pc = 0x82FFE548; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE5B8 size=248
    let mut pc: u32 = 0x82FFE5B8;
    'dispatch: loop {
        match pc {
            0x82FFE5B8 => {
    //   block [0x82FFE5B8..0x82FFE6B0)
	// 82FFE5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE5BC: 481A9BB1  bl 0x831a816c
	ctx.lr = 0x82FFE5C0;
	sub_831A8130(ctx, base);
	// 82FFE5C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE5C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFE5C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFE5CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFE5D0: 419A006C  beq cr6, 0x82ffe63c
	if ctx.cr[6].eq {
	pc = 0x82FFE63C; continue 'dispatch;
	}
	// 82FFE5D4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE5D8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE5DC: 419A0060  beq cr6, 0x82ffe63c
	if ctx.cr[6].eq {
	pc = 0x82FFE63C; continue 'dispatch;
	}
	// 82FFE5E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE5E8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFE5EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE5F0: 4E800421  bctrl
	ctx.lr = 0x82FFE5F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE5F4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFE5F8: 40820050  bne 0x82ffe648
	if !ctx.cr[0].eq {
	pc = 0x82FFE648; continue 'dispatch;
	}
	// 82FFE5FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE604: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE60C: 4E800421  bctrl
	ctx.lr = 0x82FFE610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE610: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFE614: 41820028  beq 0x82ffe63c
	if ctx.cr[0].eq {
	pc = 0x82FFE63C; continue 'dispatch;
	}
	// 82FFE618: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFE61C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFE620: 419A001C  beq cr6, 0x82ffe63c
	if ctx.cr[6].eq {
	pc = 0x82FFE63C; continue 'dispatch;
	}
	// 82FFE624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE628: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE62C: 4BFFFD2D  bl 0x82ffe358
	ctx.lr = 0x82FFE630;
	sub_82FFE358(ctx, base);
	// 82FFE630: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE634: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE638: 419A0060  beq cr6, 0x82ffe698
	if ctx.cr[6].eq {
	pc = 0x82FFE698; continue 'dispatch;
	}
	// 82FFE63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE644: 481A9B78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FFE648: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE64C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE650: 4BFFFD09  bl 0x82ffe358
	ctx.lr = 0x82FFE654;
	sub_82FFE358(ctx, base);
	// 82FFE654: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE658: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE65C: 419A0044  beq cr6, 0x82ffe6a0
	if ctx.cr[6].eq {
	pc = 0x82FFE6A0; continue 'dispatch;
	}
	// 82FFE660: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE664: 409A0034  bne cr6, 0x82ffe698
	if !ctx.cr[6].eq {
	pc = 0x82FFE698; continue 'dispatch;
	}
	// 82FFE668: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFE66C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFE670: 480000F9  bl 0x82ffe768
	ctx.lr = 0x82FFE674;
	sub_82FFE768(ctx, base);
	// 82FFE674: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFE678: 40820030  bne 0x82ffe6a8
	if !ctx.cr[0].eq {
	pc = 0x82FFE6A8; continue 'dispatch;
	}
	// 82FFE67C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE680: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE684: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFE688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE68C: 4E800421  bctrl
	ctx.lr = 0x82FFE690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE690: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE694: 40820014  bne 0x82ffe6a8
	if !ctx.cr[0].eq {
	pc = 0x82FFE6A8; continue 'dispatch;
	}
	// 82FFE698: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FFE69C: 4BFFFF38  b 0x82ffe5d4
	pc = 0x82FFE5D4; continue 'dispatch;
	// 82FFE6A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE6A4: 4BFFFF9C  b 0x82ffe640
	pc = 0x82FFE640; continue 'dispatch;
	// 82FFE6A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE6AC: 4BFFFF94  b 0x82ffe640
	pc = 0x82FFE640; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE6B0 size=180
    let mut pc: u32 = 0x82FFE6B0;
    'dispatch: loop {
        match pc {
            0x82FFE6B0 => {
    //   block [0x82FFE6B0..0x82FFE764)
	// 82FFE6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE6C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFE6CC: 419A0064  beq cr6, 0x82ffe730
	if ctx.cr[6].eq {
	pc = 0x82FFE730; continue 'dispatch;
	}
	// 82FFE6D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE6D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFE6D8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFE6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE6E0: 4E800421  bctrl
	ctx.lr = 0x82FFE6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE6E4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFE6E8: 41820048  beq 0x82ffe730
	if ctx.cr[0].eq {
	pc = 0x82FFE730; continue 'dispatch;
	}
	// 82FFE6EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE6F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE6F4: 4BFFFC65  bl 0x82ffe358
	ctx.lr = 0x82FFE6F8;
	sub_82FFE358(ctx, base);
	// 82FFE6F8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE6FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE700: 419A0038  beq cr6, 0x82ffe738
	if ctx.cr[6].eq {
	pc = 0x82FFE738; continue 'dispatch;
	}
	// 82FFE704: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE708: 409A0038  bne cr6, 0x82ffe740
	if !ctx.cr[6].eq {
	pc = 0x82FFE740; continue 'dispatch;
	}
	// 82FFE70C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE714: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFE718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE71C: 4E800421  bctrl
	ctx.lr = 0x82FFE720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE720: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE724: 4182001C  beq 0x82ffe740
	if ctx.cr[0].eq {
	pc = 0x82FFE740; continue 'dispatch;
	}
	// 82FFE728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE72C: 4BFFFFA4  b 0x82ffe6d0
	pc = 0x82FFE6D0; continue 'dispatch;
	// 82FFE730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE734: 48000018  b 0x82ffe74c
	pc = 0x82FFE74C; continue 'dispatch;
	// 82FFE738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE73C: 48000010  b 0x82ffe74c
	pc = 0x82FFE74C; continue 'dispatch;
	// 82FFE740: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE748: 4BFFFD79  bl 0x82ffe4c0
	ctx.lr = 0x82FFE74C;
	sub_82FFE4C0(ctx, base);
	// 82FFE74C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE758: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE75C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE768 size=180
    let mut pc: u32 = 0x82FFE768;
    'dispatch: loop {
        match pc {
            0x82FFE768 => {
    //   block [0x82FFE768..0x82FFE81C)
	// 82FFE768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE77C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFE780: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFE784: 419A0064  beq cr6, 0x82ffe7e8
	if ctx.cr[6].eq {
	pc = 0x82FFE7E8; continue 'dispatch;
	}
	// 82FFE788: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE78C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFE790: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFE794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE798: 4E800421  bctrl
	ctx.lr = 0x82FFE79C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE79C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFE7A0: 41820048  beq 0x82ffe7e8
	if ctx.cr[0].eq {
	pc = 0x82FFE7E8; continue 'dispatch;
	}
	// 82FFE7A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE7A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE7AC: 4BFFFBAD  bl 0x82ffe358
	ctx.lr = 0x82FFE7B0;
	sub_82FFE358(ctx, base);
	// 82FFE7B0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFE7B4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFE7B8: 419A0038  beq cr6, 0x82ffe7f0
	if ctx.cr[6].eq {
	pc = 0x82FFE7F0; continue 'dispatch;
	}
	// 82FFE7BC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FFE7C0: 409A0038  bne cr6, 0x82ffe7f8
	if !ctx.cr[6].eq {
	pc = 0x82FFE7F8; continue 'dispatch;
	}
	// 82FFE7C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFE7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE7CC: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFE7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFE7D4: 4E800421  bctrl
	ctx.lr = 0x82FFE7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFE7D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFE7DC: 4182001C  beq 0x82ffe7f8
	if ctx.cr[0].eq {
	pc = 0x82FFE7F8; continue 'dispatch;
	}
	// 82FFE7E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE7E4: 4BFFFFA4  b 0x82ffe788
	pc = 0x82FFE788; continue 'dispatch;
	// 82FFE7E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE7EC: 48000018  b 0x82ffe804
	pc = 0x82FFE804; continue 'dispatch;
	// 82FFE7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE7F4: 48000010  b 0x82ffe804
	pc = 0x82FFE804; continue 'dispatch;
	// 82FFE7F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFE7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFE800: 4BFFFDB9  bl 0x82ffe5b8
	ctx.lr = 0x82FFE804;
	sub_82FFE5B8(ctx, base);
	// 82FFE804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFE808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFE814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE820 size=80
    let mut pc: u32 = 0x82FFE820;
    'dispatch: loop {
        match pc {
            0x82FFE820 => {
    //   block [0x82FFE820..0x82FFE870)
	// 82FFE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE834: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE838: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE83C: 4082000C  bne 0x82ffe848
	if !ctx.cr[0].eq {
	pc = 0x82FFE848; continue 'dispatch;
	}
	// 82FFE840: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE844: 48000018  b 0x82ffe85c
	pc = 0x82FFE85C; continue 'dispatch;
	// 82FFE848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE84C: 4BFFFBE5  bl 0x82ffe430
	ctx.lr = 0x82FFE850;
	sub_82FFE430(ctx, base);
	// 82FFE850: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE854: 41820008  beq 0x82ffe85c
	if ctx.cr[0].eq {
	pc = 0x82FFE85C; continue 'dispatch;
	}
	// 82FFE858: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFE85C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE870 size=80
    let mut pc: u32 = 0x82FFE870;
    'dispatch: loop {
        match pc {
            0x82FFE870 => {
    //   block [0x82FFE870..0x82FFE8C0)
	// 82FFE870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE87C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE884: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE888: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE88C: 4082000C  bne 0x82ffe898
	if !ctx.cr[0].eq {
	pc = 0x82FFE898; continue 'dispatch;
	}
	// 82FFE890: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE894: 48000018  b 0x82ffe8ac
	pc = 0x82FFE8AC; continue 'dispatch;
	// 82FFE898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE89C: 4BFFFE15  bl 0x82ffe6b0
	ctx.lr = 0x82FFE8A0;
	sub_82FFE6B0(ctx, base);
	// 82FFE8A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE8A4: 41820008  beq 0x82ffe8ac
	if ctx.cr[0].eq {
	pc = 0x82FFE8AC; continue 'dispatch;
	}
	// 82FFE8A8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFE8AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE8B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE8C0 size=80
    let mut pc: u32 = 0x82FFE8C0;
    'dispatch: loop {
        match pc {
            0x82FFE8C0 => {
    //   block [0x82FFE8C0..0x82FFE910)
	// 82FFE8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE8C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE8CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE8D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE8D4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE8D8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE8DC: 4082000C  bne 0x82ffe8e8
	if !ctx.cr[0].eq {
	pc = 0x82FFE8E8; continue 'dispatch;
	}
	// 82FFE8E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE8E4: 48000018  b 0x82ffe8fc
	pc = 0x82FFE8FC; continue 'dispatch;
	// 82FFE8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE8EC: 4BFFFE7D  bl 0x82ffe768
	ctx.lr = 0x82FFE8F0;
	sub_82FFE768(ctx, base);
	// 82FFE8F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE8F4: 41820008  beq 0x82ffe8fc
	if ctx.cr[0].eq {
	pc = 0x82FFE8FC; continue 'dispatch;
	}
	// 82FFE8F8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFE8FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE910 size=80
    let mut pc: u32 = 0x82FFE910;
    'dispatch: loop {
        match pc {
            0x82FFE910 => {
    //   block [0x82FFE910..0x82FFE960)
	// 82FFE910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE91C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE924: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE928: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE92C: 4082000C  bne 0x82ffe938
	if !ctx.cr[0].eq {
	pc = 0x82FFE938; continue 'dispatch;
	}
	// 82FFE930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE934: 48000018  b 0x82ffe94c
	pc = 0x82FFE94C; continue 'dispatch;
	// 82FFE938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE93C: 4BFFFC7D  bl 0x82ffe5b8
	ctx.lr = 0x82FFE940;
	sub_82FFE5B8(ctx, base);
	// 82FFE940: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE944: 41820008  beq 0x82ffe94c
	if ctx.cr[0].eq {
	pc = 0x82FFE94C; continue 'dispatch;
	}
	// 82FFE948: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFE94C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE960 size=80
    let mut pc: u32 = 0x82FFE960;
    'dispatch: loop {
        match pc {
            0x82FFE960 => {
    //   block [0x82FFE960..0x82FFE9B0)
	// 82FFE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE974: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE978: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE97C: 4082000C  bne 0x82ffe988
	if !ctx.cr[0].eq {
	pc = 0x82FFE988; continue 'dispatch;
	}
	// 82FFE980: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE984: 48000018  b 0x82ffe99c
	pc = 0x82FFE99C; continue 'dispatch;
	// 82FFE988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE98C: 4BFFFB35  bl 0x82ffe4c0
	ctx.lr = 0x82FFE990;
	sub_82FFE4C0(ctx, base);
	// 82FFE990: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE994: 41820008  beq 0x82ffe99c
	if ctx.cr[0].eq {
	pc = 0x82FFE99C; continue 'dispatch;
	}
	// 82FFE998: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFE99C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFE9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFE9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFE9A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFE9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFE9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFE9B0 size=144
    let mut pc: u32 = 0x82FFE9B0;
    'dispatch: loop {
        match pc {
            0x82FFE9B0 => {
    //   block [0x82FFE9B0..0x82FFEA40)
	// 82FFE9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFE9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFE9B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFE9BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFE9C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFE9C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFE9C8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE9CC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE9D0: 4082000C  bne 0x82ffe9dc
	if !ctx.cr[0].eq {
	pc = 0x82FFE9DC; continue 'dispatch;
	}
	// 82FFE9D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFE9D8: 48000050  b 0x82ffea28
	pc = 0x82FFEA28; continue 'dispatch;
	// 82FFE9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE9E0: 4BFFFBD9  bl 0x82ffe5b8
	ctx.lr = 0x82FFE9E4;
	sub_82FFE5B8(ctx, base);
	// 82FFE9E4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFE9E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFE9EC: 4082001C  bne 0x82ffea08
	if !ctx.cr[0].eq {
	pc = 0x82FFEA08; continue 'dispatch;
	}
	// 82FFE9F0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFE9F4: 4BFFFA3D  bl 0x82ffe430
	ctx.lr = 0x82FFE9F8;
	sub_82FFE430(ctx, base);
	// 82FFE9F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFE9FC: 4182002C  beq 0x82ffea28
	if ctx.cr[0].eq {
	pc = 0x82FFEA28; continue 'dispatch;
	}
	// 82FFEA00: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFEA04: 48000024  b 0x82ffea28
	pc = 0x82FFEA28; continue 'dispatch;
	// 82FFEA08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFEA0C: 4BFFFD5D  bl 0x82ffe768
	ctx.lr = 0x82FFEA10;
	sub_82FFE768(ctx, base);
	// 82FFEA10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEA14: 4182000C  beq 0x82ffea20
	if ctx.cr[0].eq {
	pc = 0x82FFEA20; continue 'dispatch;
	}
	// 82FFEA18: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFEA1C: 48000008  b 0x82ffea24
	pc = 0x82FFEA24; continue 'dispatch;
	// 82FFEA20: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FFEA24: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEA28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEA34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFEA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFEA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEA40 size=152
    let mut pc: u32 = 0x82FFEA40;
    'dispatch: loop {
        match pc {
            0x82FFEA40 => {
    //   block [0x82FFEA40..0x82FFEAD8)
	// 82FFEA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEA48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFEA4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFEA50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEA54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEA58: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEA5C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEA60: 4182005C  beq 0x82ffeabc
	if ctx.cr[0].eq {
	pc = 0x82FFEABC; continue 'dispatch;
	}
	// 82FFEA64: 4BFFFC4D  bl 0x82ffe6b0
	ctx.lr = 0x82FFEA68;
	sub_82FFE6B0(ctx, base);
	// 82FFEA68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEA6C: 4182000C  beq 0x82ffea78
	if ctx.cr[0].eq {
	pc = 0x82FFEA78; continue 'dispatch;
	}
	// 82FFEA70: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FFEA74: 4800004C  b 0x82ffeac0
	pc = 0x82FFEAC0; continue 'dispatch;
	// 82FFEA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEA7C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEA80: 4BFFFA41  bl 0x82ffe4c0
	ctx.lr = 0x82FFEA84;
	sub_82FFE4C0(ctx, base);
	// 82FFEA84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEA88: 4082FFE8  bne 0x82ffea70
	if !ctx.cr[0].eq {
	pc = 0x82FFEA70; continue 'dispatch;
	}
	// 82FFEA8C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFEA90: 4800001C  b 0x82ffeaac
	pc = 0x82FFEAAC; continue 'dispatch;
	// 82FFEA94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFEA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEA9C: 4BFFFA25  bl 0x82ffe4c0
	ctx.lr = 0x82FFEAA0;
	sub_82FFE4C0(ctx, base);
	// 82FFEAA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEAA4: 4082FFCC  bne 0x82ffea70
	if !ctx.cr[0].eq {
	pc = 0x82FFEA70; continue 'dispatch;
	}
	// 82FFEAA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFEAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEAB0: 4BFFF981  bl 0x82ffe430
	ctx.lr = 0x82FFEAB4;
	sub_82FFE430(ctx, base);
	// 82FFEAB4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFEAB8: 4082FFDC  bne 0x82ffea94
	if !ctx.cr[0].eq {
	pc = 0x82FFEA94; continue 'dispatch;
	}
	// 82FFEABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFEAC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEACC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFEAD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFEAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEAD8 size=60
    let mut pc: u32 = 0x82FFEAD8;
    'dispatch: loop {
        match pc {
            0x82FFEAD8 => {
    //   block [0x82FFEAD8..0x82FFEB14)
	// 82FFEAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEADC: 481A9691  bl 0x831a816c
	ctx.lr = 0x82FFEAE0;
	sub_831A8130(ctx, base);
	// 82FFEAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEAE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEAE8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FFEAEC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82FFEAF0: 4800D969  bl 0x8300c458
	ctx.lr = 0x82FFEAF4;
	sub_8300C458(ctx, base);
	// 82FFEAF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEAF8: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82FFEAFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEB00: 93BF0044  stw r29, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 82FFEB04: 396BFDF8  addi r11, r11, -0x208
	ctx.r[11].s64 = ctx.r[11].s64 + -520;
	// 82FFEB08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFEB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEB10: 481A96AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEB18 size=88
    let mut pc: u32 = 0x82FFEB18;
    'dispatch: loop {
        match pc {
            0x82FFEB18 => {
    //   block [0x82FFEB18..0x82FFEB70)
	// 82FFEB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEB20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFEB24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFEB28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEB2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEB30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFEB34: 4800CCFD  bl 0x8300b830
	ctx.lr = 0x82FFEB38;
	sub_8300B830(ctx, base);
	// 82FFEB38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEB3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEB40: 396BFDF8  addi r11, r11, -0x208
	ctx.r[11].s64 = ctx.r[11].s64 + -520;
	// 82FFEB44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFEB48: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FFEB4C: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82FFEB50: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFEB54: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82FFEB58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEB64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFEB68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFEB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFEB70 size=8
    let mut pc: u32 = 0x82FFEB70;
    'dispatch: loop {
        match pc {
            0x82FFEB70 => {
    //   block [0x82FFEB70..0x82FFEB78)
	// 82FFEB70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFEB74: 8213FF08  lwz r16, -0xf8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-248 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEB78 size=120
    let mut pc: u32 = 0x82FFEB78;
    'dispatch: loop {
        match pc {
            0x82FFEB78 => {
    //   block [0x82FFEB78..0x82FFEBF0)
	// 82FFEB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEB7C: 481A95F1  bl 0x831a816c
	ctx.lr = 0x82FFEB80;
	sub_831A8130(ctx, base);
	// 82FFEB80: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FFEB84: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEB88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFEB8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FFEB90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEB94: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FFEB98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFEB9C: 4E800421  bctrl
	ctx.lr = 0x82FFEBA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFEBA0: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82FFEBA4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FFEBA8: 4BFE2E49  bl 0x82fe19f0
	ctx.lr = 0x82FFEBAC;
	sub_82FE19F0(ctx, base);
	// 82FFEBAC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FFEBB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFEBB4: 41820018  beq 0x82ffebcc
	if ctx.cr[0].eq {
	pc = 0x82FFEBCC; continue 'dispatch;
	}
	// 82FFEBB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FFEBBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFEBC0: 4BFFFF59  bl 0x82ffeb18
	ctx.lr = 0x82FFEBC4;
	sub_82FFEB18(ctx, base);
	// 82FFEBC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFEBC8: 48000008  b 0x82ffebd0
	pc = 0x82FFEBD0; continue 'dispatch;
	// 82FFEBCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FFEBD0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FFEBD4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FFEBD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FFEBDC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82FFEBE0: 4BFE0B99  bl 0x82fdf778
	ctx.lr = 0x82FFEBE4;
	sub_82FDF778(ctx, base);
	// 82FFEBE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFEBE8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FFEBEC: 481A95D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEBF0 size=44
    let mut pc: u32 = 0x82FFEBF0;
    'dispatch: loop {
        match pc {
            0x82FFEBF0 => {
    //   block [0x82FFEBF0..0x82FFEC1C)
	// 82FFEBF0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FFEBF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEBF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEBFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEC00: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFEC04: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFEC08: 480C8ED9  bl 0x830c7ae0
	ctx.lr = 0x82FFEC0C;
	sub_830C7AE0(ctx, base);
	// 82FFEC0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFEC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEC20 size=76
    let mut pc: u32 = 0x82FFEC20;
    'dispatch: loop {
        match pc {
            0x82FFEC20 => {
    //   block [0x82FFEC20..0x82FFEC6C)
	// 82FFEC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFEC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFEC30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEC34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFEC38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFEC3C: 48007C85  bl 0x830068c0
	ctx.lr = 0x82FFEC40;
	sub_830068C0(ctx, base);
	// 82FFEC40: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEC44: 4182000C  beq 0x82ffec50
	if ctx.cr[0].eq {
	pc = 0x82FFEC50; continue 'dispatch;
	}
	// 82FFEC48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEC4C: 4B2C161D  bl 0x822c0268
	ctx.lr = 0x82FFEC50;
	sub_822C0268(ctx, base);
	// 82FFEC50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFEC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFEC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFEC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFEC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFEC70 size=44
    let mut pc: u32 = 0x82FFEC70;
    'dispatch: loop {
        match pc {
            0x82FFEC70 => {
    //   block [0x82FFEC70..0x82FFEC9C)
	// 82FFEC70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEC74: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82FFEC78: 39202596  li r9, 0x2596
	ctx.r[9].s64 = 9622;
	// 82FFEC7C: 394B01A4  addi r10, r11, 0x1a4
	ctx.r[10].s64 = ctx.r[11].s64 + 420;
	// 82FFEC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFEC84: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82FFEC88: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FFEC8C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFEC90: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FFEC94: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFEC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFECA0 size=68
    let mut pc: u32 = 0x82FFECA0;
    'dispatch: loop {
        match pc {
            0x82FFECA0 => {
    //   block [0x82FFECA0..0x82FFECE4)
	// 82FFECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFECA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFECA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFECAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFECB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFECB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFECB8: 396B0194  addi r11, r11, 0x194
	ctx.r[11].s64 = ctx.r[11].s64 + 404;
	// 82FFECBC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFECC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFECC4: 41820008  beq 0x82ffeccc
	if ctx.cr[0].eq {
	pc = 0x82FFECCC; continue 'dispatch;
	}
	// 82FFECC8: 4B2C15A1  bl 0x822c0268
	ctx.lr = 0x82FFECCC;
	sub_822C0268(ctx, base);
	// 82FFECCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFECD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFECD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFECD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFECDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFECE8 size=8
    let mut pc: u32 = 0x82FFECE8;
    'dispatch: loop {
        match pc {
            0x82FFECE8 => {
    //   block [0x82FFECE8..0x82FFECF0)
	// 82FFECE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFECEC: 821401C0  lwz r16, 0x1c0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFECF0 size=556
    let mut pc: u32 = 0x82FFECF0;
    'dispatch: loop {
        match pc {
            0x82FFECF0 => {
    //   block [0x82FFECF0..0x82FFEF1C)
	// 82FFECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFECF4: 481A9479  bl 0x831a816c
	ctx.lr = 0x82FFECF8;
	sub_831A8130(ctx, base);
	// 82FFECF8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FFECFC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFED00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFED04: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFED08: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFED0C: 4BFD1E75  bl 0x82fd0b80
	ctx.lr = 0x82FFED10;
	sub_82FD0B80(ctx, base);
	// 82FFED10: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFED14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFED18: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FFED1C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FFED20: 4BFD3349  bl 0x82fd2068
	ctx.lr = 0x82FFED24;
	sub_82FD2068(ctx, base);
	// 82FFED24: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFED28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFED2C: 388BFF4C  addi r4, r11, -0xb4
	ctx.r[4].s64 = ctx.r[11].s64 + -180;
	// 82FFED30: 4BFD4F11  bl 0x82fd3c40
	ctx.lr = 0x82FFED34;
	sub_82FD3C40(ctx, base);
	// 82FFED34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFED38: 41820020  beq 0x82ffed58
	if ctx.cr[0].eq {
	pc = 0x82FFED58; continue 'dispatch;
	}
	// 82FFED3C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FFED40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFED44: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFED48: 4BFD3D79  bl 0x82fd2ac0
	ctx.lr = 0x82FFED4C;
	sub_82FD2AC0(ctx, base);
	// 82FFED4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFED50: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FFED54: 481A9468  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FFED58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFED5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFED60: 388BFF6C  addi r4, r11, -0x94
	ctx.r[4].s64 = ctx.r[11].s64 + -148;
	// 82FFED64: 4BFD4EDD  bl 0x82fd3c40
	ctx.lr = 0x82FFED68;
	sub_82FD3C40(ctx, base);
	// 82FFED68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFED6C: 4182000C  beq 0x82ffed78
	if ctx.cr[0].eq {
	pc = 0x82FFED78; continue 'dispatch;
	}
	// 82FFED70: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 82FFED74: 4BFFFFCC  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFED78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFED7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFED80: 388BFF8C  addi r4, r11, -0x74
	ctx.r[4].s64 = ctx.r[11].s64 + -116;
	// 82FFED84: 4BFD4EBD  bl 0x82fd3c40
	ctx.lr = 0x82FFED88;
	sub_82FD3C40(ctx, base);
	// 82FFED88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFED8C: 4182000C  beq 0x82ffed98
	if ctx.cr[0].eq {
	pc = 0x82FFED98; continue 'dispatch;
	}
	// 82FFED90: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82FFED94: 4BFFFFAC  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFED98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFED9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEDA0: 388BFFA0  addi r4, r11, -0x60
	ctx.r[4].s64 = ctx.r[11].s64 + -96;
	// 82FFEDA4: 4BFD4E9D  bl 0x82fd3c40
	ctx.lr = 0x82FFEDA8;
	sub_82FD3C40(ctx, base);
	// 82FFEDA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEDAC: 4182000C  beq 0x82ffedb8
	if ctx.cr[0].eq {
	pc = 0x82FFEDB8; continue 'dispatch;
	}
	// 82FFEDB0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82FFEDB4: 4BFFFF8C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEDB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEDBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEDC0: 388BFFD0  addi r4, r11, -0x30
	ctx.r[4].s64 = ctx.r[11].s64 + -48;
	// 82FFEDC4: 4BFD4E7D  bl 0x82fd3c40
	ctx.lr = 0x82FFEDC8;
	sub_82FD3C40(ctx, base);
	// 82FFEDC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEDCC: 4182000C  beq 0x82ffedd8
	if ctx.cr[0].eq {
	pc = 0x82FFEDD8; continue 'dispatch;
	}
	// 82FFEDD0: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82FFEDD4: 4BFFFF6C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEDD8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEDDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEDE0: 388B0000  addi r4, r11, 0
	ctx.r[4].s64 = ctx.r[11].s64 + 0;
	// 82FFEDE4: 4BFD4E5D  bl 0x82fd3c40
	ctx.lr = 0x82FFEDE8;
	sub_82FD3C40(ctx, base);
	// 82FFEDE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEDEC: 4182000C  beq 0x82ffedf8
	if ctx.cr[0].eq {
	pc = 0x82FFEDF8; continue 'dispatch;
	}
	// 82FFEDF0: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 82FFEDF4: 4BFFFF4C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEDF8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEDFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE00: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82FFEE04: 4BFD4E3D  bl 0x82fd3c40
	ctx.lr = 0x82FFEE08;
	sub_82FD3C40(ctx, base);
	// 82FFEE08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEE0C: 4182000C  beq 0x82ffee18
	if ctx.cr[0].eq {
	pc = 0x82FFEE18; continue 'dispatch;
	}
	// 82FFEE10: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 82FFEE14: 4BFFFF2C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEE18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEE1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE20: 388B0024  addi r4, r11, 0x24
	ctx.r[4].s64 = ctx.r[11].s64 + 36;
	// 82FFEE24: 4BFD4E1D  bl 0x82fd3c40
	ctx.lr = 0x82FFEE28;
	sub_82FD3C40(ctx, base);
	// 82FFEE28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEE2C: 4182000C  beq 0x82ffee38
	if ctx.cr[0].eq {
	pc = 0x82FFEE38; continue 'dispatch;
	}
	// 82FFEE30: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 82FFEE34: 4BFFFF0C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEE38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEE3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE40: 388B003C  addi r4, r11, 0x3c
	ctx.r[4].s64 = ctx.r[11].s64 + 60;
	// 82FFEE44: 4BFD4DFD  bl 0x82fd3c40
	ctx.lr = 0x82FFEE48;
	sub_82FD3C40(ctx, base);
	// 82FFEE48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEE4C: 4182000C  beq 0x82ffee58
	if ctx.cr[0].eq {
	pc = 0x82FFEE58; continue 'dispatch;
	}
	// 82FFEE50: 3BC00100  li r30, 0x100
	ctx.r[30].s64 = 256;
	// 82FFEE54: 4BFFFEEC  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEE58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE60: 388B006C  addi r4, r11, 0x6c
	ctx.r[4].s64 = ctx.r[11].s64 + 108;
	// 82FFEE64: 4BFD4DDD  bl 0x82fd3c40
	ctx.lr = 0x82FFEE68;
	sub_82FD3C40(ctx, base);
	// 82FFEE68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEE6C: 4182000C  beq 0x82ffee78
	if ctx.cr[0].eq {
	pc = 0x82FFEE78; continue 'dispatch;
	}
	// 82FFEE70: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 82FFEE74: 4BFFFECC  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEE78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEE7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEE80: 388B0098  addi r4, r11, 0x98
	ctx.r[4].s64 = ctx.r[11].s64 + 152;
	// 82FFEE84: 4BFD4DBD  bl 0x82fd3c40
	ctx.lr = 0x82FFEE88;
	sub_82FD3C40(ctx, base);
	// 82FFEE88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEE8C: 4182000C  beq 0x82ffee98
	if ctx.cr[0].eq {
	pc = 0x82FFEE98; continue 'dispatch;
	}
	// 82FFEE90: 3BC00400  li r30, 0x400
	ctx.r[30].s64 = 1024;
	// 82FFEE94: 4BFFFEAC  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEE98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEE9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEEA0: 388B00C4  addi r4, r11, 0xc4
	ctx.r[4].s64 = ctx.r[11].s64 + 196;
	// 82FFEEA4: 4BFD4D9D  bl 0x82fd3c40
	ctx.lr = 0x82FFEEA8;
	sub_82FD3C40(ctx, base);
	// 82FFEEA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEEAC: 4182000C  beq 0x82ffeeb8
	if ctx.cr[0].eq {
	pc = 0x82FFEEB8; continue 'dispatch;
	}
	// 82FFEEB0: 3BC00800  li r30, 0x800
	ctx.r[30].s64 = 2048;
	// 82FFEEB4: 4BFFFE8C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEEB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEEBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEEC0: 388B00D8  addi r4, r11, 0xd8
	ctx.r[4].s64 = ctx.r[11].s64 + 216;
	// 82FFEEC4: 4BFD4D7D  bl 0x82fd3c40
	ctx.lr = 0x82FFEEC8;
	sub_82FD3C40(ctx, base);
	// 82FFEEC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEECC: 4182000C  beq 0x82ffeed8
	if ctx.cr[0].eq {
	pc = 0x82FFEED8; continue 'dispatch;
	}
	// 82FFEED0: 3BC01000  li r30, 0x1000
	ctx.r[30].s64 = 4096;
	// 82FFEED4: 4BFFFE6C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEED8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFEEDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEEE0: 388B0100  addi r4, r11, 0x100
	ctx.r[4].s64 = ctx.r[11].s64 + 256;
	// 82FFEEE4: 4BFD4D5D  bl 0x82fd3c40
	ctx.lr = 0x82FFEEE8;
	sub_82FD3C40(ctx, base);
	// 82FFEEE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEEEC: 4182000C  beq 0x82ffeef8
	if ctx.cr[0].eq {
	pc = 0x82FFEEF8; continue 'dispatch;
	}
	// 82FFEEF0: 3BC02000  li r30, 0x2000
	ctx.r[30].s64 = 8192;
	// 82FFEEF4: 4BFFFE4C  b 0x82ffed40
	pc = 0x82FFED40; continue 'dispatch;
	// 82FFEEF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFEEFC: 80DD0014  lwz r6, 0x14(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFEF00: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FFEF04: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFEF08: 4BFFAFC9  bl 0x82ff9ed0
	ctx.lr = 0x82FFEF0C;
	sub_82FF9ED0(ctx, base);
	// 82FFEF0C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFEF10: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFEF14: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFEF18: 481B1D11  bl 0x831b0c28
	ctx.lr = 0x82FFEF1C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEF1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEF1C size=40
    let mut pc: u32 = 0x82FFEF1C;
    'dispatch: loop {
        match pc {
            0x82FFEF1C => {
    //   block [0x82FFEF1C..0x82FFEF44)
	// 82FFEF1C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FFEF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFEF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEF2C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FFEF30: 4BFD3F29  bl 0x82fd2e58
	ctx.lr = 0x82FFEF34;
	sub_82FD2E58(ctx, base);
	// 82FFEF34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFEF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFEF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFEF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFEF48 size=8
    let mut pc: u32 = 0x82FFEF48;
    'dispatch: loop {
        match pc {
            0x82FFEF48 => {
    //   block [0x82FFEF48..0x82FFEF50)
	// 82FFEF48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFEF4C: 8214024C  lwz r16, 0x24c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(588 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFEF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFEF50 size=384
    let mut pc: u32 = 0x82FFEF50;
    'dispatch: loop {
        match pc {
            0x82FFEF50 => {
    //   block [0x82FFEF50..0x82FFF0D0)
	// 82FFEF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFEF54: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FFEF58: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FFEF5C: 481A9209  bl 0x831a8164
	ctx.lr = 0x82FFEF60;
	sub_831A8130(ctx, base);
	// 82FFEF60: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 82FFEF64: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFEF68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFEF6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FFEF70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFEF74: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFEF78: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 82FFEF7C: 939F0114  stw r28, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[28].u32 ) };
	// 82FFEF80: 4BFD1C01  bl 0x82fd0b80
	ctx.lr = 0x82FFEF84;
	sub_82FD0B80(ctx, base);
	// 82FFEF84: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFEF88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFEF8C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FFEF90: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82FFEF94: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FFEF98: 4BFD30D1  bl 0x82fd2068
	ctx.lr = 0x82FFEF9C;
	sub_82FD2068(ctx, base);
	// 82FFEF9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFEFA0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FFEFA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFEFA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEFAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFEFB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFEFB4: 4E800421  bctrl
	ctx.lr = 0x82FFEFB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFEFB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFEFBC: 40820028  bne 0x82ffefe4
	if !ctx.cr[0].eq {
	pc = 0x82FFEFE4; continue 'dispatch;
	}
	// 82FFEFC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFEFC4: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFEFC8: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FFEFCC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFEFD0: 4BFFAF01  bl 0x82ff9ed0
	ctx.lr = 0x82FFEFD4;
	sub_82FF9ED0(ctx, base);
	// 82FFEFD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFEFD8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFEFDC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFEFE0: 481B1C49  bl 0x831b0c28
	ctx.lr = 0x82FFEFE4;
	sub_831B0C28(ctx, base);
	// 82FFEFE4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FFEFE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFEFEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFEFF0: 4BFFFD01  bl 0x82ffecf0
	ctx.lr = 0x82FFEFF4;
	sub_82FFECF0(ctx, base);
	// 82FFEFF4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FFEFF8: 48000018  b 0x82fff010
	pc = 0x82FFF010; continue 'dispatch;
	// 82FFEFFC: 83DF0104  lwz r30, 0x104(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 82FFF000: 839F0114  lwz r28, 0x114(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 82FFF004: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF008: 8B7F0050  lbz r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFF00C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF010: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF014: 41820028  beq 0x82fff03c
	if ctx.cr[0].eq {
	pc = 0x82FFF03C; continue 'dispatch;
	}
	// 82FFF018: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF01C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF020: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF024: 4182000C  beq 0x82fff030
	if ctx.cr[0].eq {
	pc = 0x82FFF030; continue 'dispatch;
	}
	// 82FFF028: 7D6B1B78  or r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[3].u64;
	// 82FFF02C: 48000008  b 0x82fff034
	pc = 0x82FFF034; continue 'dispatch;
	// 82FFF030: 7D6B1878  andc r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[3].u64;
	// 82FFF034: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FFF038: 48000060  b 0x82fff098
	pc = 0x82FFF098; continue 'dispatch;
	// 82FFF03C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF040: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF044: 388B013C  addi r4, r11, 0x13c
	ctx.r[4].s64 = ctx.r[11].s64 + 316;
	// 82FFF048: 4BFD4BF9  bl 0x82fd3c40
	ctx.lr = 0x82FFF04C;
	sub_82FD3C40(ctx, base);
	// 82FFF04C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF050: 4182000C  beq 0x82fff05c
	if ctx.cr[0].eq {
	pc = 0x82FFF05C; continue 'dispatch;
	}
	// 82FFF054: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FFF058: 48000040  b 0x82fff098
	pc = 0x82FFF098; continue 'dispatch;
	// 82FFF05C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF064: 388B0158  addi r4, r11, 0x158
	ctx.r[4].s64 = ctx.r[11].s64 + 344;
	// 82FFF068: 4BFD4BD9  bl 0x82fd3c40
	ctx.lr = 0x82FFF06C;
	sub_82FD3C40(ctx, base);
	// 82FFF06C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF070: 4182000C  beq 0x82fff07c
	if ctx.cr[0].eq {
	pc = 0x82FFF07C; continue 'dispatch;
	}
	// 82FFF074: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FFF078: 48000020  b 0x82fff098
	pc = 0x82FFF098; continue 'dispatch;
	// 82FFF07C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF080: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF084: 388B0170  addi r4, r11, 0x170
	ctx.r[4].s64 = ctx.r[11].s64 + 368;
	// 82FFF088: 4BFD4BB9  bl 0x82fd3c40
	ctx.lr = 0x82FFF08C;
	sub_82FD3C40(ctx, base);
	// 82FFF08C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF090: 4182001C  beq 0x82fff0ac
	if ctx.cr[0].eq {
	pc = 0x82FFF0AC; continue 'dispatch;
	}
	// 82FFF094: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82FFF098: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFF09C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF0A0: 4BFD3A21  bl 0x82fd2ac0
	ctx.lr = 0x82FFF0A4;
	sub_82FD2AC0(ctx, base);
	// 82FFF0A4: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 82FFF0A8: 481A910C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FFF0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF0B0: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF0B4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FFF0B8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFF0BC: 4BFFAE15  bl 0x82ff9ed0
	ctx.lr = 0x82FFF0C0;
	sub_82FF9ED0(ctx, base);
	// 82FFF0C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFF0C4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFF0C8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFF0CC: 481B1B5D  bl 0x831b0c28
	ctx.lr = 0x82FFF0D0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF0D0 size=8
    let mut pc: u32 = 0x82FFF0D0;
    'dispatch: loop {
        match pc {
            0x82FFF0D0 => {
    //   block [0x82FFF0D0..0x82FFF0D8)
	// 82FFF0D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFF0D4: 8214024C  lwz r16, 0x24c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(588 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF0D8 size=56
    let mut pc: u32 = 0x82FFF0D8;
    'dispatch: loop {
        match pc {
            0x82FFF0D8 => {
    //   block [0x82FFF0D8..0x82FFF110)
	// 82FFF0D8: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 82FFF0DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF0E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF0EC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FFF0F0: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82FFF0F4: 4BFFAD1D  bl 0x82ff9e10
	ctx.lr = 0x82FFF0F8;
	sub_82FF9E10(ctx, base);
	// 82FFF0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF0FC: 3C608300  lis r3, -0x7d00
	ctx.r[3].s64 = -2097152000;
	// 82FFF100: 3863EFFC  addi r3, r3, -0x1004
	ctx.r[3].s64 = ctx.r[3].s64 + -4100;
	// 82FFF104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF10C: 481A90A8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF110 size=40
    let mut pc: u32 = 0x82FFF110;
    'dispatch: loop {
        match pc {
            0x82FFF110 => {
    //   block [0x82FFF110..0x82FFF138)
	// 82FFF110: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 82FFF114: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF118: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF11C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF120: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF124: 4BFD3D35  bl 0x82fd2e58
	ctx.lr = 0x82FFF128;
	sub_82FD2E58(ctx, base);
	// 82FFF128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF138 size=8
    let mut pc: u32 = 0x82FFF138;
    'dispatch: loop {
        match pc {
            0x82FFF138 => {
    //   block [0x82FFF138..0x82FFF140)
	// 82FFF138: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFF13C: 821402FC  lwz r16, 0x2fc(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(764 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF140 size=304
    let mut pc: u32 = 0x82FFF140;
    'dispatch: loop {
        match pc {
            0x82FFF140 => {
    //   block [0x82FFF140..0x82FFF270)
	// 82FFF140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF144: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FFF148: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FFF14C: 481A901D  bl 0x831a8168
	ctx.lr = 0x82FFF150;
	sub_831A8130(ctx, base);
	// 82FFF150: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FFF154: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF158: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFF15C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFF160: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF164: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 82FFF168: 4BFD1A19  bl 0x82fd0b80
	ctx.lr = 0x82FFF16C;
	sub_82FD0B80(ctx, base);
	// 82FFF16C: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF170: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFF174: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82FFF178: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82FFF17C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FFF180: 4BFD2EE9  bl 0x82fd2068
	ctx.lr = 0x82FFF184;
	sub_82FD2068(ctx, base);
	// 82FFF184: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82FFF188: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFF18C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF190: 4BFFFB61  bl 0x82ffecf0
	ctx.lr = 0x82FFF194;
	sub_82FFECF0(ctx, base);
	// 82FFF194: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FFF198: 48000014  b 0x82fff1ac
	pc = 0x82FFF1AC; continue 'dispatch;
	// 82FFF19C: 83BF00D4  lwz r29, 0xd4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FFF1A0: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF1A4: 8B9F0050  lbz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFF1A8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF1AC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF1B0: 4182003C  beq 0x82fff1ec
	if ctx.cr[0].eq {
	pc = 0x82FFF1EC; continue 'dispatch;
	}
	// 82FFF1B4: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF1B8: 7D6B1839  and. r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF1BC: 41820024  beq 0x82fff1e0
	if ctx.cr[0].eq {
	pc = 0x82FFF1E0; continue 'dispatch;
	}
	// 82FFF1C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF1C4: 3BCBFF49  addi r30, r11, -0xb7
	ctx.r[30].s64 = ctx.r[11].s64 + -183;
	// 82FFF1C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFF1CC: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF1D0: 4BFD38F1  bl 0x82fd2ac0
	ctx.lr = 0x82FFF1D4;
	sub_82FD2AC0(ctx, base);
	// 82FFF1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF1D8: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FFF1DC: 481A8FDC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FFF1E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF1E4: 3BCBFF48  addi r30, r11, -0xb8
	ctx.r[30].s64 = ctx.r[11].s64 + -184;
	// 82FFF1E8: 4BFFFFE0  b 0x82fff1c8
	pc = 0x82FFF1C8; continue 'dispatch;
	// 82FFF1EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF1F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF1F4: 388B013C  addi r4, r11, 0x13c
	ctx.r[4].s64 = ctx.r[11].s64 + 316;
	// 82FFF1F8: 4BFD4A49  bl 0x82fd3c40
	ctx.lr = 0x82FFF1FC;
	sub_82FD3C40(ctx, base);
	// 82FFF1FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF200: 4182000C  beq 0x82fff20c
	if ctx.cr[0].eq {
	pc = 0x82FFF20C; continue 'dispatch;
	}
	// 82FFF204: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF208: 4BFFFFC0  b 0x82fff1c8
	pc = 0x82FFF1C8; continue 'dispatch;
	// 82FFF20C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF210: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF214: 388B0158  addi r4, r11, 0x158
	ctx.r[4].s64 = ctx.r[11].s64 + 344;
	// 82FFF218: 4BFD4A29  bl 0x82fd3c40
	ctx.lr = 0x82FFF21C;
	sub_82FD3C40(ctx, base);
	// 82FFF21C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF220: 4182000C  beq 0x82fff22c
	if ctx.cr[0].eq {
	pc = 0x82FFF22C; continue 'dispatch;
	}
	// 82FFF224: 83DD000C  lwz r30, 0xc(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFF228: 4BFFFFA0  b 0x82fff1c8
	pc = 0x82FFF1C8; continue 'dispatch;
	// 82FFF22C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF234: 388B0170  addi r4, r11, 0x170
	ctx.r[4].s64 = ctx.r[11].s64 + 368;
	// 82FFF238: 4BFD4A09  bl 0x82fd3c40
	ctx.lr = 0x82FFF23C;
	sub_82FD3C40(ctx, base);
	// 82FFF23C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF240: 4182000C  beq 0x82fff24c
	if ctx.cr[0].eq {
	pc = 0x82FFF24C; continue 'dispatch;
	}
	// 82FFF244: 83DD0010  lwz r30, 0x10(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF248: 4BFFFF80  b 0x82fff1c8
	pc = 0x82FFF1C8; continue 'dispatch;
	// 82FFF24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF250: 80DD0014  lwz r6, 0x14(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF254: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FFF258: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFF25C: 4BFFAC75  bl 0x82ff9ed0
	ctx.lr = 0x82FFF260;
	sub_82FF9ED0(ctx, base);
	// 82FFF260: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFF264: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFF268: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFF26C: 481B19BD  bl 0x831b0c28
	ctx.lr = 0x82FFF270;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF270 size=8
    let mut pc: u32 = 0x82FFF270;
    'dispatch: loop {
        match pc {
            0x82FFF270 => {
    //   block [0x82FFF270..0x82FFF278)
	// 82FFF270: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFF274: 821402FC  lwz r16, 0x2fc(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(764 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF278 size=56
    let mut pc: u32 = 0x82FFF278;
    'dispatch: loop {
        match pc {
            0x82FFF278 => {
    //   block [0x82FFF278..0x82FFF2B0)
	// 82FFF278: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FFF27C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF280: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF284: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF28C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FFF290: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82FFF294: 4BFFAB7D  bl 0x82ff9e10
	ctx.lr = 0x82FFF298;
	sub_82FF9E10(ctx, base);
	// 82FFF298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF29C: 3C608300  lis r3, -0x7d00
	ctx.r[3].s64 = -2097152000;
	// 82FFF2A0: 3863F19C  addi r3, r3, -0xe64
	ctx.r[3].s64 = ctx.r[3].s64 + -3684;
	// 82FFF2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF2AC: 481A8F0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF2B0 size=40
    let mut pc: u32 = 0x82FFF2B0;
    'dispatch: loop {
        match pc {
            0x82FFF2B0 => {
    //   block [0x82FFF2B0..0x82FFF2D8)
	// 82FFF2B0: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FFF2B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF2B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF2BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF2C0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF2C4: 4BFD3B95  bl 0x82fd2e58
	ctx.lr = 0x82FFF2C8;
	sub_82FD2E58(ctx, base);
	// 82FFF2C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF2D8 size=8
    let mut pc: u32 = 0x82FFF2D8;
    'dispatch: loop {
        match pc {
            0x82FFF2D8 => {
    //   block [0x82FFF2D8..0x82FFF2E0)
	// 82FFF2D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFF2DC: 821403AC  lwz r16, 0x3ac(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(940 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF2E0 size=584
    let mut pc: u32 = 0x82FFF2E0;
    'dispatch: loop {
        match pc {
            0x82FFF2E0 => {
    //   block [0x82FFF2E0..0x82FFF528)
	// 82FFF2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF2E4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FFF2E8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FFF2EC: 481A8E71  bl 0x831a815c
	ctx.lr = 0x82FFF2F0;
	sub_831A8130(ctx, base);
	// 82FFF2F0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FFF2F4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF2F8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FFF2FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FFF300: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FFF304: 409A0010  bne cr6, 0x82fff314
	if !ctx.cr[6].eq {
	pc = 0x82FFF314; continue 'dispatch;
	}
	// 82FFF308: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FFF30C: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FFF310: 481A8E9C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82FFF314: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FFF318: 809C0014  lwz r4, 0x14(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF31C: 4BFD1865  bl 0x82fd0b80
	ctx.lr = 0x82FFF320;
	sub_82FD0B80(ctx, base);
	// 82FFF320: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FFF324: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FFF328: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FFF32C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FFF330: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82FFF334: 4BFD2D35  bl 0x82fd2068
	ctx.lr = 0x82FFF338;
	sub_82FD2068(ctx, base);
	// 82FFF338: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFF33C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82FFF340: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82FFF344: 9BDF0051  stb r30, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[30].u8 ) };
	// 82FFF348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FFF34C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FFF350: 4BFFF9A1  bl 0x82ffecf0
	ctx.lr = 0x82FFF354;
	sub_82FFECF0(ctx, base);
	// 82FFF354: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FFF358: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FFF35C: 895A0000  lbz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF360: 4800001C  b 0x82fff37c
	pc = 0x82FFF37C; continue 'dispatch;
	// 82FFF364: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF368: 8B7F0050  lbz r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FFF36C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82FFF370: 895F0051  lbz r10, 0x51(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(81 as u32) ) } as u64;
	// 82FFF374: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FFF378: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FFF37C: 5769063F  clrlwi. r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFF380: 41820150  beq 0x82fff4d0
	if ctx.cr[0].eq {
	pc = 0x82FFF4D0; continue 'dispatch;
	}
	// 82FFF384: 2F0B0080  cmpwi cr6, r11, 0x80
	ctx.cr[6].compare_i32(ctx.r[11].s32, 128, &mut ctx.xer);
	// 82FFF388: 419900B4  bgt cr6, 0x82fff43c
	if ctx.cr[6].gt {
	pc = 0x82FFF43C; continue 'dispatch;
	}
	// 82FFF38C: 419A00AC  beq cr6, 0x82fff438
	if ctx.cr[6].eq {
	pc = 0x82FFF438; continue 'dispatch;
	}
	// 82FFF390: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFF394: 419A0080  beq cr6, 0x82fff414
	if ctx.cr[6].eq {
	pc = 0x82FFF414; continue 'dispatch;
	}
	// 82FFF398: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FFF39C: 419A0070  beq cr6, 0x82fff40c
	if ctx.cr[6].eq {
	pc = 0x82FFF40C; continue 'dispatch;
	}
	// 82FFF3A0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82FFF3A4: 419A0064  beq cr6, 0x82fff408
	if ctx.cr[6].eq {
	pc = 0x82FFF408; continue 'dispatch;
	}
	// 82FFF3A8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82FFF3AC: 419A004C  beq cr6, 0x82fff3f8
	if ctx.cr[6].eq {
	pc = 0x82FFF3F8; continue 'dispatch;
	}
	// 82FFF3B0: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82FFF3B4: 419A0034  beq cr6, 0x82fff3e8
	if ctx.cr[6].eq {
	pc = 0x82FFF3E8; continue 'dispatch;
	}
	// 82FFF3B8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 82FFF3BC: 419A001C  beq cr6, 0x82fff3d8
	if ctx.cr[6].eq {
	pc = 0x82FFF3D8; continue 'dispatch;
	}
	// 82FFF3C0: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 82FFF3C4: 409A00A8  bne cr6, 0x82fff46c
	if !ctx.cr[6].eq {
	pc = 0x82FFF46C; continue 'dispatch;
	}
	// 82FFF3C8: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF3CC: 41820008  beq 0x82fff3d4
	if ctx.cr[0].eq {
	pc = 0x82FFF3D4; continue 'dispatch;
	}
	// 82FFF3D0: 4800004C  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF3D4: 48000038  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF3D8: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF3DC: 41820008  beq 0x82fff3e4
	if ctx.cr[0].eq {
	pc = 0x82FFF3E4; continue 'dispatch;
	}
	// 82FFF3E0: 4800002C  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF3E4: 48000028  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF3E8: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF3EC: 41820008  beq 0x82fff3f4
	if ctx.cr[0].eq {
	pc = 0x82FFF3F4; continue 'dispatch;
	}
	// 82FFF3F0: 4800002C  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF3F4: 48000018  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF3F8: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF3FC: 41820008  beq 0x82fff404
	if ctx.cr[0].eq {
	pc = 0x82FFF404; continue 'dispatch;
	}
	// 82FFF400: 4800001C  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF404: 48000008  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF408: 48000004  b 0x82fff40c
	pc = 0x82FFF40C; continue 'dispatch;
	// 82FFF40C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82FFF410: 4800000C  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF414: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF418: 41820018  beq 0x82fff430
	if ctx.cr[0].eq {
	pc = 0x82FFF430; continue 'dispatch;
	}
	// 82FFF41C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FFF420: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF424: 4BFD369D  bl 0x82fd2ac0
	ctx.lr = 0x82FFF428;
	sub_82FD2AC0(ctx, base);
	// 82FFF428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF42C: 4BFFFEE0  b 0x82fff30c
	pc = 0x82FFF30C; continue 'dispatch;
	// 82FFF430: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82FFF434: 4BFFFFE8  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF438: 4BFFFFF8  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF43C: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82FFF440: 419A0080  beq cr6, 0x82fff4c0
	if ctx.cr[6].eq {
	pc = 0x82FFF4C0; continue 'dispatch;
	}
	// 82FFF444: 2F0B0200  cmpwi cr6, r11, 0x200
	ctx.cr[6].compare_i32(ctx.r[11].s32, 512, &mut ctx.xer);
	// 82FFF448: 419A0068  beq cr6, 0x82fff4b0
	if ctx.cr[6].eq {
	pc = 0x82FFF4B0; continue 'dispatch;
	}
	// 82FFF44C: 2F0B0400  cmpwi cr6, r11, 0x400
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1024, &mut ctx.xer);
	// 82FFF450: 419A0050  beq cr6, 0x82fff4a0
	if ctx.cr[6].eq {
	pc = 0x82FFF4A0; continue 'dispatch;
	}
	// 82FFF454: 2F0B0800  cmpwi cr6, r11, 0x800
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2048, &mut ctx.xer);
	// 82FFF458: 419A0038  beq cr6, 0x82fff490
	if ctx.cr[6].eq {
	pc = 0x82FFF490; continue 'dispatch;
	}
	// 82FFF45C: 2F0B1000  cmpwi cr6, r11, 0x1000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4096, &mut ctx.xer);
	// 82FFF460: 419A0020  beq cr6, 0x82fff480
	if ctx.cr[6].eq {
	pc = 0x82FFF480; continue 'dispatch;
	}
	// 82FFF464: 2F0B2000  cmpwi cr6, r11, 0x2000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8192, &mut ctx.xer);
	// 82FFF468: 419A0008  beq cr6, 0x82fff470
	if ctx.cr[6].eq {
	pc = 0x82FFF470; continue 'dispatch;
	}
	// 82FFF46C: 4BFFFFB0  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF470: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF474: 41820008  beq 0x82fff47c
	if ctx.cr[0].eq {
	pc = 0x82FFF47C; continue 'dispatch;
	}
	// 82FFF478: 4BFFFFB8  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF47C: 4BFFFFA0  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF480: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF484: 41820008  beq 0x82fff48c
	if ctx.cr[0].eq {
	pc = 0x82FFF48C; continue 'dispatch;
	}
	// 82FFF488: 4BFFFF94  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF48C: 4BFFFFA4  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF490: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF494: 41820008  beq 0x82fff49c
	if ctx.cr[0].eq {
	pc = 0x82FFF49C; continue 'dispatch;
	}
	// 82FFF498: 4BFFFF84  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF49C: 4BFFFF94  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF4A0: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF4A4: 41820008  beq 0x82fff4ac
	if ctx.cr[0].eq {
	pc = 0x82FFF4AC; continue 'dispatch;
	}
	// 82FFF4A8: 4BFFFF74  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF4AC: 4BFFFF70  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF4B0: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF4B4: 41820008  beq 0x82fff4bc
	if ctx.cr[0].eq {
	pc = 0x82FFF4BC; continue 'dispatch;
	}
	// 82FFF4B8: 4BFFFF64  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF4BC: 4BFFFF74  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF4C0: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF4C4: 41820008  beq 0x82fff4cc
	if ctx.cr[0].eq {
	pc = 0x82FFF4CC; continue 'dispatch;
	}
	// 82FFF4C8: 4BFFFF68  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF4CC: 4BFFFF50  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF4D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF4D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF4D8: 388B013C  addi r4, r11, 0x13c
	ctx.r[4].s64 = ctx.r[11].s64 + 316;
	// 82FFF4DC: 4BFD4765  bl 0x82fd3c40
	ctx.lr = 0x82FFF4E0;
	sub_82FD3C40(ctx, base);
	// 82FFF4E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF4E4: 41820008  beq 0x82fff4ec
	if ctx.cr[0].eq {
	pc = 0x82FFF4EC; continue 'dispatch;
	}
	// 82FFF4E8: 4BFFFF48  b 0x82fff430
	pc = 0x82FFF430; continue 'dispatch;
	// 82FFF4EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF4F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF4F4: 388B0158  addi r4, r11, 0x158
	ctx.r[4].s64 = ctx.r[11].s64 + 344;
	// 82FFF4F8: 4BFD4749  bl 0x82fd3c40
	ctx.lr = 0x82FFF4FC;
	sub_82FD3C40(ctx, base);
	// 82FFF4FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF500: 41820008  beq 0x82fff508
	if ctx.cr[0].eq {
	pc = 0x82FFF508; continue 'dispatch;
	}
	// 82FFF504: 4BFFFF18  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF508: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF50C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FFF510: 388B0170  addi r4, r11, 0x170
	ctx.r[4].s64 = ctx.r[11].s64 + 368;
	// 82FFF514: 4BFD472D  bl 0x82fd3c40
	ctx.lr = 0x82FFF518;
	sub_82FD3C40(ctx, base);
	// 82FFF518: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF51C: 41820008  beq 0x82fff524
	if ctx.cr[0].eq {
	pc = 0x82FFF524; continue 'dispatch;
	}
	// 82FFF520: 4BFFFEFC  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
	// 82FFF524: 4BFFFEF8  b 0x82fff41c
	pc = 0x82FFF41C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF528 size=8
    let mut pc: u32 = 0x82FFF528;
    'dispatch: loop {
        match pc {
            0x82FFF528 => {
    //   block [0x82FFF528..0x82FFF530)
	// 82FFF528: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FFF52C: 821403AC  lwz r16, 0x3ac(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(940 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF530 size=56
    let mut pc: u32 = 0x82FFF530;
    'dispatch: loop {
        match pc {
            0x82FFF530 => {
    //   block [0x82FFF530..0x82FFF568)
	// 82FFF530: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FFF534: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF538: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF53C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF544: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FFF548: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82FFF54C: 4BFFA8C5  bl 0x82ff9e10
	ctx.lr = 0x82FFF550;
	sub_82FF9E10(ctx, base);
	// 82FFF550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF554: 3C608300  lis r3, -0x7d00
	ctx.r[3].s64 = -2097152000;
	// 82FFF558: 3863F364  addi r3, r3, -0xc9c
	ctx.r[3].s64 = ctx.r[3].s64 + -3228;
	// 82FFF55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF564: 481A8C48  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF568 size=40
    let mut pc: u32 = 0x82FFF568;
    'dispatch: loop {
        match pc {
            0x82FFF568 => {
    //   block [0x82FFF568..0x82FFF590)
	// 82FFF568: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FFF56C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF570: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF578: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FFF57C: 4BFD38DD  bl 0x82fd2e58
	ctx.lr = 0x82FFF580;
	sub_82FD2E58(ctx, base);
	// 82FFF580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF590 size=16
    let mut pc: u32 = 0x82FFF590;
    'dispatch: loop {
        match pc {
            0x82FFF590 => {
    //   block [0x82FFF590..0x82FFF5A0)
	// 82FFF590: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF594: 396B0424  addi r11, r11, 0x424
	ctx.r[11].s64 = ctx.r[11].s64 + 1060;
	// 82FFF598: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF5A0 size=20
    let mut pc: u32 = 0x82FFF5A0;
    'dispatch: loop {
        match pc {
            0x82FFF5A0 => {
    //   block [0x82FFF5A0..0x82FFF5B4)
	// 82FFF5A0: 7D642B96  divwu r11, r4, r5
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[5].u32;
	// 82FFF5A4: 0CC50000  twi 6, r5, 0
	// 82FFF5A8: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82FFF5AC: 7C6B2050  subf r3, r11, r4
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82FFF5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF5B8 size=20
    let mut pc: u32 = 0x82FFF5B8;
    'dispatch: loop {
        match pc {
            0x82FFF5B8 => {
    //   block [0x82FFF5B8..0x82FFF5CC)
	// 82FFF5B8: 7D642850  subf r11, r4, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82FFF5BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFF5C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFF5C4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFF5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF5D0 size=68
    let mut pc: u32 = 0x82FFF5D0;
    'dispatch: loop {
        match pc {
            0x82FFF5D0 => {
    //   block [0x82FFF5D0..0x82FFF614)
	// 82FFF5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF5D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF5DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF5E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF5E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF5E8: 396B0418  addi r11, r11, 0x418
	ctx.r[11].s64 = ctx.r[11].s64 + 1048;
	// 82FFF5EC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFF5F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF5F4: 41820008  beq 0x82fff5fc
	if ctx.cr[0].eq {
	pc = 0x82FFF5FC; continue 'dispatch;
	}
	// 82FFF5F8: 4BFD8CE9  bl 0x82fd82e0
	ctx.lr = 0x82FFF5FC;
	sub_82FD82E0(ctx, base);
	// 82FFF5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFF604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF60C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFF610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF618 size=56
    let mut pc: u32 = 0x82FFF618;
    'dispatch: loop {
        match pc {
            0x82FFF618 => {
    //   block [0x82FFF618..0x82FFF650)
	// 82FFF618: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFF61C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FFF620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFF624: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82FFF628: 396B0430  addi r11, r11, 0x430
	ctx.r[11].s64 = ctx.r[11].s64 + 1072;
	// 82FFF62C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82FFF630: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FFF634: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82FFF638: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82FFF63C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFF640: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFF644: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FFF648: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FFF64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFF650 size=32
    let mut pc: u32 = 0x82FFF650;
    'dispatch: loop {
        match pc {
            0x82FFF650 => {
    //   block [0x82FFF650..0x82FFF670)
	// 82FFF650: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FFF654: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFF658: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF65C: 99640015  stb r11, 0x15(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82FFF660: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF664: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FFF668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF66C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF670 size=280
    let mut pc: u32 = 0x82FFF670;
    'dispatch: loop {
        match pc {
            0x82FFF670 => {
    //   block [0x82FFF670..0x82FFF788)
	// 82FFF670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF678: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFF67C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF684: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFF688: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFF68C: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFF690: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF694: 41820040  beq 0x82fff6d4
	if ctx.cr[0].eq {
	pc = 0x82FFF6D4; continue 'dispatch;
	}
	// 82FFF698: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF69C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF6A0: 4182000C  beq 0x82fff6ac
	if ctx.cr[0].eq {
	pc = 0x82FFF6AC; continue 'dispatch;
	}
	// 82FFF6A4: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFF6A8: 4800000C  b 0x82fff6b4
	pc = 0x82FFF6B4; continue 'dispatch;
	// 82FFF6AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFF6B0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFF6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF6B8: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFF6BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF6C0: 4BFFA811  bl 0x82ff9ed0
	ctx.lr = 0x82FFF6C4;
	sub_82FF9ED0(ctx, base);
	// 82FFF6C4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFF6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF6CC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFF6D0: 481B1559  bl 0x831b0c28
	ctx.lr = 0x82FFF6D4;
	sub_831B0C28(ctx, base);
	// 82FFF6D4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF6DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFF6E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF6E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFF6E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF6EC: 409A0034  bne cr6, 0x82fff720
	if !ctx.cr[6].eq {
	pc = 0x82FFF720; continue 'dispatch;
	}
	// 82FFF6F0: 4E800421  bctrl
	ctx.lr = 0x82FFF6F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF6F4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFF6F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FFF6FC: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFF700: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFF704: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF708: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82FFF70C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FFF710: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FFF714: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFF718: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FFF71C: 48000054  b 0x82fff770
	pc = 0x82FFF770; continue 'dispatch;
	// 82FFF720: 4E800421  bctrl
	ctx.lr = 0x82FFF724;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF724: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFF728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFF72C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFF730: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FFF734: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FFF738: 7D6B5039  and. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF73C: 4182002C  beq 0x82fff768
	if ctx.cr[0].eq {
	pc = 0x82FFF768; continue 'dispatch;
	}
	// 82FFF740: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFF748: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF74C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF754: 4E800421  bctrl
	ctx.lr = 0x82FFF758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF758: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FFF75C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FFF760: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFF764: 419A0008  beq cr6, 0x82fff76c
	if ctx.cr[6].eq {
	pc = 0x82FFF76C; continue 'dispatch;
	}
	// 82FFF768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFF76C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFF770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF77C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFF780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFF784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF788 size=100
    let mut pc: u32 = 0x82FFF788;
    'dispatch: loop {
        match pc {
            0x82FFF788 => {
    //   block [0x82FFF788..0x82FFF7EC)
	// 82FFF788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFF794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF79C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF7A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFF7A4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFF7A8: 4800001C  b 0x82fff7c4
	pc = 0x82FFF7C4; continue 'dispatch;
	// 82FFF7AC: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FFF7B0: 419A0024  beq cr6, 0x82fff7d4
	if ctx.cr[6].eq {
	pc = 0x82FFF7D4; continue 'dispatch;
	}
	// 82FFF7B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF7B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF7BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF7C0: 4E800421  bctrl
	ctx.lr = 0x82FFF7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF7C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF7C8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF7CC: 409AFFE0  bne cr6, 0x82fff7ac
	if !ctx.cr[6].eq {
	pc = 0x82FFF7AC; continue 'dispatch;
	}
	// 82FFF7D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF7D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFF7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF7E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFF7E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFF7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF7F0 size=272
    let mut pc: u32 = 0x82FFF7F0;
    'dispatch: loop {
        match pc {
            0x82FFF7F0 => {
    //   block [0x82FFF7F0..0x82FFF900)
	// 82FFF7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF7F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFF7FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF804: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFF808: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FFF80C: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFF810: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF814: 41820040  beq 0x82fff854
	if ctx.cr[0].eq {
	pc = 0x82FFF854; continue 'dispatch;
	}
	// 82FFF818: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF81C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF820: 4182000C  beq 0x82fff82c
	if ctx.cr[0].eq {
	pc = 0x82FFF82C; continue 'dispatch;
	}
	// 82FFF824: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFF828: 4800000C  b 0x82fff834
	pc = 0x82FFF834; continue 'dispatch;
	// 82FFF82C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFF830: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFF834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF838: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFF83C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF840: 4BFFA691  bl 0x82ff9ed0
	ctx.lr = 0x82FFF844;
	sub_82FF9ED0(ctx, base);
	// 82FFF844: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFF848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF84C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFF850: 481B13D9  bl 0x831b0c28
	ctx.lr = 0x82FFF854;
	sub_831B0C28(ctx, base);
	// 82FFF854: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFF858: 409A000C  bne cr6, 0x82fff864
	if !ctx.cr[6].eq {
	pc = 0x82FFF864; continue 'dispatch;
	}
	// 82FFF85C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF860: 48000088  b 0x82fff8e8
	pc = 0x82FFF8E8; continue 'dispatch;
	// 82FFF864: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF868: 41820038  beq 0x82fff8a0
	if ctx.cr[0].eq {
	pc = 0x82FFF8A0; continue 'dispatch;
	}
	// 82FFF86C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF874: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF87C: 4E800421  bctrl
	ctx.lr = 0x82FFF880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF880: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF884: 4182001C  beq 0x82fff8a0
	if ctx.cr[0].eq {
	pc = 0x82FFF8A0; continue 'dispatch;
	}
	// 82FFF888: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF88C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF890: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFF894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF898: 4E800421  bctrl
	ctx.lr = 0x82FFF89C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF89C: 4800004C  b 0x82fff8e8
	pc = 0x82FFF8E8; continue 'dispatch;
	// 82FFF8A0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF8A4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF8A8: 419A003C  beq cr6, 0x82fff8e4
	if ctx.cr[6].eq {
	pc = 0x82FFF8E4; continue 'dispatch;
	}
	// 82FFF8AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF8B4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFF8B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF8BC: 4E800421  bctrl
	ctx.lr = 0x82FFF8C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF8C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF8C4: 40820024  bne 0x82fff8e8
	if !ctx.cr[0].eq {
	pc = 0x82FFF8E8; continue 'dispatch;
	}
	// 82FFF8C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF8D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF8D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF8D8: 4E800421  bctrl
	ctx.lr = 0x82FFF8DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF8DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFF8E0: 4082FFC0  bne 0x82fff8a0
	if !ctx.cr[0].eq {
	pc = 0x82FFF8A0; continue 'dispatch;
	}
	// 82FFF8E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF8E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFF8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFF8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFF8F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFF8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFF8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFF900 size=280
    let mut pc: u32 = 0x82FFF900;
    'dispatch: loop {
        match pc {
            0x82FFF900 => {
    //   block [0x82FFF900..0x82FFFA18)
	// 82FFF900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFF904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFF908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFF90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFF910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFF914: 89630015  lbz r11, 0x15(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFF918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FFF91C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF920: 41820040  beq 0x82fff960
	if ctx.cr[0].eq {
	pc = 0x82FFF960; continue 'dispatch;
	}
	// 82FFF924: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFF928: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFF92C: 4182000C  beq 0x82fff938
	if ctx.cr[0].eq {
	pc = 0x82FFF938; continue 'dispatch;
	}
	// 82FFF930: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFF934: 4800000C  b 0x82fff940
	pc = 0x82FFF940; continue 'dispatch;
	// 82FFF938: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFF93C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFF940: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFF944: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFF948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF94C: 4BFFA585  bl 0x82ff9ed0
	ctx.lr = 0x82FFF950;
	sub_82FF9ED0(ctx, base);
	// 82FFF950: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFF954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFF958: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFF95C: 481B12CD  bl 0x831b0c28
	ctx.lr = 0x82FFF960;
	sub_831B0C28(ctx, base);
	// 82FFF960: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFF964: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FFF968: 409A000C  bne cr6, 0x82fff974
	if !ctx.cr[6].eq {
	pc = 0x82FFF974; continue 'dispatch;
	}
	// 82FFF96C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFF970: 48000090  b 0x82fffa00
	pc = 0x82FFFA00; continue 'dispatch;
	// 82FFF974: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF97C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FFF980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF984: 4E800421  bctrl
	ctx.lr = 0x82FFF988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF988: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FFF98C: 4082001C  bne 0x82fff9a8
	if !ctx.cr[0].eq {
	pc = 0x82FFF9A8; continue 'dispatch;
	}
	// 82FFF990: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFF998: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFF99C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF9A0: 4E800421  bctrl
	ctx.lr = 0x82FFF9A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF9A4: 4800005C  b 0x82fffa00
	pc = 0x82FFFA00; continue 'dispatch;
	// 82FFF9A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF9B0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF9B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF9B8: 4E800421  bctrl
	ctx.lr = 0x82FFF9BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF9BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF9C0: 4182003C  beq 0x82fff9fc
	if ctx.cr[0].eq {
	pc = 0x82FFF9FC; continue 'dispatch;
	}
	// 82FFF9C4: 4800001C  b 0x82fff9e0
	pc = 0x82FFF9E0; continue 'dispatch;
	// 82FFF9C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF9D0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFF9D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF9D8: 4E800421  bctrl
	ctx.lr = 0x82FFF9DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF9DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFF9E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FFF9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFF9E8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FFF9EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FFF9F0: 4E800421  bctrl
	ctx.lr = 0x82FFF9F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FFF9F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFF9F8: 4082FFD0  bne 0x82fff9c8
	if !ctx.cr[0].eq {
	pc = 0x82FFF9C8; continue 'dispatch;
	}
	// 82FFF9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFFA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFA0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFA10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFA18 size=212
    let mut pc: u32 = 0x82FFFA18;
    'dispatch: loop {
        match pc {
            0x82FFFA18 => {
    //   block [0x82FFFA18..0x82FFFAEC)
	// 82FFFA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFA20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFFA24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFA2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFA30: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFFA34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFA38: 41820040  beq 0x82fffa78
	if ctx.cr[0].eq {
	pc = 0x82FFFA78; continue 'dispatch;
	}
	// 82FFFA3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFA40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFA44: 4182000C  beq 0x82fffa50
	if ctx.cr[0].eq {
	pc = 0x82FFFA50; continue 'dispatch;
	}
	// 82FFFA48: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFFA4C: 4800000C  b 0x82fffa58
	pc = 0x82FFFA58; continue 'dispatch;
	// 82FFFA50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFFA54: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFFA58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFA5C: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFA60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFA64: 4BFFA46D  bl 0x82ff9ed0
	ctx.lr = 0x82FFFA68;
	sub_82FF9ED0(ctx, base);
	// 82FFFA68: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFA6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFA70: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFA74: 481B11B5  bl 0x831b0c28
	ctx.lr = 0x82FFFA78;
	sub_831B0C28(ctx, base);
	// 82FFFA78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFFA7C: 419A0058  beq cr6, 0x82fffad4
	if ctx.cr[6].eq {
	pc = 0x82FFFAD4; continue 'dispatch;
	}
	// 82FFFA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFA84: 4BFFFD05  bl 0x82fff788
	ctx.lr = 0x82FFFA88;
	sub_82FFF788(ctx, base);
	// 82FFFA88: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FFFA8C: 41820048  beq 0x82fffad4
	if ctx.cr[0].eq {
	pc = 0x82FFFAD4; continue 'dispatch;
	}
	// 82FFFA90: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFA94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFFA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFA9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFAA0: 4182000C  beq 0x82fffaac
	if ctx.cr[0].eq {
	pc = 0x82FFFAAC; continue 'dispatch;
	}
	// 82FFFAA4: 4BFFFE5D  bl 0x82fff900
	ctx.lr = 0x82FFFAA8;
	sub_82FFF900(ctx, base);
	// 82FFFAA8: 48000028  b 0x82fffad0
	pc = 0x82FFFAD0; continue 'dispatch;
	// 82FFFAAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFAB0: 4BFFFD41  bl 0x82fff7f0
	ctx.lr = 0x82FFFAB4;
	sub_82FFF7F0(ctx, base);
	// 82FFFAB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFAB8: 40820018  bne 0x82fffad0
	if !ctx.cr[0].eq {
	pc = 0x82FFFAD0; continue 'dispatch;
	}
	// 82FFFABC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFFAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFAC4: 4BFFFE3D  bl 0x82fff900
	ctx.lr = 0x82FFFAC8;
	sub_82FFF900(ctx, base);
	// 82FFFAC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFFACC: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FFFAD0: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FFFAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFFAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFAE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFAE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFAF0 size=76
    let mut pc: u32 = 0x82FFFAF0;
    'dispatch: loop {
        match pc {
            0x82FFFAF0 => {
    //   block [0x82FFFAF0..0x82FFFB3C)
	// 82FFFAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFB00: 3D608215  lis r11, -0x7deb
	ctx.r[11].s64 = -2112552960;
	// 82FFFB04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFB08: 396B8CB8  addi r11, r11, -0x7348
	ctx.r[11].s64 = ctx.r[11].s64 + -29512;
	// 82FFFB0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FFFB10: 548907FF  clrlwi. r9, r4, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FFFB14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFFB18: 995F0015  stb r10, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82FFFB1C: 41820008  beq 0x82fffb24
	if ctx.cr[0].eq {
	pc = 0x82FFFB24; continue 'dispatch;
	}
	// 82FFFB20: 4B2C0749  bl 0x822c0268
	ctx.lr = 0x82FFFB24;
	sub_822C0268(ctx, base);
	// 82FFFB24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFFB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFB34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFB40 size=236
    let mut pc: u32 = 0x82FFFB40;
    'dispatch: loop {
        match pc {
            0x82FFFB40 => {
    //   block [0x82FFFB40..0x82FFFC2C)
	// 82FFFB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFFB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFB58: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFFB5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFB60: 41820040  beq 0x82fffba0
	if ctx.cr[0].eq {
	pc = 0x82FFFBA0; continue 'dispatch;
	}
	// 82FFFB64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFB68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFB6C: 4182000C  beq 0x82fffb78
	if ctx.cr[0].eq {
	pc = 0x82FFFB78; continue 'dispatch;
	}
	// 82FFFB70: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFFB74: 4800000C  b 0x82fffb80
	pc = 0x82FFFB80; continue 'dispatch;
	// 82FFFB78: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFFB7C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFFB80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFB84: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFB88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFB8C: 4BFFA345  bl 0x82ff9ed0
	ctx.lr = 0x82FFFB90;
	sub_82FF9ED0(ctx, base);
	// 82FFFB90: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFB94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFB98: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFB9C: 481B108D  bl 0x831b0c28
	ctx.lr = 0x82FFFBA0;
	sub_831B0C28(ctx, base);
	// 82FFFBA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFFBA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFFBA8: 409A000C  bne cr6, 0x82fffbb4
	if !ctx.cr[6].eq {
	pc = 0x82FFFBB4; continue 'dispatch;
	}
	// 82FFFBAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFFBB0: 48000064  b 0x82fffc14
	pc = 0x82FFFC14; continue 'dispatch;
	// 82FFFBB4: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFFBB8: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFBBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFBC0: 40820014  bne 0x82fffbd4
	if !ctx.cr[0].eq {
	pc = 0x82FFFBD4; continue 'dispatch;
	}
	// 82FFFBC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFFBC8: 419A000C  beq cr6, 0x82fffbd4
	if ctx.cr[6].eq {
	pc = 0x82FFFBD4; continue 'dispatch;
	}
	// 82FFFBCC: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFFBD0: 48000018  b 0x82fffbe8
	pc = 0x82FFFBE8; continue 'dispatch;
	// 82FFFBD4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FFFBD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFFBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFBE0: 4BFFFC11  bl 0x82fff7f0
	ctx.lr = 0x82FFFBE4;
	sub_82FFF7F0(ctx, base);
	// 82FFFBE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFFBE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFFBEC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FFFBF0: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FFFBF4: 419AFFB8  beq cr6, 0x82fffbac
	if ctx.cr[6].eq {
	pc = 0x82FFFBAC; continue 'dispatch;
	}
	// 82FFFBF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FFFBFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFC00: 4BFFFA71  bl 0x82fff670
	ctx.lr = 0x82FFFC04;
	sub_82FFF670(ctx, base);
	// 82FFFC04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFFC08: 4182FFB0  beq 0x82fffbb8
	if ctx.cr[0].eq {
	pc = 0x82FFFBB8; continue 'dispatch;
	}
	// 82FFFC0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFFC10: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82FFFC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFFC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFC20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFC24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFC30 size=240
    let mut pc: u32 = 0x82FFFC30;
    'dispatch: loop {
        match pc {
            0x82FFFC30 => {
    //   block [0x82FFFC30..0x82FFFD20)
	// 82FFFC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FFFC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFC44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FFFC48: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FFFC4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFC50: 41820040  beq 0x82fffc90
	if ctx.cr[0].eq {
	pc = 0x82FFFC90; continue 'dispatch;
	}
	// 82FFFC54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFC58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFC5C: 4182000C  beq 0x82fffc68
	if ctx.cr[0].eq {
	pc = 0x82FFFC68; continue 'dispatch;
	}
	// 82FFFC60: 80CB0090  lwz r6, 0x90(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FFFC64: 4800000C  b 0x82fffc70
	pc = 0x82FFFC70; continue 'dispatch;
	// 82FFFC68: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FFFC6C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FFFC70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFC74: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFC78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFC7C: 4BFFA255  bl 0x82ff9ed0
	ctx.lr = 0x82FFFC80;
	sub_82FF9ED0(ctx, base);
	// 82FFFC80: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFC84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFC88: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFC8C: 481B0F9D  bl 0x831b0c28
	ctx.lr = 0x82FFFC90;
	sub_831B0C28(ctx, base);
	// 82FFFC90: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFFC94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FFFC98: 419A006C  beq cr6, 0x82fffd04
	if ctx.cr[6].eq {
	pc = 0x82FFFD04; continue 'dispatch;
	}
	// 82FFFC9C: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFFCA0: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFCA4: 41820060  beq 0x82fffd04
	if ctx.cr[0].eq {
	pc = 0x82FFFD04; continue 'dispatch;
	}
	// 82FFFCA8: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFCAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFCB0: 41820014  beq 0x82fffcc4
	if ctx.cr[0].eq {
	pc = 0x82FFFCC4; continue 'dispatch;
	}
	// 82FFFCB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFFCB8: 419A000C  beq cr6, 0x82fffcc4
	if ctx.cr[6].eq {
	pc = 0x82FFFCC4; continue 'dispatch;
	}
	// 82FFFCBC: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FFFCC0: 48000014  b 0x82fffcd4
	pc = 0x82FFFCD4; continue 'dispatch;
	// 82FFFCC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFFCC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFFCCC: 4BFFFC35  bl 0x82fff900
	ctx.lr = 0x82FFFCD0;
	sub_82FFF900(ctx, base);
	// 82FFFCD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFCD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFCD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FFFCDC: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FFFCE0: 419A0024  beq cr6, 0x82fffd04
	if ctx.cr[6].eq {
	pc = 0x82FFFD04; continue 'dispatch;
	}
	// 82FFFCE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FFFCE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FFFCEC: 4BFFF985  bl 0x82fff670
	ctx.lr = 0x82FFFCF0;
	sub_82FFF670(ctx, base);
	// 82FFFCF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FFFCF4: 4182FFB4  beq 0x82fffca8
	if ctx.cr[0].eq {
	pc = 0x82FFFCA8; continue 'dispatch;
	}
	// 82FFFCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFCFC: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82FFFD00: 48000008  b 0x82fffd08
	pc = 0x82FFFD08; continue 'dispatch;
	// 82FFFD04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FFFD08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FFFD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFD14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FFFD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFD20 size=68
    let mut pc: u32 = 0x82FFFD20;
    'dispatch: loop {
        match pc {
            0x82FFFD20 => {
    //   block [0x82FFFD20..0x82FFFD64)
	// 82FFFD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FFFD2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFD30: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFFD34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FFFD38: 396B0458  addi r11, r11, 0x458
	ctx.r[11].s64 = ctx.r[11].s64 + 1112;
	// 82FFFD3C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FFFD40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFFD44: 41820008  beq 0x82fffd4c
	if ctx.cr[0].eq {
	pc = 0x82FFFD4C; continue 'dispatch;
	}
	// 82FFFD48: 4B2C0521  bl 0x822c0268
	ctx.lr = 0x82FFFD4C;
	sub_822C0268(ctx, base);
	// 82FFFD4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FFFD50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FFFD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FFFD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FFFD68 size=92
    let mut pc: u32 = 0x82FFFD68;
    'dispatch: loop {
        match pc {
            0x82FFFD68 => {
    //   block [0x82FFFD68..0x82FFFDC4)
	// 82FFFD68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FFFD6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFFD70: 396B04C0  addi r11, r11, 0x4c0
	ctx.r[11].s64 = ctx.r[11].s64 + 1216;
	// 82FFFD74: 3944000C  addi r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 + 12;
	// 82FFFD78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FFFD7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFD80: 409A0008  bne cr6, 0x82fffd88
	if !ctx.cr[6].eq {
	pc = 0x82FFFD88; continue 'dispatch;
	}
	// 82FFFD84: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FFFD88: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FFFD8C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FFFD90: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FFFD94: 3944000C  addi r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 + 12;
	// 82FFFD98: 409A0008  bne cr6, 0x82fffda0
	if !ctx.cr[6].eq {
	pc = 0x82FFFDA0; continue 'dispatch;
	}
	// 82FFFD9C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FFFDA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FFFDA4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82FFFDA8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FFFDAC: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82FFFDB0: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FFFDB4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FFFDB8: 99230014  stb r9, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 82FFFDBC: 90A30024  stw r5, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82FFFDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFDC8 size=80
    let mut pc: u32 = 0x82FFFDC8;
    'dispatch: loop {
        match pc {
            0x82FFFDC8 => {
    //   block [0x82FFFDC8..0x82FFFE18)
	// 82FFFDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFDD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFDD4: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFDD8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFDDC: 41820028  beq 0x82fffe04
	if ctx.cr[0].eq {
	pc = 0x82FFFE04; continue 'dispatch;
	}
	// 82FFFDE0: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFDE8: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFDEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFDF0: 4BFFA0E1  bl 0x82ff9ed0
	ctx.lr = 0x82FFFDF4;
	sub_82FF9ED0(ctx, base);
	// 82FFFDF4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFDF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFDFC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFE00: 481B0E29  bl 0x831b0c28
	ctx.lr = 0x82FFFE04;
	sub_831B0C28(ctx, base);
	// 82FFFE04: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFFE08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFE18 size=80
    let mut pc: u32 = 0x82FFFE18;
    'dispatch: loop {
        match pc {
            0x82FFFE18 => {
    //   block [0x82FFFE18..0x82FFFE68)
	// 82FFFE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFE20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFE24: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFE28: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE2C: 41820028  beq 0x82fffe54
	if ctx.cr[0].eq {
	pc = 0x82FFFE54; continue 'dispatch;
	}
	// 82FFFE30: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFE34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFE38: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFE3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFE40: 4BFFA091  bl 0x82ff9ed0
	ctx.lr = 0x82FFFE44;
	sub_82FF9ED0(ctx, base);
	// 82FFFE44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFE48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFE4C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFE50: 481B0DD9  bl 0x831b0c28
	ctx.lr = 0x82FFFE54;
	sub_831B0C28(ctx, base);
	// 82FFFE54: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFE58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFE68 size=80
    let mut pc: u32 = 0x82FFFE68;
    'dispatch: loop {
        match pc {
            0x82FFFE68 => {
    //   block [0x82FFFE68..0x82FFFEB8)
	// 82FFFE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFE74: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFE78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFE7C: 41820028  beq 0x82fffea4
	if ctx.cr[0].eq {
	pc = 0x82FFFEA4; continue 'dispatch;
	}
	// 82FFFE80: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFE88: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFE8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFE90: 4BFFA041  bl 0x82ff9ed0
	ctx.lr = 0x82FFFE94;
	sub_82FF9ED0(ctx, base);
	// 82FFFE94: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFE98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFE9C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFEA0: 481B0D89  bl 0x831b0c28
	ctx.lr = 0x82FFFEA4;
	sub_831B0C28(ctx, base);
	// 82FFFEA4: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFEA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFEB8 size=80
    let mut pc: u32 = 0x82FFFEB8;
    'dispatch: loop {
        match pc {
            0x82FFFEB8 => {
    //   block [0x82FFFEB8..0x82FFFF08)
	// 82FFFEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFEC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFEC4: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFEC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFECC: 41820028  beq 0x82fffef4
	if ctx.cr[0].eq {
	pc = 0x82FFFEF4; continue 'dispatch;
	}
	// 82FFFED0: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFED8: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFEDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFEE0: 4BFF9FF1  bl 0x82ff9ed0
	ctx.lr = 0x82FFFEE4;
	sub_82FF9ED0(ctx, base);
	// 82FFFEE4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFEE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFEEC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFEF0: 481B0D39  bl 0x831b0c28
	ctx.lr = 0x82FFFEF4;
	sub_831B0C28(ctx, base);
	// 82FFFEF4: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFEF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFF08 size=120
    let mut pc: u32 = 0x82FFFF08;
    'dispatch: loop {
        match pc {
            0x82FFFF08 => {
    //   block [0x82FFFF08..0x82FFFF80)
	// 82FFFF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFF14: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFF18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFF1C: 41820028  beq 0x82ffff44
	if ctx.cr[0].eq {
	pc = 0x82FFFF44; continue 'dispatch;
	}
	// 82FFFF20: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFF24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFF28: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFF2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFF30: 4BFF9FA1  bl 0x82ff9ed0
	ctx.lr = 0x82FFFF34;
	sub_82FF9ED0(ctx, base);
	// 82FFFF34: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFF38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFF3C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFF40: 481B0CE9  bl 0x831b0c28
	ctx.lr = 0x82FFFF44;
	sub_831B0C28(ctx, base);
	// 82FFFF44: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FFFF48: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FFFF4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFFF50: 409A0018  bne cr6, 0x82ffff68
	if !ctx.cr[6].eq {
	pc = 0x82FFFF68; continue 'dispatch;
	}
	// 82FFFF54: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FFFF58: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FFFF5C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FFFF60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FFFF64: 419A0008  beq cr6, 0x82ffff6c
	if ctx.cr[6].eq {
	pc = 0x82FFFF6C; continue 'dispatch;
	}
	// 82FFFF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FFFF6C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FFFF70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFF80 size=80
    let mut pc: u32 = 0x82FFFF80;
    'dispatch: loop {
        match pc {
            0x82FFFF80 => {
    //   block [0x82FFFF80..0x82FFFFD0)
	// 82FFFF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFF88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFF8C: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFF90: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFF94: 41820028  beq 0x82ffffbc
	if ctx.cr[0].eq {
	pc = 0x82FFFFBC; continue 'dispatch;
	}
	// 82FFFF98: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFFA0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFFA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFFA8: 4BFF9F29  bl 0x82ff9ed0
	ctx.lr = 0x82FFFFAC;
	sub_82FF9ED0(ctx, base);
	// 82FFFFAC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FFFFB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFFB4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FFFFB8: 481B0C71  bl 0x831b0c28
	ctx.lr = 0x82FFFFBC;
	sub_831B0C28(ctx, base);
	// 82FFFFBC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FFFFC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FFFFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FFFFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FFFFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FFFFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FFFFD0 size=80
    let mut pc: u32 = 0x82FFFFD0;
    'dispatch: loop {
        match pc {
            0x82FFFFD0 => {
    //   block [0x82FFFFD0..0x83000020)
	// 82FFFFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FFFFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FFFFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FFFFDC: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FFFFE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FFFFE4: 41820028  beq 0x8300000c
	if ctx.cr[0].eq {
	pc = 0x8300000C; continue 'dispatch;
	}
	// 82FFFFE8: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FFFFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FFFFF0: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FFFFF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FFFFF8: 4BFF9ED9  bl 0x82ff9ed0
	ctx.lr = 0x82FFFFFC;
	sub_82FF9ED0(ctx, base);
	// 82FFFFFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83000000: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000004: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83000008: 481B0C21  bl 0x831b0c28
	ctx.lr = 0x8300000C;
	sub_831B0C28(ctx, base);
	// 8300000C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 83000010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300001C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000020 size=148
    let mut pc: u32 = 0x83000020;
    'dispatch: loop {
        match pc {
            0x83000020 => {
    //   block [0x83000020..0x830000B4)
	// 83000020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300002C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000034: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83000038: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300003C: 41820028  beq 0x83000064
	if ctx.cr[0].eq {
	pc = 0x83000064; continue 'dispatch;
	}
	// 83000040: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000044: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000048: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8300004C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000050: 4BFF9E81  bl 0x82ff9ed0
	ctx.lr = 0x83000054;
	sub_82FF9ED0(ctx, base);
	// 83000054: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83000058: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300005C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83000060: 481B0BC9  bl 0x831b0c28
	ctx.lr = 0x83000064;
	sub_831B0C28(ctx, base);
	// 83000064: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000068: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300006C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000070: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 83000074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000078: 4E800421  bctrl
	ctx.lr = 0x8300007C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300007C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83000080: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83000084: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83000088: 995F001C  stb r10, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 8300008C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83000090: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83000094: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83000098: 995F0014  stb r10, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 8300009C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830000A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830000A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830000A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830000AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830000B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830000B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830000B8 size=128
    let mut pc: u32 = 0x830000B8;
    'dispatch: loop {
        match pc {
            0x830000B8 => {
    //   block [0x830000B8..0x83000138)
	// 830000B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830000BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830000C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830000C4: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830000C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830000CC: 41820028  beq 0x830000f4
	if ctx.cr[0].eq {
	pc = 0x830000F4; continue 'dispatch;
	}
	// 830000D0: 80C30024  lwz r6, 0x24(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 830000D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830000D8: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830000DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830000E0: 4BFF9DF1  bl 0x82ff9ed0
	ctx.lr = 0x830000E4;
	sub_82FF9ED0(ctx, base);
	// 830000E4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830000E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830000EC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830000F0: 481B0B39  bl 0x831b0c28
	ctx.lr = 0x830000F4;
	sub_831B0C28(ctx, base);
	// 830000F4: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830000F8: 41820018  beq 0x83000110
	if ctx.cr[0].eq {
	pc = 0x83000110; continue 'dispatch;
	}
	// 830000FC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83000100: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83000104: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83000108: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8300010C: 48000014  b 0x83000120
	pc = 0x83000120; continue 'dispatch;
	// 83000110: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000114: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000118: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300011C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83000120: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83000124: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83000128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300012C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000138 size=176
    let mut pc: u32 = 0x83000138;
    'dispatch: loop {
        match pc {
            0x83000138 => {
    //   block [0x83000138..0x830001E8)
	// 83000138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000140: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83000144: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300014C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000150: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83000154: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000158: 41820028  beq 0x83000180
	if ctx.cr[0].eq {
	pc = 0x83000180; continue 'dispatch;
	}
	// 8300015C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83000160: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000164: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83000168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300016C: 4BFF9D65  bl 0x82ff9ed0
	ctx.lr = 0x83000170;
	sub_82FF9ED0(ctx, base);
	// 83000170: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83000174: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000178: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300017C: 481B0AAD  bl 0x831b0c28
	ctx.lr = 0x83000180;
	sub_831B0C28(ctx, base);
	// 83000180: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83000184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000188: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300018C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000190: 4E800421  bctrl
	ctx.lr = 0x83000194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000194: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83000198: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300019C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830001A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830001A4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830001A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830001AC: 4E800421  bctrl
	ctx.lr = 0x830001B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830001B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830001B4: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830001B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830001BC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830001C0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830001C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830001C8: 4E800421  bctrl
	ctx.lr = 0x830001CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830001CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830001D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830001D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830001D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830001DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830001E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830001E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830001E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830001E8 size=16
    let mut pc: u32 = 0x830001E8;
    'dispatch: loop {
        match pc {
            0x830001E8 => {
    //   block [0x830001E8..0x830001F8)
	// 830001E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830001EC: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 830001F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830001F4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830001F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830001F8 size=136
    let mut pc: u32 = 0x830001F8;
    'dispatch: loop {
        match pc {
            0x830001F8 => {
    //   block [0x830001F8..0x83000280)
	// 830001F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830001FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8300020C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83000210: 419A0050  beq cr6, 0x83000260
	if ctx.cr[6].eq {
	pc = 0x83000260; continue 'dispatch;
	}
	// 83000214: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300021C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000224: 4E800421  bctrl
	ctx.lr = 0x83000228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000228: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8300022C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83000230: 419A0048  beq cr6, 0x83000278
	if ctx.cr[6].eq {
	pc = 0x83000278; continue 'dispatch;
	}
	// 83000234: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 83000238: 419A0040  beq cr6, 0x83000278
	if ctx.cr[6].eq {
	pc = 0x83000278; continue 'dispatch;
	}
	// 8300023C: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83000240: 419A0038  beq cr6, 0x83000278
	if ctx.cr[6].eq {
	pc = 0x83000278; continue 'dispatch;
	}
	// 83000244: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300024C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000254: 4E800421  bctrl
	ctx.lr = 0x83000258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000258: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8300025C: 4082FFB8  bne 0x83000214
	if !ctx.cr[0].eq {
	pc = 0x83000214; continue 'dispatch;
	}
	// 83000260: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83000264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300026C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83000270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000274: 4E800020  blr
	return;
	// 83000278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300027C: 4BFFFFE8  b 0x83000264
	pc = 0x83000264; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000280 size=96
    let mut pc: u32 = 0x83000280;
    'dispatch: loop {
        match pc {
            0x83000280 => {
    //   block [0x83000280..0x830002E0)
	// 83000280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83000284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300028C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000294: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83000298: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8300029C: 419A0024  beq cr6, 0x830002c0
	if ctx.cr[6].eq {
	pc = 0x830002C0; continue 'dispatch;
	}
	// 830002A0: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830002A4: 419A0034  beq cr6, 0x830002d8
	if ctx.cr[6].eq {
	pc = 0x830002D8; continue 'dispatch;
	}
	// 830002A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830002AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830002B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830002B4: 4E800421  bctrl
	ctx.lr = 0x830002B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830002B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830002BC: 4082FFE4  bne 0x830002a0
	if !ctx.cr[0].eq {
	pc = 0x830002A0; continue 'dispatch;
	}
	// 830002C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830002C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830002C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830002CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830002D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830002D4: 4E800020  blr
	return;
	// 830002D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830002DC: 4BFFFFE8  b 0x830002c4
	pc = 0x830002C4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830002E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830002E0 size=164
    let mut pc: u32 = 0x830002E0;
    'dispatch: loop {
        match pc {
            0x830002E0 => {
    //   block [0x830002E0..0x83000384)
	// 830002E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830002E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830002E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830002EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830002F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830002F4: 409A000C  bne cr6, 0x83000300
	if !ctx.cr[6].eq {
	pc = 0x83000300; continue 'dispatch;
	}
	// 830002F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830002FC: 48000074  b 0x83000370
	pc = 0x83000370; continue 'dispatch;
	// 83000300: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000304: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000308: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8300030C: 48000018  b 0x83000324
	pc = 0x83000324; continue 'dispatch;
	// 83000310: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000318: 4E800421  bctrl
	ctx.lr = 0x8300031C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300031C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83000320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000324: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83000328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300032C: 4E800421  bctrl
	ctx.lr = 0x83000330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000330: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000334: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300033C: 4082FFD4  bne 0x83000310
	if !ctx.cr[0].eq {
	pc = 0x83000310; continue 'dispatch;
	}
	// 83000340: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83000344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000348: 4E800421  bctrl
	ctx.lr = 0x8300034C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300034C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83000350: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83000354: 419A0018  beq cr6, 0x8300036c
	if ctx.cr[6].eq {
	pc = 0x8300036C; continue 'dispatch;
	}
	// 83000358: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 8300035C: 419A0010  beq cr6, 0x8300036c
	if ctx.cr[6].eq {
	pc = 0x8300036C; continue 'dispatch;
	}
	// 83000360: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 83000364: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83000368: 409A0008  bne cr6, 0x83000370
	if !ctx.cr[6].eq {
	pc = 0x83000370; continue 'dispatch;
	}
	// 8300036C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83000370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83000374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83000378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300037C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83000380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000388 size=112
    let mut pc: u32 = 0x83000388;
    'dispatch: loop {
        match pc {
            0x83000388 => {
    //   block [0x83000388..0x830003F8)
	// 83000388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000394: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83000398: 419A004C  beq cr6, 0x830003e4
	if ctx.cr[6].eq {
	pc = 0x830003E4; continue 'dispatch;
	}
	// 8300039C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830003A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830003A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830003A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830003AC: 4E800421  bctrl
	ctx.lr = 0x830003B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830003B0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830003B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830003B8: 419A002C  beq cr6, 0x830003e4
	if ctx.cr[6].eq {
	pc = 0x830003E4; continue 'dispatch;
	}
	// 830003BC: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830003C0: 419A0024  beq cr6, 0x830003e4
	if ctx.cr[6].eq {
	pc = 0x830003E4; continue 'dispatch;
	}
	// 830003C4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 830003C8: 419A001C  beq cr6, 0x830003e4
	if ctx.cr[6].eq {
	pc = 0x830003E4; continue 'dispatch;
	}
	// 830003CC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 830003D0: 4099000C  ble cr6, 0x830003dc
	if !ctx.cr[6].gt {
	pc = 0x830003DC; continue 'dispatch;
	}
	// 830003D4: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 830003D8: 4099000C  ble cr6, 0x830003e4
	if !ctx.cr[6].gt {
	pc = 0x830003E4; continue 'dispatch;
	}
	// 830003DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830003E0: 48000008  b 0x830003e8
	pc = 0x830003E8; continue 'dispatch;
	// 830003E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830003E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830003EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830003F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830003F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830003F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830003F8 size=112
    let mut pc: u32 = 0x830003F8;
    'dispatch: loop {
        match pc {
            0x830003F8 => {
    //   block [0x830003F8..0x83000468)
	// 830003F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830003FC: 481A7D71  bl 0x831a816c
	ctx.lr = 0x83000400;
	sub_831A8130(ctx, base);
	// 83000400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000404: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83000408: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8300040C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83000410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83000414: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000418: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300041C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000420: 4E800421  bctrl
	ctx.lr = 0x83000424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000424: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83000428: 419A000C  beq cr6, 0x83000434
	if ctx.cr[6].eq {
	pc = 0x83000434; continue 'dispatch;
	}
	// 8300042C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83000430: 48000030  b 0x83000460
	pc = 0x83000460; continue 'dispatch;
	// 83000434: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300043C: 4800000C  b 0x83000448
	pc = 0x83000448; continue 'dispatch;
	// 83000440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83000444: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83000448: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300044C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83000450: 4E800421  bctrl
	ctx.lr = 0x83000454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83000454: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000458: 4082FFE8  bne 0x83000440
	if !ctx.cr[0].eq {
	pc = 0x83000440; continue 'dispatch;
	}
	// 8300045C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83000460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83000464: 481A7D58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83000468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83000468 size=140
    let mut pc: u32 = 0x83000468;
    'dispatch: loop {
        match pc {
            0x83000468 => {
    //   block [0x83000468..0x830004F4)
	// 83000468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300046C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83000470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83000474: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83000478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300047C: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83000480: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83000484: 41820028  beq 0x830004ac
	if ctx.cr[0].eq {
	pc = 0x830004AC; continue 'dispatch;
	}
	// 83000488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300048C: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83000490: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83000494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83000498: 4BFF9A39  bl 0x82ff9ed0
	ctx.lr = 0x8300049C;
	sub_82FF9ED0(ctx, base);
	// 8300049C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830004A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830004A4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830004A8: 481B0781  bl 0x831b0c28
	ctx.lr = 0x830004AC;
	sub_831B0C28(ctx, base);
	// 830004AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830004B0: 4BFFFD49  bl 0x830001f8
	ctx.lr = 0x830004B4;
	sub_830001F8(ctx, base);
	// 830004B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830004B8: 40820028  bne 0x830004e0
	if !ctx.cr[0].eq {
	pc = 0x830004E0; continue 'dispatch;
	}
	// 830004BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830004C0: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830004C4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830004C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830004CC: 480388BD  bl 0x83038d88
	ctx.lr = 0x830004D0;
	sub_83038D88(ctx, base);
	// 830004D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830004D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830004D8: 388BC878  addi r4, r11, -0x3788
	ctx.r[4].s64 = ctx.r[11].s64 + -14216;
	// 830004DC: 481B074D  bl 0x831b0c28
	ctx.lr = 0x830004E0;
	sub_831B0C28(ctx, base);
	// 830004E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830004E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830004E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830004EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830004F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


